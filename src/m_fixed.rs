#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use crate::i_system::I_Error;

pub(crate) type fixed_t = i32;

pub(crate) const FRACBITS: i32 = 16;
pub(crate) const FRACUNIT: i32 = 1 << FRACBITS;

pub(crate) fn FixedMul(a: fixed_t, b: fixed_t) -> fixed_t {
	fixed_t::try_from((i64::from(a) * i64::from(b)) >> FRACBITS).unwrap()
}

/// FixedDiv, C version.
pub(crate) fn FixedDiv(a: fixed_t, b: fixed_t) -> fixed_t {
	if (a.abs() >> 14) >= b.abs() {
		return if (a ^ b) < 0 { i32::MIN } else { i32::MAX };
	}
	fixed_div_2(a, b)
}

#[allow(clippy::as_conversions)]
fn fixed_div_2(a: fixed_t, b: fixed_t) -> fixed_t {
	let c = (a as f64) / (b as f64) * FRACUNIT as f64;

	if !(-2147483648.0..2147483648.0).contains(&c) {
		I_Error("FixedDiv: divide by zero");
	}
	c as fixed_t
}
