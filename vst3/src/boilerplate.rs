use super::*;
use std::slice;
use vst3_impl::*;
use vst3_interfaces::vst::SymbolicSampleSizes::kSample32;
use widestring::U16CString;
#[doc(hidden)]
#[repr(C)]
#[derive(Vst3Impl)]
#[interfaces(IComponentHandler, IComponentHandler2)]
pub struct ComponentHandlerImpl<T>
where
    T: ComponentHandler + Interface,
{
    vtbl: VTable<IComponentHandlerVtbl>,
    refcount: Refcount,
    pimpl: VstPtr<T>,
}

impl<T> ComponentHandlerImpl<T>
where
    T: ComponentHandler + Interface,
{
    pub fn new(pimpl: VstPtr<T>) -> VstPtr<IComponentHandler> {
        unsafe { VstPtr::from_raw(Self::create_raw(pimpl) as *mut _) }
    }
}

#[vst3_impl]
unsafe impl<T> IComponentHandler for ComponentHandlerImpl<T>
where
    T: ComponentHandler + Interface,
{
    fn beginEdit(&mut self, id: ParamID) -> tresult {
        if let Err(e) = self.pimpl.begin_edit(id) {
            e
        } else {
            0
        }
    }
    fn endEdit(&mut self, id: ParamID) -> tresult {
        if let Err(e) = self.pimpl.end_edit(id) {
            e
        } else {
            0
        }
    }
    fn performEdit(&mut self, id: ParamID, value: f64) -> tresult {
        if let Err(e) = self.pimpl.perform_edit(id, value) {
            e
        } else {
            0
        }
    }
    fn restartComponent(&mut self, flags: i32) -> tresult {
        if let Err(e) = self.pimpl.restart_component(flags) {
            e
        } else {
            0
        }
    }
}

#[vst3_impl]
unsafe impl<T> IComponentHandler2 for ComponentHandlerImpl<T>
where
    T: ComponentHandler + Interface,
{
    fn setDirty(&mut self, state: TBool) -> tresult {
        if let Err(e) = self.pimpl.set_dirty(state == 0) {
            e
        } else {
            0
        }
    }
    fn requestOpenEditor(&mut self, _: FIDString) -> tresult {
        if let Err(e) = self.pimpl.request_open_editor() {
            e
        } else {
            0
        }
    }
    fn startGroupEdit(&mut self) -> tresult {
        if let Err(e) = self.pimpl.start_group_edit() {
            e
        } else {
            0
        }
    }
    fn finishGroupEdit(&mut self) -> tresult {
        if let Err(e) = self.pimpl.finish_group_edit() {
            e
        } else {
            0
        }
    }
}

#[repr(C)]
#[derive(Vst3Impl)]
#[interfaces(IPluginFactory, IPluginFactory2)]
pub struct PluginFactoryImpl<T>
where
    T: PluginFactory + Interface,
{
    vtbl: VTable<IPluginFactoryVtbl>,
    refcount: Refcount,
    pimpl: VstPtr<T>,
}

impl<T> PluginFactoryImpl<T>
where
    T: PluginFactory + Interface,
{
    pub fn new(pimpl: VstPtr<T>) -> VstPtr<IPluginFactory> {
        unsafe { VstPtr::from_raw(Self::create_raw(pimpl) as *mut _) }
    }
}

#[vst3_impl]
unsafe impl<T: PluginFactory + Interface> IPluginFactory for PluginFactoryImpl<T> {
    fn getFactoryInfo(&self, pinfo: *mut PFactoryInfo) -> tresult {
        match self.pimpl.get_factory_info() {
            Ok(info) => {
                unsafe { *pinfo = info };
                0
            }
            Err(e) => e,
        }
    }
    fn countClasses(&self) -> i32 {
        self.pimpl.count_classes() as i32
    }
    fn getClassInfo(&self, index: i32, pinfo: *mut PClassInfo) -> tresult {
        match self.pimpl.get_class_info(index as usize) {
            Ok(info) => {
                unsafe { *pinfo = info };
                0
            }
            Err(e) => e,
        }
    }
    fn createInstance(&mut self, cid: FIDString, iid: FIDString, obj: *mut *mut c_void) -> tresult {
        let pimpl = unsafe { &mut *self.pimpl.as_raw() };
        match pimpl.create_instance(cid, iid) {
            Ok(ptr) => {
                unsafe { *obj = ptr };
                0
            }
            Err(e) => e,
        }
    }
}
#[vst3_impl]
unsafe impl<T: PluginFactory + Interface> IPluginFactory2 for PluginFactoryImpl<T> {
    fn getClassInfo2(&self, _idx: i32, pinfo: *mut PClassInfo2) -> tresult {
        match self.pimpl.get_class_info2() {
            Ok(info) => {
                unsafe { *pinfo = info };
                0
            }
            Err(e) => e,
        }
    }
}

