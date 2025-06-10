// NOTES: mobj_t
//
// mobj_ts are used to tell the refresh where to draw an image,
// tell the world simulation when objects are contacted,
// and tell the sound driver how to position a sound.
//
// The refresh uses the next and prev links to follow
// lists of things in sectors as they are being drawn.
// The sprite, frame, and angle elements determine which patch_t
// is used to draw the sprite if it is visible.
// The sprite and frame values are allmost allways set
// from state_t structures.
// The statescr.exe utility generates the states.h and states.c
// files that contain the sprite/frame numbers from the
// statescr.txt source file.
// The xyz origin point represents a point at the bottom middle
// of the sprite (between the feet of a biped).
// This is the default origin position for patch_ts grabbed
// with lumpy.exe.
// A walking creature will have its z equal to the floor
// it is standing on.
//
// The sound code uses the x,y, and subsector fields
// to do stereo positioning of any sound effited by the mobj_t.
//
// The play simulation uses the blocklinks, x,y,z, radius, height
// to determine when mobj_ts are touching each other,
// touching lines in the map, or hit by trace lines (gunshots,
// lines of sight, etc).
// The mobj_t->flags element has various bit flags
// used by the simulation.
//
// Every mobj_t is linked into a single sector
// based on its origin coordinates.
// The subsector_t is found with R_PointInSubsector(x,y),
// and the sector_t can be found with subsector->sector.
// The sector links are only used by the rendering code,
// the play simulation does not care about them at all.
//
// Any mobj_t that needs to be acted upon by something else
// in the play world (block movement, be shot, etc) will also
// need to be linked into the blockmap.
// If the thing has the MF_NOBLOCK flag set, it will not use
// the block links. It can still interact with other things,
// but only as the instigator (missiles will run into other
// things, but nothing can run into a missile).
// Each block in the grid is 128*128 units, and knows about
// every line_t that it contains a piece of, and every
// interactable mobj_t that has its origin contained.
//
// A valid mobj_t is a mobj_t that has the proper subsector_t
// filled in for its xy coordinates and is linked into the
// sector from which the subsector was made, or has the
// MF_NOSECTOR flag set (the subsector_t needs to be valid
// even if MF_NOSECTOR is set), and is linked into a blockmap
// block or has the MF_NOBLOCKMAP flag set.
// Links should only be modified by the P_[Un]SetThingPosition()
// functions.
// Do not change the MF_NO? flags while a thing is valid.
//
// Any questions?

#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::{ffi::c_void, mem::transmute, ptr::null_mut};

use crate::{
	d_main::nomonsters,
	d_player::{cheat_t, player_t, playerstate_t},
	d_think::thinker_t,
	doomdata::mapthing_t,
	doomdef::{MAXPLAYERS, MTF_AMBUSH, card_t, skill_t},
	g_game::{
		G_PlayerReborn, consoleplayer, deathmatch, gameskill, netgame, playeringame, players,
		respawnmonsters, totalitems, totalkills,
	},
	hu_stuff::HU_Start,
	i_system::I_Error,
	info::{mobjinfo, mobjinfo_t, mobjtype_t, spritenum_t, state_t, statenum_t, states},
	m_fixed::{FRACBITS, FRACUNIT, FixedMul, fixed_t},
	m_random::P_Random,
	p_local::*,
	p_setup::{deathmatch_p, deathmatchstarts, playerstarts},
	p_tick::{P_AddThinker, P_RemoveThinker, leveltime},
	r_defs::{line_t, subsector_t},
	r_sky::skyflatnum,
	s_sound::{S_StartSound, S_StopSound},
	sounds::sfxenum_t,
	tables::{ANG45, ANGLETOFINESHIFT, angle_t, finecos, finesine},
	z_zone::*,
};

type boolean = i32;

// Misc. mobj flags

// Call P_SpecialThing when touched.
pub const MF_SPECIAL: u32 = 1;
// Blocks.
pub const MF_SOLID: u32 = 2;
// Can be hit.
pub const MF_SHOOTABLE: u32 = 4;
// Don't use the sector links (invisible but touchable).
pub const MF_NOSECTOR: u32 = 8;
// Don't use the blocklinks (inert but displayable)
pub const MF_NOBLOCKMAP: u32 = 16;

// Not to be activated by sound, deaf monster.
pub const MF_AMBUSH: u32 = 32;
// Will try to attack right back.
pub const MF_JUSTHIT: u32 = 64;
// Will take at least one step before attacking.
pub const MF_JUSTATTACKED: u32 = 128;
// On level spawning (initial position),
//  hang from ceiling instead of stand on floor.
pub const MF_SPAWNCEILING: u32 = 256;
// Don't apply gravity (every tic),
//  that is, object will float, keeping current height
//  or changing it actively.
pub const MF_NOGRAVITY: u32 = 512;

