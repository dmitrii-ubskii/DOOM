#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

// States for the intermission

use std::ptr::null_mut;

use crate::{
	d_event::{BT_ATTACK, BT_USE},
	d_player::{wbplayerstruct_t, wbstartstruct_t},
	doomdef::{GameMode_t, MAXPLAYERS, SCREENHEIGHT, SCREENWIDTH, TICRATE},
	doomstat::gamemode,
	g_game::{G_WorldDone, deathmatch, netgame, playeringame, players},
	m_random::M_Random,
	r_defs::patch_t,
	s_sound::{S_ChangeMusic, S_StartSound},
	sounds::{musicenum_t, sfxenum_t},
	v_video::{V_DrawPatch, V_MarkRect, screens},
	w_wad::W_CacheLumpName,
	z_zone::{PU_CACHE, PU_STATIC, Z_ChangeTag, Z_Free, Z_Malloc},
};

type int = i32;

#[repr(C)]
#[derive(PartialEq, Eq)]
enum stateenum_t {
	NoState = -1,
	StatCount,
	ShowNextLoc,
}

// Data needed to add patches to full screen intermission pics.
// Patches are statistics messages, and animations.
// Loads of by-pixel layout and placement, offsets etc.

// Different vetween registered DOOM (1994) and
//  Ultimate DOOM - Final edition (retail, 1995?).
// This is supposedly ignored for commercial
//  release (aka DOOM II), which had 34 maps
//  in one episode. So there.
const NUMEPISODES: usize = 4;
const NUMMAPS: usize = 9;

// in tics
//U const PAUSELEN: usize = (TICRATE*2);
//U const SCORESTEP: usize = 100;
//U const ANIMPERIOD: usize = 32;
// pixel distance from "(YOU)" to "PLAYER N"
//U const STARDIST: usize = 10;
//U const WK: usize = 1;

// GLOBAL LOCATIONS
const WI_TITLEY: usize = 2;
const WI_SPACINGY: usize = 33;

// SINGPLE-PLAYER STUFF
const SP_STATSX: usize = 50;
const SP_STATSY: usize = 50;

const SP_TIMEX: usize = 16;
const SP_TIMEY: usize = SCREENHEIGHT - 32;

// NET GAME STUFF
const NG_STATSY: usize = 50;
fn NG_STATSX() -> usize {
	unsafe { 32 + (*star).width as usize / 2 + 32 * if dofrags == 0 { 1 } else { 0 } }
}

const NG_SPACINGX: usize = 64;

// DEATHMATCH STUFF
const DM_MATRIXX: usize = 42;
const DM_MATRIXY: usize = 68;

const DM_SPACINGX: usize = 40;

const DM_TOTALSX: usize = 269;

const DM_KILLERSX: usize = 10;
const DM_KILLERSY: usize = 100;
const DM_VICTIMSX: usize = 5;
const DM_VICTIMSY: usize = 50;

#[expect(unused)]
enum animenum_t {
	ANIM_ALWAYS,
	ANIM_RANDOM,
	ANIM_LEVEL,
}

struct point_t {
	x: usize,
	y: usize,
}

// Animation.
// There is another anim_t used in p_spec.
#[expect(unused)]
struct anim_t {
	pub ty: animenum_t,

	// period in tics between animations
	pub period: usize,

	// number of animation frames
	pub nanims: usize,

	// location of animation
	pub loc: point_t,

	// ALWAYS: n/a,
	// RANDOM: period deviation (<256),
	// LEVEL: level
	pub data1: usize,

	// ALWAYS: n/a,
	// RANDOM: random base period,
	// LEVEL: n/a
	pub data2: usize,

	// actual graphics for frames of animations
	pub p: [*mut patch_t; 3],

	// following must be initialized to zero before use!

	// next value of bcnt (used in conjunction with period)
	pub nexttic: usize,

	// last drawn animation frame
	pub lastdrawn: int,

	// next frame number to animate
	pub ctr: int,

	// used by RANDOM and LEVEL when animating
	pub state: int,
}

unsafe impl Sync for anim_t {}

static lnodes: [[point_t; NUMMAPS]; NUMEPISODES - 1] = [
	// Episode 0 World Map
	[
		point_t { x: 185, y: 164 }, // location of level 0 (CJ)
		point_t { x: 148, y: 143 }, // location of level 1 (CJ)
		point_t { x: 69, y: 122 },  // location of level 2 (CJ)
		point_t { x: 209, y: 102 }, // location of level 3 (CJ)
		point_t { x: 116, y: 89 },  // location of level 4 (CJ)
		point_t { x: 166, y: 55 },  // location of level 5 (CJ)
		point_t { x: 71, y: 56 },   // location of level 6 (CJ)
		point_t { x: 135, y: 29 },  // location of level 7 (CJ)
		point_t { x: 71, y: 24 },   // location of level 8 (CJ)
	],
	// Episode 1 World Map should go here
	[
		point_t { x: 254, y: 25 },  // location of level 0 (CJ)
		point_t { x: 97, y: 50 },   // location of level 1 (CJ)
		point_t { x: 188, y: 64 },  // location of level 2 (CJ)
		point_t { x: 128, y: 78 },  // location of level 3 (CJ)
		point_t { x: 214, y: 92 },  // location of level 4 (CJ)
		point_t { x: 133, y: 130 }, // location of level 5 (CJ)
		point_t { x: 208, y: 136 }, // location of level 6 (CJ)
		point_t { x: 148, y: 140 }, // location of level 7 (CJ)
		point_t { x: 235, y: 158 }, // location of level 8 (CJ)
	],
	// Episode 2 World Map should go here
	[
		point_t { x: 156, y: 168 }, // location of level 0 (CJ)
		point_t { x: 48, y: 154 },  // location of level 1 (CJ)
		point_t { x: 174, y: 95 },  // location of level 2 (CJ)
		point_t { x: 265, y: 75 },  // location of level 3 (CJ)
		point_t { x: 130, y: 48 },  // location of level 4 (CJ)
		point_t { x: 279, y: 23 },  // location of level 5 (CJ)
		point_t { x: 198, y: 48 },  // location of level 6 (CJ)
		point_t { x: 140, y: 25 },  // location of level 7 (CJ)
		point_t { x: 281, y: 136 }, // location of level 8 (CJ)
	],
];

