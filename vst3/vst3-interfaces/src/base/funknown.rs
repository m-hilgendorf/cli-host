use std::os::raw::c_void;
use super::ftypes::*;

pub type LARGE_INT = i64;
pub type TUID      = [i8; 16];
//pub type String    = [char8; 64]; 
//DECLARE_CLASS_IID (FUnknown, 0x00000000, 0x00000000, 0xC0000000, 0x00000046)
RIDL!{#[iid(0x00000000, 0x00000000, 0xC0000000, 0x00000046)]
    interface FUnknown(FUnknownVtbl) {
    fn queryInterface(
        iid: *const i8,
        obj: *mut *mut c_void,
    ) -> tresult,
    fn addRef()  -> u32,
    fn release() -> u32,
}}

//todo: Figure out how to deal with FUID