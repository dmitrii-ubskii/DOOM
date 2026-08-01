#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]
//	Refresh of things, i.e. objects represented by sprites.

use std::{
	ffi::CStr,
	mem,
	num::Wrapping,
	ptr::{self, null, null_mut},
};

use crate::{
	doomdef::{SCREENWIDTH, powertype_t},
	doomstat::modifiedgame,
	i_system::I_Error,
	info::spritenum_t,
	m_fixed::{FRACBITS, FRACUNIT, FixedDiv, FixedMul, fixed_t},
	p_mobj::{MF_SHADOW, MF_TRANSLATION, MF_TRANSSHIFT, mobj_t},
	p_pspr::{FF_FRAMEMASK, FF_FULLBRIGHT, pspdef_t, psprnum_t},
	r_bsp::{drawsegs, ds_p},
	r_data::{
		colormaps, firstspritelump, lastspritelump, spriteoffset, spritetopoffset, spritewidth,
	},
	r_defs::{
		SIL_BOTH, SIL_BOTTOM, SIL_TOP, column_t, lighttable_t, patch_t, sector_t, spritedef_t,
		spriteframe_t, vissprite_t,
	},
	r_draw::{
		R_DrawTranslatedColumn, dc_colormap, dc_iscale, dc_source, dc_texturemid, dc_translation,
		dc_x, dc_yh, dc_yl, translationtables, viewheight, viewwidth,
	},
	r_main::{
		LIGHTLEVELS, LIGHTSCALESHIFT, LIGHTSEGSHIFT, MAXLIGHTSCALE, R_PointOnSegSide,
		R_PointToAngle, basecolfunc, centerxfrac, centeryfrac, colfunc, detailshift, extralight,
		fixedcolormap, fuzzcolfunc, projection, scalelight, validcount, viewangleoffset, viewcos,
		viewplayer, viewsin, viewx, viewy, viewz,
	},
	r_segs::R_RenderMaskedSegRange,
	tables::ANG45,
	w_wad::{W_CacheLumpNum, W_GetNumForName, lumpinfo},
	z_zone::{PU_CACHE, PU_STATIC, Z_Malloc},
};

const MAXVISSPRITES: usize = 128;

const MINZ: i32 = FRACUNIT * 4;
const BASEYCENTER: i32 = 100;

// Sprite rotation 0 is facing the viewer,
//  rotation 1 is one angle turn CLOCKWISE around the axis.
// This is not the same as the angle,
//  which increases counter clockwise (protractor).
// There was a lot of stuff grabbed wrong, so I changed it...
pub(crate) static mut pspritescale: fixed_t = 0;
pub(crate) static mut pspriteiscale: fixed_t = 0;

static mut spritelights: *mut *mut lighttable_t = null_mut();

// constant arrays
//  used for psprite clipping and initializing clipping
pub(crate) static mut negonearray: [i16; SCREENWIDTH] = [0; SCREENWIDTH];
pub(crate) static mut screenheightarray: [i16; SCREENWIDTH] = [0; SCREENWIDTH];

// INITIALIZATION FUNCTIONS

// variables used to look up
//  and range check thing_t sprites patches
pub(crate) static mut sprites: *mut spritedef_t = null_mut();
pub(crate) static mut numsprites: usize = 0;

static mut sprtemp: [spriteframe_t; 29] = unsafe { mem::zeroed() };
static mut maxframe: i32 = 0;
static mut spritename: *const i8 = null();

