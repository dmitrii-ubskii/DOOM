#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

// DESCRIPTION:
//	Archiving: SaveGame I/O.

use std::{
	ffi::c_int,
	ptr::{self, null_mut},
};

use crate::{
	d_player::player_t,
	d_think::think_t,
	doomdef::MAXPLAYERS,
	g_game::{playeringame, players},
	i_system::I_Error,
	info::{mobjinfo, states},
	m_fixed::FRACBITS,
	p_ceiling::{P_AddActiveCeiling, activeceilings},
	p_local::thinkercap,
	p_maputl::P_SetThingPosition,
	p_mobj::{P_RemoveMobj, mobj_t},
	p_plats::P_AddActivePlat,
	p_pspr::psprnum_t,
	p_setup::{lines, numlines, numsectors, sectors, sides},
	p_spec::{
		MAXCEILINGS, ceiling_t, floormove_t, glow_t, lightflash_t, plat_t, strobe_t, vldoor_t,
	},
	p_tick::{P_AddThinker, P_InitThinkers},
	z_zone::{PU_LEVEL, Z_Free, Z_Malloc},
};

pub(crate) static mut save_p: *mut u8 = null_mut();

// Pads save_p to a 4-byte boundary
//  so that the load/save works on SGI&Gecko.
fn PADSAVEP() {
	unsafe {
		save_p = save_p.wrapping_add(save_p.align_offset(4));
	}
}

// P_ArchivePlayers
#[allow(static_mut_refs)]
pub(crate) fn P_ArchivePlayers() {
	unsafe {
		for i in 0..MAXPLAYERS {
			if playeringame[i] == 0 {
				continue;
			}

			PADSAVEP();

			let dest = save_p.cast::<player_t>();
			libc::memcpy(dest.cast(), (&raw const players[i]).cast(), size_of::<player_t>());
			let dest = &mut *dest;
			save_p = save_p.wrapping_add(size_of::<player_t>());
			for j in 0..usize::from(psprnum_t::NUMPSPRITES) {
				if !dest.psprites[j].state.is_null() {
					dest.psprites[j].state = ptr::without_provenance_mut(
						usize::try_from(dest.psprites[j].state.offset_from(states.as_ptr()))
							.unwrap(),
					);
				}
			}
		}
	}
}

// P_UnArchivePlayers
pub(crate) fn P_UnArchivePlayers() {
	unsafe {
		for i in 0..MAXPLAYERS {
			if playeringame[i] == 0 {
				continue;
			}

			PADSAVEP();

			libc::memcpy((&raw mut players[i]).cast(), save_p.cast(), size_of::<player_t>());
			save_p = save_p.wrapping_add(size_of::<player_t>());

			// will be set when unarc thinker
			players[i].mo = null_mut();
			players[i].message = null_mut();
			players[i].attacker = null_mut();

			for j in 0..usize::from(psprnum_t::NUMPSPRITES) {
				if !players[i].psprites[j].state.is_null() {
					players[i].psprites[j].state =
						&raw mut states[players[i].psprites[j].state.addr()];
				}
			}
		}
	}
}

// P_ArchiveWorld
pub(crate) fn P_ArchiveWorld() {
	unsafe {
		let mut put = save_p.cast::<i16>();

		// do sectors
		let mut sec = sectors;
		for _ in 0..numsectors {
			*put = i16::try_from((*sec).floorheight >> FRACBITS).unwrap();
			put = put.wrapping_add(1);
			*put = i16::try_from((*sec).ceilingheight >> FRACBITS).unwrap();
			put = put.wrapping_add(1);
			*put = (*sec).floorpic;
			put = put.wrapping_add(1);
			*put = (*sec).ceilingpic;
			put = put.wrapping_add(1);
			*put = (*sec).lightlevel;
			put = put.wrapping_add(1);
			*put = (*sec).special; // needed?
			put = put.wrapping_add(1);
			*put = (*sec).tag; // needed?
			put = put.wrapping_add(1);

			sec = sec.wrapping_add(1);
		}

		// do lines
		let mut li = lines;
		for _ in 0..numlines {
			*put = (*li).flags;
			put = put.wrapping_add(1);
			*put = (*li).special;
			put = put.wrapping_add(1);
			*put = (*li).tag;
			put = put.wrapping_add(1);
			for j in 0..2 {
				if (*li).sidenum[j] == -1 {
					continue;
				}

				let si = sides.wrapping_add(usize::try_from((*li).sidenum[j]).unwrap());

				*put = i16::try_from((*si).textureoffset >> FRACBITS).unwrap();
				put = put.wrapping_add(1);
				*put = i16::try_from((*si).rowoffset >> FRACBITS).unwrap();
				put = put.wrapping_add(1);
				*put = (*si).toptexture;
				put = put.wrapping_add(1);
				*put = (*si).bottomtexture;
				put = put.wrapping_add(1);
				*put = (*si).midtexture;
				put = put.wrapping_add(1);
			}
			li = li.wrapping_add(1);
		}

		save_p = put.cast();
	}
}