// Movement flags.
// This allows jumps from high places.
pub const MF_DROPOFF: u32 = 0x400;
// For players, will pick up items.
pub const MF_PICKUP: u32 = 0x800;
// Player cheat. ???
pub const MF_NOCLIP: u32 = 0x1000;
// Player: keep info about sliding along walls.
pub const MF_SLIDE: u32 = 0x2000;
// Allow moves to any height, no gravity.
// For active floaters, e.g. cacodemons, pain elementals.
pub const MF_FLOAT: u32 = 0x4000;
// Don't cross lines
//   ??? or look at heights on teleport.
pub const MF_TELEPORT: u32 = 0x8000;
// Don't hit same species, explode on block.
// Player missiles as well as fireballs of various kinds.
pub const MF_MISSILE: u32 = 0x10000;
// Dropped by a demon, not level spawned.
// E.g. ammo clips dropped by dying former humans.
pub const MF_DROPPED: u32 = 0x20000;
// Use fuzzy draw (shadow demons or spectres),
//  temporary player invisibility powerup.
pub const MF_SHADOW: u32 = 0x40000;
// Flag: don't bleed when shot (use puff),
//  barrels and shootable furniture shall not bleed.
pub const MF_NOBLOOD: u32 = 0x80000;
// Don't stop moving halfway off a step,
//  that is, have dead bodies slide down all the way.
pub const MF_CORPSE: u32 = 0x100000;
// Floating to a height for a move, ???
//  don't auto float to target's height.
pub const MF_INFLOAT: u32 = 0x200000;

// On kill, count this enemy object
//  towards intermission kill total.
// Happy gathering.
pub const MF_COUNTKILL: u32 = 0x400000;

// On picking up, count this item object
//  towards intermission item total.
pub const MF_COUNTITEM: u32 = 0x800000;

// Special handling: skull in flight.
// Neither a cacodemon nor a missile.
pub const MF_SKULLFLY: u32 = 0x1000000;

// Don't spawn this object
//  in death match mode (e.g. key cards).
pub const MF_NOTDMATCH: u32 = 0x2000000;

// Player sprites in multiplayer modes are modified
//  using an internal color lookup table for re-indexing.
// If 0x4 0x8 or 0xc,
//  use a translation table for player colormaps
pub const MF_TRANSLATION: u32 = 0xc000000;
// Hmm ???.
pub const MF_TRANSSHIFT: u32 = 26;

// Map Object definition.
#[repr(C)]
pub struct mobj_t {
	// List: thinker links.
	pub thinker: thinker_t,

	// Info for drawing: position.
	pub x: fixed_t,
	pub y: fixed_t,
	pub z: fixed_t,

	// More list: links in sector (if needed)
	pub snext: *mut mobj_t,
	pub sprev: *mut mobj_t,

	//More drawing info: to determine current sprite.
	pub angle: angle_t,      // orientation
	pub sprite: spritenum_t, // used to find patch_t and flip value
	pub frame: i32,          // might be ORed with FF_FULLBRIGHT

	// Interaction info, by BLOCKMAP.
	// Links in blocks (if needed).
	pub bnext: *mut mobj_t,
	pub bprev: *mut mobj_t,

	pub subsector: *mut subsector_t,

	// The closest interval over all contacted Sectors.
	pub floorz: fixed_t,
	pub ceilingz: fixed_t,

	// For movement checking.
	pub radius: fixed_t,
	pub height: fixed_t,

	// Momentums, used to update position.
	pub momx: fixed_t,
	pub momy: fixed_t,
	pub momz: fixed_t,

	// If == validcount, already checked.
	pub validcount: i32,

	pub ty: mobjtype_t,
	pub info: *mut mobjinfo_t, // &mobjinfo[mobj->type]

	pub tics: i32, // state tic counter
	pub state: *mut state_t,
	pub flags: u32,
	pub health: i32,

	// Movement direction, movement generation (zig-zagging).
	pub movedir: i32,   // 0-7
	pub movecount: i32, // when 0, select a new dir

	// Thing being chased/attacked (or NULL),
	// also the originator for missiles.
	pub target: *mut mobj_t,

	// Reaction time: if non 0, don't attack yet.
	// Used by player to freeze a bit after teleporting.
	pub reactiontime: i32,

	// If >0, the target will be chased
	// no matter what (even if shot)
	pub threshold: i32,

	// Additional info record for player avatars only.
	// Only valid if type == MT_PLAYER
	pub player: *mut player_t,

	// Player number last looked for.
	pub lastlook: i32,

	// For nightmare respawn.
	pub spawnpoint: mapthing_t,