#[repr(C)]
#[derive(Vst3Impl)]
#[interfaces(IPluginBase)]
pub struct PluginBaseImpl<T>
where
    T: PluginBase + Interface,
{
    vtbl: VTable<IPluginBaseVtbl>,
    refcount: Refcount,
    pimpl: VstPtr<T>,
}

impl<T> PluginBaseImpl<T>
where
    T: PluginBase + Interface,
{
    pub fn new(pimpl: VstPtr<T>) -> VstPtr<IPluginBase> {
        unsafe { VstPtr::from_raw(Self::create_raw(pimpl) as *mut _) }
    }
}

#[vst3_impl]
unsafe impl<T> IPluginBase for PluginBaseImpl<T>
where
    T: PluginBase + Interface,
{
    fn initialize(&mut self, host: *mut FUnknown) -> tresult {
        let host = unsafe { VstPtr::from_raw(host) };
        if let Err(e) = self.pimpl.initialize(host) {
            e
        } else {
            0
        }
    }
    fn terminate(&mut self) -> tresult {
        if let Err(e) = self.pimpl.terminate() {
            e
        } else {
            0
        }
    }
}
#[repr(C)]
#[derive(Vst3Impl)]
#[interfaces(IComponent)]
pub struct ComponentImpl<T>
where
    T: Component + Interface + PluginBase,
{
    vtbl: VTable<IComponentVtbl>,
    refcount: Refcount,
    pimpl: VstPtr<T>,
}

#[vst3_impl]
unsafe impl<T> IPluginBase for ComponentImpl<T>
where
    T: Component + Interface + PluginBase,
{
    fn initialize(&mut self, host: *mut FUnknown) -> tresult {
        let host = unsafe { VstPtr::from_raw(host) };
        if let Err(e) = self.pimpl.initialize(host) {
            e
        } else {
            0
        }
    }
    fn terminate(&mut self) -> tresult {
        if let Err(e) = self.pimpl.terminate() {
            e
        } else {
            0
        }
    }
}

impl<T> ComponentImpl<T>
where
    T: Component + Interface + PluginBase,
{
    pub fn new(pimpl: VstPtr<T>) -> VstPtr<IComponent> {
        unsafe { VstPtr::from_raw(Self::create_raw(pimpl) as *mut _) }
    }
}

#[vst3_impl]
unsafe impl<T> IComponent for ComponentImpl<T>
where
    T: Component + Interface + PluginBase,
{
    fn getControllerClassID(&self, classId: *mut i8) -> tresult {
        unsafe {
            std::slice::from_raw_parts_mut(classId, 16).copy_from_slice(&<T as Interface>::iid());
        }
        0
    }
    fn setIoMode(&mut self, mode: IoMode) -> tresult {
        if let Err(e) = self.pimpl.set_io_mode(mode) {
            e
        } else {
            0
        }
    }
    fn getBusCount(&self, type_: MediaType, dir: BusDirection) -> i32 {
        (self.pimpl.get_bus_count(type_, dir) as i32)
    }
    fn getBusInfo(
        &self,
        type_: MediaType,
        dir: BusDirection,
        index: i32,
        busInfo: *mut BusInfo,
    ) -> tresult {
        match self.pimpl.get_bus_info(type_, dir, index as usize) {
            Err(e) => e,
            Ok(info) => {
                unsafe { *busInfo = info };
                0
            }
        }
    }
    fn getRoutingInfo(&self, inInfo: *mut RoutingInfo, outInfo: *mut RoutingInfo) -> tresult {
        let inpt = unsafe { &mut *inInfo };
        let outp = unsafe { &mut *outInfo };
        if let Err(e) = self.pimpl.get_routing_info(inpt, outp) {
            e
        } else {
            0
        }
    }
    fn activateBus(
        &mut self,
        type_: MediaType,
        dir: BusDirection,
        index: i32,
        state: TBool,
    ) -> tresult {
        if let Err(e) = self
            .pimpl
            .activate_bus(type_, dir, index as usize, state == 0)
        {
            e
        } else {
            0
        }
    }
    fn setActive(&mut self, state: TBool) -> tresult {
        // no really, kResultTrue == 0
        if let Err(e) = self.pimpl.set_active(state == 0) {
            e
        } else {
            0
        }
    }
    fn setState(&mut self, state: *mut IBStream) -> tresult {
        if let Err(e) = unsafe { self.pimpl.set_state(VstPtr::from_raw(state)) } {
            e
        } else {
            0
        }
    }
    fn getState(&mut self, state: *mut IBStream) -> tresult {
        if let Err(e) = unsafe { self.pimpl.get_state(VstPtr::from_raw(state)) } {
            e
        } else {
            0
        }
    }
}
#[repr(C)]
#[derive(Vst3Impl)]
#[interfaces(IAudioProcessor)]
pub struct AudioProcessorImpl<T>
where
    T: Interface + AudioProcessor,
{
    vtbl: VTable<IAudioProcessorVtbl>,
    refcount: Refcount,
    pimpl: VstPtr<T>,
}

