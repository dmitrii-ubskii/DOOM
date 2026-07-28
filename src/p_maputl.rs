//	Movement/collision utility functions,
//	as used by function in p_map.c.
//	BLOCKMAP Iterator functions,
//	and some PIT_* functions to use for iteration.
#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::ptr::null_mut;

use crate::{
	m_bbox::{BOXBOTTOM, BOXLEFT, BOXRIGHT, BOXTOP},
	m_fixed::{FRACBITS, FRACUNIT, FixedDiv, FixedMul, fixed_t},
	p_local::{
		MAPBLOCKSHIFT, MAPBLOCKSIZE, MAPBTOFRAC, MAXINTERCEPTS, PT_ADDLINES, PT_ADDTHINGS,
		PT_EARLYOUT, divline_t, intercept_t, intercept_t_union,
	},
	p_mobj::{MF_NOBLOCKMAP, MF_NOSECTOR, mobj_t},
	p_setup::{
		blocklinks, blockmap, blockmaplump, bmapheight, bmaporgx, bmaporgy, bmapwidth, lines,
	},
	r_defs::{line_t, slopetype_t},
	r_main::{R_PointInSubsector, validcount},
};

// P_AproxDistance
// Gives an estimation of distance (not exact)
pub(crate) fn P_AproxDistance(dx: fixed_t, dy: fixed_t) -> fixed_t {
	let dx = fixed_t::abs(dx);
	let dy = fixed_t::abs(dy);
	dx + dy - (fixed_t::min(dx, dy) >> 1)
}

// P_PointOnLineSide
// Returns 0 or 1
// false: front side, true: back side
pub(crate) fn P_PointOnLineSide(x: fixed_t, y: fixed_t, line: &line_t) -> bool {
	unsafe {
		if line.dx == 0 {
			if x <= (*line.v1).x {
				return line.dy > 0;
			} else {
				return line.dy < 0;
			}
		}
		if line.dy == 0 {
			if y <= (*line.v1).y {
				return line.dx < 0;
			} else {
				return line.dx > 0;
			}
		}

		let dx = x - (*line.v1).x;
		let dy = y - (*line.v1).y;

		let left = FixedMul(line.dy >> FRACBITS, dx);
		let right = FixedMul(dy, line.dx >> FRACBITS);

		right >= left
	}
}

// P_BoxOnLineSide
// Considers the line to be infinite
// Returns side false or true, None if box crosses the line.
pub(crate) fn P_BoxOnLineSide(tmbox: &[fixed_t], ld: &line_t) -> Option<bool> {
	unsafe {
		let mut p1;
		let mut p2;

		match ld.slopetype {
			slopetype_t::ST_HORIZONTAL => {
				p1 = tmbox[BOXTOP] > (*ld.v1).y;
				p2 = tmbox[BOXBOTTOM] > (*ld.v1).y;
				if ld.dx < 0 {
					p1 ^= true;
					p2 ^= true;
				}
			}

			slopetype_t::ST_VERTICAL => {
				p1 = tmbox[BOXRIGHT] < (*ld.v1).x;
				p2 = tmbox[BOXLEFT] < (*ld.v1).x;
				if ld.dy < 0 {
					p1 ^= true;
					p2 ^= true;
				}
			}

			slopetype_t::ST_POSITIVE => {
				p1 = P_PointOnLineSide(tmbox[BOXLEFT], tmbox[BOXTOP], ld);
				p2 = P_PointOnLineSide(tmbox[BOXRIGHT], tmbox[BOXBOTTOM], ld);
			}

			slopetype_t::ST_NEGATIVE => {
				p1 = P_PointOnLineSide(tmbox[BOXRIGHT], tmbox[BOXTOP], ld);
				p2 = P_PointOnLineSide(tmbox[BOXLEFT], tmbox[BOXBOTTOM], ld);
			}
		}

		(p1 == p2).then_some(p1)
	}
}

