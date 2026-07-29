//	Status bar code.
//	Does the face/direction indicator animatin.
//	Does palette indicators as well (red pain/berserk, bright pickup)
#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]
#![allow(static_mut_refs)]

use std::ptr::{self, null_mut};

use crate::{
	am_map::{AM_MSGENTERED, AM_MSGEXITED, AM_MSGHEADER, automapactive},
	d_englsh::{
		STSTR_BEHOLD, STSTR_BEHOLDX, STSTR_CHOPPERS, STSTR_CLEV, STSTR_DQDOFF, STSTR_DQDON,
		STSTR_FAADDED, STSTR_KFAADDED, STSTR_MUS, STSTR_NCOFF, STSTR_NCON, STSTR_NOMUS,
	},
	d_event::{event_t, evtype_t},
	d_items::weaponinfo,
	d_player::{CF_GODMODE, CF_NOCLIP, player_t},
	doomdef::{
		GameMode_t, MAXPLAYERS, SCREEN_MUL, SCREENHEIGHT, SCREENWIDTH, TICRATE, ammotype_t, card_t,
		powertype_t, weapontype_t,
	},
	doomstat::gamemode,
	g_game::{G_DeferedInitNew, consoleplayer, deathmatch, gameskill, netgame, players},
	i_video::I_SetPalette,
	m_cheat::{cheatseq_t, cht_CheckCheat, cht_GetParam},
	m_random::M_Random,
	p_inter::P_GivePower,
	r_defs::patch_t,
	r_main::R_PointToAngle2,
	s_sound::S_ChangeMusic,
	sounds::musicenum_t,
	st_lib::{
		BG, FG, STlib_init, STlib_initBinIcon, STlib_initMultIcon, STlib_initNum,
		STlib_initPercent, STlib_updateBinIcon, STlib_updateMultIcon, STlib_updateNum,
		STlib_updatePercent, st_binicon_t, st_multicon_t, st_number_t, st_percent_t,
	},
	tables::{ANG45, ANG180},
	v_video::{V_CopyRect, V_DrawPatch, screens},
	w_wad::{W_CacheLumpName, W_CacheLumpNum, W_GetNumForName},
	z_zone::{PU_CACHE, PU_STATIC, Z_Malloc},
};

pub(crate) const ST_HEIGHT: usize = 32 * SCREEN_MUL;
pub(crate) const ST_WIDTH: usize = SCREENWIDTH;
pub(crate) const ST_Y: usize = SCREENHEIGHT - ST_HEIGHT;

// States for status bar code.
enum st_stateenum_t {
	AutomapState,
	FirstPersonState,
}

// States for the chat code.
enum st_chatstateenum_t {
	StartChatState,
	// WaitDestState,
	// GetChatState,
}

// STATUS BAR DATA

// Palette indices.
// For damage/bonus red-/gold-shifts
const STARTREDPALS: usize = 1;
const STARTBONUSPALS: usize = 9;
const NUMREDPALS: usize = 8;
const NUMBONUSPALS: usize = 4;
// Radiation suit, green shift.
const RADIATIONPAL: usize = 13;

// Location of status bar
const ST_X: usize = 0;

const ST_FX: usize = 143;

// Number of status faces.
const ST_NUMPAINFACES: usize = 5;
const ST_NUMSTRAIGHTFACES: usize = 3;
const ST_NUMTURNFACES: usize = 2;
const ST_NUMSPECIALFACES: usize = 3;

const ST_FACESTRIDE: usize = ST_NUMSTRAIGHTFACES + ST_NUMTURNFACES + ST_NUMSPECIALFACES;

const ST_NUMEXTRAFACES: usize = 2;

const ST_NUMFACES: usize = ST_FACESTRIDE * ST_NUMPAINFACES + ST_NUMEXTRAFACES;

const ST_TURNOFFSET: usize = ST_NUMSTRAIGHTFACES;
const ST_OUCHOFFSET: usize = ST_TURNOFFSET + ST_NUMTURNFACES;
const ST_EVILGRINOFFSET: usize = ST_OUCHOFFSET + 1;
const ST_RAMPAGEOFFSET: usize = ST_EVILGRINOFFSET + 1;
const ST_GODFACE: usize = ST_NUMPAINFACES * ST_FACESTRIDE;
const ST_DEADFACE: usize = ST_GODFACE + 1;

const ST_FACESX: usize = 143;
const ST_FACESY: usize = 168;

const ST_EVILGRINCOUNT: usize = 2 * TICRATE;
const ST_STRAIGHTFACECOUNT: usize = TICRATE / 2;
const ST_TURNCOUNT: usize = TICRATE;
const ST_RAMPAGEDELAY: usize = 2 * TICRATE;

const ST_MUCHPAIN: i32 = 20;

// Location and size of statistics,
//  justified according to widget type.
// Problem is, within which space? STbar? Screen?
// Note: this could be read in by a lump.
//       Problem is, is the stuff rendered
//       into a buffer,
//       or into the frame buffer?

// AMMO number pos.
const ST_AMMOWIDTH: usize = 3;
const ST_AMMOX: usize = 44;
const ST_AMMOY: usize = 171;

// HEALTH number pos.
const ST_HEALTHX: usize = 90;
const ST_HEALTHY: usize = 171;

// Weapon pos.
const ST_ARMSX: usize = 111;
const ST_ARMSY: usize = 172;
const ST_ARMSBGX: usize = 104;
const ST_ARMSBGY: usize = 168;
const ST_ARMSXSPACE: usize = 12;
const ST_ARMSYSPACE: usize = 10;

// Frags pos.
const ST_FRAGSX: usize = 138;
const ST_FRAGSY: usize = 171;
const ST_FRAGSWIDTH: usize = 2;

// ARMOR number pos.
const ST_ARMORX: usize = 221;
const ST_ARMORY: usize = 171;

