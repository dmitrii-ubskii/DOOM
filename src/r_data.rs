#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

// Graphics.
// DOOM graphics for walls and sprites
// is stored in vertical runs of opaque pixels (posts).
// A column is composed of zero or more posts,
// a patch or sprite is composed of zero or more columns.

use std::{ffi::c_char, ptr::null_mut};

use crate::{
	g_game::demoplayback,
	i_system::I_Error,
	m_fixed::{FRACBITS, fixed_t},
	p_local::thinkercap,
	p_mobj::mobj_t,
	p_setup::{numsectors, numsides, sectors, sides},
	r_defs::{column_t, lighttable_t, patch_t, spritedef_t},
	r_sky::skytexture,
	w_wad::{
		W_CacheLumpName, W_CacheLumpNum, W_CheckNumForName, W_GetNumForName, W_LumpLength,
		W_ReadLump, lumpinfo,
	},
	z_zone::{PU_CACHE, PU_STATIC, Z_ChangeTag, Z_Free, Z_Malloc},
};

type byte = u8;
type short = i16;
type int = i32;
type boolean = i32;

// Texture definition.
// Each texture is composed of one or more patches,
// with patches being lumps stored in the WAD.
// The lumps are referenced by number, and patched
// into the rectangular texture space using origin
// and possibly other attributes.
#[repr(C)]
#[derive(Debug)]
struct mappatch_t {
	pub originx: short,
	pub originy: short,
	pub patch: short,
	pub stepdir: short,
	pub colormap: short,
}

// Texture definition.
// A DOOM wall texture is a list of patches
// which are to be combined in a predefined order.
#[repr(C)]
#[derive(Debug)]
struct maptexture_t {
	pub name: [u8; 8],
	pub masked: boolean,
	pub width: short,
	pub height: short,
	pub columndirectory: *mut *mut (), // OBSOLETE
	pub patchcount: short,
	pub patches: [mappatch_t; 1],
}

// A single patch from a texture definition,
//  basically a rectangular area within
//  the texture rectangle.
#[repr(C)]
#[derive(Debug)]
struct texpatch_t {
	// Block origin (allways UL),
	// which has allready accounted
	// for the internal origin of the patch.
	pub originx: int,
	pub originy: int,
	pub patch: isize,
}

// A maptexturedef_t describes a rectangular texture,
//  which is composed of one or more mappatch_t structures
//  that arrange graphic patches.
#[repr(C)]
#[derive(Debug)]
struct texture_t {
	// Keep name for switch changing, etc.
	pub name: [u8; 8],
	pub width: short,
	pub height: short,

	// All the patches[patchcount]
	//  are drawn back to front into the cached texture.
	pub patchcount: short,
	pub patches: [texpatch_t; 1],
}

#[unsafe(no_mangle)]
pub static mut firstflat: usize = 0;
#[unsafe(no_mangle)]
pub static mut lastflat: usize = 0;
static mut numflats: usize = 0;

static mut firstpatch: usize = 0;
static mut lastpatch: usize = 0;
static mut numpatches: usize = 0;

#[unsafe(no_mangle)]
pub static mut firstspritelump: usize = 0;
#[unsafe(no_mangle)]
pub static mut lastspritelump: usize = 0;
#[unsafe(no_mangle)]
pub static mut numspritelumps: usize = 0;

static mut numtextures: usize = 0;
static mut textures: *mut *mut texture_t = null_mut();

static mut texturewidthmask: *mut usize = null_mut();
// needed for texture pegging
#[unsafe(no_mangle)]
pub static mut textureheight: *mut fixed_t = null_mut();
static mut texturecompositesize: *mut usize = null_mut();
static mut texturecolumnlump: *mut *mut short = null_mut();
static mut texturecolumnofs: *mut *mut u16 = null_mut();
static mut texturecomposite: *mut *mut byte = null_mut();

// for global animation
#[unsafe(no_mangle)]
pub static mut flattranslation: *mut usize = null_mut();
#[unsafe(no_mangle)]
pub static mut texturetranslation: *mut usize = null_mut();

