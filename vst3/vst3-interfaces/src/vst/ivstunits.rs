use super::vsttypes::*;
use crate::base::*;

pub const kRootUnitId: UnitId = 0;
pub const kNoParentUnitId: UnitId = 1;
pub const kNoProgramList: ProgramListID = -1;
pub const kAllProgramInvalid: i32 = -1;

#[repr(C)]
#[repr(align(16))]
pub struct UnitInfo {
    pub id: UnitId,
    pub parentId: UnitId,
    pub name: String128,
    pub programListId: ProgramListID,
}

#[repr(C)]
#[repr(align(16))]
pub struct ProgramListInfo {
    id: ProgramListID,
    name: String128,
    programCount: i32,
}

RIDL! { #[iid(0x4B5147F8, 0x4654486B, 0x8DAB30BA, 0x163A3C56)]
    interface IUnitHandler(IUnitHandlerVtbl) : FUnknown(FUnknownVtbl) {
        fn notifyUnitSelection (id : UnitId,) -> tresult,
        fn notifyProgramListChange (listId : ProgramListID, programIndex : i32,) -> tresult,
    }
}
RIDL! { #[iid(0xF89F8CDF, 0x699E4BA5, 0x96AAC9A4, 0x81452B01)]
    interface IUnitHandler2(IUnitHandler2Vtbl) : FUnknown(FUnknownVtbl) {
        fn notifyUnitByBusChange() -> tresult,
    }
}
RIDL! { #[iid(0x3D4BD6B5, 0x913A4FD2, 0xA886E768, 0xA5EB92C1)]
    interface IUnitInfo(IUnitInfoVtbl) : FUnknown(FUnknownVtbl) {
        fn getUnitCount() -> i32,
        fn getUnitInfo(unitIndex : i32, info : *mut UnitInfo,) -> tresult,
        fn getProgramListCount() -> i32,
        fn getProgramListInfo(listIndex : i32, info : *mut ProgramListInfo,) -> tresult,
        fn getProgramName(listId : ProgramListID, programIndex : i32, attributeId : CString, attributeValue : String128,) -> tresult,
        fn getProgramInfo(listId : ProgramListID, programIndex : i32,) -> tresult,
    }
}