// Key icon positions.
const ST_KEY0X: usize = 239;
const ST_KEY0Y: usize = 171;
const ST_KEY1X: usize = 239;
const ST_KEY1Y: usize = 181;
const ST_KEY2X: usize = 239;
const ST_KEY2Y: usize = 191;

// Ammunition counter.
const ST_AMMO0WIDTH: usize = 3;
const ST_AMMO0X: usize = 288;
const ST_AMMO0Y: usize = 173;
const ST_AMMO1WIDTH: usize = ST_AMMO0WIDTH;
const ST_AMMO1X: usize = 288;
const ST_AMMO1Y: usize = 179;
const ST_AMMO2WIDTH: usize = ST_AMMO0WIDTH;
const ST_AMMO2X: usize = 288;
const ST_AMMO2Y: usize = 191;
const ST_AMMO3WIDTH: usize = ST_AMMO0WIDTH;
const ST_AMMO3X: usize = 288;
const ST_AMMO3Y: usize = 185;

// Indicate maximum ammunition.
// Only needed because backpack exists.
const ST_MAXAMMO0WIDTH: usize = 3;
const ST_MAXAMMO0X: usize = 314;
const ST_MAXAMMO0Y: usize = 173;
const ST_MAXAMMO1WIDTH: usize = ST_MAXAMMO0WIDTH;
const ST_MAXAMMO1X: usize = 314;
const ST_MAXAMMO1Y: usize = 179;
const ST_MAXAMMO2WIDTH: usize = ST_MAXAMMO0WIDTH;
const ST_MAXAMMO2X: usize = 314;
const ST_MAXAMMO2Y: usize = 191;
const ST_MAXAMMO3WIDTH: usize = ST_MAXAMMO0WIDTH;
const ST_MAXAMMO3X: usize = 314;
const ST_MAXAMMO3Y: usize = 185;

// Dimensions given in characters.
const ST_MSGWIDTH: usize = 52;

type int = i32;
type unsigned = u32;

// main player in game
static mut plyr: *mut player_t = null_mut();

// ST_Start() has just been called
static mut st_firsttime: bool = false;

// used to execute ST_Init() only once
static mut veryfirsttime: int = 1;

// lump number for PLAYPAL
static mut lu_palette: usize = 0;

// used for timing
static mut st_clock: unsigned = 0;

// used for making messages go away
static mut st_msgcounter: int = 0;

// used when in chat
static mut st_chatstate: st_chatstateenum_t = st_chatstateenum_t::StartChatState;

// whether in automap or first-person
static mut st_gamestate: st_stateenum_t = st_stateenum_t::FirstPersonState;

// whether left-side main status bar is active
static mut st_statusbaron: bool = false;

// whether status bar chat is active
static mut st_chat: bool = false;

// value of st_chat before message popped up
static mut st_oldchat: bool = false;

// whether chat window has the cursor on
static mut st_cursoron: bool = false;

// !deathmatch
static mut st_notdeathmatch: bool = false;

// !deathmatch && st_statusbaron
static mut st_armson: bool = false;

// !deathmatch
static mut st_fragson: bool = false;

// main bar left
static mut sbar: *mut patch_t = null_mut();

// 0-9, tall numbers
static mut tallnum: [*mut patch_t; 10] = [null_mut(); 10];

// tall % sign
static mut tallpercent: *mut patch_t = null_mut();

// 0-9, short, yellow (,different!) numbers
static mut shortnum: [*mut patch_t; 10] = [null_mut(); 10];

// 3 key-cards, 3 skulls
static mut keys: [*mut patch_t; card_t::NUMCARDS.to_usize()] =
	[null_mut(); card_t::NUMCARDS.to_usize()];

// face status patches
static mut faces: [*mut patch_t; ST_NUMFACES] = [null_mut(); ST_NUMFACES];

// face background
static mut faceback: *mut patch_t = null_mut();

// main bar right
static mut armsbg: *mut patch_t = null_mut();

// weapon ownership patches
static mut arms: [[*mut patch_t; 2]; 6] = [[null_mut(); 2]; 6];

const NULL_ST_NUMBER_T: st_number_t = st_number_t {
	x: 0,
	y: 0,
	width: 0,
	oldnum: 0,
	num: null_mut(),
	on: null_mut(),
	p: null_mut(),
	data: 0,
};

// ready-weapon widget
static mut w_ready: st_number_t = NULL_ST_NUMBER_T;

// in deathmatch only, summary of frags stats
static mut w_frags: st_number_t = NULL_ST_NUMBER_T;

// health widget
static mut w_health: st_percent_t = st_percent_t { n: NULL_ST_NUMBER_T, p: null_mut() };

// arms background
static mut w_armsbg: st_binicon_t = st_binicon_t {
	x: 0,
	y: 0,
	oldval: false,
	val: null_mut(),
	on: null_mut(),
	p: null_mut(),
	data: 0,
};

const NULL_ST_MULTICON_T: st_multicon_t = st_multicon_t {
	x: 0,
	y: 0,
	oldinum: 0,
	inum: null_mut(),
	on: null_mut(),
	p: null_mut(),
	data: 0,
};

// weapon ownership widgets
static mut w_arms: [st_multicon_t; 6] = [NULL_ST_MULTICON_T; 6];

// face status widget
static mut w_faces: st_multicon_t = NULL_ST_MULTICON_T;

// keycard widgets
static mut w_keyboxes: [st_multicon_t; 3] = [NULL_ST_MULTICON_T; 3];

// armor widget
static mut w_armor: st_percent_t = st_percent_t { n: NULL_ST_NUMBER_T, p: null_mut() };

// ammo widgets
static mut w_ammo: [st_number_t; 4] = [NULL_ST_NUMBER_T; 4];

// max ammo widgets
static mut w_maxammo: [st_number_t; 4] = [NULL_ST_NUMBER_T; 4];

// number of frags so far in deathmatch
static mut st_fragscount: int = 0;

// used to use appopriately pained face
static mut st_oldhealth: int = -1;