// P_PointOnDivlineSide
// false: front side, true: back side
fn P_PointOnDivlineSide(x: fixed_t, y: fixed_t, line: &divline_t) -> bool {
	if line.dx == 0 {
		if x <= line.x {
			return line.dy > 0;
		} else {
			return line.dy < 0;
		}
	}
	if line.dy == 0 {
		if y <= line.y {
			return line.dx < 0;
		} else {
			return line.dx > 0;
		}
	}

	let dx = x - line.x;
	let dy = y - line.y;

	// try to quickly decide by looking at sign bits
	if (line.dy ^ line.dx ^ dx ^ dy) < 0 {
		return (line.dy ^ dx) < 0; // (left is negative)
	}

	let left = FixedMul(line.dy >> 8, dx >> 8);
	let right = FixedMul(dy >> 8, line.dx >> 8);

	right >= left
}

// P_MakeDivline
fn P_MakeDivline(li: &line_t) -> divline_t {
	unsafe {
		let x = (*li.v1).x;
		let y = (*li.v1).y;
		let dx = li.dx;
		let dy = li.dy;
		divline_t { x, y, dx, dy }
	}
}

// P_InterceptVector
// Returns the fractional intercept point
// along the first divline.
// This is only called by the addthings
// and addlines traversers.
fn P_InterceptVector(v2: &divline_t, v1: &divline_t) -> fixed_t {
	let den = FixedMul(v1.dy >> 8, v2.dx).wrapping_sub(FixedMul(v1.dx >> 8, v2.dy));

	if den == 0 {
		return 0;
	}

	let num = FixedMul((v1.x - v2.x) >> 8, v1.dy) + FixedMul((v2.y - v1.y) >> 8, v1.dx);

	FixedDiv(num, den)
}

// P_LineOpening
// Sets opentop and openbottom to the window
// through a two sided line.
// OPTIMIZE: keep this precalculated
pub(crate) static mut opentop: fixed_t = 0;
pub(crate) static mut openbottom: fixed_t = 0;
pub(crate) static mut openrange: fixed_t = 0;
pub(crate) static mut lowfloor: fixed_t = 0;

pub(crate) fn P_LineOpening(linedef: &line_t) {
	unsafe {
		if linedef.sidenum[1] == -1 {
			// single sided line
			openrange = 0;
			return;
		}

		let front = &*linedef.frontsector;
		let back = &*linedef.backsector;

		if front.ceilingheight < back.ceilingheight {
			opentop = front.ceilingheight;
		} else {
			opentop = back.ceilingheight;
		}

		if front.floorheight > back.floorheight {
			openbottom = front.floorheight;
			lowfloor = back.floorheight;
		} else {
			openbottom = back.floorheight;
			lowfloor = front.floorheight;
		}

		openrange = opentop - openbottom;
	}
}

// THING POSITION SETTING

// P_UnsetThingPosition
// Unlinks a thing from block map and sectors.
// On each position change, BLOCKMAP and other
// lookups maintaining lists ot things inside
// these structures need to be updated.
pub(crate) fn P_UnsetThingPosition(thing: &mut mobj_t) {
	unsafe {
		if thing.flags & MF_NOSECTOR == 0 {
			// inert things don't need to be in blockmap?
			// unlink from subsector
			if !thing.snext.is_null() {
				(*thing.snext).sprev = thing.sprev;
			}

			if !thing.sprev.is_null() {
				(*thing.sprev).snext = thing.snext;
			} else {
				(*(*thing.subsector).sector).thinglist = thing.snext;
			}
		}

		if thing.flags & MF_NOBLOCKMAP == 0 {
			// inert things don't need to be in blockmap
			// unlink from block map
			if !thing.bnext.is_null() {
				(*thing.bnext).bprev = thing.bprev;
			}

			if !thing.bprev.is_null() {
				(*thing.bprev).bnext = thing.bnext;
			} else {
				let blockx = (thing.x - bmaporgx) >> MAPBLOCKSHIFT;
				let blocky = (thing.y - bmaporgy) >> MAPBLOCKSHIFT;

				if blockx >= 0 && blocky >= 0 {
					let blockx = usize::try_from(blockx).unwrap();
					let blocky = usize::try_from(blocky).unwrap();
					if blockx < bmapwidth && blocky < bmapheight {
						*blocklinks.wrapping_add(blocky * bmapwidth + blockx) = thing.bnext;
					}
				}
			}
		}
	}
}

