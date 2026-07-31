#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

// DESCRIPTION:
//	Archiving: SaveGame I/O.

use std::{
	ffi::c_int,
	mem::MaybeUninit,
	ptr::{self, null_mut},
};

use crate::{
	d_player::{player_t, playerstate_t},
	d_think::think_t,
	d_ticcmd::ticcmd_t,
	doomdef::{MAXPLAYERS, NUMAMMO, NUMCARDS, NUMPOWERS, NUMWEAPONS, weapontype_t},
	i_system::I_Error,
	info::{mobjinfo, states},
	m_fixed::{FRACBITS, fixed_t},
	p_ceiling::{P_AddActiveCeiling, activeceilings},
	p_local::thinkercap,
	p_maputl::P_SetThingPosition,
	p_mobj::{P_RemoveMobj, mobj_t},
	p_plats::P_AddActivePlat,
	p_pspr::{pspdef_t, psprnum_t},
	p_setup::{lines, numlines, numsectors, sectors, sides},
	p_spec::{
		MAXCEILINGS, ceiling_t, floormove_t, glow_t, lightflash_t, plat_t, strobe_t, vldoor_t,
	},
	p_tick::{P_AddThinker, P_InitThinkers},
	z_zone::{PU_LEVEL, Z_Free, Z_Malloc},
};

// Pads save_p to a 4-byte boundary
//  so that the load/save works on SGI&Gecko.
fn PADSAVEP(p: &mut *mut u8) {
	*p = (*p).wrapping_add((*p).align_offset(4));
}

