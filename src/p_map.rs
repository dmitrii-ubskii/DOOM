//	Movement, collision handling.
//	Shooting and aiming.
#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::ptr::{self, null_mut};

use crate::{
	d_player::player_t,
	doomdata::{ML_BLOCKING, ML_BLOCKMONSTERS, ML_TWOSIDED},
	g_game::gamemap,
	i_system::I_Error,
	info::{mobjtype_t, statenum_t},
	m_bbox::{BOXBOTTOM, BOXLEFT, BOXRIGHT, BOXTOP},
	m_fixed::{FRACBITS, FRACUNIT, FixedDiv, FixedMul, fixed_t},
	m_random::P_Random,
	p_inter::{P_DamageMobj, P_TouchSpecialThing},
	p_local::{MAPBLOCKSHIFT, MAXRADIUS, PT_ADDLINES, PT_ADDTHINGS, USERANGE, intercept_t},
	p_maputl::{
		P_AproxDistance, P_BlockLinesIterator, P_BlockThingsIterator, P_BoxOnLineSide,
		P_LineOpening, P_PathTraverse, P_PointOnLineSide, P_SetThingPosition, P_UnsetThingPosition,
		lowfloor, openbottom, openrange, opentop, trace,
	},
	p_mobj::{
		MF_DROPOFF, MF_DROPPED, MF_FLOAT, MF_MISSILE, MF_NOBLOOD, MF_NOCLIP, MF_PICKUP,
		MF_SHOOTABLE, MF_SKULLFLY, MF_SOLID, MF_SPECIAL, MF_TELEPORT, P_RemoveMobj, P_SetMobjState,
		P_SpawnBlood, P_SpawnMobj, P_SpawnPuff, mobj_t,
	},
	p_setup::{bmaporgx, bmaporgy, lines},
	p_sight::{P_CheckSight, bottomslope, topslope},
	p_spec::{P_CrossSpecialLine, P_ShootSpecialLine},
	p_switch::P_UseSpecialLine,
	p_tick::leveltime,
	r_defs::{line_t, sector_t, slopetype_t},
	r_main::{R_PointInSubsector, R_PointToAngle2, validcount},
	r_sky::skyflatnum,
	s_sound::S_StartSound,
	sounds::sfxenum_t,
	tables::{ANG180, ANGLETOFINESHIFT, angle_t, finecos, finesine},
};

static mut tmbbox: [fixed_t; 4] = [0; 4];
static mut tmthing: *mut mobj_t = null_mut();
static mut tmflags: u32 = 0;
static mut tmx: fixed_t = 0;
static mut tmy: fixed_t = 0;

// If "floatok" true, move would be ok
// if within "tmfloorz - tmceilingz".
pub static mut floatok: bool = false;

pub static mut tmfloorz: fixed_t = 0;
static mut tmceilingz: fixed_t = 0;
static mut tmdropoffz: fixed_t = 0;

// keep track of the line that lowers the ceiling,
// so missiles don't explode against sky hack walls
pub static mut ceilingline: *mut line_t = null_mut();

// keep track of special lines as they are hit,
// but don't process them until the move is proven valid
pub const MAXSPECIALCROSS: usize = 8;

pub static mut spechit: [*mut line_t; MAXSPECIALCROSS] = [null_mut(); MAXSPECIALCROSS];
pub static mut numspechit: usize = 0;

// TELEPORT MOVE

// PIT_StompThing
pub fn PIT_StompThing(thing: &mut mobj_t) -> bool {
	unsafe {
		if thing.flags & MF_SHOOTABLE == 0 {
			return true;
		}

		let blockdist = thing.radius + (*tmthing).radius;

		if i32::abs(thing.x - tmx) >= blockdist || i32::abs(thing.y - tmy) >= blockdist {
			// didn't hit it
			return true;
		}

		// don't clip against self
		if ptr::eq(thing, tmthing) {
			return true;
		}

		// monsters don't stomp things except on boss level
		if (*tmthing).player.is_null() && gamemap != 30 {
			return false;
		}

		P_DamageMobj(thing, tmthing, tmthing, 10000);

		true
	}
}