// P_UnArchiveWorld
pub(crate) fn P_UnArchiveWorld() {
	unsafe {
		let mut get = save_p.cast::<i16>();

		// do sectors
		let mut sec = sectors;
		for _ in 0..numsectors {
			(*sec).floorheight = (i32::from(*get)) << FRACBITS;
			get = get.wrapping_add(1);
			(*sec).ceilingheight = (i32::from(*get)) << FRACBITS;
			get = get.wrapping_add(1);
			(*sec).floorpic = *get;
			get = get.wrapping_add(1);
			(*sec).ceilingpic = *get;
			get = get.wrapping_add(1);
			(*sec).lightlevel = *get;
			get = get.wrapping_add(1);
			(*sec).special = *get; // needed?
			get = get.wrapping_add(1);
			(*sec).tag = *get; // needed?
			get = get.wrapping_add(1);
			(*sec).specialdata = null_mut();
			(*sec).soundtarget = null_mut();

			sec = sec.wrapping_add(1);
		}

		// do lines
		let mut li = lines;
		for _ in 0..numlines {
			(*li).flags = *get;
			get = get.wrapping_add(1);
			(*li).special = *get;
			get = get.wrapping_add(1);
			(*li).tag = *get;
			get = get.wrapping_add(1);
			for j in 0..2 {
				if (*li).sidenum[j] == -1 {
					continue;
				}
				let si = sides.wrapping_add(usize::try_from((*li).sidenum[j]).unwrap());
				(*si).textureoffset = (i32::from(*get)) << FRACBITS;
				get = get.wrapping_add(1);
				(*si).rowoffset = (i32::from(*get)) << FRACBITS;
				get = get.wrapping_add(1);
				(*si).toptexture = *get;
				get = get.wrapping_add(1);
				(*si).bottomtexture = *get;
				get = get.wrapping_add(1);
				(*si).midtexture = *get;
				get = get.wrapping_add(1);
			}
			li = li.wrapping_add(1);
		}

		save_p = get.cast();
	}
}

// Thinkers
#[repr(C)]
enum thinkerclass_t {
	tc_end,
	tc_mobj,
}

impl From<thinkerclass_t> for u8 {
	fn from(value: thinkerclass_t) -> Self {
		match value {
			thinkerclass_t::tc_end => 0,
			thinkerclass_t::tc_mobj => 1,
		}
	}
}

// P_ArchiveThinkers
#[allow(static_mut_refs)]
pub(crate) fn P_ArchiveThinkers() {
	unsafe {
		// save off the current thinkers
		let mut th = thinkercap.next;
		while !ptr::eq(th, &raw const thinkercap) {
			if (*th).function.is_mobj() {
				*save_p = u8::from(thinkerclass_t::tc_mobj);
				save_p = save_p.wrapping_add(1);
				PADSAVEP();
				let mobj = save_p.cast::<mobj_t>();
				libc::memcpy(mobj.cast(), th.cast(), size_of::<mobj_t>());
				save_p = save_p.wrapping_add(size_of::<mobj_t>());
				(*mobj).state = ptr::without_provenance_mut(
					((*mobj).state.offset_from(states.as_ptr())).try_into().unwrap(),
				);

				if !(*mobj).player.is_null() {
					(*mobj).player = ptr::without_provenance_mut(
						(((*mobj).player.offset_from(players.as_ptr())) + 1).try_into().unwrap(),
					);
				}
				{
					th = (*th).next;
					continue;
				}
			}

			// I_Error ("P_ArchiveThinkers: Unknown thinker function");
			th = (*th).next;
		}

		// add a terminating marker
		*save_p = u8::from(thinkerclass_t::tc_end);
		save_p = save_p.wrapping_add(1);
	}
}

