#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::{
	num::Wrapping,
	ptr::{self, null_mut},
};

use crate::{
	d_main::fastparm,
	d_player::player_t,
	d_think::thinker_t,
	doomdata::{ML_SOUNDBLOCK, ML_TWOSIDED},
	doomdef::{GameMode_t, MAXPLAYERS, skill_t},
	doomstat::gamemode,
	g_game::{
		G_ExitLevel, gameepisode, gamemap, gameskill, gametic, netgame, playeringame, players,
	},
	i_system::I_Error,
	info::{mobjinfo, mobjtype_t, statenum_t},
	m_fixed::{FRACUNIT, FixedMul, fixed_t},
	m_random::P_Random,
	p_doors::EV_DoDoor,
	p_floor::EV_DoFloor,
	p_inter::P_DamageMobj,
	p_local::{FLOATSPEED, MAPBLOCKSHIFT, MAXRADIUS, MELEERANGE, MISSILERANGE, thinkercap},
	p_mobj::{
		MF_AMBUSH, MF_CORPSE, MF_FLOAT, MF_INFLOAT, MF_JUSTATTACKED, MF_JUSTHIT, MF_SHADOW,
		MF_SHOOTABLE, MF_SKULLFLY, MF_SOLID, P_RemoveMobj, P_SetMobjState, P_SpawnMissile,
		P_SpawnMobj, P_SpawnPuff, mobj_t,
	},
	p_pspr::{A_ReFire, pspdef_t},
	p_setup::{bmaporgx, bmaporgy, sides},
	p_sight::P_CheckSight,
	p_spec::{floor_e, vldoor_e},
	p_switch::P_UseSpecialLine,
	r_defs::{line_t, sector_t, slopetype_t},
	s_sound::S_StartSound,
	sounds::sfxenum_t,
	tables::{ANG90, ANG180, ANG270, ANGLETOFINESHIFT, angle_t, finecos, finesine},
};

type boolean = i32;

#[repr(C)]
#[allow(clippy::upper_case_acronyms)] // ???
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum dirtype_t {
	DI_EAST,
	DI_NORTHEAST,
	DI_NORTH,
	DI_NORTHWEST,
	DI_WEST,
	DI_SOUTHWEST,
	DI_SOUTH,
	DI_SOUTHEAST,
	DI_NODIR,
	NUMDIRS,
}

const DIRS: [dirtype_t; 8] = [
	dirtype_t::DI_EAST,
	dirtype_t::DI_NORTHEAST,
	dirtype_t::DI_NORTH,
	dirtype_t::DI_NORTHWEST,
	dirtype_t::DI_WEST,
	dirtype_t::DI_SOUTHWEST,
	dirtype_t::DI_SOUTH,
	dirtype_t::DI_SOUTHEAST,
];

// P_NewChaseDir related LUT.
const opposite: [dirtype_t; 9] = [
	dirtype_t::DI_WEST,
	dirtype_t::DI_SOUTHWEST,
	dirtype_t::DI_SOUTH,
	dirtype_t::DI_SOUTHEAST,
	dirtype_t::DI_EAST,
	dirtype_t::DI_NORTHEAST,
	dirtype_t::DI_NORTH,
	dirtype_t::DI_NORTHWEST,
	dirtype_t::DI_NODIR,
];

const diags: [dirtype_t; 4] = [
	dirtype_t::DI_NORTHWEST,
	dirtype_t::DI_NORTHEAST,
	dirtype_t::DI_SOUTHWEST,
	dirtype_t::DI_SOUTHEAST,
];

// ENEMY THINKING
// Enemies are allways spawned
// with targetplayer = -1, threshold = 0
// Most monsters are spawned unaware of all players,
// but some can be made preaware

// Called by P_NoiseAlert.
// Recursively traverse adjacent sectors,
// sound blocking lines cut off traversal.

static mut soundtarget: *mut mobj_t = null_mut();

unsafe extern "C" {
	static mut validcount: i32;
	static mut openrange: fixed_t;

	fn P_LineOpening(linedef: *mut line_t);
}

fn P_RecursiveSound(sec: &mut sector_t, soundblocks: i32) {
	unsafe {
		// wake up all monsters in this sector
		if sec.validcount == validcount && sec.soundtraversed <= soundblocks + 1 {
			return; // already flooded
		}

		sec.validcount = validcount;
		sec.soundtraversed = soundblocks + 1;
		sec.soundtarget = soundtarget;

		for i in 0..sec.linecount {
			let check = &mut **sec.lines.wrapping_add(i);
			if check.flags & ML_TWOSIDED == 0 {
				continue;
			}

			P_LineOpening(check);

			if openrange <= 0 {
				continue; // closed door
			}

			let other = if (*sides.wrapping_add(check.sidenum[0] as usize)).sector == sec {
				&mut *(*sides.wrapping_add(check.sidenum[1] as usize)).sector
			} else {
				&mut *(*sides.wrapping_add(check.sidenum[0] as usize)).sector
			};

			if check.flags as usize & ML_SOUNDBLOCK != 0 {
				if soundblocks == 0 {
					P_RecursiveSound(other, 1);
				}
			} else {
				P_RecursiveSound(other, soundblocks);
			}
		}
	}
}

// P_NoiseAlert
// If a monster yells at a player,
// it will alert other monsters to the player.
#[unsafe(no_mangle)]
pub extern "C" fn P_NoiseAlert(target: *mut mobj_t, emmiter: &mut mobj_t) {
	unsafe {
		soundtarget = target;
		validcount += 1;
		P_RecursiveSound(&mut *(*emmiter.subsector).sector, 0);
	}
}

unsafe extern "C" {
	fn P_AproxDistance(x: fixed_t, y: fixed_t) -> fixed_t;
}

// P_CheckMeleeRange
fn P_CheckMeleeRange(actor: &mobj_t) -> bool {
	unsafe {
		if actor.target.is_null() {
			return false;
		}
		let pl = &*actor.target;
		let dist = P_AproxDistance(pl.x - actor.x, pl.y - actor.y);
		if dist >= MELEERANGE - 20 * FRACUNIT + (*pl.info).radius {
			return false;
		}
		P_CheckSight(actor, &*actor.target) != 0
	}
}

