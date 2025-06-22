#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::ptr::null_mut;

use crate::{
	d_englsh::{PD_BLUEK, PD_BLUEO, PD_REDK, PD_REDO, PD_YELLOWK, PD_YELLOWO},
	d_think::think_t,
	doomdef::card_t,
	m_fixed::FRACUNIT,
	p_floor::T_MovePlane,
	p_mobj::mobj_t,
	p_setup::{sectors, sides},
	p_spec::{
		P_FindLowestCeilingSurrounding, P_FindSectorFromLineTag, VDOORSPEED, VDOORWAIT, result_e,
		vldoor_e, vldoor_t,
	},
	p_tick::{P_AddThinker, P_RemoveThinker},
	r_defs::{line_t, sector_t},
	s_sound::S_StartSound,
	sounds::sfxenum_t,
	z_zone::{PU_LEVSPEC, Z_Malloc},
};

// VERTICAL DOORS

// T_VerticalDoor
pub(crate) fn T_VerticalDoor(door: &mut vldoor_t) {
	unsafe {
		match door.direction {
			0 => {
				// WAITING
				door.topcountdown -= 1;
				if door.topcountdown == 0 {
					match door.ty {
						vldoor_e::blazeRaise => {
							door.direction = -1; // time to go back down
							S_StartSound(
								(&raw mut (*door.sector).soundorg).cast(),
								sfxenum_t::sfx_bdcls,
							);
						}

						vldoor_e::normal => {
							door.direction = -1; // time to go back down
							S_StartSound(
								(&raw mut (*door.sector).soundorg).cast(),
								sfxenum_t::sfx_dorcls,
							);
						}

						vldoor_e::close30ThenOpen => {
							door.direction = 1;
							S_StartSound(
								(&raw mut (*door.sector).soundorg).cast(),
								sfxenum_t::sfx_doropn,
							);
						}

						_ => (),
					}
				}
			}

			2 => {
				//  INITIAL WAIT
				door.topcountdown -= 1;
				if door.topcountdown == 0 && door.ty == vldoor_e::raiseIn5Mins {
					door.direction = 1;
					door.ty = vldoor_e::normal;
					S_StartSound((&raw mut (*door.sector).soundorg).cast(), sfxenum_t::sfx_doropn);
				}
			}

			-1 => {
				// DOWN
				let res = T_MovePlane(
					&mut *door.sector,
					door.speed,
					(*door.sector).floorheight,
					false,
					1,
					door.direction,
				);
				if res == result_e::pastdest {
					match door.ty {
						vldoor_e::blazeRaise | vldoor_e::blazeClose => {
							(*door.sector).specialdata = null_mut();
							P_RemoveThinker(&mut door.thinker); // unlink and free
							S_StartSound(
								(&raw mut (*door.sector).soundorg).cast(),
								sfxenum_t::sfx_bdcls,
							);
						}

						vldoor_e::normal | vldoor_e::close => {
							(*door.sector).specialdata = null_mut();
							P_RemoveThinker(&mut door.thinker); // unlink and free
						}

						vldoor_e::close30ThenOpen => {
							door.direction = 0;
							door.topcountdown = 35 * 30;
						}

						_ => (),
					}
				} else if res == result_e::crushed {
					match door.ty {
						vldoor_e::blazeClose | vldoor_e::close => (), // DO NOT GO BACK UP!

						_ => {
							door.direction = 1;
							S_StartSound(
								(&raw mut (*door.sector).soundorg).cast(),
								sfxenum_t::sfx_doropn,
							);
						}
					}
				}
			}

			1 => {
				// UP
				let res = T_MovePlane(
					&mut *door.sector,
					door.speed,
					door.topheight,
					false,
					1,
					door.direction,
				);

				if res == result_e::pastdest {
					match door.ty {
						vldoor_e::blazeRaise | vldoor_e::normal => {
							door.direction = 0; // wait at top
							door.topcountdown = door.topwait;
						}

						vldoor_e::close30ThenOpen | vldoor_e::blazeOpen | vldoor_e::open => {
							(*door.sector).specialdata = null_mut();
							P_RemoveThinker(&mut door.thinker); // unlink and free
						}

						_ => (),
					}
				}
			}
			_ => unreachable!(),
		}
	}
}

