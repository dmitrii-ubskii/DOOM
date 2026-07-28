#![allow(non_upper_case_globals)]

pub(crate) mod am_map;
pub(crate) mod d_englsh;
pub(crate) mod d_event;
pub(crate) mod d_items;
pub(crate) mod d_main;
pub(crate) mod d_net;
pub(crate) mod d_player;
pub(crate) mod d_think;
pub(crate) mod d_ticcmd;
pub(crate) mod doomdata;
pub(crate) mod doomdef;
pub(crate) mod doomstat;
pub(crate) mod dstrings;
pub(crate) mod f_finale;
pub(crate) mod f_wipe;
pub(crate) mod g_game;
pub(crate) mod hu_lib;
pub(crate) mod hu_stuff;
pub(crate) mod i_net;
pub(crate) mod i_sound;
pub(crate) mod i_system;
pub(crate) mod i_video;
pub(crate) mod info;
pub(crate) mod m_argv;
pub(crate) mod m_bbox;
pub(crate) mod m_cheat;
pub(crate) mod m_fixed;
pub(crate) mod m_menu;
pub(crate) mod m_misc;
pub(crate) mod m_random;
pub(crate) mod p_ceiling;
pub(crate) mod p_doors;
pub(crate) mod p_enemy;
pub(crate) mod p_floor;
pub(crate) mod p_inter;
pub(crate) mod p_lights;
pub(crate) mod p_local;
pub(crate) mod p_map;
pub(crate) mod p_maputl;
pub(crate) mod p_mobj;
pub(crate) mod p_plats;
pub(crate) mod p_pspr;
pub(crate) mod p_saveg;
pub(crate) mod p_setup;
pub(crate) mod p_sight;
pub(crate) mod p_spec;
pub(crate) mod p_switch;
pub(crate) mod p_telept;
pub(crate) mod p_tick;
pub(crate) mod p_user;
pub(crate) mod r_bsp;
pub(crate) mod r_data;
pub(crate) mod r_defs;
pub(crate) mod r_draw;
pub(crate) mod r_main;
pub(crate) mod r_plane;
pub(crate) mod r_segs;
pub(crate) mod r_sky;
pub(crate) mod r_things;
pub(crate) mod s_sound;
pub(crate) mod sounds;
pub(crate) mod st_lib;
pub(crate) mod st_stuff;
pub(crate) mod tables;
pub(crate) mod v_video;
pub(crate) mod w_wad;
pub(crate) mod wi_stuff;
pub(crate) mod z_zone;

pub(crate) mod const_conv;

use std::{
	env,
	ffi::{CString, c_char},
	ptr::null_mut,
};

use d_main::D_DoomMain;

static mut myargc: usize = 0;
static mut myargv: *mut *mut c_char = null_mut();

fn main() {
	let args: Vec<_> = env::args().map(|arg| CString::new(arg).unwrap()).collect();
	let argv: Vec<_> = args.iter().map(|cstring| cstring.as_ptr()).collect();
	unsafe {
		myargc = args.len();
		myargv = argv.as_ptr().cast_mut().cast::<*mut i8>(); // pinky promise not to mutate
		D_DoomMain();
	}
}
