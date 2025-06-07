#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use crate::{info::state_t, m_fixed::fixed_t};

// Frame flags:
// handles maximum brightness (torches, muzzle flare, light sources)
pub const FF_FULLBRIGHT: usize = 0x8000; // flag in thing->frame
pub const FF_FRAMEMASK: usize = 0x7fff;

// Overlay psprites are scaled shapes
// drawn directly on the view screen,
// coordinates are given for a 320*200 view screen.
#[repr(C)]
pub enum psprnum_t {
	ps_weapon,
	ps_flash,
	NUMPSPRITES,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct pspdef_t {
	pub state: *mut state_t, // a NULL state means not active
	pub tics: i32,
	pub sx: fixed_t,
	pub sy: fixed_t,
}