impl<T: Interface + AudioProcessor> AudioProcessorImpl<T> {
    pub fn new(pimpl: VstPtr<T>) -> VstPtr<IAudioProcessor> {
        unsafe { VstPtr::from_raw(Self::create_raw(pimpl) as *mut _) }
    }
}

#[vst3_impl]
unsafe impl<T: Interface + AudioProcessor> IAudioProcessor for AudioProcessorImpl<T> {
    fn setBusArrangements(
        &mut self,
        inputs: *mut SpeakerArrangement,
        numIns: i32,
        outputs: *mut SpeakerArrangement,
        numOuts: i32,
    ) -> tresult {
        let (inputs, outputs) = unsafe {
            (
                slice::from_raw_parts(inputs as *const _, numIns as usize),
                slice::from_raw_parts(outputs as *const _, numOuts as usize),
            )
        };
        if let Err(e) = self.pimpl.set_bus_arrangements(inputs, outputs) {
            e
        } else {
            0
        }
    }
    fn getBusArrangement(
        &self,
        dir: BusDirection,
        index: i32,
        arr: *mut SpeakerArrangement,
    ) -> tresult {
        match self.pimpl.get_bus_arrangement(dir, index as usize) {
            Err(e) => e,
            Ok(a) => {
                unsafe { *arr = a };
                0
            }
        }
    }
    fn canProcessSampleSize(&self, symbolixSampleSize: i32) -> tresult {
        let yes = if symbolixSampleSize == kSample32 {
            self.pimpl.can_do_32()
        } else {
            self.pimpl.can_do_64()
        };
        if yes {
            0
        } else {
            1
        }
    }
    fn getLatencySamples(&self) -> i32 {
        self.pimpl.get_latency_samples() as i32
    }
    fn setupProcessing(&mut self, setup: *mut ProcessSetup) -> tresult {
        if let Err(e) = self.pimpl.setup_processing(unsafe { &*setup }) {
            e
        } else {
            0
        }
    }
    fn setProcessing(&mut self, state: TBool) -> tresult {
        if let Err(e) = self.pimpl.set_processing(state == 0) {
            e
        } else {
            0
        }
    }
    fn process(&mut self, data: *mut ProcessData) -> tresult {
        let data = unsafe { &mut *data };
        if let Err(e) = self.pimpl.process(data) {
            e
        } else {
            0
        }
    }
    fn getTailSamples(&self) -> i32 {
        self.pimpl.get_tail_samples() as i32
    }
}
#[repr(C)]
#[derive(Vst3Impl)]
#[interfaces(IEditController)]
pub struct EditControllerImpl<T>
where
    T: Interface + EditController + PluginBase,
{
    vtbl: VTable<IEditControllerVtbl>,
    refcount: Refcount,
    pimpl: VstPtr<T>,
}