// P_CheckMissileRange
fn P_CheckMissileRange(actor: &mut mobj_t) -> bool {
	unsafe {
		if P_CheckSight(actor, &*actor.target) == 0 {
			return false;
		}

		if actor.flags & MF_JUSTHIT != 0 {
			// the target just hit the enemy,
			// so fight back!
			actor.flags &= !MF_JUSTHIT;
			return true;
		}

		if actor.reactiontime != 0 {
			return false; // do not attack yet
		}

		// OPTIMIZE: get this from a global checksight
		let mut dist = P_AproxDistance(actor.x - (*actor.target).x, actor.y - (*actor.target).y)
			- 64 * FRACUNIT;

		if (*actor.info).meleestate == statenum_t::S_NULL {
			dist -= 128 * FRACUNIT; // no melee attack, so fire more
		}

		dist >>= 16;

		if actor.ty == mobjtype_t::MT_VILE && dist > 14 * 64 {
			return false; // too far away
		}

		if actor.ty == mobjtype_t::MT_UNDEAD {
			if dist < 196 {
				return false; // close for fist attack
			}
			dist >>= 1;
		}

		if actor.ty == mobjtype_t::MT_CYBORG
			|| actor.ty == mobjtype_t::MT_SPIDER
			|| actor.ty == mobjtype_t::MT_SKULL
		{
			dist >>= 1;
		}

		if dist > 200 {
			dist = 200;
		}

		if actor.ty == mobjtype_t::MT_CYBORG && dist > 160 {
			dist = 160;
		}

		P_Random() >= dist
	}
}

// P_Move
// Move in the current direction,
// returns false if the move is blocked.
const xspeed: [fixed_t; 8] = [FRACUNIT, 47000, 0, -47000, -FRACUNIT, -47000, 0, 47000];
const yspeed: [fixed_t; 8] = [0, 47000, FRACUNIT, 47000, 0, -47000, -FRACUNIT, -47000];

const MAXSPECIALCROSS: usize = 8;

unsafe extern "C" {
	static mut floatok: boolean;
	static mut tmfloorz: fixed_t;
	static mut numspechit: usize;
	static mut spechit: [*mut line_t; MAXSPECIALCROSS];

	fn P_TryMove(thing: *mut mobj_t, x: fixed_t, y: fixed_t) -> boolean;
}

fn P_Move(actor: &mut mobj_t) -> bool {
	unsafe {
		if actor.movedir == dirtype_t::DI_NODIR {
			return false;
		}

		if actor.movedir == dirtype_t::NUMDIRS {
			I_Error(c"Weird actor.movedir!".as_ptr());
		}

		let tryx = actor.x + (*actor.info).speed * xspeed[actor.movedir as usize];
		let tryy = actor.y + (*actor.info).speed * yspeed[actor.movedir as usize];

		let try_ok = P_TryMove(actor, tryx, tryy);

		if try_ok == 0 {
			// open any specials
			if actor.flags & MF_FLOAT != 0 && floatok != 0 {
				// must adjust height
				if actor.z < tmfloorz {
					actor.z += FLOATSPEED;
				} else {
					actor.z -= FLOATSPEED;
				}

				actor.flags |= MF_INFLOAT;
				return true;
			}

			if numspechit == 0 {
				return false;
			}

			actor.movedir = dirtype_t::DI_NODIR;
			let mut good = false;
			while numspechit != 0 {
				numspechit -= 1;
				let ld = spechit[numspechit];
				// if the special is not a door
				// that can be opened,
				// return false
				if P_UseSpecialLine(actor, &mut *ld, 0) != 0 {
					good = true;
				}
			}
			return good;
		} else {
			actor.flags &= !MF_INFLOAT;
		}

		if actor.flags & MF_FLOAT == 0 {
			actor.z = actor.floorz;
		}
		true
	}
}

// TryWalk
// Attempts to move actor on
// in its current (ob.moveangle) direction.
// If blocked by either a wall or an actor
// returns FALSE
// If move is either clear or blocked only by a door,
// returns TRUE and sets...
// If a door is in the way,
// an OpenDoor call is made to start it opening.
fn P_TryWalk(actor: &mut mobj_t) -> bool {
	if !P_Move(actor) {
		return false;
	}
	actor.movecount = P_Random() & 15;
	true
}

fn P_NewChaseDir(actor: &mut mobj_t) {
	unsafe {
		if actor.target.is_null() {
			I_Error(c"P_NewChaseDir: called with no target".as_ptr());
		}

		let olddir = actor.movedir;
		let turnaround = opposite[olddir as usize];

		let deltax = (*actor.target).x - actor.x;
		let deltay = (*actor.target).y - actor.y;

		let mut d = [dirtype_t::DI_NODIR; 3];

		if deltax > 10 * FRACUNIT {
			d[1] = dirtype_t::DI_EAST;
		} else if deltax < -10 * FRACUNIT {
			d[1] = dirtype_t::DI_WEST;
		} else {
			d[1] = dirtype_t::DI_NODIR;
		}

		if deltay < -10 * FRACUNIT {
			d[2] = dirtype_t::DI_SOUTH;
		} else if deltay > 10 * FRACUNIT {
			d[2] = dirtype_t::DI_NORTH;
		} else {
			d[2] = dirtype_t::DI_NODIR;
		}

		// try direct route
		if d[1] != dirtype_t::DI_NODIR && d[2] != dirtype_t::DI_NODIR {
			actor.movedir = diags[(((deltay < 0) as usize) << 1) + ((deltax > 0) as usize)];
			if actor.movedir != turnaround && P_TryWalk(actor) {
				return;
			}
		}

		// try other directions
		if P_Random() > 200 || deltay.abs() > deltax.abs() {
			d.swap(1, 2);
		}

		if d[1] == turnaround {
			d[1] = dirtype_t::DI_NODIR;
		}
		if d[2] == turnaround {
			d[2] = dirtype_t::DI_NODIR;
		}

		if d[1] != dirtype_t::DI_NODIR {
			actor.movedir = d[1];
			if P_TryWalk(actor) {
				// either moved forward or attacked
				return;
			}
		}

		if d[2] != dirtype_t::DI_NODIR {
			actor.movedir = d[2];

			if P_TryWalk(actor) {
				return;
			}
		}

		// there is no direct path to the player,
		// so pick another direction.
		if olddir != dirtype_t::DI_NODIR {
			actor.movedir = olddir;

			if P_TryWalk(actor) {
				return;
			}
		}

		// randomly determine direction of search
		if P_Random() & 1 != 0 {
			for tdir in DIRS {
				if tdir != turnaround {
					actor.movedir = tdir;

					if P_TryWalk(actor) {
						return;
					}
				}
			}
		} else {
			for tdir in DIRS.into_iter().rev() {
				if tdir != turnaround {
					actor.movedir = tdir;

					if P_TryWalk(actor) {
						return;
					}
				}
			}
		}

		if turnaround != dirtype_t::DI_NODIR {
			actor.movedir = turnaround;
			if P_TryWalk(actor) {
				return;
			}
		}

		actor.movedir = dirtype_t::DI_NODIR; // can not move
	}
}

unsafe extern "C" {
	fn R_PointToAngle2(x_1: i32, y_1: i32, x_2: i32, y_2: i32) -> angle_t;
}

