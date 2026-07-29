#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::ptr::null_mut;

use crate::{
	i_system::I_Error,
	r_defs::patch_t,
	st_stuff::ST_Y,
	v_video::{V_CopyRect, V_DrawPatch},
	w_wad::W_CacheLumpName,
	z_zone::PU_STATIC,
};

// Background and foreground screen numbers
pub(crate) const BG: usize = 4;
pub(crate) const FG: usize = 0;

// Typedefs of widgets

// Number widget

#[repr(C)]
pub(crate) struct st_number_t {
	// upper right-hand corner
	//  of the number (right-justified)
	pub(crate) x: usize,
	pub(crate) y: usize,

	// max # of digits in number
	pub(crate) width: usize,

	// last number value
	pub(crate) oldnum: i32,

	// pointer to current value
	pub(crate) num: *mut i32,

	// pointer to i32ean stating
	//  whether to update number
	pub(crate) on: *mut bool,

	// list of patches for 0-9
	pub(crate) p: *mut *mut patch_t,

	// user data
	pub(crate) data: i32,
}

// Percent widget ("child" of number widget,
//  or, more precisely, contains a number widget.)
#[repr(C)]
pub(crate) struct st_percent_t {
	// number information
	pub(crate) n: st_number_t,

	// percent sign graphic
	pub(crate) p: *mut patch_t,
}

// Multiple Icon widget
#[repr(C)]
pub(crate) struct st_multicon_t {
	// center-justified location of icons
	pub(crate) x: usize,
	pub(crate) y: usize,

	// last icon number
	pub(crate) oldinum: i32,

	// pointer to current icon
	pub(crate) inum: *mut i32,

	// pointer to i32ean stating
	//  whether to update icon
	pub(crate) on: *mut bool,

	// list of icons
	pub(crate) p: *mut *mut patch_t,

	// user data
	pub(crate) data: i32,
}

// Binary Icon widget

#[repr(C)]
pub(crate) struct st_binicon_t {
	// center-justified location of icon
	pub(crate) x: usize,
	pub(crate) y: usize,

	// last icon value
	pub(crate) oldval: bool,

	// pointer to current icon status
	pub(crate) val: *mut bool,

	// pointer to i32ean
	//  stating whether to update icon
	pub(crate) on: *mut bool,

	pub(crate) p: *mut patch_t, // icon
	pub(crate) data: i32,       // user data
}

// Hack display negative frags.
//  Loads and store the stminus lump.
static mut sttminus: *mut patch_t = null_mut();

// Widget creation, access, and update routines

// Initializes widget library.
// More precisely, initialize STMINUS,
//  everything else is done somewhere else.
pub(crate) fn STlib_init() {
	unsafe {
		sttminus = W_CacheLumpName(c"STTMINUS".as_ptr(), PU_STATIC).cast();
	}
}

// Number widget routines

