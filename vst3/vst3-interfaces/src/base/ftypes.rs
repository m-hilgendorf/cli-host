#![allow(non_upper_case_globals)]
///! From `vst3sdk/pluginterfaces/base/ftypes.h

/*
	todo: const FIDString kPlatformStringWin = "WIN";
	todo: const FIDString kPlatformStringMac = "MAC";
	todo: const FIDString kPlatformStringIOS = "IOS";
	todo: const FIDString kPlatformStringLinux = "Linux";

todo: #if SMTG_OS_WINDOWS
	const FIDString kPlatformString = kPlatformStringWin;
todo: #elif SMTG_OS_IOS
	const FIDString kPlatformString = kPlatformStringIOS;
todo: #elif SMTG_OS_MACOS
	const FIDString kPlatformString = kPlatformStringMac;
todo: #elif SMTG_OS_LINUX
	const FIDString kPlatformString = kPlatformStringLinux;
#endif
*/

pub type uchar = std::os::raw::c_uchar;
pub type TSize = i64; 
pub type tresult = i32; 
pub type TPtrInt = usize;
pub type TBool = u8; 
pub type char8  = std::os::raw::c_char;
pub type char16 = i16;

//todo: figure what the UNICODE def is about and why this exists
#[cfg(feature = "unicode")]
pub type tchar = char16;

#[cfg(not(feature = "unicode"))]
pub type tchar = char; 
pub type CStringA = *const char8;
pub type CStringW = *const char16; 
pub type CString  = *const tchar; 
pub type FIDString = *const char8;
pub type UCoord = i32; 

pub const kMaxLong   : i32 = 0x7fffffff;
pub const kMinLong   : i32 = (-0x7fffffff - 1);
pub const kMaxInt32  : i32 = std::i32::MAX;
pub const kMinInt32  : i32 = std::i32::MIN;
pub const kMaxInt32u : u32 = std::u32::MAX;
pub const kMaxInt64  : i64 = std::i64::MAX;
pub const kMinInt64  : i64 = std::i64::MIN;
pub const kMaxInt64u : u64 = std::u64::MAX;
pub const kMaxFloat  : f32 = 3.40282346638528860e38f32;
pub const kMaxDouble : f64 = 1.7976931348623158e308f64;
pub const kMaxCoord  : UCoord = 0x7FFFFFFF;
pub const kMinCoord  : UCoord = -0x7FFFFFFF;
