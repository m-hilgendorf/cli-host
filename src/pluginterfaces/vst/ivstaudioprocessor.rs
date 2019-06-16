use crate::pluginterfaces::base::*;
use super::ivstcomponent::*;
use super::vsttypes::*;
use super::ivstprocesscontext::*;
use super::ivstevents::*;
use super::ivstparameterchanges::*;

/// Distributable means the plugin's editor doesn't
/// have to be in the same address space as the processor.
/// Pretty much completely unsupported by any host.
pub mod ComponentFlags {
    pub const kDistributable : i32 = 1;
    pub const kSimpleModesupported : i32 = 2;
}

/// Flags for representing f32/f64 processing modes
pub mod SymbolicSampleSizes {
    pub const kSample32 : i32 = 0;
    pub const kSample64 : i32 = 1;
}

pub mod ProcessModes {
    /// audio rendered in realtime, no locking/blocking
    pub const kRealtime : i32 = 0;

    /// audio rendered _faster_ than realtime, meaning two
    /// subsequent calls to processBlock may overlap
    pub const kPrefetch : i32 = 1;

    /// audio rendered offline, ok to lock
    pub const kOffline  : i32 = 2;
}

pub const kNoTail : u32 = 0;
pub const kInfiniteTail : u32 = kMaxInt32u;

#[repr(C)]
#[repr(align(16))]
pub struct ProcessSetup {
    processMode : i32,
    symbolicSampleSize : i32,
    maxSamplesPerblock : i32,
    sampleRate : f64
}

#[repr(C)]
#[repr(align(16))]
pub union AudioBusBuffersInner{
    channelBuffers32 : *mut *mut f32,
    channelBuffers64 : *mut *mut f64
}

#[repr(C)]
#[repr(align(16))]
pub struct AudioBusBuffers {
    numChannels : i32,
    silenceFlags : u32,
    buffers : AudioBusBuffersInner
}

#[repr(C)]
#[repr(align(16))]
pub struct ProcessData {
    processMode : i32,
    symbolicSampleSize : i32,
    numSamples : i32,
    numInputs : i32,
    numOutputs : i32,
    inputs : *mut AudioBusBuffers,
    outputs : *mut AudioBusBuffers,
    inputParameterChanges : *mut IParameterChanges,
    outputParameterChanges : *mut IParameterChanges,
    inputEvents : *mut IEventList,
    outputEvents : *mut IEventList,
    processContext : *mut ProcessContext,
}

RIDL!{#[uuid(0x42043F99, 0xB7DA453C, 0xA569E79D, 0x9AAEC33D)]
    interface IAudioProcessor(IAudioProcessorVtbl) : FUnknown(FUnknownVtbl) {
        fn setBusArrangements(inputs : *mut SpeakerArrangement, numIns : i32,
                              outputs : *mut SpeakerArrangement, numOuts : i32,)-> tresult,
        fn getBusArrangement (dir : BusDirection, index : i32, arr : *mut SpeakerArrangement,) -> tresult,
        fn canProcesssampleSize(symbolixSampleSize : i32,) -> tresult,
        fn getLatencySamples () -> i32,
        fn setupProcessing (setup : *mut ProcessSetup,) -> tresult,
        fn setProcessing (state : TBool,)-> tresult,
        fn process (data : *mut ProcessData,) -> tresult,
        fn getTailSamples() -> i32,
    }
}