// Animation locations for episode 0 (1).
// Using patches saves a lot of space,
//  as they replace 320x200 full screen frames.
static mut epsd0animinfo: [anim_t; 10] = [
	anim_t {
		ty: animenum_t::ANIM_ALWAYS,
		period: TICRATE / 3,
		nanims: 3,
		loc: point_t { x: 224, y: 104 },
		data1: 0,
		data2: 0,
		p: [null_mut(); 3],
		nexttic: 0,
		lastdrawn: 0,
		ctr: 0,
		state: 0,
	},
	anim_t {
		ty: animenum_t::ANIM_ALWAYS,
		period: TICRATE / 3,
		nanims: 3,
		loc: point_t { x: 184, y: 160 },
		data1: 0,
		data2: 0,
		p: [null_mut(); 3],
		nexttic: 0,
		lastdrawn: 0,
		ctr: 0,
		state: 0,
	},
	anim_t {
		ty: animenum_t::ANIM_ALWAYS,
		period: TICRATE / 3,
		nanims: 3,
		loc: point_t { x: 112, y: 136 },
		data1: 0,
		data2: 0,
		p: [null_mut(); 3],
		nexttic: 0,
		lastdrawn: 0,
		ctr: 0,
		state: 0,
	},
	anim_t {
		ty: animenum_t::ANIM_ALWAYS,
		period: TICRATE / 3,
		nanims: 3,
		loc: point_t { x: 72, y: 112 },
		data1: 0,
		data2: 0,
		p: [null_mut(); 3],
		nexttic: 0,
		lastdrawn: 0,
		ctr: 0,
		state: 0,
	},
	anim_t {
		ty: animenum_t::ANIM_ALWAYS,
		period: TICRATE / 3,
		nanims: 3,
		loc: point_t { x: 88, y: 96 },
		data1: 0,
		data2: 0,
		p: [null_mut(); 3],
		nexttic: 0,
		lastdrawn: 0,
		ctr: 0,
		state: 0,
	},
	anim_t {
		ty: animenum_t::ANIM_ALWAYS,
		period: TICRATE / 3,
		nanims: 3,
		loc: point_t { x: 64, y: 48 },
		data1: 0,
		data2: 0,
		p: [null_mut(); 3],
		nexttic: 0,
		lastdrawn: 0,
		ctr: 0,
		state: 0,
	},
	anim_t {
		ty: animenum_t::ANIM_ALWAYS,
		period: TICRATE / 3,
		nanims: 3,
		loc: point_t { x: 192, y: 40 },
		data1: 0,
		data2: 0,
		p: [null_mut(); 3],
		nexttic: 0,
		lastdrawn: 0,
		ctr: 0,
		state: 0,
	},
	anim_t {
		ty: animenum_t::ANIM_ALWAYS,
		period: TICRATE / 3,
		nanims: 3,
		loc: point_t { x: 136, y: 16 },
		data1: 0,
		data2: 0,
		p: [null_mut(); 3],
		nexttic: 0,
		lastdrawn: 0,
		ctr: 0,
		state: 0,
	},
	anim_t {
		ty: animenum_t::ANIM_ALWAYS,
		period: TICRATE / 3,
		nanims: 3,
		loc: point_t { x: 80, y: 16 },
		data1: 0,
		data2: 0,
		p: [null_mut(); 3],
		nexttic: 0,
		lastdrawn: 0,
		ctr: 0,
		state: 0,
	},
	anim_t {
		ty: animenum_t::ANIM_ALWAYS,
		period: TICRATE / 3,
		nanims: 3,
		loc: point_t { x: 64, y: 24 },
		data1: 0,
		data2: 0,
		p: [null_mut(); 3],
		nexttic: 0,
		lastdrawn: 0,
		ctr: 0,
		state: 0,
	},
];

static mut epsd1animinfo: [anim_t; 9] = [
	anim_t {
		ty: animenum_t::ANIM_LEVEL,
		period: TICRATE / 3,
		nanims: 1,
		loc: point_t { x: 128, y: 136 },
		data1: 1,
		data2: 0,
		p: [null_mut(); 3],
		nexttic: 0,
		lastdrawn: 0,
		ctr: 0,
		state: 0,
	},
	anim_t {
		ty: animenum_t::ANIM_LEVEL,
		period: TICRATE / 3,
		nanims: 1,
		loc: point_t { x: 128, y: 136 },
		data1: 2,
		data2: 0,
		p: [null_mut(); 3],
		nexttic: 0,
		lastdrawn: 0,
		ctr: 0,
		state: 0,
	},
	anim_t {
		ty: animenum_t::ANIM_LEVEL,
		period: TICRATE / 3,
		nanims: 1,
		loc: point_t { x: 128, y: 136 },
		data1: 3,
		data2: 0,
		p: [null_mut(); 3],
		nexttic: 0,
		lastdrawn: 0,
		ctr: 0,
		state: 0,
	},
	anim_t {
		ty: animenum_t::ANIM_LEVEL,
		period: TICRATE / 3,
		nanims: 1,
		loc: point_t { x: 128, y: 136 },
		data1: 4,
		data2: 0,
		p: [null_mut(); 3],
		nexttic: 0,
		lastdrawn: 0,
		ctr: 0,
		state: 0,
	},
	anim_t {
		ty: animenum_t::ANIM_LEVEL,
		period: TICRATE / 3,
		nanims: 1,
		loc: point_t { x: 128, y: 136 },
		data1: 5,
		data2: 0,
		p: [null_mut(); 3],
		nexttic: 0,
		lastdrawn: 0,
		ctr: 0,
		state: 0,
	},
	anim_t {
		ty: animenum_t::ANIM_LEVEL,
		period: TICRATE / 3,
		nanims: 1,
		loc: point_t { x: 128, y: 136 },
		data1: 6,
		data2: 0,
		p: [null_mut(); 3],
		nexttic: 0,
		lastdrawn: 0,
		ctr: 0,
		state: 0,
	},
	anim_t {
		ty: animenum_t::ANIM_LEVEL,
		period: TICRATE / 3,
		nanims: 1,
		loc: point_t { x: 128, y: 136 },
		data1: 7,
		data2: 0,
		p: [null_mut(); 3],
		nexttic: 0,
		lastdrawn: 0,
		ctr: 0,
		state: 0,
	},
	anim_t {
		ty: animenum_t::ANIM_LEVEL,
		period: TICRATE / 3,
		nanims: 3,
		loc: point_t { x: 192, y: 144 },
		data1: 8,
		data2: 0,
		p: [null_mut(); 3],
		nexttic: 0,
		lastdrawn: 0,
		ctr: 0,
		state: 0,
	},
	anim_t {
		ty: animenum_t::ANIM_LEVEL,
		period: TICRATE / 3,
		nanims: 1,
		loc: point_t { x: 128, y: 136 },
		data1: 8,
		data2: 0,
		p: [null_mut(); 3],
		nexttic: 0,
		lastdrawn: 0,
		ctr: 0,
		state: 0,
	},
];

static mut epsd2animinfo: [anim_t; 6] = [
	anim_t {
		ty: animenum_t::ANIM_ALWAYS,
		period: TICRATE / 3,
		nanims: 3,
		loc: point_t { x: 104, y: 168 },
		data1: 0,
		data2: 0,
		p: [null_mut(); 3],
		nexttic: 0,
		lastdrawn: 0,
		ctr: 0,
		state: 0,
	},
	anim_t {
		ty: animenum_t::ANIM_ALWAYS,
		period: TICRATE / 3,
		nanims: 3,
		loc: point_t { x: 40, y: 136 },
		data1: 0,
		data2: 0,
		p: [null_mut(); 3],
		nexttic: 0,
		lastdrawn: 0,
		ctr: 0,
		state: 0,
	},
	anim_t {
		ty: animenum_t::ANIM_ALWAYS,
		period: TICRATE / 3,
		nanims: 3,
		loc: point_t { x: 160, y: 96 },
		data1: 0,
		data2: 0,
		p: [null_mut(); 3],
		nexttic: 0,
		lastdrawn: 0,
		ctr: 0,
		state: 0,
	},
	anim_t {
		ty: animenum_t::ANIM_ALWAYS,
		period: TICRATE / 3,
		nanims: 3,
		loc: point_t { x: 104, y: 80 },
		data1: 0,
		data2: 0,
		p: [null_mut(); 3],
		nexttic: 0,
		lastdrawn: 0,
		ctr: 0,
		state: 0,
	},
	anim_t {
		ty: animenum_t::ANIM_ALWAYS,
		period: TICRATE / 3,
		nanims: 3,
		loc: point_t { x: 120, y: 32 },
		data1: 0,
		data2: 0,
		p: [null_mut(); 3],
		nexttic: 0,
		lastdrawn: 0,
		ctr: 0,
		state: 0,
	},
	anim_t {
		ty: animenum_t::ANIM_ALWAYS,
		period: TICRATE / 4,
		nanims: 3,
		loc: point_t { x: 40, y: 0 },
		data1: 0,
		data2: 0,
		p: [null_mut(); 3],
		nexttic: 0,
		lastdrawn: 0,
		ctr: 0,
		state: 0,
	},
];