impl<T: Interface + EditController + PluginBase> EditControllerImpl<T> {
    pub fn new(pimpl: VstPtr<T>) -> VstPtr<IAudioProcessor> {
        unsafe { VstPtr::from_raw(Self::create_raw(pimpl) as *mut _) }
    }
}

#[vst3_impl]
unsafe impl<T> IPluginBase for EditControllerImpl<T>
where
    T: EditController + Interface + PluginBase,
{
    fn initialize(&mut self, host: *mut FUnknown) -> tresult {
        let host = unsafe { VstPtr::from_raw(host) };
        if let Err(e) = self.pimpl.initialize(host) {
            e
        } else {
            0
        }
    }
    fn terminate(&mut self) -> tresult {
        if let Err(e) = self.pimpl.terminate() {
            e
        } else {
            0
        }
    }
}

#[vst3_impl]
unsafe impl<T: Interface + EditController + PluginBase> IEditController for EditControllerImpl<T> {
    fn setComponentState(&mut self, state: *mut IBStream) -> tresult {
        if let Err(e) = unsafe { self.pimpl.set_component_state(VstPtr::from_raw(state)) } {
            e
        } else {
            0
        }
    }
    fn setState(&mut self, state: *mut IBStream) -> tresult {
        if let Err(e) = unsafe { self.pimpl.set_state(VstPtr::from_raw(state)) } {
            e
        } else {
            0
        }
    }
    fn getState(&mut self, state: *mut IBStream) -> tresult {
        if let Err(e) = unsafe { self.pimpl.get_state(VstPtr::from_raw(state)) } {
            e
        } else {
            0
        }
    }
    fn getParameterCount(&self) -> i32 {
        self.pimpl.get_parameter_count() as i32
    }
    fn getParameterInfo(&self, index: i32, info: *mut ParameterInfo) -> tresult {
        match self.pimpl.get_parameter_info(index as usize) {
            Err(e) => e,
            Ok(i) => {
                unsafe { *info = i };
                0
            }
        }
    }
    fn getParamStringByValue(
        &self,
        id: ParamID,
        value: ParamValue,
        string128: *mut TChar,
    ) -> tresult {
        // this is ugly and undocumented, but the `string` parameter is no longer than 128 bytes.
        let mut bytes = [128u8; 0];
        let string = unsafe { std::str::from_utf8_unchecked_mut(&mut bytes) };
        if let Err(e) = self.pimpl.get_param_string_by_value(id, value, string) {
            e
        } else {
            let wstr = U16CString::from_str(string).unwrap(); // probs not a good ide
            unsafe {
                slice::from_raw_parts_mut(string128 as *mut _ as *mut u16, 128)
                    .copy_from_slice(wstr.as_slice())
            };
            0
        }
    }
    fn getParamValueByString(
        &self,
        id: ParamID,
        string: *mut TChar,
        value: *mut ParamValue,
    ) -> tresult {
        let string = unsafe { U16CStr::from_ptr_str(string as *mut _ as *mut u16) }
            .to_string()
            .unwrap();
        match self.pimpl.get_param_value_by_string(id, &string) {
            Err(e) => e,
            Ok(val) => {
                unsafe { *value = val };
                0
            }
        }
    }
    fn normalizedParamToPlain(&self, id: ParamID, normalized: ParamValue) -> ParamValue {
        self.pimpl.normalized_param_to_plain(id, normalized)
    }
    fn plainParamToNormalized(&self, id: ParamID, plain: ParamValue) -> ParamValue {
        self.pimpl.plain_param_to_normalized(id, plain)
    }
    fn getParamNormalized(&self, id: ParamID) -> ParamValue {
        self.pimpl.get_param_normalized(id)
    }
    fn setParamNormalized(&mut self, id: ParamID, value: ParamValue) -> tresult {
        if let Err(e) = self.pimpl.set_param_normalized(id, value) {
            e
        } else {
            0
        }
    }
    fn setComponentHandler(&mut self, handler: *mut IComponentHandler) -> tresult {
        if let Err(e) = unsafe { self.pimpl.set_component_handler(VstPtr::from_raw(handler)) } {
            e
        } else {
            0
        }
    }
    fn createView(&mut self, _name: FIDString) -> *mut IPlugView {
        if let Some(view) = self.pimpl.create_view() {
            unsafe { view.addRef() };
            view.as_raw()
        } else {
            std::ptr::null_mut()
        }
    }
}