// P_ArchivePlayers
#[allow(static_mut_refs)]
pub(crate) fn P_ArchivePlayers(
	p: &mut *mut u8,
	playeringame: [bool; MAXPLAYERS],
	players: &[player_t; MAXPLAYERS],
) {
	unsafe {
		for i in 0..MAXPLAYERS {
			if !playeringame[i] {
				continue;
			}

			PADSAVEP(p);

			let dest = (*p).cast::<player_t_saveg>();
			let player_save = player_t_saveg::from(players[i]);
			libc::memcpy(dest.cast(), (&raw const player_save).cast(), size_of::<player_t_saveg>());
			let dest = &mut *dest;
			*p = (*p).wrapping_add(size_of::<player_t_saveg>());
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
#[allow(static_mut_refs)]
pub(crate) fn P_UnArchivePlayers(
	p: &mut *mut u8,
	playeringame: [bool; MAXPLAYERS],
	players: &mut [player_t; MAXPLAYERS],
) {
	unsafe {
		for i in 0..MAXPLAYERS {
			if !playeringame[i] {
				continue;
			}

			PADSAVEP(p);

			let mut player = MaybeUninit::<player_t_saveg>::uninit();
			libc::memcpy(player.as_mut_ptr().cast(), (*p).cast(), size_of::<player_t_saveg>());

			*p = (*p).wrapping_add(size_of::<player_t_saveg>());

			players[i] = player.assume_init().into();

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
pub(crate) fn P_ArchiveWorld(p: &mut *mut u8) {
	unsafe {
		let mut put = (*p).cast::<i16>();

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

		*p = put.cast();
	}
}

// P_UnArchiveWorld
pub(crate) fn P_UnArchiveWorld(p: &mut *mut u8) {
	unsafe {
		let mut get = (*p).cast::<i16>();

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

		*p = get.cast();
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
pub(crate) fn P_ArchiveThinkers(p: &mut *mut u8, players: &[player_t]) {
	unsafe {
		// save off the current thinkers
		let mut th = thinkercap.next;
		while !ptr::eq(th, &raw const thinkercap) {
			if (*th).function.is_mobj() {
				**p = u8::from(thinkerclass_t::tc_mobj);
				*p = (*p).wrapping_add(1);
				PADSAVEP(p);
				let mobj = (*p).cast::<mobj_t>();
				libc::memcpy(mobj.cast(), th.cast(), size_of::<mobj_t>());
				*p = (*p).wrapping_add(size_of::<mobj_t>());
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
		**p = u8::from(thinkerclass_t::tc_end);
		*p = (*p).wrapping_add(1);
	}
}

// P_UnArchiveThinkers
pub(crate) fn P_UnArchiveThinkers(p: &mut *mut u8, players: &mut [player_t; MAXPLAYERS]) {
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
			let tclass = **p;
			*p = (*p).wrapping_add(1);
			match tclass {
				0 => return, // end of list

				1 => {
					PADSAVEP(p);
					let mobj = Z_Malloc(size_of::<mobj_t>(), PU_LEVEL, null_mut()).cast::<mobj_t>();
					libc::memcpy(mobj.cast(), (*p).cast(), size_of::<mobj_t>());
					*p = (*p).wrapping_add(size_of::<mobj_t>());
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

				_ => I_Error!(c"Unknown tclass %i in savegame".as_ptr(), c_int::from(tclass)),
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
pub(crate) fn P_ArchiveSpecials(p: &mut *mut u8) {
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
					**p = u8::from(specials_e::tc_ceiling);
					*p = (*p).wrapping_add(1);
					PADSAVEP(p);
					let ceiling = (*p).cast::<ceiling_t>();
					libc::memcpy(ceiling.cast(), th.cast(), size_of::<ceiling_t>());
					*p = (*p).wrapping_add(size_of::<ceiling_t>());
					(*ceiling).sector = ptr::without_provenance_mut(
						(*ceiling).sector.offset_from(sectors).try_into().unwrap(),
					);
				}
				th = (*th).next;
				continue;
			}

			if (*th).function == think_t::T_MoveCeiling {
				**p = u8::from(specials_e::tc_ceiling);
				*p = (*p).wrapping_add(1);
				PADSAVEP(p);
				let ceiling = (*p).cast::<ceiling_t>();
				libc::memcpy(ceiling.cast(), th.cast(), size_of::<ceiling_t>());
				*p = (*p).wrapping_add(size_of::<ceiling_t>());
				(*ceiling).sector = ptr::without_provenance_mut(
					(*ceiling).sector.offset_from(sectors).try_into().unwrap(),
				);
				th = (*th).next;
				continue;
			}

			if (*th).function == think_t::T_VerticalDoor {
				**p = u8::from(specials_e::tc_door);
				*p = (*p).wrapping_add(1);
				PADSAVEP(p);
				let door = (*p).cast::<vldoor_t>();
				libc::memcpy(door.cast(), th.cast(), size_of::<vldoor_t>());
				*p = (*p).wrapping_add(size_of::<vldoor_t>());
				(*door).sector = ptr::without_provenance_mut(
					(*door).sector.offset_from(sectors).try_into().unwrap(),
				);
				th = (*th).next;
				continue;
			}

			if (*th).function == think_t::T_MoveFloor {
				**p = u8::from(specials_e::tc_floor);
				*p = (*p).wrapping_add(1);
				PADSAVEP(p);
				let floor = (*p).cast::<floormove_t>();
				libc::memcpy(floor.cast(), th.cast(), size_of::<floormove_t>());
				*p = (*p).wrapping_add(size_of::<floormove_t>());
				(*floor).sector = ptr::without_provenance_mut(
					(*floor).sector.offset_from(sectors).try_into().unwrap(),
				);
				th = (*th).next;
				continue;
			}

			if (*th).function == think_t::T_PlatRaise {
				**p = u8::from(specials_e::tc_plat);
				*p = (*p).wrapping_add(1);
				PADSAVEP(p);
				let plat = (*p).cast::<plat_t>();
				libc::memcpy(plat.cast(), th.cast(), size_of::<plat_t>());
				*p = (*p).wrapping_add(size_of::<plat_t>());
				(*plat).sector = ptr::without_provenance_mut(
					(*plat).sector.offset_from(sectors).try_into().unwrap(),
				);
				th = (*th).next;
				continue;
			}

			if (*th).function == think_t::T_LightFlash {
				**p = u8::from(specials_e::tc_flash);
				*p = (*p).wrapping_add(1);
				PADSAVEP(p);
				let flash = (*p).cast::<lightflash_t>();
				libc::memcpy(flash.cast(), th.cast(), size_of::<lightflash_t>());
				*p = (*p).wrapping_add(size_of::<lightflash_t>());
				(*flash).sector = ptr::without_provenance_mut(
					(*flash).sector.offset_from(sectors).try_into().unwrap(),
				);
				th = (*th).next;
				continue;
			}

			if (*th).function == think_t::T_StrobeFlash {
				**p = u8::from(specials_e::tc_strobe);
				*p = (*p).wrapping_add(1);
				PADSAVEP(p);
				let strobe = (*p).cast::<strobe_t>();
				libc::memcpy(strobe.cast(), th.cast(), size_of::<strobe_t>());
				*p = (*p).wrapping_add(size_of::<strobe_t>());
				(*strobe).sector = ptr::without_provenance_mut(
					(*strobe).sector.offset_from(sectors).try_into().unwrap(),
				);
				th = (*th).next;
				continue;
			}

			if (*th).function == think_t::T_Glow {
				**p = u8::from(specials_e::tc_glow);
				*p = (*p).wrapping_add(1);
				PADSAVEP(p);
				let glow = (*p).cast::<glow_t>();
				libc::memcpy(glow.cast(), th.cast(), size_of::<glow_t>());
				*p = (*p).wrapping_add(size_of::<glow_t>());
				(*glow).sector = ptr::without_provenance_mut(
					(*glow).sector.offset_from(sectors).try_into().unwrap(),
				);
				th = (*th).next;
				continue;
			}

			th = (*th).next;
		}

		// add a terminating marker
		**p = u8::from(specials_e::tc_endspecials);
		*p = (*p).wrapping_add(1);
	}
}

// P_UnArchiveSpecials
pub(crate) fn P_UnArchiveSpecials(p: &mut *mut u8) {
	unsafe {
		// read in saved thinkers
		loop {
			let tclass = specials_e::from(**p);
			*p = (*p).wrapping_add(1);
			match tclass {
				specials_e::tc_endspecials => return, // end of list

				specials_e::tc_ceiling => {
					PADSAVEP(p);
					let ceiling =
						Z_Malloc(size_of::<ceiling_t>(), PU_LEVEL, null_mut()).cast::<ceiling_t>();
					libc::memcpy(ceiling.cast(), (*p).cast(), size_of::<ceiling_t>());
					*p = (*p).wrapping_add(size_of::<ceiling_t>());
					(*ceiling).sector = sectors.wrapping_add((*ceiling).sector.addr());
					(*(*ceiling).sector).specialdata = ceiling.cast();

					if (*ceiling).thinker.function.as_acp1().is_some() {
						(*ceiling).thinker.function = think_t::T_MoveCeiling;
					}

					P_AddThinker(&mut (*ceiling).thinker);
					P_AddActiveCeiling(ceiling);
				}

				specials_e::tc_door => {
					PADSAVEP(p);
					let door =
						Z_Malloc(size_of::<vldoor_t>(), PU_LEVEL, null_mut()).cast::<vldoor_t>();
					libc::memcpy(door.cast(), (*p).cast(), size_of::<vldoor_t>());
					*p = (*p).wrapping_add(size_of::<vldoor_t>());
					(*door).sector = sectors.wrapping_add((*door).sector.addr());
					(*(*door).sector).specialdata = door.cast();
					(*door).thinker.function = think_t::T_VerticalDoor;
					P_AddThinker(&mut (*door).thinker);
				}

				specials_e::tc_floor => {
					PADSAVEP(p);
					let floor = Z_Malloc(size_of::<floormove_t>(), PU_LEVEL, null_mut())
						.cast::<floormove_t>();
					libc::memcpy(floor.cast(), (*p).cast(), size_of::<floormove_t>());
					*p = (*p).wrapping_add(size_of::<floormove_t>());
					(*floor).sector = sectors.wrapping_add((*floor).sector.addr());
					(*(*floor).sector).specialdata = floor.cast();
					(*floor).thinker.function = think_t::T_MoveFloor;
					P_AddThinker(&mut (*floor).thinker);
				}

				specials_e::tc_plat => {
					PADSAVEP(p);
					let plat = Z_Malloc(size_of::<plat_t>(), PU_LEVEL, null_mut()).cast::<plat_t>();
					libc::memcpy(plat.cast(), (*p).cast(), size_of::<plat_t>());
					*p = (*p).wrapping_add(size_of::<plat_t>());
					(*plat).sector = sectors.wrapping_add((*plat).sector.addr());
					(*(*plat).sector).specialdata = plat.cast();

					if (*plat).thinker.function.as_acp1().is_some() {
						(*plat).thinker.function = think_t::T_PlatRaise;
					}

					P_AddThinker(&mut (*plat).thinker);
					P_AddActivePlat(plat);
				}

				specials_e::tc_flash => {
					PADSAVEP(p);
					let flash = Z_Malloc(size_of::<lightflash_t>(), PU_LEVEL, null_mut())
						.cast::<lightflash_t>();
					libc::memcpy(flash.cast(), (*p).cast(), size_of::<lightflash_t>());
					*p = (*p).wrapping_add(size_of::<lightflash_t>());
					(*flash).sector = sectors.wrapping_add((*flash).sector.addr());
					(*flash).thinker.function = think_t::T_LightFlash;
					P_AddThinker(&mut (*flash).thinker);
				}

				specials_e::tc_strobe => {
					PADSAVEP(p);
					let strobe =
						Z_Malloc(size_of::<strobe_t>(), PU_LEVEL, null_mut()).cast::<strobe_t>();
					libc::memcpy(strobe.cast(), (*p).cast(), size_of::<strobe_t>());
					*p = (*p).wrapping_add(size_of::<strobe_t>());
					(*strobe).sector = sectors.wrapping_add((*strobe).sector.addr());
					(*strobe).thinker.function = think_t::T_StrobeFlash;
					P_AddThinker(&mut (*strobe).thinker);
				}

				specials_e::tc_glow => {
					PADSAVEP(p);
					let glow = Z_Malloc(size_of::<glow_t>(), PU_LEVEL, null_mut()).cast::<glow_t>();
					libc::memcpy(glow.cast(), (*p).cast(), size_of::<glow_t>());
					*p = (*p).wrapping_add(size_of::<glow_t>());
					(*glow).sector = sectors.wrapping_add((*glow).sector.addr());
					(*glow).thinker.function = think_t::T_Glow;
					P_AddThinker(&mut (*glow).thinker);
				}
			}
		}
	}
}

// Extended player object info: player_t
#[repr(C)]
struct player_t_saveg {
	_mo_pad: u32,
	playerstate: playerstate_t,
	cmd: ticcmd_t,
	viewz: fixed_t,
	viewheight: fixed_t,
	deltaviewheight: fixed_t,
	bob: fixed_t,
	health: i32,
	armorpoints: i32,
	armortype: i32,
	powers: [usize; NUMPOWERS],
	cards: [i32; NUMCARDS],
	backpack: i32,
	frags: [i32; MAXPLAYERS],
	readyweapon: weapontype_t,
	pendingweapon: u32,
	weaponowned: [i32; NUMWEAPONS],
	ammo: [usize; NUMAMMO],
	maxammo: [usize; NUMAMMO],
	attackdown: i32,
	usedown: i32,
	cheats: usize,
	refire: i32,
	killcount: i32,
	itemcount: i32,
	secretcount: i32,
	_message_pad: u32,
	damagecount: i32,
	bonuscount: usize,
	_attacker_pad: u32,
	extralight: i32,
	fixedcolormap: usize,
	_colormap_pad: i32,
	psprites: [pspdef_t; psprnum_t::NUMPSPRITES.to_usize()],
	didsecret: i32,
}

impl From<player_t> for player_t_saveg {
	fn from(value: player_t) -> Self {
		Self {
			// will be set when unarc thinker
			_mo_pad: 0,
			_message_pad: 0,
			_attacker_pad: 0,
			_colormap_pad: 0,
			pendingweapon: match value.pendingweapon {
				Some(weapontype_t::wp_fist) => 0,
				Some(weapontype_t::wp_pistol) => 1,
				Some(weapontype_t::wp_shotgun) => 2,
				Some(weapontype_t::wp_chaingun) => 3,
				Some(weapontype_t::wp_missile) => 4,
				Some(weapontype_t::wp_plasma) => 5,
				Some(weapontype_t::wp_bfg) => 6,
				Some(weapontype_t::wp_chainsaw) => 7,
				Some(weapontype_t::wp_supershotgun) => 8,
				None => 10, // wp_nochange is 10
			},
			weaponowned: value.weaponowned.map(i32::from),
			attackdown: i32::from(value.attackdown),
			usedown: i32::from(value.usedown),
			// the rest are copied over
			playerstate: value.playerstate,
			cmd: value.cmd,
			viewz: value.viewz,
			viewheight: value.viewheight,
			deltaviewheight: value.deltaviewheight,
			bob: value.bob,
			health: value.health,
			armorpoints: value.armorpoints,
			armortype: value.armortype,
			powers: value.powers,
			cards: value.cards,
			backpack: value.backpack,
			frags: value.frags,
			readyweapon: value.readyweapon,
			ammo: value.ammo,
			maxammo: value.maxammo,
			cheats: value.cheats,
			refire: value.refire,
			killcount: value.killcount,
			itemcount: value.itemcount,
			secretcount: value.secretcount,
			damagecount: value.damagecount,
			bonuscount: value.bonuscount,
			extralight: value.extralight,
			fixedcolormap: value.fixedcolormap,
			psprites: value.psprites,
			didsecret: value.didsecret,
		}
	}
}

impl From<player_t_saveg> for player_t {
	fn from(value: player_t_saveg) -> Self {
		Self {
			// will be set when unarc thinker
			mo: null_mut(),
			message: null_mut(),
			attacker: null_mut(),
			pendingweapon: match value.pendingweapon {
				0 => Some(weapontype_t::wp_fist),
				1 => Some(weapontype_t::wp_pistol),
				2 => Some(weapontype_t::wp_shotgun),
				3 => Some(weapontype_t::wp_chaingun),
				4 => Some(weapontype_t::wp_missile),
				5 => Some(weapontype_t::wp_plasma),
				6 => Some(weapontype_t::wp_bfg),
				7 => Some(weapontype_t::wp_chainsaw),
				8 => Some(weapontype_t::wp_supershotgun),
				_ => None,
			},
			weaponowned: value.weaponowned.map(|wo| wo != 0),
			// the rest are copied over
			playerstate: value.playerstate,
			cmd: value.cmd,
			viewz: value.viewz,
			viewheight: value.viewheight,
			deltaviewheight: value.deltaviewheight,
			bob: value.bob,
			health: value.health,
			armorpoints: value.armorpoints,
			armortype: value.armortype,
			powers: value.powers,
			cards: value.cards,
			backpack: value.backpack,
			frags: value.frags,
			readyweapon: value.readyweapon,
			ammo: value.ammo,
			maxammo: value.maxammo,
			attackdown: value.attackdown != 0,
			usedown: value.usedown != 0,
			cheats: value.cheats,
			refire: value.refire,
			killcount: value.killcount,
			itemcount: value.itemcount,
			secretcount: value.secretcount,
			damagecount: value.damagecount,
			bonuscount: value.bonuscount,
			extralight: value.extralight,
			fixedcolormap: value.fixedcolormap,
			psprites: value.psprites,
			didsecret: value.didsecret,
		}
	}
}
