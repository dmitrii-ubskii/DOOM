#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

// The data sampled per tick (single player)
// and transmitted to other peers (multiplayer).
// Mainly movements/button commands per game tick,
// plus a checksum for internal state consistency.
#[repr(C)]
#[derive(Debug)]
pub struct ticcmd_t {
	pub forwardmove: i8,  // *2048 for move
	pub sidemove: i8,     // *2048 for move
	pub angleturn: i16,   // <<16 for angle delta
	pub consistancy: i16, // checks for net game
	pub chatchar: u8,
	pub buttons: u8,
}
