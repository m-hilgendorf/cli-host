use crate::pluginterfaces::base::*;
use super::vsttypes::*;
use std::os::raw::c_void;

type AttrID = *const char8;
RIDL! {#[iid(0x1E5F0AEB, 0xCC7F4533, 0xA2544011, 0x38AD5EE4)]
    interface IAttributeList(IAttributeListVtbl) : FUnknown(FUnknownVtbl) {
        fn setInt (id : AttrID, value : i64,) -> tresult,
        fn getInt (id : AttrID, value : *mut i64,) -> tresult,
        fn setFloat (id : AttrID, value : f64,) -> tresult,
        fn getFloat (id : AttrID, value : *mut f64,) -> tresult,
        fn setString (id : AttrID, value : *const TChar, size : u32,) -> tresult,
        fn getString (id : AttrID, value : *mut TChar, size : u32,) -> tresult,
        fn setBinary (id : AttrID, data : *const c_void, size : u32,) -> tresult,
        fn getBinary (id : AttrID, data : *mut *const c_void, size : *mut u32,) -> tresult,
    }
}

RIDL!{#[iid(0xD6CE2FFC, 0xEFAF4B8C, 0x9E74F1BB, 0x12DA44B4)]
    interface IStreamAttributes(IStreamAttributesVtbl) : FUnknown(FUnknownVtbl) {
        fn getFileName (name : String128,) -> tresult,
        fn getAttributes () -> *mut IAttributeList,
    }
}