// R_InstallSpriteLump
// Local function for R_InitSprites.
fn R_InstallSpriteLump(lump: isize, frame: u8, rotation: u8, flipped: bool) {
	unsafe {
		if frame >= 29 || rotation > 8 {
			I_Error(format_args!("R_InstallSpriteLump: Bad frame characters in lump {}", lump));
		}

		if i32::from(frame) > maxframe {
			maxframe = i32::from(frame);
		}

		let frame_index = usize::from(frame);

		if rotation == 0 {
			// the lump should be used for all rotations
			if sprtemp[frame_index].rotate == 0 {
				I_Error(format_args!(
					"R_InitSprites: Sprite {} frame {} has multip rot=0 lump",
					CStr::from_ptr(spritename).to_str().unwrap(),
					char::from(b'A' + frame),
				));
			}

			if sprtemp[frame_index].rotate == 1 {
				I_Error(format_args!(
					"R_InitSprites: Sprite {} frame {} has rotations and a rot=0 lump",
					CStr::from_ptr(spritename).to_str().unwrap(),
					char::from(b'A' + frame),
				));
			}

			sprtemp[frame_index].rotate = 0;

			for r in 0..8 {
				sprtemp[frame_index].lump[r] =
					i16::try_from(lump - isize::try_from(firstspritelump).unwrap()).unwrap();
				sprtemp[frame_index].flip[r] = i8::from(flipped);
			}
			return;
		}

		// the lump is only used for one rotation
		if sprtemp[frame_index].rotate == 0 {
			I_Error(format_args!(
				"R_InitSprites: Sprite {} frame {} has rotations and a rot=0 lump",
				CStr::from_ptr(spritename).to_str().unwrap(),
				char::from(b'A' + frame),
			));
		}

		sprtemp[frame_index].rotate = 1;

		// make 0 based
		let rotation = usize::from(rotation) - 1;
		if sprtemp[frame_index].lump[rotation] != -1 {
			I_Error(format_args!(
				"R_InitSprites: Sprite {} : {} : {} has two lumps mapped to it",
				CStr::from_ptr(spritename).to_str().unwrap(),
				char::from(b'A' + frame),
				usize::from(b'1') + rotation,
			));
		}

		sprtemp[frame_index].lump[rotation] =
			i16::try_from(lump - isize::try_from(firstspritelump).unwrap()).unwrap();
		sprtemp[frame_index].flip[rotation] = i8::from(flipped);
	}
}

// R_InitSpriteDefs
// Pass a null terminated list of sprite names
//  (4 chars exactly) to be used.
// Builds the sprite rotation matrixes to account
//  for horizontally flipped sprites.
// Will report an error if the lumps are inconsistant.
// Only called at startup.
//
// Sprite lump names are 4 characters for the actor,
//  a letter for the frame, and a number for the rotation.
// A sprite that is flippable will have an additional
//  letter/number appended.
// The rotation character can be 0 to signify no rotations.
#[allow(static_mut_refs)]
fn R_InitSpriteDefs(namelist: *const *const i8) {
	unsafe {
		// count the number of sprite names
		numsprites = spritenum_t::NUMSPRITES.into();

		if numsprites == 0 {
			return;
		}

		sprites = Z_Malloc(numsprites * size_of::<spritedef_t>(), PU_STATIC, null_mut()).cast();

		let start = firstspritelump - 1;
		let end = lastspritelump + 1;

		// scan all the lump names for each of the names,
		//  noting the highest frame letter.
		// Just compare 4 characters as ints
		for i in 0..numsprites {
			spritename = *namelist.wrapping_add(i);
			ptr::write_bytes(&raw mut sprtemp, 0xff, 1);

			maxframe = -1;
			let intname = spritename.cast::<i32>().read_unaligned();

			// scan the lumps,
			//  filling in the frames for whatever is found
			for l in start + 1..end {
				if (*lumpinfo.wrapping_add(l)).name.as_ptr().cast::<i32>().read_unaligned()
					== intname
				{
					let frame = u8::try_from((*lumpinfo.wrapping_add(l)).name[4]).unwrap() - b'A';
					let rotation =
						u8::try_from((*lumpinfo.wrapping_add(l)).name[5]).unwrap() - b'0';

					let patched = if modifiedgame != 0 {
						W_GetNumForName((*lumpinfo.wrapping_add(l)).name.as_ptr())
					} else {
						isize::try_from(l).unwrap()
					};

					R_InstallSpriteLump(patched, frame, rotation, false);

					if (*lumpinfo.wrapping_add(l)).name[6] != 0 {
						let frame =
							u8::try_from((*lumpinfo.wrapping_add(l)).name[6]).unwrap() - b'A';
						let rotation =
							u8::try_from((*lumpinfo.wrapping_add(l)).name[7]).unwrap() - b'0';
						R_InstallSpriteLump(isize::try_from(l).unwrap(), frame, rotation, true);
					}
				}
			}

			// check the frames that were found for completeness
			if maxframe == -1 {
				(*sprites.wrapping_add(i)).numframes = 0;
				continue;
			}

			maxframe += 1;

			#[allow(clippy::needless_range_loop)]
			for frame in 0..usize::try_from(maxframe).unwrap() {
				match sprtemp[frame].rotate {
					-1 => {
						// no rotations were found for that frame at all
						I_Error(format_args!(
							"R_InitSprites: No patches found for {} frame {}",
							CStr::from_ptr(*namelist.wrapping_add(i)).to_str().unwrap(),
							frame + usize::from(b'A'),
						))
					}
					0 => (), // only the first rotation is needed
					1 => {
						// must have all 8 frames
						for rotation in 0..8 {
							if sprtemp[frame].lump[rotation] == -1 {
								I_Error(format_args!(
									"R_InitSprites: Sprite {} frame {} is missing rotations",
									CStr::from_ptr(*namelist.wrapping_add(i)).to_str().unwrap(),
									frame + usize::from(b'A'),
								));
							}
						}
					}
					_ => (),
				}
			}

			// allocate space for the frames present and copy sprtemp to it
			(*sprites.wrapping_add(i)).numframes = maxframe;
			(*sprites.wrapping_add(i)).spriteframes = Z_Malloc(
				usize::try_from(maxframe).unwrap() * size_of::<spriteframe_t>(),
				PU_STATIC,
				null_mut(),
			)
			.cast();
			libc::memcpy(
				(*sprites.wrapping_add(i)).spriteframes.cast(),
				sprtemp.as_mut_ptr().cast(),
				usize::try_from(maxframe).unwrap() * size_of::<spriteframe_t>(),
			);
		}
	}
}

