#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

// FLOORS

use std::ptr::null_mut;

use crate::{
	d_think::think_t,
	doomdata::ML_TWOSIDED,
	m_fixed::{FRACUNIT, fixed_t},
	p_setup::sectors,
	p_spec::{
		FLOORSPEED, P_FindHighestFloorSurrounding, P_FindLowestCeilingSurrounding,
		P_FindLowestFloorSurrounding, P_FindNextHighestFloor, P_FindSectorFromLineTag, floor_e,
		floormove_t, getSector, getSide, result_e, stair_e, twoSided,
	},
	p_tick::{P_AddThinker, P_RemoveThinker, leveltime},
	r_data::textureheight,
	r_defs::{line_t, sector_t},
	s_sound::S_StartSound,
	sounds::sfxenum_t,
	z_zone::{PU_LEVSPEC, Z_Malloc},
};

type boolean = i32;

unsafe extern "C" {
	fn P_ChangeSector(sector: *mut sector_t, crunch: boolean) -> boolean;
}

// Move a plane (floor or ceiling) and check for crushing
pub(crate) fn T_MovePlane(
	sector: &mut sector_t,
	speed: fixed_t,
	dest: fixed_t,
	crush: bool,
	floorOrCeiling: i32,
	direction: i32,
) -> result_e {
	unsafe {
		match floorOrCeiling {
			0 => {
				// FLOOR
				match direction {
					-1 => {
						// DOWN
						if sector.floorheight - speed < dest {
							let lastpos = sector.floorheight;
							sector.floorheight = dest;
							let flag = P_ChangeSector(sector, crush as boolean);
							if flag == 1 {
								sector.floorheight = lastpos;
								P_ChangeSector(sector, crush as boolean);
								//return crushed;
							}
							return result_e::pastdest;
						} else {
							let lastpos = sector.floorheight;
							sector.floorheight -= speed;
							let flag = P_ChangeSector(sector, crush as boolean);
							if flag == 1 {
								sector.floorheight = lastpos;
								P_ChangeSector(sector, crush as boolean);
								return result_e::crushed;
							}
						}
					}

					1 => {
						// UP
						if sector.floorheight + speed > dest {
							let lastpos = sector.floorheight;
							sector.floorheight = dest;
							let flag = P_ChangeSector(sector, crush as boolean);
							if flag == 1 {
								sector.floorheight = lastpos;
								P_ChangeSector(sector, crush as boolean);
								//return crushed;
							}
							return result_e::pastdest;
						} else {
							// COULD GET CRUSHED
							let lastpos = sector.floorheight;
							sector.floorheight += speed;
							let flag = P_ChangeSector(sector, crush as boolean);
							if flag == 1 {
								if crush {
									return result_e::crushed;
								}
								sector.floorheight = lastpos;
								P_ChangeSector(sector, crush as boolean);
								return result_e::crushed;
							}
						}
					}

					_ => (),
				}
			}

			1 => {
				// CEILING
				match direction {
					-1 => {
						// DOWN
						if sector.ceilingheight - speed < dest {
							let lastpos = sector.ceilingheight;
							sector.ceilingheight = dest;
							let flag = P_ChangeSector(sector, crush as boolean);

							if flag == 1 {
								sector.ceilingheight = lastpos;
								P_ChangeSector(sector, crush as boolean);
								//return result_e::crushed;
							}
							return result_e::pastdest;
						} else {
							// COULD GET CRUSHED
							let lastpos = sector.ceilingheight;
							sector.ceilingheight -= speed;
							let flag = P_ChangeSector(sector, crush as boolean);

							if flag == 1 {
								if crush == true {
									return result_e::crushed;
								}
								sector.ceilingheight = lastpos;
								P_ChangeSector(sector, crush as boolean);
								return result_e::crushed;
							}
						}
					}

					1 => {
						// UP
						if sector.ceilingheight + speed > dest {
							let lastpos = sector.ceilingheight;
							sector.ceilingheight = dest;
							let flag = P_ChangeSector(sector, crush as boolean);
							if flag == 1 {
								sector.ceilingheight = lastpos;
								P_ChangeSector(sector, crush as boolean);
								//return result_e::crushed;
							}
							return result_e::pastdest;
						} else {
							// let lastpos = sector.ceilingheight;
							sector.ceilingheight += speed;
							P_ChangeSector(sector, crush as boolean);
						}
					}

					_ => unreachable!(),
				}
			}

			_ => unreachable!(),
		}
		return result_e::ok;
	}
}

