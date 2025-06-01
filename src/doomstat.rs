#![allow(non_camel_case_types, non_upper_case_globals)]

use crate::doomdef::{GameMission_t, GameMode_t, Language_t};

#[unsafe(no_mangle)]
pub static mut gamemode: GameMode_t = GameMode_t::indetermined;
#[unsafe(no_mangle)]
pub static mut gamemission: GameMission_t = GameMission_t::doom;
#[unsafe(no_mangle)]
pub static mut language: Language_t = Language_t::english;
#[unsafe(no_mangle)]
pub static mut modifiedgame: i32 = 0;
