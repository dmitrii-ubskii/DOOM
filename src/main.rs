#![allow(non_upper_case_globals)]

pub mod d_player;
pub mod d_think;
pub mod d_ticcmd;
pub mod doomdata;
pub mod doomdef;
pub mod doomstat;
pub mod dstrings;
pub mod i_system;
pub mod info;
pub mod m_argv;
pub mod m_bbox;
pub mod m_cheat;
pub mod m_fixed;
pub mod m_random;
pub mod m_swap;
pub mod p_mobj;
pub mod p_pspr;
pub mod p_telept;
pub mod r_defs;
pub mod r_sky;
pub mod sounds;
pub mod tables;

use std::{
	env,
	ffi::{CString, c_char},
	ptr::null,
};

#[unsafe(no_mangle)]
static mut myargc: i32 = 0;
#[unsafe(no_mangle)]
static mut myargv: *const *const c_char = null();

unsafe extern "C" {
	fn D_DoomMain();
}

fn main() {
	let args: Vec<_> = env::args().map(|arg| CString::new(arg).unwrap()).collect();
	let argv: Vec<_> = args.iter().map(|cstring| cstring.as_ptr()).collect();
	unsafe {
		myargc = args.len() as i32;
		myargv = argv.as_ptr();
		D_DoomMain();
	}
}