// MOVE A FLOOR TO IT'S DESTINATION (UP OR DOWN)
pub(crate) fn T_MoveFloor(floor: &mut floormove_t) {
	unsafe {
		let res = T_MovePlane(
			&mut *floor.sector,
			floor.speed,
			floor.floordestheight,
			floor.crush != 0,
			0,
			floor.direction,
		);

		if leveltime & 7 == 0 {
			S_StartSound((&raw mut (*floor.sector).soundorg).cast(), sfxenum_t::sfx_stnmov);
		}

		if res == result_e::pastdest {
			(*floor.sector).specialdata = null_mut();

			if floor.direction == 1 {
				match floor.ty {
					floor_e::donutRaise => {
						(*floor.sector).special = floor.newspecial as i16;
						(*floor.sector).floorpic = floor.texture;
					}
					_ => (),
				}
			} else if floor.direction == -1 {
				match floor.ty {
					floor_e::lowerAndChange => {
						(*floor.sector).special = floor.newspecial as i16;
						(*floor.sector).floorpic = floor.texture;
					}
					_ => (),
				}
			}
			P_RemoveThinker(&mut floor.thinker);

			S_StartSound((&raw mut (*floor.sector).soundorg).cast(), sfxenum_t::sfx_pstop);
		}
	}
}

// HANDLE FLOOR TYPES
pub(crate) fn EV_DoFloor(line: &mut line_t, floortype: floor_e) -> bool {
	unsafe {
		let mut secnum = -1;
		let mut rtn = false;
		while let new_secnum @ 0.. = P_FindSectorFromLineTag(line, secnum) {
			secnum = new_secnum;
			let sec = &mut *sectors.wrapping_add(secnum as usize);

			// ALREADY MOVING?  IF SO, KEEP GOING...
			if !sec.specialdata.is_null() {
				continue;
			}

			// new floor thinker
			rtn = true;
			let floor = &mut *(Z_Malloc(size_of::<floormove_t>(), PU_LEVSPEC, null_mut())
				as *mut floormove_t);
			P_AddThinker(&raw mut floor.thinker);
			sec.specialdata = (floor as *mut floormove_t).cast();
			floor.thinker.function = think_t::T_MoveFloor;
			floor.ty = floortype;
			floor.crush = 0;

			match floortype {
				floor_e::lowerFloor => {
					floor.direction = -1;
					floor.sector = sec;
					floor.speed = FLOORSPEED;
					floor.floordestheight = P_FindHighestFloorSurrounding(sec);
				}

				floor_e::lowerFloorToLowest => {
					floor.direction = -1;
					floor.sector = sec;
					floor.speed = FLOORSPEED;
					floor.floordestheight = P_FindLowestFloorSurrounding(sec);
				}

				floor_e::turboLower => {
					floor.direction = -1;
					floor.sector = sec;
					floor.speed = FLOORSPEED * 4;
					floor.floordestheight = P_FindHighestFloorSurrounding(sec);
					if floor.floordestheight != sec.floorheight {
						floor.floordestheight += 8 * FRACUNIT;
					}
				}

				floor_e::raiseFloorCrush | floor_e::raiseFloor => {
					if floortype == floor_e::raiseFloorCrush {
						floor.crush = 1;
					}
					floor.direction = 1;
					floor.sector = sec;
					floor.speed = FLOORSPEED;
					floor.floordestheight = P_FindLowestCeilingSurrounding(sec);
					if floor.floordestheight > sec.ceilingheight {
						floor.floordestheight = sec.ceilingheight;
					}
					floor.floordestheight -=
						(8 * FRACUNIT) * (floortype == floor_e::raiseFloorCrush) as fixed_t;
				}

				floor_e::raiseFloorTurbo => {
					floor.direction = 1;
					floor.sector = sec;
					floor.speed = FLOORSPEED * 4;
					floor.floordestheight = P_FindNextHighestFloor(sec, sec.floorheight);
				}

				floor_e::raiseFloorToNearest => {
					floor.direction = 1;
					floor.sector = sec;
					floor.speed = FLOORSPEED;
					floor.floordestheight = P_FindNextHighestFloor(sec, sec.floorheight);
				}

				floor_e::raiseFloor24 => {
					floor.direction = 1;
					floor.sector = sec;
					floor.speed = FLOORSPEED;
					floor.floordestheight = (*floor.sector).floorheight + 24 * FRACUNIT;
				}

				floor_e::raiseFloor512 => {
					floor.direction = 1;
					floor.sector = sec;
					floor.speed = FLOORSPEED;
					floor.floordestheight = (*floor.sector).floorheight + 512 * FRACUNIT;
				}

				floor_e::raiseFloor24AndChange => {
					floor.direction = 1;
					floor.sector = sec;
					floor.speed = FLOORSPEED;
					floor.floordestheight = (*floor.sector).floorheight + 24 * FRACUNIT;
					sec.floorpic = (*line.frontsector).floorpic;
					sec.special = (*line.frontsector).special;
				}

				floor_e::raiseToTexture => {
					let mut minsize = i32::MAX;

					floor.direction = 1;
					floor.sector = sec;
					floor.speed = FLOORSPEED;
					for i in 0..sec.linecount {
						if twoSided(secnum, i) {
							let side = getSide(secnum, i, 0);
							if (*side).bottomtexture >= 0
								&& *textureheight.wrapping_add((*side).bottomtexture as usize)
									< minsize
							{
								minsize =
									*textureheight.wrapping_add((*side).bottomtexture as usize);
							}
							let side = getSide(secnum, i, 1);
							if (*side).bottomtexture >= 0
								&& *textureheight.wrapping_add((*side).bottomtexture as usize)
									< minsize
							{
								minsize =
									*textureheight.wrapping_add((*side).bottomtexture as usize);
							}
						}
					}
					floor.floordestheight = (*floor.sector).floorheight + minsize;
				}

				floor_e::lowerAndChange => {
					floor.direction = -1;
					floor.sector = sec;
					floor.speed = FLOORSPEED;
					floor.floordestheight = P_FindLowestFloorSurrounding(sec);
					floor.texture = sec.floorpic;

					for i in 0..sec.linecount {
						if twoSided(secnum, i) {
							if (*getSide(secnum, i, 0)).sector.offset_from(sectors) == secnum {
								let sec = getSector(secnum, i, 1);

								if (*sec).floorheight == floor.floordestheight {
									floor.texture = (*sec).floorpic;
									floor.newspecial = (*sec).special as i32;
									break;
								}
							} else {
								let sec = getSector(secnum, i, 0);

								if (*sec).floorheight == floor.floordestheight {
									floor.texture = (*sec).floorpic;
									floor.newspecial = (*sec).special as i32;
									break;
								}
							}
						}
					}
				}

				_ => (),
			}
		}
		return rtn;
	}
}

