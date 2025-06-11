#![allow(non_snake_case)]

use std::{ffi::c_char, num::Wrapping};

use crate::{m_fixed::FRACUNIT, tables::angle_t};

// SKY, store the number for name.
pub const SKYFLATNAME: *const c_char = c"F_SKY1".as_ptr();

// The sky map is 256*128*4 maps.
pub const ANGLETOSKYSHIFT: angle_t = Wrapping(22);

// sky mapping
#[unsafe(no_mangle)]
pub static mut skyflatnum: i32 = 0;
#[unsafe(no_mangle)]
pub static mut skytexture: i32 = 0;
#[unsafe(no_mangle)]
pub static mut skytexturemid: i32 = 0;

// Called whenever the view size changes.
#[unsafe(no_mangle)]
pub extern "C" fn R_InitSkyMap() {
	unsafe { skytexturemid = 100 * FRACUNIT }
}
