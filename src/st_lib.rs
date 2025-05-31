#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::{
	ffi::{c_char, c_void},
	ptr::null_mut,
};

use crate::{i_system::I_Error, r_defs::patch_t, st_stuff::ST_Y, z_zone::PU_STATIC};

unsafe extern "C" {
	fn W_CacheLumpName(name: *const c_char, tag: usize) -> *mut c_void;
	fn V_DrawPatch(w: i32, y: i32, fg: i32, p: *const patch_t);
	fn V_CopyRect(
		srcx: i32,
		srcy: i32,
		srcscrn: i32,
		width: i32,
		height: i32,
		destx: i32,
		desty: i32,
		destscrn: i32,
	);
}

// Background and foreground screen numbers
const BG: i32 = 4;
const FG: i32 = 0;

// Typedefs of widgets

// Number widget

#[repr(C)]
pub struct st_number_t {
	// upper right-hand corner
	//  of the number (right-justified)
	x: i32,
	y: i32,

	// max # of digits in number
	width: i32,

	// last number value
	oldnum: i32,

	// pointer to current value
	num: *mut i32,

	// pointer to i32ean stating
	//  whether to update number
	on: *mut i32,

	// list of patches for 0-9
	p: *mut *mut patch_t,

	// user data
	data: i32,
}

// Percent widget ("child" of number widget,
//  or, more precisely, contains a number widget.)
#[repr(C)]
pub struct st_percent_t {
	// number information
	n: st_number_t,

	// percent sign graphic
	p: *mut patch_t,
}

// Multiple Icon widget
#[repr(C)]
pub struct st_multicon_t {
	// center-justified location of icons
	x: i32,
	y: i32,

	// last icon number
	oldinum: i32,

	// pointer to current icon
	inum: *mut i32,

	// pointer to i32ean stating
	//  whether to update icon
	on: *mut i32,

	// list of icons
	p: *mut *mut patch_t,

	// user data
	data: i32,
}

// Binary Icon widget

#[repr(C)]
pub struct st_binicon_t {
	// center-justified location of icon
	x: i32,
	y: i32,

	// last icon value
	oldval: i32,

	// pointer to current icon status
	val: *mut i32,

	// pointer to i32ean
	//  stating whether to update icon
	on: *mut i32,

	p: *mut patch_t, // icon
	data: i32,       // user data
}

// Hack display negative frags.
//  Loads and store the stminus lump.
static mut sttminus: *mut patch_t = null_mut();

// Widget creation, access, and update routines

// Initializes widget library.
// More precisely, initialize STMINUS,
//  everything else is done somewhere else.
#[unsafe(no_mangle)]
pub extern "C" fn STlib_init() {
	unsafe {
		sttminus = W_CacheLumpName(c"STTMINUS".as_ptr(), PU_STATIC) as _;
	}
}

// Number widget routines

// ?
#[unsafe(no_mangle)]
pub extern "C" fn STlib_initNum(
	n: &mut st_number_t,
	x: i32,
	y: i32,
	pl: *mut *mut patch_t,
	num: *mut i32,
	on: *mut i32,
	width: i32,
) {
	n.x = x;
	n.y = y;
	n.oldnum = 0;
	n.width = width;
	n.num = num;
	n.on = on;
	n.p = pl;
}

// A fairly efficient way to draw a number
//  based on differences from the old number.
// Note: worth the trouble?
#[unsafe(no_mangle)]
pub extern "C" fn STlib_drawNum(n: &mut st_number_t, _refresh: i32) {
	unsafe {
		let mut numdigits = n.width;
		let mut num = *n.num;

		let w = (**n.p).width;
		let h = (**n.p).height;

		n.oldnum = *n.num;

		let neg = num < 0;

		if neg {
			if numdigits == 2 && num < -9 {
				num = -9;
			} else if numdigits == 3 && num < -99 {
				num = -99;
			}

			num = -num;
		}

		// clear the area
		let mut x = n.x - numdigits * w as i32;

		if (n.y - ST_Y as i32) < 0 {
			I_Error(c"drawNum: n.y - ST_Y < 0".as_ptr());
		}

		V_CopyRect(x, n.y - ST_Y as i32, BG, w as i32 * numdigits, h as i32, x, n.y, FG);

		// if non-number, do not draw it
		if num == 1994 {
			return;
		}

		x = n.x;

		// in the special case of 0, you draw 0
		if num == 0 {
			V_DrawPatch(x - w as i32, n.y, FG, *n.p);
		}

		// draw the new number
		while num > 0 && numdigits > 0 {
			numdigits -= 1;
			x -= w as i32;
			V_DrawPatch(x, n.y, FG, *n.p.wrapping_add(num as usize % 10));
			num /= 10;
		}

		// draw a minus sign if necessary
		if neg {
			V_DrawPatch(x - 8, n.y, FG, sttminus);
		}
	}
}