// P_SetThingPosition
// Links a thing into both a block and a subsector
// based on it's x y.
// Sets thing->subsector properly
pub(crate) fn P_SetThingPosition(thing: &mut mobj_t) {
	unsafe {
		// link into subsector
		let ss = R_PointInSubsector(thing.x, thing.y);
		thing.subsector = ss;

		if (thing.flags & MF_NOSECTOR) == 0 {
			// invisible things don't go into the sector links
			let sec = (*ss).sector;

			thing.sprev = null_mut();
			thing.snext = (*sec).thinglist;

			if !(*sec).thinglist.is_null() {
				(*(*sec).thinglist).sprev = thing;
			}

			(*sec).thinglist = thing;
		}

		// link into blockmap
		if thing.flags & MF_NOBLOCKMAP == 0 {
			// inert things don't need to be in blockmap
			let blockx = (thing.x - bmaporgx) >> MAPBLOCKSHIFT;
			let blocky = (thing.y - bmaporgy) >> MAPBLOCKSHIFT;

			if blockx >= 0
				&& (usize::try_from(blockx).unwrap()) < bmapwidth
				&& blocky >= 0
				&& (usize::try_from(blocky).unwrap()) < bmapheight
			{
				let blockx = usize::try_from(blockx).unwrap();
				let blocky = usize::try_from(blocky).unwrap();
				let link = blocklinks.wrapping_add(blocky * bmapwidth + blockx);
				thing.bprev = null_mut();
				thing.bnext = *link;
				if !(*link).is_null() {
					(**link).bprev = thing;
				}

				*link = thing;
			} else {
				// thing is off the map
				thing.bnext = null_mut();
				thing.bprev = null_mut();
			}
		}
	}
}

// BLOCK MAP ITERATORS
// For each line/thing in the given mapblock,
// call the passed PIT_* function.
// If the function returns false,
// exit with false without checking anything else.

// P_BlockLinesIterator
// The validcount flags are used to avoid checking lines
// that are marked in multiple mapblocks,
// so increment validcount before the first call
// to P_BlockLinesIterator, then make one or more calls
// to it.
pub(crate) fn P_BlockLinesIterator(x: i32, y: i32, func: fn(&mut line_t) -> bool) -> bool {
	unsafe {
		if x < 0
			|| y < 0 || (usize::try_from(x).unwrap()) >= bmapwidth
			|| (usize::try_from(y).unwrap()) >= bmapheight
		{
			return true;
		}

		let offset = usize::try_from(y).unwrap() * bmapwidth + usize::try_from(x).unwrap();
		let offset = *blockmap.wrapping_add(offset);

		// for ( list = blockmaplump+offset ; *list != -1 ; list++)
		let mut list = blockmaplump.wrapping_add(usize::try_from(offset).unwrap());
		while *list != -1 {
			let ld = &mut *lines.wrapping_add(usize::try_from(*list).unwrap());

			if ld.validcount != validcount {
				// line hasn't been checked yet
				ld.validcount = validcount;

				if !func(ld) {
					return false;
				}
			}
			list = list.wrapping_add(1);
		}
		true // everything was checked
	}
}

// P_BlockThingsIterator
pub(crate) fn P_BlockThingsIterator(x: i32, y: i32, func: fn(&mut mobj_t) -> bool) -> bool {
	unsafe {
		if x < 0
			|| y < 0 || usize::try_from(x).unwrap() >= bmapwidth
			|| usize::try_from(y).unwrap() >= bmapheight
		{
			return true;
		}

		let mut mobj = *blocklinks
			.wrapping_add(usize::try_from(y).unwrap() * bmapwidth + usize::try_from(x).unwrap());
		while !mobj.is_null() {
			if !func(&mut *mobj) {
				return false;
			}
			mobj = (*mobj).bnext;
		}
		true
	}
}

// INTERCEPT ROUTINES
static mut intercepts: [intercept_t; MAXINTERCEPTS] =
	[intercept_t { frac: 0, isaline: 0, d: intercept_t_union { thing: null_mut() } };
		MAXINTERCEPTS];
