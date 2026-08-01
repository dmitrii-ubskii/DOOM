#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

// Graphics.
// DOOM graphics for walls and sprites
// is stored in vertical runs of opaque pixels (posts).
// A column is composed of zero or more posts,
// a patch or sprite is composed of zero or more columns.

use std::{
	ffi::{CStr, c_char},
	ptr::{self, null_mut, read_unaligned},
};

use crate::{
	g_game::demoplayback,
	i_system::I_Error,
	m_fixed::{FRACBITS, fixed_t},
	p_local::thinkercap,
	p_mobj::mobj_t,
	p_setup::{numsectors, numsides, sectors, sides},
	r_defs::{column_t, lighttable_t, patch_t},
	r_sky::skytexture,
	r_things::{numsprites, sprites},
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
	pub(crate) originx: short,
	pub(crate) originy: short,
	pub(crate) patch: short,
	pub(crate) stepdir: short,
	pub(crate) colormap: short,
}

// Texture definition.
// A DOOM wall texture is a list of patches
// which are to be combined in a predefined order.
#[repr(C)]
#[derive(Debug)]
struct maptexture_t {
	pub(crate) name: [u8; 8],
	pub(crate) masked: boolean,
	pub(crate) width: short,
	pub(crate) height: short,
	pub(crate) columndirectory: *mut *mut (), // OBSOLETE
	pub(crate) patchcount: short,
	pub(crate) patches: [mappatch_t; 1],
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
	pub(crate) originx: int,
	pub(crate) originy: int,
	pub(crate) patch: isize,
}

// A maptexturedef_t describes a rectangular texture,
//  which is composed of one or more mappatch_t structures
//  that arrange graphic patches.
#[repr(C)]
#[derive(Debug)]
struct texture_t {
	// Keep name for switch changing, etc.
	pub(crate) name: [u8; 8],
	pub(crate) width: short,
	pub(crate) height: short,

	// All the patches[patchcount]
	//  are drawn back to front into the cached texture.
	pub(crate) patchcount: short,
	pub(crate) patches: [texpatch_t; 1],
}

pub(crate) static mut firstflat: usize = 0;
pub(crate) static mut lastflat: usize = 0;
static mut numflats: usize = 0;

pub(crate) static mut firstspritelump: usize = 0;
pub(crate) static mut lastspritelump: usize = 0;
pub(crate) static mut numspritelumps: usize = 0;

static mut numtextures: usize = 0;
static mut textures: *mut *mut texture_t = null_mut();

static mut texturewidthmask: *mut usize = null_mut();
// needed for texture pegging
pub(crate) static mut textureheight: *mut fixed_t = null_mut();
static mut texturecompositesize: *mut usize = null_mut();
static mut texturecolumnlump: *mut *mut short = null_mut();
static mut texturecolumnofs: *mut *mut u16 = null_mut();
static mut texturecomposite: *mut *mut byte = null_mut();

// for global animation
pub(crate) static mut flattranslation: *mut usize = null_mut();
pub(crate) static mut texturetranslation: *mut usize = null_mut();

// needed for pre rendering
pub(crate) static mut spritewidth: *mut fixed_t = null_mut();
pub(crate) static mut spriteoffset: *mut fixed_t = null_mut();
pub(crate) static mut spritetopoffset: *mut fixed_t = null_mut();

