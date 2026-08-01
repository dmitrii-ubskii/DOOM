//	All the clipping: columns, horizontal spans, sky columns.
#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::{num::Wrapping, ptr::null_mut};

use libc::memcpy;

use crate::{
	doomdata::{ML_DONTPEGBOTTOM, ML_DONTPEGTOP, ML_MAPPED},
	i_system::I_Error,
	m_fixed::{FRACBITS, FixedMul, fixed_t},
	r_bsp::{backsector, curline, drawsegs, ds_p, frontsector, linedef, sidedef},
	r_data::{R_GetColumn, textureheight, texturetranslation},
	r_defs::{MAXDRAWSEGS, SIL_BOTH, SIL_BOTTOM, SIL_TOP, drawseg_t, lighttable_t},
	r_draw::{
		dc_colormap, dc_iscale, dc_source, dc_texturemid, dc_x, dc_yh, dc_yl, viewheight, viewwidth,
	},
	r_main::{
		LIGHTLEVELS, LIGHTSCALESHIFT, LIGHTSEGSHIFT, MAXLIGHTSCALE, R_PointToDist,
		R_ScaleFromGlobalAngle, centeryfrac, colfunc, extralight, fixedcolormap, scalelight,
		viewangle, viewz, xtoviewangle,
	},
	r_plane::{R_CheckPlane, ceilingclip, ceilingplane, floorclip, floorplane, lastopening},
	r_sky::skyflatnum,
	r_things::{
		R_DrawMaskedColumn, mceilingclip, mfloorclip, negonearray, screenheightarray, sprtopscreen,
		spryscale,
	},
	tables::{ANG90, ANG180, ANGLETOFINESHIFT, angle_t, finesine, finetangent},
};

// OPTIMIZE: closed two sided lines as single sided

// True if any of the segs textures might be visible.
static mut segtextured: bool = false;

// False if the back side is the same plane.
static mut markfloor: bool = false;
static mut markceiling: bool = false;

static mut maskedtexture: bool = false;
static mut toptexture: usize = 0;
static mut bottomtexture: usize = 0;
static mut midtexture: usize = 0;

pub(crate) static mut rw_normalangle: angle_t = Wrapping(0);
// angle to line origin
pub(crate) static mut rw_angle1: angle_t = Wrapping(0);

// regular wall
static mut rw_x: u32 = 0;
static mut rw_stopx: u32 = 0;
static mut rw_centerangle: angle_t = Wrapping(0);
static mut rw_offset: fixed_t = 0;
pub(crate) static mut rw_distance: fixed_t = 0;
static mut rw_scale: fixed_t = 0;
static mut rw_scalestep: fixed_t = 0;
static mut rw_midtexturemid: fixed_t = 0;
static mut rw_toptexturemid: fixed_t = 0;
static mut rw_bottomtexturemid: fixed_t = 0;

static mut worldtop: i32 = 0;
static mut worldbottom: i32 = 0;
static mut worldhigh: i32 = 0;
static mut worldlow: i32 = 0;

static mut pixhigh: fixed_t = 0;
static mut pixlow: fixed_t = 0;
static mut pixhighstep: fixed_t = 0;
static mut pixlowstep: fixed_t = 0;

static mut topfrac: fixed_t = 0;
static mut topstep: fixed_t = 0;

static mut bottomfrac: fixed_t = 0;
static mut bottomstep: fixed_t = 0;

pub(crate) static mut walllights: *mut *mut lighttable_t = null_mut();

pub(crate) static mut maskedtexturecol: *mut i16 = null_mut();