// EV_DoLockedDoor
// Move a locked door up/down
pub(crate) fn EV_DoLockedDoor(line: &mut line_t, ty: vldoor_e, thing: &mut mobj_t) -> bool {
	unsafe {
		let p = thing.player;

		if p.is_null() {
			return false;
		}

		let p = &mut *p;

		match line.special {
			99 | 133 => {
				// Blue Lock
				if p.cards[card_t::it_bluecard as usize] == 0
					&& p.cards[card_t::it_blueskull as usize] == 0
				{
					p.message = PD_BLUEO;
					S_StartSound(null_mut(), sfxenum_t::sfx_oof);
					return false;
				}
			}

			134 | 135 => {
				// Red Lock
				if p.cards[card_t::it_redcard as usize] == 0
					&& p.cards[card_t::it_redskull as usize] == 0
				{
					p.message = PD_REDO;
					S_StartSound(null_mut(), sfxenum_t::sfx_oof);
					return false;
				}
			}

			136 | 137 => {
				// Yellow Lock
				if p.cards[card_t::it_yellowcard as usize] == 0
					&& p.cards[card_t::it_yellowskull as usize] == 0
				{
					p.message = PD_YELLOWO;
					S_StartSound(null_mut(), sfxenum_t::sfx_oof);
					return false;
				}
			}

			_ => (),
		}

		EV_DoDoor(line, ty)
	}
}

pub(crate) fn EV_DoDoor(line: &mut line_t, ty: vldoor_e) -> bool {
	unsafe {
		let mut secnum = -1;
		let mut rtn = false;

		while let new_secnum @ 0.. = P_FindSectorFromLineTag(line, secnum) {
			secnum = new_secnum;
			let sec = &mut *sectors.wrapping_add(secnum as usize);
			if !sec.specialdata.is_null() {
				continue;
			}

			// new door thinker
			rtn = true;
			let door =
				&mut *(Z_Malloc(size_of::<vldoor_t>(), PU_LEVSPEC, null_mut()) as *mut vldoor_t);
			P_AddThinker(&raw mut door.thinker);
			sec.specialdata = (door as *mut vldoor_t).cast();

			door.thinker.function = think_t::T_VerticalDoor;
			door.sector = sec;
			door.ty = ty;
			door.topwait = VDOORWAIT;
			door.speed = VDOORSPEED;

			match ty {
				vldoor_e::blazeClose => {
					door.topheight = P_FindLowestCeilingSurrounding(sec);
					door.topheight -= 4 * FRACUNIT;
					door.direction = -1;
					door.speed = VDOORSPEED * 4;
					S_StartSound((&raw mut (*door.sector).soundorg).cast(), sfxenum_t::sfx_bdcls);
				}

				vldoor_e::close => {
					door.topheight = P_FindLowestCeilingSurrounding(sec);
					door.topheight -= 4 * FRACUNIT;
					door.direction = -1;
					S_StartSound((&raw mut (*door.sector).soundorg).cast(), sfxenum_t::sfx_dorcls);
				}

				vldoor_e::close30ThenOpen => {
					door.topheight = sec.ceilingheight;
					door.direction = -1;
					S_StartSound((&raw mut (*door.sector).soundorg).cast(), sfxenum_t::sfx_dorcls);
				}

				vldoor_e::blazeRaise | vldoor_e::blazeOpen => {
					door.direction = 1;
					door.topheight = P_FindLowestCeilingSurrounding(sec);
					door.topheight -= 4 * FRACUNIT;
					door.speed = VDOORSPEED * 4;
					if door.topheight != sec.ceilingheight {
						S_StartSound(
							(&raw mut (*door.sector).soundorg).cast(),
							sfxenum_t::sfx_bdopn,
						);
					}
				}

				vldoor_e::normal | vldoor_e::open => {
					door.direction = 1;
					door.topheight = P_FindLowestCeilingSurrounding(sec);
					door.topheight -= 4 * FRACUNIT;
					if door.topheight != sec.ceilingheight {
						S_StartSound(
							(&raw mut (*door.sector).soundorg).cast(),
							sfxenum_t::sfx_doropn,
						);
					}
				}

				_ => (),
			}
		}

		rtn
	}
}