// P_TeleportMove
pub fn P_TeleportMove(thing: &mut mobj_t, x: fixed_t, y: fixed_t) -> bool {
	unsafe {
		// kill anything occupying the position
		tmthing = thing;
		tmflags = thing.flags;

		tmx = x;
		tmy = y;

		tmbbox[BOXTOP] = y + (*tmthing).radius;
		tmbbox[BOXBOTTOM] = y - (*tmthing).radius;
		tmbbox[BOXRIGHT] = x + (*tmthing).radius;
		tmbbox[BOXLEFT] = x - (*tmthing).radius;

		let newsubsec = R_PointInSubsector(x, y);
		ceilingline = null_mut();

		// The base floor/ceiling is from the subsector
		// that contains the point.
		// Any contacted lines the step closer together
		// will adjust them.
		tmfloorz = (*(*newsubsec).sector).floorheight;
		tmdropoffz = tmfloorz;
		tmceilingz = (*(*newsubsec).sector).ceilingheight;

		validcount += 1;
		numspechit = 0;

		// stomp on any things contacted
		let xl = (tmbbox[BOXLEFT] - bmaporgx - MAXRADIUS) >> MAPBLOCKSHIFT;
		let xh = (tmbbox[BOXRIGHT] - bmaporgx + MAXRADIUS) >> MAPBLOCKSHIFT;
		let yl = (tmbbox[BOXBOTTOM] - bmaporgy - MAXRADIUS) >> MAPBLOCKSHIFT;
		let yh = (tmbbox[BOXTOP] - bmaporgy + MAXRADIUS) >> MAPBLOCKSHIFT;

		for bx in xl..=xh {
			for by in yl..=yh {
				if !P_BlockThingsIterator(bx, by, PIT_StompThing) {
					return false;
				}
			}
		}

		// the move is ok,
		// so link the thing into its new position
		P_UnsetThingPosition(thing);

		thing.floorz = tmfloorz;
		thing.ceilingz = tmceilingz;
		thing.x = x;
		thing.y = y;

		P_SetThingPosition(thing);

		true
	}
}

// MOVEMENT ITERATOR FUNCTIONS

// PIT_CheckLine
// Adjusts tmfloorz and tmceilingz as lines are contacted
#[allow(static_mut_refs)]
pub fn PIT_CheckLine(ld: &mut line_t) -> bool {
	unsafe {
		if tmbbox[BOXRIGHT] <= ld.bbox[BOXLEFT]
			|| tmbbox[BOXLEFT] >= ld.bbox[BOXRIGHT]
			|| tmbbox[BOXTOP] <= ld.bbox[BOXBOTTOM]
			|| tmbbox[BOXBOTTOM] >= ld.bbox[BOXTOP]
		{
			return true;
		}

		if P_BoxOnLineSide(&tmbbox, ld).is_some() {
			return true;
		}

		// A line has been hit

		// The moving thing's destination position will cross
		// the given line.
		// If this should not be allowed, return false.
		// If the line is special, keep track of it
		// to process later if the move is proven ok.
		// NOTE: specials are NOT sorted by order,
		// so two special lines that are only 8 pixels apart
		// could be crossed in either order.

		if ld.backsector.is_null() {
			return false; // one sided line
		}

		if (*tmthing).flags & MF_MISSILE == 0 {
			if ld.flags & ML_BLOCKING != 0 {
				return false; // explicitly blocking everything
			}

			if (*tmthing).player.is_null() && ld.flags & ML_BLOCKMONSTERS != 0 {
				return false; // block monsters only
			}
		}

		// set openrange, opentop, openbottom
		P_LineOpening(ld);

		// adjust floor / ceiling heights
		if opentop < tmceilingz {
			tmceilingz = opentop;
			ceilingline = ld;
		}

		if openbottom > tmfloorz {
			tmfloorz = openbottom;
		}

		if lowfloor < tmdropoffz {
			tmdropoffz = lowfloor;
		}

		// if contacted a special line, add it to the list
		if ld.special != 0 {
			spechit[numspechit] = ld;
			numspechit += 1;
		}

		true
	}
}

