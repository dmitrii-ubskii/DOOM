#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::{ffi::c_char, ptr::null_mut};

use crate::{
	d_englsh::{
		AMSTR_FOLLOWOFF, AMSTR_FOLLOWON, AMSTR_GRIDOFF, AMSTR_GRIDON, AMSTR_MARKEDSPOT,
		AMSTR_MARKSCLEARED,
	},
	d_event::{event_t, evtype_t},
	d_player::player_t,
	doomdata::{ML_DONTDRAW, ML_MAPPED, ML_SECRET},
	doomdef::{
		KEY_DOWNARROW, KEY_LEFTARROW, KEY_RIGHTARROW, KEY_TAB, KEY_UPARROW, MAXPLAYERS,
		SCREENHEIGHT, SCREENWIDTH, powertype_t,
	},
	g_game::{
		consoleplayer, deathmatch, gameepisode, gamemap, netgame, playeringame, players,
		singledemo, viewactive,
	},
	m_cheat::{cheatseq_t, cht_CheckCheat},
	m_fixed::{FRACBITS, FRACUNIT, FixedDiv, FixedMul, fixed_t},
	p_local::{MAPBLOCKUNITS, PLAYERRADIUS},
	p_setup::{bmaporgx, bmaporgy, lines, numlines, numsectors, numvertexes, sectors, vertexes},
	r_defs::patch_t,
	tables::{ANGLETOFINESHIFT, angle_t, finecos, finesine},
	v_video::{V_DrawPatch, V_MarkRect, screens},
	w_wad::W_CacheLumpName,
	z_zone::{PU_CACHE, PU_STATIC, Z_ChangeTag},
};

type int = i32;
type boolean = i32;

// Used by ST StatusBar stuff.
const AM_MSGHEADER: usize = ((b'a' as usize) << 24) + ((b'm' as usize) << 16);
const AM_MSGENTERED: usize = AM_MSGHEADER | ((b'e' as usize) << 8);
const AM_MSGEXITED: usize = AM_MSGHEADER | ((b'x' as usize) << 8);

// For use if I do walls with outsides/insides
const REDS: i32 = 256 - 5 * 16;
const REDRANGE: i32 = 16;
// const BLUES: i32 = 256 - 4 * 16 + 8;
// const BLUERANGE: i32 = 8;
const GREENS: i32 = 7 * 16;
const GREENRANGE: i32 = 16;
const GRAYS: i32 = 6 * 16;
const GRAYSRANGE: i32 = 16;
const BROWNS: i32 = 4 * 16;
// const BROWNRANGE: i32 = 16;
const YELLOWS: i32 = 256 - 32 + 7;
// const YELLOWRANGE: i32 = 1;
const BLACK: i32 = 0;
const WHITE: i32 = 256 - 47;

// Automap colors
const BACKGROUND: i32 = BLACK;
// const YOURCOLORS: i32 = WHITE;
// const YOURRANGE: i32 = 0;
const WALLCOLORS: i32 = REDS;
const WALLRANGE: i32 = REDRANGE;
const TSWALLCOLORS: i32 = GRAYS;
// const TSWALLRANGE: i32 = GRAYSRANGE;
const FDWALLCOLORS: i32 = BROWNS;
// const FDWALLRANGE: i32 = BROWNRANGE;
const CDWALLCOLORS: i32 = YELLOWS;
// const CDWALLRANGE: i32 = YELLOWRANGE;
const THINGCOLORS: i32 = GREENS;
const THINGRANGE: i32 = GREENRANGE;
const SECRETWALLCOLORS: i32 = WALLCOLORS;
// const SECRETWALLRANGE: i32 = WALLRANGE;
const GRIDCOLORS: i32 = GRAYS + GRAYSRANGE / 2;
// const GRIDRANGE: i32 = 0;
const XHAIRCOLORS: i32 = GRAYS;

// drawing stuff
const FB: usize = 0;

const AM_PANDOWNKEY: u8 = KEY_DOWNARROW;
const AM_PANUPKEY: u8 = KEY_UPARROW;
const AM_PANRIGHTKEY: u8 = KEY_RIGHTARROW;
const AM_PANLEFTKEY: u8 = KEY_LEFTARROW;
const AM_ZOOMINKEY: u8 = b'=';
const AM_ZOOMOUTKEY: u8 = b'-';
const AM_STARTKEY: u8 = KEY_TAB;
const AM_ENDKEY: u8 = KEY_TAB;
const AM_GOBIGKEY: u8 = b'0';
const AM_FOLLOWKEY: u8 = b'f';
const AM_GRIDKEY: u8 = b'g';
const AM_MARKKEY: u8 = b'm';
const AM_CLEARMARKKEY: u8 = b'c';

const AM_NUMMARKPOINTS: usize = 10;

// scale on entry
const INITSCALEMTOF: fixed_t = FRACUNIT / 5;
// how much the automap moves window per tic in frame-buffer coordinates
// moves 140 pixels in 1 second
const F_PANINC: fixed_t = 4;
// how much zoom-in per tic
// goes to 2x in 1 second
const M_ZOOMIN: fixed_t = FRACUNIT + FRACUNIT / 50;
// how much zoom-out per tic
// pulls out to 0.5x in 1 second
const M_ZOOMOUT: fixed_t = (FRACUNIT as f64 / 1.02) as fixed_t;

// translates between frame-buffer and map distances
fn FTOM(x: fixed_t) -> fixed_t {
	unsafe { FixedMul(x << 16, scale_ftom) }
}

fn MTOF(x: fixed_t) -> fixed_t {
	unsafe { FixedMul(x, scale_mtof) >> 16 }
}

