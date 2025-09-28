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
pub struct vertex_t {
	pub x: fixed_t,
	pub y: fixed_t,
}

// Each sector has a degenmobj_t in its center
//  for sound origin purposes.
// I suppose this does not handle sound from
//  moving objects (doppler), because
//  position is prolly just buffered, not
//  updated.
#[repr(C)]
pub struct degenmobj_t {
	pub thinker: thinker_t, // not used for anything
	pub x: fixed_t,
	pub y: fixed_t,
	pub z: fixed_t,
}

// The SECTORS record, at runtime.
// Stores things/mobjs.
#[repr(C)]
pub struct sector_t {
	pub floorheight: fixed_t,
	pub ceilingheight: fixed_t,
	pub floorpic: i16,
	pub ceilingpic: i16,
	pub lightlevel: i16,
	pub special: i16,
	pub tag: i16,

	// 0 = untraversed, 1,2 = sndlines -1
	pub soundtraversed: i32,

	// thing that made a sound (or null)
	pub soundtarget: *mut mobj_t,

	// mapblock bounding box for height changes
	pub blockbox: [i32; 4],

	// origin for any sounds played by the sector
	pub soundorg: degenmobj_t,

	// if == validcount, already checked
	pub validcount: i32,

	// list of mobjs in sector
	pub thinglist: *mut mobj_t,

	// thinker_t for reversable actions
	pub specialdata: *mut c_void,

	pub linecount: usize,
	pub lines: *mut *mut line_t, // [linecount] size
}

// The SideDef.
#[repr(C)]
pub struct side_t {
	// add this to the calculated texture column
	pub textureoffset: fixed_t,

	// add this to the calculated texture top
	pub rowoffset: fixed_t,

	// Texture indices.
	// We do not maintain names here.
	pub toptexture: i16,
	pub bottomtexture: i16,
	pub midtexture: i16,

	// Sector the SideDef is facing.
	pub sector: *mut sector_t,
}

// Move clipping aid for LineDefs.
#[repr(C)]
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum slopetype_t {
	ST_HORIZONTAL,
	ST_VERTICAL,
	ST_POSITIVE,
	ST_NEGATIVE,
}

#[repr(C)]
pub struct line_t {
	// Vertices, from v1 to v2.
	pub v1: *mut vertex_t,
	pub v2: *mut vertex_t,

	// Precalculated v2 - v1 for side checking.
	pub dx: fixed_t,
	pub dy: fixed_t,

	// Animation related.
	pub flags: i16,
	pub special: i16,
	pub tag: i16,

	// Visual appearance: SideDefs.
	//  sidenum[1] will be -1 if one sided
	pub sidenum: [i16; 2],

	// Neat. Another bounding box, for the extent
	//  of the LineDef.
	pub bbox: [fixed_t; 4],

	// To aid move clipping.
	pub slopetype: slopetype_t,

	// Front and back sector.
	// Note: redundant? Can be retrieved from SideDefs.
	pub frontsector: *mut sector_t,
	pub backsector: *mut sector_t,

	// if == validcount, already checked
	pub validcount: i32,

	// thinker_t for reversable actions
	pub specialdata: *mut c_void,
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
pub struct subsector_t {
	pub sector: *mut sector_t,
	pub numlines: i16,
	pub firstline: i16,
}

// The LineSeg.
#[repr(C)]
pub struct seg_t {
	pub v1: *mut vertex_t,
	pub v2: *mut vertex_t,

	pub offset: fixed_t,

	pub angle: angle_t,

	pub sidedef: *mut side_t,
	pub linedef: *mut line_t,

	// Sector references.
	// Could be retrieved from linedef, too.
	// backsector is NULL for one sided lines
	pub frontsector: *mut sector_t,
	pub backsector: *mut sector_t,
}

// BSP node.
#[repr(C)]
pub struct node_t {
	// Partition line.
	pub x: fixed_t,
	pub y: fixed_t,
	pub dx: fixed_t,
	pub dy: fixed_t,