// PIT_CheckThing
pub fn PIT_CheckThing(thing: &mut mobj_t) -> bool {
	unsafe {
		if thing.flags & (MF_SOLID | MF_SPECIAL | MF_SHOOTABLE) == 0 {
			return true;
		}

		let blockdist = thing.radius + (*tmthing).radius;

		if i32::abs(thing.x - tmx) >= blockdist || i32::abs(thing.y - tmy) >= blockdist {
			// didn't hit it
			return true;
		}

		// don't clip against self
		if ptr::eq(thing, tmthing) {
			return true;
		}

		// check for skulls slamming into things
		if (*tmthing).flags & MF_SKULLFLY != 0 {
			let damage = ((P_Random() % 8) + 1) * (*(*tmthing).info).damage;

			P_DamageMobj(thing, tmthing, tmthing, damage);

			(*tmthing).flags &= !MF_SKULLFLY;
			(*tmthing).momx = 0;
			(*tmthing).momy = 0;
			(*tmthing).momz = 0;

			P_SetMobjState(&mut *tmthing, (*(*tmthing).info).spawnstate);

			return false; // stop moving
		}

		// missiles can hit other things
		if (*tmthing).flags & MF_MISSILE != 0 {
			// see if it went over / under
			if (*tmthing).z > thing.z + thing.height {
				return true; // overhead
			}
			if (*tmthing).z + (*tmthing).height < thing.z {
				return true; // underneath
			}

			if !(*tmthing).target.is_null()
				&& ((*(*tmthing).target).ty == thing.ty
					|| ((*(*tmthing).target).ty == mobjtype_t::MT_KNIGHT
						&& thing.ty == mobjtype_t::MT_BRUISER)
					|| ((*(*tmthing).target).ty == mobjtype_t::MT_BRUISER
						&& thing.ty == mobjtype_t::MT_KNIGHT))
			{
				// Don't hit same species as originator.
				if ptr::eq(thing, (*tmthing).target) {
					return true;
				}

				if thing.ty != mobjtype_t::MT_PLAYER {
					// Explode, but do no damage.
					// Let players missile other players.
					return false;
				}
			}

			if thing.flags & MF_SHOOTABLE == 0 {
				// didn't do any damage
				return thing.flags & MF_SOLID == 0;
			}

			// damage / explode
			let damage = ((P_Random() % 8) + 1) * (*(*tmthing).info).damage;
			P_DamageMobj(thing, tmthing, (*tmthing).target, damage);

			// don't traverse any more
			return false;
		}

		// check for special pickup
		if thing.flags & MF_SPECIAL != 0 {
			let solid = (thing.flags & MF_SOLID) != 0;
			if tmflags & MF_PICKUP != 0 {
				// can remove thing
				P_TouchSpecialThing(thing, &mut *tmthing);
			}
			return !solid;
		}

		thing.flags & MF_SOLID == 0
	}
}

// MOVEMENT CLIPPING

// P_CheckPosition
// This is purely informative, nothing is modified
// (except things picked up).
//
// in:
//  a mobj_t (can be valid or invalid)
//  a position to be checked
//   (doesn't need to be related to the mobj_t->x,y)
//
// during:
//  special things are touched if MF_PICKUP
//  early out on solid lines?
//
// out:
//  newsubsec
//  floorz
//  ceilingz
//  tmdropoffz
//   the lowest point contacted
//   (monsters won't move to a dropoff)
//  speciallines[]
//  numspeciallines
pub fn P_CheckPosition(thing: &mut mobj_t, x: fixed_t, y: fixed_t) -> bool {
	unsafe {
		tmthing = thing;
		tmflags = thing.flags;

		tmx = x;
		tmy = y;

		tmbbox[BOXTOP] = y + (*tmthing).radius;
		tmbbox[BOXBOTTOM] = y - (*tmthing).radius;
		tmbbox[BOXRIGHT] = x + (*tmthing).radius;
		tmbbox[BOXLEFT] = x - (*tmthing).radius;

		let newsubsec = R_PointInSubsector(x, y);
		ceilingline = null_mut();

		// The base floor / ceiling is from the subsector
		// that contains the point.
		// Any contacted lines the step closer together
		// will adjust them.
		tmfloorz = (*(*newsubsec).sector).floorheight;
		tmdropoffz = tmfloorz;
		tmceilingz = (*(*newsubsec).sector).ceilingheight;

		validcount += 1;
		numspechit = 0;

		if tmflags & MF_NOCLIP != 0 {
			return true;
		}

		// Check things first, possibly picking things up.
		// The bounding box is extended by MAXRADIUS
		// because mobj_ts are grouped into mapblocks
		// based on their origin point, and can overlap
		// into adjacent blocks by up to MAXRADIUS units.
		let xl = (tmbbox[BOXLEFT] - bmaporgx - MAXRADIUS) >> MAPBLOCKSHIFT;
		let xh = (tmbbox[BOXRIGHT] - bmaporgx + MAXRADIUS) >> MAPBLOCKSHIFT;
		let yl = (tmbbox[BOXBOTTOM] - bmaporgy - MAXRADIUS) >> MAPBLOCKSHIFT;
		let yh = (tmbbox[BOXTOP] - bmaporgy + MAXRADIUS) >> MAPBLOCKSHIFT;

		for bx in xl..=xh {
			for by in yl..=yh {
				if !P_BlockThingsIterator(bx, by, PIT_CheckThing) {
					return false;
				}
			}
		}

		// check lines
		let xl = (tmbbox[BOXLEFT] - bmaporgx) >> MAPBLOCKSHIFT;
		let xh = (tmbbox[BOXRIGHT] - bmaporgx) >> MAPBLOCKSHIFT;
		let yl = (tmbbox[BOXBOTTOM] - bmaporgy) >> MAPBLOCKSHIFT;
		let yh = (tmbbox[BOXTOP] - bmaporgy) >> MAPBLOCKSHIFT;

		for bx in xl..=xh {
			for by in yl..=yh {
				if !P_BlockLinesIterator(bx, by, PIT_CheckLine) {
					return false;
				}
			}
		}

		true
	}
}

