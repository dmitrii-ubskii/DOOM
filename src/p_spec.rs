#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::{ffi::c_int, ptr::null_mut};

use crate::{
	d_player::{CF_GODMODE, player_t},
	d_think::{think_t, thinker_t},
	doomdata::ML_TWOSIDED,
	doomdef::{MAXPLAYERS, powertype_t},
	g_game::{G_ExitLevel, G_SecretExitLevel, deathmatch, totalsecret},
	i_system::I_Error,
	info::mobjtype_t,
	m_argv::M_CheckParm,
	m_fixed::{FRACUNIT, fixed_t},
	m_random::P_Random,
	myargv,
	p_ceiling::{EV_CeilingCrushStop, EV_DoCeiling, activeceilings},
	p_doors::{EV_DoDoor, P_SpawnDoorCloseIn30, P_SpawnDoorRaiseIn5Mins},
	p_floor::{EV_BuildStairs, EV_DoFloor},
	p_inter::P_DamageMobj,
	p_lights::{
		EV_LightTurnOn, EV_StartLightStrobing, EV_TurnTagLightsOff, P_SpawnFireFlicker,
		P_SpawnGlowingLight, P_SpawnLightFlash, P_SpawnStrobeFlash,
	},
	p_mobj::mobj_t,
	p_plats::{EV_DoPlat, EV_StopPlat, activeplats},
	p_setup::{lines, numlines, numsectors, sectors, sides},
	p_switch::{P_ChangeSwitchTexture, buttonlist},
	p_telept::EV_Teleport,
	p_tick::{P_AddThinker, leveltime},
	r_data::{
		R_CheckTextureNumForName, R_FlatNumForName, R_TextureNumForName, flattranslation,
		texturetranslation,
	},
	r_defs::{line_t, sector_t, side_t},
	s_sound::S_StartSound,
	sounds::sfxenum_t,
	w_wad::W_CheckNumForName,
	z_zone::{PU_LEVSPEC, Z_Malloc},
};

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
pub const FASTDARK: i32 = 15;
pub const SLOWDARK: i32 = 35;

// P_SWITCH
#[repr(C)]
pub struct switchlist_t {
	pub name1: [u8; 9],
	pub name2: [u8; 9],
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
	pub btimer: u32,
	pub soundorg: *mut mobj_t,
}

impl button_t {
	pub const fn new() -> Self {
		Self {
			line: null_mut(),
			where_: bwhere_e::top,
			btexture: 0,
			btimer: 0,
			soundorg: null_mut(),
		}
	}
}

impl Default for button_t {
	fn default() -> Self {
		Self::new()
	}
}

// max # of wall switches in a level
pub const MAXSWITCHES: usize = 50;

// 4 players, 4 buttons each at once, max.
pub const MAXBUTTONS: usize = 16;

// 1 second, in ticks.
pub const BUTTONTIME: u32 = 35;

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

// P_DOORS
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
	pub topwait: u32,
	// (keep in case a door going down is reset)
	// when it reaches 0, start going down
	pub topcountdown: u32,
}

pub const VDOORSPEED: i32 = FRACUNIT * 2;
pub const VDOORWAIT: u32 = 150;

// P_CEILNG
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

// P_FLOOR
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
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

// Animating textures and planes
// There is another anim_t used in wi_stuff, unrelated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct anim_t {
	pub istexture: bool,
	pub picnum: usize,
	pub basepic: usize,
	pub numpics: usize,
	pub speed: usize,
}

//      source animation definition
struct animdef_t {
	pub istexture: bool, // if false, it is a flat
	pub endname: [u8; 9],
	pub startname: [u8; 9],
	pub speed: usize,
}

const MAXANIMS: usize = 32;

// P_InitPicAnims