// P_LookForPlayers
// If allaround is false, only look 180 degrees in front.
// Returns true if a player is targeted.
fn P_LookForPlayers(actor: &mut mobj_t, allaround: bool) -> bool {
	unsafe {
		let mut c = 0;
		let stop = (actor.lastlook - 1) & 3;

		actor.lastlook = stop;
		loop {
			actor.lastlook = (actor.lastlook + 1) & 3;

			if playeringame[actor.lastlook as usize] == 0 {
				continue;
			}

			if c == 2 || actor.lastlook == stop {
				// done looking
				return false;
			}
			c += 1;

			let player = &mut players[actor.lastlook as usize];

			if player.health <= 0 {
				continue; // dead
			}

			if P_CheckSight(actor, &*player.mo) == 0 {
				continue; // out of sight
			}

			if !allaround {
				let an =
					R_PointToAngle2(actor.x, actor.y, (*player.mo).x, (*player.mo).y) - actor.angle;

				if an > ANG90 && an < ANG270 {
					let dist = P_AproxDistance((*player.mo).x - actor.x, (*player.mo).y - actor.y);
					// if real close, react anyway
					if dist > MELEERANGE {
						continue; // behind back
					}
				}
			}

			actor.target = player.mo;
			return true;
		}
	}
}

// A_KeenDie
// DOOM II special, map 32.
// Uses special tag 666.
pub(crate) fn A_KeenDie(mo: &mut mobj_t) {
	unsafe {
		A_Fall(mo);

		// scan the remaining thinkers
		// to see if all Keens are dead
		let mut th = &*thinkercap.next;
		while !std::ptr::eq(th, &raw const thinkercap) {
			if th.function.is_mobj() {
				th = &*th.next;
				continue;
			}

			let mo2 = &*(th as *const thinker_t as *const mobj_t);
			if !ptr::eq(mo2, mo) && mo2.ty == mo.ty && mo2.health > 0 {
				// other Keen not dead
				return;
			}
			th = &*th.next;
		}

		let mut junk = line_t {
			v1: null_mut(),
			v2: null_mut(),
			dx: 0,
			dy: 0,
			flags: 0,
			special: 0,
			tag: 666,
			sidenum: [0; 2],
			bbox: [0; 4],
			slopetype: slopetype_t::ST_VERTICAL,
			frontsector: null_mut(),
			backsector: null_mut(),
			validcount: 0,
			specialdata: null_mut(),
		};
		EV_DoDoor(&mut junk, vldoor_e::open);
	}
}

// ACTION ROUTINES

// A_Look
// Stay in state until a player is sighted.
pub(crate) fn A_Look(actor: &mut mobj_t) {
	unsafe {
		actor.threshold = 0; // any shot will wake up
		let targ = (*(*actor.subsector).sector).soundtarget;

		if !targ.is_null() && (*targ).flags & MF_SHOOTABLE != 0 {
			actor.target = targ;

			if actor.flags & MF_AMBUSH != 0
				&& P_CheckSight(actor, &*actor.target) == 0
				&& !P_LookForPlayers(actor, false)
			{
				return;
			}
		} else if !P_LookForPlayers(actor, false) {
			return;
		}

		// go into chase state
		if (*actor.info).seesound != sfxenum_t::sfx_None {
			let sound = match (*actor.info).seesound {
				sfxenum_t::sfx_posit1 | sfxenum_t::sfx_posit2 | sfxenum_t::sfx_posit3 => {
					match P_Random() % 3 {
						0 => sfxenum_t::sfx_posit1,
						1 => sfxenum_t::sfx_posit2,
						2 => sfxenum_t::sfx_posit3,
						_ => unreachable!(),
					}
				}

				sfxenum_t::sfx_bgsit1 | sfxenum_t::sfx_bgsit2 => match P_Random() % 2 {
					0 => sfxenum_t::sfx_bgsit1,
					1 => sfxenum_t::sfx_bgsit2,
					_ => unreachable!(),
				},

				sound => sound,
			};

			if actor.ty == mobjtype_t::MT_SPIDER || actor.ty == mobjtype_t::MT_CYBORG {
				// full volume
				S_StartSound(null_mut(), sound);
			} else {
				S_StartSound((actor as *mut mobj_t).cast(), sound);
			}
		}

		P_SetMobjState(actor, (*actor.info).seestate);
	}
}

// A_Chase
// Actor has a melee attack,
// so it tries to close as fast as possible
pub(crate) fn A_Chase(actor: &mut mobj_t) {
	unsafe {
		if actor.reactiontime != 0 {
			actor.reactiontime -= 1;
		}

		// modify target threshold
		if actor.threshold != 0 {
			if actor.target.is_null() || (*actor.target).health <= 0 {
				actor.threshold = 0;
			} else {
				actor.threshold -= 1;
			}
		}

		// turn towards movement direction if not there yet
		if actor.movedir != dirtype_t::DI_NODIR {
			actor.angle &= 7 << 29;
			let delta = (actor.angle - Wrapping((actor.movedir as usize) << 29)).0 as isize;

			if delta > 0 {
				actor.angle -= ANG90 / Wrapping(2);
			} else if delta < 0 {
				actor.angle += ANG90 / Wrapping(2);
			}
		}

		if actor.target.is_null() || (*actor.target).flags & MF_SHOOTABLE == 0 {
			// look for a new target
			if P_LookForPlayers(actor, true) {
				return; // got a new target
			}

			P_SetMobjState(actor, (*actor.info).spawnstate);
			return;
		}

		// do not attack twice in a row
		if actor.flags & MF_JUSTATTACKED != 0 {
			actor.flags &= !MF_JUSTATTACKED;
			if gameskill != skill_t::sk_nightmare && fastparm == 0 {
				P_NewChaseDir(actor);
			}
			return;
		}

		// check for melee attack
		if (*actor.info).meleestate != statenum_t::S_NULL && P_CheckMeleeRange(actor) {
			if (*actor.info).attacksound != sfxenum_t::sfx_None {
				S_StartSound((actor as *mut mobj_t).cast(), (*actor.info).attacksound);
			}

			P_SetMobjState(actor, (*actor.info).meleestate);
			return;
		}

		// check for missile attack
		if (*actor.info).missilestate != statenum_t::S_NULL
			&& (gameskill >= skill_t::sk_nightmare || fastparm != 0 || actor.movecount == 0)
			&& P_CheckMissileRange(actor)
		{
			P_SetMobjState(actor, (*actor.info).missilestate);
			actor.flags |= MF_JUSTATTACKED;
			return;
		}

		// possibly choose another target
		if netgame != 0
			&& actor.threshold == 0
			&& P_CheckSight(actor, &*actor.target) == 0
			&& P_LookForPlayers(actor, true)
		{
			return; // got a new target
		}

		// chase towards player
		actor.movecount -= 1;
		if actor.movecount < 0 || !P_Move(actor) {
			P_NewChaseDir(actor);
		}

		// make active sound
		if (*actor.info).activesound != sfxenum_t::sfx_None && P_Random() < 3 {
			S_StartSound((actor as *mut mobj_t).cast(), (*actor.info).activesound);
		}
	}
}

