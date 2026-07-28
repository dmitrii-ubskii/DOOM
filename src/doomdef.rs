// Global parameters/defines.

#![allow(non_camel_case_types, non_upper_case_globals, clippy::upper_case_acronyms)]

use std::fmt;

// DOOM version
pub(crate) const VERSION: i32 = 109;

// Game mode handling - identify IWAD version
//  to handle IWAD dependend animations etc.
#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub(crate) enum GameMode_t {
	shareware,  // DOOM 1 shareware, E1, M9
	registered, // DOOM 1 registered, E3, M27
	commercial, // DOOM 2 retail, E1 M34
	// DOOM 2 german edition not handled
	retail,       // DOOM 1 retail, E4, M36
	indetermined, // Well, no IWAD found.
}

// Mission packs - might be useful for TC stuff?
#[expect(unused, reason = "doom2 should be handled later; none is weird")]
#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub(crate) enum GameMission_t {
	doom,      // DOOM 1
	doom2,     // DOOM 2
	pack_tnt,  // TNT mission pack
	pack_plut, // Plutonia pack
	none,
}

// Identify language to use, software localization.
#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub(crate) enum Language_t {
	english,
	french,
}

// For resize of screen, at start of game.
// It will not work dynamically, see visplanes.
// pub(crate) const BASE_WIDTH: usize = 320;

// It is educational but futile to change this
//  scaling e.g. to 2. Drawing of status bar,
//  menues etc. is tied to the scale implied
//  by the graphics.
pub(crate) const SCREEN_MUL: usize = 1;
// pub(crate) const INV_ASPECT_RATIO: f64 = 0.625; // 0.75, ideally

// Defines suck. C sucks.
// C++ might sucks for OOP, but it sure is a better C.
// So there.
pub(crate) const SCREENWIDTH: usize = 320;
//SCREEN_MUL*BASE_WIDTH //320
pub(crate) const SCREENHEIGHT: usize = 200;
//(int)(SCREEN_MUL*BASE_WIDTH*INV_ASPECT_RATIO) //200

// The maximum number of players, multiplayer/networking.
pub(crate) const MAXPLAYERS: usize = 4;

// State updates, number of tics / second.
pub(crate) const TICRATE: usize = 35;

// The current state of the game: whether we are
// playing, gazing at the intermission screen,
// the game final animation, or a demo.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum gamestate_t {
	GS_LEVEL,
	GS_INTERMISSION,
	GS_FINALE,
	GS_DEMOSCREEN,

	None = -1,
}

// Difficulty/skill settings/filters.

// Skill flags.
// pub(crate) const MTF_EASY: u8 = 1;
// pub(crate) const MTF_NORMAL: u8 = 2;
// pub(crate) const MTF_HARD: u8 = 4;

// Deaf monsters/do not react to sound.
pub(crate) const MTF_AMBUSH: u8 = 8;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub(crate) enum skill_t {
	sk_baby,
	sk_easy,
	sk_medium,
	sk_hard,
	sk_nightmare,
}

impl From<u8> for skill_t {
	fn from(value: u8) -> Self {
		match value {
			0 => Self::sk_baby,
			1 => Self::sk_easy,
			2 => Self::sk_medium,
			3 => Self::sk_hard,
			4 => Self::sk_nightmare,
			_ => panic!("skill_t out of bounds"),
		}
	}
}

impl From<skill_t> for u8 {
	fn from(value: skill_t) -> Self {
		match value {
			skill_t::sk_baby => 0,
			skill_t::sk_easy => 1,
			skill_t::sk_medium => 2,
			skill_t::sk_hard => 3,
			skill_t::sk_nightmare => 4,
		}
	}
}

impl From<skill_t> for usize {
	fn from(value: skill_t) -> Self {
		match value {
			skill_t::sk_baby => 0,
			skill_t::sk_easy => 1,
			skill_t::sk_medium => 2,
			skill_t::sk_hard => 3,
			skill_t::sk_nightmare => 4,
		}
	}
}

impl fmt::Display for skill_t {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(self, f)
	}
}

// Key cards.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum card_t {
	it_bluecard,
	it_yellowcard,
	it_redcard,
	it_blueskull,
	it_yellowskull,
	it_redskull,

	NUMCARDS,
}

impl card_t {
	pub(crate) const fn to_usize(self) -> usize {
		match self {
			card_t::it_bluecard => 0,
			card_t::it_yellowcard => 1,
			card_t::it_redcard => 2,
			card_t::it_blueskull => 3,
			card_t::it_yellowskull => 4,
			card_t::it_redskull => 5,
			card_t::NUMCARDS => 6,
		}
	}
}

impl From<card_t> for usize {
	fn from(value: card_t) -> Self {
		value.to_usize()
	}
}

// The defined weapons,
//  including a marker indicating
//  user has not changed weapon.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum weapontype_t {
	wp_fist,
	wp_pistol,
	wp_shotgun,
	wp_chaingun,
	wp_missile,
	wp_plasma,
	wp_bfg,
	wp_chainsaw,
	wp_supershotgun,

	NUMWEAPONS,

	// No pending weapon change.
	wp_nochange,
}

impl weapontype_t {
	pub(crate) const fn to_usize(self) -> usize {
		match self {
			weapontype_t::wp_fist => 0,
			weapontype_t::wp_pistol => 1,
			weapontype_t::wp_shotgun => 2,
			weapontype_t::wp_chaingun => 3,
			weapontype_t::wp_missile => 4,
			weapontype_t::wp_plasma => 5,
			weapontype_t::wp_bfg => 6,
			weapontype_t::wp_chainsaw => 7,
			weapontype_t::wp_supershotgun => 8,
			weapontype_t::NUMWEAPONS => 9,
			weapontype_t::wp_nochange => 10,
		}
	}
}

