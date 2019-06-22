//! Copyright 2019 Michael Hilgendorf <mike@hilgendorf.audio>
//! Copyright 2008-2019 Steinberg Media Technologies, GMBH
//!
//! Licensed under the terms of the GNU General Public License (GPLv3) which
//! can be found [here]();
//!
//! The `vst3` crate is a port of the VST3 SDK developed by Steinberg Media
//! Technologies for creating audio plugins.
//!
//! This is somewhere between a work in progress and proof of concept that VST3 can be
//! ported to pure Rust from the pure C++ API.
//!
//! VST3 plugins need to be compiled as shared libraries (`cstdlib` in your `Cargo.toml`)
//! The shared library must have a single entry point defined as follows:
//!
//! ```rust
//! use vst3_interfaces::IPluginFactory;
//! pub extern "system" fn GetPluginFactory() -> *mut IPluginFactory {
//!     /* ... */
//! }
//! ```
//!
//! On licensing: this is not a clean room re-implementation of the VST3 API, it was
//! developed with full knowledge and use of the original VST3 API and its documentation.
//! The original API is distributed under a dual proprietary/GPL license, and this is distributed
//! under the latter to comply.

use std::cmp::min;
use std::ffi::CStr;
use std::mem::transmute;
use std::os::raw::c_void;
use std::ptr::copy_nonoverlapping as memcpy;
use vst3_interfaces::*;
use widestring::{U16CStr, U16String};

#[doc(hidden)]
pub mod boilerplate;

/// The plugin factory is the main interface for the shared library,
/// it controls the listing and constructing of objects by the host.
pub trait PluginFactory {
    /// Return information about the plugin factory
    fn get_factory_info(&self) -> Result<PFactoryInfo, tresult>;
    /// return the number of classes this factory can construct
    fn count_classes(&self) -> usize;
    /// Get information about a particular class
    fn get_class_info(&self, idx: usize) -> Result<PClassInfo, tresult>;
    /// Create an instance of a specific class
    fn create_instance(&mut self, cid: FIDString, iid: FIDString) -> Result<*mut c_void, tresult>;
}

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
    /// Set an attribute (key/value pair)
    fn set(&mut self, id: &str, value: AttributeValue) -> Result<(), tresult>;
    /// Get an attribute (key/value pair);
    fn get<'a>(&self, id: &str) -> Result<AttributeValue<'a>, tresult>;
    fn iid() -> TUID {
        IAttributeList::iid()
    }
}

/// The `ComponentHandler` interface alerts the host about changes to the
/// plugin parameters/state originating from the `EditController` instance.
pub trait ComponentHandler {
    /// Call before changing a parameter
    fn begin_edit(&mut self, id: ParamID) -> Result<(), tresult>;
    /// Call when finished changing a parameter
    fn end_edit(&mut self, id: ParamID) -> Result<(), tresult>;
    /// Call to change a parameter.
    fn perform_edit(&mut self, id: ParamID, normalized: f64) -> Result<(), tresult>;
    /// Used to alert the host for other changes, like a different parameter layout
    /// or bus changes.
    fn restart_component(&mut self, flags: i32) -> Result<(), tresult>;
    fn iid() -> TUID {
        IComponentHandler::iid()
    }
}

pub trait ComponentHandler2 {
    fn set_dirty(&mut self, is_dirty: bool) -> Result<(), tresult>;
    fn request_open_editor(&mut self) -> Result<(), tresult>;
    fn start_group_edit(&mut self) -> Result<(), tresult>;
    fn finish_group_edit(&mut self) -> Result<(), tresult>;
    fn iid() -> TUID {
        IComponentHandler::iid()
    }
}

pub trait EventList {
    fn get_event_count(&self) -> usize;
    fn get_event(&self, index: usize) -> Result<Event, tresult>;
    fn add_event(&mut self, event: Event) -> Result<(), tresult>;
}

pub trait HostApplication {}

pub trait ComponentHandlerBusActivation {}

pub trait Message {}

pub trait ParamValueQueue {}

pub trait ParameterChanges {}

pub trait UnitHandler {}

pub trait PlugFrame {}

/*
todo: pub trait StreamAttributes {}
todo: pub trait InfoListener {}
todo: pub trait ContextMenu {}
todo: pub trait ContextMenuTarget{}
*/