// translates between frame-buffer and map coordinates
// const CXMTOF: usize = (x)  (f_x + MTOF((x)-m_x));
fn CXMTOF(x: fixed_t) -> fixed_t {
	unsafe { f_x + MTOF(x - m_x) }
}
fn CYMTOF(y: fixed_t) -> fixed_t {
	unsafe { f_y + f_h - MTOF(y - m_y) }
}

// the following is crap
const LINE_NEVERSEE: usize = ML_DONTDRAW;

struct fpoint_t {
	x: i32,
	y: i32,
}

struct fline_t {
	a: fpoint_t,
	b: fpoint_t,
}

#[derive(Clone, Copy)]
struct mpoint_t {
	x: fixed_t,
	y: fixed_t,
}

struct mline_t {
	a: mpoint_t,
	b: mpoint_t,
}

// The vector graphics for the automap.
//  A line drawing of the player pointing right,
//   starting from the middle.
const R_: fixed_t = (8 * PLAYERRADIUS) / 7;
const NUMPLYRLINES: usize = 7;
static player_arrow: [mline_t; NUMPLYRLINES] = [
	mline_t { a: mpoint_t { x: -R_ + R_ / 8, y: 0 }, b: mpoint_t { x: R_, y: 0 } }, //  -----
	mline_t { a: mpoint_t { x: R_, y: 0 }, b: mpoint_t { x: R_ - R_ / 2, y: R_ / 4 } }, //  ----->
	mline_t { a: mpoint_t { x: R_, y: 0 }, b: mpoint_t { x: R_ - R_ / 2, y: -R_ / 4 } },
	mline_t { a: mpoint_t { x: -R_ + R_ / 8, y: 0 }, b: mpoint_t { x: -R_ - R_ / 8, y: R_ / 4 } }, //  >---->
	mline_t { a: mpoint_t { x: -R_ + R_ / 8, y: 0 }, b: mpoint_t { x: -R_ - R_ / 8, y: -R_ / 4 } },
	mline_t {
		a: mpoint_t { x: -R_ + 3 * R_ / 8, y: 0 },
		b: mpoint_t { x: -R_ + R_ / 8, y: R_ / 4 },
	}, //  >>--->
	mline_t {
		a: mpoint_t { x: -R_ + 3 * R_ / 8, y: 0 },
		b: mpoint_t { x: -R_ + R_ / 8, y: -R_ / 4 },
	},
];

const NUMCHEATPLYRLINES: usize = 16;
static cheat_player_arrow: [mline_t; NUMCHEATPLYRLINES] = [
	mline_t { a: mpoint_t { x: -R_ + R_ / 8, y: 0 }, b: mpoint_t { x: R_, y: 0 } }, //  -----
	mline_t { a: mpoint_t { x: R_, y: 0 }, b: mpoint_t { x: R_ - R_ / 2, y: R_ / 6 } }, //  ----->
	mline_t { a: mpoint_t { x: R_, y: 0 }, b: mpoint_t { x: R_ - R_ / 2, y: -R_ / 6 } },
	mline_t { a: mpoint_t { x: -R_ + R_ / 8, y: 0 }, b: mpoint_t { x: -R_ - R_ / 8, y: R_ / 6 } }, //  >----->
	mline_t { a: mpoint_t { x: -R_ + R_ / 8, y: 0 }, b: mpoint_t { x: -R_ - R_ / 8, y: -R_ / 6 } },
	mline_t {
		a: mpoint_t { x: -R_ + 3 * R_ / 8, y: 0 },
		b: mpoint_t { x: -R_ + R_ / 8, y: R_ / 6 },
	}, //  >>----->
	mline_t {
		a: mpoint_t { x: -R_ + 3 * R_ / 8, y: 0 },
		b: mpoint_t { x: -R_ + R_ / 8, y: -R_ / 6 },
	},
	mline_t { a: mpoint_t { x: -R_ / 2, y: 0 }, b: mpoint_t { x: -R_ / 2, y: -R_ / 6 } }, //  >>-d--->
	mline_t {
		a: mpoint_t { x: -R_ / 2, y: -R_ / 6 },
		b: mpoint_t { x: -R_ / 2 + R_ / 6, y: -R_ / 6 },
	},
	mline_t {
		a: mpoint_t { x: -R_ / 2 + R_ / 6, y: -R_ / 6 },
		b: mpoint_t { x: -R_ / 2 + R_ / 6, y: R_ / 4 },
	},
	mline_t { a: mpoint_t { x: -R_ / 6, y: 0 }, b: mpoint_t { x: -R_ / 6, y: -R_ / 6 } }, //  >>-dd-->
	mline_t { a: mpoint_t { x: -R_ / 6, y: -R_ / 6 }, b: mpoint_t { x: 0, y: -R_ / 6 } },
	mline_t { a: mpoint_t { x: 0, y: -R_ / 6 }, b: mpoint_t { x: 0, y: R_ / 4 } },
	mline_t { a: mpoint_t { x: R_ / 6, y: R_ / 4 }, b: mpoint_t { x: R_ / 6, y: -R_ / 7 } }, //  >>-ddt->
	mline_t {
		a: mpoint_t { x: R_ / 6, y: -R_ / 7 },
		b: mpoint_t { x: R_ / 6 + R_ / 32, y: -R_ / 7 - R_ / 32 },
	},
	mline_t {
		a: mpoint_t { x: R_ / 6 + R_ / 32, y: -R_ / 7 - R_ / 32 },
		b: mpoint_t { x: R_ / 6 + R_ / 10, y: -R_ / 7 },
	},
];