// used for evil grin
static mut oldweaponsowned: [bool; weapontype_t::NUMWEAPONS.to_usize()] =
	[false; weapontype_t::NUMWEAPONS.to_usize()];

// count until face changes
static mut st_facecount: usize = 0;

// current face index, used by w_faces
static mut st_faceindex: usize = 0;

// holds key-type for each key box on bar
static mut keyboxes: [int; 3] = [0; 3];

// a random number per tick
static mut st_randomnumber: i32 = 0;

// Massive bunches of cheat shit
//  to keep it from being easy to figure them out.
// Yeah, right...
static mut cheat_mus_seq: [u8; 9] = [0xb2, 0x26, 0xb6, 0xae, 0xea, 1, 0, 0, 0xff];

static mut cheat_choppers_seq: [u8; 11] = [
	0xb2, 0x26, 0xe2, 0x32, 0xf6, 0x2a, 0x2a, 0xa6, 0x6a, 0xea, 0xff, // id...
];

static mut cheat_god_seq: [u8; 6] = [
	0xb2, 0x26, 0x26, 0xaa, 0x26, 0xff, // iddqd
];

static mut cheat_ammo_seq: [u8; 6] = [
	0xb2, 0x26, 0xf2, 0x66, 0xa2, 0xff, // idkfa
];

static mut cheat_ammonokey_seq: [u8; 5] = [
	0xb2, 0x26, 0x66, 0xa2, 0xff, // idfa
];

// Smashing Pumpkins Into Samml Piles Of Putried Debris.
static mut cheat_noclip_seq: [u8; 11] = [
	0xb2, 0x26, 0xea, 0x2a, 0xb2, // idspispopd
	0xea, 0x2a, 0xf6, 0x2a, 0x26, 0xff,
];

//
static mut cheat_commercial_noclip_seq: [u8; 7] = [
	0xb2, 0x26, 0xe2, 0x36, 0xb2, 0x2a, 0xff, // idclip
];

static mut cheat_powerup_seq: [[u8; 10]; 7] = [
	[0xb2, 0x26, 0x62, 0xa6, 0x32, 0xf6, 0x36, 0x26, 0x6e, 0xff], // beholdv
	[0xb2, 0x26, 0x62, 0xa6, 0x32, 0xf6, 0x36, 0x26, 0xea, 0xff], // beholds
	[0xb2, 0x26, 0x62, 0xa6, 0x32, 0xf6, 0x36, 0x26, 0xb2, 0xff], // beholdi
	[0xb2, 0x26, 0x62, 0xa6, 0x32, 0xf6, 0x36, 0x26, 0x6a, 0xff], // beholdr
	[0xb2, 0x26, 0x62, 0xa6, 0x32, 0xf6, 0x36, 0x26, 0xa2, 0xff], // beholda
	[0xb2, 0x26, 0x62, 0xa6, 0x32, 0xf6, 0x36, 0x26, 0x36, 0xff], // beholdl
	[0xb2, 0x26, 0x62, 0xa6, 0x32, 0xf6, 0x36, 0x26, 0xff, 0],    // behold
];

static mut cheat_clev_seq: [u8; 10] = [
	0xb2, 0x26, 0xe2, 0x36, 0xa6, 0x6e, 1, 0, 0, 0xff, // idclev
];

// my position cheat
static mut cheat_mypos_seq: [u8; 8] = [
	0xb2, 0x26, 0xb6, 0xba, 0x2a, 0xf6, 0xea, 0xff, // idmypos
];

// Now what?
static mut cheat_mus: cheatseq_t =
	cheatseq_t { sequence: unsafe { cheat_mus_seq.as_mut_ptr() }, p: null_mut() };
static mut cheat_god: cheatseq_t =
	cheatseq_t { sequence: unsafe { cheat_god_seq.as_mut_ptr() }, p: null_mut() };
static mut cheat_ammo: cheatseq_t =
	cheatseq_t { sequence: unsafe { cheat_ammo_seq.as_mut_ptr() }, p: null_mut() };
static mut cheat_ammonokey: cheatseq_t =
	cheatseq_t { sequence: unsafe { cheat_ammonokey_seq.as_mut_ptr() }, p: null_mut() };
static mut cheat_noclip: cheatseq_t =
	cheatseq_t { sequence: unsafe { cheat_noclip_seq.as_mut_ptr() }, p: null_mut() };
static mut cheat_commercial_noclip: cheatseq_t =
	cheatseq_t { sequence: unsafe { cheat_commercial_noclip_seq.as_mut_ptr() }, p: null_mut() };

static mut cheat_powerup: [cheatseq_t; 7] = [
	cheatseq_t { sequence: unsafe { cheat_powerup_seq[0].as_mut_ptr() }, p: null_mut() },
	cheatseq_t { sequence: unsafe { cheat_powerup_seq[1].as_mut_ptr() }, p: null_mut() },
	cheatseq_t { sequence: unsafe { cheat_powerup_seq[2].as_mut_ptr() }, p: null_mut() },
	cheatseq_t { sequence: unsafe { cheat_powerup_seq[3].as_mut_ptr() }, p: null_mut() },
	cheatseq_t { sequence: unsafe { cheat_powerup_seq[4].as_mut_ptr() }, p: null_mut() },
	cheatseq_t { sequence: unsafe { cheat_powerup_seq[5].as_mut_ptr() }, p: null_mut() },
	cheatseq_t { sequence: unsafe { cheat_powerup_seq[6].as_mut_ptr() }, p: null_mut() },
];

static mut cheat_choppers: cheatseq_t =
	cheatseq_t { sequence: unsafe { cheat_choppers_seq.as_mut_ptr() }, p: null_mut() };
static mut cheat_clev: cheatseq_t =
	cheatseq_t { sequence: unsafe { cheat_clev_seq.as_mut_ptr() }, p: null_mut() };