// Floor/ceiling animation sequences,
//  defined by first and last frame,
//  i.e. the flat (64x64 tile) name to
//  be used.
// The full animation sequence is given
//  using all the flats between the start
//  and end entry, in the order found in
//  the WAD file.
static animdefs: [animdef_t; 23] = [
	animdef_t { istexture: false, endname: *b"NUKAGE3\0\0", startname: *b"NUKAGE1\0\0", speed: 8 },
	animdef_t { istexture: false, endname: *b"FWATER4\0\0", startname: *b"FWATER1\0\0", speed: 8 },
	animdef_t { istexture: false, endname: *b"SWATER4\0\0", startname: *b"SWATER1\0\0", speed: 8 },
	animdef_t {
		istexture: false,
		endname: *b"LAVA4\0\0\0\0",
		startname: *b"LAVA1\0\0\0\0",
		speed: 8,
	},
	animdef_t {
		istexture: false,
		endname: *b"BLOOD3\0\0\0",
		startname: *b"BLOOD1\0\0\0",
		speed: 8,
	},
	// DOOM II flat animations.
	animdef_t { istexture: false, endname: *b"RROCK08\0\0", startname: *b"RROCK05\0\0", speed: 8 },
	animdef_t { istexture: false, endname: *b"SLIME04\0\0", startname: *b"SLIME01\0\0", speed: 8 },
	animdef_t { istexture: false, endname: *b"SLIME08\0\0", startname: *b"SLIME05\0\0", speed: 8 },
	animdef_t { istexture: false, endname: *b"SLIME12\0\0", startname: *b"SLIME09\0\0", speed: 8 },
	animdef_t { istexture: true, endname: *b"BLODGR4\0\0", startname: *b"BLODGR1\0\0", speed: 8 },
	animdef_t { istexture: true, endname: *b"SLADRIP3\0", startname: *b"SLADRIP1\0", speed: 8 },
	animdef_t { istexture: true, endname: *b"BLODRIP4\0", startname: *b"BLODRIP1\0", speed: 8 },
	animdef_t { istexture: true, endname: *b"FIREWALL\0", startname: *b"FIREWALA\0", speed: 8 },
	animdef_t { istexture: true, endname: *b"GSTFONT3\0", startname: *b"GSTFONT1\0", speed: 8 },
	animdef_t { istexture: true, endname: *b"FIRELAVA\0", startname: *b"FIRELAV3\0", speed: 8 },
	animdef_t { istexture: true, endname: *b"FIREMAG3\0", startname: *b"FIREMAG1\0", speed: 8 },
	animdef_t { istexture: true, endname: *b"FIREBLU2\0", startname: *b"FIREBLU1\0", speed: 8 },
	animdef_t { istexture: true, endname: *b"ROCKRED3\0", startname: *b"ROCKRED1\0", speed: 8 },
	animdef_t { istexture: true, endname: *b"BFALL4\0\0\0", startname: *b"BFALL1\0\0\0", speed: 8 },
	animdef_t { istexture: true, endname: *b"SFALL4\0\0\0", startname: *b"SFALL1\0\0\0", speed: 8 },
	animdef_t { istexture: true, endname: *b"WFALL4\0\0\0", startname: *b"WFALL1\0\0\0", speed: 8 },
	animdef_t { istexture: true, endname: *b"DBRAIN4\0\0", startname: *b"DBRAIN1\0\0", speed: 8 },
	animdef_t { istexture: false, endname: [0; 9], startname: [0; 9], speed: 0 },
];

static mut anims: [anim_t; MAXANIMS] =
	[anim_t { istexture: false, picnum: 0, basepic: 0, numpics: 0, speed: 0 }; MAXANIMS];
static mut lastanim: *mut anim_t = null_mut();

//      Animating line specials
const MAXLINEANIMS: usize = 64;

#[allow(static_mut_refs)]
pub(crate) fn P_InitPicAnims() {
	unsafe {
		//	Init animation
		lastanim = anims.as_mut_ptr();
		let mut j = 0;
		while animdefs[j].endname[0] != 0 {
			let i = j;
			j += 1;
			if animdefs[i].istexture {
				// different episode ?
				if R_CheckTextureNumForName(animdefs[i].startname.as_ptr().cast()) == -1 {
					continue;
				}

				(*lastanim).picnum = R_TextureNumForName(animdefs[i].endname.as_ptr().cast());
				(*lastanim).basepic = R_TextureNumForName(animdefs[i].startname.as_ptr().cast());
			} else {
				if W_CheckNumForName(animdefs[i].startname.as_ptr().cast()) == -1 {
					continue;
				}

				(*lastanim).picnum = R_FlatNumForName(animdefs[i].endname.as_ptr().cast());
				(*lastanim).basepic = R_FlatNumForName(animdefs[i].startname.as_ptr().cast());
			}

			(*lastanim).istexture = animdefs[i].istexture;
			(*lastanim).numpics = (*lastanim).picnum - (*lastanim).basepic + 1;

			if (*lastanim).numpics < 2 {
				I_Error(
					c"P_InitPicAnims: bad cycle from %s to %s".as_ptr(),
					animdefs[i].startname,
					animdefs[i].endname,
				);
			}

			(*lastanim).speed = animdefs[i].speed;
			lastanim = lastanim.add(1);
		}
	}
}

// UTILITIES