// BUILD A STAIRCASE!
pub(crate) fn EV_BuildStairs(line: &mut line_t, ty: stair_e) -> bool {
	unsafe {
		let mut secnum = -1;
		let mut rtn = false;
		while let new_secnum @ 0.. = P_FindSectorFromLineTag(line, secnum) {
			secnum = new_secnum;
			let mut sec = &mut *sectors.wrapping_add(secnum as usize);

			// ALREADY MOVING?  IF SO, KEEP GOING...
			if !sec.specialdata.is_null() {
				continue;
			}

			// new floor thinker
			rtn = true;
			let floor = &mut *(Z_Malloc(size_of::<floormove_t>(), PU_LEVSPEC, null_mut())
				as *mut floormove_t);
			P_AddThinker(&raw mut floor.thinker);
			sec.specialdata = (floor as *mut floormove_t).cast();
			floor.thinker.function = think_t::T_MoveFloor;
			floor.direction = 1;
			floor.sector = sec;

			let (speed, stairsize) = match ty {
				stair_e::build8 => (FLOORSPEED / 4, 8 * FRACUNIT),
				stair_e::turbo16 => (FLOORSPEED * 4, 16 * FRACUNIT),
			};
			floor.speed = speed;
			let mut height = sec.floorheight + stairsize;
			floor.floordestheight = height;

			let texture = sec.floorpic;

			// Find next sector to raise
			// 1.	Find 2-sided line with same sector side[0]
			// 2.	Other side is the next sector to raise
			loop {
				let mut ok = false;
				for i in 0..sec.linecount {
					if (**sec.lines.wrapping_add(i)).flags & ML_TWOSIDED == 0 {
						continue;
					}

					let tsec = (**sec.lines.wrapping_add(i)).frontsector;
					let newsecnum = tsec.offset_from(sectors);

					if secnum != newsecnum {
						continue;
					}

					let tsec = (**sec.lines.wrapping_add(i)).backsector;
					let newsecnum = tsec.offset_from(sectors);

					if (*tsec).floorpic != texture {
						continue;
					}

					height += stairsize;

					if !(*tsec).specialdata.is_null() {
						continue;
					}

					sec = &mut *tsec;
					secnum = newsecnum;
					let floor = &mut *(Z_Malloc(size_of::<floormove_t>(), PU_LEVSPEC, null_mut())
						as *mut floormove_t);
					P_AddThinker(&raw mut floor.thinker);

					sec.specialdata = (floor as *mut floormove_t).cast();
					floor.thinker.function = think_t::T_MoveFloor;
					floor.direction = 1;
					floor.sector = sec;
					floor.speed = speed;
					floor.floordestheight = height;
					ok = true;
					break;
				}
				if !ok {
					break;
				}
			}
		}
		return rtn;
	}
}
