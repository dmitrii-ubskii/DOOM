#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::{
	ffi::c_char,
	ptr::{null, null_mut},
};

use crate::{
	am_map::automapactive,
	d_englsh::{
		C1TEXT, C2TEXT, C3TEXT, C4TEXT, C5TEXT, C6TEXT, CC_ARACH, CC_ARCH, CC_BARON, CC_CACO,
		CC_CYBER, CC_DEMON, CC_HEAVY, CC_HELL, CC_HERO, CC_IMP, CC_LOST, CC_MANCU, CC_PAIN,
		CC_REVEN, CC_SHOTGUN, CC_SPIDER, CC_ZOMBIE, E1TEXT, E2TEXT, E3TEXT, E4TEXT,
	},
	d_event::{event_t, evtype_t, gameaction_t},
	d_main::wipegamestate,
	doomdef::{GameMode_t, MAXPLAYERS, SCREENHEIGHT, SCREENWIDTH, gamestate_t},
	doomstat::gamemode,
	g_game::{gameaction, gameepisode, gamemap, gamestate, players, viewactive},
	hu_stuff::{HU_FONTSIZE, HU_FONTSTART, hu_font},
	info::{mobjinfo, mobjtype_t, state_t, statenum_t, states},
	p_pspr::FF_FRAMEMASK,
	r_data::firstspritelump,
	r_defs::{column_t, patch_t},
	r_things::sprites,
	s_sound::{S_ChangeMusic, S_StartMusic, S_StartSound},
	sounds::{musicenum_t, sfxenum_t},
	v_video::{V_DrawPatch, V_DrawPatchFlipped, V_MarkRect, screens},
	w_wad::{W_CacheLumpName, W_CacheLumpNum},
	z_zone::{PU_CACHE, PU_LEVEL},
};

// Stage of animation:
//  0 = text, 1 = art screen, 2 = character cast
static mut finalestage: i32 = 0;

static mut finalecount: i32 = 0;

const TEXTSPEED: usize = 3;
const TEXTWAIT: usize = 250;

static mut e1text: *const c_char = E1TEXT;
static mut e2text: *const c_char = E2TEXT;
static mut e3text: *const c_char = E3TEXT;
static mut e4text: *const c_char = E4TEXT;

static mut c1text: *const c_char = C1TEXT;
static mut c2text: *const c_char = C2TEXT;
static mut c3text: *const c_char = C3TEXT;
static mut c4text: *const c_char = C4TEXT;
static mut c5text: *const c_char = C5TEXT;
static mut c6text: *const c_char = C6TEXT;

/*
static mut p1text: *const c_char = P1TEXT;
static mut p2text: *const c_char = P2TEXT;
static mut p3text: *const c_char = P3TEXT;
static mut p4text: *const c_char = P4TEXT;
static mut p5text: *const c_char = P5TEXT;
static mut p6text: *const c_char = P6TEXT;

static mut t1text: *const c_char = T1TEXT;
static mut t2text: *const c_char = T2TEXT;
static mut t3text: *const c_char = T3TEXT;
static mut t4text: *const c_char = T4TEXT;
static mut t5text: *const c_char = T5TEXT;
static mut t6text: *const c_char = T6TEXT;
*/

static mut finaletext: *const c_char = null();
static mut finaleflat: *const c_char = null();

// F_StartFinale
pub(crate) fn F_StartFinale() {
	unsafe {
		gameaction = gameaction_t::ga_nothing;
		gamestate = gamestate_t::GS_FINALE;
		viewactive = false;
		automapactive = false;

		// Okay - IWAD dependend stuff.
		// This has been changed severly, and
		//  some stuff might have changed in the process.
		match gamemode {
			// DOOM 1 - E1, E3 or E4, but each nine missions
			GameMode_t::shareware | GameMode_t::registered | GameMode_t::retail => {
				S_ChangeMusic(musicenum_t::mus_victor, true);

				match gameepisode {
					1 => {
						finaleflat = c"FLOOR4_8".as_ptr();
						finaletext = e1text;
					}
					2 => {
						finaleflat = c"SFLR6_1".as_ptr();
						finaletext = e2text;
					}
					3 => {
						finaleflat = c"MFLR8_4".as_ptr();
						finaletext = e3text;
					}
					4 => {
						finaleflat = c"MFLR8_3".as_ptr();
						finaletext = e4text;
					}
					_ => (), // Ouch.
				}
			}

			// DOOM II and missions packs with E1, M34
			GameMode_t::commercial => {
				S_ChangeMusic(musicenum_t::mus_read_m, true);

				match gamemap {
					6 => {
						finaleflat = c"SLIME16".as_ptr();
						finaletext = c1text;
					}
					11 => {
						finaleflat = c"RROCK14".as_ptr();
						finaletext = c2text;
					}
					20 => {
						finaleflat = c"RROCK07".as_ptr();
						finaletext = c3text;
					}
					30 => {
						finaleflat = c"RROCK17".as_ptr();
						finaletext = c4text;
					}
					15 => {
						finaleflat = c"RROCK13".as_ptr();
						finaletext = c5text;
					}
					31 => {
						finaleflat = c"RROCK19".as_ptr();
						finaletext = c6text;
					}
					_ => (), // Ouch.
				}
			}

			// Indeterminate.
			_ => {
				S_ChangeMusic(musicenum_t::mus_read_m, true);
				finaleflat = c"F_SKY1".as_ptr(); // Not used anywhere else.
				finaletext = c1text; // FIXME - other text, music?
			}
		}

		finalestage = 0;
		finalecount = 0;
	}
}