const NUMTHINTRIANGLEGUYLINES: usize = 3;
static thintriangle_guy: [mline_t; NUMTHINTRIANGLEGUYLINES] = [
	mline_t {
		a: mpoint_t {
			x: (-0.5 * FRACUNIT as f64) as fixed_t,
			y: (-0.7 * FRACUNIT as f64) as fixed_t,
		},
		b: mpoint_t { x: FRACUNIT, y: 0 },
	},
	mline_t {
		a: mpoint_t { x: FRACUNIT, y: 0 },
		b: mpoint_t {
			x: (-0.5 * FRACUNIT as f64) as fixed_t,
			y: (0.7 * FRACUNIT as f64) as fixed_t,
		},
	},
	mline_t {
		a: mpoint_t {
			x: (-0.5 * FRACUNIT as f64) as fixed_t,
			y: (0.7 * FRACUNIT as f64) as fixed_t,
		},
		b: mpoint_t {
			x: (-0.5 * FRACUNIT as f64) as fixed_t,
			y: (-0.7 * FRACUNIT as f64) as fixed_t,
		},
	},
];

static mut cheating: int = 0;
static mut grid: int = 0;

static mut leveljuststarted: int = 1; // kluge until AM_LevelInit() is called

#[unsafe(no_mangle)]
pub(crate) static mut automapactive: boolean = 0;
static mut finit_width: fixed_t = SCREENWIDTH as fixed_t;
static mut finit_height: fixed_t = SCREENHEIGHT as fixed_t - 32;

// location of window on screen
static mut f_x: fixed_t = 0;
static mut f_y: fixed_t = 0;

// size of window on screen
static mut f_w: fixed_t = 0;
static mut f_h: fixed_t = 0;

static mut lightlev: int = 0; // used for funky strobing effect
static mut fb: *mut u8 = null_mut(); // pseudo-frame buffer
static mut amclock: int = 0;

static mut m_paninc: mpoint_t = mpoint_t { x: 0, y: 0 }; // how far the window pans each tic (map coords)
static mut mtof_zoommul: fixed_t = 0; // how far the window zooms in each tic (map coords)
static mut ftom_zoommul: fixed_t = 0; // how far the window zooms in each tic (fb coords)

// LL x,y where the window is on the map (map coords)
static mut m_x: fixed_t = 0;
static mut m_y: fixed_t = 0;
// UR x,y where the window is on the map (map coords)
static mut m_x2: fixed_t = 0;
static mut m_y2: fixed_t = 0;

// width/height of window on map (map coords)
static mut m_w: fixed_t = 0;
static mut m_h: fixed_t = 0;

// based on level size
static mut min_x: fixed_t = 0;
static mut min_y: fixed_t = 0;
static mut max_x: fixed_t = 0;
static mut max_y: fixed_t = 0;

static mut max_w: fixed_t = 0; // max_x-min_x,
static mut max_h: fixed_t = 0; // max_y-min_y

// based on player size
static mut min_w: fixed_t = 0;
static mut min_h: fixed_t = 0;

static mut min_scale_mtof: fixed_t = 0; // used to tell when to stop zooming out
static mut max_scale_mtof: fixed_t = 0; // used to tell when to stop zooming in

// old stuff for recovery later
static mut old_m_w: fixed_t = 0;
static mut old_m_h: fixed_t = 0;
static mut old_m_x: fixed_t = 0;
static mut old_m_y: fixed_t = 0;

// old location used by the Follower routine
static mut f_oldloc: mpoint_t = mpoint_t { x: 0, y: 0 };

// used by MTOF to scale from map-to-frame-buffer coords
static mut scale_mtof: fixed_t = INITSCALEMTOF;
// used by FTOM to scale from frame-buffer-to-map coords (=1/scale_mtof)
static mut scale_ftom: fixed_t = 0;

static mut plr: *mut player_t = null_mut(); // the player represented by an arrow

static mut marknums: [*mut patch_t; 10] = [null_mut(); 10]; // numbers used for marking by the automap
// where the points are
static mut markpoints: [mpoint_t; AM_NUMMARKPOINTS] = [mpoint_t { x: 0, y: 0 }; AM_NUMMARKPOINTS];
static mut markpointnum: usize = 0; // next point to be assigned

static mut followplayer: int = 1; // specifies whether to follow the player around

static mut cheat_amap_seq: [u8; 5] = [0xb2, 0x26, 0x26, 0x2e, 0xff];
#[allow(static_mut_refs)]
static mut cheat_amap: cheatseq_t =
	cheatseq_t { sequence: unsafe { cheat_amap_seq.as_mut_ptr() }, p: null_mut() };

static mut stopped: bool = true;

fn AM_activateNewScale() {
	unsafe {
		m_x += m_w / 2;
		m_y += m_h / 2;
		m_w = FTOM(f_w);
		m_h = FTOM(f_h);
		m_x -= m_w / 2;
		m_y -= m_h / 2;
		m_x2 = m_x + m_w;
		m_y2 = m_y + m_h;
	}
}

fn AM_saveScaleAndLoc() {
	unsafe {
		old_m_x = m_x;
		old_m_y = m_y;
		old_m_w = m_w;
		old_m_h = m_h;
	}
}

fn AM_restoreScaleAndLoc() {
	unsafe {
		m_w = old_m_w;
		m_h = old_m_h;
		if followplayer == 0 {
			m_x = old_m_x;
			m_y = old_m_y;
		} else {
			m_x = (*(*plr).mo).x - m_w / 2;
			m_y = (*(*plr).mo).y - m_h / 2;
		}
		m_x2 = m_x + m_w;
		m_y2 = m_y + m_h;

		// Change the scaling multipliers
		scale_mtof = FixedDiv(f_w << FRACBITS, m_w);
		scale_ftom = FixedDiv(FRACUNIT, scale_mtof);
	}
}