static mut cheat_mypos: cheatseq_t =
	cheatseq_t { sequence: unsafe { cheat_mypos_seq.as_mut_ptr() }, p: null_mut() };

// STATUS BAR CODE

fn ST_refreshBackground() {
	unsafe {
		if st_statusbaron {
			V_DrawPatch(ST_X, 0, BG, sbar);

			if netgame {
				V_DrawPatch(ST_FX, 0, BG, faceback);
			}

			V_CopyRect(ST_X, 0, BG, ST_WIDTH, ST_HEIGHT, ST_X, ST_Y, FG);
		}
	}
}

// Respond to keyboard input events,
//  intercept cheats.
#[allow(static_mut_refs)]
pub(crate) fn ST_Responder(ev: &mut event_t) -> bool {
	unsafe {
		// Filter automap on/off.
		if ev.ty == evtype_t::ev_keyup
			&& (usize::try_from(ev.data1).unwrap() & 0xffff0000) == AM_MSGHEADER
		{
			match usize::try_from(ev.data1).unwrap() {
				AM_MSGENTERED => {
					st_gamestate = st_stateenum_t::AutomapState;
					st_firsttime = true;
				}

				AM_MSGEXITED => {
					//	fprintf(stderr, "AM exited\n");
					st_gamestate = st_stateenum_t::FirstPersonState;
				}

				_ => (),
			}
		}
		// if a user keypress...
		else if ev.ty == evtype_t::ev_keydown {
			if !netgame {
				// b. - enabled for more debug fun.
				// if (gameskill != sk_nightmare) {

				// 'dqd' cheat for toggleable god mode
				if cht_CheckCheat(&mut cheat_god, u8::try_from(ev.data1).unwrap()) {
					(*plyr).cheats ^= CF_GODMODE;
					if (*plyr).cheats & CF_GODMODE != 0 {
						if !(*plyr).mo.is_null() {
							(*plyr).mo_mut().health = 100;
						}

						(*plyr).health = 100;
						(*plyr).message = STSTR_DQDON;
					} else {
						(*plyr).message = STSTR_DQDOFF;
					}
				}
				// 'fa' cheat for killer fucking arsenal
				else if cht_CheckCheat(&mut cheat_ammonokey, u8::try_from(ev.data1).unwrap()) {
					(*plyr).armorpoints = 200;
					(*plyr).armortype = 2;

					for i in 0..usize::from(weapontype_t::NUMWEAPONS) {
						(*plyr).weaponowned[i] = 1;
					}

					for i in 0..usize::from(ammotype_t::NUMAMMO) {
						(*plyr).ammo[i] = (*plyr).maxammo[i];
					}

					(*plyr).message = STSTR_FAADDED;
				}
				// 'kfa' cheat for key full ammo
				else if cht_CheckCheat(&mut cheat_ammo, u8::try_from(ev.data1).unwrap()) {
					(*plyr).armorpoints = 200;
					(*plyr).armortype = 2;

					for i in 0..usize::from(weapontype_t::NUMWEAPONS) {
						(*plyr).weaponowned[i] = 1;
					}

					for i in 0..usize::from(ammotype_t::NUMAMMO) {
						(*plyr).ammo[i] = (*plyr).maxammo[i];
					}

					for i in 0..usize::from(card_t::NUMCARDS) {
						(*plyr).cards[i] = 1;
					}

					(*plyr).message = STSTR_KFAADDED;
				}
				// 'mus' cheat for changing music
				else if cht_CheckCheat(&mut cheat_mus, u8::try_from(ev.data1).unwrap()) {
					(*plyr).message = STSTR_MUS;
					let mut buf = [0; 3];
					cht_GetParam(&mut cheat_mus, buf.as_mut_ptr());

					if gamemode == GameMode_t::commercial {
						let offset = (buf[0] - b'0') * 10 + buf[1] - b'0';
						let musnum = isize::from(musicenum_t::mus_runnin) + isize::from(offset - 1);

						if offset > 35 {
							(*plyr).message = STSTR_NOMUS;
						} else {
							S_ChangeMusic(musnum.into(), true);
						}
					} else {
						let offset = (buf[0] - b'1') * 9 + (buf[1] - b'1');
						let musnum = (isize::from(musicenum_t::mus_e1m1)) + isize::from(offset);

						if offset > 31 {
							(*plyr).message = STSTR_NOMUS;
						} else {
							S_ChangeMusic(musnum.into(), true);
						}
					}
				}
				// Simplified, accepting both "noclip" and "idspispopd".
				// no clipping mode cheat
				else if cht_CheckCheat(&mut cheat_noclip, u8::try_from(ev.data1).unwrap())
					|| cht_CheckCheat(&mut cheat_commercial_noclip, u8::try_from(ev.data1).unwrap())
				{
					(*plyr).cheats ^= CF_NOCLIP;

					if (*plyr).cheats & CF_NOCLIP != 0 {
						(*plyr).message = STSTR_NCON;
					} else {
						(*plyr).message = STSTR_NCOFF;
					}
				}
				// 'behold?' power-up cheats
				#[allow(clippy::needless_range_loop)]
				for i in 0..6 {
					if cht_CheckCheat(&mut cheat_powerup[i], u8::try_from(ev.data1).unwrap()) {
						if (*plyr).powers[i] == 0 {
							P_GivePower(&mut *plyr, i.into());
						} else if powertype_t::from(i) != powertype_t::pw_strength {
							(*plyr).powers[i] = 1;
						} else {
							(*plyr).powers[i] = 0;
						}

						(*plyr).message = STSTR_BEHOLDX;
					}
				}

				// 'behold' power-up menu
				if cht_CheckCheat(&mut cheat_powerup[6], u8::try_from(ev.data1).unwrap()) {
					(*plyr).message = STSTR_BEHOLD;
				}
				// 'choppers' invulnerability & chainsaw
				else if cht_CheckCheat(&mut cheat_choppers, u8::try_from(ev.data1).unwrap()) {
					(*plyr).weaponowned[usize::from(weapontype_t::wp_chainsaw)] = 1;
					(*plyr).powers[usize::from(powertype_t::pw_invulnerability)] = 1;
					(*plyr).message = STSTR_CHOPPERS;
				}
				// 'mypos' for player position
				else if cht_CheckCheat(&mut cheat_mypos, u8::try_from(ev.data1).unwrap()) {
					static mut buf: [i8; ST_MSGWIDTH] = [0; ST_MSGWIDTH];
					libc::sprintf(
						buf.as_mut_ptr(),
						c"ang=0x%x;x,y=(0x%x,0x%x)".as_ptr(),
						players[consoleplayer].mo().angle,
						players[consoleplayer].mo().x,
						players[consoleplayer].mo().y,
					);
					(*plyr).message = buf.as_ptr();
				}
			}

			// 'clev' change-level cheat
			if cht_CheckCheat(&mut cheat_clev, u8::try_from(ev.data1).unwrap()) {
				let epsd;
				let map;

				let mut buf = [0; 3];
				cht_GetParam(&mut cheat_clev, buf.as_mut_ptr());

				if gamemode == GameMode_t::commercial {
					epsd = 0;
					map = (buf[0] - b'0') * 10 + buf[1] - b'0';
				} else {
					epsd = buf[0] - b'0';
					map = buf[1] - b'0';
				}

				// Catch invalid maps.
				if epsd < 1 {
					return false;
				}

				if map < 1 {
					return false;
				}

				// Ohmygod - this is not going to work.
				if gamemode == GameMode_t::retail && epsd > 4 || map > 9 {
					return false;
				}

				if gamemode == GameMode_t::registered && epsd > 3 || map > 9 {
					return false;
				}

				if gamemode == GameMode_t::shareware && epsd > 1 || map > 9 {
					return false;
				}

				if gamemode == GameMode_t::commercial && epsd > 1 || map > 34 {
					return false;
				}

				// So be it.
				(*plyr).message = STSTR_CLEV;
				G_DeferedInitNew(gameskill, usize::from(epsd), usize::from(map));
			}
		}
		false
	}
}

