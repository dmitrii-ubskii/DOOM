#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]
//	The actual span/column drawing functions.
//	Here find the main potential for optimization,
//	 e.g. inline assembly, different algorithms.

use std::{ffi::c_void, ptr::null_mut};

use libc::memcpy;

use crate::{
	const_conv::i32_from_usize,
	doomdef::{GameMode_t, SCREENHEIGHT, SCREENWIDTH},
	doomstat::gamemode,
	i_system::I_Error,
	m_fixed::{FRACBITS, fixed_t},
	r_data::colormaps,
	r_defs::lighttable_t,
	r_main::centery,
	v_video::{V_DrawPatch, V_MarkRect, screens},
	w_wad::W_CacheLumpName,
	z_zone::{PU_CACHE, PU_STATIC, Z_Malloc},
};

// ?
const MAXWIDTH: usize = 1120;
const MAXHEIGHT: usize = 832;

// status bar height at bottom of screen
const SBARHEIGHT: usize = 32;

// All drawing to the view buffer is accomplished in this file.
// The other refresh files only know about ccordinates,
//  not the architecture of the frame buffer.
// Conveniently, the frame buffer is a linear one,
//  and we need only the base address,
//  and the total size == width*height*depth/8.,
pub(crate) static mut viewwidth: usize = 0;
pub(crate) static mut scaledviewwidth: i32 = 0;
pub(crate) static mut viewheight: usize = 0;
pub(crate) static mut viewwindowx: usize = 0;
pub(crate) static mut viewwindowy: usize = 0;
static mut ylookup: [*mut i8; MAXHEIGHT] = [null_mut(); MAXHEIGHT];
static mut columnofs: [i32; MAXWIDTH] = [0; MAXWIDTH];

// R_DrawColumn
// Source is the top of the column to scale.
pub(crate) static mut dc_colormap: *mut lighttable_t = null_mut();
pub(crate) static mut dc_x: i32 = 0;
pub(crate) static mut dc_yl: i32 = 0;
pub(crate) static mut dc_yh: i32 = 0;
pub(crate) static mut dc_iscale: fixed_t = 0;
pub(crate) static mut dc_texturemid: fixed_t = 0;

// first pixel in a column (possibly virtual)
pub(crate) static mut dc_source: *mut u8 = null_mut();

// A column is a vertical slice/span from a wall texture that,
//  given the DOOM style restrictions on the view orientation,
//  will always have constant z depth.
// Thus a special case loop for very fast rendering can
//  be used. It has also been used with Wolfenstein 3D.
#[allow(static_mut_refs)]
pub(crate) fn R_DrawColumn() {
	unsafe {
		let count = dc_yh - dc_yl;

		// Zero length, column does not exceed a pixel.
		if count < 0 {
			return;
		}

		if dc_x >= SCREENWIDTH.try_into().unwrap()
			|| dc_yl < 0
			|| dc_yh >= SCREENHEIGHT.try_into().unwrap()
		{
			I_Error(format_args!("R_DrawColumn: {} to {} at {}", dc_yl, dc_yh, dc_x));
		}

		// Framebuffer destination address.
		// Use ylookup LUT to avoid multiply with ScreenWidth.
		// Use columnofs LUT for subwindows?
		let mut dest = ylookup[usize::try_from(dc_yl).unwrap()]
			.wrapping_add(usize::try_from(columnofs[usize::try_from(dc_x).unwrap()]).unwrap());

		// Determine scaling,
		//  which is the only mapping to be done.
		let fracstep = dc_iscale;
		let mut frac = dc_texturemid + (dc_yl - i32::try_from(centery).unwrap()) * fracstep;

		// Inner loop that does the actual texture mapping,
		//  e.g. a DDA-lile scaling.
		// This is as fast as it gets.
		for _ in 0..=count {
			// Re-map color indices from wall texture column
			//  using a lighting/special effects LUT.
			*dest = *dc_colormap.wrapping_add(usize::from(
				*dc_source.wrapping_add(usize::try_from((frac >> FRACBITS) & 127).unwrap()),
			));

			dest = dest.wrapping_add(SCREENWIDTH);
			frac += fracstep;
		}
	}
}