// A_FaceTarget
pub(crate) fn A_FaceTarget(actor: &mut mobj_t) {
	if actor.target.is_null() {
		return;
	}

	actor.flags &= !MF_AMBUSH;

	actor.angle =
		unsafe { R_PointToAngle2(actor.x, actor.y, (*actor.target).x, (*actor.target).y) };

	if unsafe { (*actor.target).flags & MF_SHADOW != 0 } {
		actor.angle += Wrapping(((P_Random() - P_Random()) << 21) as usize);
	}
}

unsafe extern "C" {
	fn P_AimLineAttack(t1: *mut mobj_t, angle: angle_t, distance: fixed_t) -> fixed_t;
	fn P_LineAttack(
		t1: *mut mobj_t,
		angle: angle_t,
		distance: fixed_t,
		slope: fixed_t,
		damage: i32,
	);
}

// A_PosAttack
pub(crate) fn A_PosAttack(actor: &mut mobj_t) {
	if actor.target.is_null() {
		return;
	}

	A_FaceTarget(actor);
	let mut angle = actor.angle;
	let slope = unsafe { P_AimLineAttack(actor, angle, MISSILERANGE) };

	S_StartSound((actor as *mut mobj_t).cast(), sfxenum_t::sfx_pistol);
	angle += Wrapping(((P_Random() - P_Random()) << 20) as usize);
	let damage = ((P_Random() % 5) + 1) * 3;
	unsafe { P_LineAttack(actor, angle, MISSILERANGE, slope, damage) };
}

pub(crate) fn A_SPosAttack(actor: &mut mobj_t) {
	unsafe {
		if actor.target.is_null() {
			return;
		}

		S_StartSound((actor as *mut mobj_t).cast(), sfxenum_t::sfx_shotgn);
		A_FaceTarget(actor);
		let bangle = actor.angle;
		let slope = P_AimLineAttack(actor, bangle, MISSILERANGE);

		for _ in 0..3 {
			let angle = bangle + Wrapping(((P_Random() - P_Random()) << 20) as usize);
			let damage = ((P_Random() % 5) + 1) * 3;
			P_LineAttack(actor, angle, MISSILERANGE, slope, damage);
		}
	}
}

pub(crate) fn A_CPosAttack(actor: &mut mobj_t) {
	unsafe {
		if actor.target.is_null() {
			return;
		}

		S_StartSound((actor as *mut mobj_t).cast(), sfxenum_t::sfx_shotgn);
		A_FaceTarget(actor);
		let bangle = actor.angle;
		let slope = P_AimLineAttack(actor, bangle, MISSILERANGE);

		let angle = bangle + Wrapping(((P_Random() - P_Random()) << 20) as usize);
		let damage = ((P_Random() % 5) + 1) * 3;
		P_LineAttack(actor, angle, MISSILERANGE, slope, damage);
	}
}

pub(crate) fn A_CPosRefire(actor: &mut mobj_t) {
	unsafe {
		// keep firing unless target got out of sight
		A_FaceTarget(actor);

		if P_Random() < 40 {
			return;
		}

		if actor.target.is_null()
			|| (*actor.target).health <= 0
			|| P_CheckSight(actor, &*actor.target) == 0
		{
			P_SetMobjState(actor, (*actor.info).seestate);
		}
	}
}

pub(crate) fn A_SpidRefire(actor: &mut mobj_t) {
	unsafe {
		// keep firing unless target got out of sight
		A_FaceTarget(actor);

		if P_Random() < 10 {
			return;
		}

		if actor.target.is_null()
			|| (*actor.target).health <= 0
			|| P_CheckSight(actor, &*actor.target) == 0
		{
			P_SetMobjState(actor, (*actor.info).seestate);
		}
	}
}

pub(crate) fn A_BspiAttack(actor: &mut mobj_t) {
	if actor.target.is_null() {
		return;
	}

	A_FaceTarget(actor);

	// launch a missile
	unsafe {
		let dest = &mut *actor.target;
		P_SpawnMissile(actor, dest, mobjtype_t::MT_ARACHPLAZ);
	}
}

// A_TroopAttack
pub(crate) fn A_TroopAttack(actor: &mut mobj_t) {
	if actor.target.is_null() {
		return;
	}

	A_FaceTarget(actor);
	if P_CheckMeleeRange(actor) {
		S_StartSound((actor as *mut mobj_t).cast(), sfxenum_t::sfx_claw);
		let damage = (P_Random() % 8 + 1) * 3;
		unsafe { P_DamageMobj(&mut *actor.target, actor, actor, damage) };
		return;
	}

	// launch a missile
	unsafe {
		let dest = &mut *actor.target;
		P_SpawnMissile(actor, dest, mobjtype_t::MT_TROOPSHOT);
	}
}

pub(crate) fn A_SargAttack(actor: &mut mobj_t) {
	if actor.target.is_null() {
		return;
	}

	A_FaceTarget(actor);
	if P_CheckMeleeRange(actor) {
		let damage = ((P_Random() % 10) + 1) * 4;
		unsafe { P_DamageMobj(&mut *actor.target, actor, actor, damage) };
	}
}

pub(crate) fn A_HeadAttack(actor: &mut mobj_t) {
	if actor.target.is_null() {
		return;
	}

	A_FaceTarget(actor);
	if P_CheckMeleeRange(actor) {
		let damage = (P_Random() % 6 + 1) * 10;
		unsafe { P_DamageMobj(&mut *actor.target, actor, actor, damage) };
		return;
	}

	// launch a missile
	unsafe {
		let dest = &mut *actor.target;
		P_SpawnMissile(actor, dest, mobjtype_t::MT_HEADSHOT);
	}
}

pub(crate) fn A_CyberAttack(actor: &mut mobj_t) {
	if actor.target.is_null() {
		return;
	}

	A_FaceTarget(actor);
	unsafe {
		let dest = &mut *actor.target;
		P_SpawnMissile(actor, dest, mobjtype_t::MT_ROCKET);
	}
}

pub(crate) fn A_BruisAttack(actor: &mut mobj_t) {
	if actor.target.is_null() {
		return;
	}

	if P_CheckMeleeRange(actor) {
		S_StartSound((actor as *mut mobj_t).cast(), sfxenum_t::sfx_claw);
		let damage = (P_Random() % 8 + 1) * 10;
		unsafe { P_DamageMobj(&mut *actor.target, actor, actor, damage) };
		return;
	}

	// launch a missile
	unsafe {
		let dest = &mut *actor.target;
		P_SpawnMissile(actor, dest, mobjtype_t::MT_BRUISERSHOT);
	}
}

// A_SkelMissile
pub(crate) fn A_SkelMissile(actor: &mut mobj_t) {
	if actor.target.is_null() {
		return;
	}

	A_FaceTarget(actor);
	actor.z += 16 * FRACUNIT; // so missile spawns higher
	let mo = unsafe {
		let dest = &mut *actor.target;
		&mut *P_SpawnMissile(actor, dest, mobjtype_t::MT_TRACER)
	};
	actor.z -= 16 * FRACUNIT; // back to normal

	mo.x += mo.momx;
	mo.y += mo.momy;
	mo.tracer = actor.target;
}

