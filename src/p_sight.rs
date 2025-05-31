#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::cmp::Ordering;

use crate::{
	doomdata::{ML_TWOSIDED, NF_SUBSECTOR},
	i_system::I_Error,
	m_fixed::{FRACBITS, FixedDiv, FixedMul, fixed_t},
	p_local::{divline_t, openbottom, opentop, rejectmatrix},
	p_mobj::mobj_t,
	r_state::{nodes, numnodes, numsectors, numsubsectors, sectors, segs, subsectors},
};

// P_CheckSight
#[unsafe(no_mangle)]
pub static mut sightzstart: fixed_t = 0; // eye z of looker
#[unsafe(no_mangle)]
pub static mut topslope: fixed_t = 0;
#[unsafe(no_mangle)]
pub static mut bottomslope: fixed_t = 0; // slopes to top and bottom of target

pub static mut strace: divline_t = divline_t { x: 0, y: 0, dx: 0, dy: 0 }; // from t1 to t2
#[unsafe(no_mangle)]
pub static mut t2x: fixed_t = 0;
#[unsafe(no_mangle)]
pub static mut t2y: fixed_t = 0;

#[unsafe(no_mangle)]
pub static mut sightcounts: [i32; 2] = [0; 2];

// P_DivlineSide
// Returns side 0 (front), 1 (back), or 2 (on).
#[unsafe(no_mangle)]
pub extern "C" fn P_DivlineSide(x: fixed_t, y: fixed_t, node: &mut divline_t) -> i32 {
	if node.dx == 0 {
		if x == node.x {
			return 2;
		} else if x < node.x {
			return (node.dy > 0) as i32;
		} else {
			return (node.dy < 0) as i32;
		}
	}

	if node.dy == 0 {
		// v~~~ TODO bug?
		if x == node.y {
			return 2;
		} else if y < node.y {
			return (node.dx < 0) as i32;
		} else {
			return (node.dx > 0) as i32;
		}
	}

	let dx = x - node.x;
	let dy = y - node.y;

	let left = (node.dy >> FRACBITS) * (dx >> FRACBITS);
	let right = (dy >> FRACBITS) * (node.dx >> FRACBITS);

	match left.cmp(&right) {
		Ordering::Less => 1, // back side
		Ordering::Equal => 2,
		Ordering::Greater => 0, // front side
	}
}

// P_InterceptVector2
// Returns the fractional intercept point
// along the first divline.
// This is only called by the addthings and addlines traversers.
#[unsafe(no_mangle)]
pub extern "C" fn P_InterceptVector2(v2: &mut divline_t, v1: &mut divline_t) -> fixed_t {
	let den = FixedMul(v1.dy >> 8, v2.dx) - FixedMul(v1.dx >> 8, v2.dy);
	if den == 0 {
		//	I_Error ("P_InterceptVector: parallel");
		return 0;
	}
	let num = FixedMul((v1.x - v2.x) >> 8, v1.dy) + FixedMul((v2.y - v1.y) >> 8, v1.dx);
	FixedDiv(num, den)
}

unsafe extern "C" {
	static mut validcount: i32;
}