// P_TryMove
// Attempt to move to a new position,
// crossing special lines unless MF_TELEPORT is set.
pub fn P_TryMove(thing: &mut mobj_t, x: fixed_t, y: fixed_t) -> bool {
	unsafe {
		floatok = false;
		if !P_CheckPosition(thing, x, y) {
			return false; // solid wall or thing
		}

		if thing.flags & MF_NOCLIP == 0 {
			if tmceilingz - tmfloorz < thing.height {
				return false; // doesn't fit
			}

			floatok = true;

			if thing.flags & MF_TELEPORT == 0 && tmceilingz - thing.z < thing.height {
				return false; // mobj must lower itself to fit
			}

			if thing.flags & MF_TELEPORT == 0 && tmfloorz - thing.z > 24 * FRACUNIT {
				return false; // too big a step up
			}

			if thing.flags & (MF_DROPOFF | MF_FLOAT) == 0 && tmfloorz - tmdropoffz > 24 * FRACUNIT {
				return false; // don't stand over a dropoff
			}
		}

		// the move is ok,
		// so link the thing into its new position
		P_UnsetThingPosition(thing);

		let oldx = thing.x;
		let oldy = thing.y;
		thing.floorz = tmfloorz;
		thing.ceilingz = tmceilingz;
		thing.x = x;
		thing.y = y;

		P_SetThingPosition(thing);

		// if any special lines were hit, do the effect
		if thing.flags & (MF_TELEPORT | MF_NOCLIP) == 0 {
			while numspechit != 0 {
				numspechit -= 1;
				// see if the line was crossed
				let ld = spechit[numspechit];
				let side = P_PointOnLineSide(thing.x, thing.y, &*ld);
				let oldside = P_PointOnLineSide(oldx, oldy, &*ld);
				if side != oldside {
					if (*ld).special != 0 {
						P_CrossSpecialLine(ld.offset_from(lines) as usize, oldside as usize, thing);
					}
				}
			}
		}

		true
	}
}

// P_ThingHeightClip
// Takes a valid thing and adjusts the thing->floorz,
// thing->ceilingz, and possibly thing->z.
// This is called for all nearby monsters
// whenever a sector changes height.
// If the thing doesn't fit,
// the z will be set to the lowest value
// and false will be returned.
fn P_ThingHeightClip(thing: &mut mobj_t) -> bool {
	let onfloor = thing.z == thing.floorz;

	P_CheckPosition(thing, thing.x, thing.y);
	// what about stranding a monster partially off an edge?

	unsafe {
		thing.floorz = tmfloorz;
		thing.ceilingz = tmceilingz;
	}

	if onfloor {
		// walking monsters rise and fall with the floor
		thing.z = thing.floorz;
	} else {
		// don't adjust a floating monster unless forced to
		if thing.z + thing.height > thing.ceilingz {
			thing.z = thing.ceilingz - thing.height;
		}
	}

	thing.ceilingz - thing.floorz >= thing.height
}

// SLIDE MOVE
// Allows the player to slide along any angled walls.
static mut bestslidefrac: fixed_t = 0;
static mut secondslidefrac: fixed_t = 0;

static mut bestslideline: *mut line_t = null_mut();
static mut secondslideline: *mut line_t = null_mut();

static mut slidemo: *mut mobj_t = null_mut();

static mut tmxmove: fixed_t = 0;
static mut tmymove: fixed_t = 0;

