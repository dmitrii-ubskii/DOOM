// The player data structure depends on a number
// of other structs: items (internal inventory),
// animation states (closely tied to the sprites
// used to represent them, unfortunately).

// In addition, the player is just a special
// case of the generic moving object/actor.

// Finally, for odd reasons, the player input
// is buffered within the player data struct,
// as commands per game tick.

#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::{ffi::c_char, ptr::null_mut};

use crate::{
	d_ticcmd::ticcmd_t,
	doomdef::{MAXPLAYERS, NUMAMMO, NUMCARDS, NUMPOWERS, NUMWEAPONS, weapontype_t},
	m_fixed::fixed_t,
	p_mobj::mobj_t,
	p_pspr::{pspdef_t, psprnum_t},
};

// Player states.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum playerstate_t {
	// Playing or camping.
	PST_LIVE,
	// Dead on the ground, view follows killer.
	PST_DEAD,
	// Ready to restart/respawn???
	PST_REBORN,
}

// Player internal flags, for cheats and debug.
// No clipping, walk through barriers.
pub(crate) const CF_NOCLIP: usize = 1;
// No damage, no health loss.
pub(crate) const CF_GODMODE: usize = 2;
// Not really a cheat, just a debug aid.
pub(crate) const CF_NOMOMENTUM: usize = 4;

// Extended player object info: player_t
#[derive(Clone, Copy, Debug)]
pub(crate) struct player_t {
	pub(crate) mo: *mut mobj_t,
	pub(crate) playerstate: playerstate_t,
	pub(crate) cmd: ticcmd_t,

	// Determine POV,
	//  including viewpoint bobbing during movement.
	// Focal origin above r.z
	pub(crate) viewz: fixed_t,
	// Base height above floor for viewz.
	pub(crate) viewheight: fixed_t,
	// Bob/squat speed.
	pub(crate) deltaviewheight: fixed_t,
	// bounded/scaled total momentum.
	pub(crate) bob: fixed_t,

	// This is only used between levels,
	// mo->health is used during levels.
	pub(crate) health: i32,
	pub(crate) armorpoints: i32,
	// Armor type is 0-2.
	pub(crate) armortype: i32,

	// Power ups. invinc and invis are tic counters.
	pub(crate) powers: [usize; NUMPOWERS],
	pub(crate) cards: [i32; NUMCARDS],
	pub(crate) backpack: i32,

	// Frags, kills of other players.
	pub(crate) frags: [i32; MAXPLAYERS],
	pub(crate) readyweapon: weapontype_t,

	// Is wp_nochange if not changing.
	pub(crate) pendingweapon: Option<weapontype_t>,

	pub(crate) weaponowned: [bool; NUMWEAPONS],
	pub(crate) ammo: [usize; NUMAMMO],
	pub(crate) maxammo: [usize; NUMAMMO],

	// True if button down last tic.
	pub(crate) attackdown: i32,
	pub(crate) usedown: i32,

	// Bit flags, for cheats and debug.
	// See cheat_t, above.
	pub(crate) cheats: usize,

	// Refired shots are less accurate.
	pub(crate) refire: i32,

	// For intermission stats.
	pub(crate) killcount: i32,
	pub(crate) itemcount: i32,
	pub(crate) secretcount: i32,

	// Hint messages.
	pub(crate) message: *const c_char,

	// For screen flashing (red or bright).
	pub(crate) damagecount: i32,
	pub(crate) bonuscount: usize,

	// Who did damage (NULL for floors/ceilings).
	pub(crate) attacker: *mut mobj_t,

	// So gun flashes light up areas.
	pub(crate) extralight: i32,

	// Current PLAYPAL, ???
	//  can be set to REDCOLORMAP for pain, etc.
	pub(crate) fixedcolormap: usize,

	// Overlay view sprites (gun, etc).
	pub(crate) psprites: [pspdef_t; psprnum_t::NUMPSPRITES.to_usize()],

	// True if secret level has been done.
	pub(crate) didsecret: i32,
}

impl player_t {
	pub(crate) const fn new() -> Self {
		Self {
			mo: null_mut(),
			playerstate: playerstate_t::PST_LIVE,
			cmd: ticcmd_t {
				forwardmove: 0,
				sidemove: 0,
				angleturn: 0,
				consistancy: 0,
				chatchar: 0,
				buttons: 0,
			},
			viewz: 0,
			viewheight: 0,
			deltaviewheight: 0,
			bob: 0,
			health: 0,
			armorpoints: 0,
			armortype: 0,
			powers: [0; 6],
			cards: [0; 6],
			backpack: 0,
			frags: [0; 4],
			readyweapon: weapontype_t::wp_pistol,
			pendingweapon: Some(weapontype_t::wp_fist),
			weaponowned: [false; 9],
			ammo: [0; 4],
			maxammo: [0; 4],
			attackdown: 0,
			usedown: 0,
			cheats: 0,
			refire: 0,
			killcount: 0,
			itemcount: 0,
			secretcount: 0,
			message: null_mut(),
			damagecount: 0,
			bonuscount: 0,
			attacker: null_mut(),
			extralight: 0,
			fixedcolormap: 0,
			psprites: [pspdef_t { state: null_mut(), tics: 0, sx: 0, sy: 0 }; 2],
			didsecret: 0,
		}
	}

	pub(crate) fn mo(&self) -> &mobj_t {
		unsafe { &*self.mo }
	}

	pub(crate) fn mo_mut(&mut self) -> &mut mobj_t {
		unsafe { &mut *self.mo }
	}
}

impl Default for player_t {
	fn default() -> Self {
		Self::new()
	}
}

// INTERMISSION
// Structure passed e.g. to WI_Start(wb)
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct wbplayerstruct_t {
	pub(crate) in_: i32, // whether the player is in game

	// Player stats, kills, collected items etc.
	pub(crate) skills: i32,
	pub(crate) sitems: i32,
	pub(crate) ssecret: i32,
	pub(crate) stime: usize,
	pub(crate) frags: [i32; 4],
	pub(crate) score: i32, // current score on entry, modified on return
}

#[repr(C)]
pub(crate) struct wbstartstruct_t {
	pub(crate) epsd: usize, // episode # (0-2)

	// if true, splash the secret level
	pub(crate) didsecret: i32,

	// previous and next levels, origin 0
	pub(crate) last: usize,
	pub(crate) next: usize,

	pub(crate) maxkills: i32,
	pub(crate) maxitems: i32,
	pub(crate) maxsecret: i32,
	pub(crate) maxfrags: i32,

	// the par time
	pub(crate) partime: usize,

	// index of this player in game
	pub(crate) pnum: usize,

	pub(crate) plyr: [wbplayerstruct_t; MAXPLAYERS],
}