// GAME FUNCTIONS
static mut vissprites: [vissprite_t; MAXVISSPRITES] = unsafe { mem::zeroed() };
static mut vissprite_p: *mut vissprite_t = null_mut();

// R_InitSprites
// Called at program start.
pub(crate) fn R_InitSprites(namelist: *const *const i8) {
	unsafe {
		negonearray = [-1; SCREENWIDTH];
		R_InitSpriteDefs(namelist);
	}
}

// R_ClearSprites
// Called at frame start.
#[allow(static_mut_refs)]
pub(crate) fn R_ClearSprites() {
	unsafe { vissprite_p = vissprites.as_mut_ptr() }
}

// R_NewVisSprite
static mut overflowsprite: vissprite_t = unsafe { mem::zeroed() };

#[allow(static_mut_refs)]
fn R_NewVisSprite() -> *mut vissprite_t {
	unsafe {
		if ptr::eq(vissprite_p, vissprites.as_ptr().wrapping_add(MAXVISSPRITES)) {
			return &raw mut overflowsprite;
		}

		let p = vissprite_p;
		vissprite_p = vissprite_p.wrapping_add(1);
		p
	}
}

// R_DrawMaskedColumn
// Used for sprites and masked mid textures.
// Masked means: partly transparent, i.e. stored
//  in posts/runs of opaque pixels.
pub(crate) static mut mfloorclip: *mut i16 = null_mut();
pub(crate) static mut mceilingclip: *mut i16 = null_mut();

pub(crate) static mut spryscale: fixed_t = 0;
pub(crate) static mut sprtopscreen: fixed_t = 0;

pub(crate) unsafe fn R_DrawMaskedColumn(mut column: *mut column_t) {
	unsafe {
		let basetexturemid = dc_texturemid;

		while (*column).topdelta != 0xff {
			// calculate unclipped screen coordinates
			//  for post
			let topscreen = sprtopscreen + spryscale * i32::from((*column).topdelta);
			let bottomscreen = topscreen + spryscale * i32::from((*column).length);

			dc_yl = (topscreen + FRACUNIT - 1) >> FRACBITS;
			dc_yh = (bottomscreen - 1) >> FRACBITS;

			if dc_yh >= i32::from(*mfloorclip.wrapping_add(usize::try_from(dc_x).unwrap())) {
				dc_yh = i32::from(*mfloorclip.wrapping_add(usize::try_from(dc_x).unwrap())) - 1;
			}
			if dc_yl <= i32::from(*mceilingclip.wrapping_add(usize::try_from(dc_x).unwrap())) {
				dc_yl = i32::from(*mceilingclip.wrapping_add(usize::try_from(dc_x).unwrap())) + 1;
			}

			if dc_yl <= dc_yh {
				dc_source = column.cast::<u8>().wrapping_add(3);
				dc_texturemid = basetexturemid - (i32::from((*column).topdelta) << FRACBITS);

				// Drawn by either R_DrawColumn
				//  or (SHADOW) R_DrawFuzzColumn.
				colfunc();
			}
			column = (column.wrapping_byte_add(usize::from((*column).length) + 4)).cast();
		}

		dc_texturemid = basetexturemid;
	}
}

