#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::{
	mem,
	ptr::{self, null_mut},
};

use crate::{
	doomdef::{SCREENHEIGHT, SCREENWIDTH},
	i_system::I_Error,
	m_fixed::{FixedDiv, FixedMul, fixed_t},
	r_data::{R_GetColumn, colormaps, firstflat, flattranslation},
	r_defs::{lighttable_t, visplane_t},
	r_main::{
		LIGHTLEVELS, LIGHTSEGSHIFT, LIGHTZSHIFT, MAXLIGHTZ, centerxfrac, colfunc, detailshift,
		extralight, fixedcolormap, spanfunc, viewangle, viewx, viewy, viewz, xtoviewangle, zlight,
	},
	r_sky::{ANGLETOSKYSHIFT, skyflatnum, skytexture, skytexturemid},
	r_things::pspriteiscale,
	tables::{ANG90, ANGLETOFINESHIFT, finecos, finesine},
	w_wad::W_CacheLumpNum,
	z_zone::{PU_CACHE, PU_STATIC, Z_ChangeTag},
};

// opening

// Here comes the obnoxious "visplane".
const MAXVISPLANES: usize = 128;
static mut visplanes: [visplane_t; MAXVISPLANES] = unsafe { mem::zeroed() };
static mut lastvisplane: *mut visplane_t = null_mut();
#[unsafe(no_mangle)]
pub static mut floorplane: *mut visplane_t = null_mut();
#[unsafe(no_mangle)]
pub static mut ceilingplane: *mut visplane_t = null_mut();

// ?
const MAXOPENINGS: usize = SCREENWIDTH * 64;
static mut openings: [i16; MAXOPENINGS] = [0; MAXOPENINGS];
#[unsafe(no_mangle)]
pub static mut lastopening: *mut i16 = null_mut();

// Clip values are the solid pixel bounding the range.
//  floorclip starts out SCREENHEIGHT
//  ceilingclip starts out -1
#[unsafe(no_mangle)]
pub static mut floorclip: [i16; SCREENWIDTH] = [0; SCREENWIDTH];
#[unsafe(no_mangle)]
pub static mut ceilingclip: [i16; SCREENWIDTH] = [0; SCREENWIDTH];

// spanstart holds the start of a plane span
// initialized to 0 at start
static mut spanstart: [usize; SCREENHEIGHT] = [0; SCREENHEIGHT];

// texture mapping
static mut planezlight: *mut *mut lighttable_t = null_mut();
static mut planeheight: fixed_t = 0;

pub static mut yslope: [fixed_t; SCREENHEIGHT] = [0; SCREENHEIGHT];
pub static mut distscale: [fixed_t; SCREENWIDTH] = [0; SCREENWIDTH];
static mut basexscale: fixed_t = 0;
static mut baseyscale: fixed_t = 0;

static mut cachedheight: [fixed_t; SCREENHEIGHT] = [0; SCREENHEIGHT];
static mut cacheddistance: [fixed_t; SCREENHEIGHT] = [0; SCREENHEIGHT];
static mut cachedxstep: [fixed_t; SCREENHEIGHT] = [0; SCREENHEIGHT];
static mut cachedystep: [fixed_t; SCREENHEIGHT] = [0; SCREENHEIGHT];

// R_InitPlanes
// Only at game startup.
#[unsafe(no_mangle)]
pub extern "C" fn R_InitPlanes() {
	// Doh!
}

unsafe extern "C" {
	static mut ds_xstep: fixed_t;
	static mut ds_ystep: fixed_t;

	static mut ds_xfrac: fixed_t;
	static mut ds_yfrac: fixed_t;

	static mut ds_y: usize;
	static mut ds_x1: usize;
	static mut ds_x2: usize;

	static mut ds_colormap: *mut lighttable_t;
}