	// Thing being chased/attacked for tracers.
	pub tracer: *mut mobj_t,
}

// P_SetMobjState
// Returns true if the mobj is still present.
#[unsafe(no_mangle)]
pub extern "C" fn P_SetMobjState(mobj: &mut mobj_t, mut state: statenum_t) -> boolean {
	// state_t*	st;

	loop {
		if state == statenum_t::S_NULL {
			mobj.state = null_mut();
			P_RemoveMobj(mobj);
			return 0;
		}

		let st = unsafe { &mut states[state as usize] };
		mobj.state = st;
		mobj.tics = st.tics;
		mobj.sprite = st.sprite;
		mobj.frame = st.frame;

		// Modified handling.
		// Call action functions when the state is set
		unsafe {
			if let Some(acp1) = st.action.acp1 {
				acp1((mobj as *mut mobj_t).cast());
			}
		}

		state = st.nextstate;
		if mobj.tics != 0 {
			break;
		}
	}

	1
}

// P_ExplodeMissile
fn P_ExplodeMissile(mo: &mut mobj_t) {
	unsafe {
		mo.momx = 0;
		mo.momy = 0;
		mo.momz = 0;

		P_SetMobjState(mo, mobjinfo[mo.ty as usize].deathstate);

		mo.tics -= P_Random() & 3;

		if mo.tics < 1 {
			mo.tics = 1;
		}

		mo.flags &= !MF_MISSILE;

		if (*mo.info).deathsound != sfxenum_t::sfx_None {
			S_StartSound((mo as *mut mobj_t).cast(), (*mo.info).deathsound);
		}
	}
}

unsafe extern "C" {
	static mut ceilingline: *mut line_t;

	fn P_TryMove(thing: *mut mobj_t, x: fixed_t, y: fixed_t) -> boolean;
	fn P_SlideMove(thing: *mut mobj_t) -> boolean;
}

// P_XYMovement
const STOPSPEED: i32 = 0x1000;
const FRICTION: i32 = 0xe800;

fn P_XYMovement(mo: &mut mobj_t) {
	unsafe {
		if mo.momx == 0 && mo.momy == 0 {
			if mo.flags & MF_SKULLFLY != 0 {
				// the skull slammed into something
				mo.flags &= !MF_SKULLFLY;
				mo.momx = 0;
				mo.momy = 0;
				mo.momz = 0;

				P_SetMobjState(mo, (*mo.info).spawnstate);
			}
			return;
		}

		let player = mo.player;

		mo.momx = mo.momx.clamp(-MAXMOVE, MAXMOVE);
		mo.momy = mo.momy.clamp(-MAXMOVE, MAXMOVE);

		let mut xmove = mo.momx;
		let mut ymove = mo.momy;

		loop {
			let ptryx;
			let ptryy;
			if xmove > MAXMOVE / 2 || ymove > MAXMOVE / 2 {
				ptryx = mo.x + xmove / 2;
				ptryy = mo.y + ymove / 2;
				xmove >>= 1;
				ymove >>= 1;
			} else {
				ptryx = mo.x + xmove;
				ptryy = mo.y + ymove;
				xmove = 0;
				ymove = 0;
			}

			if P_TryMove(mo, ptryx, ptryy) == 0 {
				// blocked move
				if !mo.player.is_null() {
					// try to slide along it
					P_SlideMove(mo);
				} else if mo.flags & MF_MISSILE != 0 {
					// explode a missile
					if !ceilingline.is_null()
						&& !(*ceilingline).backsector.is_null()
						&& (*(*ceilingline).backsector).ceilingpic == skyflatnum as i16
					{
						// Hack to prevent missiles exploding
						// against the sky.
						// Does not handle sky floors.
						P_RemoveMobj(mo);
						return;
					}
					P_ExplodeMissile(mo);
				} else {
					mo.momx = 0;
					mo.momy = 0;
				}
			}
			if xmove == 0 && ymove == 0 {
				break;
			}
		}

		// slow down
		if !player.is_null() && (*player).cheats & cheat_t::CF_NOMOMENTUM as i32 != 0 {
			// debug option for no sliding at all
			mo.momx = 0;
			mo.momy = 0;
			return;
		}

		if mo.flags & (MF_MISSILE | MF_SKULLFLY) != 0 {
			return; // no friction for missiles ever
		}

		if mo.z > mo.floorz {
			return; // no friction when airborne
		}

		if mo.flags & MF_CORPSE != 0 {
			// do not stop sliding
			//  if halfway off a step with some momentum
			if (mo.momx > FRACUNIT / 4
				|| mo.momx < -FRACUNIT / 4
				|| mo.momy > FRACUNIT / 4
				|| mo.momy < -FRACUNIT / 4)
				&& mo.floorz != (*(*mo.subsector).sector).floorheight
			{
				return;
			}
		}

		if mo.momx > -STOPSPEED
			&& mo.momx < STOPSPEED
			&& mo.momy > -STOPSPEED
			&& mo.momy < STOPSPEED
			&& (player.is_null()
			|| ((*player).cmd.forwardmove == 0 && (*player).cmd.sidemove == 0))
		{
			// if in a walking frame, stop moving
			if !player.is_null()
				&& (((*(*player).mo).state.offset_from(&raw mut states[0])) as usize)
					< statenum_t::S_PLAY_ATK1 as usize
			{
				P_SetMobjState(&mut *(*player).mo, statenum_t::S_PLAY);
			}

			mo.momx = 0;
			mo.momy = 0;
		} else {
			mo.momx = FixedMul(mo.momx, FRICTION);
			mo.momy = FixedMul(mo.momy, FRICTION);
		}
	}
}