// R_DrawVisSprite
//  mfloorclip and mceilingclip should also be set.
fn R_DrawVisSprite(vis: &mut vissprite_t, _x1: usize, _x2: usize) {
	unsafe {
		let patch = W_CacheLumpNum(vis.patch + firstspritelump, PU_CACHE).cast::<patch_t>();

		dc_colormap = vis.colormap;

		if dc_colormap.is_null() {
			// NULL colormap = shadow draw
			colfunc = fuzzcolfunc;
		} else if vis.mobjflags & MF_TRANSLATION != 0 {
			colfunc = R_DrawTranslatedColumn;
			dc_translation = translationtables.wrapping_offset(
				isize::try_from((vis.mobjflags & MF_TRANSLATION) >> (MF_TRANSSHIFT - 8)).unwrap()
					- 256,
			);
		}

		dc_iscale = fixed_t::abs(vis.xiscale) >> detailshift;
		dc_texturemid = vis.texturemid;
		let mut frac = vis.startfrac;
		spryscale = vis.scale;
		sprtopscreen = centeryfrac - FixedMul(dc_texturemid, spryscale);

		for x in vis.x1..=vis.x2 {
			dc_x = i32::try_from(x).unwrap();

			let texturecolumn = usize::try_from(frac >> FRACBITS).unwrap();
			let column = patch
				.wrapping_byte_add(*(*patch).columnofs.as_ptr().wrapping_add(texturecolumn))
				.cast();

			R_DrawMaskedColumn(column);

			frac += vis.xiscale;
		}

		colfunc = basecolfunc;
	}
}

// R_ProjectSprite
// Generates a vissprite for a thing
//  if it might be visible.
fn R_ProjectSprite(thing: &mut mobj_t) {
	unsafe {
		// transform the origin point
		let tr_x = thing.x - viewx;
		let tr_y = thing.y - viewy;

		let gxt = FixedMul(tr_x, viewcos);
		let gyt = -FixedMul(tr_y, viewsin);

		let tz = gxt - gyt;

		// thing is behind view plane?
		if tz < MINZ {
			return;
		}

		let xscale = FixedDiv(projection, tz);

		let gxt = -FixedMul(tr_x, viewsin);
		let gyt = FixedMul(tr_y, viewcos);
		let mut tx = -(gyt + gxt);

		// too far off the side?
		if fixed_t::abs(tx) > (tz << 2) {
			return;
		}

		// decide which patch to use for sprite relative to player
		let sprdef = sprites.wrapping_add(usize::from(thing.sprite));
		let sprframe = (*sprdef).spriteframes.wrapping_add(thing.frame & FF_FRAMEMASK);

		let (lump, flip);
		if (*sprframe).rotate != 0 {
			// choose a different rotation based on player view
			let ang = R_PointToAngle(thing.x, thing.y);
			let rot = (ang - thing.angle + (ANG45 / Wrapping(2)) * Wrapping(9)).0 >> 29;
			lump = usize::try_from((*sprframe).lump[rot]).unwrap();
			flip = (*sprframe).flip[rot] != 0;
		} else {
			// use single rotation for all views
			lump = usize::try_from((*sprframe).lump[0]).unwrap();
			flip = (*sprframe).flip[0] != 0;
		}

		// calculate edges of the shape
		tx -= *spriteoffset.wrapping_add(lump);
		let x1 = (centerxfrac + FixedMul(tx, xscale)) >> FRACBITS;

		// off the right side?
		if x1 > i32::try_from(viewwidth).unwrap() {
			return;
		}

		tx += *spritewidth.wrapping_add(lump);
		let x2 = ((centerxfrac + FixedMul(tx, xscale)) >> FRACBITS) - 1;

		// off the left side
		if x2 < 0 {
			return;
		}

		// store information in a vissprite
		let vis = R_NewVisSprite();
		(*vis).mobjflags = thing.flags;
		(*vis).scale = xscale << detailshift;
		(*vis).gx = thing.x;
		(*vis).gy = thing.y;
		(*vis).gz = thing.z;
		(*vis).gzt = thing.z + *spritetopoffset.wrapping_add(lump);
		(*vis).texturemid = (*vis).gzt - viewz;
		(*vis).x1 = usize::try_from(i32::max(x1, 0)).unwrap();
		(*vis).x2 = if usize::try_from(x2).unwrap() >= viewwidth {
			viewwidth - 1
		} else {
			usize::try_from(x2).unwrap()
		};
		let iscale = FixedDiv(FRACUNIT, xscale);

		if flip {
			(*vis).startfrac = *spritewidth.wrapping_add(lump) - 1;
			(*vis).xiscale = -iscale;
		} else {
			(*vis).startfrac = 0;
			(*vis).xiscale = iscale;
		}

		if i32::try_from((*vis).x1).unwrap() > x1 {
			(*vis).startfrac += (*vis).xiscale * (i32::try_from((*vis).x1).unwrap() - x1);
		}
		(*vis).patch = lump;

		// get light level
		if thing.flags & MF_SHADOW != 0 {
			// shadow draw
			(*vis).colormap = null_mut();
		} else if !fixedcolormap.is_null() {
			// fixed map
			(*vis).colormap = fixedcolormap;
		} else if thing.frame & FF_FULLBRIGHT != 0 {
			// full bright
			(*vis).colormap = colormaps;
		} else {
			// diminished light
			let mut index = usize::try_from(xscale).unwrap() >> (LIGHTSCALESHIFT - detailshift);

			if index >= MAXLIGHTSCALE {
				index = MAXLIGHTSCALE - 1;
			}

			(*vis).colormap = *spritelights.wrapping_add(index);
		}
	}
}