// getSide()
// Will return a side_t*
//  given the number of the current sector,
//  the line number, and the side (0/1) that you want.
pub(crate) fn getSide(currentSector: isize, line: usize, side: usize) -> *mut side_t {
	unsafe {
		let sector = sectors.wrapping_add(currentSector as usize);
		let line = *(*sector).lines.wrapping_add(line);
		let side = (*line).sidenum[side] as usize;
		sides.wrapping_add(side)
	}
}

// getSector()
// Will return a sector_t*
//  given the number of the current sector,
//  the line number and the side (0/1) that you want.
pub(crate) fn getSector(currentSector: isize, line: usize, side: usize) -> *mut sector_t {
	unsafe { (*getSide(currentSector, line, side)).sector }
}

// twoSided()
// Given the sector number and the line number,
//  it will tell you whether the line is two-sided or not.
pub(crate) fn twoSided(sector: isize, line: usize) -> bool {
	unsafe {
		let sector = sectors.wrapping_add(sector as usize);
		let line = *(*sector).lines.wrapping_add(line);
		(*line).flags & ML_TWOSIDED != 0
	}
}

// getNextSector()
// Return sector_t * of sector next to current.
// null_mut() if not two-sided line
pub(crate) fn getNextSector(line: *mut line_t, sec: *mut sector_t) -> *mut sector_t {
	unsafe {
		if (*line).flags & ML_TWOSIDED == 0 {
			return null_mut();
		}
		if (*line).frontsector == sec { (*line).backsector } else { (*line).frontsector }
	}
}

// P_FindLowestFloorSurrounding()
// FIND LOWEST FLOOR HEIGHT IN SURROUNDING SECTORS
#[unsafe(no_mangle)]
pub extern "C" fn P_FindLowestFloorSurrounding(sec: &mut sector_t) -> fixed_t {
	unsafe {
		let mut floor = sec.floorheight;

		for i in 0..sec.linecount {
			let check = *sec.lines.wrapping_add(i);
			let other = getNextSector(check, sec);

			if other.is_null() {
				continue;
			}

			if (*other).floorheight < floor {
				floor = (*other).floorheight;
			}
		}

		floor
	}
}

// P_FindHighestFloorSurrounding()
// FIND HIGHEST FLOOR HEIGHT IN SURROUNDING SECTORS
pub(crate) fn P_FindHighestFloorSurrounding(sec: &mut sector_t) -> fixed_t {
	unsafe {
		let mut floor = -500 * FRACUNIT;

		for i in 0..sec.linecount {
			let check = *sec.lines.wrapping_add(i);
			let other = getNextSector(check, sec);

			if other.is_null() {
				continue;
			}

			if (*other).floorheight > floor {
				floor = (*other).floorheight;
			}
		}
		floor
	}
}

// P_FindNextHighestFloor
// FIND NEXT HIGHEST FLOOR IN SURROUNDING SECTORS
// Note: this should be doable w/o a fixed array.

// 20 adjoining sectors max!
const MAX_ADJOINING_SECTORS: usize = 20;

pub(crate) fn P_FindNextHighestFloor(sec: &mut sector_t, currentheight: fixed_t) -> fixed_t {
	unsafe {
		let mut heightlist = [0; MAX_ADJOINING_SECTORS];
		let mut h = 0;

		for i in 0..sec.linecount {
			let check = *sec.lines.wrapping_add(i);
			let other = getNextSector(check, sec);

			if other.is_null() {
				continue;
			}

			if (*other).floorheight > currentheight {
				heightlist[h] = (*other).floorheight;
				h += 1;
			}

			// Check for overflow. Exit.
			if h >= MAX_ADJOINING_SECTORS {
				eprintln!("Sector with more than 20 adjoining sectors");
				break;
			}
		}

		// Find lowest height in list
		if h == 0 {
			return currentheight;
		}

		let mut min = heightlist[0];

		// Range checking?
		for &height in &heightlist[1..h] {
			if height < min {
				min = height;
			}
		}

		min
	}
}

// FIND LOWEST CEILING IN THE SURROUNDING SECTORS
#[unsafe(no_mangle)]
pub extern "C" fn P_FindLowestCeilingSurrounding(sec: &mut sector_t) -> fixed_t {
	unsafe {
		let mut height = fixed_t::MAX;

		for i in 0..sec.linecount {
			let check = *sec.lines.wrapping_add(i);
			let other = getNextSector(check, sec);

			if other.is_null() {
				continue;
			}

			if (*other).ceilingheight < height {
				height = (*other).ceilingheight;
			}
		}

		height
	}
}

