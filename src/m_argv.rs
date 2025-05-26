#![allow(non_snake_case)]

use std::ffi::c_char;

use crate::{myargc, myargv};

unsafe extern "C" {
	fn strcasecmp(_: *const c_char, _: *const c_char) -> i32;
}

#[unsafe(no_mangle)]
/// # Safety
/// none
pub unsafe fn M_CheckParm(check: *const c_char) -> i32 {
	unsafe {
		for i in 1..myargc {
			if strcasecmp(check, *myargv.add(i as usize)) == 0 {
				return i;
			}
		}
	}

	0
}
