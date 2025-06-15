#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::ptr::null_mut;

use crate::{
	d_think::{actionf_t, thinker_t},
	m_fixed::{FRACBITS, FRACUNIT, fixed_t},
	p_mobj::mobj_t,
	r_defs::line_t,
};

pub const FLOATSPEED: i32 = FRACUNIT * 4;

pub const MAXHEALTH: i32 = 100;
pub const VIEWHEIGHT: i32 = 41 * FRACUNIT;

// mapblocks are used to check movement
// against lines and things
pub const MAPBLOCKUNITS: i32 = 128;
pub const MAPBLOCKSIZE: i32 = MAPBLOCKUNITS * FRACUNIT;
pub const MAPBLOCKSHIFT: i32 = FRACBITS + 7;
pub const MAPBMASK: i32 = MAPBLOCKSIZE - 1;
pub const MAPBTOFRAC: i32 = MAPBLOCKSHIFT - FRACBITS;

// player radius for movement checking
pub const PLAYERRADIUS: i32 = 16 * FRACUNIT;

// MAXRADIUS is for precalculated sector block boxes
// the spider demon is larger,
// but we do not have any moving sectors nearby
pub const MAXRADIUS: i32 = 32 * FRACUNIT;

pub const GRAVITY: i32 = FRACUNIT;
pub const MAXMOVE: i32 = 30 * FRACUNIT;

pub const USERANGE: i32 = 64 * FRACUNIT;
pub const MELEERANGE: i32 = 64 * FRACUNIT;
pub const MISSILERANGE: i32 = 32 * 64 * FRACUNIT;

// follow a player exlusively for 3 seconds
pub const BASETHRESHOLD: i32 = 100;

// P_TICK

// Both the head and tail of the thinker list.
#[unsafe(no_mangle)]
pub static mut thinkercap: thinker_t =
	thinker_t { prev: null_mut(), next: null_mut(), function: actionf_t { acv: None } };

//
// P_MOBJ
//
pub const ONFLOORZ: i32 = i32::MIN;
pub const ONCEILINGZ: i32 = i32::MAX;

// Time interval for item respawning.
pub const ITEMQUESIZE: usize = 128;

// P_MAPUTL
#[repr(C)]
pub struct divline_t {
	pub x: fixed_t,
	pub y: fixed_t,
	pub dx: fixed_t,
	pub dy: fixed_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct intercept_t {
	frac: fixed_t, // along trace line
	isaline: i32,
	d: intercept_t_union,
}

#[repr(C)]
#[derive(Clone, Copy)]
union intercept_t_union {
	thing: *mut mobj_t,
	line: *mut line_t,
}

pub const MAXINTERCEPTS: usize = 128;

/*
#[unsafe(no_mangle)]
pub static mut intercepts: [intercept_t; MAXINTERCEPTS] =
	[intercept_t { frac: 0, isaline: 0, d: intercept_t_union { thing: null_mut() } };
		MAXINTERCEPTS];
#[unsafe(no_mangle)]
pub static mut intercept_p: *mut intercept_t = null_mut();
*/

unsafe extern "C" {
	pub static mut opentop: fixed_t;
	pub static mut openbottom: fixed_t;
}

/*
#[unsafe(no_mangle)]
pub static mut opentop: fixed_t = 0;
#[unsafe(no_mangle)]
pub static mut openbottom: fixed_t = 0;
#[unsafe(no_mangle)]
pub static mut openrange: fixed_t = 0;
#[unsafe(no_mangle)]
pub static mut lowfloor: fixed_t = 0;
*/

pub const PT_ADDLINES: i32 = 1;
pub const PT_ADDTHINGS: i32 = 2;
pub const PT_EARLYOUT: i32 = 4;

/*
#[unsafe(no_mangle)]
pub static mut trace: divline_t = divline_t { x: 0, y: 0, dx: 0, dy: 0 };

// P_MAP

// If "floatok" true, move would be ok
// if within "tmfloorz - tmceilingz".
#[unsafe(no_mangle)]
pub static mut floatok: i32 = 0;
#[unsafe(no_mangle)]
pub static mut tmfloorz: fixed_t = 0;
#[unsafe(no_mangle)]
pub static mut tmceilingz: fixed_t = 0;

#[unsafe(no_mangle)]
pub static mut ceilingline: *mut line_t = null_mut();

#[unsafe(no_mangle)]
pub static mut linetarget: *mut mobj_t = null_mut(); // who got hit (or NULL)
*/

// P_SETUP
/*
#[unsafe(no_mangle)]
pub static mut blockmaplump: *mut i16 = null_mut(); // offsets in blockmap are from here
#[unsafe(no_mangle)]
pub static mut blockmap: *mut i16 = null_mut();
#[unsafe(no_mangle)]
pub static mut bmapwidth: i32 = 0;
#[unsafe(no_mangle)]
pub static mut bmapheight: i32 = 0; // in mapblocks
#[unsafe(no_mangle)]
pub static mut bmaporgx: fixed_t = 0;
#[unsafe(no_mangle)]
pub static mut bmaporgy: fixed_t = 0; // origin of block map
#[unsafe(no_mangle)]
pub static mut blocklinks: *mut *mut mobj_t = null_mut(); // for thing chains
*/