unsafe extern "C" {
	fn P_AproxDistance(x: fixed_t, y: fixed_t) -> fixed_t;
}

// P_ZMovement
fn P_ZMovement(mo: &mut mobj_t) {
	unsafe {
		// check for smooth step up
		if !mo.player.is_null() && mo.z < mo.floorz {
			(*mo.player).viewheight -= mo.floorz - mo.z;

			(*mo.player).deltaviewheight = (VIEWHEIGHT - (*mo.player).viewheight) >> 3;
		}

		// adjust height
		mo.z += mo.momz;

		if mo.flags & MF_FLOAT != 0 && !mo.target.is_null() {
			// float down towards target if too close
			if mo.flags & MF_SKULLFLY == 0 && mo.flags & MF_INFLOAT == 0 {
				let dist = P_AproxDistance(mo.x - (*mo.target).x, mo.y - (*mo.target).y);

				let delta = ((*mo.target).z + (mo.height >> 1)) - mo.z;

				if delta < 0 && dist < -delta * 3 {
					mo.z -= FLOATSPEED;
				} else if delta > 0 && dist < delta * 3 {
					mo.z += FLOATSPEED;
				}
			}
		}

		// clip movement
		if mo.z <= mo.floorz {
			// hit the floor

			// Note (id):
			//  somebody left this after the setting momz to 0,
			//  kinda useless there.
			if mo.flags & MF_SKULLFLY != 0 {
				// the skull slammed into something
				mo.momz = -mo.momz;
			}

			if mo.momz < 0 {
				if !mo.player.is_null() && mo.momz < -GRAVITY * 8 {
					// Squat down.
					// Decrease viewheight for a moment
					// after hitting the ground (hard),
					// and utter appropriate sound.
					(*mo.player).deltaviewheight = mo.momz >> 3;
					S_StartSound((mo as *mut mobj_t).cast(), sfxenum_t::sfx_oof);
				}
				mo.momz = 0;
			}
			mo.z = mo.floorz;

			if mo.flags & MF_MISSILE != 0 && mo.flags & MF_NOCLIP == 0 {
				P_ExplodeMissile(mo);
				return;
			}
		} else if mo.flags & MF_NOGRAVITY == 0 {
			if mo.momz == 0 {
				mo.momz = -GRAVITY * 2;
			} else {
				mo.momz -= GRAVITY;
			}
		}

		if mo.z + mo.height > mo.ceilingz {
			// hit the ceiling
			if mo.momz > 0 {
				mo.momz = 0;
			}
			mo.z = mo.ceilingz - mo.height;

			if mo.flags & MF_SKULLFLY != 0 {
				// the skull slammed into something
				mo.momz = -mo.momz;
			}

			if mo.flags & MF_MISSILE != 0 && !mo.flags & MF_NOCLIP == 0 {
				P_ExplodeMissile(mo);
				// return;
			}
		}
	}
}

unsafe extern "C" {
	fn P_CheckPosition(thing: *const mobj_t, x: fixed_t, y: fixed_t) -> boolean;
	fn R_PointInSubsector(x: fixed_t, y: fixed_t) -> *mut subsector_t;
}

