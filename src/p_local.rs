#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::ptr::null_mut;

use crate::{
	d_think::{think_t, thinker_t},
	m_fixed::{FRACBITS, FRACUNIT, fixed_t},
	p_mobj::mobj_t,
	r_defs::line_t,
};

type boolean = i32;

pub(crate) const FLOATSPEED: i32 = FRACUNIT * 4;

pub(crate) const MAXHEALTH: i32 = 100;
pub(crate) const VIEWHEIGHT: i32 = 41 * FRACUNIT;

// mapblocks are used to check movement
// against lines and things
pub(crate) const MAPBLOCKUNITS: i32 = 128;
pub(crate) const MAPBLOCKSIZE: i32 = MAPBLOCKUNITS * FRACUNIT;
pub(crate) const MAPBLOCKSHIFT: i32 = FRACBITS + 7;
pub(crate) const MAPBTOFRAC: i32 = MAPBLOCKSHIFT - FRACBITS;

// player radius for movement checking
pub(crate) const PLAYERRADIUS: i32 = 16 * FRACUNIT;

// MAXRADIUS is for precalculated sector block boxes
// the spider demon is larger,
// but we do not have any moving sectors nearby
pub(crate) const MAXRADIUS: i32 = 32 * FRACUNIT;

pub(crate) const GRAVITY: i32 = FRACUNIT;
pub(crate) const MAXMOVE: i32 = 30 * FRACUNIT;

pub(crate) const USERANGE: i32 = 64 * FRACUNIT;
pub(crate) const MELEERANGE: i32 = 64 * FRACUNIT;
pub(crate) const MISSILERANGE: i32 = 32 * 64 * FRACUNIT;

// follow a player exlusively for 3 seconds
pub(crate) const BASETHRESHOLD: i32 = 100;

// P_TICK

// Both the head and tail of the thinker list.
pub(crate) static mut thinkercap: thinker_t =
	thinker_t { prev: null_mut(), next: null_mut(), function: think_t::null };

// P_MOBJ
pub(crate) const ONFLOORZ: i32 = i32::MIN;
pub(crate) const ONCEILINGZ: i32 = i32::MAX;

// Time interval for item respawning.
pub(crate) const ITEMQUESIZE: usize = 128;

// P_MAPUTL
#[repr(C)]
pub(crate) struct divline_t {
	pub(crate) x: fixed_t,
	pub(crate) y: fixed_t,
	pub(crate) dx: fixed_t,
	pub(crate) dy: fixed_t,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct intercept_t {
	pub(crate) frac: fixed_t, // along trace line
	pub(crate) isaline: boolean,
	pub(crate) d: intercept_t_union,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) union intercept_t_union {
	pub(crate) thing: *mut mobj_t,
	pub(crate) line: *mut line_t,
}

pub(crate) const MAXINTERCEPTS: usize = 128;

pub(crate) const PT_ADDLINES: i32 = 1;
pub(crate) const PT_ADDTHINGS: i32 = 2;
pub(crate) const PT_EARLYOUT: i32 = 4;

// P_SETUP
/*
pub(crate) static mut blockmaplump: *mut i16 = null_mut(); // offsets in blockmap are from here
pub(crate) static mut blockmap: *mut i16 = null_mut();
pub(crate) static mut bmapwidth: i32 = 0;
pub(crate) static mut bmapheight: i32 = 0; // in mapblocks
pub(crate) static mut bmaporgx: fixed_t = 0;
pub(crate) static mut bmaporgy: fixed_t = 0; // origin of block map
pub(crate) static mut blocklinks: *mut *mut mobj_t = null_mut(); // for thing chains
*/
