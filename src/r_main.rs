#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::{mem, num::Wrapping, ptr::null_mut};

use crate::{
	d_player::player_t,
	doomdata::NF_SUBSECTOR,
	doomdef::{SCREENHEIGHT, SCREENWIDTH},
	m_fixed::{FRACBITS, FRACUNIT, FixedDiv, FixedMul, fixed_t},
	m_menu::{detailLevel, screenblocks},
	p_setup::{nodes, numnodes, subsectors},
	r_data::{R_InitData, colormaps},
	r_defs::{lighttable_t, node_t, seg_t, subsector_t},
	r_plane::{distscale, yslope},
	r_sky::R_InitSkyMap,
	r_things::{R_ClearSprites, R_DrawMasked, pspriteiscale, pspritescale, screenheightarray},
	tables::{
		ANG90, ANG180, ANG270, ANGLETOFINESHIFT, DBITS, FINEANGLES, SlopeDiv, angle_t, finecos,
		finesine, finetangent, tantoangle,
	},
};

// Lighting LUT.
// Used for z-depth cuing per column/row,
//  and other lighting effects (sector ambient, flash).

// Lighting constants.
// Now why not 32 levels here?
pub const LIGHTLEVELS: usize = 16;
pub const LIGHTSEGSHIFT: usize = 4;

pub const MAXLIGHTSCALE: usize = 48;
pub const LIGHTSCALESHIFT: i32 = 12;
pub const MAXLIGHTZ: usize = 128;
pub const LIGHTZSHIFT: i32 = 20;

const NUMCOLORMAPS: usize = 32;

// Fineangles in the SCREENWIDTH wide window.
const FIELDOFVIEW: usize = 2048;

pub static mut viewangleoffset: i32 = 0;

// increment every time a check is made
pub static mut validcount: i32 = 1;

#[unsafe(no_mangle)]
pub static mut fixedcolormap: *mut lighttable_t = null_mut();

#[unsafe(no_mangle)]
pub static mut centerx: usize = 0;
#[unsafe(no_mangle)]
pub static mut centery: usize = 0;

#[unsafe(no_mangle)]
pub static mut centerxfrac: fixed_t = 0;
#[unsafe(no_mangle)]
pub static mut centeryfrac: fixed_t = 0;
pub static mut projection: fixed_t = 0;

// just for profiling purposes
pub static mut framecount: i32 = 0;

#[unsafe(no_mangle)]
pub static mut sscount: i32 = 0;
pub static mut linecount: i32 = 0;
pub static mut loopcount: i32 = 0;

#[unsafe(no_mangle)]
pub static mut viewx: fixed_t = 0;
#[unsafe(no_mangle)]
pub static mut viewy: fixed_t = 0;
#[unsafe(no_mangle)]
pub static mut viewz: fixed_t = 0;

#[unsafe(no_mangle)]
pub static mut viewangle: angle_t = Wrapping(0);

pub static mut viewcos: fixed_t = 0;
pub static mut viewsin: fixed_t = 0;

#[unsafe(no_mangle)]
pub static mut viewplayer: *mut player_t = null_mut();

// 0 = high, 1 = low
#[unsafe(no_mangle)]
pub static mut detailshift: i32 = 0;

// precalculated math tables
#[unsafe(no_mangle)]
pub static mut clipangle: angle_t = Wrapping(0);

// The viewangletox[viewangle + FINEANGLES/4] lookup
// maps the visible view angles to screen X coordinates,
// flattening the arc to a flat projection plane.
// There will be many angles mapped to the same X.
#[unsafe(no_mangle)]
pub static mut viewangletox: [i32; FINEANGLES / 2] = [0; FINEANGLES / 2];

// The xtoviewangleangle[] table maps a screen pixel
// to the lowest viewangle that maps back to x ranges
// from clipangle to -clipangle.
#[unsafe(no_mangle)]
pub static mut xtoviewangle: [angle_t; SCREENWIDTH + 1] = [Wrapping(0); SCREENWIDTH + 1];

#[unsafe(no_mangle)]
pub static mut scalelight: [[*mut lighttable_t; MAXLIGHTSCALE]; LIGHTLEVELS] =
	[[null_mut(); MAXLIGHTSCALE]; LIGHTLEVELS];