// P_NightmareRespawn
fn P_NightmareRespawn(mobj: &mut mobj_t) {
	unsafe {
		// fixed_t		x;
		// fixed_t		y;
		// fixed_t		z;
		// subsector_t*	ss;
		// mobj_t*		mo;
		// mapthing_t*		mthing;

		let x = (mobj.spawnpoint.x as i32) << FRACBITS;
		let y = (mobj.spawnpoint.y as i32) << FRACBITS;

		// somthing is occupying it's position?
		if P_CheckPosition(mobj, x, y) == 0 {
			return; // no respwan
		}

		// spawn a teleport fog at old spot
		// because of removal of the body?
		let mo = P_SpawnMobj(
			mobj.x,
			mobj.y,
			(*(*mobj.subsector).sector).floorheight,
			mobjtype_t::MT_TFOG,
		);
		// initiate teleport sound
		S_StartSound(mo.cast(), sfxenum_t::sfx_telept);

		// spawn a teleport fog at the new spot
		let ss = R_PointInSubsector(x, y);

		let mo = P_SpawnMobj(x, y, (*(*ss).sector).floorheight, mobjtype_t::MT_TFOG);

		S_StartSound(mo.cast(), sfxenum_t::sfx_telept);

		// spawn the new monster
		let mthing = &mobj.spawnpoint;

		// spawn it
		let z = if (*mobj.info).flags & MF_SPAWNCEILING != 0 { ONCEILINGZ } else { ONFLOORZ };

		// inherit attributes from deceased one
		let mo = P_SpawnMobj(x, y, z, mobj.ty);
		(*mo).spawnpoint = mobj.spawnpoint;
		(*mo).angle = ANG45 * (mthing.angle as angle_t / 45);

		if mthing.options as u8 & MTF_AMBUSH != 0 {
			(*mo).flags |= MF_AMBUSH;
		}

		(*mo).reactiontime = 18;

		// remove the old monster,
		P_RemoveMobj(mobj);
	}
}

// P_MobjThinker
#[unsafe(no_mangle)]
pub extern "C" fn P_MobjThinker(mobj: &mut mobj_t) {
	unsafe {
		// momentum movement
		if mobj.momx != 0 || mobj.momy != 0 || mobj.flags & MF_SKULLFLY != 0 {
			P_XYMovement(mobj);

			// FIXME: decent NOP/NULL/Nil function pointer please.
			if mobj.thinker.function.acv.is_none() {
				return; // mobj was removed
			}
		}
		if mobj.z != mobj.floorz || mobj.momz != 0 {
			P_ZMovement(mobj);

			// FIXME: decent NOP/NULL/Nil function pointer please.
			if mobj.thinker.function.acv.is_none() {
				return; // mobj was removed
			}
		}

		// cycle through states,
		// calling action functions at transitions
		if mobj.tics != -1 {
			mobj.tics -= 1;

			// you can cycle through multiple states in a tic
			if mobj.tics == 0 {
				P_SetMobjState(mobj, (*mobj.state).nextstate);
			}
		} else {
			// check for nightmare respawn
			if mobj.flags & MF_COUNTKILL == 0 {
				return;
			}

			if respawnmonsters == 0 {
				return;
			}

			mobj.movecount += 1;

			if mobj.movecount < 12 * 35 {
				return;
			}

			if leveltime & 31 != 0 {
				return;
			}

			if P_Random() > 4 {
				return;
			}

			P_NightmareRespawn(mobj);
		}
	}
}

// P_SpawnMobj

unsafe extern "C" {
	fn P_SetThingPosition(thing: *mut mobj_t);
}

#[unsafe(no_mangle)]
pub extern "C" fn P_SpawnMobj(x: fixed_t, y: fixed_t, z: fixed_t, ty: mobjtype_t) -> *mut mobj_t {
	unsafe {
		let mobj = Z_Malloc(size_of::<mobj_t>(), PU_LEVEL, null_mut());
		libc::memset(mobj, 0, size_of::<mobj_t>());
		let info = &mut mobjinfo[ty as usize];
		let mobj = &mut *(mobj as *mut mobj_t);

		mobj.ty = ty;
		mobj.info = info;
		mobj.x = x;
		mobj.y = y;
		mobj.radius = info.radius;
		mobj.height = info.height;
		mobj.flags = info.flags;
		mobj.health = info.spawnhealth;

		if gameskill != skill_t::sk_nightmare {
			mobj.reactiontime = info.reactiontime;
		}

		mobj.lastlook = P_Random() % MAXPLAYERS as i32;
		// do not set the state with P_SetMobjState,
		// because action routines can not be called yet
		let st = &mut states[info.spawnstate as usize];

		mobj.state = st;
		mobj.tics = st.tics;
		mobj.sprite = st.sprite;
		mobj.frame = st.frame;

		// set subsector and/or block links
		P_SetThingPosition(mobj);

		mobj.floorz = (*(*mobj.subsector).sector).floorheight;
		mobj.ceilingz = (*(*mobj.subsector).sector).ceilingheight;

		if z == ONFLOORZ {
			mobj.z = mobj.floorz;
		} else if z == ONCEILINGZ {
			mobj.z = mobj.ceilingz - (*mobj.info).height;
		} else {
			mobj.z = z;
		}

		mobj.thinker.function.acp1 = Some(transmute::<
			extern "C" fn(&mut mobj_t),
			unsafe extern "C" fn(*mut c_void),
		>(P_MobjThinker as extern "C" fn(&mut mobj_t)));

		P_AddThinker(&mut mobj.thinker);

		mobj
	}
}

