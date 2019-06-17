use super::ivstprocesscontext::*;
use super::ivstnoteexpression::*;
use crate::vst::TQuarterNotes;
use crate::base::*;
use super::vsttypes::*;

#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone,Debug)]
pub struct NoteOnEvent {
    channel : i16,
    pitch : i16,
    tuning : f32,
    velocity : f32,
    length : i32,
    noteId : i32
}

#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone,Debug)]
pub struct NoteOffEvent {
    channel : i16,
    pitch : i16,
    velocity : f32,
    noteId : i32,
    tuning : f32,
}

#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone,Debug)]
pub struct DataEvent {
    size : u32,
    type_ : u32,
    bytes : *const u8,
}

#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone,Debug)]
pub struct PolyPressureEvent {
    channel : i16,
    pitch : i16,
    pressure : f32,
    noteId : i32
}

#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone,Debug)]
pub struct ChordEvent {
    root : i16,
    bassNote : i16,
    mask : i16,
    textLen : u16,
    text : *const TChar
}

#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone,Debug)]
pub struct ScaleEvent {
    root : i16,
    mask : i16,
    textLen : u16,
    text : *const TChar
}

#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone,Debug)]
pub struct LegacyMidiCCOutEvent {
    controlNumber : u8,
    channel : i8,
    value : i8,
    value2 : i8
}

#[repr(C)]
#[repr(align(16))]
pub union EventUnion {
    noteOn : NoteOnEvent,
    noteOff : NoteOffEvent,
    data : DataEvent,
    polyPressure : PolyPressureEvent,
    noteExpressionValue : NoteExpressionValueEvent,
    noteExpressionText : NoteExpressionTextEvent,
    chord : ChordEvent,
    scale : ScaleEvent,
    midiCCOut : LegacyMidiCCOutEvent
}

pub mod EventTypes {
    const kNoteOnEvent : u16 = 0;
    const kNoteOff : u16 = 1;
    const kDataEvent : u16 = 2;
    const kPolyPressureEvent : u16 = 3;
    const kNoteExpressionValueEvent : u16 = 4;
    const kNoteExpressionTextEvent : u16 = 5;
    const kChordEvent : u16 = 6;
    const kScaleEvent : u16 = 7;
    const kLegacyMidiCCOutEvent : u16 = 65535;
}

#[repr(C)]
#[repr(align(16))]
pub struct Event {
    busIndex : i16,
    sampleOffset : i32,
    ppqPosition : TQuarterNotes,
    flags : u16,
    type_ : u16,
    inner : EventUnion,
}

RIDL!{#[iid(0x3A2C4214, 0x346349FE, 0xB2C4F397, 0xB9695A44)]
    interface IEventList(IEventListVtbl) : FUnknown(FUnknownVtbl) {
        fn getEventCount() -> i32,
        fn getEvent(index : i32, e : *mut Event,) -> tresult,
        fn addEvent(e : *mut Event,) -> tresult,
    }
}