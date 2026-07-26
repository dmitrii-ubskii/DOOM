#![allow(clippy::as_conversions)]

pub const fn i8_from_usize(usize: usize) -> i8 {
	if usize > i8::MAX as usize { panic!("Positive overflow") } else { usize as i8 }
}

pub const fn i16_from_usize(usize: usize) -> i16 {
	if usize > i16::MAX as usize { panic!("Positive overflow") } else { usize as i16 }
}

pub const fn i32_from_usize(usize: usize) -> i32 {
	if usize > i32::MAX as usize { panic!("Positive overflow") } else { usize as i32 }
}

pub const fn i64_from_usize(usize: usize) -> i64 {
	if usize > i64::MAX as usize { panic!("Positive overflow") } else { usize as i64 }
}

pub const fn u8_from_u32(u32: u32) -> u8 {
	if u32 > u8::MAX as u32 { panic!("Positive overflow") } else { u32 as u8 }
}

pub const fn u32_from_u8(u8: u8) -> u32 {
	u8 as u32
}

pub const fn u32_from_usize(usize: usize) -> u32 {
	if usize > u32::MAX as usize { panic!("Positive overflow") } else { usize as u32 }
}

pub const fn usize_from_u8(u8: u8) -> usize {
	u8 as usize
}

pub const fn usize_from_u32(u32: u32) -> usize {
	u32 as usize
}
