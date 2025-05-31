#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use crate::i_system::I_Error;

pub type fixed_t = i32;

pub const FRACBITS: i32 = 16;
pub const FRACUNIT: i32 = 1 << FRACBITS;

#[unsafe(no_mangle)]
pub extern "C" fn FixedMul(a: fixed_t, b: fixed_t) -> fixed_t {
	((a as i64 * b as i64) >> FRACBITS) as fixed_t
}

/// FixedDiv, C version.
#[unsafe(no_mangle)]
pub extern "C" fn FixedDiv(a: fixed_t, b: fixed_t) -> fixed_t {
	if (a.abs() >> 14) >= b.abs() {
		return if (a ^ b) < 0 { i32::MIN } else { i32::MAX };
	}
	fixed_div_2(a, b)
}

fn fixed_div_2(a: fixed_t, b: fixed_t) -> fixed_t {
	let c = (a as f64) / (b as f64) * FRACUNIT as f64;

	if !(-2147483648.0..2147483648.0).contains(&c) {
		unsafe { I_Error(c"FixedDiv: divide by zero".as_ptr()) };
	}
	c as fixed_t
}