#[allow(static_mut_refs)]
pub(crate) fn R_DrawColumnLow() {
	unsafe {
		let count = dc_yh - dc_yl;

		// Zero length.
		if count < 0 {
			return;
		}

		if dc_x >= SCREENWIDTH.try_into().unwrap()
			|| dc_yl < 0
			|| dc_yh >= SCREENHEIGHT.try_into().unwrap()
		{
			I_Error(format_args!("R_DrawColumn: {} to {} at {}", dc_yl, dc_yh, dc_x));
		}

		// Blocky mode, need to multiply by 2.
		dc_x <<= 1;

		let mut dest = ylookup[usize::try_from(dc_yl).unwrap()]
			.wrapping_add(usize::try_from(columnofs[usize::try_from(dc_x).unwrap()]).unwrap());
		let mut dest2 = ylookup[usize::try_from(dc_yl).unwrap()]
			.wrapping_add(usize::try_from(columnofs[usize::try_from(dc_x + 1).unwrap()]).unwrap());

		let fracstep = dc_iscale;
		let mut frac = dc_texturemid + (dc_yl - i32::try_from(centery).unwrap()) * fracstep;

		for _ in 0..=count {
			// Hack. Does not work corretly.
			*dest = *dc_colormap.wrapping_add(usize::from(
				*dc_source.wrapping_add(usize::try_from(frac >> FRACBITS).unwrap() & 127),
			));
			*dest2 = *dest;
			dest = dest.wrapping_add(SCREENWIDTH);
			dest2 = dest2.wrapping_add(SCREENWIDTH);
			frac += fracstep;
		}
	}
}

// Spectre/Invisibility.
const FUZZTABLE: usize = 50;
const FUZZOFF: i32 = i32_from_usize(SCREENWIDTH);

const fuzzoffset: [i32; FUZZTABLE] = [
	FUZZOFF, -FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF,
	FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, -FUZZOFF, -FUZZOFF,
	-FUZZOFF, FUZZOFF, -FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF,
	-FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, -FUZZOFF, -FUZZOFF,
	-FUZZOFF, FUZZOFF, FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF,
];

static mut fuzzpos: usize = 0;

// Framebuffer postprocessing.
// Creates a fuzzy image by copying pixels
//  from adjacent ones to left and right.
// Used with an all black colormap, this
//  could create the SHADOW effect,
//  i.e. spectres and invisible players.
#[allow(static_mut_refs)]
pub(crate) fn R_DrawFuzzColumn() {
	unsafe {
		// Adjust borders. Low...
		if dc_yl == 0 {
			dc_yl = 1;
		}

		// .. and high.
		if dc_yh == i32::try_from(viewheight).unwrap() - 1 {
			dc_yh = i32::try_from(viewheight).unwrap() - 2;
		}

		let count = dc_yh - dc_yl;

		// Zero length.
		if count < 0 {
			return;
		}

		if dc_x >= SCREENWIDTH.try_into().unwrap()
			|| dc_yl < 0
			|| dc_yh >= SCREENHEIGHT.try_into().unwrap()
		{
			I_Error(format_args!("R_DrawFuzzColumn: {} to {} at {}", dc_yl, dc_yh, dc_x));
		}

		// Does not work with blocky mode.
		let mut dest = ylookup[usize::try_from(dc_yl).unwrap()]
			.wrapping_add(usize::try_from(columnofs[usize::try_from(dc_x).unwrap()]).unwrap());

		// Looks like an attempt at dithering,
		//  using the colormap #6 (of 0-31, a bit
		//  brighter than average).
		for _ in 0..=count {
			// Lookup framebuffer, and retrieve
			//  a pixel that is either one column
			//  left or right of the current one.
			// Add index from colormap to index.
			*dest = *colormaps.wrapping_add(
				usize::try_from(
					6 * 256
						+ isize::from(*dest.offset(isize::try_from(fuzzoffset[fuzzpos]).unwrap())),
				)
				.unwrap(),
			);

			// Clamp table lookup index.
			fuzzpos += 1;
			if fuzzpos == FUZZTABLE {
				fuzzpos = 0;
			}

			dest = dest.wrapping_add(SCREENWIDTH);
		}
	}
}

