// Global parameters/defines.

#![allow(non_camel_case_types, non_upper_case_globals)]

// DOOM version
pub const VERSION: i32 = 109;

// Game mode handling - identify IWAD version
//  to handle IWAD dependend animations etc.
#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum GameMode_t {
	shareware,  // DOOM 1 shareware, E1, M9
	registered, // DOOM 1 registered, E3, M27
	commercial, // DOOM 2 retail, E1 M34
	// DOOM 2 german edition not handled
	retail,       // DOOM 1 retail, E4, M36
	indetermined, // Well, no IWAD found.
}

// Mission packs - might be useful for TC stuff?
#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum GameMission_t {
	doom,      // DOOM 1
	doom2,     // DOOM 2
	pack_tnt,  // TNT mission pack
	pack_plut, // Plutonia pack
	none,
}

// Identify language to use, software localization.
#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Language_t {
	english,
	french,
	german,
	unknown,
}

// For resize of screen, at start of game.
// It will not work dynamically, see visplanes.
pub const BASE_WIDTH: usize = 320;

// It is educational but futile to change this
//  scaling e.g. to 2. Drawing of status bar,
//  menues etc. is tied to the scale implied
//  by the graphics.
pub const SCREEN_MUL: usize = 1;
pub const INV_ASPECT_RATIO: f64 = 0.625; // 0.75, ideally

// Defines suck. C sucks.
// C++ might sucks for OOP, but it sure is a better C.
// So there.
pub const SCREENWIDTH: usize = 320;
//SCREEN_MUL*BASE_WIDTH //320
pub const SCREENHEIGHT: usize = 200;
//(int)(SCREEN_MUL*BASE_WIDTH*INV_ASPECT_RATIO) //200

// The maximum number of players, multiplayer/networking.
pub const MAXPLAYERS: usize = 4;

// State updates, number of tics / second.
pub const TICRATE: usize = 35;

// The current state of the game: whether we are
// playing, gazing at the intermission screen,
// the game final animation, or a demo.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum gamestate_t {
	GS_LEVEL,
	GS_INTERMISSION,
	GS_FINALE,
	GS_DEMOSCREEN,

	None = -1,
}

// Difficulty/skill settings/filters.

// Skill flags.
pub const MTF_EASY: u8 = 1;
pub const MTF_NORMAL: u8 = 2;
pub const MTF_HARD: u8 = 4;

// Deaf monsters/do not react to sound.
pub const MTF_AMBUSH: u8 = 8;

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum skill_t {
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

// Key cards.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum card_t {
	it_bluecard,
	it_yellowcard,
	it_redcard,
	it_blueskull,
	it_yellowskull,
	it_redskull,

	NUMCARDS,
}

// The defined weapons,
//  including a marker indicating
//  user has not changed weapon.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum weapontype_t {
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

// Ammunition types defined.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ammotype_t {
	am_clip,  // Pistol / chaingun ammo.
	am_shell, // Shotgun / double barreled shotgun.
	am_cell,  // Plasma rifle, BFG.
	am_misl,  // Missile launcher.
	NUMAMMO,
	am_noammo, // Unlimited for chainsaw / fist.
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

// Power up artifacts.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum powertype_t {
	pw_invulnerability,
	pw_strength,
	pw_invisibility,
	pw_ironfeet,
	pw_allmap,
	pw_infrared,
	NUMPOWERS,
}

// Power up durations,
//  how many seconds till expiration,
//  assuming TICRATE is 35 ticks/second.
pub const INVULNTICS: usize = 30 * TICRATE;
pub const INVISTICS: usize = 60 * TICRATE;
pub const INFRATICS: usize = 120 * TICRATE;
pub const IRONTICS: usize = 60 * TICRATE;

// DOOM keyboard definition.
// This is the stuff configured by Setup.Exe.
// Most key data are simple ascii (uppercased).
pub const KEY_RIGHTARROW: u8 = 0xae;
pub const KEY_LEFTARROW: u8 = 0xac;
pub const KEY_UPARROW: u8 = 0xad;
pub const KEY_DOWNARROW: u8 = 0xaf;
pub const KEY_ESCAPE: u8 = 27;
pub const KEY_ENTER: u8 = 13;
pub const KEY_TAB: u8 = 9;
pub const KEY_F1: u8 = 0x80 + 0x3b;
pub const KEY_F2: u8 = 0x80 + 0x3c;
pub const KEY_F3: u8 = 0x80 + 0x3d;
pub const KEY_F4: u8 = 0x80 + 0x3e;
pub const KEY_F5: u8 = 0x80 + 0x3f;
pub const KEY_F6: u8 = 0x80 + 0x40;
pub const KEY_F7: u8 = 0x80 + 0x41;
pub const KEY_F8: u8 = 0x80 + 0x42;
pub const KEY_F9: u8 = 0x80 + 0x43;
pub const KEY_F10: u8 = 0x80 + 0x44;
pub const KEY_F11: u8 = 0x80 + 0x57;
pub const KEY_F12: u8 = 0x80 + 0x58;

pub const KEY_BACKSPACE: u8 = 127;
pub const KEY_PAUSE: u8 = 0xff;

pub const KEY_EQUALS: u8 = 0x3d;
pub const KEY_MINUS: u8 = 0x2d;

pub const KEY_RSHIFT: u8 = 0x80 + 0x36;
pub const KEY_RCTRL: u8 = 0x80 + 0x1d;
pub const KEY_RALT: u8 = 0x80 + 0x38;

pub const KEY_LALT: u8 = KEY_RALT;