// FIND HIGHEST CEILING IN THE SURROUNDING SECTORS
pub(crate) fn P_FindHighestCeilingSurrounding(sec: &mut sector_t) -> fixed_t {
	unsafe {
		let mut height = 0;

		for i in 0..sec.linecount {
			let check = *sec.lines.wrapping_add(i);
			let other = getNextSector(check, sec);

			if other.is_null() {
				continue;
			}

			if (*other).ceilingheight > height {
				height = (*other).ceilingheight;
			}
		}

		height
	}
}

// RETURN NEXT SECTOR # THAT LINE TAG REFERS TO
pub(crate) fn P_FindSectorFromLineTag(line: &mut line_t, start: isize) -> isize {
	unsafe {
		for i in (start + 1) as usize..numsectors {
			if (*sectors.wrapping_add(i)).tag == line.tag {
				return i as isize;
			}
		}
	}

	-1
}

// Find minimum light from an adjacent sector
pub(crate) fn P_FindMinSurroundingLight(sector: &mut sector_t, max: i32) -> i32 {
	unsafe {
		let mut min = max;
		for i in 0..sector.linecount {
			let line = *sector.lines.wrapping_add(i);
			let check = getNextSector(line, sector);

			if check.is_null() {
				continue;
			}

			min = i32::min(min, (*check).lightlevel as i32);
		}
		min
	}
}

// EVENTS
// Events are operations triggered by using, crossing,
// or shooting special lines, or by timed thinkers.