pub static mut scalelightfixed: [*mut lighttable_t; MAXLIGHTSCALE] = [null_mut(); MAXLIGHTSCALE];
#[unsafe(no_mangle)]
pub static mut zlight: [[*mut lighttable_t; MAXLIGHTZ]; LIGHTLEVELS] =
	[[null_mut(); MAXLIGHTZ]; LIGHTLEVELS];

// bumped light from gun blasts
#[unsafe(no_mangle)]
pub static mut extralight: i32 = 0;

unsafe extern "C" {
	fn R_DrawColumn();
	fn R_DrawFuzzColumn();
	fn R_DrawTranslatedColumn();
	fn R_DrawSpan();
	fn R_DrawColumnLow();
	fn R_DrawSpanLow();
}

#[unsafe(no_mangle)]
pub static mut colfunc: unsafe extern "C" fn() = R_DrawColumn;
pub static mut basecolfunc: unsafe extern "C" fn() = R_DrawColumn;
pub static mut fuzzcolfunc: unsafe extern "C" fn() = R_DrawColumn;
pub static mut transcolfunc: unsafe extern "C" fn() = R_DrawColumn;
#[unsafe(no_mangle)]
pub static mut spanfunc: unsafe extern "C" fn() = R_DrawColumn;

// R_PointOnSide
// Traverse BSP (sub) tree,
//  check point against partition plane.
// Returns side 0 (front) or 1 (back).
#[unsafe(no_mangle)]
pub extern "C" fn R_PointOnSide(x: fixed_t, y: fixed_t, node: &mut node_t) -> usize {
	if node.dx == 0 {
		return if x <= node.x { node.dy > 0 } else { node.dy < 0 } as usize;
	}

	if node.dy == 0 {
		return if y <= node.y { node.dx < 0 } else { node.dx > 0 } as usize;
	}

	let dx = x - node.x;
	let dy = y - node.y;

	// Try to quickly decide by looking at sign bits.
	if node.dy ^ node.dx ^ dx ^ dy < 0 {
		return (node.dy ^ dx < 0) as usize; // (left is negative)
	}

	let left = FixedMul(node.dy >> FRACBITS, dx);
	let right = FixedMul(dy, node.dx >> FRACBITS);

	(right >= left) as usize
}

pub fn R_PointOnSegSide(x: fixed_t, y: fixed_t, line: &mut seg_t) -> i32 {
	unsafe {
		let lx = (*line.v1).x;
		let ly = (*line.v1).y;

		let ldx = (*line.v2).x - lx;
		let ldy = (*line.v2).y - ly;

		if ldx == 0 {
			return if x <= lx { ldy > 0 } else { ldy < 0 } as i32;
		}

		if ldy == 0 {
			return if y <= ly { ldx < 0 } else { ldx > 0 } as i32;
		}

		let dx = x - lx;
		let dy = y - ly;

		// Try to quickly decide by looking at sign bits.
		if ldy ^ ldx ^ dx ^ dy < 0 {
			return (ldy ^ dx < 0) as i32; // (left is negative)
		}

		let left = FixedMul(ldy >> FRACBITS, dx);
		let right = FixedMul(dy, ldx >> FRACBITS);

		(right >= left) as i32
	}
}

// R_PointToAngle
// To get a global angle from cartesian coordinates,
//  the coordinates are flipped until they are in
//  the first octant of the coordinate system, then
//  the y (<=x) is scaled and divided by x to get a
//  tangent (slope) value which is looked up in the
//  tantoangle[] table.
#[unsafe(no_mangle)]
pub fn R_PointToAngle(mut x: fixed_t, mut y: fixed_t) -> angle_t {
	unsafe {
		x -= viewx;
		y -= viewy;

		if x == 0 && y == 0 {
			return Wrapping(0);
		}

		if x >= 0 {
			// x >=0
			let x = x as usize;

			if y >= 0 {
				// y>= 0
				let y = y as usize;

				if x > y {
					// octant 0
					tantoangle[SlopeDiv(y, x)]
				} else {
					// octant 1
					ANG90 - Wrapping(1) - tantoangle[SlopeDiv(x, y)]
				}
			} else {
				// y<0
				let y = -y as usize;

				if x > y {
					// octant 8
					-tantoangle[SlopeDiv(y, x)]
				} else {
					// octant 7
					ANG270 + tantoangle[SlopeDiv(x, y)]
				}
			}
		} else {
			// x<0
			let x = -x as usize;

			if y >= 0 {
				// y>= 0
				let y = y as usize;

				if x > y {
					// octant 3
					ANG180 - Wrapping(1) - tantoangle[SlopeDiv(y, x)]
				} else {
					// octant 2
					ANG90 + tantoangle[SlopeDiv(x, y)]
				}
			} else {
				// y<0
				let y = -y as usize;

				if x > y {
					// octant 4
					ANG180 + tantoangle[SlopeDiv(y, x)]
				} else {
					// octant 5
					ANG270 - Wrapping(1) - tantoangle[SlopeDiv(x, y)]
				}
			}
		}
	}
}