// needed for pre rendering
#[unsafe(no_mangle)]
pub static mut spritewidth: *mut fixed_t = null_mut();
#[unsafe(no_mangle)]
pub static mut spriteoffset: *mut fixed_t = null_mut();
#[unsafe(no_mangle)]
pub static mut spritetopoffset: *mut fixed_t = null_mut();

#[unsafe(no_mangle)]
pub static mut colormaps: *mut lighttable_t = null_mut();

// MAPTEXTURE_T CACHING
// When a texture is first needed,
//  it counts the number of composite columns
//  required in the texture and allocates space
//  for a column directory and any new columns.
// The directory will simply point inside other patches
//  if there is only one patch in a given column,
//  but any columns with multiple patches
//  will have new column_ts generated.

// R_DrawColumnInCache
// Clip and draw a column
//  from a patch into a cached post.
fn R_DrawColumnInCache(mut patch: &mut column_t, cache: &mut u8, originy: i32, cacheheight: isize) {
	unsafe {
		while patch.topdelta != 0xff {
			let source = (patch as *mut column_t as *mut u8).wrapping_add(3);
			let mut count = patch.length as isize;
			let mut position = originy as isize + patch.topdelta as isize;

			if position < 0 {
				count += position;
				position = 0;
			}

			if position + count > cacheheight {
				count = cacheheight - position;
			}

			if count > 0 {
				libc::memcpy(
					(cache as *mut u8).wrapping_add(position as usize).cast(),
					source.cast(),
					count as usize,
				);
			}

			patch = &mut *(patch as *mut column_t).wrapping_byte_add(patch.length as usize + 4);
		}
	}
}

// R_GenerateComposite
// Using the texture definition,
//  the composite texture is created from the patches,
//  and each column is cached.
fn R_GenerateComposite(texnum: usize) {
	unsafe {
		let texture = &mut **textures.wrapping_add(texnum);

		let block = Z_Malloc(
			*texturecompositesize.wrapping_add(texnum),
			PU_STATIC,
			texturecomposite.wrapping_add(texnum).cast(),
		);

		let collump = *texturecolumnlump.wrapping_add(texnum);
		let colofs = *texturecolumnofs.wrapping_add(texnum);

		// Composite the columns together.
		let patch = texture.patches.as_ptr();
		for i in 0..texture.patchcount as usize {
			let patch = &*patch.wrapping_add(i);
			let realpatch = &mut *(W_CacheLumpNum(patch.patch as usize, PU_CACHE) as *mut patch_t);
			let x1 = patch.originx;
			let mut x2 = x1 + realpatch.width as i32;

			let mut x = if x1 < 0 { 0 } else { x1 };

			if x2 > texture.width as i32 {
				x2 = texture.width as i32;
			}

			while x < x2 {
				// Column does not have multiple patches?
				if *collump.wrapping_add(x as usize) >= 0 {
					x += 1;
					continue;
				}

				let patchcol = (realpatch as *mut patch_t as *mut u8)
					.wrapping_add(*realpatch.columnofs.as_ptr().wrapping_add((x - x1) as usize))
					as *mut column_t;
				R_DrawColumnInCache(
					&mut *patchcol,
					&mut *block.wrapping_add(*colofs.wrapping_add(x as usize) as usize).cast(),
					patch.originy,
					texture.height as isize,
				);
				x += 1;
			}
		}

		// Now that the texture has been built in column cache,
		//  it is purgable from zone memory.
		Z_ChangeTag!(block, PU_CACHE);
	}
}

