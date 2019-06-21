use super::*;
use vst3_impl::*;

#[doc(hidden)]
#[repr(C)]
#[derive(Vst3Impl)]
#[interfaces(IAttributeList)]
pub struct AttributeListImpl<T>
    where T: AttributeList {
    vtbl     : VTable<IAttributeListVtbl>,
    refcount : Refcount,
    pimpl    : T
}

impl<T> AttributeListImpl<T>
    where T: AttributeList {
    pub fn new(pimpl : T) -> VstPtr<IAttributeList> {
        unsafe { VstPtr::from_raw(Self::create_raw(pimpl) as *mut _) }
    }
}

#[vst3_impl]
unsafe impl<T> IAttributeList for AttributeListImpl<T>
    where T: AttributeList {
    fn setInt (&mut self, id : AttrID, value : i64) -> tresult {
        let id = unsafe { CStr::from_ptr(id).to_str().unwrap() };
        if let Err(e) = self.pimpl.set(id, AttributeValue::Int(value)) {
            e
        }
        else { 0 }
    }
    fn getInt (&self, id : AttrID, value : *mut i64) -> tresult {
        let id = unsafe { CStr::from_ptr(id).to_str().unwrap() };
        match self.pimpl.get(id) {
            Err(e) => e,
            Ok(var) => {
                match var {
                    AttributeValue::Int(v) => {
                        unsafe {*value = v };
                        0
                    },
                    _ => -1
                }
            }
        }
    }
    fn setFloat (&mut self, id : AttrID, value : f64) -> tresult {
        let id = unsafe { CStr::from_ptr(id).to_str().unwrap() };
        if let Err(e) = self.pimpl.set(id, AttributeValue::Float(value)) {
            e
        } else { 0 }
    }
    fn getFloat (&self, id : AttrID, value : *mut f64) -> tresult {
        let id = unsafe { CStr::from_ptr(id).to_str().unwrap() };
        match self.pimpl.get(id) {
            Err(e) => e,
            Ok(var) => {
                match var {
                    AttributeValue::Float(v) => {
                        unsafe {*value = v };
                        0
                    },
                    _ => -1
                }
            }
        }
    }
    fn setString (&mut self, id : AttrID, value : *const TChar) -> tresult {
        let id = unsafe { CStr::from_ptr(id).to_str().unwrap() };
        let wstr = unsafe { U16CStr::from_ptr_str(transmute(value)) };
        if let Err(e) =
        self.pimpl
            .set(id, AttributeValue::String(&wstr.to_string_lossy())) {
            e
        } else { 0 }
    }
    fn getString (&self, id : AttrID, value : *mut TChar, size : u32) -> tresult {
        let size = (size as usize) / 2;
        let id = unsafe { CStr::from_ptr(id).to_str().unwrap() };
        match self.pimpl.get(id) {
            Err(e) => e,
            Ok(var) => {
                match var {
                    AttributeValue::String(string) => {
                        let wstr = U16String::from_str(string);
                        unsafe {
                            memcpy (
                                wstr.as_ptr(),
                                transmute(value),
                                min(wstr.len(), size)
                            )
                        };
                        0
                    }
                    _ => -1
                }
            }
        }
    }
    fn setBinary (&mut self, id : AttrID, value : *const c_void, size : u32) -> tresult {
        let id = unsafe { CStr::from_ptr(id).to_str().unwrap() };
        let slc = unsafe { std::slice::from_raw_parts(value as *const u8, size as usize) };
        if let Err(e) = self.pimpl.set(id, AttributeValue::Binary(slc)) {
            e
        } else { 0}
    }
    fn getBinary (&self, id : AttrID, value : *mut *const c_void, size : *mut u32) -> tresult {
        let id = unsafe { CStr::from_ptr(id).to_str().unwrap() };
        let size = unsafe { *size as usize };
        match self.pimpl.get(id) {
            Err(e) => e,
            Ok(var) => {
                match var {
                    AttributeValue::Binary(slc) => {
                        let dst = unsafe { *value } as *mut u8;
                        unsafe {
                            memcpy (
                                slc.as_ptr(),
                                dst,
                                min(slc.len(), size )
                            )
                        };
                        0
                    }
                    _ => -1,
                }
            }
        }
    }
}