fn ST_calcPainOffset() -> usize {
	unsafe {
		static mut lastcalc: usize = 0;
		static mut oldhealth: i32 = -1;

		let health = i32::min((*plyr).health, 100);

		if health != oldhealth {
			lastcalc =
				ST_FACESTRIDE * ((usize::try_from(100 - health).unwrap() * ST_NUMPAINFACES) / 101);
			oldhealth = health;
		}
		lastcalc
	}
}

// This is a not-very-pretty routine which handles
//  the face states and their timing.
// the precedence of expressions is:
//  dead > evil grin > turned head > straight ahead
fn ST_updateFaceWidget() {
	unsafe {
		static mut lastattackdown: int = -1;
		static mut priority: int = 0;

		if priority < 10 {
			// dead
			if (*plyr).health == 0 {
				priority = 9;
				st_faceindex = ST_DEADFACE;
				st_facecount = 1;
			}
		}

		if priority < 9 {
			if (*plyr).bonuscount != 0 {
				// picking up bonus
				let mut doevilgrin = false;

				#[allow(clippy::needless_range_loop)]
				for i in 0..usize::from(weapontype_t::NUMWEAPONS) {
					if oldweaponsowned[i] != ((*plyr).weaponowned[i] != 0) {
						doevilgrin = true;
						oldweaponsowned[i] = (*plyr).weaponowned[i] != 0;
					}
				}

				if doevilgrin {
					// evil grin if just picked up weapon
					priority = 8;
					st_facecount = ST_EVILGRINCOUNT;
					st_faceindex = ST_calcPainOffset() + ST_EVILGRINOFFSET;
				}
			}
		}

		if priority < 8 {
			if (*plyr).damagecount != 0
				&& !(*plyr).attacker.is_null()
				&& !ptr::eq((*plyr).attacker, (*plyr).mo)
			{
				// being attacked
				priority = 7;

				if (*plyr).health - st_oldhealth > ST_MUCHPAIN {
					st_facecount = ST_TURNCOUNT;
					st_faceindex = ST_calcPainOffset() + ST_OUCHOFFSET;
				} else {
					let badguyangle = R_PointToAngle2(
						(*plyr).mo().x,
						(*plyr).mo().y,
						(*(*plyr).attacker).x,
						(*(*plyr).attacker).y,
					);

					let diffang;
					let i;
					if badguyangle > (*plyr).mo().angle {
						// whether right or left
						diffang = badguyangle - (*plyr).mo().angle;
						i = diffang > ANG180;
					} else {
						// whether left or right
						diffang = (*plyr).mo().angle - badguyangle;
						i = diffang <= ANG180;
					} // confusing, aint it?

					st_facecount = ST_TURNCOUNT;
					st_faceindex = ST_calcPainOffset();

					if diffang < ANG45 {
						// head-on
						st_faceindex += ST_RAMPAGEOFFSET;
					} else if i {
						// turn face right
						st_faceindex += ST_TURNOFFSET;
					} else {
						// turn face left
						st_faceindex += ST_TURNOFFSET + 1;
					}
				}
			}
		}

		if priority < 7 {
			// getting hurt because of your own damn stupidity
			if (*plyr).damagecount != 0 {
				if (*plyr).health - st_oldhealth > ST_MUCHPAIN {
					priority = 7;
					st_facecount = ST_TURNCOUNT;
					st_faceindex = ST_calcPainOffset() + ST_OUCHOFFSET;
				} else {
					priority = 6;
					st_facecount = ST_TURNCOUNT;
					st_faceindex = ST_calcPainOffset() + ST_RAMPAGEOFFSET;
				}
			}
		}

		if priority < 6 {
			// rapid firing
			if (*plyr).attackdown != 0 {
				if lastattackdown == -1 {
					lastattackdown = i32::try_from(ST_RAMPAGEDELAY).unwrap();
				} else {
					lastattackdown -= 1;
					if lastattackdown == 0 {
						priority = 5;
						st_faceindex = ST_calcPainOffset() + ST_RAMPAGEOFFSET;
						st_facecount = 1;
						lastattackdown = 1;
					}
				}
			} else {
				lastattackdown = -1;
			}
		}

		if priority < 5 {
			// invulnerability
			if (*plyr).cheats & CF_GODMODE != 0
				|| (*plyr).powers[usize::from(powertype_t::pw_invulnerability)] != 0
			{
				priority = 4;

				st_faceindex = ST_GODFACE;
				st_facecount = 1;
			}
		}

		// look left or look right if the facecount has timed out
		if st_facecount == 0 {
			st_faceindex = ST_calcPainOffset() + usize::try_from(st_randomnumber).unwrap() % 3;
			st_facecount = ST_STRAIGHTFACECOUNT;
			priority = 0;
		}

		st_facecount -= 1;
	}
}