pub fn R_PointToAngle2(x1: fixed_t, y1: fixed_t, x2: fixed_t, y2: fixed_t) -> angle_t {
	unsafe {
		viewx = x1;
		viewy = y1;

		R_PointToAngle(x2, y2)
	}
}

#[unsafe(no_mangle)]
pub extern "C" fn R_PointToDist(x: fixed_t, y: fixed_t) -> fixed_t {
	unsafe {
		let mut dx = fixed_t::abs(x - viewx);
		let mut dy = fixed_t::abs(y - viewy);

		if dy > dx {
			mem::swap(&mut dx, &mut dy);
		}

		let angle = (tantoangle[FixedDiv(dy, dx) as usize >> DBITS] + ANG90).0 >> ANGLETOFINESHIFT;

		// use as cosine
		FixedDiv(dx, finesine[angle])
	}
}

// R_InitPointToAngle
fn R_InitPointToAngle() {
	// UNUSED - now getting from tables.c
}

unsafe extern "C" {
	static mut rw_distance: fixed_t;
	static mut rw_normalangle: angle_t;
}

// R_ScaleFromGlobalAngle
// Returns the texture mapping scale
//  for the current line (horizontal span)
//  at the given angle.
// rw_distance must be calculated first.
#[unsafe(no_mangle)]
pub extern "C" fn R_ScaleFromGlobalAngle(visangle: angle_t) -> fixed_t {
	unsafe {
		let anglea = ANG90 + (visangle - viewangle);
		let angleb = ANG90 + (visangle - rw_normalangle);

		// both sines are allways positive
		let sinea = finesine[anglea.0 >> ANGLETOFINESHIFT];
		let sineb = finesine[angleb.0 >> ANGLETOFINESHIFT];
		let num = FixedMul(projection, sineb) << detailshift;
		let den = FixedMul(rw_distance, sinea);

		if den > num >> 16 { FixedDiv(num, den).clamp(256, 64 * FRACUNIT) } else { 64 * FRACUNIT }
	}
}

// R_InitTables
fn R_InitTables() {
	// UNUSED: now getting from tables.c
}

unsafe extern "C" {
	static mut viewwidth: usize;
}

// R_InitTextureMapping
fn R_InitTextureMapping() {
	unsafe {
		// Use tangent table to generate viewangletox:
		//  viewangletox will give the next greatest x
		//  after the view angle.
		//
		// Calc focallength
		//  so FIELDOFVIEW angles covers SCREENWIDTH.
		let focallength = FixedDiv(centerxfrac, finetangent[FINEANGLES / 4 + FIELDOFVIEW / 2]);

		for i in 0..FINEANGLES / 2 {
			let mut t: i32;
			if finetangent[i] > FRACUNIT * 2 {
				t = -1;
			} else if finetangent[i] < -FRACUNIT * 2 {
				t = viewwidth as i32 + 1;
			} else {
				t = FixedMul(finetangent[i], focallength);
				t = (centerxfrac - t + FRACUNIT - 1) >> FRACBITS;
				t = t.clamp(-1, viewwidth as i32 + 1);
			}
			viewangletox[i] = t;
		}

		// Scan viewangletox[] to generate xtoviewangle[]:
		//  xtoviewangle will give the smallest view angle
		//  that maps to x.
		#[allow(clippy::needless_range_loop)]
		for x in 0..=viewwidth {
			let mut i = 0;
			while viewangletox[i] > x as i32 {
				i += 1;
			}
			xtoviewangle[x] = Wrapping(i << ANGLETOFINESHIFT) - ANG90;
		}

		// Take out the fencepost cases from viewangletox.
		#[allow(clippy::needless_range_loop)]
		for i in 0..FINEANGLES / 2 {
			if viewangletox[i] == -1 {
				viewangletox[i] = 0;
			} else if viewangletox[i] == viewwidth as i32 + 1 {
				viewangletox[i] = viewwidth as i32;
			}
		}

		clipangle = xtoviewangle[0];
	}
}