unsafe extern "C" {
	fn P_UnsetThingPosition(thing: *mut mobj_t);
}

// P_RemoveMobj
static mut itemrespawnque: [mapthing_t; ITEMQUESIZE] =
	[mapthing_t { x: 0, y: 0, angle: 0, ty: 0, options: 0 }; ITEMQUESIZE];
static mut itemrespawntime: [usize; ITEMQUESIZE] = [0; ITEMQUESIZE];
pub(crate) static mut iquehead: usize = 0;
pub(crate) static mut iquetail: usize = 0;

#[unsafe(no_mangle)]
pub extern "C" fn P_RemoveMobj(mobj: &mut mobj_t) {
	if mobj.flags & MF_SPECIAL != 0
		&& mobj.flags & MF_DROPPED == 0
		&& mobj.ty != mobjtype_t::MT_INV
		&& mobj.ty != mobjtype_t::MT_INS
	{
		unsafe {
			itemrespawnque[iquehead] = mobj.spawnpoint;
			itemrespawntime[iquehead] = leveltime;
			iquehead = (iquehead + 1) & (ITEMQUESIZE - 1);

			// lose one off the end?
			if iquehead == iquetail {
				iquetail = (iquetail + 1) & (ITEMQUESIZE - 1);
			}
		}
	}

	// unlink from sector and block lists
	unsafe { P_UnsetThingPosition(mobj) };

	// stop any playing sound
	S_StopSound((mobj as *mut mobj_t).cast());

	// free block
	unsafe { P_RemoveThinker(&mut mobj.thinker) };
}

// P_RespawnSpecials
pub(crate) fn P_RespawnSpecials() {
	unsafe {
		// only respawn items in deathmatch
		if deathmatch != 2 {
			return; //
		}

		// nothing left to respawn?
		if iquehead == iquetail {
			return;
		}

		// wait at least 30 seconds
		if leveltime - itemrespawntime[iquetail] < 30 * 35 {
			return;
		}

		let mthing = &mut itemrespawnque[iquetail];

		let x = (mthing.x as i32) << FRACBITS;
		let y = (mthing.y as i32) << FRACBITS;

		// spawn a teleport fog at the new spot
		let ss = R_PointInSubsector(x, y);
		let mo = P_SpawnMobj(x, y, (*(*ss).sector).floorheight, mobjtype_t::MT_IFOG);
		S_StartSound(mo.cast(), sfxenum_t::sfx_itmbk);

		// find which typy to spawn
		let mut i = mobjtype_t::MT_PLAYER;
		#[allow(clippy::needless_range_loop)]
		for j in 0..mobjtype_t::NUMMOBJTYPES as usize {
			i = mobjtype_t::from(j);
			if mthing.ty as i32 == mobjinfo[j].doomednum {
				break;
			}
		}

		// spawn it
		let z =
			if mobjinfo[i as usize].flags & MF_SPAWNCEILING != 0 { ONCEILINGZ } else { ONFLOORZ };

		let mo = &mut *P_SpawnMobj(x, y, z, i);
		mo.spawnpoint = *mthing;
		mo.angle = ANG45 * (mthing.angle as angle_t / 45);

		// pull it from the que
		iquetail = (iquetail + 1) & (ITEMQUESIZE - 1);
	}
}

unsafe extern "C" {
	fn ST_Start();
	fn P_SetupPsprites(curplayer: *mut player_t);
}

// P_SpawnPlayer
// Called when a player is spawned on the level.
// Most of the player structure stays unchanged
//  between levels.
#[unsafe(no_mangle)]
pub extern "C" fn P_SpawnPlayer(mthing: &mut mapthing_t) {
	unsafe {
		// not playing?
		if playeringame[mthing.ty as usize - 1] == 0 {
			return;
		}

		let p = &mut players[mthing.ty as usize - 1];

		if p.playerstate == playerstate_t::PST_REBORN {
			G_PlayerReborn(mthing.ty as usize - 1);
		}

		let x = (mthing.x as i32) << FRACBITS;
		let y = (mthing.y as i32) << FRACBITS;
		let z = ONFLOORZ;
		let mobj = &mut *P_SpawnMobj(x, y, z, mobjtype_t::MT_PLAYER);

		// set color translations for player sprites
		if mthing.ty > 1 {
			mobj.flags |= (mthing.ty as u32 - 1) << MF_TRANSSHIFT;
		}

		mobj.angle = ANG45 * (mthing.angle as angle_t / 45);
		mobj.player = p;
		mobj.health = p.health;

		p.mo = mobj;
		p.playerstate = playerstate_t::PST_LIVE;
		p.refire = 0;
		p.message = null_mut();
		p.damagecount = 0;
		p.bonuscount = 0;
		p.extralight = 0;
		p.fixedcolormap = 0;
		p.viewheight = VIEWHEIGHT;

		// setup gun psprite
		P_SetupPsprites(p);

		// give all cards in death match mode
		if deathmatch != 0 {
			for i in 0..card_t::NUMCARDS as usize {
				p.cards[i] = 1;
			}
		}

		if mthing.ty as usize - 1 == consoleplayer {
			// wake up the status bar
			ST_Start();
			// wake up the heads up text
			HU_Start();
		}
	}
}

