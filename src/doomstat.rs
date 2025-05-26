#![allow(non_camel_case_types, non_upper_case_globals)]

#[repr(C)]
pub enum GameMode_t {
	shareware,  // DOOM 1 shareware, E1, M9
	registered, // DOOM 1 registered, E3, M27
	commercial, // DOOM 2 retail, E1 M34
	// DOOM 2 german edition not handled
	retail,       // DOOM 1 retail, E4, M36
	indetermined, // Well, no IWAD found.
}

#[repr(C)]
pub enum GameMission_t {
	doom,      // DOOM 1
	doom2,     // DOOM 2
	pack_tnt,  // TNT mission pack
	pack_plut, // Plutonia pack
	none,
}

#[repr(C)]
pub enum Language_t {
	english,
	french,
	german,
	unknown,
}

#[unsafe(no_mangle)]
pub static mut gamemode: GameMode_t = GameMode_t::indetermined;
#[unsafe(no_mangle)]
pub static mut gamemission: GameMission_t = GameMission_t::doom;
#[unsafe(no_mangle)]
pub static mut language: Language_t = Language_t::english;
#[unsafe(no_mangle)]
pub static mut modifiedgame: bool = false;