// R_MapPlane
//
// Uses global vars:
//  planeheight
//  ds_source
//  basexscale
//  baseyscale
//  viewx
//  viewy
//
// BASIC PRIMITIVE
fn R_MapPlane(y: usize, x1: usize, x2: usize) {
	unsafe {
		let distance;
		if planeheight != cachedheight[y] {
			cachedheight[y] = planeheight;
			distance = FixedMul(planeheight, yslope[y]);
			cacheddistance[y] = distance;

			ds_xstep = FixedMul(distance, basexscale);
			cachedxstep[y] = ds_xstep;

			ds_ystep = FixedMul(distance, baseyscale);
			cachedystep[y] = ds_ystep;
		} else {
			distance = cacheddistance[y];
			ds_xstep = cachedxstep[y];
			ds_ystep = cachedystep[y];
		}

		let length = FixedMul(distance, distscale[x1]);
		let angle = (viewangle + xtoviewangle[x1]).0 >> ANGLETOFINESHIFT;
		ds_xfrac = viewx + FixedMul(finecos(angle), length);
		ds_yfrac = -viewy - FixedMul(finesine[angle], length);

		if !fixedcolormap.is_null() {
			ds_colormap = fixedcolormap;
		} else {
			let mut index = (distance >> LIGHTZSHIFT) as usize;

			if index >= MAXLIGHTZ {
				index = MAXLIGHTZ - 1;
			}

			ds_colormap = *planezlight.wrapping_add(index);
		}

		ds_y = y;
		ds_x1 = x1;
		ds_x2 = x2;

		// high or low detail
		spanfunc();
	}
}

unsafe extern "C" {
	static mut viewwidth: usize;
	static mut viewheight: usize;
}

// R_ClearPlanes
// At begining of frame.
#[allow(static_mut_refs)]
#[unsafe(no_mangle)]
pub extern "C" fn R_ClearPlanes() {
	unsafe {
		// opening / clipping determination
		for i in 0..viewwidth {
			floorclip[i] = viewheight as i16;
			ceilingclip[i] = -1;
		}

		lastvisplane = visplanes.as_mut_ptr();
		lastopening = openings.as_mut_ptr();

		// texture calculation
		cachedheight = [0; SCREENHEIGHT];

		// left to right mapping
		let angle = (viewangle - ANG90).0 >> ANGLETOFINESHIFT;

		// scale will be unit scale at SCREENWIDTH/2 distance
		basexscale = FixedDiv(finecos(angle), centerxfrac);
		baseyscale = -FixedDiv(finesine[angle], centerxfrac);
	}
}

// R_FindPlane
#[allow(static_mut_refs)]
#[unsafe(no_mangle)]
pub extern "C" fn R_FindPlane(
	mut height: fixed_t,
	picnum: usize,
	mut lightlevel: i32,
) -> *mut visplane_t {
	unsafe {
		if picnum == skyflatnum {
			height = 0; // all skys map together
			lightlevel = 0;
		}

		let mut check = visplanes.as_mut_ptr();

		while !ptr::eq(check, lastvisplane) {
			if height == (*check).height
				&& picnum == (*check).picnum
				&& lightlevel == (*check).lightlevel
			{
				break;
			}
			check = check.wrapping_add(1);
		}

		if !ptr::eq(check, lastvisplane) {
			return check;
		}

		if lastvisplane.offset_from_unsigned(visplanes.as_ptr()) == MAXVISPLANES {
			I_Error(c"R_FindPlane: no more visplanes".as_ptr());
		}

		lastvisplane = lastvisplane.wrapping_add(1);

		(*check).height = height;
		(*check).picnum = picnum;
		(*check).lightlevel = lightlevel;
		(*check).minx = SCREENWIDTH as isize;
		(*check).maxx = -1;

		(*check).top = [0xff; SCREENWIDTH];

		check
	}
}

// R_CheckPlane
#[unsafe(no_mangle)]
pub extern "C" fn R_CheckPlane(pl: &mut visplane_t, start: isize, stop: isize) -> *mut visplane_t {
	let intrl;
	let unionl;
	if start < pl.minx {
		intrl = pl.minx;
		unionl = start;
	} else {
		unionl = pl.minx;
		intrl = start;
	}

	let intrh;
	let unionh;
	if stop > pl.maxx {
		intrh = pl.maxx;
		unionh = stop;
	} else {
		unionh = pl.maxx;
		intrh = stop;
	}

	let mut x = intrl;
	while x <= intrh {
		if pl.top[x as usize] != 0xff {
			break;
		}
		x += 1;
	}

	if x > intrh {
		pl.minx = unionl;
		pl.maxx = unionh;

		// use the same one
		return pl;
	}

	unsafe {
		// make a new visplane
		(*lastvisplane).height = pl.height;
		(*lastvisplane).picnum = pl.picnum;
		(*lastvisplane).lightlevel = pl.lightlevel;

		let pl = lastvisplane;
		lastvisplane = lastvisplane.wrapping_add(1);
		(*pl).minx = start;
		(*pl).maxx = stop;
		(*pl).top = [0xff; SCREENWIDTH];
		pl
	}
}