// P_UnArchiveThinkers
pub(crate) fn P_UnArchiveThinkers() {
	unsafe {
		// remove all the current thinkers
		let mut currentthinker = thinkercap.next;
		while !ptr::eq(currentthinker, &raw const thinkercap) {
			let next = (*currentthinker).next;

			if (*currentthinker).function.is_mobj() {
				P_RemoveMobj(&mut *currentthinker.cast());
			} else {
				Z_Free(currentthinker.cast());
			}

			currentthinker = next;
		}
		P_InitThinkers();

		// read in saved thinkers
		loop {
			let tclass = *save_p;
			save_p = save_p.wrapping_add(1);
			match tclass {
				0 => return, // end of list

				1 => {
					PADSAVEP();
					let mobj = Z_Malloc(size_of::<mobj_t>(), PU_LEVEL, null_mut()).cast::<mobj_t>();
					libc::memcpy(mobj.cast(), save_p.cast(), size_of::<mobj_t>());
					save_p = save_p.wrapping_add(size_of::<mobj_t>());
					(*mobj).state = &raw mut states[(*mobj).state.addr()];
					(*mobj).target = null_mut();
					if !(*mobj).player.is_null() {
						(*mobj).player = &raw mut players[(*mobj).player.addr() - 1];
						(*(*mobj).player).mo = mobj;
					}
					P_SetThingPosition(&mut *mobj);
					(*mobj).info = &raw mut mobjinfo[usize::from((*mobj).ty)];
					(*mobj).floorz = (*(*(*mobj).subsector).sector).floorheight;
					(*mobj).ceilingz = (*(*(*mobj).subsector).sector).ceilingheight;
					(*mobj).thinker.function = think_t::mobj;
					P_AddThinker(&mut (*mobj).thinker);
				}

				_ => I_Error(c"Unknown tclass %i in savegame".as_ptr(), c_int::from(tclass)),
			}
		}
	}
}

// P_ArchiveSpecials
enum specials_e {
	tc_ceiling,
	tc_door,
	tc_floor,
	tc_plat,
	tc_flash,
	tc_strobe,
	tc_glow,
	tc_endspecials,
}

impl From<u8> for specials_e {
	fn from(value: u8) -> Self {
		match value {
			0 => Self::tc_ceiling,
			1 => Self::tc_door,
			2 => Self::tc_floor,
			3 => Self::tc_plat,
			4 => Self::tc_flash,
			5 => Self::tc_strobe,
			6 => Self::tc_glow,
			7 => Self::tc_endspecials,
			_ => panic!("specials_e out of bounds"),
		}
	}
}

impl From<specials_e> for u8 {
	fn from(value: specials_e) -> Self {
		match value {
			specials_e::tc_ceiling => 0,
			specials_e::tc_door => 1,
			specials_e::tc_floor => 2,
			specials_e::tc_plat => 3,
			specials_e::tc_flash => 4,
			specials_e::tc_strobe => 5,
			specials_e::tc_glow => 6,
			specials_e::tc_endspecials => 7,
		}
	}
}

