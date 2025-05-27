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

use std::ffi::c_char;

use crate::{
	d_ticcmd::ticcmd_t,
	doomdef::{MAXPLAYERS, ammotype_t, card_t, powertype_t, weapontype_t},
	p_mobj::mobj_t,
	p_pspr::{pspdef_t, psprnum_t},
};

type fixed_t = i32;

// Player states.
#[repr(C)]
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
	pub powers: [i32; powertype_t::NUMPOWERS as usize],
	pub cards: [bool; card_t::NUMCARDS as usize],
	pub backpack: bool,

	// Frags, kills of other players.
	pub frags: [i32; MAXPLAYERS],
	pub readyweapon: weapontype_t,

	// Is wp_nochange if not changing.
	pub pendingweapon: weapontype_t,

	pub weaponowned: [bool; weapontype_t::NUMWEAPONS as usize],
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
	pub message: *mut c_char,

	// For screen flashing (red or bright).
	pub damagecount: i32,
	pub bonuscount: i32,

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
	pub didsecret: bool,
}

// INTERMISSION
// Structure passed e.g. to WI_Start(wb)
#[repr(C)]
pub struct wbplayerstruct_t {
	in_: bool, // whether the player is in game

	// Player stats, kills, collected items etc.
	skills: i32,
	sitems: i32,
	ssecret: i32,
	stime: i32,
	frags: [i32; 4],
	score: i32, // current score on entry, modified on return
}

#[repr(C)]
pub struct wbstartstruct_t {
	epsd: i32, // episode # (0-2)

	// if true, splash the secret level
	didsecret: bool,

	// previous and next levels, origin 0
	last: i32,
	next: i32,

	maxkills: i32,
	maxitems: i32,
	maxsecret: i32,
	maxfrags: i32,

	// the par time
	partime: i32,

	// index of this player in game
	pnum: i32,

	plyr: [wbplayerstruct_t; MAXPLAYERS],
}