// R_AddSprites
// During BSP traversal, this adds sprites by sector.
pub(crate) fn R_AddSprites(sec: &mut sector_t) {
	unsafe {
		// BSP is traversed by subsector.
		// A sector might have been split into several
		//  subsectors during BSP building.
		// Thus we check whether its already added.
		if sec.validcount == validcount {
			return;
		}

		// Well, now it will be done.
		sec.validcount = validcount;

		let lightnum = (sec.lightlevel >> LIGHTSEGSHIFT) + i16::try_from(extralight).unwrap();

		spritelights = if lightnum < 0 {
			scalelight[0].as_mut_ptr()
		} else if usize::try_from(lightnum).unwrap() >= LIGHTLEVELS {
			scalelight[LIGHTLEVELS - 1].as_mut_ptr()
		} else {
			scalelight[usize::try_from(lightnum).unwrap()].as_mut_ptr()
		};

		// Handle all things in sector.
		let mut thing = sec.thinglist;
		while !thing.is_null() {
			R_ProjectSprite(&mut *thing);
			thing = (*thing).snext;
		}
	}
}

// R_DrawPSprite
fn R_DrawPSprite(psp: &pspdef_t) {
	unsafe {
		// decide which patch to use
		let sprdef = sprites.wrapping_add(usize::from((*psp.state).sprite));
		let sprframe = (*sprdef).spriteframes.wrapping_add((*psp.state).frame & FF_FRAMEMASK);

		let lump = usize::try_from((*sprframe).lump[0]).unwrap();
		let flip = (*sprframe).flip[0] != 0;

		// calculate edges of the shape
		let mut tx = psp.sx - 160 * FRACUNIT;

		tx -= *spriteoffset.wrapping_add(lump);
		let x1 = (centerxfrac + FixedMul(tx, pspritescale)) >> FRACBITS;

		// off the right side
		if x1 > i32::try_from(viewwidth).unwrap() {
			return;
		}

		tx += *spritewidth.wrapping_add(lump);
		let x2 = ((centerxfrac + FixedMul(tx, pspritescale)) >> FRACBITS) - 1;

		// off the left side
		if x2 < 0 {
			return;
		}

		// store information in a vissprite
		let mut avis = vissprite_t {
			prev: null_mut(),
			next: null_mut(),
			x1: usize::try_from(i32::max(x1, 0)).unwrap(),
			x2: if x2 >= i32::try_from(viewwidth).unwrap() {
				viewwidth - 1
			} else {
				usize::try_from(x2).unwrap()
			},
			gx: 0,
			gy: 0,
			gz: 0,
			gzt: 0,
			startfrac: 0,
			scale: pspritescale << detailshift,
			xiscale: 0,
			texturemid: (BASEYCENTER << FRACBITS) + FRACUNIT / 2
				- (psp.sy - *spritetopoffset.wrapping_add(lump)),
			patch: 0,
			colormap: null_mut(),
			mobjflags: 0,
		};

		if flip {
			avis.xiscale = -pspriteiscale;
			avis.startfrac = *spritewidth.wrapping_add(lump) - 1;
		} else {
			avis.xiscale = pspriteiscale;
			avis.startfrac = 0;
		}

		if i32::try_from(avis.x1).unwrap() > x1 {
			avis.startfrac += avis.xiscale * (i32::try_from(avis.x1).unwrap() - x1);
		}

		avis.patch = lump;

		if (&(*viewplayer).powers)[powertype_t::pw_invisibility] > 4 * 32
			|| (&(*viewplayer).powers)[powertype_t::pw_invisibility] & 8 != 0
		{
			// shadow draw
			avis.colormap = null_mut();
		} else if !fixedcolormap.is_null() {
			// fixed color
			avis.colormap = fixedcolormap;
		} else if (*psp.state).frame & FF_FULLBRIGHT != 0 {
			// full bright
			avis.colormap = colormaps;
		} else {
			// local light
			avis.colormap = *spritelights.wrapping_add(MAXLIGHTSCALE - 1);
		}

		let x1 = avis.x1;
		let x2 = avis.x2;
		R_DrawVisSprite(&mut avis, x1, x2);
	}
}