// Things to handle:
//
// T_MoveCeiling, (ceiling_t: sector_t * swizzle), - active list
// T_VerticalDoor, (vldoor_t: sector_t * swizzle),
// T_MoveFloor, (floormove_t: sector_t * swizzle),
// T_LightFlash, (lightflash_t: sector_t * swizzle),
// T_StrobeFlash, (strobe_t: sector_t *),
// T_Glow, (glow_t: sector_t *),
// T_PlatRaise, (plat_t: sector_t *), - active list
pub(crate) fn P_ArchiveSpecials() {
	unsafe {
		// save off the current thinkers
		let mut th = thinkercap.next;
		while !ptr::eq(th, &raw const thinkercap) {
			if (*th).function.is_null() {
				let mut i = 0;
				#[allow(clippy::needless_range_loop)]
				for j in 0..MAXCEILINGS {
					if std::ptr::eq(activeceilings[j], th.cast()) {
						i = j;
						break;
					}
				}

				if i < MAXCEILINGS {
					*save_p = u8::from(specials_e::tc_ceiling);
					save_p = save_p.wrapping_add(1);
					PADSAVEP();
					let ceiling = save_p.cast::<ceiling_t>();
					libc::memcpy(ceiling.cast(), th.cast(), size_of::<ceiling_t>());
					save_p = save_p.wrapping_add(size_of::<ceiling_t>());
					(*ceiling).sector = ptr::without_provenance_mut(
						(*ceiling).sector.offset_from(sectors).try_into().unwrap(),
					);
				}
				th = (*th).next;
				continue;
			}

			if (*th).function == think_t::T_MoveCeiling {
				*save_p = u8::from(specials_e::tc_ceiling);
				save_p = save_p.wrapping_add(1);
				PADSAVEP();
				let ceiling = save_p.cast::<ceiling_t>();
				libc::memcpy(ceiling.cast(), th.cast(), size_of::<ceiling_t>());
				save_p = save_p.wrapping_add(size_of::<ceiling_t>());
				(*ceiling).sector = ptr::without_provenance_mut(
					(*ceiling).sector.offset_from(sectors).try_into().unwrap(),
				);
				th = (*th).next;
				continue;
			}

			if (*th).function == think_t::T_VerticalDoor {
				*save_p = u8::from(specials_e::tc_door);
				save_p = save_p.wrapping_add(1);
				PADSAVEP();
				let door = save_p.cast::<vldoor_t>();
				libc::memcpy(door.cast(), th.cast(), size_of::<vldoor_t>());
				save_p = save_p.wrapping_add(size_of::<vldoor_t>());
				(*door).sector = ptr::without_provenance_mut(
					(*door).sector.offset_from(sectors).try_into().unwrap(),
				);
				th = (*th).next;
				continue;
			}

			if (*th).function == think_t::T_MoveFloor {
				*save_p = u8::from(specials_e::tc_floor);
				save_p = save_p.wrapping_add(1);
				PADSAVEP();
				let floor = save_p.cast::<floormove_t>();
				libc::memcpy(floor.cast(), th.cast(), size_of::<floormove_t>());
				save_p = save_p.wrapping_add(size_of::<floormove_t>());
				(*floor).sector = ptr::without_provenance_mut(
					(*floor).sector.offset_from(sectors).try_into().unwrap(),
				);
				th = (*th).next;
				continue;
			}

			if (*th).function == think_t::T_PlatRaise {
				*save_p = u8::from(specials_e::tc_plat);
				save_p = save_p.wrapping_add(1);
				PADSAVEP();
				let plat = save_p.cast::<plat_t>();
				libc::memcpy(plat.cast(), th.cast(), size_of::<plat_t>());
				save_p = save_p.wrapping_add(size_of::<plat_t>());
				(*plat).sector = ptr::without_provenance_mut(
					(*plat).sector.offset_from(sectors).try_into().unwrap(),
				);
				th = (*th).next;
				continue;
			}

			if (*th).function == think_t::T_LightFlash {
				*save_p = u8::from(specials_e::tc_flash);
				save_p = save_p.wrapping_add(1);
				PADSAVEP();
				let flash = save_p.cast::<lightflash_t>();
				libc::memcpy(flash.cast(), th.cast(), size_of::<lightflash_t>());
				save_p = save_p.wrapping_add(size_of::<lightflash_t>());
				(*flash).sector = ptr::without_provenance_mut(
					(*flash).sector.offset_from(sectors).try_into().unwrap(),
				);
				th = (*th).next;
				continue;
			}

			if (*th).function == think_t::T_StrobeFlash {
				*save_p = u8::from(specials_e::tc_strobe);
				save_p = save_p.wrapping_add(1);
				PADSAVEP();
				let strobe = save_p.cast::<strobe_t>();
				libc::memcpy(strobe.cast(), th.cast(), size_of::<strobe_t>());
				save_p = save_p.wrapping_add(size_of::<strobe_t>());
				(*strobe).sector = ptr::without_provenance_mut(
					(*strobe).sector.offset_from(sectors).try_into().unwrap(),
				);
				th = (*th).next;
				continue;
			}

			if (*th).function == think_t::T_Glow {
				*save_p = u8::from(specials_e::tc_glow);
				save_p = save_p.wrapping_add(1);
				PADSAVEP();
				let glow = save_p.cast::<glow_t>();
				libc::memcpy(glow.cast(), th.cast(), size_of::<glow_t>());
				save_p = save_p.wrapping_add(size_of::<glow_t>());
				(*glow).sector = ptr::without_provenance_mut(
					(*glow).sector.offset_from(sectors).try_into().unwrap(),
				);
				th = (*th).next;
				continue;
			}

			th = (*th).next;
		}

		// add a terminating marker
		*save_p = u8::from(specials_e::tc_endspecials);
		save_p = save_p.wrapping_add(1);
	}
}