// P_CrossSpecialLine - TRIGGER
// Called every time a thing origin is about
//  to cross a line with a non 0 special.
#[unsafe(no_mangle)]
pub extern "C" fn P_CrossSpecialLine(linenum: usize, side: usize, thing: &mut mobj_t) {
	unsafe {
		let line = &mut *lines.wrapping_add(linenum);

		//	Triggers that other things can activate
		if thing.player.is_null() {
			// Things that should NOT trigger specials...
			match thing.ty {
				mobjtype_t::MT_ROCKET
				| mobjtype_t::MT_PLASMA
				| mobjtype_t::MT_BFG
				| mobjtype_t::MT_TROOPSHOT
				| mobjtype_t::MT_HEADSHOT
				| mobjtype_t::MT_BRUISERSHOT => return,

				_ => (),
			}

			let mut ok = false;
			match line.special {
				// 39: TELEPORT TRIGGER
				// 97: TELEPORT RETRIGGER
				// 125: TELEPORT MONSTERONLY TRIGGER
				// 126: TELEPORT MONSTERONLY RETRIGGER
				// 4: RAISE DOOR
				// 10: PLAT DOWN-WAIT-UP-STAY TRIGGER
				// 88: PLAT DOWN-WAIT-UP-STAY RETRIGGER
				39 | 97 | 125 | 126 | 4 | 10 | 88 => ok = true,
				_ => (),
			}
			if !ok {
				return;
			}
		}

		// Note: could use some const's here.
		match line.special {
			// TRIGGERS.
			// All from here to RETRIGGERS.
			2 => {
				// Open Door
				EV_DoDoor(line, vldoor_e::open);
				line.special = 0;
			}

			3 => {
				// Close Door
				EV_DoDoor(line, vldoor_e::close);
				line.special = 0;
			}

			4 => {
				// Raise Door
				EV_DoDoor(line, vldoor_e::normal);
				line.special = 0;
			}

			5 => {
				// Raise Floor
				EV_DoFloor(line, floor_e::raiseFloor);
				line.special = 0;
			}

			6 => {
				// Fast Ceiling Crush & Raise
				EV_DoCeiling(line, ceiling_e::fastCrushAndRaise);
				line.special = 0;
			}

			8 => {
				// Build Stairs
				EV_BuildStairs(line, stair_e::build8);
				line.special = 0;
			}

			10 => {
				// PlatDownWaitUp
				EV_DoPlat(line, plattype_e::downWaitUpStay, 0);
				line.special = 0;
			}

			12 => {
				// Light Turn On - brightest near
				EV_LightTurnOn(line, 0);
				line.special = 0;
			}

			13 => {
				// Light Turn On 255
				EV_LightTurnOn(line, 255);
				line.special = 0;
			}

			16 => {
				// Close Door 30
				EV_DoDoor(line, vldoor_e::close30ThenOpen);
				line.special = 0;
			}

			17 => {
				// Start Light Strobing
				EV_StartLightStrobing(line);
				line.special = 0;
			}

			19 => {
				// Lower Floor
				EV_DoFloor(line, floor_e::lowerFloor);
				line.special = 0;
			}

			22 => {
				// Raise floor to nearest height and change texture
				EV_DoPlat(line, plattype_e::raiseToNearestAndChange, 0);
				line.special = 0;
			}

			25 => {
				// Ceiling Crush and Raise
				EV_DoCeiling(line, ceiling_e::crushAndRaise);
				line.special = 0;
			}

			30 => {
				// Raise floor to shortest texture height
				//  on either side of lines.
				EV_DoFloor(line, floor_e::raiseToTexture);
				line.special = 0;
			}

			35 => {
				// Lights Very Dark
				EV_LightTurnOn(line, 35);
				line.special = 0;
			}

			36 => {
				// Lower Floor (TURBO)
				EV_DoFloor(line, floor_e::turboLower);
				line.special = 0;
			}

			37 => {
				// LowerAndChange
				EV_DoFloor(line, floor_e::lowerAndChange);
				line.special = 0;
			}

			38 => {
				// Lower Floor To Lowest
				EV_DoFloor(line, floor_e::lowerFloorToLowest);
				line.special = 0;
			}

			39 => {
				// TELEPORT!
				EV_Teleport(line, side, thing);
				line.special = 0;
			}

			40 => {
				// RaiseCeilingLowerFloor
				EV_DoCeiling(line, ceiling_e::raiseToHighest);
				EV_DoFloor(line, floor_e::lowerFloorToLowest);
				line.special = 0;
			}

			44 => {
				// Ceiling Crush
				EV_DoCeiling(line, ceiling_e::lowerAndCrush);
				line.special = 0;
			}

			52 => {
				// EXIT!
				G_ExitLevel();
			}

			53 => {
				// Perpetual Platform Raise
				EV_DoPlat(line, plattype_e::perpetualRaise, 0);
				line.special = 0;
			}

			54 => {
				// Platform Stop
				EV_StopPlat(line);
				line.special = 0;
			}

			56 => {
				// Raise Floor Crush
				EV_DoFloor(line, floor_e::raiseFloorCrush);
				line.special = 0;
			}

			57 => {
				// Ceiling Crush Stop
				EV_CeilingCrushStop(line);
				line.special = 0;
			}

			58 => {
				// Raise Floor 24
				EV_DoFloor(line, floor_e::raiseFloor24);
				line.special = 0;
			}

			59 => {
				// Raise Floor 24 And Change
				EV_DoFloor(line, floor_e::raiseFloor24AndChange);
				line.special = 0;
			}

			104 => {
				// Turn lights off in sector(tag)
				EV_TurnTagLightsOff(line);
				line.special = 0;
			}

			108 => {
				// Blazing Door Raise (faster than TURBO!)
				EV_DoDoor(line, vldoor_e::blazeRaise);
				line.special = 0;
			}

			109 => {
				// Blazing Door Open (faster than TURBO!)
				EV_DoDoor(line, vldoor_e::blazeOpen);
				line.special = 0;
			}

			100 => {
				// Build Stairs Turbo 16
				EV_BuildStairs(line, stair_e::turbo16);
				line.special = 0;
			}

			110 => {
				// Blazing Door Close (faster than TURBO!)
				EV_DoDoor(line, vldoor_e::blazeClose);
				line.special = 0;
			}

			119 => {
				// Raise floor to nearest surr. floor
				EV_DoFloor(line, floor_e::raiseFloorToNearest);
				line.special = 0;
			}

			121 => {
				// Blazing PlatDownWaitUpStay
				EV_DoPlat(line, plattype_e::blazeDWUS, 0);
				line.special = 0;
			}

			124 => {
				// Secret EXIT
				G_SecretExitLevel();
			}

			125 => {
				// TELEPORT MonsterONLY
				if thing.player.is_null() {
					EV_Teleport(line, side, thing);
					line.special = 0;
				}
			}

			130 => {
				// Raise Floor Turbo
				EV_DoFloor(line, floor_e::raiseFloorTurbo);
				line.special = 0;
			}

			141 => {
				// Silent Ceiling Crush & Raise
				EV_DoCeiling(line, ceiling_e::silentCrushAndRaise);
				line.special = 0;
			}

			// RETRIGGERS.  All from here till end.
			72 => {
				// Ceiling Crush
				EV_DoCeiling(line, ceiling_e::lowerAndCrush);
			}

			73 => {
				// Ceiling Crush and Raise
				EV_DoCeiling(line, ceiling_e::crushAndRaise);
			}

			74 => {
				// Ceiling Crush Stop
				EV_CeilingCrushStop(line);
			}

			75 => {
				// Close Door
				EV_DoDoor(line, vldoor_e::close);
			}

			76 => {
				// Close Door 30
				EV_DoDoor(line, vldoor_e::close30ThenOpen);
			}

			77 => {
				// Fast Ceiling Crush & Raise
				EV_DoCeiling(line, ceiling_e::fastCrushAndRaise);
			}

			79 => {
				// Lights Very Dark
				EV_LightTurnOn(line, 35);
			}

			80 => {
				// Light Turn On - brightest near
				EV_LightTurnOn(line, 0);
			}

			81 => {
				// Light Turn On 255
				EV_LightTurnOn(line, 255);
			}

			82 => {
				// Lower Floor To Lowest
				EV_DoFloor(line, floor_e::lowerFloorToLowest);
			}

			83 => {
				// Lower Floor
				EV_DoFloor(line, floor_e::lowerFloor);
			}

			84 => {
				// LowerAndChange
				EV_DoFloor(line, floor_e::lowerAndChange);
			}

			86 => {
				// Open Door
				EV_DoDoor(line, vldoor_e::open);
			}

			87 => {
				// Perpetual Platform Raise
				EV_DoPlat(line, plattype_e::perpetualRaise, 0);
			}

			88 => {
				// PlatDownWaitUp
				EV_DoPlat(line, plattype_e::downWaitUpStay, 0);
			}

			89 => {
				// Platform Stop
				EV_StopPlat(line);
			}

			90 => {
				// Raise Door
				EV_DoDoor(line, vldoor_e::normal);
			}

			91 => {
				// Raise Floor
				EV_DoFloor(line, floor_e::raiseFloor);
			}

			92 => {
				// Raise Floor 24
				EV_DoFloor(line, floor_e::raiseFloor24);
			}

			93 => {
				// Raise Floor 24 And Change
				EV_DoFloor(line, floor_e::raiseFloor24AndChange);
			}

			94 => {
				// Raise Floor Crush
				EV_DoFloor(line, floor_e::raiseFloorCrush);
			}

			95 => {
				// Raise floor to nearest height
				// and change texture.
				EV_DoPlat(line, plattype_e::raiseToNearestAndChange, 0);
			}

			96 => {
				// Raise floor to shortest texture height
				// on either side of lines.
				EV_DoFloor(line, floor_e::raiseToTexture);
			}

			97 => {
				// TELEPORT!
				EV_Teleport(line, side, thing);
			}

			98 => {
				// Lower Floor (TURBO)
				EV_DoFloor(line, floor_e::turboLower);
			}

			105 => {
				// Blazing Door Raise (faster than TURBO!)
				EV_DoDoor(line, vldoor_e::blazeRaise);
			}

			106 => {
				// Blazing Door Open (faster than TURBO!)
				EV_DoDoor(line, vldoor_e::blazeOpen);
			}

			107 => {
				// Blazing Door Close (faster than TURBO!)
				EV_DoDoor(line, vldoor_e::blazeClose);
			}

			120 => {
				// Blazing PlatDownWaitUpStay.
				EV_DoPlat(line, plattype_e::blazeDWUS, 0);
			}

			126 => {
				// TELEPORT MonsterONLY.
				if thing.player.is_null() {
					EV_Teleport(line, side, thing);
				}
			}

			128 => {
				// Raise To Nearest Floor
				EV_DoFloor(line, floor_e::raiseFloorToNearest);
			}

			129 => {
				// Raise Floor Turbo
				EV_DoFloor(line, floor_e::raiseFloorTurbo);
			}

			_ => (),
		}
	}
}

