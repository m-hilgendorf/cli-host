use crate::pluginterfaces::base::*;
use super::vsttypes::*;
use crate::pluginterfaces::gui::*;

// todo: kVstComponentControllerClass
pub type KnobMode = i32;
pub mod ParameterFlags {
    pub const kNoFlags : i32 = 0;
    pub const kCanAutomate : i32 = 1;
    pub const kIsReadonly : i32 = 1 << 1;
    pub const kIsWrapAaround: i32 = 1 << 2;
    pub const kIsProgramChange : i32 = 1 << 15;
    pub const kIsBypass : i32 = 1 << 16;
}

pub mod KnobModes {
    pub const kCircularMode : i32 = 0;
    pub const kRelativCircularMode : i32 = 1;
    pub const kLinearMode : i32 = 2;
}

#[repr(C)]
#[repr(align(16))]
pub struct ParameterInfo {
    id : ParamID,
    title : String128,
    shortTitle : String128,
    units : String128,
    stepCount : i32,
    defaultNormalizedValue : ParamValue,
    unitId : UnitId,
    flags : i32,
}

// todo: Vst::ViewType::kEditor = "editor"
pub mod RestartFlags {
    pub const kReloadComponent : i32 = 1 << 0;
    pub const kIoChanges : i32 = 1 << 1;
    pub const kParamValuesChanged : i32 = 1 << 2;
    pub const kLatencyChanged : i32 = 1 << 3;
    pub const kParamTitlesChanged : i32 = 1 << 4;
    pub const kMidiCCAssignmentChanged : i32 = 1 << 5;
    pub const kNoteExpressionChanged : i32 = 1 << 6;
    pub const kIoTitlesChanged : i32 = 1 << 7;
    pub const kPrefetchableSupportChanged : i32 = 1 << 8;
    pub const kRoutingInfoChanged: i32 = 1 << 9;
}

RIDL! {#[uuid(0x93A0BEA3, 0x0BD045DB, 0x8E890B0C, 0xC1E46AC6)]
    interface IComponentHandler(IComponentHandlerVtbl) : FUnknown(FUnknownVtbl) {
        fn beginEdit(id : ParamID,) -> tresult,
        fn performEdit(id : ParamID, valueNormalized : ParamValue,) -> tresult,
        fn endEdit(id : ParamID,) -> tresult,
        fn restartComponent(flags : i32,) -> tresult,
    }
}
RIDL!{ #[uuid(0xF040B4B3, 0xA36045EC, 0xABCDC045, 0xB4D5A2CC)]
    interface IComponentHandler2(IComponentHandler2Vtbl) : FUnknown(FUnknownVtbl) {
        fn setDirty(state :TBool,) -> tresult,
        // todo: verify that adhoc polymorphism doesn't break this smh
        fn requestOpenEditor(name : FIDString,) -> tresult,
        fn startGroupEdit() -> tresult,
        fn finishGroupEdit() -> tresult,
    }
}
RIDL! {#[uuid(0x067D02C1, 0x5B4E274D, 0xA92D90FD, 0x6EAF7240)]
    interface IComponentHandlerBusActivation(IComponentHandlerBusActivationVtbl) : FUnknown(FUnknownVtbl) {
        fn requestBusActivation(type_ : MediaType, dir : BusDirection, index : i32, state : TBool,) -> tresult,
    }
}
RIDL! {#[uuid(0xDCD7BBE3, 0x7742448D, 0xA874AACC, 0x979C759E)]
    interface IEditController(IEditControllerVtbl) : IPluginBase (IPluginBaseVtbl) {
        fn setComponentState (state : *mut IBStream,) -> tresult,
        fn setState (state : *mut IBStream,) -> tresult,
        fn getState (state : *mut IBStream,) -> tresult,
        fn getParameterCount () -> i32,
        fn getParameterInfo (index : i32, info : *mut ParameterInfo,) -> tresult,

        //todo: double check the ABI for IEditController::getParamStringByValue
        fn getParamStringByValue (id : ParamID, value : ParamValue, string : String128,) -> tresult,
        fn getParamValueByString (id : ParamID, string : *mut TChar, value : *mut ParamValue,) -> tresult,
        fn normalizedParamToPlain (id : ParamID, normalized : ParamValue,) -> ParamValue,
        fn plainParamToNormalized (id : ParamID, plain : ParamValue,) -> ParamValue,
        fn getParamNormalized (id : ParamID,) -> ParamValue,
        fn setParamNormalized (id : ParamID, value :ParamValue,) -> tresult,
        fn setComponentHandler (handler : *mut IComponentHandler,) -> tresult,
        fn createView (name : FIDString,) -> *mut IPlugView,
    }
}
RIDL! {#[uuid(0x7F4EFE59, 0xF3204967, 0xAC27A3AE, 0xAFB63038)]
    interface IEditController2 (IEditController2Vtbl) : FUnknown(FUnknownVtbl) {
        fn setKnobMode(mode :KnobMode,) -> tresult,
        fn openHelp(onlyCheck : TBool,) -> tresult,
        fn openAboutBox(onlyCheck : TBool,) -> tresult,
    }
}
RIDL! {#[uuid(0xDF0FF9F7, 0x49B74669, 0xB63AB732, 0x7ADBF5E5)]
    interface IMidiMapping(IMidiMappingVtbl) : FUnknown(FUnknownVtbl) {
        fn getMidiControllerAssignment (busINdex : i32, channel : i16, midiControllerNumber : CtrlNumber, id : *mut ParamID,) -> tresult,
    }
}

RIDL! {#[uuid(0xC1271208, 0x70594098, 0xB9DD34B3, 0x6BB0195E)]
    interface IEditControllerHostEditing(IEditControllerHostEditingVtbl) : FUnknown(FUnknownVtbl) {
        fn beginEditFromHost (paramId : ParamID,) -> tresult,
        fn endEditFromHost (paramId : ParamID,) -> tresult,
    }
}