pub(crate) static mut colormaps: *mut lighttable_t = null_mut();

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
			let source = ptr::from_mut(patch).wrapping_byte_add(3);
			let mut count = isize::from(patch.length);
			let mut position = isize::try_from(originy).unwrap() + isize::from(patch.topdelta);

			if position < 0 {
				count += position;
				position = 0;
			}

			if position + count > cacheheight {
				count = cacheheight - position;
			}

			if count > 0 {
				libc::memcpy(
					ptr::from_mut(cache)
						.wrapping_byte_add(usize::try_from(position).unwrap())
						.cast(),
					source.cast(),
					usize::try_from(count).unwrap(),
				);
			}

			patch = &mut *ptr::from_mut(patch).wrapping_byte_add(usize::from(patch.length) + 4);
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
		for i in 0..usize::try_from(texture.patchcount).unwrap() {
			let patch = &*patch.wrapping_add(i);
			let realpatch: &mut patch_t =
				&mut *W_CacheLumpNum(usize::try_from(patch.patch).unwrap(), PU_CACHE).cast();
			let x1 = patch.originx;
			let mut x2 = x1 + i32::from(realpatch.width);

			let mut x = if x1 < 0 { 0 } else { x1 };

			if x2 > i32::from(texture.width) {
				x2 = i32::from(texture.width);
			}

			while x < x2 {
				// Column does not have multiple patches?
				if *collump.wrapping_add(usize::try_from(x).unwrap()) >= 0 {
					x += 1;
					continue;
				}

				let patchcol = ptr::from_mut(realpatch)
					.wrapping_byte_add(
						*realpatch
							.columnofs
							.as_ptr()
							.wrapping_add(usize::try_from(x - x1).unwrap()),
					)
					.cast::<column_t>();
				R_DrawColumnInCache(
					&mut *patchcol,
					&mut *block
						.wrapping_add(usize::from(
							*colofs.wrapping_add(usize::try_from(x).unwrap()),
						))
						.cast(),
					patch.originy,
					isize::from(texture.height),
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
		let mut patchcount = vec![0; usize::try_from(texture.width).unwrap()];

		let patch = texture.patches.as_ptr();
		for i in 0..usize::try_from(texture.patchcount).unwrap() {
			let patch = &*patch.wrapping_add(i);
			let realpatch = &mut *(W_CacheLumpNum(usize::try_from(patch.patch).unwrap(), PU_CACHE)
				.cast::<patch_t>());
			let x1 = patch.originx;
			let mut x2 = x1 + i32::from(realpatch.width);

			let mut x = if x1 < 0 { 0 } else { x1 };

			if x2 > i32::from(texture.width) {
				x2 = i32::from(texture.width);
			}

			while x < x2 {
				patchcount[usize::try_from(x).unwrap()] += 1;
				*collump.wrapping_add(usize::try_from(x).unwrap()) =
					i16::try_from(patch.patch).unwrap();
				*colofs.wrapping_add(usize::try_from(x).unwrap()) = u16::try_from(
					*realpatch.columnofs.as_ptr().wrapping_add(usize::try_from(x - x1).unwrap()),
				)
				.unwrap() + 3;
				x += 1;
			}
		}

		#[allow(clippy::needless_range_loop)]
		for x in 0..usize::try_from(texture.width).unwrap() {
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
				*colofs.wrapping_add(x) =
					u16::try_from(*texturecompositesize.wrapping_add(texnum)).unwrap();

				if *texturecompositesize.wrapping_add(texnum)
					> 0x10000 - usize::try_from(texture.height).unwrap()
				{
					I_Error(format_args!("R_GenerateLookup: texture {} is >64k", texnum));
				}

				*texturecompositesize.wrapping_add(texnum) +=
					usize::try_from(texture.height).unwrap();
			}
		}
	}
}

