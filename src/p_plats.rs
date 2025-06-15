#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::{ffi::c_void, ptr::null_mut};

use crate::{
	i_system::I_Error,
	m_fixed::{FRACUNIT, fixed_t},
	m_random::P_Random,
	p_setup::{sectors, sides},
	p_spec::{MAXPLATS, PLATSPEED, PLATWAIT, plat_e, plat_t, plattype_e, result_e},
	p_tick::{P_AddThinker, P_RemoveThinker, leveltime},
	r_defs::{line_t, sector_t},
	s_sound::S_StartSound,
	sounds::sfxenum_t,
	z_zone::{PU_LEVSPEC, Z_Malloc},
};

type boolean = i32;

#[unsafe(no_mangle)]
pub static mut activeplats: [*mut plat_t; MAXPLATS] = [null_mut(); MAXPLATS];

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

pub(crate) unsafe extern "C" fn T_PlatRaise_action(plat: *mut c_void) {
	unsafe {
		T_PlatRaise(&mut *plat.cast());
	}
}

// Move a plat up and down
fn T_PlatRaise(plat: &mut plat_t) {
	unsafe {
		match plat.status {
			plat_e::up => {
				let res = T_MovePlane(plat.sector, plat.speed, plat.high, plat.crush, 0, 1);
				if (plat.ty == plattype_e::raiseAndChange
					|| plat.ty == plattype_e::raiseToNearestAndChange)
					&& leveltime & 7 == 0
				{
					S_StartSound((&raw mut (*plat.sector).soundorg).cast(), sfxenum_t::sfx_stnmov);
				}
				if res == result_e::crushed && plat.crush == 0 {
					plat.count = plat.wait;
					plat.status = plat_e::down;
					S_StartSound((&raw mut (*plat.sector).soundorg).cast(), sfxenum_t::sfx_pstart);
				} else if res == result_e::pastdest {
					plat.count = plat.wait;
					plat.status = plat_e::waiting;
					S_StartSound((&raw mut (*plat.sector).soundorg).cast(), sfxenum_t::sfx_pstop);

					match plat.ty {
						plattype_e::blazeDWUS | plattype_e::downWaitUpStay => {
							P_RemoveActivePlat(plat)
						}
						plattype_e::raiseAndChange | plattype_e::raiseToNearestAndChange => {
							P_RemoveActivePlat(plat)
						}
						plattype_e::perpetualRaise => (),
					}
				}
			}

			plat_e::down => {
				if let result_e::pastdest = T_MovePlane(plat.sector, plat.speed, plat.low, 0, 0, -1)
				{
					plat.count = plat.wait;
					plat.status = plat_e::waiting;
					S_StartSound((&raw mut (*plat.sector).soundorg).cast(), sfxenum_t::sfx_pstop);
				}
			}

			plat_e::waiting => {
				plat.count -= 1;
				if plat.count == 0 {
					if (*plat.sector).floorheight == plat.low {
						plat.status = plat_e::up;
					} else {
						plat.status = plat_e::down;
					}
					S_StartSound((&raw mut (*plat.sector).soundorg).cast(), sfxenum_t::sfx_pstart);
				}
			}

			plat_e::in_stasis => (),
		}
	}
}

unsafe extern "C" {
	fn P_FindSectorFromLineTag(line: *mut line_t, start: i32) -> i32;
	fn P_FindLowestFloorSurrounding(sec: *mut sector_t) -> fixed_t;
	fn P_FindHighestFloorSurrounding(sec: *mut sector_t) -> fixed_t;
	fn P_FindNextHighestFloor(sec: *mut sector_t, currentheight: fixed_t) -> fixed_t;
}

