// INTERNAL MAP TYPES
//  used by play and refresh

#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::{ffi::c_void, ptr::null_mut};

use crate::{d_think::thinker_t, m_fixed::fixed_t, p_mobj::mobj_t, tables::angle_t};

// doomdef.h
const SCREENWIDTH: usize = 320;

// Your plain vanilla vertex.
// Note: transformed values not buffered locally,
//  like some DOOM-alikes ("wt", "WebView") did.
#[repr(C)]
pub(crate) struct vertex_t {
	pub(crate) x: fixed_t,
	pub(crate) y: fixed_t,
}

// Each sector has a degenmobj_t in its center
//  for sound origin purposes.
// I suppose this does not handle sound from
//  moving objects (doppler), because
//  position is prolly just buffered, not
//  updated.
#[repr(C)]
pub(crate) struct degenmobj_t {
	pub(crate) thinker: thinker_t, // not used for anything
	pub(crate) x: fixed_t,
	pub(crate) y: fixed_t,
	pub(crate) z: fixed_t,
}

// The SECTORS record, at runtime.
// Stores things/mobjs.
#[repr(C)]
pub(crate) struct sector_t {
	pub(crate) floorheight: fixed_t,
	pub(crate) ceilingheight: fixed_t,
	pub(crate) floorpic: i16,
	pub(crate) ceilingpic: i16,
	pub(crate) lightlevel: i16,
	pub(crate) special: i16,
	pub(crate) tag: i16,

	// 0 = untraversed, 1,2 = sndlines -1
	pub(crate) soundtraversed: i32,

	// thing that made a sound (or null)
	pub(crate) soundtarget: *mut mobj_t,

	// mapblock bounding box for height changes
	pub(crate) blockbox: [i32; 4],

	// origin for any sounds played by the sector
	pub(crate) soundorg: degenmobj_t,

	// if == validcount, already checked
	pub(crate) validcount: i32,

	// list of mobjs in sector
	pub(crate) thinglist: *mut mobj_t,

	// thinker_t for reversable actions
	pub(crate) specialdata: *mut c_void,

	pub(crate) linecount: usize,
	pub(crate) lines: *mut *mut line_t, // [linecount] size
}

// The SideDef.
#[repr(C)]
pub(crate) struct side_t {
	// add this to the calculated texture column
	pub(crate) textureoffset: fixed_t,

	// add this to the calculated texture top
	pub(crate) rowoffset: fixed_t,

	// Texture indices.
	// We do not maintain names here.
	pub(crate) toptexture: i16,
	pub(crate) bottomtexture: i16,
	pub(crate) midtexture: i16,

	// Sector the SideDef is facing.
	pub(crate) sector: *mut sector_t,
}

// Move clipping aid for LineDefs.
#[repr(C)]
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub(crate) enum slopetype_t {
	ST_HORIZONTAL,
	ST_VERTICAL,
	ST_POSITIVE,
	ST_NEGATIVE,
}

#[repr(C)]
pub(crate) struct line_t {
	// Vertices, from v1 to v2.
	pub(crate) v1: *mut vertex_t,
	pub(crate) v2: *mut vertex_t,

	// Precalculated v2 - v1 for side checking.
	pub(crate) dx: fixed_t,
	pub(crate) dy: fixed_t,

	// Animation related.
	pub(crate) flags: i16,
	pub(crate) special: i16,
	pub(crate) tag: i16,

	// Visual appearance: SideDefs.
	//  sidenum[1] will be -1 if one sided
	pub(crate) sidenum: [i16; 2],

	// Neat. Another bounding box, for the extent
	//  of the LineDef.
	pub(crate) bbox: [fixed_t; 4],

	// To aid move clipping.
	pub(crate) slopetype: slopetype_t,

	// Front and back sector.
	// Note: redundant? Can be retrieved from SideDefs.
	pub(crate) frontsector: *mut sector_t,
	pub(crate) backsector: *mut sector_t,

	// if == validcount, already checked
	pub(crate) validcount: i32,

	// thinker_t for reversable actions
	pub(crate) specialdata: *mut c_void,
}

impl Default for line_t {
	fn default() -> Self {
		Self {
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
		}
	}
}

// A SubSector.
// References a Sector.
// Basically, this is a list of LineSegs,
//  indicating the visible walls that define
//  (all or some) sides of a convex BSP leaf.
#[repr(C)]
pub(crate) struct subsector_t {
	pub(crate) sector: *mut sector_t,
	pub(crate) numlines: i16,
	pub(crate) firstline: i16,
}

// The LineSeg.
#[repr(C)]
pub(crate) struct seg_t {
	pub(crate) v1: *mut vertex_t,
	pub(crate) v2: *mut vertex_t,

	pub(crate) offset: fixed_t,

	pub(crate) angle: angle_t,

	pub(crate) sidedef: *mut side_t,
	pub(crate) linedef: *mut line_t,

	// Sector references.
	// Could be retrieved from linedef, too.
	// backsector is NULL for one sided lines
	pub(crate) frontsector: *mut sector_t,
	pub(crate) backsector: *mut sector_t,
}

// BSP node.
#[repr(C)]
pub(crate) struct node_t {
	// Partition line.
	pub(crate) x: fixed_t,
	pub(crate) y: fixed_t,
	pub(crate) dx: fixed_t,
	pub(crate) dy: fixed_t,

	// Bounding box for each child.
	pub(crate) bbox: [[fixed_t; 4]; 2],

