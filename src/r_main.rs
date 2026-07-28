#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::{mem, num::Wrapping, ops::Index, ptr::null_mut};

use crate::{
	d_net::*,
	d_player::player_t,
	doomdata::NF_SUBSECTOR,
	doomdef::{SCREENHEIGHT, SCREENWIDTH},
	m_fixed::{FRACBITS, FRACUNIT, FixedDiv, FixedMul, fixed_t},
	m_menu::{detailLevel, screenblocks},
	p_setup::{nodes, numnodes, subsectors},
	r_bsp::{R_ClearClipSegs, R_ClearDrawSegs, R_RenderBSPNode},
	r_data::{R_InitData, colormaps},
	r_defs::{lighttable_t, node_t, seg_t, subsector_t},
	r_draw::{
		R_DrawColumn, R_DrawColumnLow, R_DrawFuzzColumn, R_DrawSpan, R_DrawSpanLow,
		R_DrawTranslatedColumn, R_InitBuffer, R_InitTranslationTables, scaledviewwidth, viewheight,
		viewwidth,
	},
	r_plane::{R_ClearPlanes, R_DrawPlanes, R_InitPlanes, distscale, yslope},
	r_segs::{rw_distance, rw_normalangle, walllights},
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

pub static mut fixedcolormap: *mut lighttable_t = null_mut();

pub static mut centerx: usize = 0;
pub static mut centery: usize = 0;

pub static mut centerxfrac: fixed_t = 0;
pub static mut centeryfrac: fixed_t = 0;
pub static mut projection: fixed_t = 0;

// just for profiling purposes
pub static mut framecount: i32 = 0;

pub static mut sscount: i32 = 0;
pub static mut linecount: i32 = 0;
pub static mut loopcount: i32 = 0;

pub static mut viewx: fixed_t = 0;
pub static mut viewy: fixed_t = 0;
pub static mut viewz: fixed_t = 0;

pub static mut viewangle: angle_t = Wrapping(0);

pub static mut viewcos: fixed_t = 0;
pub static mut viewsin: fixed_t = 0;

pub static mut viewplayer: *mut player_t = null_mut();

// 0 = high, 1 = low
pub static mut detailshift: i32 = 0;

// precalculated math tables
pub static mut clipangle: angle_t = Wrapping(0);

// The viewangletox[viewangle + FINEANGLES/4] lookup
// maps the visible view angles to screen X coordinates,
// flattening the arc to a flat projection plane.
// There will be many angles mapped to the same X.
pub static mut viewangletox: [u32; FINEANGLES / 2] = [0; FINEANGLES / 2];

// The xtoviewangleangle[] table maps a screen pixel
// to the lowest viewangle that maps back to x ranges
// from clipangle to -clipangle.
pub static mut xtoviewangle: [angle_t; SCREENWIDTH + 1] = [Wrapping(0); SCREENWIDTH + 1];

pub static mut scalelight: [[*mut lighttable_t; MAXLIGHTSCALE]; LIGHTLEVELS] =
	[[null_mut(); MAXLIGHTSCALE]; LIGHTLEVELS];
pub static mut scalelightfixed: [*mut lighttable_t; MAXLIGHTSCALE] = [null_mut(); MAXLIGHTSCALE];
pub static mut zlight: [[*mut lighttable_t; MAXLIGHTZ]; LIGHTLEVELS] =
	[[null_mut(); MAXLIGHTZ]; LIGHTLEVELS];

// bumped light from gun blasts
pub static mut extralight: i32 = 0;

pub static mut colfunc: unsafe fn() = R_DrawColumn;
pub static mut basecolfunc: unsafe fn() = R_DrawColumn;
pub static mut fuzzcolfunc: unsafe fn() = R_DrawColumn;
pub static mut transcolfunc: unsafe fn() = R_DrawColumn;
pub static mut spanfunc: unsafe fn() = R_DrawColumn;

#[derive(Debug, Clone, Copy)]
pub enum Side {
	Front,
	Back,
}

impl Side {
	pub fn flip(self) -> Self {
		match self {
			Self::Front => Self::Back,
			Self::Back => Self::Front,
		}
	}
}

impl<T> Index<Side> for [T; 2] {
	type Output = T;

	fn index(&self, index: Side) -> &Self::Output {
		match index {
			Side::Front => &self[0],
			Side::Back => &self[1],
		}
	}
}

// R_PointOnSide
// Traverse BSP (sub) tree,
//  check point against partition plane.
// Returns side 0 (front) or 1 (back).
pub fn R_PointOnSide(x: fixed_t, y: fixed_t, node: &node_t) -> Side {
	if node.dx == 0 {
		if x <= node.x && node.dy > 0 || x > node.x && node.dy < 0 {
			return Side::Back;
		} else {
			return Side::Front;
		};
	}

	if node.dy == 0 {
		if y <= node.y && node.dx < 0 || y > node.y && node.dx > 0 {
			return Side::Back;
		} else {
			return Side::Front;
		}
	}

	let dx = x - node.x;
	let dy = y - node.y;

	// Try to quickly decide by looking at sign bits.
	if node.dy ^ node.dx ^ dx ^ dy < 0 {
		// (left is negative)
		if node.dy ^ dx < 0 {
			return Side::Back;
		} else {
			return Side::Front;
		}
	}

	let left = FixedMul(node.dy >> FRACBITS, dx);
	let right = FixedMul(dy, node.dx >> FRACBITS);

	if right >= left { Side::Back } else { Side::Front }
}

pub fn R_PointOnSegSide(x: fixed_t, y: fixed_t, line: &mut seg_t) -> i32 {
	unsafe {
		let lx = (*line.v1).x;
		let ly = (*line.v1).y;

		let ldx = (*line.v2).x - lx;
		let ldy = (*line.v2).y - ly;

		if ldx == 0 {
			return i32::from(if x <= lx { ldy > 0 } else { ldy < 0 });
		}

		if ldy == 0 {
			return i32::from(if y <= ly { ldx < 0 } else { ldx > 0 });
		}

		let dx = x - lx;
		let dy = y - ly;

		// Try to quickly decide by looking at sign bits.
		if ldy ^ ldx ^ dx ^ dy < 0 {
			return i32::from(ldy ^ dx < 0); // (left is negative)
		}

		let left = FixedMul(ldy >> FRACBITS, dx);
		let right = FixedMul(dy, ldx >> FRACBITS);

		i32::from(right >= left)
	}
}

// R_PointToAngle
// To get a global angle from cartesian coordinates,
//  the coordinates are flipped until they are in
//  the first octant of the coordinate system, then
//  the y (<=x) is scaled and divided by x to get a
//  tangent (slope) value which is looked up in the
//  tantoangle[] table.
pub fn R_PointToAngle(mut x: fixed_t, mut y: fixed_t) -> angle_t {
	unsafe {
		x -= viewx;
		y -= viewy;

		if x == 0 && y == 0 {
			return Wrapping(0);
		}

		if x >= 0 {
			// x >=0
			let x = usize::try_from(x).unwrap();

			if y >= 0 {
				// y>= 0
				let y = usize::try_from(y).unwrap();

				if x > y {
					// octant 0
					tantoangle[SlopeDiv(y, x)]
				} else {
					// octant 1
					ANG90 - Wrapping(1) - tantoangle[SlopeDiv(x, y)]
				}
			} else {
				// y<0
				let y = usize::try_from(-y).unwrap();

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
			let x = usize::try_from(-x).unwrap();

			if y >= 0 {
				// y>= 0
				let y = usize::try_from(y).unwrap();

				if x > y {
					// octant 3
					ANG180 - Wrapping(1) - tantoangle[SlopeDiv(y, x)]
				} else {
					// octant 2
					ANG90 + tantoangle[SlopeDiv(x, y)]
				}
			} else {
				// y<0
				let y = usize::try_from(-y).unwrap();

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

pub fn R_PointToDist(x: fixed_t, y: fixed_t) -> fixed_t {
	unsafe {
		let mut dx = fixed_t::abs(x - viewx);
		let mut dy = fixed_t::abs(y - viewy);

		if dy > dx {
			mem::swap(&mut dx, &mut dy);
		}

		let angle = (tantoangle[usize::try_from(FixedDiv(dy, dx)).unwrap() >> DBITS] + ANG90).0
			>> ANGLETOFINESHIFT;

		// use as cosine
		FixedDiv(dx, finesine[angle])
	}
}

// R_InitPointToAngle
fn R_InitPointToAngle() {
	// UNUSED - now getting from tables.c
}

// R_ScaleFromGlobalAngle
// Returns the texture mapping scale
//  for the current line (horizontal span)
//  at the given angle.
// rw_distance must be calculated first.
pub fn R_ScaleFromGlobalAngle(visangle: angle_t) -> fixed_t {
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
			let t: u32;
			if finetangent[i] > FRACUNIT * 2 {
				t = u32::MAX;
			} else if finetangent[i] < -FRACUNIT * 2 {
				t = u32::try_from(viewwidth).unwrap() + 1;
			} else {
				let t_ = FixedMul(finetangent[i], focallength);
				let t_ = (centerxfrac - t_ + FRACUNIT - 1) >> FRACBITS;
				t = t_.clamp(-1, i32::try_from(viewwidth).unwrap() + 1).cast_unsigned();
			}
			viewangletox[i] = t;
		}

		// Scan viewangletox[] to generate xtoviewangle[]:
		//  xtoviewangle will give the smallest view angle
		//  that maps to x.
		#[allow(clippy::needless_range_loop)]
		for x in 0..=viewwidth {
			let mut i = 0;
			while viewangletox[i] > u32::try_from(x).unwrap() {
				i += 1;
			}
			xtoviewangle[x] = Wrapping(i << ANGLETOFINESHIFT) - ANG90;
		}

		// Take out the fencepost cases from viewangletox.
		#[allow(clippy::needless_range_loop)]
		for i in 0..FINEANGLES / 2 {
			if viewangletox[i] == u32::MAX {
				viewangletox[i] = 0;
			} else if viewangletox[i] == u32::try_from(viewwidth).unwrap() + 1 {
				viewangletox[i] = u32::try_from(viewwidth).unwrap();
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
			for j in 0..i32::try_from(MAXLIGHTZ).unwrap() {
				let mut scale = FixedDiv(
					i32::try_from(SCREENWIDTH).unwrap() / 2 * FRACUNIT,
					(j + 1) << LIGHTZSHIFT,
				);
				scale >>= LIGHTSCALESHIFT;
				let mut level = startmap.saturating_sub(usize::try_from(scale).unwrap() / DISTMAP);

				if level >= NUMCOLORMAPS {
					level = NUMCOLORMAPS - 1;
				}

				zlight[i][usize::try_from(j).unwrap()] = colormaps.wrapping_add(level * 256);
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

// R_ExecuteSetViewSize
pub fn R_ExecuteSetViewSize() {
	unsafe {
		setsizeneeded = false;

		if setblocks == 11 {
			scaledviewwidth = i32::try_from(SCREENWIDTH).unwrap();
			viewheight = SCREENHEIGHT;
		} else {
			scaledviewwidth = i32::try_from(setblocks).unwrap() * 32;
			viewheight = (setblocks * 168 / 10) & !7;
		}

		detailshift = setdetail;
		viewwidth = usize::try_from(scaledviewwidth).unwrap() >> detailshift;

		centery = viewheight / 2;
		centerx = viewwidth / 2;
		centerxfrac = fixed_t::try_from(centerx << FRACBITS).unwrap();
		centeryfrac = fixed_t::try_from(centery << FRACBITS).unwrap();
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

		R_InitBuffer(usize::try_from(scaledviewwidth).unwrap(), viewheight);

		R_InitTextureMapping();

		// psprite scales
		pspritescale = FRACUNIT * i32::try_from(viewwidth / SCREENWIDTH).unwrap();
		pspriteiscale = FRACUNIT * i32::try_from(SCREENWIDTH / viewwidth).unwrap();

		// thing clipping
		#[allow(clippy::needless_range_loop)]
		for i in 0..viewwidth {
			screenheightarray[i] = i16::try_from(viewheight).unwrap();
		}

		// planes
		#[allow(clippy::needless_range_loop)]
		for i in 0..viewheight {
			let mut dy = ((i32::try_from(i).unwrap() - i32::try_from(viewheight).unwrap() / 2)
				<< FRACBITS) + FRACUNIT / 2;
			dy = fixed_t::abs(dy);
			yslope[i] =
				FixedDiv(fixed_t::try_from(viewwidth << detailshift).unwrap() / 2 * FRACUNIT, dy);
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
			let side = R_PointOnSide(x, y, &*node);
			nodenum = usize::from((&(*node)).children[side]);
		}

		subsectors.wrapping_add(nodenum & !NF_SUBSECTOR)
	}
}

// R_SetupFrame
#[allow(static_mut_refs)]
fn R_SetupFrame(player: &mut player_t) {
	unsafe {
		viewplayer = player;
		viewx = (*player.mo).x;
		viewy = (*player.mo).y;
		viewangle = (*player.mo).angle
			+ Wrapping(isize::try_from(viewangleoffset).unwrap().cast_unsigned());
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
		R_RenderBSPNode(isize::try_from(numnodes).unwrap() - 1);

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
