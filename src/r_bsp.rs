//	BSP traversal, handling of LineSegs for rendering.
#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::{
	mem,
	num::Wrapping,
	ptr::{self, null_mut},
};

use crate::{
	doomdata::NF_SUBSECTOR,
	m_bbox::{BOXBOTTOM, BOXLEFT, BOXRIGHT, BOXTOP},
	m_fixed::fixed_t,
	p_setup::{nodes, segs, subsectors},
	r_defs::{MAXDRAWSEGS, drawseg_t, line_t, sector_t, seg_t, side_t},
	r_draw::viewwidth,
	r_main::{
		R_PointOnSide, R_PointToAngle, clipangle, sscount, viewangle, viewangletox, viewx, viewy,
		viewz,
	},
	r_plane::{R_FindPlane, ceilingplane, floorplane},
	r_segs::{R_StoreWallRange, rw_angle1},
	r_sky::skyflatnum,
	r_things::R_AddSprites,
	tables::{ANG90, ANG180, ANGLETOFINESHIFT},
};

#[unsafe(no_mangle)]
pub static mut curline: *mut seg_t = null_mut();
#[unsafe(no_mangle)]
pub static mut sidedef: *mut side_t = null_mut();
#[unsafe(no_mangle)]
pub static mut linedef: *mut line_t = null_mut();
#[unsafe(no_mangle)]
pub static mut frontsector: *mut sector_t = null_mut();
#[unsafe(no_mangle)]
pub static mut backsector: *mut sector_t = null_mut();

#[unsafe(no_mangle)]
pub static mut drawsegs: [drawseg_t; MAXDRAWSEGS] = unsafe { mem::zeroed() };
#[unsafe(no_mangle)]
pub static mut ds_p: *mut drawseg_t = null_mut();

// R_ClearDrawSegs
#[allow(static_mut_refs)]
pub fn R_ClearDrawSegs() {
	unsafe {
		ds_p = drawsegs.as_mut_ptr();
	}
}

// ClipWallSegment
// Clips the given range of columns
// and includes it in the new clip list.
#[derive(Debug, Clone, Copy)]
struct cliprange_t {
	first: u32,
	last: u32,
}

const MAXSEGS: usize = 32;

// newend is one past the last valid seg
static mut newend: *mut cliprange_t = null_mut();
static mut solidsegs: [cliprange_t; MAXSEGS] = [cliprange_t { first: 0, last: 0 }; MAXSEGS];

// R_ClipSolidWallSegment
// Does handle solid walls,
//  e.g. single sided LineDefs (middle texture)
//  that entirely block the view.
#[allow(static_mut_refs)]
fn R_ClipSolidWallSegment(first: u32, last: u32) {
	unsafe {
		// Find the first range that touches the range
		//  (adjacent pixels are touching).
		let mut start = solidsegs.as_mut_ptr();
		while (*start).last < first.wrapping_sub(1) {
			start = start.wrapping_add(1);
		}

		if first < (*start).first {
			if last < (*start).first.wrapping_sub(1) {
				// Post is entirely visible (above start),
				//  so insert a new clippost.
				R_StoreWallRange(first, last);
				let mut next = newend;
				newend = newend.wrapping_add(1);

				while !ptr::eq(next, start) {
					*next = *next.wrapping_sub(1);
					next = next.wrapping_sub(1);
				}
				(*next).first = first;
				(*next).last = last;
				return;
			}

			// There is a fragment above *start.
			R_StoreWallRange(first, (*start).first.wrapping_sub(1));
			// Now adjust the clip size.
			(*start).first = first;
		}

		// Bottom contained in start?
		if last <= (*start).last {
			return;
		}

		let mut next = start;
		let mut crunch = false;
		while last >= (*(next.wrapping_add(1))).first.wrapping_sub(1) {
			// There is a fragment between two posts.
			R_StoreWallRange((*next).last + 1, (*(next.wrapping_add(1))).first - 1);
			next = next.wrapping_add(1);

			if last <= (*next).last {
				// Bottom is contained in next.
				// Adjust the clip size.
				(*start).last = (*next).last;
				crunch = true;
				break;
			}
		}

		if !crunch {
			// There is a fragment after *next.
			R_StoreWallRange((*next).last + 1, last);
			// Adjust the clip size.
			(*start).last = last;
		}

		// Remove start+1 to next from the clip list,
		// because start now covers their area.
		if next == start {
			// Post just extended past the bottom of one post.
			return;
		}

		while next != newend {
			next = next.wrapping_add(1);
			// Remove a post.
			start = start.wrapping_add(1);
			*start = *next;
		}

		newend = start.wrapping_add(1);
	}
}

