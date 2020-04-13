use super::ftypes::*;
use std::os::raw::c_void;

pub const NO_INTERFACE: tresult = -1;
pub const OK: tresult = 0;
pub const TRUE: tresult = 0;
pub const FALSE: tresult = 1;
pub const INVALID_ARG: tresult = 2;
pub const NOT_IMPLEMENTED: tresult = 3;
pub const INTERNAL_ERROR: tresult = 4;
pub const NOT_INITIALIZED: tresult = 5;
pub const OUT_OF_MEMORY: tresult = 6;

pub type LARGE_INT = i64;
pub type TUID = [i8; 16];
//pub type String    = [char8; 64];
//DECLARE_CLASS_IID (FUnknown, 0x00000000, 0x00000000, 0xC0000000, 0x00000046)
RIDL! {#[iid(0x00000000, 0x00000000, 0xC0000000, 0x00000046)]
    interface FUnknown(FUnknownVtbl) {
    fn queryInterface(
        iid: *const i8,
        obj: *mut *mut c_void,
    ) -> tresult,
    fn addRef()  -> u32,
    fn release() -> u32,
}}

//todo: Figure out how to deal with FUID