#[allow(static_mut_refs)]
static NUMANIMS: [usize; NUMEPISODES - 1] = unsafe {
	[
		size_of_val(&epsd0animinfo) / size_of::<anim_t>(),
		size_of_val(&epsd1animinfo) / size_of::<anim_t>(),
		size_of_val(&epsd2animinfo) / size_of::<anim_t>(),
	]
};

#[allow(static_mut_refs)]
static mut anims: [*mut anim_t; NUMEPISODES - 1] =
	unsafe { [epsd0animinfo.as_mut_ptr(), epsd1animinfo.as_mut_ptr(), epsd2animinfo.as_mut_ptr()] };

// GENERAL DATA

// Locally used stuff.
const FB: usize = 0;

// States for single-player
// const SP_KILLS: usize = 0;
// const SP_ITEMS: usize = 2;
// const SP_SECRET: usize = 4;
// const SP_FRAGS: usize = 6;
// const SP_TIME: usize = 8;
//
// const SP_PAUSE: usize = 1;

// in seconds
const SHOWNEXTLOCDELAY: usize = 4;
//const SHOWLASTLOCDELAY: usize = SHOWNEXTLOCDELAY;

// used to accelerate or skip a stage
static mut acceleratestage: int = 0;

// wbs->pnum
static mut me: usize = 0;

// specifies current state
static mut state: stateenum_t = stateenum_t::NoState;

// contains information passed into intermission
static mut wbs: *mut wbstartstruct_t = null_mut();

static mut plrs: *mut wbplayerstruct_t = null_mut(); // wbs->plyr[]

// used for general timing
static mut cnt: usize = 0;

// used for timing of background animation
static mut bcnt: usize = 0;

// signals to refresh everything for one frame
static mut firstrefresh: int = 0;

static mut cnt_kills: [i32; MAXPLAYERS] = [0; MAXPLAYERS];
static mut cnt_items: [i32; MAXPLAYERS] = [0; MAXPLAYERS];
static mut cnt_secret: [i32; MAXPLAYERS] = [0; MAXPLAYERS];
static mut cnt_time: i32 = 0;
static mut cnt_par: i32 = 0;
static mut cnt_pause: usize = 0;

// # of commercial levels
static mut NUMCMAPS: usize = 0;

//	GRAPHICS

// background (map of levels).
static mut bg: *mut patch_t = null_mut();

// You Are Here graphic
static mut yah: [*mut patch_t; 2] = [null_mut(); 2];

// splat
static mut splat: *mut patch_t = null_mut();

// %, : graphics
static mut percent: *mut patch_t = null_mut();
static mut colon: *mut patch_t = null_mut();

// 0-9 graphic
static mut num: [*mut patch_t; 10] = [null_mut(); 10];

// minus sign
static mut wiminus: *mut patch_t = null_mut();

// "Finished!" graphics
static mut finished: *mut patch_t = null_mut();

// "Entering" graphic
static mut entering: *mut patch_t = null_mut();

// "secret"
static mut sp_secret: *mut patch_t = null_mut();

// "Kills", "Scrt", "Items", "Frags"
static mut kills: *mut patch_t = null_mut();
static mut secret: *mut patch_t = null_mut();
static mut items: *mut patch_t = null_mut();
static mut frags: *mut patch_t = null_mut();

// Time sucks.
static mut time: *mut patch_t = null_mut();
static mut par: *mut patch_t = null_mut();
static mut sucks: *mut patch_t = null_mut();

// "killers", "victims"
static mut killers: *mut patch_t = null_mut();
static mut victims: *mut patch_t = null_mut();

// "Total", your face, your dead face
static mut total: *mut patch_t = null_mut();
static mut star: *mut patch_t = null_mut();
static mut bstar: *mut patch_t = null_mut();

// "red P[1..MAXPLAYERS]"
static mut p: [*mut patch_t; MAXPLAYERS] = [null_mut(); MAXPLAYERS];

// "gray P[1..MAXPLAYERS]"
static mut bp: [*mut patch_t; MAXPLAYERS] = [null_mut(); MAXPLAYERS];

// Name graphics of each level (centered)
static mut lnames: *mut *mut patch_t = null_mut();

// CODE

// slam background
// UNUSED static unsigned char *background=0;

fn WI_slamBackground() {
	unsafe {
		libc::memcpy(screens[0].cast(), screens[1].cast(), SCREENWIDTH * SCREENHEIGHT);
	}
	V_MarkRect(0, 0, SCREENWIDTH, SCREENHEIGHT);
}

// Draws "<Levelname> Finished!"
fn WI_drawLF() {
	unsafe {
		let mut y = WI_TITLEY;

		let lname = *lnames.wrapping_add((*wbs).last);

		// draw <LevelName>
		V_DrawPatch((SCREENWIDTH - (*lname).width as usize) / 2, y, FB, lname);

		// draw "Finished!"
		y += (5 * (*lname).height as usize) / 4;

		V_DrawPatch((SCREENWIDTH - (*finished).width as usize) / 2, y, FB, finished);
	}
}

// Draws "Entering <LevelName>"
fn WI_drawEL() {
	unsafe {
		let mut y = WI_TITLEY;

		// draw "Entering"
		V_DrawPatch((SCREENWIDTH - (*entering).width as usize) / 2, y, FB, entering);

		let lname = *lnames.wrapping_add((*wbs).next);

		// draw level
		y += (5 * (*lname).height as usize) / 4;

		V_DrawPatch((SCREENWIDTH - (*lname).width as usize) / 2, y, FB, lname);
	}
}

fn WI_drawOnLnode(n: usize, c: *mut *mut patch_t) {
	unsafe {
		let mut fits = false;

		let c = |x| &**c.wrapping_add(x);

		let mut i = 0;
		loop {
			let left = lnodes[(*wbs).epsd][n].x.wrapping_sub(c(i).leftoffset as usize);
			let top = lnodes[(*wbs).epsd][n].y.wrapping_sub(c(i).topoffset as usize);
			let right = left + (c(i).width as usize);
			let bottom = top + (c(i).height as usize);

			if
			/*left >= 0 &&*/
			right < SCREENWIDTH && /*top >= 0 &&*/ bottom < SCREENHEIGHT {
				fits = true;
			} else {
				i += 1;
			}

			if fits || i == 2 {
				break;
			}
		}

		if fits && i < 2 {
			V_DrawPatch(lnodes[(*wbs).epsd][n].x, lnodes[(*wbs).epsd][n].y, FB, c(i));
		} else {
			// DEBUG
			eprintln!("Could not place patch on level {}", n + 1);
		}
	}
}

