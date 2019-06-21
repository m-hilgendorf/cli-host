use super::vsttypes::*;
use crate::base::*;

RIDL!{#[iid(0x01263A18, 0xED074F6F, 0x98C9D356, 0x4686F9BA)]
    interface IParamValueQueue(IParamValueQueueVtbl) : FUnknown(FUnknownVtbl) {
        fn getParameterID() -> ParamID,
        fn getPointCount() -> i32,
        fn getPoint(index : i32, sampleOffset : *mut i32, value : *mut ParamValue,)-> tresult,
        fn addPoint(sampleOffset : i32, value : ParamValue, index : *mut i32,) -> tresult,
    }
}

RIDL!{#[iid(0xA4779663, 0x0BB64A56, 0xB44384A8, 0x466FEB9D)]
    interface IParameterChanges(IParameterChangesVtbl) : FUnknown(FUnknownVtbl) {
        fn getParameterCount() -> i32,
        fn getParameterData(index : i32,) -> *mut IParamValueQueue,
        fn addParameterData(id : *mut ParamID, index : *mut i32,) -> *mut IParamValueQueue,
    }
}