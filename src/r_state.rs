#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

// Refresh internal data structures,
//  for rendering.

use crate::r_defs::{node_t, sector_t, seg_t, subsector_t};

/*
// needed for texture pegging
#[unsafe(no_mangle)]
pub static mut textureheight: *mut fixed_t = null_mut();

// needed for pre rendering (fracs)
#[unsafe(no_mangle)]
pub static mut spritewidth: *mut fixed_t = null_mut();

#[unsafe(no_mangle)]
pub static mut spriteoffset: *mut fixed_t = null_mut();
#[unsafe(no_mangle)]
pub static mut spritetopoffset: *mut fixed_t = null_mut();

#[unsafe(no_mangle)]
pub static mut colormaps: *mut lighttable_t = null_mut();

#[unsafe(no_mangle)]
pub static mut viewwidth: i32 = 0;
*/
/*
#[unsafe(no_mangle)]
pub static mut viewheight: i32 = 0;

#[unsafe(no_mangle)]
pub static mut firstflat: i32 = 0;

// for global animation
#[unsafe(no_mangle)]
pub static mut flattranslation: *mut i32 = null_mut();
#[unsafe(no_mangle)]
pub static mut texturetranslation: *mut i32 = null_mut();

// Sprite....
#[unsafe(no_mangle)]
pub static mut firstspritelump: i32 = 0;
#[unsafe(no_mangle)]
pub static mut lastspritelump: i32 = 0;
#[unsafe(no_mangle)]
pub static mut numspritelumps: i32 = 0;

// Lookup tables for map data.
#[unsafe(no_mangle)]
pub static mut numsprites: i32 = 0;
#[unsafe(no_mangle)]
pub static mut sprites: *mut spritedef_t = null_mut();

#[unsafe(no_mangle)]
pub static mut numvertexes: i32 = 0;
#[unsafe(no_mangle)]
pub static mut vertexes: *mut vertex_t = null_mut();
*/

unsafe extern "C" {
	pub static mut numsegs: i32;
	pub static mut segs: *mut seg_t;

	pub static mut numsectors: i32;
	pub static mut sectors: *mut sector_t;

	pub static mut numsubsectors: i32;
	pub static mut subsectors: *mut subsector_t;

	pub static mut numnodes: i32;
	pub static mut nodes: *mut node_t;
}

/*
#[unsafe(no_mangle)]
pub static mut numsegs: i32 = 0;
#[unsafe(no_mangle)]
pub static mut segs: *mut seg_t = null_mut();

#[unsafe(no_mangle)]
pub static mut numsectors: i32 = 0;
#[unsafe(no_mangle)]
pub static mut sectors: *mut sector_t = null_mut();

#[unsafe(no_mangle)]
pub static mut numsubsectors: i32 = 0;
#[unsafe(no_mangle)]
pub static mut subsectors: *mut subsector_t = null_mut();

#[unsafe(no_mangle)]
pub static mut numnodes: i32 = 0;
#[unsafe(no_mangle)]
pub static mut nodes: *mut node_t = null_mut();

#[unsafe(no_mangle)]
pub static mut numlines: i32 = 0;
#[unsafe(no_mangle)]
pub static mut lines: *mut line_t = null_mut();

#[unsafe(no_mangle)]
pub static mut numsides: i32 = 0;
#[unsafe(no_mangle)]
pub static mut sides: *mut side_t = null_mut();

// POV data.
#[unsafe(no_mangle)]
pub static mut viewx: fixed_t = 0;
#[unsafe(no_mangle)]
pub static mut viewy: fixed_t = 0;
#[unsafe(no_mangle)]
pub static mut viewz: fixed_t = 0;

#[unsafe(no_mangle)]
pub static mut viewangle: angle_t = 0;
#[unsafe(no_mangle)]
pub static mut viewplayer: *mut player_t = null_mut();

// ?
#[unsafe(no_mangle)]
pub static mut clipangle: angle_t = 0;

#[unsafe(no_mangle)]
pub static mut viewangletox: [i32; FINEANGLES / 2] = [0; FINEANGLES / 2];
#[unsafe(no_mangle)]
pub static mut xtoviewangle: [angle_t; SCREENWIDTH + 1] = [0; SCREENWIDTH + 1];

#[unsafe(no_mangle)]
pub static mut rw_distance: fixed_t = 0;
#[unsafe(no_mangle)]
pub static mut rw_normalangle: angle_t = 0;

// angle to line origin
#[unsafe(no_mangle)]
pub static mut rw_angle1: i32 = 0;

// Segs count?
#[unsafe(no_mangle)]
pub static mut sscount: i32 = 0;

#[unsafe(no_mangle)]
pub static mut floorplane: *mut visplane_t = null_mut();
#[unsafe(no_mangle)]
pub static mut ceilingplane: *mut visplane_t = null_mut();
*/
