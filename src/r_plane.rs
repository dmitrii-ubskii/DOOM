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
	r_draw::{
		dc_colormap, dc_iscale, dc_source, dc_texturemid, dc_x, dc_yh, dc_yl, ds_colormap,
		ds_source, ds_x1, ds_x2, ds_xfrac, ds_xstep, ds_y, ds_yfrac, ds_ystep, viewheight,
		viewwidth,
	},
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
pub(crate) static mut floorplane: *mut visplane_t = null_mut();
pub(crate) static mut ceilingplane: *mut visplane_t = null_mut();

// ?
const MAXOPENINGS: usize = SCREENWIDTH * 64;
static mut openings: [i16; MAXOPENINGS] = [0; MAXOPENINGS];
pub(crate) static mut lastopening: *mut i16 = null_mut();

// Clip values are the solid pixel bounding the range.
//  floorclip starts out SCREENHEIGHT
//  ceilingclip starts out -1
pub(crate) static mut floorclip: [i16; SCREENWIDTH] = [0; SCREENWIDTH];
pub(crate) static mut ceilingclip: [i16; SCREENWIDTH] = [0; SCREENWIDTH];

// spanstart holds the start of a plane span
// initialized to 0 at start
static mut spanstart: [usize; SCREENHEIGHT] = [0; SCREENHEIGHT];

// texture mapping
static mut planezlight: *mut *mut lighttable_t = null_mut();
static mut planeheight: fixed_t = 0;

pub(crate) static mut yslope: [fixed_t; SCREENHEIGHT] = [0; SCREENHEIGHT];
pub(crate) static mut distscale: [fixed_t; SCREENWIDTH] = [0; SCREENWIDTH];
static mut basexscale: fixed_t = 0;
static mut baseyscale: fixed_t = 0;

static mut cachedheight: [fixed_t; SCREENHEIGHT] = [0; SCREENHEIGHT];
static mut cacheddistance: [fixed_t; SCREENHEIGHT] = [0; SCREENHEIGHT];
static mut cachedxstep: [fixed_t; SCREENHEIGHT] = [0; SCREENHEIGHT];
static mut cachedystep: [fixed_t; SCREENHEIGHT] = [0; SCREENHEIGHT];

// R_InitPlanes
// Only at game startup.
pub(crate) fn R_InitPlanes() {
	// Doh!
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
			let mut index = usize::try_from(distance >> LIGHTZSHIFT).unwrap();

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

// R_ClearPlanes
// At begining of frame.
#[allow(static_mut_refs)]
pub(crate) fn R_ClearPlanes() {
	unsafe {
		// opening / clipping determination
		for i in 0..viewwidth {
			floorclip[i] = i16::try_from(viewheight).unwrap();
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
pub(crate) fn R_FindPlane(
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
			I_Error("R_FindPlane: no more visplanes");
		}

		lastvisplane = lastvisplane.wrapping_add(1);

		(*check).height = height;
		(*check).picnum = picnum;
		(*check).lightlevel = lightlevel;
		(*check).minx = isize::try_from(SCREENWIDTH).unwrap();
		(*check).maxx = -1;

		(*check).top = [0xff; SCREENWIDTH];

		check
	}
}

// R_CheckPlane
pub(crate) fn R_CheckPlane(pl: &mut visplane_t, start: isize, stop: isize) -> *mut visplane_t {
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
		if pl.top[usize::try_from(x).unwrap()] != 0xff {
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
			R_MapPlane(usize::from(t1), spanstart[usize::from(t1)], x - 1);
			t1 += 1;
		}

		while b1 > b2 && b1 >= t1 {
			R_MapPlane(usize::from(b1), spanstart[usize::from(b1)], x - 1);
			b1 -= 1;
		}

		while t2 < t1 && t2 <= b2 {
			spanstart[usize::from(t2)] = x;
			t2 += 1;
		}

		while b2 > b1 && b2 >= t2 {
			spanstart[usize::from(b2)] = x;
			b2 -= 1;
		}
	}
}

// R_DrawPlanes
// At the end of each frame.
#[allow(static_mut_refs)]
pub(crate) fn R_DrawPlanes() {
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
					dc_yl = i32::from((*pl).top[usize::try_from(x).unwrap()]);
					dc_yh = i32::from((*pl).bottom[usize::try_from(x).unwrap()]);

					if dc_yl <= dc_yh {
						let angle = (viewangle + xtoviewangle[usize::try_from(x).unwrap()]).0
							>> ANGLETOSKYSHIFT.0;
						dc_x = i32::try_from(x).unwrap();
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
			let light = (((*pl).lightlevel >> LIGHTSEGSHIFT) + extralight)
				.clamp(0, i32::try_from(LIGHTLEVELS).unwrap() - 1);

			planezlight = zlight[usize::try_from(light).unwrap()].as_mut_ptr();

			// CAN BE OUT OF BOUNDS!
			*(*pl).top.as_mut_ptr().wrapping_offset((*pl).maxx + 1) = 0xff;
			*(*pl).top.as_mut_ptr().wrapping_offset((*pl).minx - 1) = 0xff;

			let stop = (*pl).maxx + 1;

			for x in (*pl).minx..=stop {
				R_MakeSpans(
					usize::try_from(x).unwrap(),
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