// P_SpawnMapThing
// The fields of the mapthing should
// already be in host byte order.
pub(crate) fn P_SpawnMapThing(mthing: &mut mapthing_t) {
	unsafe {
		// count deathmatch start positions
		if mthing.ty == 11 {
			if deathmatch_p.offset_from(&raw const deathmatchstarts[0]) < 10 {
				libc::memcpy(
					deathmatch_p.cast(),
					(mthing as *mut mapthing_t).cast(),
					size_of::<mapthing_t>(),
				);
				deathmatch_p = deathmatch_p.wrapping_add(1);
			}
			return;
		}

		// check for players specially
		if mthing.ty <= 4 {
			// save spots for respawning in network games
			playerstarts[mthing.ty as usize - 1] = *mthing;
			if deathmatch == 0 {
				P_SpawnPlayer(mthing);
			}

			return;
		}

		// check for apropriate skill level
		if netgame == 0 && mthing.options & 16 != 0 {
			return;
		}

		let bit = if gameskill == skill_t::sk_baby {
			1
		} else if gameskill == skill_t::sk_nightmare {
			4
		} else {
			1 << (gameskill as usize - 1)
		};

		if mthing.options & bit == 0 {
			return;
		}

		// find which type to spawn
		let mut i = mobjtype_t::NUMMOBJTYPES;
		#[allow(clippy::needless_range_loop)]
		for j in 0..mobjtype_t::NUMMOBJTYPES as usize {
			i = mobjtype_t::from(j);
			if mthing.ty as i32 == mobjinfo[j].doomednum {
				break;
			}
		}

		if i == mobjtype_t::NUMMOBJTYPES {
			I_Error(
				c"P_SpawnMapThing: Unknown ty %i at (%i, %i)".as_ptr(),
				mthing.ty as i32,
				mthing.x as i32,
				mthing.y as i32,
			);
		}

		// don't spawn keycards and players in deathmatch
		if deathmatch != 0 && mobjinfo[i as usize].flags & MF_NOTDMATCH != 0 {
			return;
		}

		// don't spawn any monsters if -nomonsters
		if nomonsters != 0
			&& (i == mobjtype_t::MT_SKULL || mobjinfo[i as usize].flags & MF_COUNTKILL != 0)
		{
			return;
		}

		// spawn it
		let x = (mthing.x as i32) << FRACBITS;
		let y = (mthing.y as i32) << FRACBITS;

		let z =
			if mobjinfo[i as usize].flags & MF_SPAWNCEILING != 0 { ONCEILINGZ } else { ONFLOORZ };

		let mobj = &mut *P_SpawnMobj(x, y, z, i);
		mobj.spawnpoint = *mthing;

		if mobj.tics > 0 {
			mobj.tics = 1 + (P_Random() % mobj.tics);
		}
		if mobj.flags & MF_COUNTKILL != 0 {
			totalkills += 1;
		}
		if mobj.flags & MF_COUNTITEM != 0 {
			totalitems += 1;
		}

		mobj.angle = ANG45 * (mthing.angle as angle_t / 45);
		if mthing.options as u8 & MTF_AMBUSH != 0 {
			mobj.flags |= MF_AMBUSH;
		}
	}
}

// GAME SPAWN FUNCTIONS

// P_SpawnPuff
unsafe extern "C" {
	static mut attackrange: fixed_t;
}

#[unsafe(no_mangle)]
pub extern "C" fn P_SpawnPuff(x: fixed_t, y: fixed_t, mut z: fixed_t) {
	unsafe {
		z += (P_Random() - P_Random()) << 10;

		let th = &mut *P_SpawnMobj(x, y, z, mobjtype_t::MT_PUFF);
		th.momz = FRACUNIT;
		th.tics -= P_Random() & 3;

		if th.tics < 1 {
			th.tics = 1;
		}

		// don't make punches spark on the wall
		if attackrange == MELEERANGE {
			P_SetMobjState(th, statenum_t::S_PUFF3);
		}
	}
}