// R_GenerateLookup
fn R_GenerateLookup(texnum: usize) {
	unsafe {
		let texture = &mut **textures.wrapping_add(texnum);

		// Composited texture not created yet.
		*texturecomposite.wrapping_add(texnum) = null_mut();

		*texturecompositesize.wrapping_add(texnum) = 0;
		let collump = *texturecolumnlump.wrapping_add(texnum);
		let colofs = *texturecolumnofs.wrapping_add(texnum);

		// Now count the number of columns
		//  that are covered by more than one patch.
		// Fill in the lump / offset, so columns
		//  with only a single patch are all done.
		let mut patchcount = vec![0; texture.width as usize];

		let patch = texture.patches.as_ptr();
		for i in 0..texture.patchcount as usize {
			let patch = &*patch.wrapping_add(i);
			let realpatch = &mut *(W_CacheLumpNum(patch.patch as usize, PU_CACHE) as *mut patch_t);
			let x1 = patch.originx;
			let mut x2 = x1 + realpatch.width as i32;

			let mut x = if x1 < 0 { 0 } else { x1 };

			if x2 > texture.width as i32 {
				x2 = texture.width as i32;
			}

			while x < x2 {
				patchcount[x as usize] += 1;
				*collump.wrapping_add(x as usize) = patch.patch as i16;
				*colofs.wrapping_add(x as usize) =
					(*realpatch.columnofs.as_ptr().wrapping_add((x - x1) as usize)) as u16 + 3;
				x += 1;
			}
		}

		#[allow(clippy::needless_range_loop)]
		for x in 0..texture.width as usize {
			if patchcount[x] == 0 {
				println!(
					"R_GenerateLookup: column without a patch ({})",
					std::str::from_utf8(&texture.name).unwrap()
				);
				return;
			}
			if patchcount[x] > 1 {
				// Use the cached block.
				*collump.wrapping_add(x) = -1;
				*colofs.wrapping_add(x) = (*texturecompositesize.wrapping_add(texnum)) as u16;

				if *texturecompositesize.wrapping_add(texnum) > 0x10000 - texture.height as usize {
					I_Error(c"R_GenerateLookup: texture %i is >64k".as_ptr(), texnum);
				}

				*texturecompositesize.wrapping_add(texnum) += texture.height as usize;
			}
		}
	}
}

// R_GetColumn
#[unsafe(no_mangle)]
pub extern "C" fn R_GetColumn(tex: usize, mut col: usize) -> *mut u8 {
	unsafe {
		col &= *texturewidthmask.wrapping_add(tex);
		let lump = *(*texturecolumnlump.wrapping_add(tex)).wrapping_add(col);
		let ofs = *(*texturecolumnofs.wrapping_add(tex)).wrapping_add(col) as usize;

		if lump > 0 {
			return W_CacheLumpNum(lump as usize, PU_CACHE).wrapping_byte_add(ofs).cast();
		}

		if (*texturecomposite.wrapping_add(tex)).is_null() {
			R_GenerateComposite(tex);
		}

		(*texturecomposite.wrapping_add(tex)).wrapping_add(ofs)
	}
}

