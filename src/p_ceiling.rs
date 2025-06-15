#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::{ffi::c_void, ptr::null_mut};

use crate::{
	m_fixed::{FRACUNIT, fixed_t},
	p_setup::sectors,
	p_spec::{CEILSPEED, MAXCEILINGS, ceiling_e, ceiling_t, result_e},
	p_tick::{P_AddThinker, P_RemoveThinker, leveltime},
	r_defs::{line_t, sector_t},
	s_sound::S_StartSound,
	sounds::sfxenum_t,
	z_zone::{PU_LEVSPEC, Z_Malloc},
};

type boolean = i32;

// CEILINGS
#[unsafe(no_mangle)]
pub static mut activeceilings: [*mut ceiling_t; MAXCEILINGS] = [null_mut(); MAXCEILINGS];

unsafe extern "C" {
	fn T_MovePlane(
		sector: *mut crate::r_defs::sector_t,
		speed: i32,
		high: i32,
		crush: boolean,
		arg_1: i32,
		arg_2: i32,
	) -> result_e;
}

pub(crate) unsafe extern "C" fn T_MoveCeiling_action(ceiling: *mut c_void) {
	unsafe {
		T_MoveCeiling(&mut *ceiling.cast());
	}
}

// T_MoveCeiling
fn T_MoveCeiling(ceiling: &mut ceiling_t) {
	unsafe {
		match ceiling.direction {
			0 => (), // IN STASIS
			1 => {
				// UP
				let res = T_MovePlane(
					ceiling.sector,
					ceiling.speed,
					ceiling.topheight,
					0,
					1,
					ceiling.direction,
				);

				if leveltime & 7 == 0 {
					match ceiling.ty {
						ceiling_e::silentCrushAndRaise => (),
						_ => S_StartSound(
							(&raw mut (*ceiling.sector).soundorg).cast(),
							sfxenum_t::sfx_stnmov,
						),
					}
				}

				if res == result_e::pastdest {
					match ceiling.ty {
						ceiling_e::raiseToHighest => P_RemoveActiveCeiling(ceiling),

						ceiling_e::silentCrushAndRaise => S_StartSound(
							(&raw mut (*ceiling.sector).soundorg).cast(),
							sfxenum_t::sfx_pstop,
						),
						ceiling_e::fastCrushAndRaise | ceiling_e::crushAndRaise => {
							ceiling.direction = -1
						}

						_ => (),
					}
				}
			}

			-1 => {
				// DOWN
				let res = T_MovePlane(
					ceiling.sector,
					ceiling.speed,
					ceiling.bottomheight,
					ceiling.crush,
					1,
					ceiling.direction,
				);

				if leveltime & 7 == 0 {
					match ceiling.ty {
						ceiling_e::silentCrushAndRaise => (),
						_ => S_StartSound(
							(&raw mut (*ceiling.sector).soundorg).cast(),
							sfxenum_t::sfx_stnmov,
						),
					}
				}

				if res == result_e::pastdest {
					match ceiling.ty {
						ceiling_e::silentCrushAndRaise => S_StartSound(
							(&raw mut (*ceiling.sector).soundorg).cast(),
							sfxenum_t::sfx_pstop,
						),
						ceiling_e::crushAndRaise => ceiling.speed = CEILSPEED,
						ceiling_e::fastCrushAndRaise => ceiling.direction = 1,
						ceiling_e::lowerAndCrush | ceiling_e::lowerToFloor => {
							P_RemoveActiveCeiling(ceiling)
						}
						ceiling_e::raiseToHighest => (),
					}
				} else {
					// ( res != pastdest )
					if res == result_e::crushed {
						match ceiling.ty {
							ceiling_e::silentCrushAndRaise
							| ceiling_e::crushAndRaise
							| ceiling_e::lowerAndCrush => ceiling.speed = CEILSPEED / 8,
							_ => (),
						}
					}
				}
			}
			_ => unreachable!(),
		}
	}
}

unsafe extern "C" {
	fn P_FindSectorFromLineTag(line: *mut line_t, start: i32) -> i32;
	fn P_FindHighestCeilingSurrounding(sec: *mut sector_t) -> fixed_t;
}