// R_InitLightTables
// Only inits the zlight table,
//  because the scalelight table changes with view size.
const DISTMAP: usize = 2;

fn R_InitLightTables() {
	unsafe {
		// Calculate the light levels to use
		//  for each level / distance combination.
		#[allow(clippy::needless_range_loop)]
		for i in 0..LIGHTLEVELS {
			let startmap = ((LIGHTLEVELS - 1 - i) * 2) * NUMCOLORMAPS / LIGHTLEVELS;
			for j in 0..MAXLIGHTZ as i32 {
				let mut scale = FixedDiv(SCREENWIDTH as i32 / 2 * FRACUNIT, (j + 1) << LIGHTZSHIFT);
				scale >>= LIGHTSCALESHIFT;
				let mut level = startmap.saturating_sub(scale as usize / DISTMAP);

				if level >= NUMCOLORMAPS {
					level = NUMCOLORMAPS - 1;
				}

				zlight[i][j as usize] = colormaps.wrapping_add(level * 256);
			}
		}
	}
}

// R_SetViewSize
// Do not really change anything here,
//  because it might be in the middle of a refresh.
// The change will take effect next refresh.
pub static mut setsizeneeded: bool = false;
static mut setblocks: usize = 0;
static mut setdetail: i32 = 0;

pub fn R_SetViewSize(blocks: usize, detail: i32) {
	unsafe {
		setsizeneeded = true;
		setblocks = blocks;
		setdetail = detail;
	}
}

unsafe extern "C" {
	static mut scaledviewwidth: usize;
	static mut viewheight: usize;

	fn R_InitBuffer(width: usize, height: usize);
}

// R_ExecuteSetViewSize
pub fn R_ExecuteSetViewSize() {
	unsafe {
		setsizeneeded = false;

		if setblocks == 11 {
			scaledviewwidth = SCREENWIDTH;
			viewheight = SCREENHEIGHT;
		} else {
			scaledviewwidth = setblocks * 32;
			viewheight = (setblocks * 168 / 10) & !7;
		}

		detailshift = setdetail;
		viewwidth = scaledviewwidth >> detailshift;

		centery = viewheight / 2;
		centerx = viewwidth / 2;
		centerxfrac = (centerx << FRACBITS) as fixed_t;
		centeryfrac = (centery << FRACBITS) as fixed_t;
		projection = centerxfrac;

		if detailshift == 0 {
			colfunc = R_DrawColumn;
			basecolfunc = R_DrawColumn;
			fuzzcolfunc = R_DrawFuzzColumn;
			transcolfunc = R_DrawTranslatedColumn;
			spanfunc = R_DrawSpan;
		} else {
			colfunc = R_DrawColumnLow;
			basecolfunc = R_DrawColumnLow;
			fuzzcolfunc = R_DrawFuzzColumn;
			transcolfunc = R_DrawTranslatedColumn;
			spanfunc = R_DrawSpanLow;
		}

		R_InitBuffer(scaledviewwidth, viewheight);

		R_InitTextureMapping();

		// psprite scales
		pspritescale = FRACUNIT * (viewwidth / SCREENWIDTH) as i32;
		pspriteiscale = FRACUNIT * (SCREENWIDTH / viewwidth) as i32;

		// thing clipping
		#[allow(clippy::needless_range_loop)]
		for i in 0..viewwidth {
			screenheightarray[i] = viewheight as i16;
		}

		// planes
		#[allow(clippy::needless_range_loop)]
		for i in 0..viewheight {
			let mut dy = ((i as i32 - viewheight as i32 / 2) << FRACBITS) + FRACUNIT / 2;
			dy = fixed_t::abs(dy);
			yslope[i] = FixedDiv((viewwidth << detailshift) as fixed_t / 2 * FRACUNIT, dy);
		}

		#[allow(clippy::needless_range_loop)]
		for i in 0..viewwidth {
			let cosadj = finecos(xtoviewangle[i].0 >> ANGLETOFINESHIFT).abs();
			distscale[i] = FixedDiv(FRACUNIT, cosadj);
		}

		// Calculate the light levels to use
		//  for each level / scale combination.
		#[allow(clippy::needless_range_loop)]
		for i in 0..LIGHTLEVELS {
			let startmap = ((LIGHTLEVELS - 1 - i) * 2) * NUMCOLORMAPS / LIGHTLEVELS;
			for j in 0..MAXLIGHTSCALE {
				let mut level =
					startmap.saturating_sub(j * SCREENWIDTH / (viewwidth << detailshift) / DISTMAP);

				level = level.clamp(0, NUMCOLORMAPS - 1);

				scalelight[i][j] = colormaps.wrapping_add(level * 256);
			}
		}
	}
}