// adds a marker at the current location
fn AM_addMark() {
	unsafe {
		markpoints[markpointnum].x = m_x + m_w / 2;
		markpoints[markpointnum].y = m_y + m_h / 2;
		markpointnum = (markpointnum + 1) % AM_NUMMARKPOINTS;
	}
}

// Determines bounding box of all vertices,
// sets global variables controlling zoom range.
fn AM_findMinMaxBoundaries() {
	unsafe {
		min_x = i32::MAX;
		min_y = i32::MAX;
		max_x = -i32::MAX;
		max_y = -i32::MAX;

		for i in 0..numvertexes {
			let vertex = &*vertexes.wrapping_add(i);
			if vertex.x < min_x {
				min_x = vertex.x;
			} else if vertex.x > max_x {
				max_x = vertex.x;
			}

			if vertex.y < min_y {
				min_y = vertex.y;
			} else if vertex.y > max_y {
				max_y = vertex.y;
			}
		}

		max_w = max_x - min_x;
		max_h = max_y - min_y;

		min_w = 2 * PLAYERRADIUS; // const? never changed?
		min_h = 2 * PLAYERRADIUS;

		let a = FixedDiv(f_w << FRACBITS, max_w);
		let b = FixedDiv(f_h << FRACBITS, max_h);

		min_scale_mtof = if a < b { a } else { b };
		max_scale_mtof = FixedDiv(f_h << FRACBITS, 2 * PLAYERRADIUS);
	}
}

fn AM_changeWindowLoc() {
	unsafe {
		if m_paninc.x == 0 || m_paninc.y == 0 {
			followplayer = 0;
			f_oldloc.x = i32::MAX;
		}

		m_x += m_paninc.x;
		m_y += m_paninc.y;

		if m_x + m_w / 2 > max_x {
			m_x = max_x - m_w / 2;
		} else if m_x + m_w / 2 < min_x {
			m_x = min_x - m_w / 2;
		}

		if m_y + m_h / 2 > max_y {
			m_y = max_y - m_h / 2;
		} else if m_y + m_h / 2 < min_y {
			m_y = min_y - m_h / 2;
		}

		m_x2 = m_x + m_w;
		m_y2 = m_y + m_h;
	}
}

unsafe extern "C" {
	fn ST_Responder(ev: *mut event_t) -> boolean;
}

fn AM_initVariables() {
	unsafe {
		static mut st_notify: event_t =
			event_t { ty: evtype_t::ev_keyup, data1: AM_MSGENTERED as i32, data2: 0, data3: 0 };

		automapactive = 1;
		fb = screens[0];

		f_oldloc.x = i32::MAX;
		amclock = 0;
		lightlev = 0;

		m_paninc.x = 0;
		m_paninc.y = 0;
		ftom_zoommul = FRACUNIT;
		mtof_zoommul = FRACUNIT;

		m_w = FTOM(f_w);
		m_h = FTOM(f_h);

		// find player to center on initially
		let mut pnum = consoleplayer;
		if playeringame[pnum] == 0 {
			for i in 0..MAXPLAYERS {
				pnum = i;
				if playeringame[pnum] != 0 {
					break;
				}
			}
		}
		plr = &raw mut players[pnum];
		m_x = (*(*plr).mo).x - m_w / 2;
		m_y = (*(*plr).mo).y - m_h / 2;
		AM_changeWindowLoc();

		// for saving & restoring
		old_m_x = m_x;
		old_m_y = m_y;
		old_m_w = m_w;
		old_m_h = m_h;

		// inform the status bar of the change
		ST_Responder(&raw mut st_notify);
	}
}

fn AM_loadPics() {
	unsafe {
		let mut namebuf = [0; 9];
		#[allow(clippy::needless_range_loop)]
		for i in 0..10 {
			libc::sprintf(namebuf.as_mut_ptr(), c"AMMNUM%d".as_ptr(), i);
			marknums[i] = W_CacheLumpName(namebuf.as_ptr(), PU_STATIC).cast();
		}
	}
}

fn AM_unloadPics() {
	unsafe {
		#[allow(clippy::needless_range_loop)]
		for i in 0..10 {
			Z_ChangeTag!(marknums[i], PU_CACHE);
		}
	}
}

fn AM_clearMarks() {
	unsafe {
		#[allow(clippy::needless_range_loop)]
		for i in 0..AM_NUMMARKPOINTS {
			markpoints[i].x = -1; // means empty
		}
		markpointnum = 0;
	}
}

// should be called at the start of every level
// right now, i figure it out myself
fn AM_LevelInit() {
	unsafe {
		leveljuststarted = 0;

		f_x = 0;
		f_y = 0;
		f_w = finit_width;
		f_h = finit_height;

		AM_clearMarks();

		AM_findMinMaxBoundaries();
		scale_mtof = FixedDiv(min_scale_mtof, (0.7 * FRACUNIT as f64) as fixed_t);
		if scale_mtof > max_scale_mtof {
			scale_mtof = min_scale_mtof;
		}
		scale_ftom = FixedDiv(FRACUNIT, scale_mtof);
	}
}

