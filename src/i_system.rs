#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::{ffi::c_char, process::exit, ptr::null_mut};

use libc::{gettimeofday, malloc, memset, timeval};

use crate::{d_ticcmd::ticcmd_t, doomdef::TICRATE, m_misc::M_SaveDefaults};

pub(crate) static mut mb_used: usize = 6;

pub(crate) fn I_Tactile(_on: i32, _off: i32, _total: i32) {
	// UNUSED.
}

static mut emptycmd: ticcmd_t =
	ticcmd_t { forwardmove: 0, sidemove: 0, angleturn: 0, consistancy: 0, chatchar: 0, buttons: 0 };
pub(crate) fn I_BaseTiccmd() -> *const ticcmd_t {
	&raw const emptycmd
}

pub(crate) fn I_ZoneBase(size: &mut usize) -> *mut u8 {
	unsafe {
		*size = mb_used * 1024 * 1024;
		malloc(*size) as _
	}
}

// I_GetTime
// returns time in 1/70th second tics
#[unsafe(no_mangle)]
pub extern "C" fn I_GetTime() -> usize {
	let mut tp = timeval { tv_sec: 0, tv_usec: 0 };
	static mut basetime: i32 = 0;

	unsafe {
		gettimeofday(&raw mut tp, null_mut());
		if basetime == 0 {
			basetime = tp.tv_sec;
		}

		(tp.tv_sec - basetime) as usize * TICRATE + tp.tv_usec as usize * TICRATE / 1_000_000
	}
}

unsafe extern "C" {
	fn I_InitSound();
}

// I_Init
#[unsafe(no_mangle)]
pub extern "C" fn I_Init() {
	unsafe {
		I_InitSound();
		//  I_InitGraphics();
	}
}

unsafe extern "C" {
	fn D_QuitNetGame();
	fn I_ShutdownSound();
	fn I_ShutdownMusic();
	fn I_ShutdownGraphics();
}

// I_Quit
#[unsafe(no_mangle)]
pub extern "C" fn I_Quit() {
	unsafe {
		D_QuitNetGame();
		I_ShutdownSound();
		I_ShutdownMusic();
		M_SaveDefaults();
		I_ShutdownGraphics();
		exit(0);
	}
}

#[unsafe(no_mangle)]
pub extern "C" fn I_WaitVBL(_count: i32) {
	// #ifdef SGI
	//     sginap(1);
	// #else
	// #ifdef SUN
	//     sleep(0);
	// #else
	//     usleep (count * (1000000/70) );
	// #endif
	// #endif
}

pub(crate) fn I_AllocLow(length: usize) -> *mut u8 {
	unsafe {
		let mem = malloc(length);
		memset(mem, 0, length);
		mem as _
	}
}

// I_Error
unsafe extern "C" {
	pub fn I_Error(error: *const c_char, ...) -> !;
}