// R_ClipPassWallSegment
// Clips the given range of columns,
//  but does not includes it in the clip list.
// Does handle windows,
//  e.g. LineDefs with upper and lower texture.
#[allow(static_mut_refs)]
fn R_ClipPassWallSegment(first: u32, last: u32) {
	unsafe {
		// cliprange_t*	start;

		// Find the first range that touches the range
		//  (adjacent pixels are touching).
		let mut start = solidsegs.as_mut_ptr();
		while (*start).last < first.wrapping_sub(1) {
			start = start.wrapping_add(1);
		}

		if first < (*start).first {
			if last < (*start).first - 1 {
				// Post is entirely visible (above start).
				R_StoreWallRange(first, last);
				return;
			}

			// There is a fragment above *start.
			R_StoreWallRange(first, (*start).first - 1);
		}

		// Bottom contained in start?
		if last <= (*start).last {
			return;
		}

		while last >= (*(start.wrapping_add(1))).first.wrapping_sub(1) {
			// There is a fragment between two posts.
			R_StoreWallRange((*start).last + 1, (*(start.wrapping_add(1))).first - 1);
			start = start.wrapping_add(1);

			if last <= (*start).last {
				return;
			}
		}

		// There is a fragment after *next.
		R_StoreWallRange((*start).last + 1, last);
	}
}

// R_ClearClipSegs
pub fn R_ClearClipSegs() {
	unsafe {
		solidsegs[0].first = 0x80000001;
		solidsegs[0].last = u32::MAX;
		solidsegs[1].first = u32::try_from(viewwidth).unwrap();
		solidsegs[1].last = 0x7fffffff;
		newend = &raw mut solidsegs[2];
	}
}

// R_AddLine
// Clips the given segment
// and adds any visible pieces to the line list.
fn R_AddLine(line: *mut seg_t) {
	unsafe {
		curline = line;

		// OPTIMIZE: quickly reject orthogonal back sides.
		let mut angle1 = R_PointToAngle((*(*line).v1).x, (*(*line).v1).y);
		let mut angle2 = R_PointToAngle((*(*line).v2).x, (*(*line).v2).y);

		// Clip to view edges.
		// OPTIMIZE: make constant out of 2*clipangle (FIELDOFVIEW).
		let span = angle1 - angle2;

		// Back side? I.e. backface culling?
		if span >= ANG180 {
			return;
		}

		// Global angle needed by segcalc.
		rw_angle1 = angle1;
		angle1 -= viewangle;
		angle2 -= viewangle;

		let mut tspan = angle1 + clipangle;
		if tspan > Wrapping(2) * clipangle {
			tspan -= Wrapping(2) * clipangle;

			// Totally off the left edge?
			if tspan >= span {
				return;
			}

			angle1 = clipangle;
		}
		tspan = clipangle - angle2;
		if tspan > Wrapping(2) * clipangle {
			tspan -= Wrapping(2) * clipangle;

			// Totally off the left edge?
			if tspan >= span {
				return;
			}
			angle2 = -clipangle;
		}

		// The seg is in the view range,
		// but not necessarily visible.
		angle1 = (angle1 + ANG90) >> ANGLETOFINESHIFT;
		angle2 = (angle2 + ANG90) >> ANGLETOFINESHIFT;
		let x1 = viewangletox[angle1.0];
		let x2 = viewangletox[angle2.0];

		// Does not cross a pixel?
		if x1 == x2 {
			return;
		}

		backsector = (*line).backsector;

		// Single sided line?
		if backsector.is_null() {
			R_ClipSolidWallSegment(x1, x2 - 1);
			return;
		}

		// Closed door.
		if (*backsector).ceilingheight <= (*frontsector).floorheight
			|| (*backsector).floorheight >= (*frontsector).ceilingheight
		{
			R_ClipSolidWallSegment(x1, x2 - 1);
			return;
		}

		// Window.
		if (*backsector).ceilingheight != (*frontsector).ceilingheight
			|| (*backsector).floorheight != (*frontsector).floorheight
		{
			R_ClipPassWallSegment(x1, x2 - 1);
			return;
		}

		// Reject empty lines used for triggers
		//  and special events.
		// Identical floor and ceiling on both sides,
		// identical light levels on both sides,
		// and no middle texture.
		if (*backsector).ceilingpic == (*frontsector).ceilingpic
			&& (*backsector).floorpic == (*frontsector).floorpic
			&& (*backsector).lightlevel == (*frontsector).lightlevel
			&& (*(*curline).sidedef).midtexture == 0
		{
			return;
		}

		R_ClipPassWallSegment(x1, x2 - 1);
	}
}

// R_CheckBBox
// Checks BSP node/subtree bounding box.
// Returns true
//  if some part of the bbox might be visible.
static mut checkcoord: [[usize; 4]; 12] = [
	[3, 0, 2, 1],
	[3, 0, 2, 0],
	[3, 1, 2, 0],
	[0, 0, 0, 0],
	[2, 0, 2, 1],
	[0, 0, 0, 0],
	[3, 1, 3, 0],
	[0, 0, 0, 0],
	[2, 0, 3, 1],
	[2, 1, 3, 1],
	[2, 1, 3, 0],
	[0, 0, 0, 0],
];

