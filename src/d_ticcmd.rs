#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

// The data sampled per tick (single player)
// and transmitted to other peers (multiplayer).
// Mainly movements/button commands per game tick,
// plus a checksum for internal state consistency.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct ticcmd_t {
	pub(crate) forwardmove: i8,  // *2048 for move
	pub(crate) sidemove: i8,     // *2048 for move
	pub(crate) angleturn: i16,   // <<16 for angle delta
	pub(crate) consistancy: i16, // checks for net game
	pub(crate) chatchar: u8,
	pub(crate) buttons: u8,
}