// P_ShootSpecialLine - IMPACT SPECIALS
// Called when a thing shoots a special line.
#[unsafe(no_mangle)]
pub extern "C" fn P_ShootSpecialLine(thing: &mut mobj_t, line: &mut line_t) {
	//	Impacts that other things can activate.
	if thing.player.is_null() {
		let mut ok = false;
		match line.special {
			46 => ok = true, // OPEN DOOR IMPACT
			_ => (),
		}
		if !ok {
			return;
		}
	}

	match line.special {
		24 => {
			// RAISE FLOOR
			EV_DoFloor(line, floor_e::raiseFloor);
			P_ChangeSwitchTexture(line, false);
		}

		46 => {
			// OPEN DOOR
			EV_DoDoor(line, vldoor_e::open);
			P_ChangeSwitchTexture(line, true);
		}

		47 => {
			// RAISE FLOOR NEAR AND CHANGE
			EV_DoPlat(line, plattype_e::raiseToNearestAndChange, 0);
			P_ChangeSwitchTexture(line, false);
		}
		_ => (),
	}
}

// P_PlayerInSpecialSector
// Called every tic frame
//  that the player origin is in a special sector
pub(crate) fn P_PlayerInSpecialSector(player: &mut player_t) {
	unsafe {
		let sector = &mut *(*(*player.mo).subsector).sector;

		// Falling, not all the way down yet?
		if (*player.mo).z != sector.floorheight {
			return;
		}

		// Has hitten ground.
		match sector.special {
			5 => {
				// HELLSLIME DAMAGE
				if player.powers[powertype_t::pw_ironfeet as usize] == 0 && leveltime & 0x1f == 0 {
					P_DamageMobj(&mut *player.mo, null_mut(), null_mut(), 10);
				}
			}

			7 => {
				// NUKAGE DAMAGE
				if player.powers[powertype_t::pw_ironfeet as usize] == 0 && leveltime & 0x1f == 0 {
					P_DamageMobj(&mut *player.mo, null_mut(), null_mut(), 5);
				}
			}

			// SUPER HELLSLIME DAMAGE
			// STROBE HURT
			16 | 4 => {
				if (player.powers[powertype_t::pw_ironfeet as usize] == 0 || P_Random() < 5)
					&& leveltime & 0x1f == 0
				{
					P_DamageMobj(&mut *player.mo, null_mut(), null_mut(), 20);
				}
			}

			9 => {
				// SECRET SECTOR
				player.secretcount += 1;
				sector.special = 0;
			}

			11 => {
				// EXIT SUPER DAMAGE! (for E1M8 finale)
				player.cheats &= !CF_GODMODE;

				if leveltime & 0x1f == 0 {
					P_DamageMobj(&mut *player.mo, null_mut(), null_mut(), 20);
				}

				if player.health <= 10 {
					G_ExitLevel();
				}
			}

			_ => {
				I_Error(
					c"P_PlayerInSpecialSector: unknown special %i".as_ptr(),
					sector.special as c_int,
				);
			}
		};
	}
}