pub(crate) fn F_Responder(event: *mut event_t) -> bool {
	unsafe { finalestage == 2 && F_CastResponder(event) }
}

// F_Ticker
pub(crate) fn F_Ticker() {
	unsafe {
		// check for skipping
		if gamemode == GameMode_t::commercial && finalecount > 50 {
			// go on to the next level
			let i = (0..MAXPLAYERS).find(|&i| players[i].cmd.buttons != 0);

			if i.is_some() {
				if gamemap == 30 {
					F_StartCast();
				} else {
					gameaction = gameaction_t::ga_worlddone;
				}
			}
		}

		// advance animation
		finalecount += 1;

		if finalestage == 2 {
			F_CastTicker();
			return;
		}

		if gamemode == GameMode_t::commercial {
			return;
		}

		if finalestage == 0
			&& usize::try_from(finalecount).unwrap()
				> libc::strlen(finaletext) * TEXTSPEED + TEXTWAIT
		{
			finalecount = 0;
			finalestage = 1;
			wipegamestate = gamestate_t::None; // force a wipe
			if gameepisode == 3 {
				S_StartMusic(musicenum_t::mus_bunny);
			}
		}
	}
}

// F_TextWrite
fn F_TextWrite() {
	unsafe {
		// erase the entire screen to a tiled background
		let src = W_CacheLumpName(finaleflat, PU_CACHE);
		let mut dest = screens[0];

		for y in 0..SCREENHEIGHT {
			for _x in 0..SCREENWIDTH / 64 {
				libc::memcpy(dest.cast(), src.wrapping_add((y & 63) << 6), 64);
				dest = dest.wrapping_add(64);
			}
			if SCREENWIDTH & 63 != 0 {
				libc::memcpy(dest.cast(), src.wrapping_add((y & 63) << 6), SCREENWIDTH & 63);
				dest = dest.wrapping_add(SCREENWIDTH & 63);
			}
		}

		V_MarkRect(0, 0, SCREENWIDTH, SCREENHEIGHT);

		// draw some of the text onto the screen
		let mut cx = 10;
		let mut cy = 10;
		let mut ch = finaletext;

		let mut count = (finalecount - 10) / i32::try_from(TEXTSPEED).unwrap();
		if count < 0 {
			count = 0;
		}
		while count != 0 {
			let c = u8::try_from(*ch).unwrap();
			ch = ch.wrapping_add(1);
			if c == 0 {
				break;
			}
			if c == b'\n' {
				cx = 10;
				cy += 11;
				count -= 1;
				continue;
			}

			let c = c.to_ascii_uppercase().wrapping_sub(HU_FONTSTART);
			if c > HU_FONTSIZE {
				cx += 4;
				count -= 1;
				continue;
			}

			let w = (*hu_font[usize::from(c)]).width;
			if cx + w > i16::try_from(SCREENWIDTH).unwrap() {
				break;
			}
			V_DrawPatch(usize::try_from(cx).unwrap(), cy, 0, hu_font[usize::from(c)]);
			cx += w;
			count -= 1;
		}
	}
}

// Final DOOM 2 animation
// Casting by id Software.
//   in order of appearance
struct castinfo_t {
	name: *const c_char,
	ty: mobjtype_t,
}