pub(crate) fn AM_Stop() {
	unsafe {
		static mut st_notify: event_t = event_t {
			ty: evtype_t::ev_keydown, /*0 // FIXME bug?*/
			data1: evtype_t::ev_keyup as i32,
			data2: AM_MSGEXITED as i32,
			data3: 0,
		};

		AM_unloadPics();
		automapactive = 0;
		ST_Responder(&raw mut st_notify);
		stopped = true;
	}
}

fn AM_Start() {
	unsafe {
		static mut lastlevel: i32 = -1;
		static mut lastepisode: i32 = -1;

		if !stopped {
			AM_Stop();
		}
		stopped = false;
		if lastlevel != gamemap as i32 || lastepisode != gameepisode as i32 {
			AM_LevelInit();
			lastlevel = gamemap as i32;
			lastepisode = gameepisode as i32;
		}
		AM_initVariables();
		AM_loadPics();
	}
}

// set the window scale to the maximum size
fn AM_minOutWindowScale() {
	unsafe {
		scale_mtof = min_scale_mtof;
		scale_ftom = FixedDiv(FRACUNIT, scale_mtof);
		AM_activateNewScale();
	}
}

// set the window scale to the minimum size
fn AM_maxOutWindowScale() {
	unsafe {
		scale_mtof = max_scale_mtof;
		scale_ftom = FixedDiv(FRACUNIT, scale_mtof);
		AM_activateNewScale();
	}
}

// Handle events (user inputs) in automap mode
#[allow(static_mut_refs)]
pub(crate) fn AM_Responder(ev: *mut event_t) -> boolean {
	unsafe {
		static mut cheatstate: i32 = 0;
		static mut bigstate: i32 = 0;
		static mut buffer: [c_char; 20] = [0; 20];

		let mut rc = false;

		if automapactive == 0 {
			if (*ev).ty == evtype_t::ev_keydown && (*ev).data1 == AM_STARTKEY as i32 {
				AM_Start();
				viewactive = 0;
				rc = true;
			}
		} else if (*ev).ty == evtype_t::ev_keydown {
			rc = true;
			match (*ev).data1 as u8 {
				AM_PANRIGHTKEY => {
					// pan right
					if followplayer == 0 {
						m_paninc.x = FTOM(F_PANINC);
					} else {
						rc = false;
					}
				}
				AM_PANLEFTKEY => {
					// pan left
					if followplayer == 0 {
						m_paninc.x = -FTOM(F_PANINC);
					} else {
						rc = false;
					}
				}
				AM_PANUPKEY => {
					// pan up
					if followplayer == 0 {
						m_paninc.y = FTOM(F_PANINC);
					} else {
						rc = false;
					}
				}
				AM_PANDOWNKEY => {
					// pan down
					if followplayer == 0 {
						m_paninc.y = -FTOM(F_PANINC);
					} else {
						rc = false;
					}
				}
				AM_ZOOMOUTKEY => {
					// zoom out
					mtof_zoommul = M_ZOOMOUT;
					ftom_zoommul = M_ZOOMIN;
				}
				AM_ZOOMINKEY => {
					// zoom in
					mtof_zoommul = M_ZOOMIN;
					ftom_zoommul = M_ZOOMOUT;
				}
				AM_ENDKEY => {
					bigstate = 0;
					viewactive = 1;
					AM_Stop();
				}
				AM_GOBIGKEY => {
					bigstate = if bigstate == 0 { 1 } else { 0 };
					if bigstate != 0 {
						AM_saveScaleAndLoc();
						AM_minOutWindowScale();
					} else {
						AM_restoreScaleAndLoc();
					}
				}
				AM_FOLLOWKEY => {
					followplayer = if followplayer == 0 { 1 } else { 0 };
					f_oldloc.x = i32::MAX;
					(*plr).message =
						if followplayer != 0 { AMSTR_FOLLOWON } else { AMSTR_FOLLOWOFF };
				}
				AM_GRIDKEY => {
					grid = if grid == 0 { 1 } else { 0 };
					(*plr).message = if grid != 0 { AMSTR_GRIDON } else { AMSTR_GRIDOFF };
				}
				AM_MARKKEY => {
					libc::sprintf(
						buffer.as_mut_ptr(),
						c"%s %d".as_ptr(),
						AMSTR_MARKEDSPOT,
						markpointnum,
					);
					(*plr).message = buffer.as_ptr();
					AM_addMark();
				}
				AM_CLEARMARKKEY => {
					AM_clearMarks();
					(*plr).message = AMSTR_MARKSCLEARED;
				}
				_ => {
					cheatstate = 0;
					rc = false;
				}
			}
			if deathmatch == 0 && cht_CheckCheat(&mut cheat_amap, (*ev).data1 as u8) != 0 {
				rc = false;
				cheating = (cheating + 1) % 3;
			}
		} else if (*ev).ty == evtype_t::ev_keyup {
			rc = false;
			match (*ev).data1 as u8 {
				AM_PANRIGHTKEY => {
					if followplayer == 0 {
						m_paninc.x = 0;
					}
				}
				AM_PANLEFTKEY => {
					if followplayer == 0 {
						m_paninc.x = 0;
					}
				}
				AM_PANUPKEY => {
					if followplayer == 0 {
						m_paninc.y = 0;
					}
				}
				AM_PANDOWNKEY => {
					if followplayer == 0 {
						m_paninc.y = 0;
					}
				}
				AM_ZOOMOUTKEY | AM_ZOOMINKEY => {
					mtof_zoommul = FRACUNIT;
					ftom_zoommul = FRACUNIT;
				}
				_ => (),
			}
		}

		rc as boolean
	}
}

