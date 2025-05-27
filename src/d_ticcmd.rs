#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]
use std::ffi::c_char;

// The data sampled per tick (single player)
// and transmitted to other peers (multiplayer).
// Mainly movements/button commands per game tick,
// plus a checksum for internal state consistency.
#[repr(C)]
pub struct ticcmd_t {
	forwardmove: c_char, // *2048 for move
	sidemove: c_char,    // *2048 for move
	angleturn: i16,      // <<16 for angle delta
	consistancy: i16,    // checks for net game
	chatchar: u8,
	buttons: u8,
}