const TRACEANGLE: angle_t = Wrapping(0xc000000);

pub(crate) fn A_Tracer(actor: &mut mobj_t) {
	unsafe {
		if gametic & 3 != 0 {
			return;
		}

		// spawn a puff of smoke behind the rocket
		P_SpawnPuff(actor.x, actor.y, actor.z);

		let th = &mut *P_SpawnMobj(
			actor.x - actor.momx,
			actor.y - actor.momy,
			actor.z,
			mobjtype_t::MT_SMOKE,
		);

		th.momz = FRACUNIT;
		th.tics -= P_Random() & 3;
		if th.tics < 1 {
			th.tics = 1;
		}

		// adjust direction
		let dest = actor.tracer;

		if dest.is_null() || (*dest).health <= 0 {
			return;
		}

		let dest = &mut *actor.tracer;

		// change angle
		let exact = R_PointToAngle2(actor.x, actor.y, dest.x, dest.y);

		if exact != actor.angle {
			if (exact - actor.angle).0 > 0x80000000 {
				actor.angle -= TRACEANGLE;
				if (exact - actor.angle).0 < 0x80000000 {
					actor.angle = exact;
				}
			} else {
				actor.angle += TRACEANGLE;
				if (exact - actor.angle).0 > 0x80000000 {
					actor.angle = exact;
				}
			}
		}

		let exact = actor.angle.0 >> ANGLETOFINESHIFT;
		actor.momx = FixedMul((*actor.info).speed, finecos(exact));
		actor.momy = FixedMul((*actor.info).speed, finesine[exact]);

		// change slope
		let mut dist = P_AproxDistance(dest.x - actor.x, dest.y - actor.y);

		dist /= (*actor.info).speed;

		if dist < 1 {
			dist = 1;
		}
		let slope = (dest.z + 40 * FRACUNIT - actor.z) / dist;

		if slope < actor.momz {
			actor.momz -= FRACUNIT / 8;
		} else {
			actor.momz += FRACUNIT / 8;
		}
	}
}

pub(crate) fn A_SkelWhoosh(actor: &mut mobj_t) {
	if actor.target.is_null() {
		return;
	}
	A_FaceTarget(actor);
	S_StartSound((actor as *mut mobj_t).cast(), sfxenum_t::sfx_skeswg);
}

pub(crate) fn A_SkelFist(actor: &mut mobj_t) {
	if actor.target.is_null() {
		return;
	}

	A_FaceTarget(actor);

	if P_CheckMeleeRange(actor) {
		unsafe {
			let damage = ((P_Random() % 10) + 1) * 6;
			S_StartSound((actor as *mut mobj_t).cast(), sfxenum_t::sfx_skepch);
			P_DamageMobj(&mut *actor.target, actor, actor, damage);
		}
	}
}

// PIT_VileCheck
// Detect a corpse that could be raised.
static mut corpsehit: *mut mobj_t = null_mut();
static mut vileobj: *mut mobj_t = null_mut();
static mut viletryx: fixed_t = 0;
static mut viletryy: fixed_t = 0;

unsafe extern "C" {
	fn P_CheckPosition(thing: *const mobj_t, x: fixed_t, y: fixed_t) -> boolean;
}

extern "C" fn PIT_VileCheck(thing: *mut mobj_t) -> boolean {
	unsafe {
		let thing = &mut *thing;

		if thing.flags & MF_CORPSE == 0 {
			return 1; // not a monster
		}

		if thing.tics != -1 {
			return 1; // not lying still yet
		}

		if (*thing.info).raisestate == statenum_t::S_NULL {
			return 1; // monster doesn't have a raise state
		}

		let maxdist = (*thing.info).radius + mobjinfo[mobjtype_t::MT_VILE as usize].radius;

		if fixed_t::abs(thing.x - viletryx) > maxdist || fixed_t::abs(thing.y - viletryy) > maxdist
		{
			return 1; // not actually touching
		}

		corpsehit = thing;
		(*corpsehit).momx = 0;
		(*corpsehit).momy = 0;
		(*corpsehit).height <<= 2;
		let check = P_CheckPosition(corpsehit, (*corpsehit).x, (*corpsehit).y) != 0;
		(*corpsehit).height >>= 2;

		// !check: doesn't fit here
		// check: got one, so stop checking
		(!check) as boolean
	}
}

unsafe extern "C" {
	fn P_BlockThingsIterator(
		x: i32,
		y: i32,
		func: unsafe extern "C" fn(*mut mobj_t) -> boolean,
	) -> boolean;
}

// A_VileChase
// Check for ressurecting a body
pub(crate) fn A_VileChase(actor: &mut mobj_t) {
	unsafe {
		if actor.movedir != dirtype_t::DI_NODIR {
			// check for corpses to raise
			viletryx = actor.x + (*actor.info).speed * xspeed[actor.movedir as usize];
			viletryy = actor.y + (*actor.info).speed * yspeed[actor.movedir as usize];

			let xl = (viletryx - bmaporgx - MAXRADIUS * 2) >> MAPBLOCKSHIFT;
			let xh = (viletryx - bmaporgx + MAXRADIUS * 2) >> MAPBLOCKSHIFT;
			let yl = (viletryy - bmaporgy - MAXRADIUS * 2) >> MAPBLOCKSHIFT;
			let yh = (viletryy - bmaporgy + MAXRADIUS * 2) >> MAPBLOCKSHIFT;

			vileobj = actor;
			for bx in xl..=xh {
				for by in yl..=yh {
					// Call PIT_VileCheck to check
					// whether object is a corpse
					// that canbe raised.
					if P_BlockThingsIterator(bx, by, PIT_VileCheck as _) == 0 {
						// got one!
						let temp = actor.target;
						actor.target = corpsehit;
						A_FaceTarget(actor);
						actor.target = temp;

						P_SetMobjState(actor, statenum_t::S_VILE_HEAL1);
						S_StartSound(corpsehit.cast(), sfxenum_t::sfx_slop);
						let info = (*corpsehit).info;

						P_SetMobjState(&mut *corpsehit, (*info).raisestate);
						(*corpsehit).height <<= 2;
						(*corpsehit).flags = (*info).flags;
						(*corpsehit).health = (*info).spawnhealth;
						(*corpsehit).target = null_mut();

						return;
					}
				}
			}
		}

		// Return to normal attack.
		A_Chase(actor);
	}
}

// A_VileStart
pub(crate) fn A_VileStart(actor: &mut mobj_t) {
	S_StartSound((actor as *mut mobj_t).cast(), sfxenum_t::sfx_vilatk);
}

// A_Fire
// Keep fire in front of player unless out of sight

pub(crate) fn A_StartFire(actor: &mut mobj_t) {
	S_StartSound((actor as *mut mobj_t).cast(), sfxenum_t::sfx_flamst);
	A_Fire(actor);
}