// P_CrossSubsector
// Returns true
//  if strace crosses the given subsector successfully.
#[unsafe(no_mangle)]
pub extern "C" fn P_CrossSubsector(num: i32) -> i32 {
	let mut divl = divline_t { x: 0, y: 0, dx: 0, dy: 0 };

	unsafe {
		// #ifdef RANGECHECK
		// if num >= numsubsectors {
		// 	I_Error(c"P_CrossSubsector: ss %i with numss = %i".as_ptr(), num, numsubsectors);
		// }
		// #endif

		let sub = &mut *subsectors.wrapping_add(num as usize);

		// check lines
		let count = sub.numlines as usize;
		let segp = segs.wrapping_add(sub.firstline as usize);

		for i in 0..count {
			let seg = &mut *segp.wrapping_add(i);
			let line = &mut *seg.linedef;

			// allready checked other side?
			if line.validcount == validcount {
				continue;
			}

			line.validcount = validcount;

			let v1 = &mut *line.v1;
			let v2 = &mut *line.v2;
			#[allow(static_mut_refs)]
			let s1 = P_DivlineSide(v1.x, v1.y, &mut strace);
			#[allow(static_mut_refs)]
			let s2 = P_DivlineSide(v2.x, v2.y, &mut strace);

			// line isn't crossed?
			if s1 == s2 {
				continue;
			}

			divl.x = v1.x;
			divl.y = v1.y;
			divl.dx = v2.x - v1.x;
			divl.dy = v2.y - v1.y;
			let s1 = P_DivlineSide(strace.x, strace.y, &mut divl);
			let s2 = P_DivlineSide(t2x, t2y, &mut divl);

			// line isn't crossed?
			if s1 == s2 {
				continue;
			}

			// stop because it is not two sided anyway
			// might do this after updating validcount?
			if line.flags & ML_TWOSIDED == 0 {
				return 0;
			}

			// crosses a two sided line
			let front = &mut *seg.frontsector;
			let back = &mut *seg.backsector;

			// no wall to block sight with?
			if front.floorheight == back.floorheight && front.ceilingheight == back.ceilingheight {
				continue;
			}

			// possible occluder
			// because of ceiling height differences
			if front.ceilingheight < back.ceilingheight {
				opentop = front.ceilingheight;
			} else {
				opentop = back.ceilingheight;
			}

			// because of ceiling height differences
			if front.floorheight > back.floorheight {
				openbottom = front.floorheight;
			} else {
				openbottom = back.floorheight;
			}

			// quick test for totally closed doors
			if openbottom >= opentop {
				return 0; // stop
			}

			#[allow(static_mut_refs)]
			let frac = P_InterceptVector2(&mut strace, &mut divl);

			if front.floorheight != back.floorheight {
				let slope = FixedDiv(openbottom - sightzstart, frac);
				if slope > bottomslope {
					bottomslope = slope;
				}
			}

			if front.ceilingheight != back.ceilingheight {
				let slope = FixedDiv(opentop - sightzstart, frac);
				if slope < topslope {
					topslope = slope;
				}
			}

			if topslope <= bottomslope {
				return 0; // stop
			}
		}

		// passed the subsector ok
		1
	}
}

// P_CrossBSPNode
// Returns true
//  if strace crosses the given node successfully.
#[unsafe(no_mangle)]
pub extern "C" fn P_CrossBSPNode(bspnum: usize) -> i32 {
	unsafe {
		if bspnum & NF_SUBSECTOR != 0 {
			if bspnum == usize::MAX {
				return P_CrossSubsector(0);
			} else {
				return P_CrossSubsector((bspnum & !NF_SUBSECTOR) as i32);
			}
		}

		let bsp = nodes.add(bspnum);

		// decide which side the start point is on
		let mut side = P_DivlineSide(strace.x, strace.y, &mut *(bsp as *mut divline_t));
		if side == 2 {
			side = 0; // an "on" should cross both sides
		}

		// cross the starting side
		if (P_CrossBSPNode((*bsp).children[side as usize] as usize)) == 0 {
			return 0;
		}

		// the partition plane is crossed here
		if side == P_DivlineSide(t2x, t2y, &mut *(bsp as *mut divline_t)) {
			// the line doesn't touch the other side
			return 1;
		}

		// cross the ending side
		P_CrossBSPNode((*bsp).children[side as usize ^ 1] as usize)
	}
}

// P_CheckSight
// Returns true
//  if a straight line between t1 and t2 is unobstructed.
// Uses REJECT.
#[unsafe(no_mangle)]
pub extern "C" fn P_CheckSight(t1: &mut mobj_t, t2: &mut mobj_t) -> i32 {
	unsafe {
		// First check for trivial rejection.

		// Determine subsector entries in REJECT table.
		let s1 = (*t1.subsector).sector.offset_from(sectors);
		let s2 = (*t2.subsector).sector.offset_from(sectors);
		let pnum = s1 * numsectors as isize + s2;
		let bytenum = (pnum >> 3) as usize;
		let bitnum = 1 << (pnum & 7);

		// Check in REJECT table.
		if *rejectmatrix.wrapping_add(bytenum) & bitnum != 0 {
			sightcounts[0] += 1;

			// can't possibly be connected
			return 0;
		}

		// An unobstructed LOS is possible.
		// Now look from eyes of t1 to any part of t2.
		sightcounts[1] += 1;

		validcount += 1;

		sightzstart = t1.z + t1.height - (t1.height >> 2);
		topslope = (t2.z + t2.height) - sightzstart;
		bottomslope = (t2.z) - sightzstart;

		strace.x = t1.x;
		strace.y = t1.y;
		t2x = t2.x;
		t2y = t2.y;
		strace.dx = t2.x - t1.x;
		strace.dy = t2.y - t1.y;

		// the head node is the last node output
		P_CrossBSPNode(numnodes as usize - 1)
	}
}