fn WI_initAnimatedBack() {
	unsafe {
		if gamemode == GameMode_t::commercial {
			return;
		}

		if (*wbs).epsd > 2 {
			return;
		}

		for i in 0..NUMANIMS[(*wbs).epsd] {
			let a = &mut *anims[(*wbs).epsd].wrapping_add(i);

			// init variables
			a.ctr = -1;

			// specify the next time to draw it
			match a.ty {
				animenum_t::ANIM_ALWAYS => a.nexttic = bcnt + 1 + (M_Random() as usize % a.period),
				animenum_t::ANIM_RANDOM => {
					a.nexttic = bcnt + 1 + a.data2 + (M_Random() as usize % a.data1)
				}
				animenum_t::ANIM_LEVEL => a.nexttic = bcnt + 1,
			}
		}
	}
}

fn WI_updateAnimatedBack() {
	unsafe {
		if gamemode == GameMode_t::commercial {
			return;
		}

		if (*wbs).epsd > 2 {
			return;
		}

		for i in 0..NUMANIMS[(*wbs).epsd] {
			let a = &mut *anims[(*wbs).epsd].wrapping_add(i);

			if bcnt == a.nexttic {
				match a.ty {
					animenum_t::ANIM_ALWAYS => {
						a.ctr += 1;
						if a.ctr >= a.nanims as i32 {
							a.ctr = 0;
						}
						a.nexttic = bcnt + a.period;
					}

					animenum_t::ANIM_RANDOM => {
						a.ctr += 1;
						if a.ctr == a.nanims as i32 {
							a.ctr = -1;
							a.nexttic = bcnt + a.data2 + (M_Random() as usize % a.data1);
						} else {
							a.nexttic = bcnt + a.period;
						}
					}
					animenum_t::ANIM_LEVEL => {
						// gawd-awful hack for level anims
						if !(state == stateenum_t::StatCount && i == 7) && (*wbs).next == a.data1 {
							a.ctr += 1;
							if a.ctr == a.nanims as i32 {
								a.ctr -= 1;
							}
							a.nexttic = bcnt + a.period;
						}
					}
				}
			}
		}
	}
}

fn WI_drawAnimatedBack() {
	unsafe {
		if gamemode == GameMode_t::commercial {
			return;
		}

		if (*wbs).epsd > 2 {
			return;
		}

		for i in 0..NUMANIMS[(*wbs).epsd] {
			let a = &mut *anims[(*wbs).epsd].wrapping_add(i);

			if a.ctr >= 0 {
				V_DrawPatch(a.loc.x, a.loc.y, FB, a.p[a.ctr as usize]);
			}
		}
	}
}

// Draws a number.
// If digits > 0, then use that many digits minimum,
//  otherwise only use as many as necessary.
// Returns new x position.
fn WI_drawNum(mut x: usize, y: usize, mut n: i32, mut digits: i32) -> usize {
	unsafe {
		if digits < 0 {
			if n == 0 {
				// make variable-length zeros 1 digit long
				digits = 1;
			} else {
				// figure out # of digits in #
				digits = 0;
				let mut temp = n;

				while temp != 0 {
					temp /= 10;
					digits += 1;
				}
			}
		}

		let neg = n < 0;
		if neg {
			n = -n;
		}

		// if non-number, do not draw it
		if n == 1994 {
			return 0;
		}

		let fontwidth = (*num[0]).width;

		// draw the new number
		while digits > 0 {
			digits -= 1;
			x -= fontwidth as usize;
			V_DrawPatch(x, y, FB, num[n as usize % 10]);
			n /= 10;
		}

		// draw a minus sign if necessary
		if neg {
			x -= 8;
			V_DrawPatch(x, y, FB, wiminus);
		}

		x
	}
}

fn WI_drawPercent(x: usize, y: usize, pc: i32) {
	if pc < 0 {
		return;
	}

	unsafe { V_DrawPatch(x, y, FB, percent) };
	WI_drawNum(x, y, pc, -1);
}

// Display level completion time and par,
//  or "sucks" message if overflow.
fn WI_drawTime(mut x: usize, y: usize, t: i32) {
	unsafe {
		if t < 0 {
			return;
		}

		if t <= 61 * 59 {
			let mut div = 1;

			loop {
				let n = (t / div) % 60;
				x = WI_drawNum(x, y, n, 2) - (*colon).width as usize;
				div *= 60;

				// draw
				if div == 60 || t / div != 0 {
					V_DrawPatch(x, y, FB, colon);
				}

				if t / div == 0 {
					break;
				}
			}
		} else {
			// "sucks"
			V_DrawPatch(x - (*sucks).width as usize, y, FB, sucks);
		}
	}
}

fn WI_End() {
	WI_unloadData();
}

fn WI_initNoState() {
	unsafe {
		state = stateenum_t::NoState;
		acceleratestage = 0;
		cnt = 10;
	}
}

fn WI_updateNoState() {
	unsafe {
		WI_updateAnimatedBack();

		cnt -= 1;
		if cnt == 0 {
			WI_End();
			G_WorldDone();
		}
	}
}

static mut snl_pointeron: bool = false;

fn WI_initShowNextLoc() {
	unsafe {
		state = stateenum_t::ShowNextLoc;
		acceleratestage = 0;
		cnt = SHOWNEXTLOCDELAY * TICRATE;

		WI_initAnimatedBack();
	}
}

fn WI_updateShowNextLoc() {
	unsafe {
		WI_updateAnimatedBack();

		cnt -= 1;
		if cnt == 0 || acceleratestage != 0 {
			WI_initNoState();
		} else {
			snl_pointeron = (cnt & 31) < 20;
		}
	}
}

#[allow(static_mut_refs)]
fn WI_drawShowNextLoc() {
	unsafe {
		WI_slamBackground();

		// draw animated background
		WI_drawAnimatedBack();

		if gamemode != GameMode_t::commercial {
			if (*wbs).epsd > 2 {
				WI_drawEL();
				return;
			}

			let last = if (*wbs).last == 8 { (*wbs).next - 1 } else { (*wbs).last };

			// draw a splat on taken cities.
			for i in 0..=last {
				WI_drawOnLnode(i, &raw mut splat);
			}

			// splat the secret level?
			if (*wbs).didsecret != 0 {
				WI_drawOnLnode(8, &raw mut splat);
			}

			// draw flashing ptr
			if snl_pointeron {
				WI_drawOnLnode((*wbs).next, yah.as_mut_ptr());
			}
		}

		// draws which level you are entering..
		if gamemode != GameMode_t::commercial || (*wbs).next != 30 {
			WI_drawEL();
		}
	}
}

fn WI_drawNoState() {
	unsafe {
		snl_pointeron = true;
		WI_drawShowNextLoc();
	}
}

fn WI_fragSum(playernum: usize) -> i32 {
	unsafe {
		let mut fragsum = 0;
		#[allow(clippy::needless_range_loop)]
		for i in 0..MAXPLAYERS {
			if playeringame[i] != 0 && i != playernum {
				fragsum += (*plrs.wrapping_add(playernum)).frags[i];
			}
		}

		// JDC hack - negative frags.
		fragsum -= (*plrs.wrapping_add(playernum)).frags[playernum];
		// UNUSED if (frags < 0)
		//	 frags = 0;

		fragsum
	}
}

static mut dm_state: int = 0;
static mut dm_frags: [[int; MAXPLAYERS]; MAXPLAYERS] = [[0; MAXPLAYERS]; MAXPLAYERS];
static mut dm_totals: [int; MAXPLAYERS] = [0; MAXPLAYERS];