// R_DrawTranslatedColumn
// Used to draw player sprites
//  with the green colorramp mapped to others.
// Could be used with different translation
//  tables, e.g. the lighter colored version
//  of the BaronOfHell, the HellKnight, uses
//  identical sprites, kinda brightened up.
pub(crate) static mut dc_translation: *mut u8 = null_mut();
pub(crate) static mut translationtables: *mut u8 = null_mut();

#[allow(static_mut_refs)]
pub(crate) fn R_DrawTranslatedColumn() {
	unsafe {
		let count = dc_yh - dc_yl;
		if count < 0 {
			return;
		}

		if dc_x >= SCREENWIDTH.try_into().unwrap()
			|| dc_yl < 0
			|| dc_yh >= SCREENHEIGHT.try_into().unwrap()
		{
			I_Error(format_args!("R_DrawColumn: {} to {} at {}", dc_yl, dc_yh, dc_x));
		}

		// FIXME. As above.
		let mut dest = ylookup[usize::try_from(dc_yl).unwrap()]
			.wrapping_add(usize::try_from(columnofs[usize::try_from(dc_x).unwrap()]).unwrap());

		// Looks familiar.
		let fracstep = dc_iscale;
		let mut frac = dc_texturemid + (dc_yl - i32::try_from(centery).unwrap()) * fracstep;

		// Here we do an additional index re-mapping.
		for _ in 0..=count {
			// Translation tables are used
			//  to map certain colorramps to other ones,
			//  used with PLAY sprites.
			// Thus the "green" ramp of the player 0 sprite
			//  is mapped to gray, red, black/indigo.
			*dest = *dc_colormap.wrapping_add(usize::from(*dc_translation.wrapping_add(
				usize::from(*dc_source.wrapping_add(usize::try_from(frac >> FRACBITS).unwrap())),
			)));
			dest = dest.wrapping_add(SCREENWIDTH);

			frac += fracstep;
		}
	}
}

// R_InitTranslationTables
// Creates the translation tables to map
//  the green color ramp to gray, brown, red.
// Assumes a given structure of the PLAYPAL.
// Could be read from a lump instead.
pub(crate) fn R_InitTranslationTables() {
	unsafe {
		translationtables = Z_Malloc(256 * 3 + 255, PU_STATIC, null_mut()).cast();
		translationtables = translationtables.wrapping_add(255 - (translationtables.addr() & 255));

		// translate just the 16 green colors
		for i in 0..=255 {
			if (0x70..=0x7f).contains(&i) {
				// map green ramp to gray, brown, red
				*translationtables.wrapping_add(usize::from(i)) = 0x60 + (i & 0xf);
				*translationtables.wrapping_add(usize::from(i) + 256) = 0x40 + (i & 0xf);
				*translationtables.wrapping_add(usize::from(i) + 512) = 0x20 + (i & 0xf);
			} else {
				// Keep all other colors as is.
				*translationtables.wrapping_add(usize::from(i)) = i;
				*translationtables.wrapping_add(usize::from(i) + 256) = i;
				*translationtables.wrapping_add(usize::from(i) + 512) = i;
			}
		}
	}
}

// R_DrawSpan
// With DOOM style restrictions on view orientation,
//  the floors and ceilings consist of horizontal slices
//  or spans with constant z depth.
// However, rotation around the world z axis is possible,
//  thus this mapping, while simpler and faster than
//  perspective correct texture mapping, has to traverse
//  the texture at an angle in all but a few cases.
// In consequence, flats are not stored by column (like walls),
//  and the inner loop has to step in texture space u and v.
pub(crate) static mut ds_y: usize = 0;
pub(crate) static mut ds_x1: usize = 0;
pub(crate) static mut ds_x2: usize = 0;

pub(crate) static mut ds_colormap: *mut lighttable_t = null_mut();

pub(crate) static mut ds_xfrac: fixed_t = 0;
pub(crate) static mut ds_yfrac: fixed_t = 0;

pub(crate) static mut ds_xstep: fixed_t = 0;
pub(crate) static mut ds_ystep: fixed_t = 0;

// start of a 64*64 tile image
pub(crate) static mut ds_source: *mut u8 = null_mut();