// R_GetColumn
pub(crate) fn R_GetColumn(tex: usize, mut col: usize) -> *mut u8 {
	unsafe {
		col &= *texturewidthmask.wrapping_add(tex);
		let lump = *(*texturecolumnlump.wrapping_add(tex)).wrapping_add(col);
		let ofs = usize::from(*(*texturecolumnofs.wrapping_add(tex)).wrapping_add(col));

		if lump > 0 {
			return W_CacheLumpNum(usize::try_from(lump).unwrap(), PU_CACHE)
				.wrapping_byte_add(ofs)
				.cast();
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
		let names = W_CacheLumpName(c"PNAMES".as_ptr(), PU_STATIC).cast::<c_char>();
		let nummappatches = *names.cast::<usize>();
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
		let mut maptex = W_CacheLumpName(c"TEXTURE1".as_ptr(), PU_STATIC).cast::<usize>();
		let maptex1 = maptex;
		let numtextures1 = *maptex;
		let mut maxoff =
			W_LumpLength(usize::try_from(W_GetNumForName(c"TEXTURE1".as_ptr())).unwrap());
		let mut directory = maptex.wrapping_add(1);

		let maptex2;
		let numtextures2;
		let maxoff2;
		if W_CheckNumForName(c"TEXTURE2".as_ptr()) != -1 {
			maptex2 = W_CacheLumpName(c"TEXTURE2".as_ptr(), PU_STATIC).cast::<usize>();
			numtextures2 = *maptex2;
			maxoff2 = W_LumpLength(usize::try_from(W_GetNumForName(c"TEXTURE2".as_ptr())).unwrap());
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
		let temp3 = (usize::try_from(temp2 - temp1 + 63).unwrap() / 64) + numtextures.div_ceil(64);
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
				I_Error("R_InitTextures: bad texture directory");
			}

			let mtexture: *mut maptexture_t = maptex.wrapping_byte_add(offset).cast();

			let texture = Z_Malloc(
				size_of::<texture_t>()
					+ size_of::<texpatch_t>()
						* usize::try_from(read_unaligned(&raw const (*mtexture).patchcount) - 1)
							.unwrap(),
				PU_STATIC,
				null_mut(),
			)
			.cast();
			*textures.wrapping_add(i) = texture;
			let texture = &mut *texture;

			texture.width = read_unaligned(&raw const (*mtexture).width);
			texture.height = read_unaligned(&raw const (*mtexture).height);
			texture.patchcount = read_unaligned(&raw const (*mtexture).patchcount);

			texture.name = read_unaligned(&raw const (*mtexture).name);
			let mpatch: *const mappatch_t = (&raw const (*mtexture).patches).cast();
			let patch = texture.patches.as_mut_ptr();

			for j in 0..usize::try_from(texture.patchcount).unwrap() {
				let mpatch = &*mpatch.wrapping_add(j);
				let patch = &mut *patch.wrapping_add(j);
				patch.originx = i32::from(mpatch.originx);
				patch.originy = i32::from(mpatch.originy);
				patch.patch = patchlookup[usize::try_from(mpatch.patch).unwrap()];
				if patch.patch == -1 {
					I_Error(format_args!(
						"R_InitTextures: Missing patch in texture {}",
						CStr::from_ptr(texture.name.as_ptr().cast()).to_str().unwrap(),
					));
				}
			}

			*texturecolumnlump.wrapping_add(i) =
				Z_Malloc(usize::try_from(texture.width).unwrap() * 2, PU_STATIC, null_mut()).cast();
			*texturecolumnofs.wrapping_add(i) =
				Z_Malloc(usize::try_from(texture.width).unwrap() * 2, PU_STATIC, null_mut()).cast();

			let mut j = 1;
			while j * 2 <= usize::try_from(texture.width).unwrap() {
				j <<= 1;
			}

			*texturewidthmask.wrapping_add(i) = j - 1;
			*textureheight.wrapping_add(i) = fixed_t::from(texture.height) << FRACBITS;

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
		firstflat = usize::try_from(W_GetNumForName(c"F_START".as_ptr())).unwrap() + 1;
		lastflat = usize::try_from(W_GetNumForName(c"F_END".as_ptr())).unwrap() - 1;
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
		firstspritelump = usize::try_from(W_GetNumForName(c"S_START".as_ptr())).unwrap() + 1;
		lastspritelump = usize::try_from(W_GetNumForName(c"S_END".as_ptr())).unwrap() - 1;

		numspritelumps = lastspritelump - firstspritelump + 1;
		spritewidth = Z_Malloc(numspritelumps * 4, PU_STATIC, null_mut()).cast();
		spriteoffset = Z_Malloc(numspritelumps * 4, PU_STATIC, null_mut()).cast();
		spritetopoffset = Z_Malloc(numspritelumps * 4, PU_STATIC, null_mut()).cast();

		for i in 0..numspritelumps {
			if i & 63 == 0 {
				print!(".");
			}

			let patch = &*(W_CacheLumpNum(firstspritelump + i, PU_CACHE).cast::<patch_t>());
			*spritewidth.wrapping_add(i) = fixed_t::from(patch.width) << FRACBITS;
			*spriteoffset.wrapping_add(i) = fixed_t::from(patch.leftoffset) << FRACBITS;
			*spritetopoffset.wrapping_add(i) = fixed_t::from(patch.topoffset) << FRACBITS;
		}
	}
}

// R_InitColormaps
fn R_InitColormaps() {
	unsafe {
		// Load in the light tables,
		//  256 byte align tables.
		let lump = usize::try_from(W_GetNumForName(c"COLORMAP".as_ptr())).unwrap();
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
pub(crate) fn R_InitData() {
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
			I_Error(format_args!(
				"R_FlatNumForName: {} not found",
				CStr::from_ptr(namet.as_ptr()).to_str().unwrap()
			));
		}
		usize::try_from(i).unwrap() - firstflat
	}
}

// R_CheckTextureNumForName
// Check whether texture is available.
// Filter out NoTexture indicator.
pub(crate) fn R_CheckTextureNumForName(name: *const c_char) -> i32 {
	unsafe {
		// "NoTexture" marker.
		if *name == c_char::try_from(b'-').unwrap() {
			return 0;
		}

		for i in 0..numtextures {
			if libc::strncasecmp((**textures.wrapping_add(i)).name.as_ptr().cast(), name, 8) == 0 {
				return i32::try_from(i).unwrap();
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
		unsafe {
			I_Error(format_args!(
				"R_TextureNumForName: {} not found",
				CStr::from_ptr(name).to_str().unwrap()
			))
		};
	}
	usize::try_from(i).unwrap()
}

// R_PrecacheLevel
// Preloads all relevant graphics for the level.
static mut flatmemory: usize = 0;
static mut texturememory: usize = 0;
static mut spritememory: usize = 0;

pub(crate) fn R_PrecacheLevel() {
	unsafe {
		if demoplayback {
			return;
		}

		// Precache flats.
		let mut flatpresent = vec![0u8; numflats];

		for i in 0..numsectors {
			flatpresent[usize::try_from((*sectors.wrapping_add(i)).floorpic).unwrap()] = 1;
			flatpresent[usize::try_from((*sectors.wrapping_add(i)).ceilingpic).unwrap()] = 1;
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
			texturepresent[usize::try_from((*sides.wrapping_add(i)).toptexture).unwrap()] = 1;
			texturepresent[usize::try_from((*sides.wrapping_add(i)).midtexture).unwrap()] = 1;
			texturepresent[usize::try_from((*sides.wrapping_add(i)).bottomtexture).unwrap()] = 1;
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

			for j in 0..usize::try_from(texture.patchcount).unwrap() {
				let lump =
					usize::try_from((*texture.patches.as_ptr().wrapping_add(j)).patch).unwrap();
				texturememory += (*lumpinfo.wrapping_add(lump)).size;
				W_CacheLumpNum(lump, PU_CACHE);
			}
		}

		// Precache sprites.
		let mut spritepresent = vec![0u8; numsprites];

		let mut th = thinkercap.next;
		while !std::ptr::eq(th, &raw const thinkercap) {
			if (*th).function.is_mobj() {
				spritepresent[usize::from((*(th.cast::<mobj_t>())).sprite)] = 1;
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
			for j in 0..usize::try_from((*s).numframes).unwrap() {
				let sf = (*s).spriteframes.wrapping_add(j);
				for k in 0..8 {
					let lump = firstspritelump + usize::try_from((*sf).lump[k]).unwrap();
					spritememory += (*lumpinfo.wrapping_add(lump)).size;
					W_CacheLumpNum(lump, PU_CACHE);
				}
			}
		}
	}
}