// P_HitSlideLine
// Adjusts the xmove / ymove
// so that the next move will slide along the wall.
fn P_HitSlideLine(ld: &line_t) {
	unsafe {
		if ld.slopetype == slopetype_t::ST_HORIZONTAL {
			tmymove = 0;
			return;
		}

		if ld.slopetype == slopetype_t::ST_VERTICAL {
			tmxmove = 0;
			return;
		}

		let side = P_PointOnLineSide((*slidemo).x, (*slidemo).y, ld);

		let mut lineangle = R_PointToAngle2(0, 0, ld.dx, ld.dy);

		if side {
			lineangle += ANG180;
		}

		let moveangle = R_PointToAngle2(0, 0, tmxmove, tmymove);
		let mut deltaangle = moveangle - lineangle;

		if deltaangle > ANG180 {
			deltaangle += ANG180;
		}

		lineangle >>= ANGLETOFINESHIFT;
		deltaangle >>= ANGLETOFINESHIFT;

		let movelen = P_AproxDistance(tmxmove, tmymove);
		let newlen = FixedMul(movelen, finecos(deltaangle.0));

		tmxmove = FixedMul(newlen, finecos(lineangle.0));
		tmymove = FixedMul(newlen, finesine[lineangle.0]);
	}
}

// PTR_SlideTraverse
pub fn PTR_SlideTraverse(intercept: &mut intercept_t) -> bool {
	unsafe {
		if intercept.isaline == 0 {
			I_Error(c"PTR_SlideTraverse: not a line?".as_ptr());
		}

		let li = &mut *intercept.d.line;

		if li.flags & ML_TWOSIDED == 0 {
			if P_PointOnLineSide((*slidemo).x, (*slidemo).y, li) {
				// don't hit the back side
				return true;
			}
		} else {
			// set openrange, opentop, openbottom
			P_LineOpening(li);

			if openrange < (*slidemo).height {
				// doesn't fit
			} else if opentop - (*slidemo).z < (*slidemo).height {
				// mobj is too high
			} else if openbottom - (*slidemo).z > 24 * FRACUNIT {
				// too big a step up
			} else {
				// this line doesn't block movement
				return true;
			}
		}

		// the line does block movement,
		// see if it is closer than best so far
		if intercept.frac < bestslidefrac {
			secondslidefrac = bestslidefrac;
			secondslideline = bestslideline;
			bestslidefrac = intercept.frac;
			bestslideline = li;
		}

		false // stop
	}
}

// P_SlideMove
// The momx / momy move is bad, so try to slide
// along a wall.
// Find the first line hit, move flush to it,
// and slide along it
//
// This is a kludgy mess.
pub fn P_SlideMove(mo: &mut mobj_t) {
	fn stairstep(mo: &mut mobj_t) {
		if !P_TryMove(mo, mo.x, mo.y + mo.momy) {
			P_TryMove(mo, mo.x + mo.momx, mo.y);
		};
	}

	unsafe {
		slidemo = mo;
		let mut hitcount = 0;

		loop {
			hitcount += 1;
			if hitcount == 3 {
				stairstep(mo); // don't loop forever
				return;
			}

			// trace along the three leading corners
			let leadx;
			let trailx;
			if mo.momx > 0 {
				leadx = mo.x + mo.radius;
				trailx = mo.x - mo.radius;
			} else {
				leadx = mo.x - mo.radius;
				trailx = mo.x + mo.radius;
			}

			let leady;
			let traily;
			if mo.momy > 0 {
				leady = mo.y + mo.radius;
				traily = mo.y - mo.radius;
			} else {
				leady = mo.y - mo.radius;
				traily = mo.y + mo.radius;
			}

			bestslidefrac = FRACUNIT + 1;

			P_PathTraverse(
				leadx,
				leady,
				leadx + mo.momx,
				leady + mo.momy,
				PT_ADDLINES,
				PTR_SlideTraverse,
			);
			P_PathTraverse(
				trailx,
				leady,
				trailx + mo.momx,
				leady + mo.momy,
				PT_ADDLINES,
				PTR_SlideTraverse,
			);
			P_PathTraverse(
				leadx,
				traily,
				leadx + mo.momx,
				traily + mo.momy,
				PT_ADDLINES,
				PTR_SlideTraverse,
			);

			// move up to the wall
			if bestslidefrac == FRACUNIT + 1 {
				// the move most have hit the middle, so stairstep
				stairstep(mo);
				return;
			}

			// fudge a bit to make sure it doesn't hit
			bestslidefrac -= 0x800;
			if bestslidefrac > 0 {
				let newx = FixedMul(mo.momx, bestslidefrac);
				let newy = FixedMul(mo.momy, bestslidefrac);

				if !P_TryMove(mo, mo.x + newx, mo.y + newy) {
					stairstep(mo);
					return;
				}
			}

			// Now continue along the wall.
			// First calculate remainder.
			bestslidefrac = FRACUNIT - (bestslidefrac + 0x800);

			if bestslidefrac > FRACUNIT {
				bestslidefrac = FRACUNIT;
			}

			if bestslidefrac <= 0 {
				return;
			}

			tmxmove = FixedMul(mo.momx, bestslidefrac);
			tmymove = FixedMul(mo.momy, bestslidefrac);

			P_HitSlideLine(&*bestslideline); // clip the moves

			mo.momx = tmxmove;
			mo.momy = tmymove;

			if P_TryMove(mo, mo.x + tmxmove, mo.y + tmymove) {
				break;
			}
		}
	}
}

