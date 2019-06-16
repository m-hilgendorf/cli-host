#![allow(non_upper_case_globals)]

use crate::pluginterfaces::base::*;
use super::vsttypes::*;

//todo: kDefaultFactoryFlags

use crate::pluginterfaces::vst::MediaType;
pub mod MediaTypes {
    pub const kAudio : i32 = 0;
    pub const kEvent: i32 = 1;
    pub const kNumMediaTypes : i32 = 2;
}

pub mod BusDirections {
    pub const kInput : i32 = 0;
    pub const kOutput : i32 = 1;
}

//todo: document this, since the API is different
pub mod BusFlags {
    pub const kDefaultActive : u32 = 1;
}

pub mod IoModes {
    pub const kSimple : i32 = 0;
    pub const kAdvanced : i32 = 1;
    pub const kOfflineProcessing: i32  = 2;
}

#[repr(C)]
#[repr(align(16))]
pub struct BusInfo {
    mediaType : MediaType,
    direction : BusDirection,
    channelLayout : i32,
    name : String128,
    busType : BusType,
    flags : u32
}

#[repr(C)]
#[repr(align(16))]
pub struct RoutingInfo {
    mediaType : MediaType,
    busIndex : i32,
    channel : i32
}
RIDL! {#[iid(0xE831FF31, 0xE831FF31, 0x928EBBEE, 0x25697802)]
    interface IComponent(IComponentVtbl) : IPluginBase(IPluginBaseVtbl) {
        fn getControllerClassID(classId : TUID,) -> tresult,
        fn setIoMode (mode : IoMode,) -> tresult,
        fn getBusCount(type_ : MediaType, dir : BusDirection,) -> i32,
        //todo: write tests for functions that take C++ references instead of C pointers
        fn getBusInfo (type_ : MediaType, dir : BusDirection, index : i32, busInfo : *mut BusInfo,) -> tresult,
        fn getRoutingInfo(inInfo : *mut RoutingInfo, outInfo : *mut RoutingInfo,) -> tresult,
        fn activateBus (type_ : MediaType, dir : BusDirection, index : i32, state : TBool,)-> tresult,
        fn setActive (state : TBool,)-> tresult,
	    fn setState (state : *mut IBStream,) -> tresult,
        fn getState (state : *mut IBStream,) -> tresult,
    }
}