// Zooming
fn AM_changeWindowScale() {
	unsafe {
		// Change the scaling multipliers
		scale_mtof = FixedMul(scale_mtof, mtof_zoommul);
		scale_ftom = FixedDiv(FRACUNIT, scale_mtof);

		if scale_mtof < min_scale_mtof {
			AM_minOutWindowScale();
		} else if scale_mtof > max_scale_mtof {
			AM_maxOutWindowScale();
		} else {
			AM_activateNewScale();
		}
	}
}

fn AM_doFollowPlayer() {
	unsafe {
		if f_oldloc.x != (*(*plr).mo).x || f_oldloc.y != (*(*plr).mo).y {
			m_x = FTOM(MTOF((*(*plr).mo).x)) - m_w / 2;
			m_y = FTOM(MTOF((*(*plr).mo).y)) - m_h / 2;
			m_x2 = m_x + m_w;
			m_y2 = m_y + m_h;
			f_oldloc.x = (*(*plr).mo).x;
			f_oldloc.y = (*(*plr).mo).y;
		}
	}
}

// Updates on Game Tick
pub(crate) fn AM_Ticker() {
	unsafe {
		if automapactive == 0 {
			return;
		}

		amclock += 1;

		if followplayer != 0 {
			AM_doFollowPlayer();
		}

		// Change the zoom if necessary
		if ftom_zoommul != FRACUNIT {
			AM_changeWindowScale();
		}

		// Change x,y location
		if m_paninc.x != 0 || m_paninc.y != 0 {
			AM_changeWindowLoc();
		}

		// Update light level
		// AM_updateLightLev();
	}
}

// Clear automap frame buffer.
fn AM_clearFB(color: i32) {
	unsafe { libc::memset(fb.cast(), color, (f_w * f_h) as usize) };
}

// Automap clipping of lines.
//
// Based on Cohen-Sutherland clipping algorithm but with a slightly
// faster reject and precalculated slopes.  If the speed is needed,
// use a hash algorithm to handle  the common cases.
fn AM_clipMline(ml: &mline_t, fl: &mut fline_t) -> boolean {
	unsafe {
		const LEFT: i32 = 1;
		const RIGHT: i32 = 2;
		const BOTTOM: i32 = 4;
		const TOP: i32 = 8;

		macro_rules! DOOUTCODE {
			($oc:ident, $mx:expr, $my:expr) => {
				$oc = 0;
				if $my < 0 {
					$oc |= TOP;
				} else if $my >= f_h {
					$oc |= BOTTOM;
				}
				if $mx < 0 {
					$oc |= LEFT;
				} else if $mx >= f_w {
					$oc |= RIGHT;
				}
			};
		}

		let mut outcode1 = 0;
		let mut outcode2 = 0;

		// do trivial rejects and outcodes
		if ml.a.y > m_y2 {
			outcode1 = TOP;
		} else if ml.a.y < m_y {
			outcode1 = BOTTOM;
		}

		if ml.b.y > m_y2 {
			outcode2 = TOP;
		} else if ml.b.y < m_y {
			outcode2 = BOTTOM;
		}

		if outcode1 & outcode2 != 0 {
			return 0; // trivially outside
		}

		if ml.a.x < m_x {
			outcode1 |= LEFT;
		} else if ml.a.x > m_x2 {
			outcode1 |= RIGHT;
		}

		if ml.b.x < m_x {
			outcode2 |= LEFT;
		} else if ml.b.x > m_x2 {
			outcode2 |= RIGHT;
		}

		if outcode1 & outcode2 != 0 {
			return 0; // trivially outside
		}

		// transform to frame-buffer coordinates.
		fl.a.x = CXMTOF(ml.a.x);
		fl.a.y = CYMTOF(ml.a.y);
		fl.b.x = CXMTOF(ml.b.x);
		fl.b.y = CYMTOF(ml.b.y);

		DOOUTCODE!(outcode1, fl.a.x, fl.a.y);
		DOOUTCODE!(outcode2, fl.b.x, fl.b.y);

		if outcode1 & outcode2 != 0 {
			return 0;
		}

		while outcode1 | outcode2 != 0 {
			// may be partially inside box
			// find an outside point
			let outside = if outcode1 != 0 { outcode1 } else { outcode2 };

			// clip to each side
			let dx;
			let dy;
			let mut tmp = fpoint_t { x: 0, y: 0 };
			if outside & TOP != 0 {
				dy = fl.a.y - fl.b.y;
				dx = fl.b.x - fl.a.x;
				tmp.x = fl.a.x + (dx * (fl.a.y)) / dy;
				tmp.y = 0;
			} else if outside & BOTTOM != 0 {
				dy = fl.a.y - fl.b.y;
				dx = fl.b.x - fl.a.x;
				tmp.x = fl.a.x + (dx * (fl.a.y - f_h)) / dy;
				tmp.y = f_h - 1;
			} else if outside & RIGHT != 0 {
				dy = fl.b.y - fl.a.y;
				dx = fl.b.x - fl.a.x;
				tmp.y = fl.a.y + (dy * (f_w - 1 - fl.a.x)) / dx;
				tmp.x = f_w - 1;
			} else if outside & LEFT != 0 {
				dy = fl.b.y - fl.a.y;
				dx = fl.b.x - fl.a.x;
				tmp.y = fl.a.y + (dy * (-fl.a.x)) / dx;
				tmp.x = 0;
			}

			if outside == outcode1 {
				fl.a = tmp;
				DOOUTCODE!(outcode1, fl.a.x, fl.a.y);
			} else {
				fl.b = tmp;
				DOOUTCODE!(outcode2, fl.b.x, fl.b.y);
			}

			if outcode1 & outcode2 != 0 {
				return 0; // trivially outside
			}
		}

		1
	}
}