	// Bounding box for each child.
	pub bbox: [[fixed_t; 4]; 2],

	// If NF_SUBSECTOR its a subsector.
	pub children: [u16; 2],
}

// posts are runs of non masked source pixels
#[repr(C)]
#[derive(Debug)]
pub struct post_t {
	pub topdelta: u8, // -1 is the last post in a column
	pub length: u8,   // length data bytes follows
}

// column_t is a list of 0 or more post_t, (byte)-1 terminated
pub type column_t = post_t;

// OTHER TYPES

// This could be wider for >8 bit display.
// Indeed, true color support is posibble
//  precalculating 24bpp lightmap/colormap LUT.
//  from darkening PLAYPAL to all black.
// Could even us emore than 32 levels.
pub type lighttable_t = i8;

// ?
#[repr(C)]
pub struct drawseg_t {
	curline: *mut seg_t,
	x1: i32,
	x2: i32,

	scale1: fixed_t,
	scale2: fixed_t,
	scalestep: fixed_t,

	// 0=none, 1=bottom, 2=top, 3=both
	silhouette: i32,

	// do not clip sprites above this
	bsilheight: fixed_t,

	// do not clip sprites below this
	tsilheight: fixed_t,

	// Pointers to lists for sprite clipping,
	//  all three adjusted so [x1] is first value.
	sprtopclip: *mut i16,
	sprbottomclip: *mut i16,
	maskedtexturecol: *mut i16,
}

// Patches.
// A patch holds one or more columns.
// Patches are used for sprites and all masked pictures,
// and we compose textures from the TEXTURE1/2 lists
// of patches.
#[repr(C)]
#[derive(Debug)]
pub struct patch_t {
	pub width: i16, // bounding box size
	pub height: i16,
	pub leftoffset: i16, // pixels to the left of origin
	pub topoffset: i16,  // pixels below the origin
	pub columnofs: [usize; 8], // only [width] used
	                     // the [0] is &columnofs[width]
}

// A vissprite_t is a thing
//  that will be drawn during a refresh.
// I.e. a sprite object that is partly visible.
#[repr(C)]
pub struct vissprite_t {
	// Doubly linked list.
	prev: *mut vissprite_t,
	next: *mut vissprite_t,

	x1: i32,
	x2: i32,

	// for line side calculation
	gx: fixed_t,
	gy: fixed_t,

	// global bottom / top for silhouette clipping
	gz: fixed_t,
	gzt: fixed_t,

	// horizontal position of x1
	startfrac: fixed_t,

	scale: fixed_t,

	// negative if flipped
	xiscale: fixed_t,

	texturemid: fixed_t,
	patch: i32,

	// for color translation and shadow draw,
	//  maxbright frames as well
	colormap: *mut lighttable_t,

	mobjflags: i32,
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
pub struct spriteframe_t {
	// If false use 0 for any position.
	// Note: as eight entries are available,
	//  we might as well insert the same name eight times.
	pub rotate: i32,

	// Lump to use for view angles 0-7.
	pub lump: [i16; 8],

	// Flip bit (1 = flip) to use for view angles 0-7.
	pub flip: [i8; 8],
}

// A sprite definition:
//  a number of animation frames.
#[repr(C)]
pub struct spritedef_t {
	pub numframes: i32,
	pub spriteframes: *mut spriteframe_t,
}

// Now what is a visplane, anyway?
#[repr(C)]
pub struct visplane_t {
	height: fixed_t,
	picnum: i32,
	lightlevel: i32,
	minx: i32,
	maxx: i32,

	// leave pads for [minx-1]/[maxx+1]
	pad1: i8,
	// Here lies the rub for all
	//  dynamic resize/change of resolution.
	top: [i8; SCREENWIDTH],
	pad2: i8,
	pad3: i8,
	// See above.
	bottom: [i8; SCREENWIDTH],
	pad4: i8,
}
