#![allow(non_upper_case_globals)]

// todo: vst3 version defines
use crate::base::*;

pub type TChar = char16;
pub type String128 = [TChar; 128];
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
pub const kNoParamID : ParamID = 0xffffffff;
pub type Sample32 = f32;
pub type Sample64 = f64;
pub type SampleRate = f64;
pub type SpeakerArrangement = u64;
pub type Speaker = u64;