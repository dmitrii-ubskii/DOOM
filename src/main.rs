#![allow(non_upper_case_globals)]

pub mod d_englsh;
pub mod d_event;
pub mod d_items;
pub mod d_main;
pub mod d_net;
pub mod d_player;
pub mod d_think;
pub mod d_ticcmd;
pub mod doomdata;
pub mod doomdef;
pub mod doomstat;
pub mod dstrings;
pub mod f_wipe;
pub mod i_system;
pub mod info;
pub mod m_argv;
pub mod m_bbox;
pub mod m_cheat;
pub mod m_fixed;
pub mod m_random;
pub mod m_swap;
pub mod p_lights;
pub mod p_local;
pub mod p_mobj;
pub mod p_pspr;
pub mod p_sight;
pub mod p_spec;
pub mod p_telept;
pub mod p_tick;
pub mod p_user;
pub mod r_defs;
pub mod r_sky;
pub mod r_state;
pub mod sounds;
pub mod st_lib;
pub mod st_stuff;
pub mod tables;
pub mod v_video;
pub mod w_wad;
pub mod z_zone;

use std::{
	env,
	ffi::{CString, c_char},
	ptr::null_mut,
};

use d_main::D_DoomMain;

#[unsafe(no_mangle)]
static mut myargc: usize = 0;
#[unsafe(no_mangle)]
static mut myargv: *mut *mut c_char = null_mut();

fn main() {
	let args: Vec<_> = env::args().map(|arg| CString::new(arg).unwrap()).collect();
	let argv: Vec<_> = args.iter().map(|cstring| cstring.as_ptr()).collect();
	unsafe {
		myargc = args.len();
		myargv = argv.as_ptr() as *mut *mut i8; // pinky promise not to mutate
		D_DoomMain();
	}
}