static mut intercept_p: *mut intercept_t = null_mut();

pub(crate) static mut trace: divline_t = divline_t { x: 0, y: 0, dx: 0, dy: 0 };
static mut earlyout: bool = false;

// PIT_AddLineIntercepts.
// Looks for lines in the given block
// that intercept the given trace
// to add to the intercepts list.
//
// A line is crossed if its endpoints
// are on opposite sides of the trace.
// Returns true if earlyout and a solid line hit.
#[allow(static_mut_refs)]
fn PIT_AddLineIntercepts(ld: &mut line_t) -> bool {
	unsafe {
		// avoid precision problems with two routines
		let s1;
		let s2;
		if trace.dx > FRACUNIT * 16
			|| trace.dy > FRACUNIT * 16
			|| trace.dx < -FRACUNIT * 16
			|| trace.dy < -FRACUNIT * 16
		{
			s1 = P_PointOnDivlineSide((*ld.v1).x, (*ld.v1).y, &trace);
			s2 = P_PointOnDivlineSide((*ld.v2).x, (*ld.v2).y, &trace);
		} else {
			s1 = P_PointOnLineSide(trace.x, trace.y, ld);
			s2 = P_PointOnLineSide(trace.x + trace.dx, trace.y + trace.dy, ld);
		}

		if s1 == s2 {
			return true; // line isn't crossed
		}

		// hit the line
		let dl = P_MakeDivline(ld);
		let frac = P_InterceptVector(&trace, &dl);

		if frac < 0 {
			return true; // behind source
		}

		// try to early out the check
		if earlyout && frac < FRACUNIT && ld.backsector.is_null() {
			return false; // stop checking
		}

		(*intercept_p).frac = frac;
		(*intercept_p).isaline = 1;
		(*intercept_p).d.line = ld;
		intercept_p = intercept_p.wrapping_add(1);

		true // continue
	}
}

// PIT_AddThingIntercepts
#[allow(static_mut_refs)]
fn PIT_AddThingIntercepts(thing: &mut mobj_t) -> bool {
	unsafe {
		let tracepositive = (trace.dx ^ trace.dy) > 0;

		// check a corner to corner crossection for hit
		let ((x1, y1), (x2, y2));

		if tracepositive {
			x1 = thing.x - thing.radius;
			y1 = thing.y + thing.radius;

			x2 = thing.x + thing.radius;
			y2 = thing.y - thing.radius;
		} else {
			x1 = thing.x - thing.radius;
			y1 = thing.y - thing.radius;

			x2 = thing.x + thing.radius;
			y2 = thing.y + thing.radius;
		}

		let s1 = P_PointOnDivlineSide(x1, y1, &trace);
		let s2 = P_PointOnDivlineSide(x2, y2, &trace);

		if s1 == s2 {
			return true; // line isn't crossed
		}

		let dl = divline_t { x: x1, y: y1, dx: x2 - x1, dy: y2 - y1 };

		let frac = P_InterceptVector(&trace, &dl);

		if frac < 0 {
			return true; // behind source
		}

		(*intercept_p).frac = frac;
		(*intercept_p).isaline = 0;
		(*intercept_p).d.thing = thing;
		intercept_p = intercept_p.wrapping_add(1);

		true // keep going
	}
}

// P_TraverseIntercepts
// Returns true if the traverser function returns true
// for all lines.
#[allow(static_mut_refs)]
fn P_TraverseIntercepts(func: fn(&mut intercept_t) -> bool, maxfrac: fixed_t) -> bool {
	unsafe {
		let mut count = intercept_p.offset_from(intercepts.as_ptr());
		while count != 0 {
			count -= 1;
			let mut intercept = None;
			let mut dist = i32::MAX;
			let mut scan = intercepts.as_mut_ptr();
			while scan < intercept_p {
				if (*scan).frac < dist {
					dist = (*scan).frac;
					intercept = Some(scan);
				}
				scan = scan.wrapping_add(1);
			}

			if dist > maxfrac {
				return true; // checked everything intercept range
			}

			let intercept = intercept.unwrap();

			if !func(&mut *intercept) {
				return false; // don't bother going farther
			}

			(*intercept).frac = i32::MAX;
		}

		true // everything was traversed
	}
}

