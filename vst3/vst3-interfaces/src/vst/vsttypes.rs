#![allow(non_upper_case_globals)]
use std::str::FromStr;
// todo: vst3 version defines
use crate::base::*;

pub type TChar = char16;
#[derive(Copy, Clone)]
pub struct String128 ([TChar; 128]);
pub type MediaType = i32;
pub type BusDirection = i32;
pub type BusType = i32;
pub type IoMode = i32;
pub type UnitId = i32;
pub type ParamValue = f64;
pub type ParamID = u32;
pub type ProgramListID = i32;
pub type CtrlNumber = i16;
pub type TQuarterNotes = f64;
pub type TSamples = i64;
pub type ColorSpec = u32;
pub const kNoParamID: ParamID = 0xffffffff;
pub type Sample32 = f32;
pub type Sample64 = f64;
pub type SampleRate = f64;
pub type SpeakerArrangement = u64;
pub type Speaker = u64;

use widestring::{U16String, U16CStr};
//todo: make this not allocate
impl String128 {
    pub fn from_str(s : &str) -> Self{
        let wstr = U16String::from_str(s);
        let raw = (wstr.as_ptr(), wstr.len());
        let mut s = [0; 128];
        let t = unsafe { std::slice::from_raw_parts(raw.0 as *const _ as *const TChar, raw.1) };
        for (s, t) in s.iter_mut().zip(t.iter()) { *s = *t };
        let _ = wstr.into_boxed_ustr();
        Self(s)
    }
}
use std::fmt;
impl fmt::Debug for String128 {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        let s = unsafe { U16CStr::from_ptr_str(self.0.as_ptr() as *const _ as *const u16).to_string_lossy() };
        write!(f, "{}", s)
    }
}