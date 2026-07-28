#![allow(non_snake_case)]

use std::{ffi::c_char, num::Wrapping};

use crate::{m_fixed::FRACUNIT, tables::angle_t};

// SKY, store the number for name.
pub(crate) const SKYFLATNAME: *const c_char = c"F_SKY1".as_ptr();

// The sky map is 256*128*4 maps.
pub(crate) const ANGLETOSKYSHIFT: angle_t = Wrapping(22);

// sky mapping
#[unsafe(no_mangle)]
pub(crate) static mut skyflatnum: usize = 0;
#[unsafe(no_mangle)]
pub(crate) static mut skytexture: usize = 0;
#[unsafe(no_mangle)]
pub(crate) static mut skytexturemid: i32 = 0;

// Called whenever the view size changes.
pub(crate) fn R_InitSkyMap() {
	unsafe { skytexturemid = 100 * FRACUNIT }
}