// P_PathTraverse
// Traces a line from x1,y1 to x2,y2,
// calling the traverser function for each.
// Returns true if the traverser function returns true
// for all lines.
#[allow(static_mut_refs)]
pub(crate) fn P_PathTraverse(
	mut x1: fixed_t,
	mut y1: fixed_t,
	mut x2: fixed_t,
	mut y2: fixed_t,
	flags: i32,
	trav: fn(&mut intercept_t) -> bool,
) -> bool {
	unsafe {
		earlyout = flags & PT_EARLYOUT != 0;

		validcount += 1;
		intercept_p = intercepts.as_mut_ptr();

		if (x1 - bmaporgx) & (MAPBLOCKSIZE - 1) == 0 {
			x1 += FRACUNIT; // don't side exactly on a line
		}

		if (y1 - bmaporgy) & (MAPBLOCKSIZE - 1) == 0 {
			y1 += FRACUNIT; // don't side exactly on a line
		}

		trace.x = x1;
		trace.y = y1;
		trace.dx = x2 - x1;
		trace.dy = y2 - y1;

		x1 -= bmaporgx;
		y1 -= bmaporgy;
		let xt1 = x1 >> MAPBLOCKSHIFT;
		let yt1 = y1 >> MAPBLOCKSHIFT;

		x2 -= bmaporgx;
		y2 -= bmaporgy;
		let xt2 = x2 >> MAPBLOCKSHIFT;
		let yt2 = y2 >> MAPBLOCKSHIFT;

		let mapxstep;
		let partial;
		let ystep;
		if xt2 > xt1 {
			mapxstep = 1;
			partial = FRACUNIT - ((x1 >> MAPBTOFRAC) & (FRACUNIT - 1));
			ystep = FixedDiv(y2 - y1, i32::abs(x2 - x1));
		} else if xt2 < xt1 {
			mapxstep = -1;
			partial = (x1 >> MAPBTOFRAC) & (FRACUNIT - 1);
			ystep = FixedDiv(y2 - y1, i32::abs(x2 - x1));
		} else {
			mapxstep = 0;
			partial = FRACUNIT;
			ystep = 256 * FRACUNIT;
		}

		let mut yintercept = (y1 >> MAPBTOFRAC) + FixedMul(partial, ystep);

		let mapystep;
		let partial;
		let xstep;
		if yt2 > yt1 {
			mapystep = 1;
			partial = FRACUNIT - ((y1 >> MAPBTOFRAC) & (FRACUNIT - 1));
			xstep = FixedDiv(x2 - x1, i32::abs(y2 - y1));
		} else if yt2 < yt1 {
			mapystep = -1;
			partial = (y1 >> MAPBTOFRAC) & (FRACUNIT - 1);
			xstep = FixedDiv(x2 - x1, i32::abs(y2 - y1));
		} else {
			mapystep = 0;
			partial = FRACUNIT;
			xstep = 256 * FRACUNIT;
		}
		let mut xintercept = (x1 >> MAPBTOFRAC) + FixedMul(partial, xstep);

		// Step through map blocks.
		// Count is present to prevent a round off error
		// from skipping the break.
		let mut mapx = xt1;
		let mut mapy = yt1;

		for _ in 0..64 {
			if flags & PT_ADDLINES != 0 {
				if !P_BlockLinesIterator(mapx, mapy, PIT_AddLineIntercepts) {
					return false; // early out
				}
			}

			if flags & PT_ADDTHINGS != 0 {
				if !P_BlockThingsIterator(mapx, mapy, PIT_AddThingIntercepts) {
					return false; // early out
				}
			}

			if mapx == xt2 && mapy == yt2 {
				break;
			}

			if (yintercept >> FRACBITS) == mapy {
				yintercept += ystep;
				mapx += mapxstep;
			} else if (xintercept >> FRACBITS) == mapx {
				xintercept += xstep;
				mapy += mapystep;
			}
		}
		// go through the sorted list
		P_TraverseIntercepts(trav, FRACUNIT)
	}
}