// Draws the actual span.
#[allow(static_mut_refs)]
pub(crate) fn R_DrawSpan() {
	unsafe {
		if ds_x2 < ds_x1 || ds_x2 >= SCREENWIDTH || ds_y > SCREENHEIGHT {
			I_Error(format_args!("R_DrawSpan: {} to {} at {}", ds_x1, ds_x2, ds_y));
		}

		let mut xfrac = ds_xfrac;
		let mut yfrac = ds_yfrac;

		let mut dest = ylookup[ds_y].wrapping_add(usize::try_from(columnofs[ds_x1]).unwrap());

		// We do not check for zero spans here?
		let count = ds_x2 - ds_x1;

		for _ in 0..=count {
			// Current texture index in u,v.
			let spot = ((yfrac >> (16 - 6)) & (63 * 64)) + ((xfrac >> 16) & 63);

			// Lookup pixel from flat texture tile,
			//  re-index using light/colormap.
			*dest = *ds_colormap
				.wrapping_add(usize::from(*ds_source.wrapping_add(usize::try_from(spot).unwrap())));
			dest = dest.wrapping_add(1);

			// Next step in u,v.
			xfrac += ds_xstep;
			yfrac += ds_ystep;
		}
	}
}

// Again..
#[allow(static_mut_refs)]
pub(crate) fn R_DrawSpanLow() {
	unsafe {
		if ds_x2 < ds_x1 || ds_x2 >= SCREENWIDTH || ds_y > SCREENHEIGHT {
			I_Error(format_args!("R_DrawSpan: {} to {} at {}", ds_x1, ds_x2, ds_y));
		}

		let mut xfrac = ds_xfrac;
		let mut yfrac = ds_yfrac;

		// Blocky mode, need to multiply by 2.
		ds_x1 <<= 1;
		ds_x2 <<= 1;

		let mut dest = ylookup[ds_y].wrapping_add(usize::try_from(columnofs[ds_x1]).unwrap());

		let count = ds_x2 - ds_x1;
		for _ in 0..=count {
			let spot = ((yfrac >> (16 - 6)) & (63 * 64)) + ((xfrac >> 16) & 63);
			// Lowres/blocky mode does it twice,
			//  while scale is adjusted appropriately.
			*dest = *ds_colormap
				.wrapping_add(usize::from(*ds_source.wrapping_add(usize::try_from(spot).unwrap())));
			dest = dest.wrapping_add(1);
			*dest = *ds_colormap
				.wrapping_add(usize::from(*ds_source.wrapping_add(usize::try_from(spot).unwrap())));
			dest = dest.wrapping_add(1);

			xfrac += ds_xstep;
			yfrac += ds_ystep;
		}
	}
}

// R_InitBuffer
// Creats lookup tables that avoid
//  multiplies and other hazzles
//  for getting the framebuffer address
//  of a pixel to draw.
#[allow(static_mut_refs)]
pub(crate) fn R_InitBuffer(width: usize, height: usize) {
	unsafe {
		// Handle resize,
		//  e.g. smaller view windows
		//  with border and/or status bar.
		viewwindowx = (SCREENWIDTH - width) >> 1;

		// Column offset. For windows.
		#[allow(clippy::needless_range_loop)]
		for i in 0..width {
			columnofs[i] = i32::try_from(viewwindowx).unwrap() + i32::try_from(i).unwrap();
		}

		// Samw with base row offset.
		if width == SCREENWIDTH {
			viewwindowy = 0;
		} else {
			viewwindowy = (SCREENHEIGHT - SBARHEIGHT - height) >> 1;
		}

		// Preclaculate all row offsets.
		#[allow(clippy::needless_range_loop)]
		for i in 0..height {
			ylookup[i] = screens[0].wrapping_add((i + viewwindowy) * SCREENWIDTH).cast();
		}
	}
}