// ?
pub(crate) fn STlib_initNum(
	n: &mut st_number_t,
	x: usize,
	y: usize,
	pl: *mut *mut patch_t,
	num: *mut i32,
	on: *mut bool,
	width: usize,
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
fn STlib_drawNum(n: &mut st_number_t, _refresh: bool) {
	unsafe {
		let mut numdigits = n.width;
		let mut num = *n.num;

		let w = usize::try_from((**n.p).width).unwrap();
		let h = usize::try_from((**n.p).height).unwrap();

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
		let mut x = n.x - numdigits * w;

		if n.y < ST_Y {
			I_Error!(c"drawNum: n.y - ST_Y < 0".as_ptr());
		}

		V_CopyRect(x, n.y - ST_Y, BG, w * numdigits, h, x, n.y, FG);

		// if non-number, do not draw it
		if num == 1994 {
			return;
		}

		x = n.x;

		// in the special case of 0, you draw 0
		if num == 0 {
			V_DrawPatch(x - w, n.y, FG, *n.p);
		}

		// draw the new number
		while num > 0 && numdigits > 0 {
			numdigits -= 1;
			x -= w;
			V_DrawPatch(x, n.y, FG, *n.p.wrapping_add(usize::try_from(num).unwrap() % 10));
			num /= 10;
		}

		// draw a minus sign if necessary
		if neg {
			V_DrawPatch(x - 8, n.y, FG, sttminus);
		}
	}
}

pub(crate) fn STlib_updateNum(n: &mut st_number_t, refresh: bool) {
	unsafe {
		if *n.on {
			STlib_drawNum(n, refresh);
		}
	}
}

// Percent widget routines

pub(crate) fn STlib_initPercent(
	p: &mut st_percent_t,
	x: usize,
	y: usize,
	pl: *mut *mut patch_t,
	num: *mut i32,
	on: *mut bool,
	percent: *mut patch_t,
) {
	STlib_initNum(&mut p.n, x, y, pl, num, on, 3);
	p.p = percent;
}

pub(crate) fn STlib_updatePercent(per: &mut st_percent_t, refresh: bool) {
	unsafe {
		if refresh && *per.n.on {
			V_DrawPatch(per.n.x, per.n.y, FG, per.p);
		}

		STlib_updateNum(&mut per.n, refresh);
	}
}

// Multiple Icon widget routines

pub(crate) fn STlib_initMultIcon(
	i: &mut st_multicon_t,
	x: usize,
	y: usize,
	il: *mut *mut patch_t,
	inum: *mut i32,
	on: *mut bool,
) {
	i.x = x;
	i.y = y;
	i.oldinum = -1;
	i.inum = inum;
	i.on = on;
	i.p = il;
}

pub(crate) fn STlib_updateMultIcon(mi: &mut st_multicon_t, refresh: bool) {
	unsafe {
		if *mi.on && (mi.oldinum != *mi.inum || refresh) && (*mi.inum != -1) {
			if mi.oldinum != -1 {
				let x =
					mi.x.checked_add_signed(isize::from(
						-(**mi.p.wrapping_add(usize::try_from(mi.oldinum).unwrap())).leftoffset,
					))
					.unwrap();
				let y =
					mi.y.checked_add_signed(isize::from(
						-(**mi.p.wrapping_add(usize::try_from(mi.oldinum).unwrap())).topoffset,
					))
					.unwrap();
				let w = usize::try_from(
					(**mi.p.wrapping_add(usize::try_from(mi.oldinum).unwrap())).width,
				)
				.unwrap();
				let h = usize::try_from(
					(**mi.p.wrapping_add(usize::try_from(mi.oldinum).unwrap())).height,
				)
				.unwrap();

				if y < ST_Y {
					I_Error!(c"updateMultIcon: y - ST_Y < 0".as_ptr());
				}

				V_CopyRect(x, y - ST_Y, BG, w, h, x, y, FG);
			}
			V_DrawPatch(mi.x, mi.y, FG, *mi.p.wrapping_add(usize::try_from(*mi.inum).unwrap()));
			mi.oldinum = *mi.inum;
		}
	}
}

// Binary Icon widget routines

pub(crate) fn STlib_initBinIcon(
	b: &mut st_binicon_t,
	x: usize,
	y: usize,
	i: *mut patch_t,
	val: *mut bool,
	on: &mut bool,
) {
	b.x = x;
	b.y = y;
	b.oldval = false;
	b.val = val;
	b.on = on;
	b.p = i;
}

pub(crate) fn STlib_updateBinIcon(bi: &mut st_binicon_t, refresh: bool) {
	unsafe {
		if *bi.on && (bi.oldval != *bi.val || refresh) {
			let x = bi.x.checked_add_signed(isize::from(-((*bi.p).leftoffset))).unwrap();
			let y = bi.y.checked_add_signed(isize::from(-((*bi.p).topoffset))).unwrap();
			let w = usize::try_from((*bi.p).width).unwrap();
			let h = usize::try_from((*bi.p).height).unwrap();

			if y < ST_Y {
				I_Error!(c"updateBinIcon: y - ST_Y < 0".as_ptr());
			}

			if *bi.val {
				V_DrawPatch(bi.x, bi.y, FG, bi.p);
			} else {
				V_CopyRect(x, y - ST_Y, BG, w, h, x, y, FG);
			}

			bi.oldval = *bi.val;
		}
	}
}