static mut castorder: [castinfo_t; 18] = [
	castinfo_t { name: CC_ZOMBIE, ty: mobjtype_t::MT_POSSESSED },
	castinfo_t { name: CC_SHOTGUN, ty: mobjtype_t::MT_SHOTGUY },
	castinfo_t { name: CC_HEAVY, ty: mobjtype_t::MT_CHAINGUY },
	castinfo_t { name: CC_IMP, ty: mobjtype_t::MT_TROOP },
	castinfo_t { name: CC_DEMON, ty: mobjtype_t::MT_SERGEANT },
	castinfo_t { name: CC_LOST, ty: mobjtype_t::MT_SKULL },
	castinfo_t { name: CC_CACO, ty: mobjtype_t::MT_HEAD },
	castinfo_t { name: CC_HELL, ty: mobjtype_t::MT_KNIGHT },
	castinfo_t { name: CC_BARON, ty: mobjtype_t::MT_BRUISER },
	castinfo_t { name: CC_ARACH, ty: mobjtype_t::MT_BABY },
	castinfo_t { name: CC_PAIN, ty: mobjtype_t::MT_PAIN },
	castinfo_t { name: CC_REVEN, ty: mobjtype_t::MT_UNDEAD },
	castinfo_t { name: CC_MANCU, ty: mobjtype_t::MT_FATSO },
	castinfo_t { name: CC_ARCH, ty: mobjtype_t::MT_VILE },
	castinfo_t { name: CC_SPIDER, ty: mobjtype_t::MT_SPIDER },
	castinfo_t { name: CC_CYBER, ty: mobjtype_t::MT_CYBORG },
	castinfo_t { name: CC_HERO, ty: mobjtype_t::MT_PLAYER },
	castinfo_t { name: null(), ty: mobjtype_t::MT_PLAYER },
];

static mut castnum: usize = 0;
static mut casttics: i32 = 0;
static mut caststate: *mut state_t = null_mut();
static mut castdeath: bool = false;
static mut castframes: usize = 0;
static mut castonmelee: bool = false;
static mut castattacking: bool = false;

// F_StartCast
fn F_StartCast() {
	unsafe {
		wipegamestate = gamestate_t::None; // force a screen wipe
		castnum = 0;
		caststate =
			&raw mut states[usize::from(mobjinfo[usize::from(castorder[castnum].ty)].seestate)];
		casttics = (*caststate).tics;
		castdeath = false;
		finalestage = 2;
		castframes = 0;
		castonmelee = false;
		castattacking = false;
		S_ChangeMusic(musicenum_t::mus_evil, true);
	}
}