	// If NF_SUBSECTOR its a subsector.
	pub(crate) children: [u16; 2],
}

// posts are runs of non masked source pixels
#[repr(C)]
#[derive(Debug)]
pub(crate) struct post_t {
	pub(crate) topdelta: u8, // -1 is the last post in a column
	pub(crate) length: u8,   // length data bytes follows
}

// column_t is a list of 0 or more post_t, (byte)-1 terminated
pub(crate) type column_t = post_t;

// OTHER TYPES

// This could be wider for >8 bit display.
// Indeed, true color support is posibble
//  precalculating 24bpp lightmap/colormap LUT.
//  from darkening PLAYPAL to all black.
// Could even us emore than 32 levels.
pub(crate) type lighttable_t = i8;

// Silhouette, needed for clipping Segs (mainly)
// and sprites representing things.
pub(crate) const SIL_BOTTOM: usize = 1;
pub(crate) const SIL_TOP: usize = 2;
pub(crate) const SIL_BOTH: usize = 3;

pub(crate) const MAXDRAWSEGS: usize = 256;

// ?
#[repr(C)]
pub(crate) struct drawseg_t {
	pub(crate) curline: *mut seg_t,
	pub(crate) x1: usize,
	pub(crate) x2: usize,

	pub(crate) scale1: fixed_t,
	pub(crate) scale2: fixed_t,
	pub(crate) scalestep: fixed_t,

	// 0=none, 1=bottom, 2=top, 3=both
	pub(crate) silhouette: usize,

	// do not clip sprites above this
	pub(crate) bsilheight: fixed_t,

	// do not clip sprites below this
	pub(crate) tsilheight: fixed_t,

	// Pointers to lists for sprite clipping,
	//  all three adjusted so [x1] is first value.
	pub(crate) sprtopclip: *mut i16,
	pub(crate) sprbottomclip: *mut i16,
	pub(crate) maskedtexturecol: *mut i16,
}

// Patches.
// A patch holds one or more columns.
// Patches are used for sprites and all masked pictures,
// and we compose textures from the TEXTURE1/2 lists
// of patches.
#[repr(C)]
#[derive(Debug)]
pub(crate) struct patch_t {
	pub(crate) width: u16, // bounding box size
	pub(crate) height: u16,
	pub(crate) leftoffset: i16, // pixels to the left of origin
	pub(crate) topoffset: i16,  // pixels below the origin
	pub(crate) columnofs: [usize; 8], // only [width] used
	                            // the [0] is &columnofs[width]
}

// A vissprite_t is a thing
//  that will be drawn during a refresh.
// I.e. a sprite object that is partly visible.
#[repr(C)]
pub(crate) struct vissprite_t {
	// Doubly linked list.
	pub(crate) prev: *mut vissprite_t,
	pub(crate) next: *mut vissprite_t,

	pub(crate) x1: usize,
	pub(crate) x2: usize,

	// for line side calculation
	pub(crate) gx: fixed_t,
	pub(crate) gy: fixed_t,

	// global bottom / top for silhouette clipping
	pub(crate) gz: fixed_t,
	pub(crate) gzt: fixed_t,

	// horizontal position of x1
	pub(crate) startfrac: fixed_t,

	pub(crate) scale: fixed_t,

	// negative if flipped
	pub(crate) xiscale: fixed_t,

	pub(crate) texturemid: fixed_t,
	pub(crate) patch: usize,

	// for color translation and shadow draw,
	//  maxbright frames as well
	pub(crate) colormap: *mut lighttable_t,

	pub(crate) mobjflags: u32,
}

// Sprites are patches with a special naming convention
//  so they can be recognized by R_InitSprites.
// The base name is NNNNFx or NNNNFxFx, with
//  x indicating the rotation, x = 0, 1-7.
// The sprite and frame specified by a thing_t
//  is range checked at run time.
// A sprite is a patch_t that is assumed to represent
//  a three dimensional object and may have multiple
//  rotations pre drawn.
// Horizontal flipping is used to save space,
//  thus NNNNF2F5 defines a mirrored patch.
// Some sprites will only have one picture used
// for all views: NNNNF0
//
#[repr(C)]
pub(crate) struct spriteframe_t {
	// If false use 0 for any position.
	// Note: as eight entries are available,
	//  we might as well insert the same name eight times.
	pub(crate) rotate: i32,

	// Lump to use for view angles 0-7.
	pub(crate) lump: [i16; 8],

	// Flip bit (1 = flip) to use for view angles 0-7.
	pub(crate) flip: [i8; 8],
}

// A sprite definition:
//  a number of animation frames.
#[repr(C)]
pub(crate) struct spritedef_t {
	pub(crate) numframes: i32,
	pub(crate) spriteframes: *mut spriteframe_t,
}

// Now what is a visplane, anyway?
#[repr(C)]
pub(crate) struct visplane_t {
	pub(crate) height: fixed_t,
	pub(crate) picnum: usize,
	pub(crate) lightlevel: i32,
	pub(crate) minx: isize,
	pub(crate) maxx: isize,

	// leave pads for [minx-1]/[maxx+1]
	pub(crate) pad1: i8,
	// Here lies the rub for all
	//  dynamic resize/change of resolution.
	pub(crate) top: [u8; SCREENWIDTH],
	pub(crate) pad2: i8,
	pub(crate) pad3: i8,
	// See above.
	pub(crate) bottom: [u8; SCREENWIDTH],
	pub(crate) pad4: i8,
}
