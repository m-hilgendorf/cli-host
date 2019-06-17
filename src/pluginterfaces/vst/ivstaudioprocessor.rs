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
    pub const kDistributable       : i32 = 1;
    pub const kSimpleModeSupported : i32 = 2;
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
    pub processMode        : i32,
    pub symbolicSampleSize : i32,
    pub maxSamplesPerblock : i32,
    pub sampleRate         : f64
}

#[repr(C)]
#[repr(align(16))]
pub union AudioBusBuffersInner{
    pub channelBuffers32 : *mut *mut f32,
    pub channelBuffers64 : *mut *mut f64
}

#[repr(C)]
#[repr(align(16))]
pub struct AudioBusBuffers {
    pub numChannels  : i32,
    pub silenceFlags : u32,
    pub buffers      : AudioBusBuffersInner
}

#[repr(C)]
#[repr(align(16))]
pub struct ProcessData {
    pub processMode        : i32,
    pub symbolicSampleSize : i32,
    pub numSamples : i32,
    pub numInputs  : i32,
    pub numOutputs : i32,
    pub inputs  : *mut AudioBusBuffers,
    pub outputs : *mut AudioBusBuffers,
    pub inputParameterChanges  : *mut IParameterChanges,
    pub outputParameterChanges : *mut IParameterChanges,
    pub inputEvents    : *mut IEventList,
    pub outputEvents   : *mut IEventList,
    pub processContext : *mut ProcessContext,
}

RIDL!{#[iid(0x42043F99, 0xB7DA453C, 0xA569E79D, 0x9AAEC33D)]
    interface IAudioProcessor(IAudioProcessorVtbl) : FUnknown(FUnknownVtbl) {
        fn setBusArrangements(inputs : *mut SpeakerArrangement, numIns : i32,
                              outputs : *mut SpeakerArrangement, numOuts : i32,)-> tresult,
        fn getBusArrangement (dir : BusDirection, index : i32, arr : *mut SpeakerArrangement,) -> tresult,
        fn canProcessSampleSize(symbolixSampleSize : i32,) -> tresult,
        fn getLatencySamples () -> i32,
        fn setupProcessing (setup : *mut ProcessSetup,) -> tresult,
        fn setProcessing (state : TBool,)-> tresult,
        fn process (data : *mut ProcessData,) -> tresult,
        fn getTailSamples() -> i32,
    }
}