#[allow(static_mut_refs)]
fn R_CheckBBox(bspcoord: *const fixed_t) -> bool {
	unsafe {
		// Find the corners of the box
		// that define the edges from current viewpoint.
		let boxx = if viewx <= *bspcoord.wrapping_add(BOXLEFT) {
			0
		} else if viewx < *bspcoord.wrapping_add(BOXRIGHT) {
			1
		} else {
			2
		};

		let boxy = if viewy >= *bspcoord.wrapping_add(BOXTOP) {
			0
		} else if viewy > *bspcoord.wrapping_add(BOXBOTTOM) {
			1
		} else {
			2
		};

		let boxpos = (boxy << 2) + boxx;
		if boxpos == 5 {
			return true;
		}

		let x1 = *bspcoord.wrapping_add(checkcoord[boxpos][0]);
		let y1 = *bspcoord.wrapping_add(checkcoord[boxpos][1]);
		let x2 = *bspcoord.wrapping_add(checkcoord[boxpos][2]);
		let y2 = *bspcoord.wrapping_add(checkcoord[boxpos][3]);

		// check clip list for an open space
		let mut angle1 = R_PointToAngle(x1, y1) - viewangle;
		let mut angle2 = R_PointToAngle(x2, y2) - viewangle;

		let span = angle1 - angle2;

		// Sitting on a line?
		if span >= ANG180 {
			return true;
		}

		let mut tspan = angle1 + clipangle;

		if tspan > Wrapping(2) * clipangle {
			tspan -= Wrapping(2) * clipangle;

			// Totally off the left edge?
			if tspan >= span {
				return false;
			}

			angle1 = clipangle;
		}
		tspan = clipangle - angle2;
		if tspan > Wrapping(2) * clipangle {
			tspan -= Wrapping(2) * clipangle;

			// Totally off the left edge?
			if tspan >= span {
				return false;
			}

			angle2 = -clipangle;
		}

		// Find the first clippost
		//  that touches the source post
		//  (adjacent pixels are touching).
		angle1 = (angle1 + ANG90) >> ANGLETOFINESHIFT;
		angle2 = (angle2 + ANG90) >> ANGLETOFINESHIFT;
		let sx1 = viewangletox[angle1.0];
		let mut sx2 = viewangletox[angle2.0];

		// Does not cross a pixel.
		if sx1 == sx2 {
			return false;
		}
		sx2 -= 1;

		let mut start = solidsegs.as_mut_ptr();
		while (*start).last < sx2 {
			start = start.wrapping_add(1);
		}

		// False if the clippost contains the new span.
		sx1 < (*start).first || sx2 > (*start).last
	}
}

// R_Subsector
// Determine floor/ceiling planes.
// Add sprites of things in sector.
// Draw one or more line segments.
fn R_Subsector(num: usize) {
	unsafe {
		sscount += 1;
		let sub = subsectors.wrapping_add(num);
		frontsector = (*sub).sector;
		let count = (*sub).numlines;
		let mut line = segs.wrapping_add(usize::try_from((*sub).firstline).unwrap());

		if (*frontsector).floorheight < viewz {
			floorplane = R_FindPlane(
				(*frontsector).floorheight,
				usize::try_from((*frontsector).floorpic).unwrap(),
				i32::from((*frontsector).lightlevel),
			);
		} else {
			floorplane = null_mut();
		}

		if (*frontsector).ceilingheight > viewz
			|| usize::try_from((*frontsector).ceilingpic).unwrap() == skyflatnum
		{
			ceilingplane = R_FindPlane(
				(*frontsector).ceilingheight,
				usize::try_from((*frontsector).ceilingpic).unwrap(),
				i32::from((*frontsector).lightlevel),
			);
		} else {
			ceilingplane = null_mut();
		}

		R_AddSprites(&mut *frontsector);

		for _ in 0..count {
			R_AddLine(line);
			line = line.wrapping_add(1);
		}
	}
}

// RenderBSPNode
// Renders all subsectors below a given node,
//  traversing subtree recursively.
// Just call with BSP root.
pub fn R_RenderBSPNode(bspnum: isize) {
	unsafe {
		// Found a subsector?
		if usize::try_from(bspnum).unwrap() & NF_SUBSECTOR != 0 {
			if bspnum == -1 {
				R_Subsector(0);
			} else {
				R_Subsector(usize::try_from(bspnum).unwrap() & !NF_SUBSECTOR);
			}
			return;
		}

		let bsp = nodes.wrapping_add(usize::try_from(bspnum).unwrap());

		// Decide which side the view point is on.
		let side = R_PointOnSide(viewx, viewy, &*bsp);

		// Recursively divide front space.
		R_RenderBSPNode(isize::try_from((&(*bsp).children)[side]).unwrap());

		// Possibly divide back space.
		if R_CheckBBox((&(*bsp).bbox)[side.flip()].as_ptr()) {
			R_RenderBSPNode(isize::try_from((&(*bsp).children)[side.flip()]).unwrap());
		}
	}
}