// P_SpawnBlood
#[unsafe(no_mangle)]
pub extern "C" fn P_SpawnBlood(x: fixed_t, y: fixed_t, mut z: fixed_t, damage: i32) {
	unsafe {
		z += (P_Random() - P_Random()) << 10;
		let th = &mut *P_SpawnMobj(x, y, z, mobjtype_t::MT_BLOOD);
		th.momz = FRACUNIT * 2;
		th.tics -= P_Random() & 3;

		if th.tics < 1 {
			th.tics = 1;
		}

		match damage {
			..9 => P_SetMobjState(th, statenum_t::S_BLOOD3),
			9..=12 => P_SetMobjState(th, statenum_t::S_BLOOD2),
			_ => 0,
		};
	}
}

// P_CheckMissileSpawn
// Moves the missile forward a bit
//  and possibly explodes it right there.
fn P_CheckMissileSpawn(th: &mut mobj_t) {
	th.tics -= P_Random() & 3;
	if th.tics < 1 {
		th.tics = 1;
	}

	// move a little forward so an angle can
	// be computed if it immediately explodes
	th.x += th.momx >> 1;
	th.y += th.momy >> 1;
	th.z += th.momz >> 1;

	if unsafe { P_TryMove(th, th.x, th.y) } == 0 {
		P_ExplodeMissile(th);
	}
}

unsafe extern "C" {
	fn R_PointToAngle2(x_1: i32, y_1: i32, x_2: i32, y_2: i32) -> u32;
}

// P_SpawnMissile
#[unsafe(no_mangle)]
pub extern "C" fn P_SpawnMissile(
	source: &mut mobj_t,
	dest: &mut mobj_t,
	ty: mobjtype_t,
) -> *mut mobj_t {
	unsafe {
		let th = &mut *P_SpawnMobj(source.x, source.y, source.z + 4 * 8 * FRACUNIT, ty);

		if (*th.info).seesound != sfxenum_t::sfx_None {
			S_StartSound((th as *mut mobj_t).cast(), (*th.info).seesound);
		}

		th.target = source; // where it came from
		let mut an = R_PointToAngle2(source.x, source.y, dest.x, dest.y);

		// fuzzy player
		if dest.flags & MF_SHADOW != 0 {
			an += ((P_Random() - P_Random()) << 20) as angle_t;
		}

		th.angle = an;
		an >>= ANGLETOFINESHIFT;
		let an = an as usize;
		th.momx = FixedMul((*th.info).speed, finecos(an));
		th.momy = FixedMul((*th.info).speed, finesine[an]);

		let mut dist = P_AproxDistance(dest.x - source.x, dest.y - source.y);
		dist /= (*th.info).speed;

		if dist < 1 {
			dist = 1;
		}

		th.momz = (dest.z - source.z) / dist;
		P_CheckMissileSpawn(th);

		th
	}
}

unsafe extern "C" {
	static mut linetarget: *mut mobj_t;
	fn P_AimLineAttack(t1: *mut mobj_t, angle: angle_t, distance: fixed_t) -> fixed_t;
}

// P_SpawnPlayerMissile
// Tries to aim at a nearby monster
#[unsafe(no_mangle)]
pub extern "C" fn P_SpawnPlayerMissile(source: &mut mobj_t, ty: mobjtype_t) {
	unsafe {
		// see which target is to be aimed at
		let mut an = source.angle;
		let mut slope = P_AimLineAttack(source, an, 16 * 64 * FRACUNIT);

		if linetarget .is_null() {
			an += 1 << 26;
			slope = P_AimLineAttack(source, an, 16 * 64 * FRACUNIT);

			if linetarget.is_null() {
				an -= 2 << 26;
				slope = P_AimLineAttack(source, an, 16 * 64 * FRACUNIT);
			}

			if linetarget.is_null() {
				an = source.angle;
				slope = 0;
			}
		}

		let x = source.x;
		let y = source.y;
		let z = source.z + 4 * 8 * FRACUNIT;

		let th = &mut * P_SpawnMobj(x, y, z, ty);

		if (*th.info).seesound != sfxenum_t::sfx_None {
			S_StartSound((th as *mut mobj_t).cast(), (*th.info).seesound);
		}

		th.target = source;
		th.angle = an;
		th.momx = FixedMul((*th.info).speed, finecos((an >> ANGLETOFINESHIFT) as usize));
		th.momy = FixedMul((*th.info).speed, finesine[(an >> ANGLETOFINESHIFT) as usize]);
		th.momz = FixedMul((*th.info).speed, slope);

		P_CheckMissileSpawn(th);
	}
}
