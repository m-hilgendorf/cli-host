use super::base::*;
use std::os::raw::c_void;

pub type TimerInterval = u64;
pub type FileDescriptor = i32;

#[repr(C)]
#[repr(align(16))]
pub struct ViewRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

//todo: const FIDString kPlatformTypeHWND = "HWND"
//todo: const FIDString kPlatformTypeNSView = "NSView"
//todo: const FIDString kPlatformTypeUIView = "UIView"
//todo: const FIDString kPlatformTypeX11EmbedWindowID = "X11EmbedWindowID"

RIDL!{#[iid(0x5BC32507, 0xD06049EA, 0xA6151B52, 0x2B755B29)]
    interface IPlugView (IPlugViewVtbl) : FUnknown(FUnknownVtbl) {
        fn isPlatformTypeSupported(platform : FIDString,) -> tresult,
        fn attached(parent : *mut c_void, platform : FIDString,) -> tresult,
        fn removed() -> tresult,
        fn onWheel(distance : f32,) -> tresult,
        fn onKeyDown(key : char16, keyCode : i16, modifiers : i16,) -> tresult,
        fn onKeyUp(key : char16, keyCode : i16, modifiers : i16, ) -> tresult,
        fn onSize(newSize : *mut ViewRect,) -> tresult,
        fn onFocus(state : TBool,) -> tresult,
        fn setFrame(frame : *mut c_void,) -> tresult,
        fn canResize() -> tresult,
        fn checkSizeConstraint(rect : *mut ViewRect,) -> tresult,
    }
}
RIDL!{#[iid(0x367FAF01, 0xAFA94693, 0x8D4DA2A0, 0xED0882A3)]
    interface IPlugFrame(IPlugFrameVtbl) : FUnknown(FUnknownVtbl) {
        fn resizeView(view : *mut IPlugView, newSize : *mut ViewRect,) -> tresult,
    }
}
RIDL!{#[iid(0x561E65C9, 0x13A0496F, 0x813A2C35, 0x654D7983)]
    interface IEventHandler(IEventHandlerVtbl) : FUnknown(FUnknownVtbl) {
        fn onFDIsSet(fd : FileDescriptor,) -> (),
    }
}
RIDL!{#[iid(0x10BDD94F, 0x41424774, 0x821FAD8F, 0xECA72CA9)]
    interface ITimerHandler (ITimerHandlerVtbl) : FUnknown(FUnknownVtbl) {
        fn onTimer() -> (),
    }
}