#![allow(non_snake_case)]

use crate::m_fixed::FRACUNIT;

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
