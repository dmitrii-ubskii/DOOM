#![allow(non_snake_case, clippy::missing_safety_doc)]

use std::ffi::c_char;

use crate::{myargc, myargv};

unsafe extern "C" {
	fn strcasecmp(lhs: *const c_char, rhs: *const c_char) -> i32;
}

pub(crate) unsafe fn M_CheckParm(check: *const c_char) -> usize {
	unsafe {
		for i in 1..myargc {
			if strcasecmp(check, *myargv.add(i)) == 0 {
				return i;
			}
		}
	}

	0
}
