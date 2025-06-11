#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

// DESCRIPTION:
//	Archiving: SaveGame I/O.

use std::{
	ffi::{c_int, c_void},
	ptr::{self, null_mut},
};

use crate::{
	d_player::player_t,
	d_think::actionf_p1,
	doomdef::MAXPLAYERS,
	g_game::{playeringame, players},
	i_system::I_Error,
	info::{mobjinfo, state_t, states},
	m_fixed::FRACBITS,
	p_lights::{T_Glow_action, T_LightFlash_action, T_StrobeFlash_action},
	p_local::thinkercap,
	p_mobj::mobj_t,
	p_pspr::psprnum_t,
	p_setup::{lines, numlines, numsectors, sectors, sides},
	p_spec::{
		MAXCEILINGS, ceiling_t, floormove_t, glow_t, lightflash_t, plat_t, strobe_t, vldoor_t,
	},
	p_tick::{P_AddThinker, P_InitThinkers},
	r_defs::sector_t,
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

			let dest = save_p as *mut player_t;
			libc::memcpy(dest.cast(), (&raw const players[i]).cast(), size_of::<player_t>());
			let dest = &mut *dest;
			save_p = save_p.wrapping_add(size_of::<player_t>());
			for j in 0..psprnum_t::NUMPSPRITES as usize {
				if !dest.psprites[j].state.is_null() {
					dest.psprites[j].state =
						dest.psprites[j].state.offset_from(states.as_ptr()) as *mut state_t;
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

			for j in 0..psprnum_t::NUMPSPRITES as usize {
				if !players[i].psprites[j].state.is_null() {
					players[i].psprites[j].state =
						&raw mut states[players[i].psprites[j].state as usize];
				}
			}
		}
	}
}

