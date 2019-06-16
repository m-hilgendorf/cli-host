#![allow(non_upper_case_globals)]

use super::ftypes::*;
pub const kIBSeakSet : i32 = 0;
pub const kIBSeakCut : i32 = 1;
pub const kIBSeakEnd : i32 = 2;
use std::os::raw::c_void;
use super::funknown::*;

RIDL! {#[iid(0xC3BF6EA2, 0x30994752, 0x9B6BF990, 0x1EE33E9B)]
    interface IBStream(IBStreamVtbl) : FUnknown(FUnknownVtbl) {
    	fn read(buffer : *mut c_void, numBytes : i32, numBytesRead : *mut i32,) -> tresult,
        fn write(buffer : *mut c_void, numBytes : i32, numBytesWritten : *mut i32,) -> tresult,
        fn seek(pos : i64, mode : i32, result : *mut i64,) -> tresult,
        fn tell (pos : *mut i64,) -> tresult,
    }
}