#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

// DESCRIPTION:
//	Handle Sector base lighting effects.
//	Muzzle flash?

//
// FIRELIGHT FLICKER
//

use std::{ffi::c_void, ptr::null_mut};

use crate::{
	d_think::actionf_p1,
	m_random::P_Random,
	p_spec::{GLOWSPEED, SLOWDARK, STROBEBRIGHT, fireflicker_t, glow_t, lightflash_t, strobe_t},
	p_tick::P_AddThinker,
	r_defs::{line_t, sector_t},
	r_state::{numsectors, sectors},
	z_zone::{PU_LEVSPEC, Z_Malloc},
};

// T_FireFlicker
fn T_FireFlicker(flick: &mut fireflicker_t) {
	unsafe {
		flick.count -= 1;
		if (flick.count) != 0 {
			return;
		}

		let amount = (P_Random() & 3) * 16;

		if (*flick.sector).lightlevel as i32 - amount < flick.minlight {
			(*flick.sector).lightlevel = flick.minlight as i16;
		} else {
			(*flick.sector).lightlevel = (flick.maxlight - amount) as i16;
		};

		flick.count = 4;
	}
}

extern "C" fn T_FireFlicker_action(flick: *mut c_void) {
	unsafe { T_FireFlicker(&mut *(flick.cast())) };
}

unsafe extern "C" {
	fn P_FindMinSurroundingLight(sector: *mut sector_t, max: i32) -> i32;
}

// P_SpawnFireFlicker
#[unsafe(no_mangle)]
pub extern "C" fn P_SpawnFireFlicker(sector: &mut sector_t) {
	unsafe {
		// Note that we are resetting sector attributes.
		// Nothing special about it during gameplay.
		sector.special = 0;

		let flick =
			Z_Malloc(size_of::<fireflicker_t>(), PU_LEVSPEC, null_mut()) as *mut fireflicker_t;
		let flick = &mut *flick;

		P_AddThinker(&raw mut flick.thinker);

		flick.thinker.function.acp1 = Some(T_FireFlicker_action as actionf_p1);
		flick.sector = sector;
		flick.maxlight = sector.lightlevel as i32;
		flick.minlight = P_FindMinSurroundingLight(sector, sector.lightlevel as i32) + 16;
		flick.count = 4;
	}
}

// BROKEN LIGHT FLASHING

// T_LightFlash
// Do flashing lights.
fn T_LightFlash(flash: &mut lightflash_t) {
	unsafe {
		flash.count -= 1;
		if (flash.count) != 0 {
			return;
		}

		if (*flash.sector).lightlevel == flash.maxlight as i16 {
			(*flash.sector).lightlevel = flash.minlight as i16;
			flash.count = (P_Random() & flash.mintime) + 1;
		} else {
			(*flash.sector).lightlevel = flash.maxlight as i16;
			flash.count = (P_Random() & flash.maxtime) + 1;
		}
	}
}

pub(crate) unsafe extern "C" fn T_LightFlash_action(flash: *mut c_void) {
	unsafe { T_LightFlash(&mut *(flash.cast())) };
}

// P_SpawnLightFlash
// After the map has been loaded, scan each sector
// for specials that spawn thinkers
#[unsafe(no_mangle)]
pub extern "C" fn P_SpawnLightFlash(sector: &mut sector_t) {
	unsafe {
		// nothing special about it during gameplay
		sector.special = 0;

		let flash =
			Z_Malloc(size_of::<lightflash_t>(), PU_LEVSPEC, null_mut()) as *mut lightflash_t;
		let flash = &mut *flash;

		P_AddThinker(&raw mut flash.thinker);

		flash.thinker.function.acp1 = Some(T_LightFlash_action as actionf_p1);
		flash.sector = sector;
		flash.maxlight = sector.lightlevel as i32;

		flash.minlight = P_FindMinSurroundingLight(sector, sector.lightlevel as i32);
		flash.maxtime = 64;
		flash.mintime = 7;
		flash.count = (P_Random() & flash.maxtime) + 1;
	}
}

// STROBE LIGHT FLASHING

// T_StrobeFlash
fn T_StrobeFlash(flash: &mut strobe_t) {
	unsafe {
		flash.count -= 1;
		if (flash.count) != 0 {
			return;
		}

		if (*flash.sector).lightlevel == flash.minlight as i16 {
			(*flash.sector).lightlevel = flash.maxlight as i16;
			flash.count = flash.brighttime;
		} else {
			(*flash.sector).lightlevel = flash.minlight as i16;
			flash.count = flash.darktime;
		}
	}
}

pub(crate) extern "C" fn T_StrobeFlash_action(flash: *mut c_void) {
	unsafe { T_StrobeFlash(&mut *(flash.cast())) };
}