// P_UpdateSpecials
// Animate planes, scroll walls, etc.
static mut levelTimer: bool = false;
static mut levelTimeCount: usize = 0;

unsafe extern "C" {}

#[allow(static_mut_refs)]
pub(crate) fn P_UpdateSpecials() {
	unsafe {
		//	LEVEL TIMER
		if levelTimer == true {
			levelTimeCount -= 1;
			if levelTimeCount == 0 {
				G_ExitLevel();
			}
		}

		//	ANIMATE FLATS AND TEXTURES GLOBALLY
		let mut anim_p = anims.as_mut_ptr();
		while !std::ptr::eq(anim_p, lastanim) {
			let anim = &mut *anim_p;
			anim_p = anim_p.wrapping_add(1);
			for i in anim.basepic..anim.basepic + anim.numpics {
				let pic = anim.basepic + ((leveltime / anim.speed + i) % anim.numpics);
				if anim.istexture {
					*texturetranslation.wrapping_add(i) = pic;
				} else {
					*flattranslation.wrapping_add(i) = pic;
				}
			}
		}

		//	ANIMATE LINE SPECIALS
		for i in 0..numlinespecials {
			let line = &mut *linespeciallist[i];
			match line.special {
				48 => {
					// EFFECT FIRSTCOL SCROLL +
					(*sides.wrapping_add(line.sidenum[0] as usize)).textureoffset += FRACUNIT;
				}
				_ => (),
			}
		}

		//	DO BUTTONS
		for i in 0..MAXBUTTONS {
			if buttonlist[i].btimer != 0 {
				buttonlist[i].btimer -= 1;
				if buttonlist[i].btimer == 0 {
					let side = &mut *sides.wrapping_add((*buttonlist[i].line).sidenum[0] as usize);
					match buttonlist[i].where_ {
						bwhere_e::top => side.toptexture = buttonlist[i].btexture as i16,
						bwhere_e::middle => side.midtexture = buttonlist[i].btexture as i16,
						bwhere_e::bottom => side.bottomtexture = buttonlist[i].btexture as i16,
					}
					S_StartSound(buttonlist[i].soundorg.cast(), sfxenum_t::sfx_swtchn);
					buttonlist[i] = button_t::new();
				}
			}
		}
	}
}