// R_MakeSpans
fn R_MakeSpans(x: usize, mut t1: u8, mut b1: u8, mut t2: u8, mut b2: u8) {
	unsafe {
		while t1 < t2 && t1 <= b1 {
			R_MapPlane(t1 as usize, spanstart[t1 as usize], x - 1);
			t1 += 1;
		}

		while b1 > b2 && b1 >= t1 {
			R_MapPlane(b1 as usize, spanstart[b1 as usize], x - 1);
			b1 -= 1;
		}

		while t2 < t1 && t2 <= b2 {
			spanstart[t2 as usize] = x;
			t2 += 1;
		}

		while b2 > b1 && b2 >= t2 {
			spanstart[b2 as usize] = x;
			b2 -= 1;
		}
	}
}

unsafe extern "C" {
	static mut dc_colormap: *mut lighttable_t;
	static mut dc_x: i32;
	static mut dc_yl: i32;
	static mut dc_yh: i32;
	static mut dc_iscale: fixed_t;
	static mut dc_texturemid: fixed_t;

	static mut dc_source: *mut u8;
	static mut ds_source: *mut u8;
}

// R_DrawPlanes
// At the end of each frame.
#[allow(static_mut_refs)]
#[unsafe(no_mangle)]
pub extern "C" fn R_DrawPlanes() {
	unsafe {
		let mut pl = visplanes.as_mut_ptr();
		while !ptr::eq(pl, lastvisplane) {
			if (*pl).minx > (*pl).maxx {
				pl = pl.wrapping_add(1);
				continue;
			}

			// sky flat
			if (*pl).picnum == skyflatnum {
				dc_iscale = pspriteiscale >> detailshift;

				// Sky is allways drawn full bright,
				//  i.e. colormaps[0] is used.
				// Because of this hack, sky is not affected
				//  by INVUL inverse mapping.
				dc_colormap = colormaps;
				dc_texturemid = skytexturemid;
				#[allow(clippy::needless_range_loop)]
				for x in (*pl).minx..=(*pl).maxx {
					dc_yl = (*pl).top[x as usize] as i32;
					dc_yh = (*pl).bottom[x as usize] as i32;

					if dc_yl <= dc_yh {
						let angle = (viewangle + xtoviewangle[x as usize]).0 >> ANGLETOSKYSHIFT.0;
						dc_x = x as i32;
						dc_source = R_GetColumn(skytexture, angle);
						colfunc();
					}
				}
				pl = pl.wrapping_add(1);
				continue;
			}

			// regular flat
			ds_source =
				W_CacheLumpNum(firstflat + *flattranslation.wrapping_add((*pl).picnum), PU_STATIC)
					.cast();

			planeheight = fixed_t::abs((*pl).height - viewz);
			let light =
				(((*pl).lightlevel >> LIGHTSEGSHIFT) + extralight).clamp(0, LIGHTLEVELS as i32 - 1);

			planezlight = zlight[light as usize].as_mut_ptr();

			// CAN BE OUT OF BOUNDS!
			*(*pl).top.as_mut_ptr().wrapping_offset((*pl).maxx + 1) = 0xff;
			*(*pl).top.as_mut_ptr().wrapping_offset((*pl).minx - 1) = 0xff;

			let stop = (*pl).maxx + 1;

			for x in (*pl).minx..=stop {
				R_MakeSpans(
					x as usize,
					*(*pl).top.as_mut_ptr().wrapping_offset(x - 1),
					*(*pl).bottom.as_mut_ptr().wrapping_offset(x - 1),
					*(*pl).top.as_mut_ptr().wrapping_offset(x),
					*(*pl).bottom.as_mut_ptr().wrapping_offset(x),
				);
			}

			Z_ChangeTag!(ds_source, PU_CACHE);

			pl = pl.wrapping_add(1);
		}
	}
}