// R_RenderMaskedSegRange
#[allow(static_mut_refs)]
pub(crate) unsafe fn R_RenderMaskedSegRange(ds: *mut drawseg_t, x1: i32, x2: i32) {
	unsafe {
		// Calculate light table.
		// Use different light tables
		//   for horizontal / vertical / diagonal. Diagonal?
		// OPTIMIZE: get rid of LIGHTSEGSHIFT globally
		curline = (*ds).curline;
		frontsector = (*curline).frontsector;
		backsector = (*curline).backsector;
		let texnum =
			*texturetranslation.wrapping_add((*(*curline).sidedef).midtexture.try_into().unwrap());

		let mut lightnum = i32::from((*frontsector).lightlevel >> LIGHTSEGSHIFT) + extralight;

		if (*(*curline).v1).y == (*(*curline).v2).y {
			lightnum -= 1;
		} else if (*(*curline).v1).x == (*(*curline).v2).x {
			lightnum += 1;
		}

		if lightnum < 0 {
			walllights = scalelight[0].as_mut_ptr();
		} else if usize::try_from(lightnum).unwrap() >= LIGHTLEVELS {
			walllights = scalelight[LIGHTLEVELS - 1].as_mut_ptr();
		} else {
			walllights = scalelight[usize::try_from(lightnum).unwrap()].as_mut_ptr();
		}

		maskedtexturecol = (*ds).maskedtexturecol;

		rw_scalestep = (*ds).scalestep;
		spryscale = (*ds).scale1 + (x1 - i32::try_from((*ds).x1).unwrap()) * rw_scalestep;
		mfloorclip = (*ds).sprbottomclip;
		mceilingclip = (*ds).sprtopclip;

		// find positioning
		if (*(*curline).linedef).flags & ML_DONTPEGBOTTOM != 0 {
			dc_texturemid = i32::max((*frontsector).floorheight, (*backsector).floorheight);
			dc_texturemid += *textureheight.wrapping_add(texnum) - viewz;
		} else {
			dc_texturemid = i32::min((*frontsector).ceilingheight, (*backsector).ceilingheight);
			dc_texturemid -= viewz;
		}
		dc_texturemid += (*(*curline).sidedef).rowoffset;

		if !fixedcolormap.is_null() {
			dc_colormap = fixedcolormap;
		}

		// draw the columns
		dc_x = x1;
		while dc_x <= x2 {
			// calculate lighting
			if *maskedtexturecol.wrapping_add(dc_x.try_into().unwrap()) != i16::MAX {
				if fixedcolormap.is_null() {
					let mut index = spryscale >> LIGHTSCALESHIFT;

					if index >= MAXLIGHTSCALE.try_into().unwrap() {
						index = i32::try_from(MAXLIGHTSCALE).unwrap() - 1;
					}

					dc_colormap = *walllights.wrapping_add(index.try_into().unwrap());
				}

				sprtopscreen = centeryfrac - FixedMul(dc_texturemid, spryscale);
				dc_iscale = (0xffffffff / u32::try_from(spryscale).unwrap()).try_into().unwrap();

				// draw the texture
				let col = R_GetColumn(
					texnum,
					(i32::from(*maskedtexturecol).wrapping_add(dc_x))
						.cast_unsigned()
						.try_into()
						.unwrap(),
				)
				.wrapping_byte_sub(3)
				.cast();

				R_DrawMaskedColumn(col);
				*maskedtexturecol.wrapping_add(dc_x.try_into().unwrap()) = i16::MAX;
			}
			spryscale += rw_scalestep;
			dc_x += 1;
		}
	}
}

// R_RenderSegLoop
// Draws zero, one, or two textures (and possibly a masked
//  texture) for walls.
// Can draw or mark the starting pixel of floor and ceiling
//  textures.
// CALLED: CORE LOOPING ROUTINE.
const HEIGHTBITS: usize = 12;
const HEIGHTUNIT: i32 = 1 << HEIGHTBITS;