// Special Stuff that can not be categorized
pub(crate) fn EV_DoDonut(line: &mut line_t) -> bool {
	unsafe {
		let mut secnum = -1;
		let mut rtn = false;
		while let new_secnum @ 0.. = P_FindSectorFromLineTag(line, secnum) {
			secnum = new_secnum;
			let s1 = &mut *sectors.wrapping_add(secnum as usize);

			// ALREADY MOVING?  IF SO, KEEP GOING...
			if !s1.specialdata.is_null() {
				continue;
			}

			rtn = true;
			let s2 = &mut *getNextSector(*s1.lines, s1);
			for i in 0..s2.linecount {
				if (**s2.lines.wrapping_add(i)).flags & ML_TWOSIDED == 0
					|| std::ptr::eq((**s2.lines.wrapping_add(i)).backsector, s1)
				{
					continue;
				}
				let s3 = &*(**s2.lines.wrapping_add(i)).backsector;

				//	Spawn rising slime
				let floor = &mut *(Z_Malloc(size_of::<floormove_t>(), PU_LEVSPEC, null_mut())
					as *mut floormove_t);
				P_AddThinker(&raw mut floor.thinker);
				s2.specialdata = (floor as *mut floormove_t).cast();
				floor.thinker.function = think_t::T_MoveFloor;
				floor.ty = floor_e::donutRaise;
				floor.crush = 0;
				floor.direction = 1;
				floor.sector = s2;
				floor.speed = FLOORSPEED / 2;
				floor.texture = s3.floorpic;
				floor.newspecial = 0;
				floor.floordestheight = s3.floorheight;

				//	Spawn lowering donut-hole
				let floor = &mut *(Z_Malloc(size_of::<floormove_t>(), PU_LEVSPEC, null_mut())
					as *mut floormove_t);
				P_AddThinker(&raw mut floor.thinker);
				s1.specialdata = (floor as *mut floormove_t).cast();
				floor.thinker.function = think_t::T_MoveFloor;
				floor.ty = floor_e::lowerFloor;
				floor.crush = 0;
				floor.direction = -1;
				floor.sector = s1;
				floor.speed = FLOORSPEED / 2;
				floor.floordestheight = s3.floorheight;
				break;
			}
		}
		rtn
	}
}

// SPECIAL SPAWNING

// P_SpawnSpecials
// After the map has been loaded, scan for specials
//  that spawn thinkers
static mut numlinespecials: usize = 0;
static mut linespeciallist: [*mut line_t; MAXLINEANIMS] = [null_mut(); MAXLINEANIMS];

// Parses command line parameters.
pub(crate) fn P_SpawnSpecials() {
	unsafe {
		// See if -TIMER needs to be used.
		levelTimer = false;

		let i = M_CheckParm(c"-avg".as_ptr());
		if i != 0 && deathmatch != 0 {
			levelTimer = true;
			levelTimeCount = 20 * 60 * 35;
		}

		let i = M_CheckParm(c"-timer".as_ptr());
		if i != 0 && deathmatch != 0 {
			let time = libc::atoi(*myargv.wrapping_add(i + 1)) * 60 * 35;
			levelTimer = true;
			levelTimeCount = time as usize;
		}

		//	Init special SECTORs.
		for i in 0..numsectors {
			let sector = &mut *sectors.wrapping_add(i);
			if sector.special == 0 {
				continue;
			}

			match sector.special {
				1 => P_SpawnLightFlash(sector),               // FLICKERING LIGHTS
				2 => P_SpawnStrobeFlash(sector, FASTDARK, 0), // STROBE FAST
				3 => P_SpawnStrobeFlash(sector, SLOWDARK, 0), // STROBE SLOW
				4 => {
					// STROBE FAST/DEATH SLIME
					P_SpawnStrobeFlash(sector, FASTDARK, 0);
					sector.special = 4;
				}
				8 => P_SpawnGlowingLight(sector),              // GLOWING LIGHT
				9 => totalsecret += 1,                         // SECRET SECTOR
				10 => P_SpawnDoorCloseIn30(sector),            // DOOR CLOSE IN 30 SECONDS
				12 => P_SpawnStrobeFlash(sector, SLOWDARK, 1), // SYNC STROBE SLOW
				13 => P_SpawnStrobeFlash(sector, FASTDARK, 1), // SYNC STROBE FAST
				14 => P_SpawnDoorRaiseIn5Mins(sector, i),      // DOOR RAISE IN 5 MINUTES
				17 => P_SpawnFireFlicker(sector),
				_ => (),
			}
		}

		//	Init line EFFECTs
		numlinespecials = 0;
		for i in 0..numlines {
			if (*lines.wrapping_add(i)).special == 48 {
				// EFFECT FIRSTCOL SCROLL+
				linespeciallist[numlinespecials] = lines.wrapping_add(i);
				numlinespecials += 1;
			}
		}

		//	Init other misc stuff
		for i in 0..MAXCEILINGS {
			activeceilings[i] = null_mut();
		}

		for i in 0..MAXPLAYERS {
			activeplats[i] = null_mut();
		}

		for i in 0..MAXBUTTONS {
			buttonlist[i] = button_t::new();
		}
	}
}
