extern crate vst3_derive;
extern crate vst3_interfaces;

use std::sync::atomic::{AtomicUsize, Ordering};
pub use vst3_derive::{vst3_impl, Vst3Impl};

#[repr(transparent)]
pub struct VTable<T> {
    pub ptr: *const T,
}

impl<T> VTable<T> {
    pub fn new(ptr: &'static T) -> Self {
        VTable { ptr }
    }
}

impl<T> std::fmt::Debug for VTable<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_tuple("VTable").field(&self.ptr).finish()
    }
}

pub unsafe trait BuildVTable<T: 'static> {
    const VTBL: T;
    fn static_vtable() -> VTable<T>;
}

#[derive(Debug)]
pub struct Refcount {
    count: AtomicUsize,
}

impl Default for Refcount {
    fn default() -> Self {
        Refcount {
            count: AtomicUsize::new(1),
        }
    }
}

impl Refcount {
    #[inline]
    /// `fetch_add(1, Acquire) + 1`
    pub unsafe fn add_ref(&self) -> u32 {
        self.count.fetch_add(1, Ordering::Acquire) as u32 + 1
    }

    #[inline]
    /// `fetch_sub(1, Release) - 1`
    pub unsafe fn release(&self) -> u32 {
        self.count.fetch_sub(1, Ordering::Release) as u32 - 1
    }
}