// EV_VerticalDoor : open a door manually, no tag value
pub(crate) fn EV_VerticalDoor(line: &mut line_t, thing: &mut mobj_t) {
	unsafe {
		let side = 0; // only front sides can be used

		//	Check for locks
		let player = thing.player;

		match line.special {
			26 | 32 => {
				// Blue Lock
				let Some(player) = player.as_mut() else { return };

				if player.cards[card_t::it_bluecard as usize] == 0
					&& player.cards[card_t::it_blueskull as usize] == 0
				{
					player.message = PD_BLUEK;
					S_StartSound(null_mut(), sfxenum_t::sfx_oof);
					return;
				}
			}

			27 | 34 => {
				// Yellow Lock
				let Some(player) = player.as_mut() else { return };

				if player.cards[card_t::it_yellowcard as usize] == 0
					&& player.cards[card_t::it_yellowskull as usize] == 0
				{
					player.message = PD_YELLOWK;
					S_StartSound(null_mut(), sfxenum_t::sfx_oof);
					return;
				}
			}

			28 | 33 => {
				// Red Lock
				let Some(player) = player.as_mut() else { return };

				if player.cards[card_t::it_redcard as usize] == 0
					&& player.cards[card_t::it_redskull as usize] == 0
				{
					player.message = PD_REDK;
					S_StartSound(null_mut(), sfxenum_t::sfx_oof);
					return;
				}
			}

			_ => (),
		}

		// if the sector has an active thinker, use it
		let sec = (*sides.wrapping_add(line.sidenum[side ^ 1] as usize)).sector;

		if !(*sec).specialdata.is_null() {
			let door = &mut *((*sec).specialdata as *mut vldoor_t);
			match line.special {
				1 | 26 | 27 | 28 | 117 => {
					// ONLY FOR "RAISE" DOORS, NOT "OPEN"s
					if door.direction == -1 {
						door.direction = 1; // go back up
					} else {
						if thing.player.is_null() {
							return; // JDC: bad guys never close doors
						}

						door.direction = -1; // start going down immediately
					}
					return;
				}
				_ => (),
			}
		}

		// for proper sound
		match line.special {
			// BLAZING DOOR RAISE | BLAZING DOOR OPEN
			117 | 118 => S_StartSound((&raw mut (*sec).soundorg).cast(), sfxenum_t::sfx_bdopn),

			// NORMAL DOOR SOUND
			1 | 31 => S_StartSound((&raw mut (*sec).soundorg).cast(), sfxenum_t::sfx_doropn),

			// LOCKED DOOR SOUND
			_ => S_StartSound((&raw mut (*sec).soundorg).cast(), sfxenum_t::sfx_doropn),
		}

		// new door thinker
		let door = &mut *(Z_Malloc(size_of::<vldoor_t>(), PU_LEVSPEC, null_mut()) as *mut vldoor_t);
		P_AddThinker(&raw mut door.thinker);
		(*sec).specialdata = (door as *mut vldoor_t).cast();
		door.thinker.function = think_t::T_VerticalDoor;
		door.sector = sec;
		door.direction = 1;
		door.speed = VDOORSPEED;
		door.topwait = VDOORWAIT;

		match line.special {
			1 | 26 | 27 | 28 => door.ty = vldoor_e::normal,

			31..=34 => {
				door.ty = vldoor_e::open;
				line.special = 0;
			}

			117 => {
				// blazing door raise
				door.ty = vldoor_e::blazeRaise;
				door.speed = VDOORSPEED * 4;
			}

			118 => {
				// blazing door open
				door.ty = vldoor_e::blazeOpen;
				line.special = 0;
				door.speed = VDOORSPEED * 4;
			}

			_ => (),
		}

		// find the top and bottom of the movement range
		door.topheight = P_FindLowestCeilingSurrounding(&mut *sec);
		door.topheight -= 4 * FRACUNIT;
	}
}

// Spawn a door that closes after 30 seconds
pub(crate) fn P_SpawnDoorCloseIn30(sec: &mut sector_t) {
	unsafe {
		let door = &mut *(Z_Malloc(size_of::<vldoor_t>(), PU_LEVSPEC, null_mut()) as *mut vldoor_t);

		P_AddThinker(&raw mut door.thinker);

		sec.specialdata = (door as *mut vldoor_t).cast();
		sec.special = 0;

		door.thinker.function = think_t::T_VerticalDoor;
		door.sector = sec;
		door.direction = 0;
		door.ty = vldoor_e::normal;
		door.speed = VDOORSPEED;
		door.topcountdown = 30 * 35;
	}
}

// Spawn a door that opens after 5 minutes
pub(crate) fn P_SpawnDoorRaiseIn5Mins(sec: &mut sector_t, _secnum: usize) {
	unsafe {
		let door = &mut *(Z_Malloc(size_of::<vldoor_t>(), PU_LEVSPEC, null_mut()) as *mut vldoor_t);

		P_AddThinker(&raw mut door.thinker);

		sec.specialdata = (door as *mut vldoor_t).cast();
		sec.special = 0;

		door.thinker.function = think_t::T_VerticalDoor;
		door.sector = sec;
		door.direction = 2;
		door.ty = vldoor_e::raiseIn5Mins;
		door.speed = VDOORSPEED;
		door.topheight = P_FindLowestCeilingSurrounding(sec);
		door.topheight -= 4 * FRACUNIT;
		door.topwait = VDOORWAIT;
		door.topcountdown = 5 * 60 * 35;
	}
}