// Do Platforms
//  "amount" is only used for SOME platforms.
#[unsafe(no_mangle)]
pub extern "C" fn EV_DoPlat(line: &mut line_t, ty: plattype_e, amount: i32) -> boolean {
	unsafe {
		let mut secnum = -1;
		let mut rtn = 0;

		//	Activate all <type> plats that are in_stasis
		if let plattype_e::perpetualRaise = ty {
			P_ActivateInStasis(line.tag.into())
		}

		while let new_secnum @ 0.. = P_FindSectorFromLineTag(line, secnum) {
			secnum = new_secnum;
			let sec = &mut *sectors.wrapping_add(secnum as usize);

			if !sec.specialdata.is_null() {
				continue;
			}

			// Find lowest & highest floors around sector
			rtn = 1;
			let plat_p = Z_Malloc(size_of::<plat_t>(), PU_LEVSPEC, null_mut()) as *mut plat_t;
			let plat = &mut *plat_p;
			P_AddThinker(&raw mut plat.thinker);

			plat.ty = ty;
			plat.sector = sec;
			(*plat.sector).specialdata = plat_p.cast();
			plat.thinker.function.acp1 = Some(T_PlatRaise_action);
			plat.crush = 0;
			plat.tag = line.tag.into();

			match ty {
				plattype_e::raiseToNearestAndChange => {
					plat.speed = PLATSPEED / 2;
					sec.floorpic =
						(*(*sides.wrapping_add(line.sidenum[0] as usize)).sector).floorpic;
					plat.high = P_FindNextHighestFloor(sec, sec.floorheight);
					plat.wait = 0;
					plat.status = plat_e::up;
					// NO MORE DAMAGE, IF APPLICABLE
					sec.special = 0;

					S_StartSound((&raw mut sec.soundorg).cast(), sfxenum_t::sfx_stnmov);
				}

				plattype_e::raiseAndChange => {
					plat.speed = PLATSPEED / 2;
					sec.floorpic =
						(*(*sides.wrapping_add(line.sidenum[0] as usize)).sector).floorpic;
					plat.high = sec.floorheight + amount * FRACUNIT;
					plat.wait = 0;
					plat.status = plat_e::up;

					S_StartSound((&raw mut sec.soundorg).cast(), sfxenum_t::sfx_stnmov);
				}

				plattype_e::downWaitUpStay => {
					plat.speed = PLATSPEED * 4;
					plat.low = P_FindLowestFloorSurrounding(sec);

					if plat.low > sec.floorheight {
						plat.low = sec.floorheight;
					}

					plat.high = sec.floorheight;
					plat.wait = 35 * PLATWAIT;
					plat.status = plat_e::down;
					S_StartSound((&raw mut sec.soundorg).cast(), sfxenum_t::sfx_pstart);
				}

				plattype_e::blazeDWUS => {
					plat.speed = PLATSPEED * 8;
					plat.low = P_FindLowestFloorSurrounding(sec);

					if plat.low > sec.floorheight {
						plat.low = sec.floorheight;
					}

					plat.high = sec.floorheight;
					plat.wait = 35 * PLATWAIT;
					plat.status = plat_e::down;
					S_StartSound((&raw mut sec.soundorg).cast(), sfxenum_t::sfx_pstart);
				}

				plattype_e::perpetualRaise => {
					plat.speed = PLATSPEED;
					plat.low = P_FindLowestFloorSurrounding(sec);

					if plat.low > sec.floorheight {
						plat.low = sec.floorheight;
					}

					plat.high = P_FindHighestFloorSurrounding(sec);

					if plat.high < sec.floorheight {
						plat.high = sec.floorheight;
					}

					plat.wait = 35 * PLATWAIT;
					plat.status = if P_Random() & 1 == 0 { plat_e::up } else { plat_e::down };

					S_StartSound((&raw mut sec.soundorg).cast(), sfxenum_t::sfx_pstart);
				}
			}
			P_AddActivePlat(plat);
		}
		rtn
	}
}

fn P_ActivateInStasis(tag: i32) {
	unsafe {
		#[allow(clippy::needless_range_loop)]
		for i in 0..MAXPLATS {
			if !activeplats[i].is_null()
				&& (*activeplats[i]).tag == tag
				&& (*activeplats[i]).status == plat_e::in_stasis
			{
				(*activeplats[i]).status = (*activeplats[i]).oldstatus;
				(*activeplats[i]).thinker.function.acp1 = Some(T_PlatRaise_action);
			}
		}
	}
}

#[unsafe(no_mangle)]
pub extern "C" fn EV_StopPlat(line: &mut line_t) {
	unsafe {
		#[allow(clippy::needless_range_loop)]
		for j in 0..MAXPLATS {
			if !activeplats[j].is_null()
				&& (*activeplats[j]).status != plat_e::in_stasis
				&& (*activeplats[j]).tag == line.tag.into()
			{
				(*activeplats[j]).oldstatus = (*activeplats[j]).status;
				(*activeplats[j]).status = plat_e::in_stasis;
				(*activeplats[j]).thinker.function.acv = None;
			}
		}
	}
}

pub(crate) fn P_AddActivePlat(plat: *mut plat_t) {
	unsafe {
		#[allow(clippy::needless_range_loop)]
		for i in 0..MAXPLATS {
			if activeplats[i].is_null() {
				activeplats[i] = plat;
				return;
			}
		}
		I_Error(c"P_AddActivePlat: no more plats!".as_ptr());
	}
}

fn P_RemoveActivePlat(plat: *mut plat_t) {
	unsafe {
		#[allow(clippy::needless_range_loop)]
		for i in 0..MAXPLATS {
			if plat == activeplats[i] {
				(*(*activeplats[i]).sector).specialdata = null_mut();
				P_RemoveThinker(&mut (*activeplats[i]).thinker);
				activeplats[i] = null_mut();

				return;
			}
		}
		I_Error(c"P_RemoveActivePlat: can't find plat!".as_ptr());
	}
}