// Classic Bresenham w/ whatever optimizations needed for speed
#[allow(static_mut_refs)]
fn AM_drawFline(fl: &fline_t, color: i32) {
	unsafe {
		static mut fuck: i32 = 0;

		// For debugging only
		if fl.a.x < 0
			|| fl.a.x >= f_w
			|| fl.a.y < 0
			|| fl.a.y >= f_h
			|| fl.b.x < 0
			|| fl.b.x >= f_w
			|| fl.b.y < 0
			|| fl.b.y >= f_h
		{
			eprintln!("fuck {} \r", fuck);
			fuck += 1;
			return;
		}

		let PUTDOT = |xx: usize, yy: usize, cc: u8| *fb.wrapping_add(yy * f_w as usize + xx) = cc;

		let dx = fl.b.x - fl.a.x;
		let ax = 2 * (if dx < 0 { -dx } else { dx });
		let sx = if dx < 0 { -1 } else { 1 };

		let dy = fl.b.y - fl.a.y;
		let ay = 2 * (if dy < 0 { -dy } else { dy });
		let sy = if dy < 0 { -1 } else { 1 };

		let mut x = fl.a.x;
		let mut y = fl.a.y;

		if ax > ay {
			let mut d = ay - ax / 2;
			loop {
				PUTDOT(x as usize, y as usize, color as u8);
				if x == fl.b.x {
					return;
				}
				if d >= 0 {
					y += sy;
					d -= ax;
				}
				x += sx;
				d += ay;
			}
		} else {
			let mut d = ax - ay / 2;
			loop {
				PUTDOT(x as usize, y as usize, color as u8);
				if y == fl.b.y {
					return;
				}
				if d >= 0 {
					x += sx;
					d -= ay;
				}
				y += sy;
				d += ax;
			}
		}
	}
}

// Clip lines, draw visible part sof lines.
#[allow(static_mut_refs)]
fn AM_drawMline(ml: &mline_t, color: i32) {
	unsafe {
		static mut fl: fline_t = fline_t { a: fpoint_t { x: 0, y: 0 }, b: fpoint_t { x: 0, y: 0 } };

		if AM_clipMline(ml, &mut fl) != 0 {
			AM_drawFline(&fl, color); // draws it on frame buffer using fb coords
		}
	}
}

// Draws flat (floor/ceiling tile) aligned grid lines.
fn AM_drawGrid(color: i32) {
	unsafe {
		let mut ml = mline_t { a: mpoint_t { x: 0, y: 0 }, b: mpoint_t { x: 0, y: 0 } };

		// Figure out start of vertical gridlines
		let mut start = m_x;
		if (start - bmaporgx) % (MAPBLOCKUNITS << FRACBITS) != 0 {
			start +=
				(MAPBLOCKUNITS << FRACBITS) - ((start - bmaporgx) % (MAPBLOCKUNITS << FRACBITS));
		}
		let end = m_x + m_w;

		// draw vertical gridlines
		ml.a.y = m_y;
		ml.b.y = m_y + m_h;
		for x in (start..end).step_by((MAPBLOCKUNITS << FRACBITS) as usize) {
			ml.a.x = x;
			ml.b.x = x;
			AM_drawMline(&ml, color);
		}

		// Figure out start of horizontal gridlines
		let mut start = m_y;
		if (start - bmaporgy) % (MAPBLOCKUNITS << FRACBITS) != 0 {
			start +=
				(MAPBLOCKUNITS << FRACBITS) - ((start - bmaporgy) % (MAPBLOCKUNITS << FRACBITS));
		}
		let end = m_y + m_h;

		// draw horizontal gridlines
		ml.a.x = m_x;
		ml.b.x = m_x + m_w;
		for y in (start..end).step_by((MAPBLOCKUNITS << FRACBITS) as usize) {
			ml.a.y = y;
			ml.b.y = y;
			AM_drawMline(&ml, color);
		}
	}
}

// Determines visible lines, draws them.
// This is LineDef based, not LineSeg based.
#[allow(static_mut_refs)]
fn AM_drawWalls() {
	unsafe {
		static mut l: mline_t = mline_t { a: mpoint_t { x: 0, y: 0 }, b: mpoint_t { x: 0, y: 0 } };

		for i in 0..numlines {
			let line = lines.wrapping_add(i);
			l.a.x = (*(*line).v1).x;
			l.a.y = (*(*line).v1).y;
			l.b.x = (*(*line).v2).x;
			l.b.y = (*(*line).v2).y;
			if cheating != 0 || (*line).flags as usize & ML_MAPPED != 0 {
				if (*line).flags as usize & LINE_NEVERSEE != 0 && cheating == 0 {
					continue;
				}
				if (*line).backsector.is_null() {
					AM_drawMline(&l, WALLCOLORS + lightlev);
				} else if (*line).special == 39 {
					// teleporters
					AM_drawMline(&l, WALLCOLORS + WALLRANGE / 2);
				} else if (*line).flags as usize & ML_SECRET != 0 {
					// secret door
					if cheating != 0 {
						AM_drawMline(&l, SECRETWALLCOLORS + lightlev);
					} else {
						AM_drawMline(&l, WALLCOLORS + lightlev);
					}
				} else if (*(*line).backsector).floorheight != (*(*line).frontsector).floorheight {
					AM_drawMline(&l, FDWALLCOLORS + lightlev); // floor level change
				} else if (*(*line).backsector).ceilingheight
					!= (*(*line).frontsector).ceilingheight
				{
					AM_drawMline(&l, CDWALLCOLORS + lightlev); // ceiling level change
				} else if cheating != 0 {
					AM_drawMline(&l, TSWALLCOLORS + lightlev);
				}
			} else if (*plr).powers[powertype_t::pw_allmap as usize] != 0
				&& (*line).flags as usize & LINE_NEVERSEE == 0
			{
				AM_drawMline(&l, GRAYS + 3);
			}
		}
	}
}

