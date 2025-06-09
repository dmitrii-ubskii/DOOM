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
	doomdef::{MAXPLAYERS, ammotype_t, card_t, powertype_t, weapontype_t},
	m_fixed::fixed_t,
	p_mobj::mobj_t,
	p_pspr::{pspdef_t, psprnum_t},
};

// Player states.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum playerstate_t {
	// Playing or camping.
	PST_LIVE,
	// Dead on the ground, view follows killer.
	PST_DEAD,
	// Ready to restart/respawn???
	PST_REBORN,
}

// Player internal flags, for cheats and debug.
#[repr(C)]
pub enum cheat_t {
	// No clipping, walk through barriers.
	CF_NOCLIP = 1,
	// No damage, no health loss.
	CF_GODMODE = 2,
	// Not really a cheat, just a debug aid.
	CF_NOMOMENTUM = 4,
}

// Extended player object info: player_t
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct player_t {
	pub mo: *mut mobj_t,
	pub playerstate: playerstate_t,
	pub cmd: ticcmd_t,

	// Determine POV,
	//  including viewpoint bobbing during movement.
	// Focal origin above r.z
	pub viewz: fixed_t,
	// Base height above floor for viewz.
	pub viewheight: fixed_t,
	// Bob/squat speed.
	pub deltaviewheight: fixed_t,
	// bounded/scaled total momentum.
	pub bob: fixed_t,

	// This is only used between levels,
	// mo->health is used during levels.
	pub health: i32,
	pub armorpoints: i32,
	// Armor type is 0-2.
	pub armortype: i32,

	// Power ups. invinc and invis are tic counters.
	pub powers: [usize; powertype_t::NUMPOWERS as usize],
	pub cards: [i32; card_t::NUMCARDS as usize],
	pub backpack: i32,

	// Frags, kills of other players.
	pub frags: [i32; MAXPLAYERS],
	pub readyweapon: weapontype_t,

	// Is wp_nochange if not changing.
	pub pendingweapon: weapontype_t,

	pub weaponowned: [i32; weapontype_t::NUMWEAPONS as usize],
	pub ammo: [i32; ammotype_t::NUMAMMO as usize],
	pub maxammo: [i32; ammotype_t::NUMAMMO as usize],

	// True if button down last tic.
	pub attackdown: i32,
	pub usedown: i32,

	// Bit flags, for cheats and debug.
	// See cheat_t, above.
	pub cheats: i32,

	// Refired shots are less accurate.
	pub refire: i32,

	// For intermission stats.
	pub killcount: i32,
	pub itemcount: i32,
	pub secretcount: i32,

	// Hint messages.
	pub message: *const c_char,

	// For screen flashing (red or bright).
	pub damagecount: i32,
	pub bonuscount: usize,

	// Who did damage (NULL for floors/ceilings).
	pub attacker: *mut mobj_t,

	// So gun flashes light up areas.
	pub extralight: i32,

	// Current PLAYPAL, ???
	//  can be set to REDCOLORMAP for pain, etc.
	pub fixedcolormap: i32,

	// Player skin colorshift,
	//  0-3 for which color to draw player.
	pub colormap: i32,

	// Overlay view sprites (gun, etc).
	pub psprites: [pspdef_t; psprnum_t::NUMPSPRITES as usize],

	// True if secret level has been done.
	pub didsecret: i32,
}

impl player_t {
	pub const fn new() -> Self {
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
			pendingweapon: weapontype_t::wp_fist,
			weaponowned: [0; 9],
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
			colormap: 0,
			psprites: [pspdef_t { state: null_mut(), tics: 0, sx: 0, sy: 0 }; 2],
			didsecret: 0,
		}
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
pub struct wbplayerstruct_t {
	pub in_: i32, // whether the player is in game

	// Player stats, kills, collected items etc.
	pub skills: i32,
	pub sitems: i32,
	pub ssecret: i32,
	pub stime: usize,
	pub frags: [i32; 4],
	pub score: i32, // current score on entry, modified on return
}

#[repr(C)]
pub struct wbstartstruct_t {
	pub epsd: usize, // episode # (0-2)

	// if true, splash the secret level
	pub didsecret: i32,

	// previous and next levels, origin 0
	pub last: usize,
	pub next: usize,

	pub maxkills: i32,
	pub maxitems: i32,
	pub maxsecret: i32,
	pub maxfrags: i32,

	// the par time
	pub partime: usize,

	// index of this player in game
	pub pnum: usize,

	pub plyr: [wbplayerstruct_t; MAXPLAYERS],
}
