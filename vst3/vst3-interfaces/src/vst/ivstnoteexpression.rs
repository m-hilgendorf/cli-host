#![allow(non_upper_case_globals)]

use super::vsttypes::*;

pub type NoteExpressionTypeID = u32;
pub type NoteExpressionValue = f64;

pub mod NoteExpressionTypeIDs {
    const kVolumeTypeID: u32 = 0;
    const kPanTypeID: u32 = 1;
    const kTuningTypeID: u32 = 2;
    const kVibratoTypeID: u32 = 3;
    const kExpressionTypeID: u32 = 4;
    const kBrightnessTypeID: u32 = 5;
    const kTextTypeID: u32 = 6;
    const kPhonemeTypeID: u32 = 7;
    const kCustomStart: u32 = 10000;
    const kCustomEnd: u32 = 20000;
    const kInvalidTypeID: u32 = 0xFFFFFFFF;
}

pub mod NoteExpressionFlags {
    pub const kIsBipolar: i32 = 1;
    pub const kIsOneShot: i32 = 2;
    pub const kIsAbsolute: i32 = 4;
    pub const kAssocatedParameterIDValid: i32 = 8;
}

#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone, Debug)]
pub struct NoteExpressionValueDescription {
    pub defaultValue: NoteExpressionValue,
    pub minimum: NoteExpressionValue,
    pub maximum: NoteExpressionValue,
    pub stepCount: i32,
}

#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone, Debug)]
pub struct NoteExpressionValueEvent {
    pub typeId: u32,
    pub noteId: i32,
    pub value: NoteExpressionValueDescription,
}

#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone, Debug)]
pub struct NoteExpressionTextEvent {
    pub typeId: u32,
    pub noteId: i32,
    pub textLen: u32,
    pub text: *const TChar,
}

#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone)]
pub struct NoteExpressionTypeInfo {
    pub typeId: u32,
    pub title: String128,
    pub shortTitle: String128,
    pub units: String128,
    pub unitId: i32,
    pub valueDesc: NoteExpressionValueDescription,
    pub associatedParameterID: ParamID,
    pub flags: i32,
}
