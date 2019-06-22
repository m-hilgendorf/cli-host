#![allow(non_upper_case_globals)]
use super::vsttypes::*;

pub mod FrameRateFlags {
    pub const kPullDownRate: u32 = 0;
    pub const kDropRate: u32 = 1;
}

pub mod ChordMasks {
    pub const kChordMask: i16 = 0x0fff;
    pub const kReservedMask: i16 = -4096i16;
}

pub mod StatesAndFlags {
    pub const kPlaying: u32 = 1 << 1;
    pub const kCycleActive: u32 = 1 << 2;
    pub const kRecording: u32 = 1 << 3;
    pub const kSystemTimeValid: u32 = 1 << 8;
    pub const kContTimeValid: u32 = 1 << 17;
    pub const kProjectTimeMusicValid: u32 = 1 << 9;
    pub const kBarPositionValid: u32 = 1 << 11;
    pub const kCycleValid: u32 = 1 << 12;
    pub const kTempoValid: u32 = 1 << 10;
    pub const kTimeSigValid: u32 = 1 << 13;
    pub const kChordValid: u32 = 1 << 18;
    pub const kSmpteValid: u32 = 1 << 14;
    pub const kClockValid: u32 = 1 << 15;
}

#[repr(C)]
#[repr(align(16))]
pub struct FrameRate {
    pub framesPerSecond: u32,
    pub flags: u32,
}

#[repr(C)]
#[repr(align(16))]
pub struct Chord {
    pub keyNote: u8,
    pub rootNote: u8,
    pub chordMask: i16,
}

#[repr(C)]
#[repr(align(16))]
pub struct ProcessContext {
    state: u32,
    sampleRate: f64,
    projectTimeSamples: TSamples,
    systemTime: i64,
    continuousTimeSamples: f64,
    projectTimeMusic: TQuarterNotes,
    barPositionMusic: TQuarterNotes,
    cycleStartMusic: TQuarterNotes,
    cycleEndMusic: TQuarterNotes,
    tempo: f64,
    timeSigNumerator: i32,
    timeSigDenominator: i32,
    chord: Chord,
    smpteOfffsetSubframes: i32,
    frameRate: FrameRate,
    samplesToNextClock: i32,
}