pub(crate) fn A_FireCrackle(actor: &mut mobj_t) {
	S_StartSound((actor as *mut mobj_t).cast(), sfxenum_t::sfx_flame);
	A_Fire(actor);
}

unsafe extern "C" {
	fn P_SetThingPosition(thing: *mut mobj_t);
	fn P_UnsetThingPosition(thing: *mut mobj_t);
}

pub(crate) fn A_Fire(actor: &mut mobj_t) {
	unsafe {
		let dest = actor.tracer;
		if dest.is_null() {
			return;
		}

		// don't move it if the vile lost sight
		if P_CheckSight(&*actor.target, &*dest) == 0 {
			return;
		}

		let an = ((*dest).angle >> ANGLETOFINESHIFT).0;

		P_UnsetThingPosition(actor);
		actor.x = (*dest).x + FixedMul(24 * FRACUNIT, finecos(an));
		actor.y = (*dest).y + FixedMul(24 * FRACUNIT, finesine[an]);
		actor.z = (*dest).z;
		P_SetThingPosition(actor);
	}
}

// A_VileTarget
// Spawn the hellfire
pub(crate) fn A_VileTarget(actor: &mut mobj_t) {
	unsafe {
		if actor.target.is_null() {
			return;
		}

		A_FaceTarget(actor);

		// FIXME: Bug?                                           v
		let fog = P_SpawnMobj(
			(*actor.target).x,
			(*actor.target).x,
			(*actor.target).z,
			mobjtype_t::MT_FIRE,
		);

		actor.tracer = fog;
		(*fog).target = actor;
		(*fog).tracer = actor.target;
		A_Fire(&mut *fog);
	}
}

unsafe extern "C" {
	fn P_RadiusAttack(spot: *mut mobj_t, source: *mut mobj_t, damage: i32);
}

// A_VileAttack
pub(crate) fn A_VileAttack(actor: &mut mobj_t) {
	unsafe {
		if actor.target.is_null() {
			return;
		}

		A_FaceTarget(actor);

		if P_CheckSight(actor, &*actor.target) == 0 {
			return;
		}

		S_StartSound((actor as *mut mobj_t).cast(), sfxenum_t::sfx_barexp);
		P_DamageMobj(&mut *actor.target, actor, actor, 20);
		(*actor.target).momz = 1000 * FRACUNIT / (*(*actor.target).info).mass;

		let fire = actor.tracer;

		if fire.is_null() {
			return;
		}

		let an = (actor.angle >> ANGLETOFINESHIFT).0;

		// move the fire between the vile and the player
		(*fire).x = (*actor.target).x - FixedMul(24 * FRACUNIT, finecos(an));
		(*fire).y = (*actor.target).y - FixedMul(24 * FRACUNIT, finesine[an]);
		P_RadiusAttack(fire, actor, 70);
	}
}

// Mancubus attack,
// firing three missiles (bruisers)
// in three different directions?
// Doesn't look like it.
const FATSPREAD: angle_t = Wrapping(ANG90.0 / 8);

pub(crate) fn A_FatRaise(actor: &mut mobj_t) {
	A_FaceTarget(actor);
	S_StartSound((actor as *mut mobj_t).cast(), sfxenum_t::sfx_manatk);
}

pub(crate) fn A_FatAttack1(actor: &mut mobj_t) {
	unsafe {
		A_FaceTarget(actor);
		// Change direction  to ...
		actor.angle += FATSPREAD;
		let dest = &mut *actor.target;
		P_SpawnMissile(actor, dest, mobjtype_t::MT_FATSHOT);

		let mo = &mut *P_SpawnMissile(actor, dest, mobjtype_t::MT_FATSHOT);
		mo.angle += FATSPREAD;
		let an = (mo.angle >> ANGLETOFINESHIFT).0;
		mo.momx = FixedMul((*mo.info).speed, finecos(an));
		mo.momy = FixedMul((*mo.info).speed, finesine[an]);
	}
}

pub(crate) fn A_FatAttack2(actor: &mut mobj_t) {
	unsafe {
		A_FaceTarget(actor);
		// Now here choose opposite deviation.
		actor.angle -= FATSPREAD;
		let dest = &mut *actor.target;
		P_SpawnMissile(actor, dest, mobjtype_t::MT_FATSHOT);

		let dest = &mut *actor.target;
		let mo = &mut *P_SpawnMissile(actor, dest, mobjtype_t::MT_FATSHOT);
		mo.angle -= FATSPREAD * Wrapping(2);
		let an = (mo.angle >> ANGLETOFINESHIFT).0;
		mo.momx = FixedMul((*mo.info).speed, finecos(an));
		mo.momy = FixedMul((*mo.info).speed, finesine[an]);
	}
}

pub(crate) fn A_FatAttack3(actor: &mut mobj_t) {
	unsafe {
		A_FaceTarget(actor);

		let dest = &mut *actor.target;
		let mo = &mut *P_SpawnMissile(actor, dest, mobjtype_t::MT_FATSHOT);
		mo.angle -= FATSPREAD / Wrapping(2);
		let an = (mo.angle >> ANGLETOFINESHIFT).0;
		mo.momx = FixedMul((*mo.info).speed, finecos(an));
		mo.momy = FixedMul((*mo.info).speed, finesine[an]);

		let mo = &mut *P_SpawnMissile(actor, dest, mobjtype_t::MT_FATSHOT);
		mo.angle += FATSPREAD / Wrapping(2);
		let an = (mo.angle >> ANGLETOFINESHIFT).0;
		mo.momx = FixedMul((*mo.info).speed, finecos(an));
		mo.momy = FixedMul((*mo.info).speed, finesine[an]);
	}
}

// SkullAttack
// Fly at the player like a missile.
const SKULLSPEED: fixed_t = 20 * FRACUNIT;

pub(crate) fn A_SkullAttack(actor: &mut mobj_t) {
	unsafe {
		if actor.target.is_null() {
			return;
		}

		let dest = actor.target;
		actor.flags |= MF_SKULLFLY;

		S_StartSound((actor as *mut mobj_t).cast(), (*actor.info).attacksound);
		A_FaceTarget(actor);
		let an = (actor.angle >> ANGLETOFINESHIFT).0;
		actor.momx = FixedMul(SKULLSPEED, finecos(an));
		actor.momy = FixedMul(SKULLSPEED, finesine[an]);
		let mut dist = P_AproxDistance((*dest).x - actor.x, (*dest).y - actor.y);
		dist /= SKULLSPEED;

		if dist < 1 {
			dist = 1;
		}
		actor.momz = ((*dest).z + ((*dest).height >> 1) - actor.z) / dist;
	}
}