fn WI_initDeathmatchStats() {
	unsafe {
		state = stateenum_t::StatCount;
		acceleratestage = 0;
		dm_state = 1;

		cnt_pause = TICRATE;

		for i in 0..MAXPLAYERS {
			if playeringame[i] != 0 {
				#[allow(clippy::needless_range_loop)]
				for j in 0..MAXPLAYERS {
					if playeringame[j] != 0 {
						dm_frags[i][j] = 0;
					}
				}

				dm_totals[i] = 0;
			}
		}

		WI_initAnimatedBack();
	}
}

fn WI_updateDeathmatchStats() {
	unsafe {
		WI_updateAnimatedBack();

		if acceleratestage != 0 && dm_state != 4 {
			acceleratestage = 0;

			for i in 0..MAXPLAYERS {
				if playeringame[i] != 0 {
					#[allow(clippy::needless_range_loop)]
					for j in 0..MAXPLAYERS {
						if playeringame[j] != 0 {
							dm_frags[i][j] = (*plrs.wrapping_add(i)).frags[j];
						}
					}

					dm_totals[i] = WI_fragSum(i);
				}
			}

			S_StartSound(null_mut(), sfxenum_t::sfx_barexp);
			dm_state = 4;
		}

		if dm_state == 2 {
			if bcnt & 3 == 0 {
				S_StartSound(null_mut(), sfxenum_t::sfx_pistol);
			}

			let mut stillticking = false;

			#[allow(clippy::needless_range_loop)]
			for i in 0..MAXPLAYERS {
				if playeringame[i] != 0 {
					#[allow(clippy::needless_range_loop)]
					for j in 0..MAXPLAYERS {
						if playeringame[j] != 0
							&& dm_frags[i][j] != (*plrs.wrapping_add(i)).frags[j]
						{
							if (*plrs.wrapping_add(i)).frags[j] < 0 {
								dm_frags[i][j] -= 1;
							} else {
								dm_frags[i][j] += 1;
							}
							dm_frags[i][j] = dm_frags[i][j].clamp(-99, 99);
							stillticking = true;
						}
					}
					dm_totals[i] = WI_fragSum(i);

					dm_totals[i] = dm_totals[i].clamp(-99, 99);
				}
			}
			if !stillticking {
				S_StartSound(null_mut(), sfxenum_t::sfx_barexp);
				dm_state += 1;
			}
		} else if dm_state == 4 {
			if acceleratestage != 0 {
				S_StartSound(null_mut(), sfxenum_t::sfx_slop);

				if gamemode == GameMode_t::commercial {
					WI_initNoState();
				} else {
					WI_initShowNextLoc();
				}
			}
		} else if dm_state & 1 != 0 {
			cnt_pause -= 1;
			if cnt_pause == 0 {
				dm_state += 1;
				cnt_pause = TICRATE;
			}
		}
	}
}

fn WI_drawDeathmatchStats() {
	unsafe {
		WI_slamBackground();

		// draw animated background
		WI_drawAnimatedBack();
		WI_drawLF();

		// draw stat titles (top line)
		V_DrawPatch(
			DM_TOTALSX - (*total).width as usize / 2,
			DM_MATRIXY - WI_SPACINGY + 10,
			FB,
			total,
		);

		V_DrawPatch(DM_KILLERSX, DM_KILLERSY, FB, killers);
		V_DrawPatch(DM_VICTIMSX, DM_VICTIMSY, FB, victims);

		// draw P?
		let mut x = DM_MATRIXX + DM_SPACINGX;
		let mut y = DM_MATRIXY;

		for i in 0..MAXPLAYERS {
			if playeringame[i] != 0 {
				V_DrawPatch(x - (*p[i]).width as usize / 2, DM_MATRIXY - WI_SPACINGY, FB, p[i]);

				V_DrawPatch(DM_MATRIXX - (*p[i]).width as usize / 2, y, FB, p[i]);

				if i == me {
					V_DrawPatch(
						x - (*p[i]).width as usize / 2,
						DM_MATRIXY - WI_SPACINGY,
						FB,
						bstar,
					);

					V_DrawPatch(DM_MATRIXX - (*p[i]).width as usize / 2, y, FB, star);
				}
			} else {
				// V_DrawPatch(x-(bp[i].width)/2,
				//   DM_MATRIXY - WI_SPACINGY, FB, bp[i]);
				// V_DrawPatch(DM_MATRIXX-(bp[i].width)/2,
				//   y, FB, bp[i]);
			}
			x += DM_SPACINGX;
			y += WI_SPACINGY;
		}

		// draw stats
		y = DM_MATRIXY + 10;
		let w = (*num[0]).width as usize;

		for i in 0..MAXPLAYERS {
			x = DM_MATRIXX + DM_SPACINGX;

			if playeringame[i] != 0 {
				#[allow(clippy::needless_range_loop)]
				for j in 0..MAXPLAYERS {
					if playeringame[j] != 0 {
						WI_drawNum(x + w, y, dm_frags[i][j], 2);
					}

					x += DM_SPACINGX;
				}
				WI_drawNum(DM_TOTALSX + w, y, dm_totals[i], 2);
			}
			y += WI_SPACINGY;
		}
	}
}

static mut cnt_frags: [int; MAXPLAYERS] = [0; MAXPLAYERS];
static mut dofrags: int = 0;
static mut ng_state: int = 0;

fn WI_initNetgameStats() {
	unsafe {
		state = stateenum_t::StatCount;
		acceleratestage = 0;
		ng_state = 1;

		cnt_pause = TICRATE;

		for i in 0..MAXPLAYERS {
			if playeringame[i] == 0 {
				continue;
			}

			cnt_kills[i] = 0;
			cnt_items[i] = 0;
			cnt_secret[i] = 0;
			cnt_frags[i] = 0;

			dofrags += WI_fragSum(i);
		}

		if dofrags != 0 {
			dofrags = 1;
		}

		WI_initAnimatedBack();
	}
}