// P_LineAttack
pub static mut linetarget: *mut mobj_t = null_mut(); // who got hit (or NULL)
static mut shootthing: *mut mobj_t = null_mut();

// Height if not aiming up or down
// ???: use slope for monsters?
static mut shootz: fixed_t = 0;

static mut la_damage: i32 = 0;
pub static mut attackrange: fixed_t = 0;

static mut aimslope: fixed_t = 0;

// PTR_AimTraverse
// Sets linetaget and aimslope when a target is aimed at.
fn PTR_AimTraverse(intercept: &mut intercept_t) -> bool {
	unsafe {
		if intercept.isaline != 0 {
			let li = &*intercept.d.line;

			if li.flags & ML_TWOSIDED == 0 {
				return false; // stop
			}

			// Crosses a two sided line.
			// A two sided line will restrict
			// the possible target ranges.
			P_LineOpening(li);

			if openbottom >= opentop {
				return false; // stop
			}

			let dist = FixedMul(attackrange, intercept.frac);

			if (*li.frontsector).floorheight != (*li.backsector).floorheight {
				let slope = FixedDiv(openbottom - shootz, dist);
				if slope > bottomslope {
					bottomslope = slope;
				}
			}

			if (*li.frontsector).ceilingheight != (*li.backsector).ceilingheight {
				let slope = FixedDiv(opentop - shootz, dist);
				if slope < topslope {
					topslope = slope;
				}
			}

			if topslope <= bottomslope {
				return false; // stop
			}

			return true; // shot continues
		}

		// shoot a thing
		let th = intercept.d.thing;
		if ptr::eq(th, shootthing) {
			return true; // can't shoot self
		}

		if (*th).flags & MF_SHOOTABLE == 0 {
			return true; // corpse or something
		}

		// check angles to see if the thing can be aimed at
		let dist = FixedMul(attackrange, intercept.frac);
		let mut thingtopslope = FixedDiv((*th).z + (*th).height - shootz, dist);

		if thingtopslope < bottomslope {
			return true; // shot over the thing
		}

		let mut thingbottomslope = FixedDiv((*th).z - shootz, dist);

		if thingbottomslope > topslope {
			return true; // shot under the thing
		}

		// this thing can be hit!
		if thingtopslope > topslope {
			thingtopslope = topslope;
		}

		if thingbottomslope < bottomslope {
			thingbottomslope = bottomslope;
		}

		aimslope = (thingtopslope + thingbottomslope) / 2;
		linetarget = th;

		false // don't go any farther
	}
}

// PTR_ShootTraverse
fn PTR_ShootTraverse(intercept: &mut intercept_t) -> bool {
	unsafe {
		if intercept.isaline != 0 {
			let li = &mut *intercept.d.line;

			if li.special != 0 {
				P_ShootSpecialLine(&mut *shootthing, li);
			}

			if li.flags & ML_TWOSIDED != 0 {
				// crosses a two sided line
				P_LineOpening(li);

				let dist = FixedMul(attackrange, intercept.frac);

				if !((*li.frontsector).floorheight != (*li.backsector).floorheight
					&& FixedDiv(openbottom - shootz, dist) > aimslope
					|| (*li.frontsector).ceilingheight != (*li.backsector).ceilingheight
						&& FixedDiv(opentop - shootz, dist) < aimslope)
				{
					// shot continues
					return true;
				}
			}

			// hit line
			// position a bit closer
			let frac = intercept.frac - FixedDiv(4 * FRACUNIT, attackrange);
			let x = trace.x + FixedMul(trace.dx, frac);
			let y = trace.y + FixedMul(trace.dy, frac);
			let z = shootz + FixedMul(aimslope, FixedMul(frac, attackrange));

			if (*li.frontsector).ceilingpic as usize == skyflatnum {
				// don't shoot the sky!
				if z > (*li.frontsector).ceilingheight {
					return false;
				}

				// it's a sky hack wall
				if !li.backsector.is_null() && (*li.backsector).ceilingpic as usize == skyflatnum {
					return false;
				}
			}

			// Spawn bullet puffs.
			P_SpawnPuff(x, y, z);

			// don't go any farther
			return false;
		}

		// shoot a thing
		let th = intercept.d.thing;
		if ptr::eq(th, shootthing) {
			return true; // can't shoot self
		}

		if (*th).flags & MF_SHOOTABLE == 0 {
			return true; // corpse or something
		}

		// check angles to see if the thing can be aimed at
		let dist = FixedMul(attackrange, intercept.frac);
		let thingtopslope = FixedDiv((*th).z + (*th).height - shootz, dist);

		if thingtopslope < aimslope {
			return true; // shot over the thing
		}

		let thingbottomslope = FixedDiv((*th).z - shootz, dist);

		if thingbottomslope > aimslope {
			return true; // shot under the thing
		}

		// hit thing
		// position a bit closer
		let frac = intercept.frac - FixedDiv(10 * FRACUNIT, attackrange);

		let x = trace.x + FixedMul(trace.dx, frac);
		let y = trace.y + FixedMul(trace.dy, frac);
		let z = shootz + FixedMul(aimslope, FixedMul(frac, attackrange));

		// Spawn bullet puffs or blod spots,
		// depending on target type.
		if (*intercept.d.thing).flags & MF_NOBLOOD != 0 {
			P_SpawnPuff(x, y, z);
		} else {
			P_SpawnBlood(x, y, z, la_damage);
		}

		if la_damage != 0 {
			P_DamageMobj(&mut *th, shootthing, shootthing, la_damage);
		}

		// don't go any farther
		false
	}
}

