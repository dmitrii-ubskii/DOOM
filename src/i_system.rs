#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::{fmt, process::exit, ptr::null_mut};

use libc::{calloc, gettimeofday, malloc, timeval};

use crate::{
	d_net::D_QuitNetGame,
	d_ticcmd::ticcmd_t,
	doomdef::TICRATE,
	g_game::{G_CheckDemoStatus, demorecording},
	i_sound::{I_InitSound, I_ShutdownMusic, I_ShutdownSound},
	i_video::I_ShutdownGraphics,
	m_misc::M_SaveDefaults,
};

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
		malloc(*size).cast()
	}
}

// I_GetTime
// returns time in 1/70th second tics
pub(crate) fn I_GetTime() -> usize {
	let mut tp = timeval { tv_sec: 0, tv_usec: 0 };
	static mut basetime: i32 = 0;

	unsafe {
		gettimeofday(&raw mut tp, null_mut());
		if basetime == 0 {
			basetime = tp.tv_sec;
		}

		usize::try_from(tp.tv_sec - basetime).unwrap() * TICRATE
			+ usize::try_from(tp.tv_usec).unwrap() * TICRATE / 1_000_000
	}
}

// I_Init
pub(crate) fn I_Init() {
	I_InitSound();
	//  I_InitGraphics();
}

// I_Quit
pub(crate) fn I_Quit() {
	D_QuitNetGame();
	I_ShutdownSound();
	I_ShutdownMusic();
	M_SaveDefaults();
	I_ShutdownGraphics();
	exit(0);
}

pub(crate) fn I_WaitVBL(_count: i32) {
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
	unsafe { calloc(length, 1).cast() }
}

pub(crate) fn I_Error(message: impl fmt::Display) -> ! {
	eprintln!("Error: {message}");

	unsafe {
		// Shutdown. Here might be other errors.
		if demorecording {
			G_CheckDemoStatus();
		}
	}

	D_QuitNetGame();
	I_ShutdownGraphics();

	exit(1)
}
