#![allow(non_snake_case)]

// Swap 16bit, that is, MSB and LSB byte.
#[unsafe(no_mangle)]
pub extern "C" fn SwapSHORT(x: u16) -> u16 {
	// No masking with 0xFF should be necessary.
	x.rotate_left(8)
}

// Swapping 32bit.
#[unsafe(no_mangle)]
pub extern "C" fn SwapLONG(x: u32) -> u32 {
	x.reverse_bits()
}