// R_FillBackScreen
// Fills the back screen with a pattern
//  for variable screen sizes
// Also draws a beveled edge.
pub(crate) fn R_FillBackScreen() {
	unsafe {
		// DOOM border patch.
		let name1 = c"FLOOR7_2";

		// DOOM II border patch.
		let name2 = c"GRNROCK";

		if scaledviewwidth == 320 {
			return;
		}

		let name = if gamemode == GameMode_t::commercial { name2 } else { name1 };

		let src = W_CacheLumpName(name, PU_CACHE).cast::<c_void>();
		let mut dest = screens[1].cast();

		for y in 0..SCREENHEIGHT - SBARHEIGHT {
			for _x in 0..SCREENWIDTH / 64 {
				memcpy(dest, src.wrapping_add((y & 63) << 6), 64);
				dest = dest.wrapping_add(64);
			}

			if SCREENWIDTH & 63 != 0 {
				memcpy(dest, src.wrapping_add((y & 63) << 6), SCREENWIDTH & 63);
				dest = dest.wrapping_add(SCREENWIDTH & 63);
			}
		}

		let patch = W_CacheLumpName(c"brdr_t", PU_CACHE).cast();
		for x in (0..usize::try_from(scaledviewwidth).unwrap()).step_by(8) {
			V_DrawPatch(viewwindowx + x, viewwindowy - 8, 1, patch);
		}

		let patch = W_CacheLumpName(c"brdr_b", PU_CACHE).cast();
		for x in (0..usize::try_from(scaledviewwidth).unwrap()).step_by(8) {
			V_DrawPatch(viewwindowx + x, viewwindowy + viewheight, 1, patch);
		}

		let patch = W_CacheLumpName(c"brdr_l", PU_CACHE).cast();
		for y in (0..viewheight).step_by(8) {
			V_DrawPatch(viewwindowx - 8, viewwindowy + y, 1, patch);
		}

		let patch = W_CacheLumpName(c"brdr_r", PU_CACHE).cast();
		for y in (0..viewheight).step_by(8) {
			V_DrawPatch(
				viewwindowx + usize::try_from(scaledviewwidth).unwrap(),
				viewwindowy + y,
				1,
				patch,
			);
		}

		// Draw beveled edge.
		V_DrawPatch(
			viewwindowx - 8,
			viewwindowy - 8,
			1,
			W_CacheLumpName(c"brdr_tl", PU_CACHE).cast(),
		);

		V_DrawPatch(
			viewwindowx + usize::try_from(scaledviewwidth).unwrap(),
			viewwindowy - 8,
			1,
			W_CacheLumpName(c"brdr_tr", PU_CACHE).cast(),
		);

		V_DrawPatch(
			viewwindowx - 8,
			viewwindowy + viewheight,
			1,
			W_CacheLumpName(c"brdr_bl", PU_CACHE).cast(),
		);

		V_DrawPatch(
			viewwindowx + usize::try_from(scaledviewwidth).unwrap(),
			viewwindowy + viewheight,
			1,
			W_CacheLumpName(c"brdr_br", PU_CACHE).cast(),
		);
	}
}

// Copy a screen buffer.
#[allow(static_mut_refs)]
pub(crate) fn R_VideoErase(ofs: usize, count: usize) {
	// LFB copy.
	// This might not be a good idea if memcpy
	//  is not optiomal, e.g. byte by byte on
	//  a 32bit CPU, as GNU GCC/Linux libc did
	//  at one point.
	unsafe {
		memcpy(screens[0].wrapping_add(ofs).cast(), screens[1].wrapping_add(ofs).cast(), count);
	}
}

// R_DrawViewBorder
// Draws the border around the view
//  for different size windows?
pub(crate) fn R_DrawViewBorder() {
	unsafe {
		if usize::try_from(scaledviewwidth).unwrap() == SCREENWIDTH {
			return;
		}

		let top = ((SCREENHEIGHT - SBARHEIGHT) - viewheight) / 2;
		let mut side = (SCREENWIDTH - usize::try_from(scaledviewwidth).unwrap()) / 2;

		// copy top and one line of left side
		R_VideoErase(0, top * SCREENWIDTH + side);

		// copy one line of right side and bottom
		let mut ofs = (viewheight + top) * SCREENWIDTH - side;
		R_VideoErase(ofs, top * SCREENWIDTH + side);

		// copy sides using wraparound
		ofs = top * SCREENWIDTH + SCREENWIDTH - side;
		side <<= 1;

		for _ in 1..viewheight {
			R_VideoErase(ofs, side);
			ofs += SCREENWIDTH;
		}

		// ?
		V_MarkRect(0, 0, SCREENWIDTH, SCREENHEIGHT - SBARHEIGHT);
	}
}