// F_CastTicker
fn F_CastTicker() {
	unsafe {
		casttics -= 1;
		if casttics > 0 {
			return; // not time to change state yet
		}

		if (*caststate).tics == -1 || (*caststate).nextstate == statenum_t::S_NULL {
			// switch from deathstate to next monster
			castnum += 1;
			castdeath = false;
			if castorder[castnum].name.is_null() {
				castnum = 0;
			}
			if mobjinfo[usize::from(castorder[castnum].ty)].seesound == sfxenum_t::sfx_None {
				S_StartSound(null_mut(), mobjinfo[usize::from(castorder[castnum].ty)].seesound);
			}
			caststate =
				&raw mut states[usize::from(mobjinfo[usize::from(castorder[castnum].ty)].seestate)];
			castframes = 0;
		} else {
			// just advance to next state in animation
			if caststate == &raw mut states[usize::from(statenum_t::S_PLAY_ATK1)] {
				// Oh, gross hack!
				castattacking = false;
				castframes = 0;
				caststate = &raw mut states
					[usize::from(mobjinfo[usize::from(castorder[castnum].ty)].seestate)];
				casttics = (*caststate).tics;
				if casttics == -1 {
					casttics = 15;
				}
				return;
			}
			let st = (*caststate).nextstate;
			caststate = &raw mut states[usize::from(st)];
			castframes += 1;

			// sound hacks....
			let sfx = match st {
				statenum_t::S_PLAY_ATK1 => sfxenum_t::sfx_dshtgn,
				statenum_t::S_POSS_ATK2 => sfxenum_t::sfx_pistol,
				statenum_t::S_SPOS_ATK2 => sfxenum_t::sfx_shotgn,
				statenum_t::S_VILE_ATK2 => sfxenum_t::sfx_vilatk,
				statenum_t::S_SKEL_FIST2 => sfxenum_t::sfx_skeswg,
				statenum_t::S_SKEL_FIST4 => sfxenum_t::sfx_skepch,
				statenum_t::S_SKEL_MISS2 => sfxenum_t::sfx_skeatk,
				statenum_t::S_FATT_ATK8 | statenum_t::S_FATT_ATK5 | statenum_t::S_FATT_ATK2 => {
					sfxenum_t::sfx_firsht
				}
				statenum_t::S_CPOS_ATK2 | statenum_t::S_CPOS_ATK3 | statenum_t::S_CPOS_ATK4 => {
					sfxenum_t::sfx_shotgn
				}
				statenum_t::S_TROO_ATK3 => sfxenum_t::sfx_claw,
				statenum_t::S_SARG_ATK2 => sfxenum_t::sfx_sgtatk,
				statenum_t::S_BOSS_ATK2 | statenum_t::S_BOS2_ATK2 | statenum_t::S_HEAD_ATK2 => {
					sfxenum_t::sfx_firsht
				}
				statenum_t::S_SKULL_ATK2 => sfxenum_t::sfx_sklatk,
				statenum_t::S_SPID_ATK2 | statenum_t::S_SPID_ATK3 => sfxenum_t::sfx_shotgn,
				statenum_t::S_BSPI_ATK2 => sfxenum_t::sfx_plasma,
				statenum_t::S_CYBER_ATK2 | statenum_t::S_CYBER_ATK4 | statenum_t::S_CYBER_ATK6 => {
					sfxenum_t::sfx_rlaunc
				}
				statenum_t::S_PAIN_ATK3 => sfxenum_t::sfx_sklatk,
				_ => sfxenum_t::sfx_None,
			};

			if sfx != sfxenum_t::sfx_None {
				S_StartSound(null_mut(), sfx);
			}
		}

		if castframes == 12 {
			// go into attack frame
			castattacking = true;
			if castonmelee {
				caststate = &raw mut states
					[usize::from(mobjinfo[usize::from(castorder[castnum].ty)].meleestate)];
			} else {
				caststate = &raw mut states
					[usize::from(mobjinfo[usize::from(castorder[castnum].ty)].missilestate)];
			}
			castonmelee = !castonmelee;
			if caststate == &raw mut states[usize::from(statenum_t::S_NULL)] {
				if castonmelee {
					caststate = &raw mut states
						[usize::from(mobjinfo[usize::from(castorder[castnum].ty)].meleestate)];
				} else {
					caststate = &raw mut states
						[usize::from(mobjinfo[usize::from(castorder[castnum].ty)].missilestate)];
				}
			}
		}

		if castattacking {
			if castframes == 24
				|| caststate
					== &raw mut states
						[usize::from(mobjinfo[usize::from(castorder[castnum].ty)].seestate)]
			{
				castattacking = false;
				castframes = 0;
				caststate = &raw mut states
					[usize::from(mobjinfo[usize::from(castorder[castnum].ty)].seestate)];
			}
		}

		casttics = (*caststate).tics;
		if casttics == -1 {
			casttics = 15;
		}
	}
}

// F_CastResponder
fn F_CastResponder(ev: *mut event_t) -> bool {
	unsafe {
		if (*ev).ty != evtype_t::ev_keydown {
			return false;
		}

		if castdeath {
			return true; // already in dying frames
		}

		// go into death frame
		castdeath = true;
		caststate =
			&raw mut states[usize::from(mobjinfo[usize::from(castorder[castnum].ty)].deathstate)];
		casttics = (*caststate).tics;
		castframes = 0;
		castattacking = false;
		if mobjinfo[usize::from(castorder[castnum].ty)].deathsound != sfxenum_t::sfx_None {
			S_StartSound(null_mut(), mobjinfo[usize::from(castorder[castnum].ty)].deathsound);
		}

		true
	}
}

fn F_CastPrint(text: *const c_char) {
	unsafe {
		// find width
		let mut ch = text;
		let mut width = 0;

		while !ch.is_null() {
			let mut c = u8::try_from(*ch).unwrap();
			ch = ch.wrapping_add(1);
			if c == 0 {
				break;
			}
			c = c.to_ascii_uppercase().wrapping_sub(HU_FONTSTART);
			if c > HU_FONTSIZE {
				width += 4;
				continue;
			}

			let w = (*hu_font[usize::from(c)]).width;
			width += w;
		}

		// draw it
		let mut ch = text;
		let mut cx = 160 - width / 2;
		while !ch.is_null() {
			let mut c = u8::try_from(*ch).unwrap();
			ch = ch.wrapping_add(1);
			if c == 0 {
				break;
			}
			c = c.to_ascii_uppercase().wrapping_sub(HU_FONTSTART);
			if c > HU_FONTSIZE {
				cx += 4;
				continue;
			}

			let w = (*hu_font[usize::from(c)]).width;
			V_DrawPatch(usize::try_from(cx).unwrap(), 180, 0, hu_font[usize::from(c)]);
			cx += w;
		}
	}
}

