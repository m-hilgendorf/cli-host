#![allow(unused_imports)]
use std::cmp::min;
use std::ffi::CStr;
use std::mem::transmute;
use std::os::raw::c_void;
use std::ptr::copy_nonoverlapping as memcpy;
use vst3_interfaces::*;
use widestring::{U16CStr, U16String};

pub use vst3_interfaces::*;
#[doc(hidden)]
pub mod boilerplate;

pub trait PluginFactory {
    fn get_factory_info(&self) -> Result<PFactoryInfo, tresult>;
    fn count_classes(&self) -> usize;
    fn get_class_info(&self, idx: usize) -> Result<PClassInfo, tresult>;
    fn create_instance(&mut self, cid: FIDString, iid: FIDString) -> Result<*mut c_void, tresult>;
    fn get_class_info2(&self) -> Result<PClassInfo2, tresult>;
}
pub trait ComponentHandler {
    fn begin_edit(&mut self, id: ParamID) -> Result<(), tresult>;
    fn end_edit(&mut self, id: ParamID) -> Result<(), tresult>;
    fn perform_edit(&mut self, id: ParamID, normalized: f64) -> Result<(), tresult>;
    fn restart_component(&mut self, flags: i32) -> Result<(), tresult>;
    fn set_dirty(&mut self, is_dirty: bool) -> Result<(), tresult>;
    fn request_open_editor(&mut self) -> Result<(), tresult>;
    fn start_group_edit(&mut self) -> Result<(), tresult>;
    fn finish_group_edit(&mut self) -> Result<(), tresult>;
}

pub trait EventList {
    fn get_event_count(&self) -> usize;
    fn get_event(&self, index: usize) -> Result<Event, tresult>;
    fn add_event(&mut self, event: Event) -> Result<(), tresult>;
}

pub trait PluginBase {
    fn initialize(&mut self, host: VstPtr<FUnknown>) -> Result<(), tresult>;
    fn terminate(&mut self) -> Result<(), tresult>;
}

pub trait Component {
    fn set_io_mode(&mut self, id: i32) -> Result<(), tresult>;
    fn get_bus_count(&self, media: MediaType, dir: BusDirection) -> usize;
    fn get_bus_info(
        &self,
        media: MediaType,
        dir: BusDirection,
        idx: usize,
    ) -> Result<BusInfo, tresult>;
    fn get_routing_info(
        &self,
        inpt: &mut RoutingInfo,
        outp: &mut RoutingInfo,
    ) -> Result<(), tresult>;
    fn activate_bus(
        &mut self,
        media: MediaType,
        dir: BusDirection,
        idx: usize,
        state: bool,
    ) -> Result<(), tresult>;
    fn set_active(&mut self, state: bool) -> Result<(), tresult>;
    fn set_state(&mut self, str: VstPtr<IBStream>) -> Result<(), tresult>;
    fn get_state(&mut self, str: VstPtr<IBStream>) -> Result<(), tresult>;
}

pub trait AudioProcessor {
    fn set_bus_arrangements(
        &self,
        ins: &[SpeakerArrangement],
        outs: &[SpeakerArrangement],
    ) -> Result<(), tresult>;
    fn get_bus_arrangement(
        &self,
        dir: BusDirection,
        idx: usize,
    ) -> Result<SpeakerArrangement, tresult>;
    fn can_do_32(&self) -> bool {
        true
    }
    fn can_do_64(&self) -> bool {
        false
    }
    fn setup_processing(&mut self, setup: &ProcessSetup) -> Result<(), tresult>;
    fn set_processing(&mut self, state: bool) -> Result<(), tresult>;
    fn process(&mut self, data: &mut ProcessData) -> Result<(), tresult>;
    fn get_latency_samples(&self) -> u32;
    fn get_tail_samples(&self) -> u32;
}

pub trait EditController {
    fn set_component_state(&mut self, state: VstPtr<IBStream>) -> Result<(), tresult>;
    fn set_state(&mut self, state: VstPtr<IBStream>) -> Result<(), tresult>;
    fn get_state(&self, state: VstPtr<IBStream>) -> Result<(), tresult>;
    fn get_parameter_count(&self) -> usize;
    fn get_parameter_info(&self, idx: usize) -> Result<ParameterInfo, tresult>;
    fn get_param_string_by_value(
        &self,
        id: ParamID,
        value: f64,
        string: &mut str,
    ) -> Result<(), tresult>;
    fn normalized_param_to_plain(&self, id: ParamID, value: f64) -> f64;
    fn plain_param_to_normalized(&self, id: ParamID, value: f64) -> f64;
    fn get_param_value_by_string(&self, id: ParamID, string: &str) -> Result<f64, tresult>;
    fn get_param_normalized(&self, id: ParamID) -> f64;
    fn set_param_normalized(&mut self, id: ParamID, value: f64) -> Result<(), tresult>;
    fn set_component_handler(&mut self, handler: VstPtr<IComponentHandler>) -> Result<(), tresult>;
    fn create_view(&mut self) -> Option<VstPtr<IPlugView>>;
}