// R_InitTextures
// Initializes the texture list
//  with the textures from the world map.
fn R_InitTextures() {
	unsafe {
		// Load the patch names from pnames.lmp.
		let mut name = [0; 9];
		let names = W_CacheLumpName(c"PNAMES".as_ptr(), PU_STATIC) as *mut c_char;
		let nummappatches = *(names as *const usize);
		let name_p = names.wrapping_add(4);
		let mut patchlookup = vec![0isize; nummappatches];

		#[allow(clippy::needless_range_loop)]
		for i in 0..nummappatches {
			libc::strncpy(name.as_mut_ptr().cast(), name_p.wrapping_add(i * 8), 8);
			patchlookup[i] = W_CheckNumForName(name.as_ptr());
		}
		Z_Free(names.cast());

		// Load the map texture definitions from textures.lmp.
		// The data is contained in one or two lumps,
		//  TEXTURE1 for shareware, plus TEXTURE2 for commercial.
		let mut maptex = W_CacheLumpName(c"TEXTURE1".as_ptr(), PU_STATIC) as *mut usize;
		let maptex1 = maptex;
		let numtextures1 = *maptex;
		let mut maxoff = W_LumpLength(W_GetNumForName(c"TEXTURE1".as_ptr()) as usize);
		let mut directory = maptex.wrapping_add(1);

		let maptex2;
		let numtextures2;
		let maxoff2;
		if W_CheckNumForName(c"TEXTURE2".as_ptr()) != -1 {
			maptex2 = W_CacheLumpName(c"TEXTURE2".as_ptr(), PU_STATIC) as *mut usize;
			numtextures2 = *maptex2;
			maxoff2 = W_LumpLength(W_GetNumForName(c"TEXTURE2".as_ptr()) as usize);
		} else {
			maptex2 = null_mut();
			numtextures2 = 0;
			maxoff2 = 0;
		}
		numtextures = numtextures1 + numtextures2;

		textures = Z_Malloc(numtextures * 4, PU_STATIC, null_mut()).cast();
		texturecolumnlump = Z_Malloc(numtextures * 4, PU_STATIC, null_mut()).cast();
		texturecolumnofs = Z_Malloc(numtextures * 4, PU_STATIC, null_mut()).cast();
		texturecomposite = Z_Malloc(numtextures * 4, PU_STATIC, null_mut()).cast();
		texturecompositesize = Z_Malloc(numtextures * 4, PU_STATIC, null_mut()).cast();
		texturewidthmask = Z_Malloc(numtextures * 4, PU_STATIC, null_mut()).cast();
		textureheight = Z_Malloc(numtextures * 4, PU_STATIC, null_mut()).cast();

		//	Really complex printing shit...
		let temp1 = W_GetNumForName(c"S_START".as_ptr()); // P_???????
		let temp2 = W_GetNumForName(c"S_END".as_ptr()) - 1;
		let temp3 = ((temp2 - temp1 + 63) as usize / 64) + numtextures.div_ceil(64);
		print!("[");
		for _ in 0..temp3 {
			print!(" ");
		}
		print!("         ]");
		for _ in 0..temp3 {
			print!("\x08");
		}
		print!("\x08\x08\x08\x08\x08\x08\x08\x08\x08\x08");

		for i in 0..numtextures {
			if i & 63 == 0 {
				print!(".");
			}

			if i == numtextures1 {
				// Start looking in second texture file.
				maptex = maptex2;
				maxoff = maxoff2;
				directory = maptex.wrapping_add(1);
			}

			let offset = *directory;

			if offset > maxoff {
				I_Error(c"R_InitTextures: bad texture directory".as_ptr());
			}

			let mtexture = &mut *(maptex.wrapping_byte_add(offset) as *mut maptexture_t);

			let texture = Z_Malloc(
				size_of::<texture_t>()
					+ size_of::<texpatch_t>() * (mtexture.patchcount - 1) as usize,
				PU_STATIC,
				null_mut(),
			)
			.cast();
			*textures.wrapping_add(i) = texture;
			let texture = &mut *texture;

			texture.width = mtexture.width;
			texture.height = mtexture.height;
			texture.patchcount = mtexture.patchcount;

			texture.name = mtexture.name;
			let mpatch = mtexture.patches.as_ptr();
			let patch = texture.patches.as_mut_ptr();

			for j in 0..texture.patchcount as usize {
				let mpatch = &*mpatch.wrapping_add(j);
				let patch = &mut *patch.wrapping_add(j);
				patch.originx = mpatch.originx as i32;
				patch.originy = mpatch.originy as i32;
				patch.patch = patchlookup[mpatch.patch as usize];
				if patch.patch == -1 {
					I_Error(
						c"R_InitTextures: Missing patch in texture %s".as_ptr(),
						texture.name.as_ptr(),
					);
				}
			}

			*texturecolumnlump.wrapping_add(i) =
				Z_Malloc(texture.width as usize * 2, PU_STATIC, null_mut()).cast();
			*texturecolumnofs.wrapping_add(i) =
				Z_Malloc(texture.width as usize * 2, PU_STATIC, null_mut()).cast();

			let mut j = 1;
			while j * 2 <= texture.width as usize {
				j <<= 1;
			}

			*texturewidthmask.wrapping_add(i) = j - 1;
			*textureheight.wrapping_add(i) = (texture.height as fixed_t) << FRACBITS;

			directory = directory.wrapping_add(1);
		}

		Z_Free(maptex1.cast());
		if !maptex2.is_null() {
			Z_Free(maptex2.cast());
		}

		// Precalculate whatever possible.
		for i in 0..numtextures {
			R_GenerateLookup(i);
		}

		// Create translation table for global animation.
		texturetranslation = Z_Malloc((numtextures + 1) * 4, PU_STATIC, null_mut()).cast();

		for i in 0..numtextures {
			*texturetranslation.wrapping_add(i) = i;
		}
	}
}