fn ST_updateWidgets() {
	unsafe {
		static mut largeammo: i32 = 1994; // means "n/a"

		// must redirect the pointer if the ready weapon has changed.
		if weaponinfo[usize::from((*plyr).readyweapon)].ammo == ammotype_t::am_noammo {
			w_ready.num = &raw mut largeammo;
		} else {
			w_ready.num = (&raw mut (*plyr).ammo
				[usize::from(weaponinfo[usize::from((*plyr).readyweapon)].ammo)])
				.cast();
		}
		w_ready.data = i32::from((*plyr).readyweapon);

		// update keycard multiple widgets
		#[allow(clippy::needless_range_loop)]
		for i in 0..3 {
			keyboxes[i] = if (*plyr).cards[i] != 0 { i32::try_from(i).unwrap() } else { -1 };

			if (*plyr).cards[i + 3] != 0 {
				keyboxes[i] = i32::try_from(i).unwrap() + 3;
			}
		}

		// refresh everything if this is him coming back to life
		ST_updateFaceWidget();

		// used by the w_armsbg widget
		st_notdeathmatch = deathmatch == 0;

		// used by w_arms[] widgets
		st_armson = st_statusbaron && deathmatch == 0;

		// used by w_frags widget
		st_fragson = st_statusbaron && deathmatch != 0;
		st_fragscount = 0;

		for i in 0..MAXPLAYERS {
			if i != consoleplayer {
				st_fragscount += (*plyr).frags[i];
			} else {
				st_fragscount -= (*plyr).frags[i];
			}
		}

		// get rid of chat window if up because of message
		st_msgcounter -= 1;
		if st_msgcounter == 0 {
			st_chat = st_oldchat;
		}
	}
}

pub(crate) fn ST_Ticker() {
	unsafe {
		st_clock += 1;
		st_randomnumber = M_Random();
		ST_updateWidgets();
		st_oldhealth = (*plyr).health;
	}
}

static mut st_palette: isize = 0;

fn ST_doPaletteStuff() {
	unsafe {
		let mut cnt = usize::try_from((*plyr).damagecount).unwrap();

		if (*plyr).powers[usize::from(powertype_t::pw_strength)] != 0 {
			// slowly fade the berzerk out
			let bzc =
				12usize.saturating_sub((*plyr).powers[usize::from(powertype_t::pw_strength)] >> 6);

			if bzc > cnt {
				cnt = bzc;
			}
		}

		let mut palette;
		if cnt != 0 {
			palette = (cnt + 7) >> 3;

			if palette >= NUMREDPALS {
				palette = NUMREDPALS - 1;
			}

			palette += STARTREDPALS;
		} else if (*plyr).bonuscount != 0 {
			palette = ((*plyr).bonuscount + 7) >> 3;

			if palette >= NUMBONUSPALS {
				palette = NUMBONUSPALS - 1;
			}

			palette += STARTBONUSPALS;
		} else if (*plyr).powers[usize::from(powertype_t::pw_ironfeet)] > 4 * 32
			|| (*plyr).powers[usize::from(powertype_t::pw_ironfeet)] & 8 != 0
		{
			palette = RADIATIONPAL;
		} else {
			palette = 0;
		}

		if isize::try_from(palette).unwrap() != st_palette {
			st_palette = isize::try_from(palette).unwrap();
			let pal = W_CacheLumpNum(lu_palette, PU_CACHE).wrapping_byte_add(palette * 768);
			I_SetPalette(pal.cast());
		}
	}
}

fn ST_drawWidgets(refresh: bool) {
	unsafe {
		// used by w_arms[] widgets
		st_armson = st_statusbaron && deathmatch == 0;

		// used by w_frags widget
		st_fragson = deathmatch != 0 && st_statusbaron;

		STlib_updateNum(&mut w_ready, refresh);

		for i in 0..4 {
			STlib_updateNum(&mut w_ammo[i], refresh);
			STlib_updateNum(&mut w_maxammo[i], refresh);
		}

		STlib_updatePercent(&mut w_health, refresh);
		STlib_updatePercent(&mut w_armor, refresh);

		STlib_updateBinIcon(&mut w_armsbg, refresh);

		#[allow(clippy::needless_range_loop)]
		for i in 0..6 {
			STlib_updateMultIcon(&mut w_arms[i], refresh);
		}

		STlib_updateMultIcon(&mut w_faces, refresh);

		#[allow(clippy::needless_range_loop)]
		for i in 0..3 {
			STlib_updateMultIcon(&mut w_keyboxes[i], refresh);
		}

		STlib_updateNum(&mut w_frags, refresh);
	}
}

fn ST_doRefresh() {
	unsafe { st_firsttime = false };

	// draw status bar background to off-screen buff
	ST_refreshBackground();

	// and refresh all widgets
	ST_drawWidgets(true);
}

fn ST_diffDraw() {
	// update all widgets
	ST_drawWidgets(false);
}