fn WI_updateNetgameStats() {
	unsafe {
		let mut stillticking;

		WI_updateAnimatedBack();

		if acceleratestage != 0 && ng_state != 10 {
			acceleratestage = 0;

			for i in 0..MAXPLAYERS {
				if playeringame[i] == 0 {
					continue;
				}

				cnt_kills[i] = ((*plrs.wrapping_add(i)).skills * 100) / (*wbs).maxkills;
				cnt_items[i] = ((*plrs.wrapping_add(i)).sitems * 100) / (*wbs).maxitems;
				cnt_secret[i] = ((*plrs.wrapping_add(i)).ssecret * 100) / (*wbs).maxsecret;

				if dofrags != 0 {
					cnt_frags[i] = WI_fragSum(i);
				}
			}
			S_StartSound(null_mut(), sfxenum_t::sfx_barexp);
			ng_state = 10;
		}

		if ng_state == 2 {
			if bcnt & 3 == 0 {
				S_StartSound(null_mut(), sfxenum_t::sfx_pistol);
			}

			stillticking = false;

			for i in 0..MAXPLAYERS {
				if playeringame[i] == 0 {
					continue;
				}

				cnt_kills[i] += 2;

				if cnt_kills[i] >= ((*plrs.wrapping_add(i)).skills * 100) / (*wbs).maxkills {
					cnt_kills[i] = ((*plrs.wrapping_add(i)).skills * 100) / (*wbs).maxkills;
				} else {
					stillticking = true;
				}
			}

			if !stillticking {
				S_StartSound(null_mut(), sfxenum_t::sfx_barexp);
				ng_state += 1;
			}
		} else if ng_state == 4 {
			if bcnt & 3 == 0 {
				S_StartSound(null_mut(), sfxenum_t::sfx_pistol);
			}

			stillticking = false;

			for i in 0..MAXPLAYERS {
				if playeringame[i] == 0 {
					continue;
				}

				cnt_items[i] += 2;
				if cnt_items[i] >= ((*plrs.wrapping_add(i)).sitems * 100) / (*wbs).maxitems {
					cnt_items[i] = ((*plrs.wrapping_add(i)).sitems * 100) / (*wbs).maxitems;
				} else {
					stillticking = true;
				}
			}
			if !stillticking {
				S_StartSound(null_mut(), sfxenum_t::sfx_barexp);
				ng_state += 1;
			}
		} else if ng_state == 6 {
			if bcnt & 3 == 0 {
				S_StartSound(null_mut(), sfxenum_t::sfx_pistol);
			}

			stillticking = false;

			for i in 0..MAXPLAYERS {
				if playeringame[i] == 0 {
					continue;
				}

				cnt_secret[i] += 2;

				if cnt_secret[i] >= ((*plrs.wrapping_add(i)).ssecret * 100) / (*wbs).maxsecret {
					cnt_secret[i] = ((*plrs.wrapping_add(i)).ssecret * 100) / (*wbs).maxsecret;
				} else {
					stillticking = true;
				}
			}

			if !stillticking {
				S_StartSound(null_mut(), sfxenum_t::sfx_barexp);
				ng_state += 1 + 2 * !dofrags;
			}
		} else if ng_state == 8 {
			if bcnt & 3 == 0 {
				S_StartSound(null_mut(), sfxenum_t::sfx_pistol);
			}

			stillticking = false;

			for i in 0..MAXPLAYERS {
				if playeringame[i] == 0 {
					continue;
				}

				cnt_frags[i] += 1;
				let fsum = WI_fragSum(i);
				if cnt_frags[i] >= (fsum) {
					cnt_frags[i] = fsum;
				} else {
					stillticking = true;
				}
			}

			if !stillticking {
				S_StartSound(null_mut(), sfxenum_t::sfx_pldeth);
				ng_state += 1;
			}
		} else if ng_state == 10 {
			if acceleratestage != 0 {
				S_StartSound(null_mut(), sfxenum_t::sfx_sgcock);
				if gamemode == GameMode_t::commercial {
					WI_initNoState();
				} else {
					WI_initShowNextLoc();
				}
			}
		} else if ng_state & 1 != 0 {
			cnt_pause -= 1;
			if cnt_pause == 0 {
				ng_state += 1;
				cnt_pause = TICRATE;
			}
		}
	}
}

fn WI_drawNetgameStats() {
	unsafe {
		let pwidth = (*percent).width as usize;

		WI_slamBackground();

		// draw animated background
		WI_drawAnimatedBack();

		WI_drawLF();

		// draw stat titles (top line)
		V_DrawPatch(NG_STATSX() + NG_SPACINGX - (*kills).width as usize, NG_STATSY, FB, kills);
		V_DrawPatch(NG_STATSX() + 2 * NG_SPACINGX - (*items).width as usize, NG_STATSY, FB, items);
		V_DrawPatch(
			NG_STATSX() + 3 * NG_SPACINGX - (*secret).width as usize,
			NG_STATSY,
			FB,
			secret,
		);

		if dofrags != 0 {
			V_DrawPatch(
				NG_STATSX() + 4 * NG_SPACINGX - (*frags).width as usize,
				NG_STATSY,
				FB,
				frags,
			);
		}

		// draw stats
		let mut y = NG_STATSY + (*kills).height as usize;

		for i in 0..MAXPLAYERS {
			if playeringame[i] == 0 {
				continue;
			}

			let mut x = NG_STATSX();
			V_DrawPatch(x - (*p[i]).width as usize, y, FB, p[i]);

			if i == me {
				V_DrawPatch(x - (*p[i]).width as usize, y, FB, star);
			}

			x += NG_SPACINGX;
			WI_drawPercent(x - pwidth, y + 10, cnt_kills[i]);
			x += NG_SPACINGX;
			WI_drawPercent(x - pwidth, y + 10, cnt_items[i]);
			x += NG_SPACINGX;
			WI_drawPercent(x - pwidth, y + 10, cnt_secret[i]);
			x += NG_SPACINGX;

			if dofrags != 0 {
				WI_drawNum(x, y + 10, cnt_frags[i], -1);
			}

			y += WI_SPACINGY;
		}
	}
}

static mut sp_state: int = 0;

fn WI_initStats() {
	unsafe {
		state = stateenum_t::StatCount;
		acceleratestage = 0;
		sp_state = 1;
		cnt_kills[0] = -1;
		cnt_items[0] = -1;
		cnt_secret[0] = -1;
		cnt_time = -1;
		cnt_par = -1;
		cnt_pause = TICRATE;

		WI_initAnimatedBack();
	}
}

fn WI_updateStats() {
	unsafe {
		WI_updateAnimatedBack();

		if acceleratestage != 0 && sp_state != 10 {
			acceleratestage = 0;
			cnt_kills[0] = (*plrs.wrapping_add(me)).skills * 100 / (*wbs).maxkills;
			cnt_items[0] = (*plrs.wrapping_add(me)).sitems * 100 / (*wbs).maxitems;
			cnt_secret[0] = (*plrs.wrapping_add(me)).ssecret * 100 / (*wbs).maxsecret;
			cnt_time = ((*plrs.wrapping_add(me)).stime / TICRATE) as i32;
			cnt_par = ((*wbs).partime / TICRATE) as i32;
			S_StartSound(null_mut(), sfxenum_t::sfx_barexp);
			sp_state = 10;
		}

		if sp_state == 2 {
			cnt_kills[0] += 2;

			if bcnt & 3 == 0 {
				S_StartSound(null_mut(), sfxenum_t::sfx_pistol);
			}

			if cnt_kills[0] >= (*plrs.wrapping_add(me)).skills * 100 / (*wbs).maxkills {
				cnt_kills[0] = (*plrs.wrapping_add(me)).skills * 100 / (*wbs).maxkills;
				S_StartSound(null_mut(), sfxenum_t::sfx_barexp);
				sp_state += 1;
			}
		} else if sp_state == 4 {
			cnt_items[0] += 2;

			if bcnt & 3 == 0 {
				S_StartSound(null_mut(), sfxenum_t::sfx_pistol);
			}

			if cnt_items[0] >= (*plrs.wrapping_add(me)).sitems * 100 / (*wbs).maxitems {
				cnt_items[0] = (*plrs.wrapping_add(me)).sitems * 100 / (*wbs).maxitems;
				S_StartSound(null_mut(), sfxenum_t::sfx_barexp);
				sp_state += 1;
			}
		} else if sp_state == 6 {
			cnt_secret[0] += 2;

			if bcnt & 3 == 0 {
				S_StartSound(null_mut(), sfxenum_t::sfx_pistol);
			}

			if cnt_secret[0] >= (*plrs.wrapping_add(me)).ssecret * 100 / (*wbs).maxsecret {
				cnt_secret[0] = (*plrs.wrapping_add(me)).ssecret * 100 / (*wbs).maxsecret;
				S_StartSound(null_mut(), sfxenum_t::sfx_barexp);
				sp_state += 1;
			}
		} else if sp_state == 8 {
			if bcnt & 3 == 0 {
				S_StartSound(null_mut(), sfxenum_t::sfx_pistol);
			}

			cnt_time += 3;

			if cnt_time >= ((*plrs.wrapping_add(me)).stime / TICRATE) as i32 {
				cnt_time = ((*plrs.wrapping_add(me)).stime / TICRATE) as i32;
			}

			cnt_par += 3;

			if cnt_par >= ((*wbs).partime / TICRATE) as i32 {
				cnt_par = ((*wbs).partime / TICRATE) as i32;

				if cnt_time >= ((*plrs.wrapping_add(me)).stime / TICRATE) as i32 {
					S_StartSound(null_mut(), sfxenum_t::sfx_barexp);
					sp_state += 1;
				}
			}
		} else if sp_state == 10 {
			if acceleratestage != 0 {
				S_StartSound(null_mut(), sfxenum_t::sfx_sgcock);

				if gamemode == GameMode_t::commercial {
					WI_initNoState();
				} else {
					WI_initShowNextLoc();
				}
			}
		} else if sp_state & 1 != 0 {
			cnt_pause -= 1;
			if cnt_pause == 0 {
				sp_state += 1;
				cnt_pause = TICRATE;
			}
		}
	}
}