// P_SpawnStrobeFlash
// After the map has been loaded, scan each sector
// for specials that spawn thinkers
#[unsafe(no_mangle)]
pub extern "C" fn P_SpawnStrobeFlash(sector: &mut sector_t, fastOrSlow: i32, inSync: i32) {
	unsafe {
		let flash = Z_Malloc(size_of::<strobe_t>(), PU_LEVSPEC, null_mut()) as *mut strobe_t;
		let flash = &mut *flash;

		P_AddThinker(&raw mut flash.thinker);

		flash.sector = sector;
		flash.darktime = fastOrSlow;
		flash.brighttime = STROBEBRIGHT;
		flash.thinker.function.acp1 = Some(T_StrobeFlash_action as actionf_p1);
		flash.maxlight = sector.lightlevel as i32;
		flash.minlight = P_FindMinSurroundingLight(sector, sector.lightlevel as i32);

		if flash.minlight == flash.maxlight {
			flash.minlight = 0;
		}

		// nothing special about it during gameplay
		sector.special = 0;

		if inSync == 0 {
			flash.count = (P_Random() & 7) + 1;
		} else {
			flash.count = 1;
		}
	}
}

unsafe extern "C" {
	fn P_FindSectorFromLineTag(line: *mut line_t, start: i32) -> i32;
}

// Start strobing lights (usually from a trigger)
#[unsafe(no_mangle)]
pub extern "C" fn EV_StartLightStrobing(line: &mut line_t) {
	let secnum = -1;
	unsafe {
		while let secnum @ 0.. = P_FindSectorFromLineTag(line, secnum) {
			let sec = &mut *sectors.wrapping_add(secnum as usize);
			if !sec.specialdata.is_null() {
				continue;
			}

			P_SpawnStrobeFlash(sec, SLOWDARK, 0);
		}
	}
}

unsafe extern "C" {
	fn getNextSector(line: *mut line_t, sec: *mut sector_t) -> *mut sector_t;
}

// TURN LINE'S TAG LIGHTS OFF
#[unsafe(no_mangle)]
pub extern "C" fn EV_TurnTagLightsOff(line: &mut line_t) {
	unsafe {
		for j in 0..numsectors as usize {
			let sector = &mut *sectors.wrapping_add(j);
			if sector.tag == line.tag {
				let mut min = sector.lightlevel;
				for i in 0..sector.linecount as usize {
					let templine = *sector.lines.wrapping_add(i);
					let tsec = getNextSector(templine, sector);
					if tsec.is_null() {
						continue;
					}
					if (*tsec).lightlevel < min {
						min = (*tsec).lightlevel;
					}
				}
				sector.lightlevel = min;
			}
		}
	}
}

// TURN LINE'S TAG LIGHTS ON
#[unsafe(no_mangle)]
pub extern "C" fn EV_LightTurnOn(line: &mut line_t, mut bright: i32) {
	unsafe {
		for i in 0..numsectors as usize {
			let sector = &mut *sectors.wrapping_add(i);
			if sector.tag == line.tag {
				// bright = 0 means to search
				// for highest light level
				// surrounding sector
				if bright != 0 {
					for j in 0..sector.linecount as usize {
						let templine = *sector.lines.wrapping_add(j);
						let temp = getNextSector(templine, sector);

						if temp.is_null() {
							continue;
						}

						if (*temp).lightlevel as i32 > bright {
							bright = (*temp).lightlevel as i32;
						}
					}
					sector.lightlevel = bright as i16;
				}
			}
		}
	}
}

// Spawn glowing light
fn T_Glow(g: &mut glow_t) {
	match g.direction {
		-1 => {
			// DOWN
			let sector = unsafe { &mut *g.sector };
			sector.lightlevel -= GLOWSPEED;
			if sector.lightlevel <= g.minlight as i16 {
				sector.lightlevel += GLOWSPEED;
				g.direction = 1;
			}
		}

		1 => {
			// UP
			let sector = unsafe { &mut *g.sector };
			sector.lightlevel += GLOWSPEED;
			if sector.lightlevel >= g.maxlight as i16 {
				sector.lightlevel -= GLOWSPEED;
				g.direction = -1;
			}
		}
		_ => unreachable!(),
	}
}

pub(crate) extern "C" fn T_Glow_action(g: *mut c_void) {
	unsafe { T_Glow(&mut *(g.cast())) };
}

#[unsafe(no_mangle)]
pub extern "C" fn P_SpawnGlowingLight(sector: &mut sector_t) {
	unsafe {
		let g = Z_Malloc(size_of::<glow_t>(), PU_LEVSPEC, null_mut()) as *mut glow_t;
		let g = &mut *g;

		P_AddThinker(&raw mut g.thinker);

		g.sector = sector;
		g.minlight = P_FindMinSurroundingLight(sector, sector.lightlevel as i32);
		g.maxlight = sector.lightlevel as i32;
		g.thinker.function.acp1 = Some(T_Glow_action as actionf_p1);
		g.direction = -1;

		sector.special = 0;
	}
}
