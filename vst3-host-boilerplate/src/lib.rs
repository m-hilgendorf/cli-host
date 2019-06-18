use vst3_interfaces::*;
use std::os::raw::c_void;
use std::ffi::CStr;
use widestring::{U16CStr, U16String};
use std::mem::transmute;
use std::ptr::copy_nonoverlapping as memcpy;
use std::cmp::min;

#[doc(hidden)]
pub mod boilerplate;

/// Enum variant for AttributeList
pub enum AttributeValue<'a> {
    Float(f64),
    Int(i64),
    String(&'a str),
    Binary(&'a [u8]),
}

/// Provides an interface for getting/setting variables
/// in the host using key/value pairs
pub trait AttributeList {
    fn set(&mut self, id : &str, value : AttributeValue) -> Result<(), tresult>;
    fn get<'a> (&self, id : &str) -> Result<AttributeValue<'a>, tresult>;
    fn iid() -> TUID { IAttributeList::iid() }
}

pub trait ComponentHandler {
    fn begin_edit(&mut self, id: ParamID) -> Result<(), tresult>;
    fn end_edit(&mut self, id: ParamID) -> Result<(), tresult>;
    fn perform_edit(&mut self, id: ParamID, normalized: f64) -> Result<(), tresult>;
    fn restart_component(&mut self, flags : i32) -> Result<(), tresult>;
    fn iid() -> TUID { IComponentHandler::iid() }
}

pub trait ComponentHandler2 {
    fn set_dirty(&mut self, is_dirty : bool) -> Result<(), tresult>;
    fn request_open_editor(&mut self, ) -> Result<(), tresult>;
    fn start_group_edit(&mut self, ) -> Result<(), tresult>;
    fn finish_group_edit(&mut self, ) -> Result<(), tresult>;
    fn iid() -> TUID { IComponentHandler::iid() }
}

pub trait EventList {
    fn get_event_count(&self) -> usize;
    fn get_event(&self, index : usize) -> Result<Event, tresult>;
    fn add_event(&mut self, event : Event) -> Result<(), tresult>;
}

pub trait HostApplication {

}

pub trait ComponentHandlerBusActivation {

}

pub trait Message {

}

pub trait ParamValueQueue {

}

pub trait ParameterChanges {

}

pub trait UnitHandler {

}

pub trait PlugFrame {

}

/*
todo: pub trait StreamAttributes {}
todo: pub trait InfoListener {}
todo: pub trait ContextMenu {}
todo: pub trait ContextMenuTarget{}
*/