#[allow(static_mut_refs)]
fn R_RenderSegLoop() {
	unsafe {
		while rw_x < rw_stopx {
			// mark floor / ceiling areas
			let mut yl = (topfrac + HEIGHTUNIT - 1) >> HEIGHTBITS;

			// no space above wall?
			if yl < i32::from(ceilingclip[usize::try_from(rw_x).unwrap()]) + 1 {
				yl = i32::from(ceilingclip[usize::try_from(rw_x).unwrap()]) + 1;
			}

			if markceiling {
				let top = i32::from(ceilingclip[usize::try_from(rw_x).unwrap()]) + 1;
				let mut bottom = yl - 1;

				if bottom >= floorclip[usize::try_from(rw_x).unwrap()].into() {
					bottom = i32::from(floorclip[usize::try_from(rw_x).unwrap()]) - 1;
				}

				if top <= bottom {
					(*ceilingplane).top[usize::try_from(rw_x).unwrap()] = top.try_into().unwrap();
					(*ceilingplane).bottom[usize::try_from(rw_x).unwrap()] =
						bottom.try_into().unwrap();
				}
			}

			let mut yh = bottomfrac >> HEIGHTBITS;

			if yh >= floorclip[usize::try_from(rw_x).unwrap()].into() {
				yh = i32::from(floorclip[usize::try_from(rw_x).unwrap()]) - 1;
			}

			if markfloor {
				let mut top = yh + 1;
				let bottom = i32::from(floorclip[usize::try_from(rw_x).unwrap()]) - 1;
				if top <= ceilingclip[usize::try_from(rw_x).unwrap()].into() {
					top = i32::from(ceilingclip[usize::try_from(rw_x).unwrap()]) + 1;
				}
				if top <= bottom {
					(*floorplane).top[usize::try_from(rw_x).unwrap()] = top.try_into().unwrap();
					(*floorplane).bottom[usize::try_from(rw_x).unwrap()] =
						bottom.try_into().unwrap();
				}
			}

			let mut texturecolumn: fixed_t = 0;

			// texturecolumn and lighting are independent of wall tiers
			if segtextured {
				// calculate texture offset
				let angle = (rw_centerangle + xtoviewangle[usize::try_from(rw_x).unwrap()])
					>> ANGLETOFINESHIFT;
				texturecolumn = rw_offset - FixedMul(finetangent[angle.0], rw_distance);
				texturecolumn >>= FRACBITS;
				// calculate lighting
				let mut index = usize::try_from(rw_scale).unwrap() >> LIGHTSCALESHIFT;

				if index >= MAXLIGHTSCALE {
					index = MAXLIGHTSCALE - 1;
				}

				dc_colormap = *walllights.wrapping_add(index);
				dc_x = rw_x.try_into().unwrap();
				dc_iscale =
					fixed_t::try_from(0xffffffff / u32::try_from(rw_scale).unwrap()).unwrap();
			}

			// draw the wall tiers
			if midtexture != 0 {
				// single sided line
				dc_yl = yl;
				dc_yh = yh;
				dc_texturemid = rw_midtexturemid;
				dc_source =
					R_GetColumn(midtexture, texturecolumn.cast_unsigned().try_into().unwrap());
				colfunc();
				ceilingclip[usize::try_from(rw_x).unwrap()] = viewheight.try_into().unwrap();
				floorclip[usize::try_from(rw_x).unwrap()] = -1;
			} else {
				// two sided line
				if toptexture != 0 {
					// top wall
					let mut mid = (pixhigh >> HEIGHTBITS).try_into().unwrap();
					pixhigh += pixhighstep;

					if mid >= floorclip[usize::try_from(rw_x).unwrap()] {
						mid = floorclip[usize::try_from(rw_x).unwrap()] - 1;
					}

					if i32::from(mid) >= yl {
						dc_yl = yl;
						dc_yh = mid.into();
						dc_texturemid = rw_toptexturemid;
						dc_source = R_GetColumn(
							toptexture,
							texturecolumn.cast_unsigned().try_into().unwrap(),
						);
						colfunc();
						ceilingclip[usize::try_from(rw_x).unwrap()] = mid;
					} else {
						ceilingclip[usize::try_from(rw_x).unwrap()] =
							i16::try_from(yl).unwrap() - 1;
					}
				} else {
					// no top wall
					if markceiling {
						ceilingclip[usize::try_from(rw_x).unwrap()] =
							i16::try_from(yl).unwrap() - 1;
					}
				}

				if bottomtexture != 0 {
					// bottom wall
					let mut mid = ((pixlow + HEIGHTUNIT - 1) >> HEIGHTBITS).try_into().unwrap();
					pixlow += pixlowstep;

					// no space above wall?
					if mid <= ceilingclip[usize::try_from(rw_x).unwrap()] {
						mid = ceilingclip[usize::try_from(rw_x).unwrap()] + 1;
					}

					if i32::from(mid) <= yh {
						dc_yl = mid.into();
						dc_yh = yh;
						dc_texturemid = rw_bottomtexturemid;
						dc_source = R_GetColumn(
							bottomtexture,
							texturecolumn.cast_unsigned().try_into().unwrap(),
						);
						colfunc();
						floorclip[usize::try_from(rw_x).unwrap()] = mid;
					} else {
						floorclip[usize::try_from(rw_x).unwrap()] = i16::try_from(yh).unwrap() + 1;
					}
				} else {
					// no bottom wall
					if markfloor {
						floorclip[usize::try_from(rw_x).unwrap()] = i16::try_from(yh).unwrap() + 1;
					}
				}

				if maskedtexture {
					// save texturecol
					//  for backdrawing of masked mid texture
					*maskedtexturecol.wrapping_add(rw_x.try_into().unwrap()) =
						texturecolumn.try_into().unwrap();
				}
			}

			rw_scale += rw_scalestep;
			topfrac += topstep;
			bottomfrac += bottomstep;

			rw_x += 1;
		}
	}
}