// A_PainShootSkull
// Spawn a lost soul and launch it at the target
fn A_PainShootSkull(actor: &mut mobj_t, angle: angle_t) {
	unsafe {
		// count total number of skull currently on the level
		let mut count = 0;

		let mut currentthinker = thinkercap.next;
		while !std::ptr::eq(currentthinker, &raw const thinkercap) {
			if (*currentthinker).function.is_mobj()
				&& (*(currentthinker as *mut mobj_t)).ty == mobjtype_t::MT_SKULL
			{
				count += 1;
			}
			currentthinker = (*currentthinker).next;
		}

		// if there are allready 20 skulls on the level,
		// don't spit another one
		if count > 20 {
			return;
		}

		// okay, there's playe for another one
		let an = (angle >> ANGLETOFINESHIFT).0;

		let prestep = 4 * FRACUNIT
			+ 3 * ((*actor.info).radius + mobjinfo[mobjtype_t::MT_SKULL as usize].radius) / 2;

		let x = actor.x + FixedMul(prestep, finecos(an));
		let y = actor.y + FixedMul(prestep, finesine[an]);
		let z = actor.z + 8 * FRACUNIT;

		let newmobj = P_SpawnMobj(x, y, z, mobjtype_t::MT_SKULL);

		// Check for movements.
		if P_TryMove(newmobj, (*newmobj).x, (*newmobj).y) == 0 {
			// kill it immediately
			P_DamageMobj(&mut *newmobj, actor, actor, 10000);
			return;
		}

		(*newmobj).target = actor.target;
		A_SkullAttack(&mut *newmobj);
	}
}

// A_PainAttack
// Spawn a lost soul and launch it at the target
pub(crate) fn A_PainAttack(actor: &mut mobj_t) {
	if actor.target.is_null() {
		return;
	}

	A_FaceTarget(actor);
	A_PainShootSkull(actor, actor.angle);
}

pub(crate) fn A_PainDie(actor: &mut mobj_t) {
	A_Fall(actor);
	A_PainShootSkull(actor, actor.angle + ANG90);
	A_PainShootSkull(actor, actor.angle + ANG180);
	A_PainShootSkull(actor, actor.angle + ANG270);
}

pub(crate) fn A_Scream(actor: &mut mobj_t) {
	let sound = match unsafe { (*actor.info).deathsound } {
		sfxenum_t::sfx_None => return,

		sfxenum_t::sfx_podth1 | sfxenum_t::sfx_podth2 | sfxenum_t::sfx_podth3 => {
			[sfxenum_t::sfx_podth1, sfxenum_t::sfx_podth2, sfxenum_t::sfx_podth3]
				[(P_Random() % 3) as usize]
		}

		sfxenum_t::sfx_bgdth1 | sfxenum_t::sfx_bgdth2 => {
			[sfxenum_t::sfx_bgdth1, sfxenum_t::sfx_podth2][(P_Random() % 2) as usize]
		}

		deathsound => deathsound,
	};

	// Check for bosses.
	if actor.ty == mobjtype_t::MT_SPIDER || actor.ty == mobjtype_t::MT_CYBORG {
		// full volume
		S_StartSound(null_mut(), sound);
	} else {
		S_StartSound((actor as *mut mobj_t).cast(), sound);
	}
}

pub(crate) fn A_XScream(actor: &mut mobj_t) {
	S_StartSound((actor as *mut mobj_t).cast(), sfxenum_t::sfx_slop);
}

pub(crate) fn A_Pain(actor: &mut mobj_t) {
	unsafe {
		if (*actor.info).painsound != sfxenum_t::sfx_None {
			S_StartSound((actor as *mut mobj_t).cast(), (*actor.info).painsound);
		}
	}
}

pub(crate) fn A_Fall(actor: &mut mobj_t) {
	// actor is on ground, it can be walked over
	actor.flags &= !MF_SOLID;

	// So change this if corpse objects
	// are meant to be obstacles.
}

// A_Explode
pub(crate) fn A_Explode(thingy: &mut mobj_t) {
	unsafe { P_RadiusAttack(thingy, thingy.target, 128) };
}

// A_BossDeath
// Possibly trigger special effects
// if on first boss level
pub(crate) fn A_BossDeath(mo: &mut mobj_t) {
	unsafe {
		if gamemode == GameMode_t::commercial {
			if gamemap != 7 {
				return;
			}
			if mo.ty != mobjtype_t::MT_FATSO && mo.ty != mobjtype_t::MT_BABY {
				return;
			}
		} else {
			match gameepisode {
				1 => {
					if gamemap != 8 {
						return;
					}
					if mo.ty != mobjtype_t::MT_BRUISER {
						return;
					}
				}
				2 => {
					if gamemap != 8 {
						return;
					}
					if mo.ty != mobjtype_t::MT_CYBORG {
						return;
					}
				}
				3 => {
					if gamemap != 8 {
						return;
					}
					if mo.ty != mobjtype_t::MT_SPIDER {
						return;
					}
				}
				4 => match gamemap {
					6 => {
						if mo.ty != mobjtype_t::MT_CYBORG {
							return;
						}
					}
					8 => {
						if mo.ty != mobjtype_t::MT_SPIDER {
							return;
						}
					}
					_ => return,
				},
				_ => {
					if gamemap != 8 {
						return;
					}
				}
			}
		}

		// make sure there is a player alive for victory
		if !(0..MAXPLAYERS).any(|i| playeringame[i] != 0 && players[i].health > 0) {
			return; // no one left alive, so do not end game
		}

		// scan the remaining thinkers to see
		// if all bosses are dead
		let mut th = thinkercap.next;
		while !std::ptr::eq(th, &raw const thinkercap) {
			if !(*th).function.is_mobj() {
				th = (*th).next;
				continue;
			}

			let mo2 = &*(th as *mut mobj_t);
			if !std::ptr::eq(mo2, mo) && mo2.ty == mo.ty && mo2.health > 0 {
				// other boss not dead
				return;
			}
			th = (*th).next;
		}

		let mut junk = line_t::default();
		// victory!
		if gamemode == GameMode_t::commercial {
			if gamemap == 7 {
				if mo.ty == mobjtype_t::MT_FATSO {
					junk.tag = 666;
					EV_DoFloor(&mut junk, floor_e::lowerFloorToLowest);
					return;
				}

				if mo.ty == mobjtype_t::MT_BABY {
					junk.tag = 667;
					EV_DoFloor(&mut junk, floor_e::raiseToTexture);
					return;
				}
			}
		} else {
			match gameepisode {
				1 => {
					junk.tag = 666;
					EV_DoFloor(&mut junk, floor_e::lowerFloorToLowest);
					return;
				}

				4 => match gamemap {
					6 => {
						junk.tag = 666;
						EV_DoDoor(&mut junk, vldoor_e::blazeOpen);
						return;
					}

					8 => {
						junk.tag = 666;
						EV_DoFloor(&mut junk, floor_e::lowerFloorToLowest);
						return;
					}

					_ => (),
				},

				_ => unreachable!(),
			}
		}

		G_ExitLevel();
	}
}

pub(crate) fn A_Hoof(mo: &mut mobj_t) {
	S_StartSound((mo as *mut mobj_t).cast(), sfxenum_t::sfx_hoof);
	A_Chase(mo);
}