// F_CastDrawer
fn F_CastDrawer() {
	unsafe {
		// erase the entire screen to a background
		V_DrawPatch(0, 0, 0, W_CacheLumpName(c"BOSSBACK".as_ptr(), PU_CACHE).cast());

		F_CastPrint(castorder[castnum].name);

		// draw the current frame in the middle of the screen
		let sprdef = sprites.wrapping_add(usize::from((*caststate).sprite));
		let sprframe = (*sprdef).spriteframes.wrapping_add((*caststate).frame & FF_FRAMEMASK);
		let lump = (*sprframe).lump[0];
		let flip = (*sprframe).flip[0] != 0;

		let patch =
			W_CacheLumpNum(usize::try_from(lump).unwrap() + firstspritelump, PU_CACHE).cast();
		if flip {
			V_DrawPatchFlipped(160, 170, 0, patch);
		} else {
			V_DrawPatch(160, 170, 0, patch);
		}
	}
}

// F_DrawPatchCol
fn F_DrawPatchCol(x: usize, patch: *mut patch_t, col: usize) {
	unsafe {
		let mut column = patch.wrapping_byte_add((*patch).columnofs[col]).cast::<column_t>();
		let desttop = screens[0].wrapping_add(x);
		while (*column).topdelta != 0xff {
			let mut source = column.cast::<u8>().wrapping_add(3);
			let mut dest = desttop.wrapping_add(usize::from((*column).topdelta) * SCREENWIDTH);
			let mut count = (*column).length;
			while count != 0 {
				*dest = *source;
				source = source.wrapping_add(1);
				dest = dest.wrapping_add(SCREENWIDTH);
				count -= 1;
			}
			column = column.wrapping_byte_add(usize::from((*column).length) + 4);
		}
	}
}

// F_BunnyScroll
fn F_BunnyScroll() {
	unsafe {
		static mut laststage: i32 = 0;

		let p1 = W_CacheLumpName(c"PFUB2".as_ptr(), PU_LEVEL).cast();
		let p2 = W_CacheLumpName(c"PFUB1".as_ptr(), PU_LEVEL).cast();

		V_MarkRect(0, 0, SCREENWIDTH, SCREENHEIGHT);

		let scrolled = usize::try_from((320 - (finalecount - 230) / 2).clamp(0, 320)).unwrap();

		for x in 0..SCREENWIDTH {
			if x + scrolled < 320 {
				F_DrawPatchCol(x, p1, x + scrolled);
			} else {
				F_DrawPatchCol(x, p2, x + scrolled - 320);
			}
		}

		if finalecount < 1130 {
			return;
		}
		if finalecount < 1180 {
			V_DrawPatch(
				(SCREENWIDTH - 13 * 8) / 2,
				(SCREENHEIGHT - 8 * 8) / 2,
				0,
				W_CacheLumpName(c"END0".as_ptr(), PU_CACHE).cast(),
			);
			laststage = 0;
			return;
		}

		let mut stage = (finalecount - 1180) / 5;
		if stage > 6 {
			stage = 6;
		}
		if stage > laststage {
			S_StartSound(null_mut(), sfxenum_t::sfx_pistol);
			laststage = stage;
		}

		let mut name = [0; 10];
		libc::sprintf(name.as_mut_ptr(), c"END%i".as_ptr(), stage);
		V_DrawPatch(
			(SCREENWIDTH - 13 * 8) / 2,
			(SCREENHEIGHT - 8 * 8) / 2,
			0,
			W_CacheLumpName(name.as_ptr(), PU_CACHE).cast(),
		);
	}
}

// F_Drawer
pub(crate) fn F_Drawer() {
	unsafe {
		if finalestage == 2 {
			F_CastDrawer();
			return;
		}

		if finalestage == 0 {
			F_TextWrite();
		} else {
			match gameepisode {
				1 if gamemode == GameMode_t::retail => {
					V_DrawPatch(0, 0, 0, W_CacheLumpName(c"CREDIT".as_ptr(), PU_CACHE).cast())
				}
				1 => V_DrawPatch(0, 0, 0, W_CacheLumpName(c"HELP2".as_ptr(), PU_CACHE).cast()),
				2 => V_DrawPatch(0, 0, 0, W_CacheLumpName(c"VICTORY2".as_ptr(), PU_CACHE).cast()),
				3 => F_BunnyScroll(),
				4 => V_DrawPatch(0, 0, 0, W_CacheLumpName(c"ENDPIC".as_ptr(), PU_CACHE).cast()),
				_ => (),
			}
		}
	}
}