// P_AimLineAttack
pub fn P_AimLineAttack(t1: &mut mobj_t, mut angle: angle_t, distance: fixed_t) -> fixed_t {
	unsafe {
		angle >>= ANGLETOFINESHIFT;
		shootthing = t1;

		let x2 = t1.x + (distance >> FRACBITS) * finecos(angle.0);
		let y2 = t1.y + (distance >> FRACBITS) * finesine[angle.0];
		shootz = t1.z + (t1.height >> 1) + 8 * FRACUNIT;

		// can't shoot outside view angles
		topslope = 100 * FRACUNIT / 160;
		bottomslope = -100 * FRACUNIT / 160;

		attackrange = distance;
		linetarget = null_mut();

		P_PathTraverse(t1.x, t1.y, x2, y2, PT_ADDLINES | PT_ADDTHINGS, PTR_AimTraverse);

		if !linetarget.is_null() {
			return aimslope;
		}

		0
	}
}

// P_LineAttack
// If damage == 0, it is just a test trace
// that will leave linetarget set.
pub fn P_LineAttack(
	t1: &mut mobj_t,
	mut angle: angle_t,
	distance: fixed_t,
	slope: fixed_t,
	damage: i32,
) {
	unsafe {
		angle >>= ANGLETOFINESHIFT;
		shootthing = t1;
		la_damage = damage;
		let x2 = t1.x + (distance >> FRACBITS) * finecos(angle.0);
		let y2 = t1.y + (distance >> FRACBITS) * finesine[angle.0];
		shootz = t1.z + (t1.height >> 1) + 8 * FRACUNIT;
		attackrange = distance;
		aimslope = slope;

		P_PathTraverse(t1.x, t1.y, x2, y2, PT_ADDLINES | PT_ADDTHINGS, PTR_ShootTraverse);
	}
}

// USE LINES
static mut usething: *mut mobj_t = null_mut();

fn PTR_UseTraverse(intercept: &mut intercept_t) -> bool {
	unsafe {
		if (*intercept.d.line).special == 0 {
			P_LineOpening(&*intercept.d.line);
			if openrange <= 0 {
				S_StartSound(usething.cast(), sfxenum_t::sfx_noway);

				// can't use through a wall
				return false;
			} else {
				// not a special line, but keep checking
				return true;
			}
		}

		let mut side = 0;
		if P_PointOnLineSide((*usething).x, (*usething).y, &*intercept.d.line) {
			side = 1;
		}

		P_UseSpecialLine(&mut *usething, &mut *intercept.d.line, side);

		// can't use for than one special line in a row
		false
	}
}

// P_UseLines
// Looks for special lines in front of the player to activate.
pub fn P_UseLines(player: &mut player_t) {
	unsafe {
		usething = player.mo;

		let angle = (*player.mo).angle >> ANGLETOFINESHIFT;

		let x1 = (*player.mo).x;
		let y1 = (*player.mo).y;
		let x2 = x1 + (USERANGE >> FRACBITS) * finecos(angle.0);
		let y2 = y1 + (USERANGE >> FRACBITS) * finesine[angle.0];

		P_PathTraverse(x1, y1, x2, y2, PT_ADDLINES, PTR_UseTraverse);
	}
}