impl From<weapontype_t> for i32 {
	fn from(value: weapontype_t) -> Self {
		value.to_usize().try_into().unwrap()
	}
}

impl From<weapontype_t> for usize {
	fn from(value: weapontype_t) -> Self {
		value.to_usize()
	}
}

// Ammunition types defined.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum ammotype_t {
	am_clip,  // Pistol / chaingun ammo.
	am_shell, // Shotgun / double barreled shotgun.
	am_cell,  // Plasma rifle, BFG.
	am_misl,  // Missile launcher.
	NUMAMMO,
	am_noammo, // Unlimited for chainsaw / fist.
}

impl ammotype_t {
	pub(crate) const fn to_u8(self) -> u8 {
		match self {
			ammotype_t::am_clip => 0,
			ammotype_t::am_shell => 1,
			ammotype_t::am_cell => 2,
			ammotype_t::am_misl => 3,
			ammotype_t::NUMAMMO => 4,
			ammotype_t::am_noammo => 5,
		}
	}

	pub(crate) const fn to_usize(self) -> usize {
		match self {
			ammotype_t::am_clip => 0,
			ammotype_t::am_shell => 1,
			ammotype_t::am_cell => 2,
			ammotype_t::am_misl => 3,
			ammotype_t::NUMAMMO => 4,
			ammotype_t::am_noammo => 5,
		}
	}
}

impl From<u8> for ammotype_t {
	fn from(value: u8) -> Self {
		match value {
			0 => Self::am_clip,
			1 => Self::am_shell,
			2 => Self::am_cell,
			3 => Self::am_misl,
			_ => panic!("ammo_t out of bounds"),
		}
	}
}

impl From<ammotype_t> for u8 {
	fn from(value: ammotype_t) -> Self {
		value.to_u8()
	}
}

impl From<ammotype_t> for usize {
	fn from(value: ammotype_t) -> Self {
		value.to_usize()
	}
}

// Power up artifacts.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum powertype_t {
	pw_invulnerability,
	pw_strength,
	pw_invisibility,
	pw_ironfeet,
	pw_allmap,
	pw_infrared,
	NUMPOWERS,
}

impl powertype_t {
	pub(crate) const fn to_usize(self) -> usize {
		match self {
			powertype_t::pw_invulnerability => 0,
			powertype_t::pw_strength => 1,
			powertype_t::pw_invisibility => 2,
			powertype_t::pw_ironfeet => 3,
			powertype_t::pw_allmap => 4,
			powertype_t::pw_infrared => 5,
			powertype_t::NUMPOWERS => 6,
		}
	}
}

impl From<usize> for powertype_t {
	fn from(value: usize) -> Self {
		match value {
			0 => Self::pw_invulnerability,
			1 => Self::pw_strength,
			2 => Self::pw_invisibility,
			3 => Self::pw_ironfeet,
			4 => Self::pw_allmap,
			5 => Self::pw_infrared,
			_ => panic!("powertype_t out of bounds"),
		}
	}
}

impl From<powertype_t> for usize {
	fn from(value: powertype_t) -> Self {
		value.to_usize()
	}
}

// Power up durations,
//  how many seconds till expiration,
//  assuming TICRATE is 35 ticks/second.
pub(crate) const INVULNTICS: usize = 30 * TICRATE;
pub(crate) const INVISTICS: usize = 60 * TICRATE;
pub(crate) const INFRATICS: usize = 120 * TICRATE;
pub(crate) const IRONTICS: usize = 60 * TICRATE;

// DOOM keyboard definition.
// This is the stuff configured by Setup.Exe.
// Most key data are simple ascii (uppercased).
pub(crate) const KEY_RIGHTARROW: u8 = 0xae;
pub(crate) const KEY_LEFTARROW: u8 = 0xac;
pub(crate) const KEY_UPARROW: u8 = 0xad;
pub(crate) const KEY_DOWNARROW: u8 = 0xaf;
pub(crate) const KEY_ESCAPE: u8 = 27;
pub(crate) const KEY_ENTER: u8 = 13;
pub(crate) const KEY_TAB: u8 = 9;
pub(crate) const KEY_F1: u8 = 0x80 + 0x3b;
pub(crate) const KEY_F2: u8 = 0x80 + 0x3c;
pub(crate) const KEY_F3: u8 = 0x80 + 0x3d;
pub(crate) const KEY_F4: u8 = 0x80 + 0x3e;
pub(crate) const KEY_F5: u8 = 0x80 + 0x3f;
pub(crate) const KEY_F6: u8 = 0x80 + 0x40;
pub(crate) const KEY_F7: u8 = 0x80 + 0x41;
pub(crate) const KEY_F8: u8 = 0x80 + 0x42;
pub(crate) const KEY_F9: u8 = 0x80 + 0x43;
pub(crate) const KEY_F10: u8 = 0x80 + 0x44;
pub(crate) const KEY_F11: u8 = 0x80 + 0x57;
pub(crate) const KEY_F12: u8 = 0x80 + 0x58;

pub(crate) const KEY_BACKSPACE: u8 = 127;
pub(crate) const KEY_PAUSE: u8 = 0xff;

pub(crate) const KEY_EQUALS: u8 = 0x3d;
pub(crate) const KEY_MINUS: u8 = 0x2d;

pub(crate) const KEY_RSHIFT: u8 = 0x80 + 0x36;
pub(crate) const KEY_RCTRL: u8 = 0x80 + 0x1d;
pub(crate) const KEY_RALT: u8 = 0x80 + 0x38;

pub(crate) const KEY_LALT: u8 = KEY_RALT;
