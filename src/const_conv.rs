#![allow(clippy::as_conversions)]

#[track_caller]
pub(crate) const fn i32_from_usize(usize: usize) -> i32 {
	if usize > i32::MAX as usize { panic!("Positive overflow") } else { usize as i32 }
}

#[track_caller]
pub(crate) const fn u8_from_u32(u32: u32) -> u8 {
	if u32 > u8::MAX as u32 { panic!("Positive overflow") } else { u32 as u8 }
}

#[track_caller]
pub(crate) const fn u32_from_u8(u8: u8) -> u32 {
	u8 as u32
}

#[track_caller]
pub(crate) const fn u32_from_usize(usize: usize) -> u32 {
	if usize > u32::MAX as usize { panic!("Positive overflow") } else { usize as u32 }
}

#[track_caller]
pub(crate) const fn usize_from_u8(u8: u8) -> usize {
	u8 as usize
}