pub(crate) fn ST_Drawer(fullscreen: bool, refresh: bool) {
	unsafe {
		st_statusbaron = !fullscreen || automapactive;
		st_firsttime = st_firsttime || refresh;

		// Do red-/gold-shifts from damage/items
		ST_doPaletteStuff();

		if st_firsttime {
			// If just after ST_Start(), refresh all
			ST_doRefresh();
		} else {
			// Otherwise, update as little as possible
			ST_diffDraw();
		}
	}
}

fn ST_loadGraphics() {
	unsafe {
		// int		i;
		// int		j;
		// int		facenum;

		// char	namebuf[9];
		let mut namebuf = [0; 9];

		// Load the numbers, tall and short
		for i in 0..10 {
			libc::sprintf(namebuf.as_mut_ptr(), c"STTNUM%d".as_ptr(), i);
			tallnum[i] = W_CacheLumpName(namebuf.as_ptr(), PU_STATIC).cast();

			libc::sprintf(namebuf.as_mut_ptr(), c"STYSNUM%d".as_ptr(), i);
			shortnum[i] = W_CacheLumpName(namebuf.as_ptr(), PU_STATIC).cast();
		}

		// Load percent key.
		//Note: why not load STMINUS here, too?
		tallpercent = W_CacheLumpName(c"STTPRCNT".as_ptr(), PU_STATIC).cast();

		// key cards
		#[allow(clippy::needless_range_loop)]
		for i in 0..usize::from(card_t::NUMCARDS) {
			libc::sprintf(namebuf.as_mut_ptr(), c"STKEYS%d".as_ptr(), i);
			keys[i] = W_CacheLumpName(namebuf.as_ptr(), PU_STATIC).cast();
		}

		// arms background
		armsbg = W_CacheLumpName(c"STARMS".as_ptr(), PU_STATIC).cast();

		// arms ownership widgets
		for i in 0..6 {
			libc::sprintf(namebuf.as_mut_ptr(), c"STGNUM%d".as_ptr(), i + 2);

			// gray #
			arms[i][0] = W_CacheLumpName(namebuf.as_ptr(), PU_STATIC).cast();

			// yellow #
			arms[i][1] = shortnum[i + 2];
		}

		// face backgrounds for different color players
		libc::sprintf(namebuf.as_mut_ptr(), c"STFB%d".as_ptr(), consoleplayer);
		faceback = W_CacheLumpName(namebuf.as_ptr(), PU_STATIC).cast();

		// status bar background bits
		sbar = W_CacheLumpName(c"STBAR".as_ptr(), PU_STATIC).cast();

		// face states
		let mut facenum = 0;
		for i in 0..ST_NUMPAINFACES {
			for j in 0..ST_NUMSTRAIGHTFACES {
				libc::sprintf(namebuf.as_mut_ptr(), c"STFST%d%d".as_ptr(), i, j);
				faces[facenum] = W_CacheLumpName(namebuf.as_ptr(), PU_STATIC).cast();
				facenum += 1;
			}
			libc::sprintf(namebuf.as_mut_ptr(), c"STFTR%d0".as_ptr(), i); // turn right
			faces[facenum] = W_CacheLumpName(namebuf.as_ptr(), PU_STATIC).cast();
			facenum += 1;
			libc::sprintf(namebuf.as_mut_ptr(), c"STFTL%d0".as_ptr(), i); // turn left
			faces[facenum] = W_CacheLumpName(namebuf.as_ptr(), PU_STATIC).cast();
			facenum += 1;
			libc::sprintf(namebuf.as_mut_ptr(), c"STFOUCH%d".as_ptr(), i); // ouch!
			faces[facenum] = W_CacheLumpName(namebuf.as_ptr(), PU_STATIC).cast();
			facenum += 1;
			libc::sprintf(namebuf.as_mut_ptr(), c"STFEVL%d".as_ptr(), i); // evil grin ;
			faces[facenum] = W_CacheLumpName(namebuf.as_ptr(), PU_STATIC).cast();
			facenum += 1;
			libc::sprintf(namebuf.as_mut_ptr(), c"STFKILL%d".as_ptr(), i); // pissed off
			faces[facenum] = W_CacheLumpName(namebuf.as_ptr(), PU_STATIC).cast();
			facenum += 1;
		}
		faces[facenum] = W_CacheLumpName(c"STFGOD0".as_ptr(), PU_STATIC).cast();
		facenum += 1;
		faces[facenum] = W_CacheLumpName(c"STFDEAD0".as_ptr(), PU_STATIC).cast();
	}
}

fn ST_loadData() {
	unsafe {
		lu_palette = usize::try_from(W_GetNumForName(c"PLAYPAL".as_ptr())).unwrap();
		ST_loadGraphics();
	}
}

fn ST_initData() {
	unsafe {
		st_firsttime = true;
		plyr = &raw mut players[consoleplayer];

		st_clock = 0;
		st_chatstate = st_chatstateenum_t::StartChatState;
		st_gamestate = st_stateenum_t::FirstPersonState;

		st_statusbaron = true;
		st_oldchat = false;
		st_chat = false;
		st_cursoron = false;

		st_faceindex = 0;
		st_palette = -1;

		st_oldhealth = -1;

		#[allow(clippy::needless_range_loop)]
		for i in 0..usize::from(weapontype_t::NUMWEAPONS) {
			oldweaponsowned[i] = (*plyr).weaponowned[i] != 0;
		}

		#[allow(clippy::needless_range_loop)]
		for i in 0..3 {
			keyboxes[i] = -1;
		}

		STlib_init();
	}
}

