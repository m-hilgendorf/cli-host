/// The contents of `vst3sdk/pluginterfaces`, ported to Rust
#[macro_use]
mod macros;

pub mod base;
pub mod vst;
pub mod gui;
pub use self::base::*;
pub use self::vst::*;
pub use self::gui::*;

use std::ptr::{NonNull,null_mut};
use std::mem::forget;
use std::fmt::{Debug, Error as FmtError, Formatter};
use std::ops::Deref;

pub fn is_iid_equal(iid1 : *const i8, iid2 : *const i8) -> bool {
    for i in 0..16 {
        unsafe {
            if *iid1.offset(i) != *iid2.offset(i) {
                return false;
            }
        }
    }
    true
}

pub trait Interface {
    // Returns the IID of the Interface
    fn iid() -> TUID;
}
// borrowed from the wio crate
pub struct VstPtr<T: Interface>(NonNull<T>);
impl<T> VstPtr<T> where T : Interface {
    pub unsafe fn from_raw(ptr : *mut T) -> Self {
        VstPtr(NonNull::new(ptr).expect("Pointer cannot be null"))
    }
    
    pub unsafe fn up <U>(self) -> VstPtr<U> where T: Deref<Target=U>, U: Interface {
         VstPtr::from_raw(self.into_raw() as *mut U)
    } 

    pub fn into_raw(self) -> *mut T {
        let p = self.0.as_ptr(); 
        forget(self);
        p
    }

    pub fn as_unknown(&self) -> &FUnknown {
        unsafe { &*(self.as_raw() as *mut FUnknown) }
    }

    pub fn cast<U> (&self) -> Result<VstPtr<U>, i32> where U: Interface {
        let mut obj = null_mut();
        let err = unsafe { 
            let iid = U::iid();
            self.as_unknown().queryInterface(&iid as *const i8, &mut obj)
        };
        if err < 0 {  
            Err(err)
        }
        else {  
           unsafe { Ok (VstPtr::from_raw(obj as *mut U)) }
        }
    }

    pub fn as_raw(&self) -> *mut T {
        self.0.as_ptr()
    }
}

impl<T> Deref for VstPtr<T> where T: Interface {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.as_raw() }
    }
}

impl<T> Clone for VstPtr<T> where T: Interface {
    fn clone(&self) -> Self {
        unsafe {
            self.as_unknown().addRef();
            VstPtr::from_raw(self.as_raw())
        }
    }
}

impl<T> Debug for VstPtr<T> where T: Interface {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{:?}", self.0)
    }
}

impl<T> Drop for VstPtr<T> where T: Interface{
    fn drop(&mut self) {
        unsafe { self.as_unknown().release(); }
    }
}

impl<T> PartialEq <VstPtr<T>> for VstPtr<T> where T: Interface {
    fn eq(&self, other: &VstPtr<T>) -> bool {
        self.0 == other.0
    }
}

