#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::ffi::c_char;

use crate::{
	d_think::thinker_t,
	m_fixed::{FRACUNIT, fixed_t},
	p_mobj::mobj_t,
	r_defs::{line_t, sector_t},
};

/*
#[unsafe(no_mangle)]
pub static mut levelTimer: i32 = 0;
#[unsafe(no_mangle)]
pub static mut levelTimeCount: i32 = 0;
*/

//      Define values for map objects
pub const MO_TELEPORTMAN: u32 = 14;

// P_LIGHTS
#[repr(C)]
pub struct fireflicker_t {
	pub thinker: thinker_t,
	pub sector: *mut sector_t,
	pub count: i32,
	pub maxlight: i32,
	pub minlight: i32,
}

#[repr(C)]
pub struct lightflash_t {
	pub thinker: thinker_t,
	pub sector: *mut sector_t,
	pub count: i32,
	pub maxlight: i32,
	pub minlight: i32,
	pub maxtime: i32,
	pub mintime: i32,
}

#[repr(C)]
pub struct strobe_t {
	pub thinker: thinker_t,
	pub sector: *mut sector_t,
	pub count: i32,
	pub minlight: i32,
	pub maxlight: i32,
	pub darktime: i32,
	pub brighttime: i32,
}

#[repr(C)]
pub struct glow_t {
	pub thinker: thinker_t,
	pub sector: *mut sector_t,
	pub minlight: i32,
	pub maxlight: i32,
	pub direction: i32,
}

pub const GLOWSPEED: i16 = 8;
pub const STROBEBRIGHT: i32 = 5;
pub const FASTDARK: u32 = 15;
pub const SLOWDARK: i32 = 35;

// P_SWITCH
#[repr(C)]
pub struct switchlist_t {
	pub name1: [c_char; 9],
	pub name2: [c_char; 9],
	pub episode: i16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum bwhere_e {
	top,
	middle,
	bottom,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct button_t {
	pub line: *mut line_t,
	pub where_: bwhere_e,
	pub btexture: i32,
	pub btimer: i32,
	pub soundorg: *mut mobj_t,
}

// max # of wall switches in a level
pub const MAXSWITCHES: u32 = 50;

// 4 players, 4 buttons each at once, max.
pub const MAXBUTTONS: usize = 16;

// 1 second, in ticks.
pub const BUTTONTIME: u32 = 35;

/*
#[unsafe(no_mangle)]
pub static mut buttonlist: [button_t; MAXBUTTONS] = [button_t {
	line: null_mut(),
	where_: bwhere_e::top,
	btexture: 0,
	btimer: 0,
	soundorg: null_mut(),
}; MAXBUTTONS];
*/

// P_PLATS
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum plat_e {
	up,
	down,
	waiting,
	in_stasis,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum plattype_e {
	perpetualRaise,
	downWaitUpStay,
	raiseAndChange,
	raiseToNearestAndChange,
	blazeDWUS,
}

#[repr(C)]
pub struct plat_t {
	pub thinker: thinker_t,
	pub sector: *mut sector_t,
	pub speed: fixed_t,
	pub low: fixed_t,
	pub high: fixed_t,
	pub wait: i32,
	pub count: i32,
	pub status: plat_e,
	pub oldstatus: plat_e,
	pub crush: i32,
	pub tag: i32,
	pub ty: plattype_e,
}

pub const PLATWAIT: i32 = 3;
pub const PLATSPEED: i32 = FRACUNIT;
pub const MAXPLATS: usize = 30;

/*
#[unsafe(no_mangle)]
pub static mut activeplats: [*mut plat_t; MAXPLATS] = [null_mut(); MAXPLATS];
*/

// P_DOORS
#[repr(C)]
pub enum vldoor_e {
	normal,
	close30ThenOpen,
	close,
	open,
	raiseIn5Mins,
	blazeRaise,
	blazeOpen,
	blazeClose,
}

#[repr(C)]
pub struct vldoor_t {
	pub thinker: thinker_t,
	pub ty: vldoor_e,
	pub sector: *mut sector_t,
	pub topheight: fixed_t,
	pub speed: fixed_t,

	// 1 = up, 0 = waiting at top, -1 = down
	pub direction: i32,

	// tics to wait at the top
	pub topwait: i32,
	// (keep in case a door going down is reset)
	// when it reaches 0, start going down
	pub topcountdown: i32,
}

pub const VDOORSPEED: i32 = FRACUNIT * 2;
pub const VDOORWAIT: u32 = 150;

// P_CEILNG
#[repr(C)]
pub enum ceiling_e {
	lowerToFloor,
	raiseToHighest,
	lowerAndCrush,
	crushAndRaise,
	fastCrushAndRaise,
	silentCrushAndRaise,
}

#[repr(C)]
pub struct ceiling_t {
	pub thinker: thinker_t,
	pub ty: ceiling_e,
	pub sector: *mut sector_t,
	pub bottomheight: fixed_t,
	pub topheight: fixed_t,
	pub speed: fixed_t,
	pub crush: i32,

	// 1 = up, 0 = waiting, -1 = down
	pub direction: i32,

	// ID
	pub tag: i32,
	pub olddirection: i32,
}

pub const CEILSPEED: i32 = FRACUNIT;
pub const CEILWAIT: u32 = 150;
pub const MAXCEILINGS: usize = 30;

/*
#[unsafe(no_mangle)]
pub static mut activeceilings: [*mut ceiling_t; MAXCEILINGS] = [null_mut(); MAXCEILINGS];
*/

// P_FLOOR
#[repr(C)]
pub enum floor_e {
	// lower floor to highest surrounding floor
	lowerFloor,

	// lower floor to lowest surrounding floor
	lowerFloorToLowest,

	// lower floor to highest surrounding floor VERY FAST
	turboLower,

	// raise floor to lowest surrounding CEILING
	raiseFloor,

	// raise floor to next highest surrounding floor
	raiseFloorToNearest,

	// raise floor to shortest height texture around it
	raiseToTexture,

	// lower floor to lowest surrounding floor
	//  and change floorpic
	lowerAndChange,

	raiseFloor24,
	raiseFloor24AndChange,
	raiseFloorCrush,

	// raise to next highest floor, turbo-speed
	raiseFloorTurbo,
	donutRaise,
	raiseFloor512,
}

#[repr(C)]
pub enum stair_e {
	build8,  // slowly build by 8
	turbo16, // quickly build by 16
}

#[repr(C)]
pub struct floormove_t {
	pub thinker: thinker_t,
	pub ty: floor_e,
	pub crush: i32,
	pub sector: *mut sector_t,
	pub direction: i32,
	pub newspecial: i32,
	pub texture: i16,
	pub floordestheight: fixed_t,
	pub speed: fixed_t,
}

pub const FLOORSPEED: i32 = FRACUNIT;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum result_e {
	ok,
	crushed,
	pastdest,
}
