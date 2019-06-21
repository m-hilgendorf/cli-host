use crate::*;
use std::ptr::null_mut;
use vst3_derive::*;
use vst3_impl::*;
use widestring::{U16Str, WideCStr};

pub enum ProcessMode {
    RealTime,
    PreFetch,
    Offline,
}
pub enum SampleSize {
    F32,
    F64,
}
use std::mem::transmute;
use std::os::raw::c_void;
use widestring::U16CStr;

/// Safe wrapper around IAudioProcessor
pub struct Processor(VstPtr<IAudioProcessor>);
impl Processor {
    //todo: take a host as an argument for the initialization method
    pub fn new(processor: VstPtr<IAudioProcessor>) -> Self {
        unsafe {
            (*processor
                .cast::<IPluginBase>()
                .expect("Processor doesn't implement IPluginBase"))
            .initialize(null_mut());
        }
        Self(processor)
    }

    fn setup_busses(&self, flags: SpeakerArrangement) -> Result<(), tresult> {
        let mut arr = flags;
        let e =
            unsafe { (*self.0).setBusArrangements(&mut arr as *mut _, 1, &mut arr as *mut _, 1) };
        if e != 0 {
            Err(e)
        } else {
            Ok(())
        }
    }

    //todo: support exotic speaker arrangments
    pub fn set_mono(&self) -> Result<(), tresult> {
        self.setup_busses(1)
    }
    pub fn set_stereo(&self) -> Result<(), tresult> {
        self.setup_busses(3)
    }
    pub fn supports32(&self) -> bool {
        unsafe { (*self.0).canProcessSampleSize(SymbolicSampleSizes::kSample32) == 0 }
    }
    pub fn supports64(&self) -> bool {
        unsafe { (*self.0).canProcessSampleSize(SymbolicSampleSizes::kSample64) == 0 }
    }

    //todo: give this a generic argument?
    pub fn setup_processing(
        &self,
        mode: ProcessMode,
        sample_size: SampleSize,
        max_block_size: usize,
        sample_rate: f64,
    ) -> Result<(), tresult> {
        let pmode = match mode {
            ProcessMode::RealTime => ProcessModes::kRealtime,
            ProcessMode::PreFetch => ProcessModes::kPrefetch,
            ProcessMode::Offline => ProcessModes::kOffline,
        };

        let sampleSize = match sample_size {
            SampleSize::F32 => SymbolicSampleSizes::kSample32,
            SampleSize::F64 => SymbolicSampleSizes::kSample64,
        };

        let mut setup = ProcessSetup {
            processMode: pmode,
            symbolicSampleSize: sampleSize,
            maxSamplesPerblock: max_block_size as i32,
            sampleRate: sample_rate,
        };
        let e = unsafe { (*self.0).setupProcessing(&mut setup as *mut _) };

        if e != 0 {
            Err(e)
        } else {
            Ok(())
        }
    }

    pub fn tail_samples(&self) -> usize {
        unsafe { (*self.0).getTailSamples() as usize }
    }

    pub fn latency_samples(&self) -> usize {
        unsafe { (*self.0).getLatencySamples() as usize }
    }
}

impl Drop for Processor {
    fn drop(&mut self) {
        unsafe {
            (*(self.0.cast::<IPluginBase>().unwrap())).terminate();
        }
    }
}

pub struct BStream(VstPtr<IBStream>);
pub struct Editor(VstPtr<IEditController>);

#[allow(unused_variables)]
impl Editor {
    pub fn new(editor: VstPtr<IEditController>) -> Self {
        unsafe {
            (*editor
                .cast::<IPluginBase>()
                .expect("Processor doesn't implement IPluginBase"))
            .initialize(null_mut());
        }
        Self(editor)
    }

    pub fn set_state(&self, stream: &BStream) -> Result<(), tresult> {
        let e = unsafe { (*self.0).setState(stream.0.as_raw()) };

        if e != 0 {
            Err(e)
        } else {
            Ok(())
        }
    }

    pub fn get_state(&self, stream: &BStream) -> Result<(), tresult> {
        let e = unsafe { (*self.0).getState(stream.0.as_raw()) };

        if e != 0 {
            Err(e)
        } else {
            Ok(())
        }
    }

    pub fn get_parameter_count(&self) -> usize {
        unsafe { self.0.getParameterCount() as usize }
    }