pub(crate) fn A_Metal(mo: &mut mobj_t) {
	S_StartSound((mo as *mut mobj_t).cast(), sfxenum_t::sfx_metal);
	A_Chase(mo);
}

pub(crate) fn A_BabyMetal(mo: &mut mobj_t) {
	S_StartSound((mo as *mut mobj_t).cast(), sfxenum_t::sfx_bspwlk);
	A_Chase(mo);
}

pub(crate) fn A_OpenShotgun2(player: &mut player_t, _psp: &mut pspdef_t) {
	S_StartSound(player.mo.cast(), sfxenum_t::sfx_dbopn);
}

pub(crate) fn A_LoadShotgun2(player: &mut player_t, _psp: &mut pspdef_t) {
	S_StartSound(player.mo.cast(), sfxenum_t::sfx_dbload);
}

pub(crate) fn A_CloseShotgun2(player: &mut player_t, psp: &mut pspdef_t) {
	S_StartSound(player.mo.cast(), sfxenum_t::sfx_dbcls);
	A_ReFire(player, psp);
}

static mut braintargets: [*mut mobj_t; 32] = [null_mut(); 32];
static mut numbraintargets: usize = 0;
static mut braintargeton: usize = 0;

pub(crate) fn A_BrainAwake(_mo: &mut mobj_t) {
	unsafe {
		// find all the target spots
		numbraintargets = 0;
		braintargeton = 0;

		let mut thinker = thinkercap.next;
		while !std::ptr::eq(thinker, &raw const thinkercap) {
			if (*thinker).function.is_mobj() {
				thinker = (*thinker).next;
				continue; // not a mobj
			}

			let m = &mut *thinker.cast::<mobj_t>();

			if m.ty == mobjtype_t::MT_BOSSTARGET {
				braintargets[numbraintargets] = m;
				numbraintargets += 1;
			}
			thinker = (*thinker).next;
		}

		S_StartSound(null_mut(), sfxenum_t::sfx_bossit);
	}
}

pub(crate) fn A_BrainPain(_mo: &mut mobj_t) {
	S_StartSound(null_mut(), sfxenum_t::sfx_bospn);
}

pub(crate) fn A_BrainScream(mo: &mut mobj_t) {
	for x in ((mo.x - 196 * FRACUNIT)..(mo.x + 320 * FRACUNIT)).step_by(FRACUNIT as usize * 8) {
		let y = mo.y - 320 * FRACUNIT;
		let z = 128 + P_Random() * 2 * FRACUNIT;
		let th = unsafe { &mut *P_SpawnMobj(x, y, z, mobjtype_t::MT_ROCKET) };
		th.momz = P_Random() * 512;

		P_SetMobjState(th, statenum_t::S_BRAINEXPLODE1);

		th.tics -= P_Random() & 7;
		if th.tics < 1 {
			th.tics = 1;
		}
	}

	S_StartSound(null_mut(), sfxenum_t::sfx_bosdth);
}

pub(crate) fn A_BrainExplode(mo: &mut mobj_t) {
	let x = mo.x + (P_Random() - P_Random()) * 2048;
	let y = mo.y;
	let z = 128 + P_Random() * 2 * FRACUNIT;
	let th = unsafe { &mut *P_SpawnMobj(x, y, z, mobjtype_t::MT_ROCKET) };
	th.momz = P_Random() * 512;

	P_SetMobjState(th, statenum_t::S_BRAINEXPLODE1);

	th.tics -= P_Random() & 7;
	if th.tics < 1 {
		th.tics = 1;
	}
}

pub(crate) fn A_BrainDie(_mo: &mut mobj_t) {
	G_ExitLevel();
}

pub(crate) fn A_BrainSpit(mo: &mut mobj_t) {
	unsafe {
		static mut easy: bool = false;

		easy = !easy;
		if gameskill <= skill_t::sk_easy && !easy {
			return;
		}

		// shoot a cube at current target
		let targ = &mut *braintargets[braintargeton];
		braintargeton = (braintargeton + 1) % numbraintargets;

		// spawn brain missile
		let newmobj = &mut *P_SpawnMissile(mo, targ, mobjtype_t::MT_SPAWNSHOT);
		newmobj.target = targ;
		newmobj.reactiontime = ((targ.y - mo.y) / newmobj.momy) / (*newmobj.state).tics;

		S_StartSound(null_mut(), sfxenum_t::sfx_bospit);
	}
}

// travelling cube sound
pub(crate) fn A_SpawnSound(mo: &mut mobj_t) {
	S_StartSound((mo as *mut mobj_t).cast(), sfxenum_t::sfx_boscub);
	A_SpawnFly(mo);
}

unsafe extern "C" {
	fn P_TeleportMove(thing: *mut mobj_t, x: fixed_t, y: fixed_t) -> bool;
}

pub(crate) fn A_SpawnFly(mo: &mut mobj_t) {
	unsafe {
		mo.reactiontime -= 1;
		if mo.reactiontime != 0 {
			return; // still flying
		}

		let targ = &mut *mo.target;

		// First spawn teleport fog.
		let fog = P_SpawnMobj(targ.x, targ.y, targ.z, mobjtype_t::MT_SPAWNFIRE);
		S_StartSound(fog.cast(), sfxenum_t::sfx_telept);

		// Randomly select monster to spawn.
		let r = P_Random() as u8;

		// Probability distribution (kind of :),
		// decreasing likelihood.
		let ty = match r {
			0..50 => mobjtype_t::MT_TROOP,
			50..90 => mobjtype_t::MT_SERGEANT,
			90..120 => mobjtype_t::MT_SHADOWS,
			120..130 => mobjtype_t::MT_PAIN,
			130..160 => mobjtype_t::MT_HEAD,
			160..162 => mobjtype_t::MT_VILE,
			162..172 => mobjtype_t::MT_UNDEAD,
			172..192 => mobjtype_t::MT_BABY,
			192..222 => mobjtype_t::MT_FATSO,
			222..246 => mobjtype_t::MT_KNIGHT,
			246.. => mobjtype_t::MT_BRUISER,
		};

		let newmobj = &mut *P_SpawnMobj(targ.x, targ.y, targ.z, ty);
		if P_LookForPlayers(newmobj, true) {
			P_SetMobjState(newmobj, (*newmobj.info).seestate);
		}

		// telefrag anything in this spot
		P_TeleportMove(newmobj, newmobj.x, newmobj.y);

		// remove self (i.e., cube).
		P_RemoveMobj(mo);
	}
}

pub(crate) fn A_PlayerScream(mo: &mut mobj_t) {
	// Default death sound.
	let mut sound = sfxenum_t::sfx_pldeth;

	unsafe {
		if gamemode == GameMode_t::commercial && mo.health < -50 {
			// IF THE PLAYER DIES
			// LESS THAN -50% WITHOUT GIBBING
			sound = sfxenum_t::sfx_pdiehi;
		}
	}

	S_StartSound((mo as *mut mobj_t).cast(), sound);
}