// R_DrawPlayerSprites
#[allow(static_mut_refs)]
fn R_DrawPlayerSprites() {
	unsafe {
		// get light level
		let lightnum = ((*(*(*viewplayer).mo().subsector).sector).lightlevel >> LIGHTSEGSHIFT)
			+ i16::try_from(extralight).unwrap();

		spritelights = if lightnum < 0 {
			scalelight[0].as_mut_ptr()
		} else if usize::try_from(lightnum).unwrap() >= LIGHTLEVELS {
			scalelight[LIGHTLEVELS - 1].as_mut_ptr()
		} else {
			scalelight[usize::try_from(lightnum).unwrap()].as_mut_ptr()
		};

		// clip to screen bounds
		mfloorclip = screenheightarray.as_mut_ptr();
		mceilingclip = negonearray.as_mut_ptr();

		for i in 0..usize::from(psprnum_t::NUMPSPRITES) {
			let psp = &(*viewplayer).psprites[i];
			if !psp.state.is_null() {
				R_DrawPSprite(psp);
			}
		}
	}
}

// R_SortVisSprites
static mut vsprsortedhead: vissprite_t = unsafe { mem::zeroed() };

#[allow(static_mut_refs)]
fn R_SortVisSprites() {
	unsafe {
		let count = vissprite_p.offset_from(vissprites.as_ptr());

		if count == 0 {
			return;
		}

		let mut unsorted = mem::zeroed::<vissprite_t>();
		unsorted.next = &raw mut unsorted;
		unsorted.prev = &raw mut unsorted;

		#[allow(clippy::needless_range_loop)]
		for ds in 0..usize::try_from(vissprite_p.offset_from(vissprites.as_ptr())).unwrap() {
			let ds = &raw mut vissprites[ds];
			(*ds).next = ds.offset(1);
			(*ds).prev = ds.offset(-1);
		}

		vissprites[0].prev = &raw mut unsorted;
		unsorted.next = vissprites.as_mut_ptr();
		(*vissprite_p.offset(-1)).next = &raw mut unsorted;
		unsorted.prev = vissprite_p.offset(-1);

		// pull the vissprites out by scale
		vsprsortedhead.next = &raw mut vsprsortedhead;
		vsprsortedhead.prev = &raw mut vsprsortedhead;

		for _ in 0..count {
			let mut bestscale = i32::MAX;
			let mut ds = unsorted.next;
			let mut best = ds;
			while !ptr::eq(ds, &raw const unsorted) {
				if (*ds).scale < bestscale {
					bestscale = (*ds).scale;
					best = ds;
				}
				ds = (*ds).next;
			}
			(*(*best).next).prev = (*best).prev;
			(*(*best).prev).next = (*best).next;
			(*best).next = &raw mut vsprsortedhead;
			(*best).prev = vsprsortedhead.prev;
			(*vsprsortedhead.prev).next = best;
			vsprsortedhead.prev = best;
		}
	}
}