    pub fn get_parameter_info(&self, index: usize) -> Result<ParameterInfo, tresult> {
        let mut info = ParameterInfo::default();
        let e = unsafe { self.0.getParameterInfo(index as i32, &mut info as *mut _) };

        if e != 0 {
            Err(e)
        } else {
            Ok(info)
        }
    }

    //todo: make this not allocate
    pub fn get_param_string_by_value(&self, id: ParamID, value: f64) -> Result<String, tresult> {
        let mut s = [0; 128];
        let e = unsafe { self.0.getParamStringByValue(id, value, s.as_mut_ptr()) };
        if e != 0 {
            Err(e)
        } else {
            unsafe { Ok(U16CStr::from_ptr_str(transmute(s.as_ptr())).to_string_lossy()) }
        }
    }

    // todo: utf8 version
    pub fn get_param_value_by_string(id: ParamID, string: &U16CStr) -> Result<f64, tresult> {
        //todo:
        unimplemented!()
    }
    pub fn normalized_param_to_plain(&self, id: ParamID, normalized: f64) -> f64 {
        unsafe { self.0.normalizedParamToPlain(id, normalized) }
    }
    pub fn plain_param_to_normlaized(&self, id: ParamID, normalized: f64) -> f64 {
        unsafe { self.0.plainParamToNormalized(id, normalized) }
    }
    pub fn get_param_normalized(&self, id: ParamID) -> f64 {
        unsafe { self.0.getParamNormalized(id) }
    }
    pub fn set_param_normalized(&self, id: ParamID, value: f64) -> Result<(), tresult> {
        let e = unsafe { self.0.setParamNormalized(id, value) };
        if e != 0 {
            Err(e)
        } else {
            Ok(())
        }
    }
    pub fn set_component_handler(&self, handler: ComponentHandler) -> Result<(), tresult> {
        Err(-1)
    }
    pub fn create_view(&self) -> PlugView {
        unsafe {
            let name: *mut i8 = transmute("editor\0".as_ptr());
            let view = self.0.createView(name);
            PlugView::new(VstPtr::from_raw(view))
        }
    }
}
// todo: impl component handler
pub struct ComponentHandler(VstPtr<IComponentHandler>);
pub struct PlugView(VstPtr<IPlugView>);
impl PlugView {
    pub fn new(view: VstPtr<IPlugView>) -> Self {
        Self(view)
    }

    pub fn attached(&self, parent: *mut c_void) -> Result<(), tresult> {
        unsafe {
            let platform: *mut i8 = transmute("X11EmbedWindowID\0".as_ptr());
            let e = self.0.attached(parent, platform);
            if e != 0 {
                Err(e)
            } else {
                Ok(())
            }
        }
    }

    pub fn get_size(&self) -> Result<ViewRect, tresult> {
        let mut rect = ViewRect {
            left: 0,
            right: 0,
            top: 0,
            bottom: 0,
        };
        let e = unsafe { self.0.getSize(&mut rect as *mut _) };
        if e != 0 {
            Err(e)
        } else {
            Ok(rect)
        }
    }

    pub fn removed(&self) -> Result<(), tresult> {
        let e = unsafe { self.0.removed() };
        if e != 0 {
            Err(e)
        } else {
            Ok(())
        }
    }

    pub fn set_frame(&self, frame: *mut IPlugFrame) {
        //    unsafe { self.0.setFrame(frame.as_raw() as *mut _) };
        unsafe { self.0.setFrame(frame as *mut _) };
    }
}

#[repr(C)]
#[derive(Vst3Impl)]
#[interfaces(IPlugFrame)]
pub struct HostPlugFrame {
    vtbl: VTable<IPlugFrameVtbl>,
    refcount: Refcount,
}

impl HostPlugFrame {
    pub fn new() -> VstPtr<IPlugFrame> {
        let ptr = HostPlugFrame::create_raw();
        let ptr = ptr as *mut IPlugFrame;
        unsafe { VstPtr::from_raw(ptr) }
    }
}

#[vst3_impl]
unsafe impl IPlugFrame for HostPlugFrame {
    fn resizeView(&self, view: *mut IPlugView, newSize: *mut ViewRect) -> tresult {
        println!("view: {:?}", view);
        let size = unsafe { &*newSize };
        println!(
            "size: {},{},{},{}",
            size.top, size.bottom, size.left, size.right
        );
        0
    }
}