unsafe extern "C" {
	fn R_InitPlanes();
	fn R_InitTranslationTables();
}

// R_Init
pub fn R_Init() {
	unsafe {
		R_InitData();
		print!("\nR_InitData");
		R_InitPointToAngle();
		print!("\nR_InitPointToAngle");
		R_InitTables();
		// viewwidth / viewheight / detailLevel are set by the defaults
		print!("\nR_InitTables");

		R_SetViewSize(screenblocks, detailLevel);
		R_InitPlanes();
		print!("\nR_InitPlanes");
		R_InitLightTables();
		print!("\nR_InitLightTables");
		R_InitSkyMap();
		print!("\nR_InitSkyMap");
		R_InitTranslationTables();
		print!("\nR_InitTranslationsTables");

		framecount = 0;
	}
}

// R_PointInSubsector
pub fn R_PointInSubsector(x: fixed_t, y: fixed_t) -> *mut subsector_t {
	unsafe {
		// single subsector is a special case
		if numnodes == 0 {
			return subsectors;
		}

		let mut nodenum = numnodes - 1;

		while (nodenum & NF_SUBSECTOR) == 0 {
			let node = nodes.wrapping_add(nodenum);
			let side = R_PointOnSide(x, y, &mut *node);
			nodenum = (*node).children[side] as usize;
		}

		subsectors.wrapping_add(nodenum & !NF_SUBSECTOR)
	}
}

unsafe extern "C" {
	static mut walllights: *mut *mut lighttable_t;
}

// R_SetupFrame
#[allow(static_mut_refs)]
fn R_SetupFrame(player: &mut player_t) {
	unsafe {
		viewplayer = player;
		viewx = (*player.mo).x;
		viewy = (*player.mo).y;
		viewangle = (*player.mo).angle + Wrapping(viewangleoffset as usize);
		extralight = player.extralight;

		viewz = player.viewz;

		viewsin = finesine[viewangle.0 >> ANGLETOFINESHIFT];
		viewcos = finecos(viewangle.0 >> ANGLETOFINESHIFT);

		sscount = 0;

		if player.fixedcolormap != 0 {
			fixedcolormap = colormaps.wrapping_add(player.fixedcolormap * 256);

			walllights = scalelightfixed.as_mut_ptr();

			#[allow(clippy::needless_range_loop)]
			for i in 0..MAXLIGHTSCALE {
				scalelightfixed[i] = fixedcolormap;
			}
		} else {
			fixedcolormap = null_mut();
		}

		framecount += 1;
		validcount += 1;
	}
}

unsafe extern "C" {
	fn R_ClearClipSegs();
	fn R_ClearDrawSegs();

	fn R_ClearPlanes();
	fn R_DrawPlanes();

	fn NetUpdate();

	fn R_RenderBSPNode(bspnum: usize);
}

// R_RenderView
pub fn R_RenderPlayerView(player: &mut player_t) {
	unsafe {
		R_SetupFrame(player);

		// Clear buffers.
		R_ClearClipSegs();
		R_ClearDrawSegs();
		R_ClearPlanes();
		R_ClearSprites();

		// check for new console commands.
		NetUpdate();

		// The head node is the last node output.
		R_RenderBSPNode(numnodes - 1);

		// Check for new console commands.
		NetUpdate();

		R_DrawPlanes();

		// Check for new console commands.
		NetUpdate();

		R_DrawMasked();

		// Check for new console commands.
		NetUpdate();
	}
}