fn WI_drawStats() {
	unsafe {
		// line height
		let lh = 3 * (*num[0]).height as usize / 2;

		WI_slamBackground();

		// draw animated background
		WI_drawAnimatedBack();

		WI_drawLF();

		V_DrawPatch(SP_STATSX, SP_STATSY, FB, kills);
		WI_drawPercent(SCREENWIDTH - SP_STATSX, SP_STATSY, cnt_kills[0]);

		V_DrawPatch(SP_STATSX, SP_STATSY + lh, FB, items);
		WI_drawPercent(SCREENWIDTH - SP_STATSX, SP_STATSY + lh, cnt_items[0]);

		V_DrawPatch(SP_STATSX, SP_STATSY + 2 * lh, FB, sp_secret);
		WI_drawPercent(SCREENWIDTH - SP_STATSX, SP_STATSY + 2 * lh, cnt_secret[0]);

		V_DrawPatch(SP_TIMEX, SP_TIMEY, FB, time);
		WI_drawTime(SCREENWIDTH / 2 - SP_TIMEX, SP_TIMEY, cnt_time);

		if (*wbs).epsd < 3 {
			V_DrawPatch(SCREENWIDTH / 2 + SP_TIMEX, SP_TIMEY, FB, par);
			WI_drawTime(SCREENWIDTH - SP_TIMEX, SP_TIMEY, cnt_par);
		}
	}
}

fn WI_checkForAccelerate() {
	unsafe {
		// check for button presses to skip delays
		// for (i=0, player = players ; i<MAXPLAYERS ; i++, player++)
		for i in 0..MAXPLAYERS {
			if playeringame[i] != 0 {
				if players[i].cmd.buttons & BT_ATTACK != 0 {
					if players[i].attackdown == 0 {
						acceleratestage = 1;
					}
					players[i].attackdown = 1;
				} else {
					players[i].attackdown = 0;
				}
				if players[i].cmd.buttons & BT_USE != 0 {
					if players[i].usedown == 0 {
						acceleratestage = 1;
					}
					players[i].usedown = 1;
				} else {
					players[i].usedown = 0;
				}
			}
		}
	}
}

// Updates stuff each tick
pub(crate) fn WI_Ticker() {
	unsafe {
		// counter for general background animation
		bcnt += 1;

		if bcnt == 1 {
			// intermission music
			if gamemode == GameMode_t::commercial {
				S_ChangeMusic(musicenum_t::mus_dm2int, 1);
			} else {
				S_ChangeMusic(musicenum_t::mus_inter, 1);
			}
		}

		WI_checkForAccelerate();

		match state {
			stateenum_t::StatCount if deathmatch != 0 => WI_updateDeathmatchStats(),
			stateenum_t::StatCount if netgame != 0 => WI_updateNetgameStats(),
			stateenum_t::StatCount => WI_updateStats(),
			stateenum_t::ShowNextLoc => WI_updateShowNextLoc(),
			stateenum_t::NoState => WI_updateNoState(),
		}
	}
}

fn WI_loadData() {
	unsafe {
		let mut name = [0; 9];

		if gamemode == GameMode_t::commercial {
			libc::strcpy(name.as_mut_ptr(), c"INTERPIC".as_ptr());
		} else {
			libc::sprintf(name.as_mut_ptr(), c"WIMAP%d".as_ptr(), (*wbs).epsd);
		}

		if gamemode == GameMode_t::retail && (*wbs).epsd == 3 {
			libc::strcpy(name.as_mut_ptr(), c"INTERPIC".as_ptr());
		}

		// background
		bg = W_CacheLumpName(name.as_mut_ptr(), PU_CACHE).cast();
		V_DrawPatch(0, 0, 1, bg);

		// UNUSED unsigned char *pic = screens[1];
		// if (gamemode == commercial)
		// {
		// darken the background image
		// while (pic != screens[1] + SCREENHEIGHT*SCREENWIDTH)
		// {
		//   *pic = colormaps[256*25 + *pic];
		//   pic++;
		// }
		//}

		if gamemode == GameMode_t::commercial {
			NUMCMAPS = 32;
			lnames = Z_Malloc(size_of::<*mut patch_t>() * NUMCMAPS, PU_STATIC, null_mut()).cast();
			for i in 0..NUMCMAPS {
				libc::sprintf(name.as_mut_ptr(), c"CWILV%2.2d".as_ptr(), i);
				*lnames.wrapping_add(i) = W_CacheLumpName(name.as_mut_ptr(), PU_STATIC).cast();
			}
		} else {
			lnames = Z_Malloc(size_of::<*mut patch_t>() * NUMMAPS, PU_STATIC, null_mut()).cast();
			for i in 0..NUMMAPS {
				libc::sprintf(name.as_mut_ptr(), c"WILV%d%d".as_ptr(), (*wbs).epsd, i);
				*lnames.wrapping_add(i) = W_CacheLumpName(name.as_mut_ptr(), PU_STATIC).cast();
			}

			// you are here
			yah[0] = W_CacheLumpName(c"WIURH0".as_ptr(), PU_STATIC).cast();

			// you are here (alt.)
			yah[1] = W_CacheLumpName(c"WIURH1".as_ptr(), PU_STATIC).cast();

			// splat
			splat = W_CacheLumpName(c"WISPLAT".as_ptr(), PU_STATIC).cast();

			if (*wbs).epsd < 3 {
				for j in 0..NUMANIMS[(*wbs).epsd] {
					let a = &mut *anims[(*wbs).epsd].wrapping_add(j);
					for i in 0..a.nanims {
						// MONDO HACK!
						if (*wbs).epsd != 1 || j != 8 {
							// animations
							libc::sprintf(
								name.as_mut_ptr(),
								c"WIA%d%.2d%.2d".as_ptr(),
								(*wbs).epsd,
								j,
								i,
							);
							a.p[i] = W_CacheLumpName(name.as_mut_ptr(), PU_STATIC).cast();
						} else {
							// HACK ALERT!
							a.p[i] = (*anims[1].wrapping_add(4)).p[i];
						}
					}
				}
			}
		}

		// More hacks on minus sign.
		wiminus = W_CacheLumpName(c"WIMINUS".as_ptr(), PU_STATIC).cast();

		#[allow(clippy::needless_range_loop)]
		for i in 0..10 {
			// numbers 0-9
			libc::sprintf(name.as_mut_ptr(), c"WINUM%d".as_ptr(), i);
			num[i] = W_CacheLumpName(name.as_mut_ptr(), PU_STATIC).cast();
		}

		// percent sign
		percent = W_CacheLumpName(c"WIPCNT".as_ptr(), PU_STATIC).cast();

		// c"finished".as_ptr()
		finished = W_CacheLumpName(c"WIF".as_ptr(), PU_STATIC).cast();

		// c"entering".as_ptr()
		entering = W_CacheLumpName(c"WIENTER".as_ptr(), PU_STATIC).cast();

		// c"kills".as_ptr()
		kills = W_CacheLumpName(c"WIOSTK".as_ptr(), PU_STATIC).cast();

		// c"scrt".as_ptr()
		secret = W_CacheLumpName(c"WIOSTS".as_ptr(), PU_STATIC).cast();

		// c"secret".as_ptr()
		sp_secret = W_CacheLumpName(c"WISCRT2".as_ptr(), PU_STATIC).cast();

		// Yuck.
		items = /* if french {
			// c"items".as_ptr()
			if (netgame && !deathmatch) {
				 W_CacheLumpName(c"WIOBJ".as_ptr(), PU_STATIC)
			} else {
				 W_CacheLumpName(c"WIOSTI".as_ptr(), PU_STATIC)
			}
		} else { */
			 W_CacheLumpName(c"WIOSTI".as_ptr(), PU_STATIC).cast()
		/*}*/;

		// c"frgs".as_ptr()
		frags = W_CacheLumpName(c"WIFRGS".as_ptr(), PU_STATIC).cast();

		// c":".as_ptr()
		colon = W_CacheLumpName(c"WICOLON".as_ptr(), PU_STATIC).cast();

		// c"time".as_ptr()
		time = W_CacheLumpName(c"WITIME".as_ptr(), PU_STATIC).cast();

		// c"sucks".as_ptr()
		sucks = W_CacheLumpName(c"WISUCKS".as_ptr(), PU_STATIC).cast();

		// c"par".as_ptr()
		par = W_CacheLumpName(c"WIPAR".as_ptr(), PU_STATIC).cast();

		// c"killers".as_ptr() (vertical)
		killers = W_CacheLumpName(c"WIKILRS".as_ptr(), PU_STATIC).cast();

		// c"victims".as_ptr() (horiz)
		victims = W_CacheLumpName(c"WIVCTMS".as_ptr(), PU_STATIC).cast();

		// c"total".as_ptr()
		total = W_CacheLumpName(c"WIMSTT".as_ptr(), PU_STATIC).cast();

		// your face
		star = W_CacheLumpName(c"STFST01".as_ptr(), PU_STATIC).cast();

		// dead face
		bstar = W_CacheLumpName(c"STFDEAD0".as_ptr(), PU_STATIC).cast();

		for i in 0..MAXPLAYERS {
			// c"1,2,3,4".as_ptr()
			libc::sprintf(name.as_mut_ptr(), c"STPB%d".as_ptr(), i);
			p[i] = W_CacheLumpName(name.as_mut_ptr(), PU_STATIC).cast();

			// c"1,2,3,4".as_ptr()
			libc::sprintf(name.as_mut_ptr(), c"WIBP%d".as_ptr(), i + 1);
			bp[i] = W_CacheLumpName(name.as_mut_ptr(), PU_STATIC).cast();
		}
	}
}

