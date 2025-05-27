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

use crate::{
	d_player::player_t,
	d_think::thinker_t,
	doomdata::mapthing_t,
	info::{mobjinfo_t, mobjtype_t, spritenum_t, state_t},
	r_defs::subsector_t,
};

type fixed_t = i32;
type angle_t = u32;

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