// Rotation in 2D.
// Used to rotate player arrow line character.
fn AM_rotate(x: &mut fixed_t, y: &mut fixed_t, a: angle_t) {
	let a = a.0 >> ANGLETOFINESHIFT;
	let tmpx = FixedMul(*x, finecos(a)) - FixedMul(*y, finesine[a]);
	*y = FixedMul(*x, finesine[a]) + FixedMul(*y, finecos(a));
	*x = tmpx;
}

fn AM_drawLineCharacter(
	lineguy: *const mline_t,
	lineguylines: usize,
	scale: fixed_t,
	angle: angle_t,
	color: i32,
	x: fixed_t,
	y: fixed_t,
) {
	unsafe {
		let mut l = mline_t { a: mpoint_t { x: 0, y: 0 }, b: mpoint_t { x: 0, y: 0 } };

		for i in 0..lineguylines {
			l.a.x = (*lineguy.wrapping_add(i)).a.x;
			l.a.y = (*lineguy.wrapping_add(i)).a.y;

			if scale != 0 {
				l.a.x = FixedMul(scale, l.a.x);
				l.a.y = FixedMul(scale, l.a.y);
			}

			if angle.0 != 0 {
				AM_rotate(&mut l.a.x, &mut l.a.y, angle);
			}

			l.a.x += x;
			l.a.y += y;

			l.b.x = (*lineguy.wrapping_add(i)).b.x;
			l.b.y = (*lineguy.wrapping_add(i)).b.y;

			if scale != 0 {
				l.b.x = FixedMul(scale, l.b.x);
				l.b.y = FixedMul(scale, l.b.y);
			}

			if angle.0 != 0 {
				AM_rotate(&mut l.b.x, &mut l.b.y, angle);
			}

			l.b.x += x;
			l.b.y += y;

			AM_drawMline(&l, color);
		}
	}
}

fn AM_drawPlayers() {
	unsafe {
		static their_colors: [i32; 4] = [GREENS, GRAYS, BROWNS, REDS];
		let mut their_color = -1;

		if netgame == 0 {
			if cheating != 0 {
				AM_drawLineCharacter(
					cheat_player_arrow.as_ptr(),
					NUMCHEATPLYRLINES,
					0,
					(*(*plr).mo).angle,
					WHITE,
					(*(*plr).mo).x,
					(*(*plr).mo).y,
				);
			} else {
				AM_drawLineCharacter(
					player_arrow.as_ptr(),
					NUMPLYRLINES,
					0,
					(*(*plr).mo).angle,
					WHITE,
					(*(*plr).mo).x,
					(*(*plr).mo).y,
				);
			}
			return;
		}

		for i in 0..MAXPLAYERS {
			their_color += 1;
			let p = &raw const players[i];

			if deathmatch != 0 && !singledemo && !std::ptr::eq(p, plr) {
				continue;
			}

			if playeringame[i] == 0 {
				continue;
			}

			let color = if (*p).powers[powertype_t::pw_invisibility as usize] != 0 {
				246 // *close* to black
			} else {
				their_colors[their_color as usize]
			};

			AM_drawLineCharacter(
				player_arrow.as_ptr(),
				NUMPLYRLINES,
				0,
				(*(*p).mo).angle,
				color,
				(*(*p).mo).x,
				(*(*p).mo).y,
			);
		}
	}
}

fn AM_drawThings(colors: i32, _colorrange: i32) {
	unsafe {
		for i in 0..numsectors {
			let mut t = (*sectors.wrapping_add(i)).thinglist;
			while !t.is_null() {
				AM_drawLineCharacter(
					thintriangle_guy.as_ptr(),
					NUMTHINTRIANGLEGUYLINES,
					16 << FRACBITS,
					(*t).angle,
					colors + lightlev,
					(*t).x,
					(*t).y,
				);
				t = (*t).snext;
			}
		}
	}
}

fn AM_drawMarks() {
	unsafe {
		for i in 0..AM_NUMMARKPOINTS {
			if markpoints[i].x != -1 {
				let w = 5; // because something's wrong with the wad, i guess
				let h = 6; // because something's wrong with the wad, i guess
				let fx = CXMTOF(markpoints[i].x);
				let fy = CYMTOF(markpoints[i].y);
				if fx >= f_x && fx <= f_w - w && fy >= f_y && fy <= f_h - h {
					V_DrawPatch(fx as usize, fy as usize, FB, marknums[i]);
				}
			}
		}
	}
}

fn AM_drawCrosshair(color: i32) {
	// single point for now
	unsafe { *fb.wrapping_add((f_w as usize * (f_h as usize + 1)) / 2) = color as u8 };
}

pub(crate) fn AM_Drawer() {
	unsafe {
		if automapactive == 0 {
			return;
		}

		AM_clearFB(BACKGROUND);
		if grid != 0 {
			AM_drawGrid(GRIDCOLORS);
		}
		AM_drawWalls();
		AM_drawPlayers();
		if cheating == 2 {
			AM_drawThings(THINGCOLORS, THINGRANGE);
		}
		AM_drawCrosshair(XHAIRCOLORS);

		AM_drawMarks();

		V_MarkRect(f_x as usize, f_y as usize, f_w as usize, f_h as usize);
	}
}