// P_UnArchiveSpecials
pub(crate) fn P_UnArchiveSpecials() {
	unsafe {
		// read in saved thinkers
		loop {
			let tclass = specials_e::from(*save_p);
			save_p = save_p.wrapping_add(1);
			match tclass {
				specials_e::tc_endspecials => return, // end of list

				specials_e::tc_ceiling => {
					PADSAVEP();
					let ceiling =
						Z_Malloc(size_of::<ceiling_t>(), PU_LEVEL, null_mut()).cast::<ceiling_t>();
					libc::memcpy(ceiling.cast(), save_p.cast(), size_of::<ceiling_t>());
					save_p = save_p.wrapping_add(size_of::<ceiling_t>());
					(*ceiling).sector = sectors.wrapping_add((*ceiling).sector.addr());
					(*(*ceiling).sector).specialdata = ceiling.cast();

					if (*ceiling).thinker.function.as_acp1().is_some() {
						(*ceiling).thinker.function = think_t::T_MoveCeiling;
					}

					P_AddThinker(&mut (*ceiling).thinker);
					P_AddActiveCeiling(ceiling);
				}

				specials_e::tc_door => {
					PADSAVEP();
					let door =
						Z_Malloc(size_of::<vldoor_t>(), PU_LEVEL, null_mut()).cast::<vldoor_t>();
					libc::memcpy(door.cast(), save_p.cast(), size_of::<vldoor_t>());
					save_p = save_p.wrapping_add(size_of::<vldoor_t>());
					(*door).sector = sectors.wrapping_add((*door).sector.addr());
					(*(*door).sector).specialdata = door.cast();
					(*door).thinker.function = think_t::T_VerticalDoor;
					P_AddThinker(&mut (*door).thinker);
				}

				specials_e::tc_floor => {
					PADSAVEP();
					let floor = Z_Malloc(size_of::<floormove_t>(), PU_LEVEL, null_mut())
						.cast::<floormove_t>();
					libc::memcpy(floor.cast(), save_p.cast(), size_of::<floormove_t>());
					save_p = save_p.wrapping_add(size_of::<floormove_t>());
					(*floor).sector = sectors.wrapping_add((*floor).sector.addr());
					(*(*floor).sector).specialdata = floor.cast();
					(*floor).thinker.function = think_t::T_MoveFloor;
					P_AddThinker(&mut (*floor).thinker);
				}

				specials_e::tc_plat => {
					PADSAVEP();
					let plat = Z_Malloc(size_of::<plat_t>(), PU_LEVEL, null_mut()).cast::<plat_t>();
					libc::memcpy(plat.cast(), save_p.cast(), size_of::<plat_t>());
					save_p = save_p.wrapping_add(size_of::<plat_t>());
					(*plat).sector = sectors.wrapping_add((*plat).sector.addr());
					(*(*plat).sector).specialdata = plat.cast();

					if (*plat).thinker.function.as_acp1().is_some() {
						(*plat).thinker.function = think_t::T_PlatRaise;
					}

					P_AddThinker(&mut (*plat).thinker);
					P_AddActivePlat(plat);
				}

				specials_e::tc_flash => {
					PADSAVEP();
					let flash = Z_Malloc(size_of::<lightflash_t>(), PU_LEVEL, null_mut())
						.cast::<lightflash_t>();
					libc::memcpy(flash.cast(), save_p.cast(), size_of::<lightflash_t>());
					save_p = save_p.wrapping_add(size_of::<lightflash_t>());
					(*flash).sector = sectors.wrapping_add((*flash).sector.addr());
					(*flash).thinker.function = think_t::T_LightFlash;
					P_AddThinker(&mut (*flash).thinker);
				}

				specials_e::tc_strobe => {
					PADSAVEP();
					let strobe =
						Z_Malloc(size_of::<strobe_t>(), PU_LEVEL, null_mut()).cast::<strobe_t>();
					libc::memcpy(strobe.cast(), save_p.cast(), size_of::<strobe_t>());
					save_p = save_p.wrapping_add(size_of::<strobe_t>());
					(*strobe).sector = sectors.wrapping_add((*strobe).sector.addr());
					(*strobe).thinker.function = think_t::T_StrobeFlash;
					P_AddThinker(&mut (*strobe).thinker);
				}

				specials_e::tc_glow => {
					PADSAVEP();
					let glow = Z_Malloc(size_of::<glow_t>(), PU_LEVEL, null_mut()).cast::<glow_t>();
					libc::memcpy(glow.cast(), save_p.cast(), size_of::<glow_t>());
					save_p = save_p.wrapping_add(size_of::<glow_t>());
					(*glow).sector = sectors.wrapping_add((*glow).sector.addr());
					(*glow).thinker.function = think_t::T_Glow;
					P_AddThinker(&mut (*glow).thinker);
				}
			}
		}
	}
}