#[unsafe(no_mangle)]
pub extern "C" fn STlib_updateNum(n: &mut st_number_t, refresh: i32) {
	unsafe {
		if *n.on != 0 {
			STlib_drawNum(n, refresh);
		}
	}
}

// Percent widget routines

#[unsafe(no_mangle)]
pub extern "C" fn STlib_initPercent(
	p: &mut st_percent_t,
	x: i32,
	y: i32,
	pl: *mut *mut patch_t,
	num: *mut i32,
	on: *mut i32,
	percent: *mut patch_t,
) {
	STlib_initNum(&mut p.n, x, y, pl, num, on, 3);
	p.p = percent;
}

#[unsafe(no_mangle)]
pub extern "C" fn STlib_updatePercent(per: &mut st_percent_t, refresh: i32) {
	unsafe {
		if refresh != 0 && *per.n.on != 0 {
			V_DrawPatch(per.n.x, per.n.y, FG, per.p);
		}

		STlib_updateNum(&mut per.n, refresh);
	}
}

// Multiple Icon widget routines

#[unsafe(no_mangle)]
pub extern "C" fn STlib_initMultIcon(
	i: &mut st_multicon_t,
	x: i32,
	y: i32,
	il: *mut *mut patch_t,
	inum: *mut i32,
	on: *mut i32,
) {
	i.x = x;
	i.y = y;
	i.oldinum = -1;
	i.inum = inum;
	i.on = on;
	i.p = il;
}

#[unsafe(no_mangle)]
pub extern "C" fn STlib_updateMultIcon(mi: &mut st_multicon_t, refresh: i32) {
	unsafe {
		if *mi.on != 0 && (mi.oldinum != *mi.inum || refresh != 0) && (*mi.inum != -1) {
			if mi.oldinum != -1 {
				let x = mi.x - (**mi.p.wrapping_add(mi.oldinum as usize)).leftoffset as i32;
				let y = mi.y - (**mi.p.wrapping_add(mi.oldinum as usize)).topoffset as i32;
				let w = (**mi.p.wrapping_add(mi.oldinum as usize)).width;
				let h = (**mi.p.wrapping_add(mi.oldinum as usize)).height;

				if y < ST_Y as i32 {
					I_Error(c"updateMultIcon: y - ST_Y < 0".as_ptr());
				}

				V_CopyRect(x, y - ST_Y as i32, BG, w as _, h as _, x, y, FG);
			}
			V_DrawPatch(mi.x, mi.y, FG, *mi.p.wrapping_add(*mi.inum as usize));
			mi.oldinum = *mi.inum;
		}
	}
}

// Binary Icon widget routines

#[unsafe(no_mangle)]
pub extern "C" fn STlib_initBinIcon(
	b: &mut st_binicon_t,
	x: i32,
	y: i32,
	i: *mut patch_t,
	val: *mut i32,
	on: &mut i32,
) {
	b.x = x;
	b.y = y;
	b.oldval = 0;
	b.val = val;
	b.on = on;
	b.p = i;
}

#[unsafe(no_mangle)]
pub extern "C" fn STlib_updateBinIcon(bi: &mut st_binicon_t, refresh: i32) {
	unsafe {
		if *bi.on != 0 && (bi.oldval != *bi.val || refresh != 0) {
			let x = bi.x - ((*bi.p).leftoffset) as i32;
			let y = bi.y - ((*bi.p).topoffset) as i32;
			let w = (*bi.p).width as i32;
			let h = (*bi.p).height as i32;

			if y < ST_Y as i32 {
				I_Error(c"updateBinIcon: y - ST_Y < 0".as_ptr());
			}

			if *bi.val != 0 {
				V_DrawPatch(bi.x, bi.y, FG, bi.p);
			} else {
				V_CopyRect(x, y - ST_Y as i32, BG, w, h, x, y, FG);
			}

			bi.oldval = *bi.val;
		}
	}
}