// R_InitFlats
fn R_InitFlats() {
	unsafe {
		firstflat = W_GetNumForName(c"F_START".as_ptr()) as usize + 1;
		lastflat = W_GetNumForName(c"F_END".as_ptr()) as usize - 1;
		numflats = lastflat - firstflat + 1;

		// Create translation table for global animation.
		flattranslation = Z_Malloc((numflats + 1) * 4, PU_STATIC, null_mut()).cast();

		for i in 0..numflats {
			*flattranslation.wrapping_add(i) = i;
		}
	}
}

// R_InitSpriteLumps
// Finds the width and hoffset of all sprites in the wad,
//  so the sprite does not need to be cached completely
//  just for having the header info ready during rendering.
fn R_InitSpriteLumps() {
	unsafe {
		firstspritelump = W_GetNumForName(c"S_START".as_ptr()) as usize + 1;
		lastspritelump = W_GetNumForName(c"S_END".as_ptr()) as usize - 1;

		numspritelumps = lastspritelump - firstspritelump + 1;
		spritewidth = Z_Malloc(numspritelumps * 4, PU_STATIC, null_mut()).cast();
		spriteoffset = Z_Malloc(numspritelumps * 4, PU_STATIC, null_mut()).cast();
		spritetopoffset = Z_Malloc(numspritelumps * 4, PU_STATIC, null_mut()).cast();

		for i in 0..numspritelumps {
			if i & 63 == 0 {
				print!(".");
			}

			let patch = &*(W_CacheLumpNum(firstspritelump + i, PU_CACHE) as *mut patch_t);
			*spritewidth.wrapping_add(i) = (patch.width as fixed_t) << FRACBITS;
			*spriteoffset.wrapping_add(i) = (patch.leftoffset as fixed_t) << FRACBITS;
			*spritetopoffset.wrapping_add(i) = (patch.topoffset as fixed_t) << FRACBITS;
		}
	}
}

// R_InitColormaps
fn R_InitColormaps() {
	unsafe {
		// Load in the light tables,
		//  256 byte align tables.
		let lump = W_GetNumForName(c"COLORMAP".as_ptr()) as usize;
		let length = W_LumpLength(lump) + 255;
		let p = Z_Malloc(length, PU_STATIC, null_mut());
		colormaps = p.wrapping_byte_add(p.align_offset(0x100)).cast();
		W_ReadLump(lump, colormaps.cast());
	}
}

// R_InitData
// Locates all the lumps
//  that will be used by all views
// Must be called after W_Init.
#[unsafe(no_mangle)]
pub extern "C" fn R_InitData() {
	R_InitTextures();
	print!("\nInitTextures");
	R_InitFlats();
	print!("\nInitFlats");
	R_InitSpriteLumps();
	print!("\nInitSprites");
	R_InitColormaps();
	print!("\nInitColormaps");
}

// R_FlatNumForName
// Retrieval, get a flat number for a flat name.
pub(crate) fn R_FlatNumForName(name: *const c_char) -> usize {
	unsafe {
		let mut namet = [0i8; 9];
		let i = W_CheckNumForName(name);
		if i == -1 {
			namet[8] = 0;
			libc::memcpy(namet.as_mut_ptr().cast(), name.cast(), 8);
			I_Error(c"R_FlatNumForName: %s not found".as_ptr(), namet.as_ptr());
		}
		i as usize - firstflat
	}
}