fn WI_unloadData() {
	unsafe {
		Z_ChangeTag!(wiminus, PU_CACHE);

		#[allow(clippy::needless_range_loop)]
		for i in 0..10 {
			Z_ChangeTag!(num[i], PU_CACHE);
		}

		if gamemode == GameMode_t::commercial {
			for i in 0..NUMCMAPS {
				Z_ChangeTag!(*lnames.wrapping_add(i), PU_CACHE);
			}
		} else {
			Z_ChangeTag!(yah[0], PU_CACHE);
			Z_ChangeTag!(yah[1], PU_CACHE);

			Z_ChangeTag!(splat, PU_CACHE);

			for i in 0..NUMMAPS {
				Z_ChangeTag!(*lnames.wrapping_add(i), PU_CACHE);
			}

			if (*wbs).epsd < 3 {
				for j in 0..NUMANIMS[(*wbs).epsd] {
					if (*wbs).epsd != 1 || j != 8 {
						for i in 0..(*anims[(*wbs).epsd].wrapping_add(j)).nanims {
							Z_ChangeTag!((*anims[(*wbs).epsd].wrapping_add(j)).p[i], PU_CACHE);
						}
					}
				}
			}
		}

		Z_Free(lnames.cast());

		Z_ChangeTag!(percent, PU_CACHE);
		Z_ChangeTag!(colon, PU_CACHE);
		Z_ChangeTag!(finished, PU_CACHE);
		Z_ChangeTag!(entering, PU_CACHE);
		Z_ChangeTag!(kills, PU_CACHE);
		Z_ChangeTag!(secret, PU_CACHE);
		Z_ChangeTag!(sp_secret, PU_CACHE);
		Z_ChangeTag!(items, PU_CACHE);
		Z_ChangeTag!(frags, PU_CACHE);
		Z_ChangeTag!(time, PU_CACHE);
		Z_ChangeTag!(sucks, PU_CACHE);
		Z_ChangeTag!(par, PU_CACHE);

		Z_ChangeTag!(victims, PU_CACHE);
		Z_ChangeTag!(killers, PU_CACHE);
		Z_ChangeTag!(total, PU_CACHE);
		//  Z_ChangeTag!(star, PU_CACHE);
		//  Z_ChangeTag!(bstar, PU_CACHE);

		#[allow(clippy::needless_range_loop)]
		for i in 0..MAXPLAYERS {
			Z_ChangeTag!(p[i], PU_CACHE);
		}

		#[allow(clippy::needless_range_loop)]
		for i in 0..MAXPLAYERS {
			Z_ChangeTag!(bp[i], PU_CACHE);
		}
	}
}

pub(crate) fn WI_Drawer() {
	unsafe {
		match state {
			stateenum_t::StatCount if deathmatch != 0 => WI_drawDeathmatchStats(),
			stateenum_t::StatCount if netgame != 0 => WI_drawNetgameStats(),
			stateenum_t::StatCount => WI_drawStats(),
			stateenum_t::ShowNextLoc => WI_drawShowNextLoc(),
			stateenum_t::NoState => WI_drawNoState(),
		}
	}
}

fn WI_initVariables(wbstartstruct: &mut wbstartstruct_t) {
	unsafe {
		wbs = wbstartstruct;

		acceleratestage = 0;
		cnt = 0;
		bcnt = 0;
		firstrefresh = 1;
		me = (*wbs).pnum;
		plrs = (*wbs).plyr.as_mut_ptr();

		if (*wbs).maxkills == 0 {
			(*wbs).maxkills = 1;
		}

		if (*wbs).maxitems == 0 {
			(*wbs).maxitems = 1;
		}

		if (*wbs).maxsecret == 0 {
			(*wbs).maxsecret = 1;
		}

		if gamemode != GameMode_t::retail && (*wbs).epsd > 2 {
			(*wbs).epsd -= 3;
		}
	}
}

pub(crate) fn WI_Start(wbstartstruct: &mut wbstartstruct_t) {
	unsafe {
		WI_initVariables(wbstartstruct);
		WI_loadData();

		if deathmatch != 0 {
			WI_initDeathmatchStats();
		} else if netgame != 0 {
			WI_initNetgameStats();
		} else {
			WI_initStats();
		}
	}
}