// P_ArchiveWorld
pub(crate) fn P_ArchiveWorld() {
	unsafe {
		let mut put = save_p as *mut i16;

		// do sectors
		let mut sec = sectors;
		for _ in 0..numsectors {
			*put = ((*sec).floorheight >> FRACBITS) as i16;
			put = put.wrapping_add(1);
			*put = ((*sec).ceilingheight >> FRACBITS) as i16;
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

				let si = sides.wrapping_add((*li).sidenum[j] as usize);

				*put = ((*si).textureoffset >> FRACBITS) as i16;
				put = put.wrapping_add(1);
				*put = ((*si).rowoffset >> FRACBITS) as i16;
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
		let mut get = save_p as *mut i16;

		// do sectors
		let mut sec = sectors;
		for _ in 0..numsectors {
			(*sec).floorheight = (*get as i32) << FRACBITS;
			get = get.wrapping_add(1);
			(*sec).ceilingheight = (*get as i32) << FRACBITS;
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
				let si = sides.wrapping_add((*li).sidenum[j] as usize);
				(*si).textureoffset = (*get as i32) << FRACBITS;
				get = get.wrapping_add(1);
				(*si).rowoffset = (*get as i32) << FRACBITS;
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

unsafe extern "C" {
	fn P_MobjThinker(_: *mut c_void);
}

// P_ArchiveThinkers
#[allow(static_mut_refs)]
pub(crate) fn P_ArchiveThinkers() {
	unsafe {
		// save off the current thinkers
		let mut th = thinkercap.next;
		while !ptr::eq(th, &raw const thinkercap) {
			if (*th).function.acp1.is_some_and(|f| ptr::fn_addr_eq(f, P_MobjThinker as actionf_p1))
			{
				*save_p = thinkerclass_t::tc_mobj as u8;
				save_p = save_p.wrapping_add(1);
				PADSAVEP();
				let mobj = save_p as *mut mobj_t;
				libc::memcpy(mobj.cast(), th.cast(), size_of::<mobj_t>());
				save_p = save_p.wrapping_add(size_of::<mobj_t>());
				(*mobj).state = ((*mobj).state.offset_from(states.as_ptr())) as *mut state_t;

				if !(*mobj).player.is_null() {
					(*mobj).player =
						(((*mobj).player.offset_from(players.as_ptr())) + 1) as *mut player_t;
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
		*save_p = thinkerclass_t::tc_end as u8;
		save_p = save_p.wrapping_add(1);
	}
}

unsafe extern "C" {
	fn P_RemoveMobj(thing: *mut mobj_t);
	fn P_SetThingPosition(thing: *mut mobj_t);
}

// P_UnArchiveThinkers
pub(crate) fn P_UnArchiveThinkers() {
	unsafe {
		// remove all the current thinkers
		let mut currentthinker = thinkercap.next;
		while !ptr::eq(currentthinker, &raw const thinkercap) {
			let next = (*currentthinker).next;

			if (*currentthinker)
				.function
				.acp1
				.is_some_and(|f| ptr::fn_addr_eq(f, P_MobjThinker as actionf_p1))
			{
				P_RemoveMobj(currentthinker.cast());
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
					let mobj = Z_Malloc(size_of::<mobj_t>(), PU_LEVEL, null_mut()) as *mut mobj_t;
					libc::memcpy(mobj.cast(), save_p.cast(), size_of::<mobj_t>());
					save_p = save_p.wrapping_add(size_of::<mobj_t>());
					(*mobj).state = &raw mut states[(*mobj).state as usize];
					(*mobj).target = null_mut();
					if !(*mobj).player.is_null() {
						(*mobj).player = &raw mut players[(*mobj).player as usize - 1];
						(*(*mobj).player).mo = mobj;
					}
					P_SetThingPosition(mobj);
					(*mobj).info = &raw mut mobjinfo[(*mobj).ty as usize];
					(*mobj).floorz = (*(*(*mobj).subsector).sector).floorheight;
					(*mobj).ceilingz = (*(*(*mobj).subsector).sector).ceilingheight;
					(*mobj).thinker.function.acp1 = Some(P_MobjThinker);
					P_AddThinker(&mut (*mobj).thinker);
				}

				_ => I_Error(c"Unknown tclass %i in savegame".as_ptr(), tclass as c_int),
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

unsafe extern "C" {
	static mut activeceilings: [*mut ceiling_t; MAXCEILINGS];

	fn T_MoveCeiling(_: *mut c_void);
	fn T_VerticalDoor(_: *mut c_void);
	fn T_MoveFloor(_: *mut c_void);
	fn T_PlatRaise(_: *mut c_void);
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
			if (*th).function.acv.is_none() {
				let mut i = 0;
				#[allow(clippy::needless_range_loop)]
				for j in 0..MAXCEILINGS {
					if std::ptr::eq(activeceilings[j], th.cast()) {
						i = j;
						break;
					}
				}

				if i < MAXCEILINGS {
					*save_p = specials_e::tc_ceiling as u8;
					save_p = save_p.wrapping_add(1);
					PADSAVEP();
					let ceiling = save_p.cast::<ceiling_t>();
					libc::memcpy(ceiling.cast(), th.cast(), size_of::<ceiling_t>());
					save_p = save_p.wrapping_add(size_of::<ceiling_t>());
					(*ceiling).sector = ((*ceiling).sector.offset_from(sectors)) as *mut sector_t;
				}
				th = (*th).next;
				continue;
			}

			if (*th).function.acp1.is_some_and(|f| ptr::fn_addr_eq(f, T_MoveCeiling as actionf_p1))
			{
				*save_p = specials_e::tc_ceiling as u8;
				save_p = save_p.wrapping_add(1);
				PADSAVEP();
				let ceiling = save_p as *mut ceiling_t;
				libc::memcpy(ceiling.cast(), th.cast(), size_of::<ceiling_t>());
				save_p = save_p.wrapping_add(size_of::<ceiling_t>());
				(*ceiling).sector = ((*ceiling).sector.offset_from(sectors)) as *mut sector_t;
				th = (*th).next;
				continue;
			}

			if (*th).function.acp1.is_some_and(|f| ptr::fn_addr_eq(f, T_VerticalDoor as actionf_p1))
			{
				*save_p = specials_e::tc_door as u8;
				save_p = save_p.wrapping_add(1);
				PADSAVEP();
				let door = save_p as *mut vldoor_t;
				libc::memcpy(door.cast(), th.cast(), size_of::<vldoor_t>());
				save_p = save_p.wrapping_add(size_of::<vldoor_t>());
				(*door).sector = ((*door).sector.offset_from(sectors)) as *mut sector_t;
				th = (*th).next;
				continue;
			}

			if (*th).function.acp1.is_some_and(|f| ptr::fn_addr_eq(f, T_MoveFloor as actionf_p1)) {
				*save_p = specials_e::tc_floor as u8;
				save_p = save_p.wrapping_add(1);
				PADSAVEP();
				let floor = save_p as *mut floormove_t;
				libc::memcpy(floor.cast(), th.cast(), size_of::<floormove_t>());
				save_p = save_p.wrapping_add(size_of::<floormove_t>());
				(*floor).sector = ((*floor).sector.offset_from(sectors)) as *mut sector_t;
				th = (*th).next;
				continue;
			}

			if (*th).function.acp1.is_some_and(|f| ptr::fn_addr_eq(f, T_PlatRaise as actionf_p1)) {
				*save_p = specials_e::tc_plat as u8;
				save_p = save_p.wrapping_add(1);
				PADSAVEP();
				let plat = save_p as *mut plat_t;
				libc::memcpy(plat.cast(), th.cast(), size_of::<plat_t>());
				save_p = save_p.wrapping_add(size_of::<plat_t>());
				(*plat).sector = ((*plat).sector.offset_from(sectors)) as *mut sector_t;
				th = (*th).next;
				continue;
			}

			if (*th)
				.function
				.acp1
				.is_some_and(|f| ptr::fn_addr_eq(f, T_LightFlash_action as actionf_p1))
			{
				*save_p = specials_e::tc_flash as u8;
				save_p = save_p.wrapping_add(1);
				PADSAVEP();
				let flash = save_p as *mut lightflash_t;
				libc::memcpy(flash.cast(), th.cast(), size_of::<lightflash_t>());
				save_p = save_p.wrapping_add(size_of::<lightflash_t>());
				(*flash).sector = ((*flash).sector.offset_from(sectors)) as *mut sector_t;
				th = (*th).next;
				continue;
			}

			if (*th)
				.function
				.acp1
				.is_some_and(|f| ptr::fn_addr_eq(f, T_StrobeFlash_action as actionf_p1))
			{
				*save_p = specials_e::tc_strobe as u8;
				save_p = save_p.wrapping_add(1);
				PADSAVEP();
				let strobe = save_p as *mut strobe_t;
				libc::memcpy(strobe.cast(), th.cast(), size_of::<strobe_t>());
				save_p = save_p.wrapping_add(size_of::<strobe_t>());
				(*strobe).sector = ((*strobe).sector.offset_from(sectors)) as *mut sector_t;
				th = (*th).next;
				continue;
			}

			if (*th).function.acp1.is_some_and(|f| ptr::fn_addr_eq(f, T_Glow_action as actionf_p1))
			{
				*save_p = specials_e::tc_glow as u8;
				save_p = save_p.wrapping_add(1);
				PADSAVEP();
				let glow = save_p as *mut glow_t;
				libc::memcpy(glow.cast(), th.cast(), size_of::<glow_t>());
				save_p = save_p.wrapping_add(size_of::<glow_t>());
				(*glow).sector = ((*glow).sector.offset_from(sectors)) as *mut sector_t;
				th = (*th).next;
				continue;
			}

			th = (*th).next;
		}

		// add a terminating marker
		*save_p = specials_e::tc_endspecials as u8;
		save_p = save_p.wrapping_add(1);
	}
}

unsafe extern "C" {
	fn P_AddActiveCeiling(ceiling: *mut ceiling_t);
	fn P_AddActivePlat(plat: *mut plat_t);
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
						Z_Malloc(size_of::<ceiling_t>(), PU_LEVEL, null_mut()) as *mut ceiling_t;
					libc::memcpy(ceiling.cast(), save_p.cast(), size_of::<ceiling_t>());
					save_p = save_p.wrapping_add(size_of::<ceiling_t>());
					(*ceiling).sector = sectors.wrapping_add((*ceiling).sector as usize);
					(*(*ceiling).sector).specialdata = ceiling.cast();

					if (*ceiling).thinker.function.acp1.is_some() {
						(*ceiling).thinker.function.acp1 = Some(T_MoveCeiling);
					}

					P_AddThinker(&mut (*ceiling).thinker);
					P_AddActiveCeiling(ceiling);
				}

				specials_e::tc_door => {
					PADSAVEP();
					let door =
						Z_Malloc(size_of::<vldoor_t>(), PU_LEVEL, null_mut()) as *mut vldoor_t;
					libc::memcpy(door.cast(), save_p.cast(), size_of::<vldoor_t>());
					save_p = save_p.wrapping_add(size_of::<vldoor_t>());
					(*door).sector = sectors.wrapping_add((*door).sector as usize);
					(*(*door).sector).specialdata = door.cast();
					(*door).thinker.function.acp1 = Some(T_VerticalDoor);
					P_AddThinker(&mut (*door).thinker);
				}

				specials_e::tc_floor => {
					PADSAVEP();
					let floor = Z_Malloc(size_of::<floormove_t>(), PU_LEVEL, null_mut())
						as *mut floormove_t;
					libc::memcpy(floor.cast(), save_p.cast(), size_of::<floormove_t>());
					save_p = save_p.wrapping_add(size_of::<floormove_t>());
					(*floor).sector = sectors.wrapping_add((*floor).sector as usize);
					(*(*floor).sector).specialdata = floor.cast();
					(*floor).thinker.function.acp1 = Some(T_MoveFloor);
					P_AddThinker(&mut (*floor).thinker);
				}

				specials_e::tc_plat => {
					PADSAVEP();
					let plat = Z_Malloc(size_of::<plat_t>(), PU_LEVEL, null_mut()) as *mut plat_t;
					libc::memcpy(plat.cast(), save_p.cast(), size_of::<plat_t>());
					save_p = save_p.wrapping_add(size_of::<plat_t>());
					(*plat).sector = sectors.wrapping_add((*plat).sector as usize);
					(*(*plat).sector).specialdata = plat.cast();

					if (*plat).thinker.function.acp1.is_some() {
						(*plat).thinker.function.acp1 = Some(T_PlatRaise);
					}

					P_AddThinker(&mut (*plat).thinker);
					P_AddActivePlat(plat);
				}

				specials_e::tc_flash => {
					PADSAVEP();
					let flash = Z_Malloc(size_of::<lightflash_t>(), PU_LEVEL, null_mut())
						as *mut lightflash_t;
					libc::memcpy(flash.cast(), save_p.cast(), size_of::<lightflash_t>());
					save_p = save_p.wrapping_add(size_of::<lightflash_t>());
					(*flash).sector = sectors.wrapping_add((*flash).sector as usize);
					(*flash).thinker.function.acp1 = Some(T_LightFlash_action);
					P_AddThinker(&mut (*flash).thinker);
				}

				specials_e::tc_strobe => {
					PADSAVEP();
					let strobe =
						Z_Malloc(size_of::<strobe_t>(), PU_LEVEL, null_mut()) as *mut strobe_t;
					libc::memcpy(strobe.cast(), save_p.cast(), size_of::<strobe_t>());
					save_p = save_p.wrapping_add(size_of::<strobe_t>());
					(*strobe).sector = sectors.wrapping_add((*strobe).sector as usize);
					(*strobe).thinker.function.acp1 = Some(T_StrobeFlash_action);
					P_AddThinker(&mut (*strobe).thinker);
				}

				specials_e::tc_glow => {
					PADSAVEP();
					let glow = Z_Malloc(size_of::<glow_t>(), PU_LEVEL, null_mut()) as *mut glow_t;
					libc::memcpy(glow.cast(), save_p.cast(), size_of::<glow_t>());
					save_p = save_p.wrapping_add(size_of::<glow_t>());
					(*glow).sector = sectors.wrapping_add((*glow).sector as usize);
					(*glow).thinker.function.acp1 = Some(T_Glow_action);
					P_AddThinker(&mut (*glow).thinker);
				}
			}
		}
	}
}