// R_CheckTextureNumForName
// Check whether texture is available.
// Filter out NoTexture indicator.
pub(crate) fn R_CheckTextureNumForName(name: *const c_char) -> i32 {
	unsafe {
		// "NoTexture" marker.
		if *name == b'-' as c_char {
			return 0;
		}

		for i in 0..numtextures {
			if libc::strncasecmp((**textures.wrapping_add(i)).name.as_ptr().cast(), name, 8) == 0 {
				return i as i32;
			}
		}

		-1
	}
}

// R_TextureNumForName
// Calls R_CheckTextureNumForName,
//  aborts with error message.
pub(crate) fn R_TextureNumForName(name: *const c_char) -> usize {
	let i = R_CheckTextureNumForName(name);
	if i == -1 {
		unsafe { I_Error(c"R_TextureNumForName: %s not found".as_ptr(), name) };
	}
	i as usize
}

// R_PrecacheLevel
// Preloads all relevant graphics for the level.
static mut flatmemory: usize = 0;
static mut texturememory: usize = 0;
static mut spritememory: usize = 0;

unsafe extern "C" {
	static mut numsprites: usize;
	static mut sprites: *mut spritedef_t;
}

pub(crate) fn R_PrecacheLevel() {
	unsafe {
		if demoplayback != 0 {
			return;
		}

		// Precache flats.
		let mut flatpresent = vec![0u8; numflats];

		for i in 0..numsectors {
			flatpresent[(*sectors.wrapping_add(i)).floorpic as usize] = 1;
			flatpresent[(*sectors.wrapping_add(i)).ceilingpic as usize] = 1;
		}

		flatmemory = 0;

		#[allow(clippy::needless_range_loop)]
		for i in 0..numflats {
			if flatpresent[i] != 0 {
				let lump = firstflat + i;
				flatmemory += (*lumpinfo.wrapping_add(lump)).size;
				W_CacheLumpNum(lump, PU_CACHE);
			}
		}

		// Precache textures.
		let mut texturepresent = vec![0u8; numtextures];

		for i in 0..numsides {
			texturepresent[(*sides.wrapping_add(i)).toptexture as usize] = 1;
			texturepresent[(*sides.wrapping_add(i)).midtexture as usize] = 1;
			texturepresent[(*sides.wrapping_add(i)).bottomtexture as usize] = 1;
		}

		// Sky texture is always present.
		// Note that F_SKY1 is the name used to
		//  indicate a sky floor/ceiling as a flat,
		//  while the sky texture is stored like
		//  a wall texture, with an episode dependend
		//  name.
		texturepresent[skytexture] = 1;

		texturememory = 0;
		#[allow(clippy::needless_range_loop)]
		for i in 0..numtextures {
			if texturepresent[i] == 0 {
				continue;
			}

			let texture = &mut **textures.wrapping_add(i);

			for j in 0..texture.patchcount as usize {
				let lump = (*texture.patches.as_ptr().wrapping_add(j)).patch as usize;
				texturememory += (*lumpinfo.wrapping_add(lump)).size;
				W_CacheLumpNum(lump, PU_CACHE);
			}
		}

		// Precache sprites.
		let mut spritepresent = vec![0u8; numsprites];

		let mut th = thinkercap.next;
		while !std::ptr::eq(th, &raw const thinkercap) {
			if (*th).function.is_mobj() {
				spritepresent[(*(th as *const mobj_t)).sprite as usize] = 1;
			}
			th = (*th).next;
		}

		spritememory = 0;
		#[allow(clippy::needless_range_loop)]
		for i in 0..numsprites {
			if spritepresent[i] == 0 {
				continue;
			}

			let s = sprites.wrapping_add(i);
			for j in 0..(*s).numframes as usize {
				let sf = (*s).spriteframes.wrapping_add(j);
				for k in 0..8 {
					let lump = firstspritelump + (*sf).lump[k] as usize;
					spritememory += (*lumpinfo.wrapping_add(lump)).size;
					W_CacheLumpNum(lump, PU_CACHE);
				}
			}
		}
	}
}
