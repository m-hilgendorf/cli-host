use super::funknown::*;
use super::ftypes::*;
use std::fmt;
use std::os::raw::c_void;
#[repr(C)]
pub struct PFactoryInfo {
    pub vendor : [char8; 64], 
    pub url    : [char8; 256], 
    pub email  : [char8; 128], 
    pub flags  : i32
}

#[repr(C)]
pub struct PClassInfo {
    pub cid         : TUID,
    pub cardinality : i32, 
    pub category    : [char8; 32], 
    pub name        : [char8; 64],
}



#[repr(C)]
pub struct PClassInfo2 {
    pub cid           : TUID,
    pub cardinality   : i32, 
    pub category      : [char8; 32], 
    pub name          : [char8; 64],
    pub classFlags    : u32, 
    pub subcategories : [char8; 128], 
    pub vendor        : [char8; 64],
    pub version       : [char8; 64], 
    pub sdkVersion    : [char8; 64], 
}

pub struct PClassInfoW {
    pub cid           : TUID,
    pub cardinality   : i32, 
    pub category      : [char8; 32], 
    pub name          : [char16; 64],
    pub classFlags    : u32, 
    pub subcategories : [char8; 128], 
    pub vendor        : [char16; 64],
    pub version       : [char16; 64], 
    pub sdkVersion    : [char16; 64], 
}

//DECLARE_CLASS_IID (IPluginBase, 0x22888DDB, 0x156E45AE, 0x8358B348, 0x08190625)
RIDL!{#[uuid(0x22888DDB, 0x156E45AE, 0x8358B348, 0x08190625)]
    interface IPluginBase(IPluginBaseVtbl) : FUnknown(FUnknownVtbl) {
        fn initialize (context: *mut FUnknown,) -> tresult,
        fn terminate () -> tresult,
    }
}

//DECLARE_CLASS_IID (IPluginFactory, 0x7A4D811C, 0x52114A1F, 0xAED9D2EE, 0x0B43BF9F)
RIDL!{#[uuid(0x7A4D811C, 0x52114A1F, 0xAED9D2EE, 0x0B43BF9F)]
    interface IPluginFactory(IPluginFactoryVtbl) : FUnknown(FUnknownVtbl) {
        fn getFactoryInfo (info : *mut PFactoryInfo,) -> tresult, 
        fn countClasses() -> i32, 
        fn getClassInfo(index : i32, info : *mut PClassInfo,) -> tresult,
        fn createInstance(cid : FIDString, iid : FIDString, obj : *mut *mut c_void,) -> tresult,
    }
}

//DECLARE_CLASS_IID (IPluginFactory2, 0x0007B650, 0xF24B4C0B, 0xA464EDB9, 0xF00B2ABB)
RIDL!{#[uuid(0x0007B650, 0xF24B4C0B, 0xA464EDB9, 0xF00B2ABB)]
    interface IPluginFactory2(IPluginFactory2Vtbl) : IPluginFactory(IPluginFactoryVtbl) {
        fn getClassInfo2(index : i32, info :  *mut PClassInfo2,) -> tresult,
    }    
}

//DECLARE_CLASS_IID (IPluginFactory3, 0x4555A2AB, 0xC1234E57, 0x9B122910, 0x36878931)
//RIDL!{#[uuid(0x4555A2AB, 0xC123, 0x4E57, 0x9B, 0x12, 0x29, 0x10, 0x36, 0x87, 0x89, 0x31)]
RIDL!{#[uuid(0x4555A2AB, 0xC1234E57, 0x9B122910, 0x36878931)]
    interface IPluginFactory3(IPluginFactory3Vtbl) : IPluginFactory2(IPluginFactory2Vtbl) {
        fn getClassInfoUnicode(index : i32, info : *mut PClassInfoW,) -> tresult,
        fn setHostContext (context : *mut FUnknown,) -> tresult,
    }
}

pub type GetFactoryProc = unsafe extern fn () -> *mut IPluginFactory;

impl fmt::Debug for PClassInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
        use std::ffi::CStr;
        let name = CStr::from_ptr(self.name.as_ptr());
        let category = CStr::from_ptr(self.category.as_ptr());
        write!(f, r#"
    cid: {:?}
    cardinality: {}, 
    category: {:?}, 
    name :{:?}    
        "#, self.cid, self.cardinality, category, name)
        }
    }
}
impl fmt::Debug for PClassInfo2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
        use std::ffi::CStr;
        let name = CStr::from_ptr(self.name.as_ptr());

        let category = CStr::from_ptr(self.category.as_ptr());
        let subcategories = CStr::from_ptr(self.subcategories.as_ptr());
        let vendor = CStr::from_ptr(self.vendor.as_ptr());
        let version = CStr::from_ptr(self.version.as_ptr());
        let sdkVersion = CStr::from_ptr(self.sdkVersion.as_ptr());
        
        write!(f, r#"
    cid           : {:?}
    cardinality   : {}
    category      : {:?} 
    name          : {:?} 
    classFlags    : {} 
    subcategories : {:?} 
    vendor        : {:?}
    version       : {:?} 
    sdkVersion    : {:?} 
"#, 
        self.cid, 
        self.cardinality, 
        category, 
        name, 
        self.classFlags, 
        subcategories, 
        vendor, version,
        sdkVersion
        )
        }
    }
}
impl Default for PClassInfo {
    fn default() -> Self {
        PClassInfo {
            cid         : [0; 16], 
            cardinality : 0, 
            category    : [0; 32], 
            name        : [0; 64]
        }
    }
}

impl Default for PClassInfo2 {
    fn default() -> Self {
        PClassInfo2 {
            cid         : [0; 16], 
            cardinality : 0, 
            category    : [0; 32], 
            name        : [0; 64],
            classFlags    : 0, 
            subcategories : [0; 128],
            vendor        : [0; 64],
            version       : [0; 64],
            sdkVersion    : [0; 64],
        }
    }
}