// R_DrawSprite
#[allow(static_mut_refs)]
fn R_DrawSprite(spr: &mut vissprite_t) {
	unsafe {
		let mut clipbot = [0; SCREENWIDTH];
		let mut cliptop = [0; SCREENWIDTH];

		for x in spr.x1..=spr.x2 {
			clipbot[x] = -2;
			cliptop[x] = -2;
		}

		// Scan drawsegs from end to start for obscuring segs.
		// The first drawseg that has a greater scale
		//  is the clip seg.
		for ds in (0..usize::try_from(ds_p.offset_from(drawsegs.as_ptr())).unwrap()).rev() {
			let ds = &mut drawsegs[ds];

			// determine if the drawseg obscures the sprite
			if ds.x1 > spr.x2
				|| ds.x2 < spr.x1
				|| ds.silhouette == 0 && ds.maskedtexturecol.is_null()
			{
				// does not cover sprite
				continue;
			}

			let r1 = usize::max(ds.x1, spr.x1);
			let r2 = usize::min(ds.x2, spr.x2);

			let lowscale = i32::min(ds.scale1, ds.scale2);
			let scale = i32::max(ds.scale1, ds.scale2);

			if scale < spr.scale
				|| lowscale < spr.scale && R_PointOnSegSide(spr.gx, spr.gy, &mut *ds.curline) == 0
			{
				// masked mid texture?
				if !ds.maskedtexturecol.is_null() {
					R_RenderMaskedSegRange(
						ds,
						i32::try_from(r1).unwrap(),
						i32::try_from(r2).unwrap(),
					);
				}
				// seg is behind sprite
				continue;
			}

			// clip this piece of the sprite
			let mut silhouette = ds.silhouette;

			if spr.gz >= ds.bsilheight {
				silhouette &= !SIL_BOTTOM;
			}

			if spr.gzt <= ds.tsilheight {
				silhouette &= !SIL_TOP;
			}

			if silhouette == SIL_BOTTOM {
				#[allow(clippy::needless_range_loop)]
				for x in r1..=r2 {
					if clipbot[x] == -2 {
						clipbot[x] = *ds.sprbottomclip.wrapping_add(x);
					}
				}
			} else if silhouette == SIL_TOP {
				#[allow(clippy::needless_range_loop)]
				for x in r1..=r2 {
					if cliptop[x] == -2 {
						cliptop[x] = *ds.sprtopclip.wrapping_add(x);
					}
				}
			} else if silhouette == SIL_BOTH {
				for x in r1..=r2 {
					if clipbot[x] == -2 {
						clipbot[x] = *ds.sprbottomclip.wrapping_add(x);
					}
					if cliptop[x] == -2 {
						cliptop[x] = *ds.sprtopclip.wrapping_add(x);
					}
				}
			}
		}

		// all clipping has been performed, so draw the sprite

		// check for unclipped columns
		for x in spr.x1..=spr.x2 {
			if clipbot[x] == -2 {
				clipbot[x] = i16::try_from(viewheight).unwrap();
			}

			if cliptop[x] == -2 {
				cliptop[x] = -1;
			}
		}

		mfloorclip = clipbot.as_mut_ptr();
		mceilingclip = cliptop.as_mut_ptr();
		R_DrawVisSprite(spr, spr.x1, spr.x2);
	}
}

// R_DrawMasked
#[allow(static_mut_refs)]
pub(crate) fn R_DrawMasked() {
	unsafe {
		R_SortVisSprites();

		if vissprite_p.addr() > vissprites.as_ptr().addr() {
			// draw all vissprites back to front
			let mut spr = vsprsortedhead.next;
			while !ptr::eq(spr, &raw const vsprsortedhead) {
				R_DrawSprite(&mut *spr);
				spr = (*spr).next;
			}
		}

		// render any remaining masked mid textures
		let mut ds = ds_p.wrapping_offset(-1);
		while ds.addr() >= drawsegs.as_ptr().addr() {
			if !(*ds).maskedtexturecol.is_null() {
				R_RenderMaskedSegRange(
					ds,
					i32::try_from((*ds).x1).unwrap(),
					i32::try_from((*ds).x2).unwrap(),
				);
			}
			ds = ds.wrapping_offset(-1);
		}

		// draw the psprites on top of everything
		//  but does not draw on side views
		if viewangleoffset == 0 {
			R_DrawPlayerSprites();
		}
	}
}
