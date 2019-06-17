use crate::pluginterfaces::base::*;
use super::vsttypes::*;
use crate::pluginterfaces::gui::*;
use crate::pluginterfaces::kNoParentUnitId;

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
    pub id         : ParamID,
    pub title      : String128,
    pub shortTitle : String128,
    pub units      : String128,
    pub stepCount  : i32,
    pub defaultNormalizedValue : ParamValue,
    pub unitId : UnitId,
    pub flags  : i32,
}

impl Default for ParameterInfo {
    fn default() -> Self {
        Self {
            id : kNoParamID,
            title : [0; 128],
            shortTitle : [0; 128],
            units : [0; 128],
            stepCount : -1,
            defaultNormalizedValue : 0.0,
            unitId : kNoParentUnitId,
            flags : 0
        }
    }
}
use std::fmt;
impl fmt::Debug for ParameterInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use widestring::U16CStr;
        use std::mem::transmute;
        unsafe {
            write!(f, r#"
    id : {}
    title : {:?}
    short title : {:?}
    units : {:?}
    step count : {}
    default : {}
    unit : {}
    flags: {:x}
    "#,
        self.id,
        U16CStr::from_ptr_str(std::mem::transmute(self.title.as_ptr())).to_string_lossy(),
        U16CStr::from_ptr_str(std::mem::transmute(self.shortTitle.as_ptr())).to_string_lossy(),
        U16CStr::from_ptr_str(std::mem::transmute(self.units.as_ptr())).to_string_lossy(),
        self.stepCount,
        self.defaultNormalizedValue,
        self.unitId,
        self.flags)
        }
    }
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

RIDL! {#[iid(0x93A0BEA3, 0x0BD045DB, 0x8E890B0C, 0xC1E46AC6)]
    interface IComponentHandler(IComponentHandlerVtbl) : FUnknown(FUnknownVtbl) {
        fn beginEdit(id : ParamID,) -> tresult,
        fn performEdit(id : ParamID, valueNormalized : ParamValue,) -> tresult,
        fn endEdit(id : ParamID,) -> tresult,
        fn restartComponent(flags : i32,) -> tresult,
    }
}
RIDL!{ #[iid(0xF040B4B3, 0xA36045EC, 0xABCDC045, 0xB4D5A2CC)]
    interface IComponentHandler2(IComponentHandler2Vtbl) : FUnknown(FUnknownVtbl) {
        fn setDirty(state :TBool,) -> tresult,
        // todo: verify that adhoc polymorphism doesn't break this smh
        fn requestOpenEditor(name : FIDString,) -> tresult,
        fn startGroupEdit() -> tresult,
        fn finishGroupEdit() -> tresult,
    }
}
RIDL! {#[iid(0x067D02C1, 0x5B4E274D, 0xA92D90FD, 0x6EAF7240)]
    interface IComponentHandlerBusActivation(IComponentHandlerBusActivationVtbl) : FUnknown(FUnknownVtbl) {
        fn requestBusActivation(type_ : MediaType, dir : BusDirection, index : i32, state : TBool,) -> tresult,
    }
}
RIDL! {#[iid(0xDCD7BBE3, 0x7742448D, 0xA874AACC, 0x979C759E)]
    interface IEditController(IEditControllerVtbl) : IPluginBase (IPluginBaseVtbl) {
        fn setComponentState (state : *mut IBStream,) -> tresult,
        fn setState (state : *mut IBStream,) -> tresult,
        fn getState (state : *mut IBStream,) -> tresult,
        fn getParameterCount () -> i32,
        fn getParameterInfo (index : i32, info : *mut ParameterInfo,) -> tresult,
        fn getParamStringByValue (id : ParamID, value : ParamValue, string : *mut TChar,) -> tresult,
        fn getParamValueByString (id : ParamID, string : *mut TChar, value : *mut ParamValue,) -> tresult,
        fn normalizedParamToPlain (id : ParamID, normalized : ParamValue,) -> ParamValue,
        fn plainParamToNormalized (id : ParamID, plain : ParamValue,) -> ParamValue,
        fn getParamNormalized (id : ParamID,) -> ParamValue,
        fn setParamNormalized (id : ParamID, value :ParamValue,) -> tresult,
        fn setComponentHandler (handler : *mut IComponentHandler,) -> tresult,
        fn createView (name : FIDString,) -> *mut IPlugView,
    }
}
RIDL! {#[iid(0x7F4EFE59, 0xF3204967, 0xAC27A3AE, 0xAFB63038)]
    interface IEditController2 (IEditController2Vtbl) : FUnknown(FUnknownVtbl) {
        fn setKnobMode(mode :KnobMode,) -> tresult,
        fn openHelp(onlyCheck : TBool,) -> tresult,
        fn openAboutBox(onlyCheck : TBool,) -> tresult,
    }
}
RIDL! {#[iid(0xDF0FF9F7, 0x49B74669, 0xB63AB732, 0x7ADBF5E5)]
    interface IMidiMapping(IMidiMappingVtbl) : FUnknown(FUnknownVtbl) {
        fn getMidiControllerAssignment (busINdex : i32, channel : i16, midiControllerNumber : CtrlNumber, id : *mut ParamID,) -> tresult,
    }
}

RIDL! {#[iid(0xC1271208, 0x70594098, 0xB9DD34B3, 0x6BB0195E)]
    interface IEditControllerHostEditing(IEditControllerHostEditingVtbl) : FUnknown(FUnknownVtbl) {
        fn beginEditFromHost (paramId : ParamID,) -> tresult,
        fn endEditFromHost (paramId : ParamID,) -> tresult,
    }
}