#[doc(hidden)]
#[repr(C)]
#[derive(Vst3Impl)]
#[interfaces(IComponentHandler)]
pub struct ComponentHandlerImpl<T> where T: ComponentHandler {
    vtbl     : VTable<IComponentHandlerVtbl>,
    refcount : Refcount,
    pimpl    : T
}

impl<T> ComponentHandlerImpl<T>
    where T: ComponentHandler {
    pub fn new(pimpl : T) -> VstPtr<IComponentHandler> {
        unsafe { VstPtr::from_raw(Self::create_raw(pimpl) as *mut _) }
    }
}

#[vst3_impl]
unsafe impl<T> IComponentHandler for ComponentHandlerImpl<T>
  where T: ComponentHandler {
    fn beginEdit(&mut self, id : ParamID) -> tresult {
        if let Err(e) = self.pimpl.begin_edit(id) {
            e
        } else { 0 }
    }
    fn endEdit(&mut self, id : ParamID) -> tresult {
        if let Err(e) = self.pimpl.end_edit(id) {
            e
        } else { 0 }
    }
    fn performEdit(&mut self, id : ParamID, value : f64) -> tresult {
        if let Err(e) = self.pimpl.perform_edit(id, value) {
            e
        } else { 0 }
    }
    fn restartComponent(&mut self, flags : i32) -> tresult {
        if let Err(e) = self.pimpl.restart_component(flags) {
            e
        } else { 0 }
    }
}

#[doc(hidden)]
#[repr(C)]
#[derive(Vst3Impl)]
#[interfaces(IComponentHandler2)]
pub struct ComponentHandler2Impl<T> where T: ComponentHandler2 {
    vtbl     : VTable<IComponentHandler2Vtbl>,
    refcount : Refcount,
    pimpl    : T
}

impl<T> ComponentHandler2Impl<T>
    where T: ComponentHandler2 {
    pub fn new(pimpl : T) -> VstPtr<IComponentHandler2> {
        unsafe { VstPtr::from_raw(Self::create_raw(pimpl) as *mut _) }
    }
}

#[vst3_impl]
unsafe impl<T> IComponentHandler2 for ComponentHandler2Impl <T>
  where T: ComponentHandler2 {
    fn setDirty(&mut self, state : TBool) -> tresult {
        if let Err(e) = self.pimpl.set_dirty (state == 0) {
            e
        }
        else { 0 }
    }
    fn requestOpenEditor(&mut self, _ : FIDString) -> tresult {
        if let Err(e) = self.pimpl.request_open_editor() {
            e
        } else { 0 }
    }
    fn startGroupEdit(&mut self) -> tresult {
        if let Err(e) = self.pimpl.start_group_edit() {
            e
        } else { 0 }
    }
    fn finishGroupEdit(&mut self) -> tresult {
        if let Err(e) = self.pimpl.finish_group_edit() {
            e
        } else { 0 }
    }
}

#[repr(C)]
#[derive(Vst3Impl)]
#[interfaces(IPluginFactory)]
pub struct PluginFactoryImpl <T> where T: PluginFactory {
    vtbl     : VTable <IPluginFactoryVtbl>,
    refcount : Refcount,
    pimpl    : T
}

impl<T> PluginFactoryImpl<T>
    where T: PluginFactory {
    pub fn new(pimpl : T) -> VstPtr<IPluginFactory> {
        unsafe { VstPtr::from_raw(Self::create_raw(pimpl) as *mut _) }
    }
}

#[vst3_impl]
unsafe impl<T: PluginFactory> IPluginFactory for PluginFactoryImpl<T> {
    fn getFactoryInfo (&self, pinfo : *mut PFactoryInfo,) -> tresult {
        match self.pimpl.get_factory_info() {
            Ok(info) => {
                unsafe { *pinfo = info };
                0
            },
            Err(e) => e
        }
    }
    fn countClasses(&self) -> i32 {
        self.pimpl.count_classes() as i32
    }
    fn getClassInfo(&self, index : i32, pinfo : *mut PClassInfo,) -> tresult {
        match self.pimpl.get_class_info(index as usize) {
            Ok(info) => {
                unsafe { *pinfo = info };
                0
            }
            Err(e) => e
        }
    }
    fn createInstance(&mut self, cid : FIDString, iid : FIDString, obj : *mut *mut c_void,) -> tresult {
        match self.pimpl.create_instance(cid, iid) {
            Ok(ptr) => {
                unsafe { *obj = ptr };
                0
            }
            Err(e) => e
        }
    }
}