// RADIUS ATTACK
static mut bombsource: *mut mobj_t = null_mut();
static mut bombspot: *mut mobj_t = null_mut();
static mut bombdamage: i32 = 0;

// PIT_RadiusAttack
// "bombsource" is the creature
// that caused the explosion at "bombspot".
fn PIT_RadiusAttack(thing: &mut mobj_t) -> bool {
	unsafe {
		if (thing.flags & MF_SHOOTABLE) == 0 {
			return true;
		}

		// Boss spider and cyborg
		// take no damage from concussion.
		if thing.ty == mobjtype_t::MT_CYBORG || thing.ty == mobjtype_t::MT_SPIDER {
			return true;
		}

		let dx = i32::abs(thing.x - (*bombspot).x);
		let dy = i32::abs(thing.y - (*bombspot).y);

		let mut dist = i32::max(dx, dy);
		dist = (dist - thing.radius) >> FRACBITS;

		if dist < 0 {
			dist = 0;
		}

		if dist >= bombdamage {
			return true; // out of range
		}

		if P_CheckSight(thing, &*bombspot) {
			// must be in direct path
			P_DamageMobj(thing, bombspot, bombsource, bombdamage - dist);
		}

		true
	}
}

// P_RadiusAttack
// Source is the creature that caused the explosion at spot.
pub fn P_RadiusAttack(spot: &mut mobj_t, source: *mut mobj_t, damage: i32) {
	unsafe {
		let dist = (damage + MAXRADIUS) << FRACBITS;
		let yh = (spot.y + dist - bmaporgy) >> MAPBLOCKSHIFT;
		let yl = (spot.y - dist - bmaporgy) >> MAPBLOCKSHIFT;
		let xh = (spot.x + dist - bmaporgx) >> MAPBLOCKSHIFT;
		let xl = (spot.x - dist - bmaporgx) >> MAPBLOCKSHIFT;
		bombspot = spot;
		bombsource = source;
		bombdamage = damage;

		for y in yl..=yh {
			for x in xl..=xh {
				P_BlockThingsIterator(x, y, PIT_RadiusAttack);
			}
		}
	}
}

// SECTOR HEIGHT CHANGING
// After modifying a sectors floor or ceiling height,
// call this routine to adjust the positions
// of all things that touch the sector.
//
// If anything doesn't fit anymore, true will be returned.
// If crunch is true, they will take damage
//  as they are being crushed.
// If Crunch is false, you should set the sector height back
//  the way it was and call P_ChangeSector again
//  to undo the changes.
static mut crushchange: bool = false;
static mut nofit: bool = false;

// PIT_ChangeSector
fn PIT_ChangeSector(thing: &mut mobj_t) -> bool {
	unsafe {
		if P_ThingHeightClip(thing) {
			// keep checking
			return true;
		}

		// crunch bodies to giblets
		if thing.health <= 0 {
			P_SetMobjState(thing, statenum_t::S_GIBS);

			thing.flags &= !MF_SOLID;
			thing.height = 0;
			thing.radius = 0;

			// keep checking
			return true;
		}

		// crunch dropped items
		if thing.flags & MF_DROPPED != 0 {
			P_RemoveMobj(thing);

			// keep checking
			return true;
		}

		if thing.flags & MF_SHOOTABLE == 0 {
			// assume it is bloody gibs or something
			return true;
		}

		nofit = true;

		if crushchange && leveltime & 3 == 0 {
			P_DamageMobj(thing, null_mut(), null_mut(), 10);

			// spray blood in a random direction
			let mo =
				P_SpawnMobj(thing.x, thing.y, thing.z + thing.height / 2, mobjtype_t::MT_BLOOD);

			(*mo).momx = (P_Random() - P_Random()) << 12;
			(*mo).momy = (P_Random() - P_Random()) << 12;
		}

		// keep checking (crush other things)
		true
	}
}

// P_ChangeSector
pub fn P_ChangeSector(sector: &mut sector_t, crunch: bool) -> bool {
	unsafe {
		nofit = false;
		crushchange = crunch;

		// re-check heights for all things near the moving sector
		for x in sector.blockbox[BOXLEFT]..=sector.blockbox[BOXRIGHT] {
			for y in sector.blockbox[BOXBOTTOM]..=sector.blockbox[BOXTOP] {
				P_BlockThingsIterator(x, y, PIT_ChangeSector);
			}
		}

		nofit
	}
}