// R_StoreWallRange
// A wall segment will be drawn
//  between start and stop pixels (inclusive).
#[allow(static_mut_refs)]
pub(crate) fn R_StoreWallRange(start: u32, stop: u32) {
	unsafe {
		// don't overflow and crash
		if ds_p == drawsegs.as_mut_ptr().wrapping_add(MAXDRAWSEGS) {
			return;
		}

		if usize::try_from(start).unwrap() >= viewwidth || start > stop {
			I_Error(format_args!("Bad R_RenderWallRange: {} to {}", start, stop));
		}

		sidedef = (*curline).sidedef;
		linedef = (*curline).linedef;

		// mark the segment as visible for auto map
		(*linedef).flags |= i16::try_from(ML_MAPPED).unwrap();

		// calculate rw_distance for scale calculation
		rw_normalangle = (*curline).angle + ANG90;
		let mut offsetangle = Wrapping((rw_normalangle - rw_angle1).0.cast_signed().unsigned_abs());

		if offsetangle > ANG90 {
			offsetangle = ANG90;
		}

		let distangle = ANG90 - offsetangle;
		let hyp = R_PointToDist((*(*curline).v1).x, (*(*curline).v1).y);
		let sineval = finesine[distangle.0 >> ANGLETOFINESHIFT];
		rw_distance = FixedMul(hyp, sineval);

		(*ds_p).x1 = start.try_into().unwrap();
		rw_x = start;
		(*ds_p).x2 = stop.try_into().unwrap();
		(*ds_p).curline = curline;
		rw_stopx = stop + 1;

		// calculate scale at both ends and step
		rw_scale =
			R_ScaleFromGlobalAngle(viewangle + xtoviewangle[usize::try_from(start).unwrap()]);
		(*ds_p).scale1 = rw_scale;

		if stop > start {
			(*ds_p).scale2 =
				R_ScaleFromGlobalAngle(viewangle + xtoviewangle[usize::try_from(stop).unwrap()]);
			rw_scalestep = ((*ds_p).scale2 - rw_scale) / fixed_t::try_from(stop - start).unwrap();
			(*ds_p).scalestep = rw_scalestep;
		} else {
			(*ds_p).scale2 = (*ds_p).scale1;
		}

		// calculate texture boundaries
		//  and decide if floor / ceiling marks are needed
		worldtop = (*frontsector).ceilingheight - viewz;
		worldbottom = (*frontsector).floorheight - viewz;

		midtexture = 0;
		toptexture = 0;
		bottomtexture = 0;
		maskedtexture = false;
		(*ds_p).maskedtexturecol = null_mut();

		let vtop: fixed_t;

		if backsector.is_null() {
			// single sided line
			midtexture =
				*texturetranslation.wrapping_add((*sidedef).midtexture.try_into().unwrap());
			// a single sided line is terminal, so it must mark ends
			markfloor = true;
			markceiling = true;
			if (*linedef).flags & ML_DONTPEGBOTTOM != 0 {
				vtop = (*frontsector).floorheight
					+ *textureheight.wrapping_add((*sidedef).midtexture.try_into().unwrap());
				// bottom of texture at bottom
				rw_midtexturemid = vtop - viewz;
			} else {
				// top of texture at top
				rw_midtexturemid = worldtop;
			}
			rw_midtexturemid += (*sidedef).rowoffset;

			(*ds_p).silhouette = SIL_BOTH;
			(*ds_p).sprtopclip = screenheightarray.as_mut_ptr();
			(*ds_p).sprbottomclip = negonearray.as_mut_ptr();
			(*ds_p).bsilheight = i32::MAX;
			(*ds_p).tsilheight = i32::MIN;
		} else {
			// two sided line
			(*ds_p).sprtopclip = null_mut();
			(*ds_p).sprbottomclip = null_mut();
			(*ds_p).silhouette = 0;

			if (*frontsector).floorheight > (*backsector).floorheight {
				(*ds_p).silhouette = SIL_BOTTOM;
				(*ds_p).bsilheight = (*frontsector).floorheight;
			} else if (*backsector).floorheight > viewz {
				(*ds_p).silhouette = SIL_BOTTOM;
				(*ds_p).bsilheight = i32::MAX;
				// (*ds_p).sprbottomclip = negonearray;
			}

			if (*frontsector).ceilingheight < (*backsector).ceilingheight {
				(*ds_p).silhouette |= SIL_TOP;
				(*ds_p).tsilheight = (*frontsector).ceilingheight;
			} else if (*backsector).ceilingheight < viewz {
				(*ds_p).silhouette |= SIL_TOP;
				(*ds_p).tsilheight = i32::MIN;
				// (*ds_p).sprtopclip = screenheightarray;
			}

			if (*backsector).ceilingheight <= (*frontsector).floorheight {
				(*ds_p).sprbottomclip = negonearray.as_mut_ptr();
				(*ds_p).bsilheight = i32::MAX;
				(*ds_p).silhouette |= SIL_BOTTOM;
			}

			if (*backsector).floorheight >= (*frontsector).ceilingheight {
				(*ds_p).sprtopclip = screenheightarray.as_mut_ptr();
				(*ds_p).tsilheight = i32::MIN;
				(*ds_p).silhouette |= SIL_TOP;
			}

			worldhigh = (*backsector).ceilingheight - viewz;
			worldlow = (*backsector).floorheight - viewz;

			// hack to allow height changes in outdoor areas
			if usize::try_from((*frontsector).ceilingpic).unwrap() == skyflatnum
				&& usize::try_from((*backsector).ceilingpic).unwrap() == skyflatnum
			{
				worldtop = worldhigh;
			}

			if worldlow != worldbottom
				|| (*backsector).floorpic != (*frontsector).floorpic
				|| (*backsector).lightlevel != (*frontsector).lightlevel
			{
				markfloor = true;
			} else {
				// same plane on both sides
				markfloor = false;
			}

			if worldhigh != worldtop
				|| (*backsector).ceilingpic != (*frontsector).ceilingpic
				|| (*backsector).lightlevel != (*frontsector).lightlevel
			{
				markceiling = true;
			} else {
				// same plane on both sides
				markceiling = false;
			}

			if (*backsector).ceilingheight <= (*frontsector).floorheight
				|| (*backsector).floorheight >= (*frontsector).ceilingheight
			{
				// closed door
				markceiling = true;
				markfloor = true;
			}

			if worldhigh < worldtop {
				// top texture
				toptexture =
					*texturetranslation.wrapping_add((*sidedef).toptexture.try_into().unwrap());
				if (*linedef).flags & ML_DONTPEGTOP != 0 {
					// top of texture at top
					rw_toptexturemid = worldtop;
				} else {
					vtop = (*backsector).ceilingheight
						+ *textureheight.wrapping_add((*sidedef).toptexture.try_into().unwrap());

					// bottom of texture
					rw_toptexturemid = vtop - viewz;
				}
			}
			if worldlow > worldbottom {
				// bottom texture
				bottomtexture =
					*texturetranslation.wrapping_add((*sidedef).bottomtexture.try_into().unwrap());

				if (*linedef).flags & ML_DONTPEGBOTTOM != 0 {
					// bottom of texture at bottom
					// top of texture at top
					rw_bottomtexturemid = worldtop;
				} else {
					// top of texture at top
					rw_bottomtexturemid = worldlow;
				}
			}
			rw_toptexturemid += (*sidedef).rowoffset;
			rw_bottomtexturemid += (*sidedef).rowoffset;

			// allocate space for masked texture tables
			if (*sidedef).midtexture != 0 {
				// masked midtexture
				maskedtexture = true;
				maskedtexturecol = lastopening.wrapping_sub(rw_x.try_into().unwrap());
				(*ds_p).maskedtexturecol = maskedtexturecol;
				lastopening = lastopening.wrapping_add((rw_stopx - rw_x).try_into().unwrap());
			}
		}

		// calculate rw_offset (only needed for textured lines)
		segtextured = midtexture != 0 || toptexture != 0 || bottomtexture != 0 || maskedtexture;

		if segtextured {
			offsetangle = rw_normalangle - rw_angle1;

			if offsetangle > ANG180 {
				offsetangle = -offsetangle;
			}

			if offsetangle > ANG90 {
				offsetangle = ANG90;
			}

			let sineval = finesine[offsetangle.0 >> ANGLETOFINESHIFT];
			rw_offset = FixedMul(hyp, sineval);

			if rw_normalangle - rw_angle1 < ANG180 {
				rw_offset = -rw_offset;
			}

			rw_offset += (*sidedef).textureoffset + (*curline).offset;
			rw_centerangle = ANG90 + viewangle - rw_normalangle;

			// calculate light table
			//  use different light tables
			//  for horizontal / vertical / diagonal
			// OPTIMIZE: get rid of LIGHTSEGSHIFT globally
			if fixedcolormap.is_null() {
				let mut lightnum =
					i32::from((*frontsector).lightlevel >> LIGHTSEGSHIFT) + extralight;

				if (*(*curline).v1).y == (*(*curline).v2).y {
					lightnum -= 1;
				} else if (*(*curline).v1).x == (*(*curline).v2).x {
					lightnum += 1;
				}

				if lightnum < 0 {
					walllights = scalelight[0].as_mut_ptr();
				} else if usize::try_from(lightnum).unwrap() >= LIGHTLEVELS {
					walllights = scalelight[LIGHTLEVELS - 1].as_mut_ptr();
				} else {
					walllights = scalelight[usize::try_from(lightnum).unwrap()].as_mut_ptr();
				}
			}
		}

		// if a floor / ceiling plane is on the wrong side
		//  of the view plane, it is definitely invisible
		//  and doesn't need to be marked.

		if (*frontsector).floorheight >= viewz {
			// above view plane
			markfloor = false;
		}

		if (*frontsector).ceilingheight <= viewz
			&& usize::try_from((*frontsector).ceilingpic).unwrap() != skyflatnum
		{
			// below view plane
			markceiling = false;
		}

		// calculate incremental stepping values for texture edges
		worldtop >>= 4;
		worldbottom >>= 4;

		topstep = -FixedMul(rw_scalestep, worldtop);
		topfrac = (centeryfrac >> 4) - FixedMul(worldtop, rw_scale);

		bottomstep = -FixedMul(rw_scalestep, worldbottom);
		bottomfrac = (centeryfrac >> 4) - FixedMul(worldbottom, rw_scale);

		if !backsector.is_null() {
			worldhigh >>= 4;
			worldlow >>= 4;

			if worldhigh < worldtop {
				pixhigh = (centeryfrac >> 4) - FixedMul(worldhigh, rw_scale);
				pixhighstep = -FixedMul(rw_scalestep, worldhigh);
			}

			if worldlow > worldbottom {
				pixlow = (centeryfrac >> 4) - FixedMul(worldlow, rw_scale);
				pixlowstep = -FixedMul(rw_scalestep, worldlow);
			}
		}

		// render it
		if markceiling {
			ceilingplane = R_CheckPlane(
				&mut *ceilingplane,
				rw_x.try_into().unwrap(),
				isize::try_from(rw_stopx).unwrap() - 1,
			);
		}

		if markfloor {
			floorplane = R_CheckPlane(
				&mut *floorplane,
				rw_x.try_into().unwrap(),
				isize::try_from(rw_stopx).unwrap() - 1,
			);
		}

		R_RenderSegLoop();

		// save sprite clipping info
		if ((*ds_p).silhouette & SIL_TOP != 0 || maskedtexture) && (*ds_p).sprtopclip.is_null() {
			memcpy(
				lastopening.cast(),
				ceilingclip.as_ptr().wrapping_add(start.try_into().unwrap()).cast(),
				2 * usize::try_from(rw_stopx - start).unwrap(),
			);
			(*ds_p).sprtopclip = lastopening.wrapping_sub(start.try_into().unwrap());
			lastopening = lastopening.wrapping_add((rw_stopx - start).try_into().unwrap());
		}

		if (((*ds_p).silhouette & SIL_BOTTOM) != 0 || maskedtexture)
			&& (*ds_p).sprbottomclip.is_null()
		{
			memcpy(
				lastopening.cast(),
				floorclip.as_ptr().wrapping_add(start.try_into().unwrap()).cast(),
				2 * usize::try_from(rw_stopx - start).unwrap(),
			);
			(*ds_p).sprbottomclip = lastopening.wrapping_sub(start.try_into().unwrap());
			lastopening = lastopening.wrapping_add((rw_stopx - start).try_into().unwrap());
		}

		if maskedtexture && (*ds_p).silhouette & SIL_TOP == 0 {
			(*ds_p).silhouette |= SIL_TOP;
			(*ds_p).tsilheight = i32::MIN;
		}
		if maskedtexture && (*ds_p).silhouette & SIL_BOTTOM == 0 {
			(*ds_p).silhouette |= SIL_BOTTOM;
			(*ds_p).bsilheight = i32::MAX;
		}
		ds_p = ds_p.wrapping_add(1);
	}
}