// EV_DoCeiling
// Move a ceiling up/down and all around!
#[unsafe(no_mangle)]
pub extern "C" fn EV_DoCeiling(line: &mut line_t, ty: ceiling_e) -> boolean {
	unsafe {
		//	Reactivate in-stasis ceilings...for certain types.
		match ty {
			ceiling_e::fastCrushAndRaise
			| ceiling_e::silentCrushAndRaise
			| ceiling_e::crushAndRaise => P_ActivateInStasisCeiling(line),
			_ => (),
		}

		let mut secnum = -1;
		let mut rtn = 0;

		while let new_secnum @ 0.. = P_FindSectorFromLineTag(line, secnum) {
			secnum = new_secnum;
			let sec = &mut *sectors.wrapping_add(secnum as usize);
			if !sec.specialdata.is_null() {
				continue;
			}

			// new door thinker
			rtn = 1;
			let ceiling_p =
				Z_Malloc(size_of::<ceiling_t>(), PU_LEVSPEC, null_mut()) as *mut ceiling_t;
			let ceiling = &mut *ceiling_p;
			P_AddThinker(&raw mut ceiling.thinker);
			sec.specialdata = ceiling_p.cast();
			ceiling.thinker.function.acp1 = Some(T_MoveCeiling_action);
			ceiling.sector = sec;
			ceiling.crush = 0;

			match ty {
				ceiling_e::fastCrushAndRaise => {
					ceiling.crush = 1;
					ceiling.topheight = sec.ceilingheight;
					ceiling.bottomheight = sec.floorheight + (8 * FRACUNIT);
					ceiling.direction = -1;
					ceiling.speed = CEILSPEED * 2;
				}

				ceiling_e::silentCrushAndRaise | ceiling_e::crushAndRaise => {
					ceiling.crush = 1;
					ceiling.topheight = sec.ceilingheight;
				}

				ceiling_e::lowerAndCrush | ceiling_e::lowerToFloor => {
					ceiling.bottomheight = sec.floorheight;
					if ty != ceiling_e::lowerToFloor {
						ceiling.bottomheight += 8 * FRACUNIT;
					}
					ceiling.direction = -1;
					ceiling.speed = CEILSPEED;
				}

				ceiling_e::raiseToHighest => {
					ceiling.topheight = P_FindHighestCeilingSurrounding(sec);
					ceiling.direction = 1;
					ceiling.speed = CEILSPEED;
				}
			}

			ceiling.tag = sec.tag.into();
			ceiling.ty = ty;
			P_AddActiveCeiling(ceiling);
		}
		rtn
	}
}

// Add an active ceiling
pub(crate) fn P_AddActiveCeiling(ceiling: *mut ceiling_t) {
	unsafe {
		#[allow(clippy::needless_range_loop)]
		for i in 0..MAXCEILINGS {
			if activeceilings[i].is_null() {
				activeceilings[i] = ceiling;
				return;
			}
		}
	}
}

// Remove a ceiling's thinker
fn P_RemoveActiveCeiling(ceiling: *mut ceiling_t) {
	unsafe {
		#[allow(clippy::needless_range_loop)]
		for i in 0..MAXCEILINGS {
			if ceiling == activeceilings[i] {
				(*(*activeceilings[i]).sector).specialdata = null_mut();
				P_RemoveThinker(&mut (*activeceilings[i]).thinker);
				activeceilings[i] = null_mut();
				return;
			}
		}
	}
}

// Restart a ceiling that's in-stasis
fn P_ActivateInStasisCeiling(line: &mut line_t) {
	unsafe {
		#[allow(clippy::needless_range_loop)]
		for i in 0..MAXCEILINGS {
			if !activeceilings[i].is_null()
				&& (*activeceilings[i]).tag == line.tag.into()
				&& (*activeceilings[i]).direction == 0
			{
				(*activeceilings[i]).direction = (*activeceilings[i]).olddirection;
				(*activeceilings[i]).thinker.function.acp1 = Some(T_MoveCeiling_action);
			}
		}
	}
}

// EV_CeilingCrushStop
// Stop a ceiling from crushing!
#[unsafe(no_mangle)]
pub extern "C" fn EV_CeilingCrushStop(line: &mut line_t) -> boolean {
	unsafe {
		let mut rtn = 0;
		#[allow(clippy::needless_range_loop)]
		for i in 0..MAXCEILINGS {
			if !activeceilings[i].is_null()
				&& (*activeceilings[i]).tag == line.tag.into()
				&& (*activeceilings[i]).direction != 0
			{
				(*activeceilings[i]).olddirection = (*activeceilings[i]).direction;
				(*activeceilings[i]).thinker.function.acv = None;
				(*activeceilings[i]).direction = 0; // in-stasis
				rtn = 1;
			}
		}

		rtn
	}
}