fn ST_createWidgets() {
	unsafe {
		// ready weapon ammo
		STlib_initNum(
			&mut w_ready,
			ST_AMMOX,
			ST_AMMOY,
			tallnum.as_mut_ptr(),
			(&raw mut (*plyr).ammo[usize::from(weaponinfo[usize::from((*plyr).readyweapon)].ammo)])
				.cast(),
			&raw mut st_statusbaron,
			ST_AMMOWIDTH,
		);

		// the last weapon type
		w_ready.data = i32::from((*plyr).readyweapon);

		// health percentage
		STlib_initPercent(
			&mut w_health,
			ST_HEALTHX,
			ST_HEALTHY,
			tallnum.as_mut_ptr(),
			&raw mut (*plyr).health,
			&raw mut st_statusbaron,
			tallpercent,
		);

		// arms background
		STlib_initBinIcon(
			&mut w_armsbg,
			ST_ARMSBGX,
			ST_ARMSBGY,
			armsbg,
			&mut st_notdeathmatch,
			&mut st_statusbaron,
		);

		// weapons owned
		for i in 0..6 {
			STlib_initMultIcon(
				&mut w_arms[i],
				ST_ARMSX + (i % 3) * ST_ARMSXSPACE,
				ST_ARMSY + (i / 3) * ST_ARMSYSPACE,
				arms[i].as_mut_ptr(),
				&mut (*plyr).weaponowned[i + 1],
				&mut st_armson,
			);
		}

		// frags sum
		STlib_initNum(
			&mut w_frags,
			ST_FRAGSX,
			ST_FRAGSY,
			tallnum.as_mut_ptr(),
			&mut st_fragscount,
			&mut st_fragson,
			ST_FRAGSWIDTH,
		);

		// faces
		STlib_initMultIcon(
			&mut w_faces,
			ST_FACESX,
			ST_FACESY,
			faces.as_mut_ptr(),
			(&raw mut st_faceindex).cast(),
			&mut st_statusbaron,
		);

		// armor percentage - should be colored later
		STlib_initPercent(
			&mut w_armor,
			ST_ARMORX,
			ST_ARMORY,
			tallnum.as_mut_ptr(),
			&mut (*plyr).armorpoints,
			&mut st_statusbaron,
			tallpercent,
		);

		// keyboxes 0-2
		STlib_initMultIcon(
			&mut w_keyboxes[0],
			ST_KEY0X,
			ST_KEY0Y,
			keys.as_mut_ptr(),
			&mut keyboxes[0],
			&mut st_statusbaron,
		);

		STlib_initMultIcon(
			&mut w_keyboxes[1],
			ST_KEY1X,
			ST_KEY1Y,
			keys.as_mut_ptr(),
			&mut keyboxes[1],
			&mut st_statusbaron,
		);

		STlib_initMultIcon(
			&mut w_keyboxes[2],
			ST_KEY2X,
			ST_KEY2Y,
			keys.as_mut_ptr(),
			&mut keyboxes[2],
			&mut st_statusbaron,
		);

		// ammo count (all four kinds)
		STlib_initNum(
			&mut w_ammo[0],
			ST_AMMO0X,
			ST_AMMO0Y,
			shortnum.as_mut_ptr(),
			(&raw mut (*plyr).ammo[0]).cast(),
			&mut st_statusbaron,
			ST_AMMO0WIDTH,
		);

		STlib_initNum(
			&mut w_ammo[1],
			ST_AMMO1X,
			ST_AMMO1Y,
			shortnum.as_mut_ptr(),
			(&raw mut (*plyr).ammo[1]).cast(),
			&mut st_statusbaron,
			ST_AMMO1WIDTH,
		);

		STlib_initNum(
			&mut w_ammo[2],
			ST_AMMO2X,
			ST_AMMO2Y,
			shortnum.as_mut_ptr(),
			(&raw mut (*plyr).ammo[2]).cast(),
			&mut st_statusbaron,
			ST_AMMO2WIDTH,
		);

		STlib_initNum(
			&mut w_ammo[3],
			ST_AMMO3X,
			ST_AMMO3Y,
			shortnum.as_mut_ptr(),
			(&raw mut (*plyr).ammo[3]).cast(),
			&mut st_statusbaron,
			ST_AMMO3WIDTH,
		);

		// max ammo count (all four kinds)
		STlib_initNum(
			&mut w_maxammo[0],
			ST_MAXAMMO0X,
			ST_MAXAMMO0Y,
			shortnum.as_mut_ptr(),
			(&raw mut (*plyr).maxammo[0]).cast(),
			&mut st_statusbaron,
			ST_MAXAMMO0WIDTH,
		);

		STlib_initNum(
			&mut w_maxammo[1],
			ST_MAXAMMO1X,
			ST_MAXAMMO1Y,
			shortnum.as_mut_ptr(),
			(&raw mut (*plyr).maxammo[1]).cast(),
			&mut st_statusbaron,
			ST_MAXAMMO1WIDTH,
		);

		STlib_initNum(
			&mut w_maxammo[2],
			ST_MAXAMMO2X,
			ST_MAXAMMO2Y,
			shortnum.as_mut_ptr(),
			(&raw mut (*plyr).maxammo[2]).cast(),
			&mut st_statusbaron,
			ST_MAXAMMO2WIDTH,
		);

		STlib_initNum(
			&mut w_maxammo[3],
			ST_MAXAMMO3X,
			ST_MAXAMMO3Y,
			shortnum.as_mut_ptr(),
			(&raw mut (*plyr).maxammo[3]).cast(),
			&mut st_statusbaron,
			ST_MAXAMMO3WIDTH,
		);
	}
}

static mut st_stopped: bool = true;

pub(crate) fn ST_Start() {
	unsafe {
		if !st_stopped {
			ST_Stop();
		}

		ST_initData();
		ST_createWidgets();
		st_stopped = false;
	}
}

fn ST_Stop() {
	unsafe {
		if st_stopped {
			return;
		}

		I_SetPalette(W_CacheLumpNum(lu_palette, PU_CACHE).cast());

		st_stopped = true;
	}
}

pub(crate) fn ST_Init() {
	unsafe {
		veryfirsttime = 0;
		ST_loadData();
		screens[4] = Z_Malloc(ST_WIDTH * ST_HEIGHT, PU_STATIC, null_mut()).cast();
	}
}
