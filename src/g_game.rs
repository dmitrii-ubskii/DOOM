#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::{
	ffi::{c_char, c_int, c_void},
	ptr::{null, null_mut},
};

use crate::{
	d_englsh::GGSAVED,
	d_event::{
		BT_ATTACK, BT_CHANGE, BT_SPECIAL, BT_SPECIALMASK, BT_USE, BT_WEAPONSHIFT, BTS_PAUSE,
		BTS_SAVEGAME, BTS_SAVEMASK, BTS_SAVESHIFT, event_t, evtype_t, gameaction_t,
	},
	d_main::{
		D_AdvanceDemo, D_PageTicker, fastparm, nomonsters, respawnparm, singletics, wipegamestate,
	},
	d_net::BACKUPTICS,
	d_player::{player_t, playerstate_t, wbplayerstruct_t, wbstartstruct_t},
	d_ticcmd::ticcmd_t,
	doomdata::mapthing_t,
	doomdef::{
		GameMission_t, GameMode_t, KEY_F12, KEY_PAUSE, MAXPLAYERS, VERSION, ammotype_t,
		gamestate_t, skill_t, weapontype_t,
	},
	doomstat::{gamemission, gamemode},
	dstrings::SAVEGAMENAME,
	i_system::{I_BaseTiccmd, I_Error, I_GetTime, I_Quit},
	info::{mobjinfo, mobjtype_t, statenum_t, states},
	m_argv::M_CheckParm,
	m_fixed::{FRACBITS, FRACUNIT, fixed_t},
	m_random::{M_ClearRandom, P_Random, rndindex},
	myargc, myargv,
	p_local::MAXHEALTH,
	p_mobj::{MF_SHADOW, mobj_t},
	p_setup::{P_SetupLevel, deathmatch_p, deathmatchstarts, playerstarts},
	p_tick::leveltime,
	r_defs::subsector_t,
	r_sky::{SKYFLATNAME, skyflatnum, skytexture},
	sounds::sfxenum_t,
	tables::{ANG45, ANGLETOFINESHIFT, angle_t, finecos, finesine},
	v_video::screens,
	w_wad::{W_CacheLumpName, W_CheckNumForName},
	z_zone::{PU_CACHE, PU_STATIC, Z_ChangeTag, Z_CheckHeap, Z_Free, Z_Malloc},
};

type byte = u8;
type short = i16;
type int = i32;
type boolean = i32;

pub const SAVEGAMESIZE: usize = 0x2c000;
pub const SAVESTRINGSIZE: usize = 24;

#[unsafe(no_mangle)]
pub static mut gameaction: gameaction_t = gameaction_t::ga_nothing;
#[unsafe(no_mangle)]
pub static mut gamestate: gamestate_t = gamestate_t::GS_LEVEL;
#[unsafe(no_mangle)]
pub static mut gameskill: skill_t = skill_t::sk_baby;
#[unsafe(no_mangle)]
pub static mut respawnmonsters: boolean = 0;
#[unsafe(no_mangle)]
pub static mut gameepisode: usize = 0;
#[unsafe(no_mangle)]
pub static mut gamemap: usize = 0;

#[unsafe(no_mangle)]
pub static mut paused: boolean = 0;
#[unsafe(no_mangle)]
pub static mut sendpause: boolean = 0; // send a pause event next tic 
#[unsafe(no_mangle)]
pub static mut sendsave: boolean = 0; // send a save event next tic 
#[unsafe(no_mangle)]
pub static mut usergame: boolean = 0; // ok to save / end game 

#[unsafe(no_mangle)]
pub static mut timingdemo: boolean = 0; // if true, exit with report on completion 
#[unsafe(no_mangle)]
pub static mut nodrawers: boolean = 0; // for comparative timing purposes 
#[unsafe(no_mangle)]
pub static mut noblit: boolean = 0; // for comparative timing purposes 
#[unsafe(no_mangle)]
pub static mut starttime: int = 0; // for comparative timing purposes  	 

#[unsafe(no_mangle)]
pub static mut viewactive: boolean = 0;

#[unsafe(no_mangle)]
pub static mut deathmatch: boolean = 0; // only if started as net death 
#[unsafe(no_mangle)]
pub static mut netgame: boolean = 0; // only true if packets are broadcast 
#[unsafe(no_mangle)]
pub static mut playeringame: [boolean; MAXPLAYERS] = [0; MAXPLAYERS];
#[unsafe(no_mangle)]
pub static mut players: [player_t; MAXPLAYERS] = [player_t::new(); MAXPLAYERS];

#[unsafe(no_mangle)]
pub static mut consoleplayer: usize = 0; // player taking events and displaying 
#[unsafe(no_mangle)]
pub static mut displayplayer: usize = 0; // view being displayed 
#[unsafe(no_mangle)]
pub static mut gametic: int = 0;
#[unsafe(no_mangle)]
pub static mut levelstarttic: int = 0; // gametic at level start 

// for intermission
#[unsafe(no_mangle)]
pub static mut totalkills: int = 0;
#[unsafe(no_mangle)]
pub static mut totalitems: int = 0;
#[unsafe(no_mangle)]
pub static mut totalsecret: int = 0;

#[unsafe(no_mangle)]
pub static mut demoname: [c_char; 32] = [0; 32];
#[unsafe(no_mangle)]
pub static mut demorecording: boolean = 0;
#[unsafe(no_mangle)]
pub static mut demoplayback: boolean = 0;
#[unsafe(no_mangle)]
pub static mut netdemo: boolean = 0;
#[unsafe(no_mangle)]
pub static mut demobuffer: *mut byte = null_mut();
#[unsafe(no_mangle)]
pub static mut demo_p: *mut byte = null_mut();
#[unsafe(no_mangle)]
pub static mut demoend: *mut byte = null_mut();
#[unsafe(no_mangle)]
pub static mut singledemo: boolean = 0; // quit after playing a demo from cmdline 

#[unsafe(no_mangle)]
pub static mut precache: boolean = 1; // if true, load all graphics at start 

#[unsafe(no_mangle)]
pub static mut wminfo: wbstartstruct_t = wbstartstruct_t {
	epsd: 0,
	didsecret: 0,
	last: 0,
	next: 0,
	maxkills: 0,
	maxitems: 0,
	maxsecret: 0,
	maxfrags: 0,
	partime: 0,
	pnum: 0,
	plyr: [wbplayerstruct_t {
		in_: 0,
		skills: 0,
		sitems: 0,
		ssecret: 0,
		stime: 0,
		frags: [0; 4],
		score: 0,
	}; 4],
}; // parms for world map / intermission 

#[unsafe(no_mangle)]
pub static mut consistancy: [[short; BACKUPTICS]; MAXPLAYERS] = [[0; BACKUPTICS]; MAXPLAYERS];

#[unsafe(no_mangle)]
pub static mut savebuffer: *mut byte = null_mut();

// controls (have defaults)
#[unsafe(no_mangle)]
pub static mut key_right: usize = 0;
#[unsafe(no_mangle)]
pub static mut key_left: usize = 0;

#[unsafe(no_mangle)]
pub static mut key_up: usize = 0;
#[unsafe(no_mangle)]
pub static mut key_down: usize = 0;
#[unsafe(no_mangle)]
pub static mut key_strafeleft: usize = 0;
#[unsafe(no_mangle)]
pub static mut key_straferight: usize = 0;
#[unsafe(no_mangle)]
pub static mut key_fire: usize = 0;
#[unsafe(no_mangle)]
pub static mut key_use: usize = 0;
#[unsafe(no_mangle)]
pub static mut key_strafe: usize = 0;
#[unsafe(no_mangle)]
pub static mut key_speed: usize = 0;

#[unsafe(no_mangle)]
pub static mut mousebfire: isize = 0;
#[unsafe(no_mangle)]
pub static mut mousebstrafe: isize = 0;
#[unsafe(no_mangle)]
pub static mut mousebforward: isize = 0;

#[unsafe(no_mangle)]
pub static mut joybfire: isize = 0;
#[unsafe(no_mangle)]
pub static mut joybstrafe: isize = 0;
#[unsafe(no_mangle)]
pub static mut joybuse: isize = 0;
#[unsafe(no_mangle)]
pub static mut joybspeed: isize = 0;

pub const MAXPLMOVE: fixed_t = 0x32; // forwardmove[1]

pub const TURBOTHRESHOLD: usize = 0x32;

#[unsafe(no_mangle)]
pub static mut forwardmove: [fixed_t; 2] = [0x19, 0x32];
#[unsafe(no_mangle)]
pub static mut sidemove: [fixed_t; 2] = [0x18, 0x28];
#[unsafe(no_mangle)]
pub static mut angleturn: [fixed_t; 3] = [640, 1280, 320]; // + slow turn 

pub const SLOWTURNTICS: usize = 6;

pub const NUMKEYS: usize = 256;

#[unsafe(no_mangle)]
pub static mut gamekeydown: [boolean; NUMKEYS] = [0; NUMKEYS];
#[unsafe(no_mangle)]
pub static mut turnheld: usize = 0; // for accelerative turning 

#[unsafe(no_mangle)]
pub static mut mousearray: [boolean; 4] = [0; 4];
#[unsafe(no_mangle)]
pub static mut mousebuttons: *mut boolean = unsafe { &raw mut mousearray[1] }; // allow [-1]

// mouse values are used once
#[unsafe(no_mangle)]
pub static mut mousex: int = 0;
#[unsafe(no_mangle)]
pub static mut mousey: int = 0;

#[unsafe(no_mangle)]
pub static mut dclicktime: usize = 0;
#[unsafe(no_mangle)]
pub static mut dclickstate: int = 0;
#[unsafe(no_mangle)]
pub static mut dclicks: int = 0;
#[unsafe(no_mangle)]
pub static mut dclicktime2: usize = 0;
#[unsafe(no_mangle)]
pub static mut dclickstate2: int = 0;
#[unsafe(no_mangle)]
pub static mut dclicks2: int = 0;

// joystick values are repeated
#[unsafe(no_mangle)]
pub static mut joyxmove: int = 0;
#[unsafe(no_mangle)]
pub static mut joyymove: int = 0;
#[unsafe(no_mangle)]
pub static mut joyarray: [boolean; 5] = [0; 5];
#[unsafe(no_mangle)]
pub static mut joybuttons: *mut boolean = unsafe { &raw mut joyarray[1] }; // allow [-1] 

#[unsafe(no_mangle)]
pub static mut savegameslot: usize = 0;
#[unsafe(no_mangle)]
pub static mut savedescription: [c_char; 32] = [0; 32];

pub const BODYQUESIZE: usize = 32;

#[unsafe(no_mangle)]
pub static mut bodyque: [*mut mobj_t; BODYQUESIZE] = [null_mut(); BODYQUESIZE];
#[unsafe(no_mangle)]
pub static mut bodyqueslot: usize = 0;

#[unsafe(no_mangle)]
pub static mut statcopy: *mut c_void = null_mut(); // for statistics driver

//  D_DoomLoop
unsafe extern "C" {
	static mut maketic: usize;
	static mut ticdup: usize;

	fn HU_dequeueChatChar() -> u8;
}

// G_BuildTiccmd
// Builds a ticcmd from all of the available inputs
// or reads it from the demo buffer.
// If recording a demo, write it out
#[unsafe(no_mangle)]
pub unsafe extern "C" fn G_BuildTiccmd(cmd: *mut ticcmd_t) {
	unsafe {
		let base = I_BaseTiccmd(); // empty, or external driver
		libc::memcpy(cmd.cast(), base.cast(), size_of::<ticcmd_t>());

		(*cmd).consistancy = consistancy[consoleplayer][maketic % BACKUPTICS];

		let strafe = gamekeydown[key_strafe]
			| *mousebuttons.wrapping_offset(mousebstrafe)
			| *joybuttons.wrapping_offset(joybstrafe);
		let speed = if gamekeydown[key_speed] | *joybuttons.wrapping_offset(joybspeed) == 0 {
			0
		} else {
			1
		};

		let mut forward = 0;
		let mut side = 0;

		// use two stage accelerative turning
		// on the keyboard and joystick
		if joyxmove != 0 || gamekeydown[key_right] != 0 || gamekeydown[key_left] != 0 {
			turnheld += ticdup;
		} else {
			turnheld = 0;
		}

		let tspeed = if turnheld < SLOWTURNTICS {
			2 // slow turn
		} else {
			speed
		};

		// let movement keys cancel each other out
		if strafe != 0 {
			if gamekeydown[key_right] != 0 {
				// fprintf(stderr, "strafe right\n");
				side += sidemove[speed];
			}
			if gamekeydown[key_left] != 0 {
				//	fprintf(stderr, "strafe left\n");
				side -= sidemove[speed];
			}
			if joyxmove > 0 {
				side += sidemove[speed];
			}
			if joyxmove < 0 {
				side -= sidemove[speed];
			}
		} else {
			let cmd_angleturn = &mut (*cmd).angleturn;
			if gamekeydown[key_right] != 0 {
				*cmd_angleturn = (*cmd_angleturn).wrapping_sub(angleturn[tspeed] as i16);
			}
			if gamekeydown[key_left] != 0 {
				*cmd_angleturn = (*cmd_angleturn).wrapping_add(angleturn[tspeed] as i16);
			}
			if joyxmove > 0 {
				*cmd_angleturn = (*cmd_angleturn).wrapping_sub(angleturn[tspeed] as i16);
			}
			if joyxmove < 0 {
				*cmd_angleturn = (*cmd_angleturn).wrapping_add(angleturn[tspeed] as i16);
			}
		}

		if gamekeydown[key_up] != 0 {
			// fprintf(stderr, "up\n");
			forward += forwardmove[speed];
		}
		if gamekeydown[key_down] != 0 {
			// fprintf(stderr, "down\n");
			forward -= forwardmove[speed];
		}
		if joyymove < 0 {
			forward += forwardmove[speed];
		}
		if joyymove > 0 {
			forward -= forwardmove[speed];
		}
		if gamekeydown[key_straferight] != 0 {
			side += sidemove[speed];
		}
		if gamekeydown[key_strafeleft] != 0 {
			side -= sidemove[speed];
		}

		// buttons
		(*cmd).chatchar = HU_dequeueChatChar();

		if gamekeydown[key_fire]
			| *mousebuttons.wrapping_offset(mousebfire)
			| *joybuttons.wrapping_offset(joybfire)
			!= 0
		{
			(*cmd).buttons |= BT_ATTACK;
		}

		if gamekeydown[key_use] | *joybuttons.wrapping_offset(joybuse) != 0 {
			(*cmd).buttons |= BT_USE;
			// clear double clicks if hit use button
			dclicks = 0;
		}

		// chainsaw overrides
		for i in 0..weapontype_t::NUMWEAPONS as usize - 1 {
			if gamekeydown[b'1' as usize + i] != 0 {
				(*cmd).buttons |= BT_CHANGE;
				(*cmd).buttons |= (i << BT_WEAPONSHIFT) as u8;
				break;
			}
		}

		// mouse
		if *mousebuttons.wrapping_offset(mousebforward) != 0 {
			forward += forwardmove[speed];
		}

		// forward double click
		if *mousebuttons.wrapping_offset(mousebforward) != dclickstate && dclicktime > 1 {
			dclickstate = *mousebuttons.wrapping_offset(mousebforward);
			if dclickstate != 0 {
				dclicks += 1;
			}
			if dclicks == 2 {
				(*cmd).buttons |= BT_USE;
				dclicks = 0;
			} else {
				dclicktime = 0;
			}
		} else {
			dclicktime += ticdup;
			if dclicktime > 20 {
				dclicks = 0;
				dclickstate = 0;
			}
		}

		// strafe double click
		let bstrafe = (*mousebuttons.wrapping_offset(mousebstrafe) != 0
			|| *joybuttons.wrapping_offset(joybstrafe) != 0) as i32;
		if bstrafe != dclickstate2 && dclicktime2 > 1 {
			dclickstate2 = bstrafe;
			if dclickstate2 != 0 {
				dclicks2 += 1;
			}
			if dclicks2 == 2 {
				(*cmd).buttons |= BT_USE;
				dclicks2 = 0;
			} else {
				dclicktime2 = 0;
			}
		} else {
			dclicktime2 += ticdup;
			if dclicktime2 > 20 {
				dclicks2 = 0;
				dclickstate2 = 0;
			}
		}

		forward += mousey;
		if strafe != 0 {
			side += mousex * 2;
		} else {
			(*cmd).angleturn = (*cmd).angleturn.wrapping_sub((mousex * 0x8) as i16);
		}

		mousex = 0;
		mousey = 0;

		forward = forward.clamp(-MAXPLMOVE, MAXPLMOVE);
		side = side.clamp(-MAXPLMOVE, MAXPLMOVE);

		(*cmd).forwardmove += forward as i8;
		(*cmd).sidemove += side as i8;

		// special buttons
		if sendpause != 0 {
			sendpause = 0;
			(*cmd).buttons = BT_SPECIAL | BTS_PAUSE;
		}

		if sendsave != 0 {
			sendsave = 0;
			(*cmd).buttons = BT_SPECIAL | BTS_SAVEGAME | ((savegameslot as u8) << BTS_SAVESHIFT);
		}
	}
}

// G_DoLoadLevel

unsafe extern "C" {
	fn R_FlatNumForName(name: *const c_char) -> i32;
	fn R_TextureNumForName(name: *const c_char) -> i32;
}

fn G_DoLoadLevel() {
	unsafe {
		// Set the sky map.
		// First thing, we have a dummy sky texture name,
		//  a flat. The data is in the WAD only because
		//  we look for an actual index, instead of simply
		//  setting one.
		skyflatnum = R_FlatNumForName(SKYFLATNAME);

		// DOOM determines the sky texture to be used
		// depending on the current episode, and the game version.
		if (gamemode == GameMode_t::commercial)
			|| (/*gamemode*/gamemission == GameMission_t::pack_tnt)
			|| (/*gamemode*/gamemission == GameMission_t::pack_plut)
		{
			skytexture = R_TextureNumForName(c"SKY3".as_ptr());
			if gamemap < 12 {
				skytexture = R_TextureNumForName(c"SKY1".as_ptr());
			} else if gamemap < 21 {
				skytexture = R_TextureNumForName(c"SKY2".as_ptr());
			}
		}

		levelstarttic = gametic; // for time calculation

		if wipegamestate == gamestate_t::GS_LEVEL {
			wipegamestate = gamestate_t::None; // force a wipe
		}

		gamestate = gamestate_t::GS_LEVEL;

		for i in 0..MAXPLAYERS {
			if playeringame[i] != 0 && players[i].playerstate == playerstate_t::PST_DEAD {
				players[i].playerstate = playerstate_t::PST_REBORN;
			}
			players[i].frags = [0; MAXPLAYERS];
		}

		P_SetupLevel(gameepisode, gamemap, 0, gameskill);
		displayplayer = consoleplayer; // view the guy you are playing
		starttime = I_GetTime();
		gameaction = gameaction_t::ga_nothing;
		Z_CheckHeap();

		// clear cmd building stuff
		gamekeydown = [0; NUMKEYS];
		joyxmove = 0;
		joyymove = 0;
		mousex = 0;
		mousey = 0;
		sendpause = 0;
		sendsave = 0;
		paused = 0;
		// libc::memset(mousebuttons.cast(), 0, size_of_val(&mousebuttons));
		mousearray = [mousearray[0], 0, 0, 0];
		// libc::memset(joybuttons.cast(), 0, size_of_val(&joybuttons));
		joyarray = [joyarray[0], 0, 0, 0, 0];
	}
}

unsafe extern "C" {
	static mut mouseSensitivity: i32;

	fn M_StartControlPanel();
	fn F_Responder(ev: *mut event_t) -> boolean;
	fn HU_Responder(ev: *mut event_t) -> boolean;
	fn ST_Responder(ev: *mut event_t) -> boolean;
	fn AM_Responder(ev: *mut event_t) -> boolean;
}

// G_Responder
// Get info needed to make ticcmd_ts for the players.
pub(crate) fn G_Responder(ev: *mut event_t) -> boolean {
	unsafe {
		// allow spy mode changes even during the demo
		if gamestate == gamestate_t::GS_LEVEL
			&& (*ev).ty == evtype_t::ev_keydown
			&& (*ev).data1 == KEY_F12 as i32
			&& (singledemo != 0 || deathmatch == 0)
		{
			// spy mode
			loop {
				displayplayer += 1;
				if displayplayer == MAXPLAYERS {
					displayplayer = 0;
				}
				if playeringame[displayplayer] != 0 || displayplayer == consoleplayer {
					break;
				}
			}
			return 1;
		}

		// any other key pops up menu if in demos
		if gameaction == gameaction_t::ga_nothing
			&& singledemo == 0
			&& (demoplayback != 0 || gamestate == gamestate_t::GS_DEMOSCREEN)
		{
			if (*ev).ty == evtype_t::ev_keydown
				|| ((*ev).ty == evtype_t::ev_mouse && (*ev).data1 != 0)
				|| ((*ev).ty == evtype_t::ev_joystick && (*ev).data1 != 0)
			{
				M_StartControlPanel();
				return 1;
			}
			return 0;
		}

		if gamestate == gamestate_t::GS_LEVEL {
			if HU_Responder(ev) != 0 {
				return 1; // chat ate the event
			}
			if ST_Responder(ev) != 0 {
				return 1; // status window ate it
			}
			if AM_Responder(ev) != 0 {
				return 1; // automap ate it
			}
		}

		if gamestate == gamestate_t::GS_FINALE && F_Responder(ev) != 0 {
			return 1; // finale ate the event
		}

		match (*ev).ty {
			evtype_t::ev_keydown => {
				if (*ev).data1 == KEY_PAUSE as i32 {
					sendpause = 1;
					return 1;
				}
				if (*ev).data1 < NUMKEYS as i32 {
					gamekeydown[(*ev).data1 as usize] = 1;
				}
				1 // eat key down events
			}
			evtype_t::ev_keyup => {
				if (*ev).data1 < NUMKEYS as i32 {
					gamekeydown[(*ev).data1 as usize] = 0;
				}
				0 // always let key up events filter down
			}
			evtype_t::ev_mouse => {
				*mousebuttons.wrapping_add(0) = (*ev).data1 & 1;
				*mousebuttons.wrapping_add(1) = (*ev).data1 & 2;
				*mousebuttons.wrapping_add(2) = (*ev).data1 & 4;
				mousex = (*ev).data2 * (mouseSensitivity + 5) / 10;
				mousey = (*ev).data3 * (mouseSensitivity + 5) / 10;
				1 // eat events
			}
			evtype_t::ev_joystick => {
				*joybuttons.wrapping_add(0) = (*ev).data1 & 1;
				*joybuttons.wrapping_add(1) = (*ev).data1 & 2;
				*joybuttons.wrapping_add(2) = (*ev).data1 & 4;
				*joybuttons.wrapping_add(3) = (*ev).data1 & 8;
				joyxmove = (*ev).data2;
				joyymove = (*ev).data3;
				1 // eat events
			}
		}
	}
}

unsafe extern "C" {
	static mut netcmds: [[ticcmd_t; BACKUPTICS]; MAXPLAYERS];

	fn F_StartFinale();
	fn M_ScreenShot();
	fn S_PauseSound();
	fn S_ResumeSound();
	fn P_Ticker();
	fn ST_Ticker();
	fn AM_Ticker();
	fn HU_Ticker();
	fn WI_Ticker();
	fn F_Ticker();
}

// G_Ticker
// Make ticcmd_ts for the players.
#[unsafe(no_mangle)]
pub extern "C" fn G_Ticker() {
	unsafe {
		// do player reborns if needed
		for i in 0..MAXPLAYERS {
			if playeringame[i] != 0 && players[i].playerstate == playerstate_t::PST_REBORN {
				G_DoReborn(i);
			}
		}

		// do things to change the game state
		while gameaction != gameaction_t::ga_nothing {
			match gameaction {
				gameaction_t::ga_loadlevel => G_DoLoadLevel(),
				gameaction_t::ga_newgame => G_DoNewGame(),
				gameaction_t::ga_loadgame => G_DoLoadGame(),
				gameaction_t::ga_savegame => G_DoSaveGame(),
				gameaction_t::ga_playdemo => G_DoPlayDemo(),
				gameaction_t::ga_completed => G_DoCompleted(),
				gameaction_t::ga_victory => F_StartFinale(),
				gameaction_t::ga_worlddone => G_DoWorldDone(),
				gameaction_t::ga_screenshot => {
					M_ScreenShot();
					gameaction = gameaction_t::ga_nothing;
				}
				gameaction_t::ga_nothing => (),
			}
		}

		// get commands, check consistancy,
		// and build new consistancy check
		let buf = (gametic as usize / ticdup) % BACKUPTICS;

		for i in 0..MAXPLAYERS {
			if playeringame[i] != 0 {
				let cmd = &raw mut players[i].cmd;

				libc::memcpy(
					cmd.cast(),
					(&raw const netcmds[i][buf]).cast(),
					size_of::<ticcmd_t>(),
				);

				if demoplayback != 0 {
					G_ReadDemoTiccmd(cmd);
				}
				if demorecording != 0 {
					G_WriteDemoTiccmd(cmd);
				}

				// check for turbo cheats
				if (*cmd).forwardmove > TURBOTHRESHOLD as i8
					&& gametic & 31 == 0
					&& ((gametic >> 5) & 3) == i as i32
				{
					static mut turbomessage: [c_char; 80] = [0; 80];
					unsafe extern "C" {
						static mut player_names: [*mut c_char; 4];
					}
					libc::sprintf(
						&raw mut turbomessage[0],
						c"%s is turbo!".as_ptr(),
						player_names[i],
					);
					players[consoleplayer].message = &raw mut turbomessage[0];
				}

				if netgame != 0 && netdemo == 0 && gametic % ticdup as i32 == 0 {
					if gametic > BACKUPTICS as i32 && consistancy[i][buf] != (*cmd).consistancy {
						I_Error(
							c"consistency failure (%i should be %i)".as_ptr(),
							(*cmd).consistancy as c_int,
							consistancy[i][buf] as c_int,
						);
					}
					if !players[i].mo.is_null() {
						consistancy[i][buf] = (*players[i].mo).x as i16;
					} else {
						consistancy[i][buf] = rndindex as i16;
					}
				}
			}
		}

		// check for special buttons
		for i in 0..MAXPLAYERS {
			if playeringame[i] != 0 && players[i].cmd.buttons & BT_SPECIAL != 0 {
				match players[i].cmd.buttons & BT_SPECIALMASK {
					BTS_PAUSE => {
						paused ^= 1;
						if paused != 0 {
							S_PauseSound();
						} else {
							S_ResumeSound();
						}
					}

					BTS_SAVEGAME => {
						if savedescription[0] == 0 {
							libc::strcpy(&raw mut savedescription[0], c"NET GAME".as_ptr());
						}
						savegameslot =
							((players[i].cmd.buttons & BTS_SAVEMASK) >> BTS_SAVESHIFT) as usize;
						gameaction = gameaction_t::ga_savegame;
					}

					_ => (),
				}
			}
		}

		// do main actions
		match gamestate {
			gamestate_t::GS_LEVEL => {
				P_Ticker();
				ST_Ticker();
				AM_Ticker();
				HU_Ticker();
			}
			gamestate_t::GS_INTERMISSION => WI_Ticker(),
			gamestate_t::GS_FINALE => F_Ticker(),
			gamestate_t::GS_DEMOSCREEN => D_PageTicker(),
			gamestate_t::None => (),
		}
	}
}

// PLAYER STRUCTURE FUNCTIONS
// also see P_SpawnPlayer in P_Things

// G_PlayerFinishLevel
// Can when a player completes a level.
fn G_PlayerFinishLevel(player: usize) {
	unsafe {
		let p = &raw mut players[player];

		(*p).powers = [0; 6];
		(*p).cards = [0; 6];
		(*(*p).mo).flags &= !MF_SHADOW; // cancel invisibility
		(*p).extralight = 0; // cancel gun flashes
		(*p).fixedcolormap = 0; // cancel ir gogles
		(*p).damagecount = 0; // no palette changes
		(*p).bonuscount = 0;
	}
}

unsafe extern "C" {
	static mut maxammo: [i32; ammotype_t::NUMAMMO as usize];
}

// G_PlayerReborn
// Called after a player dies
// almost everything is cleared and initialized
#[unsafe(no_mangle)]
pub extern "C" fn G_PlayerReborn(player: usize) {
	unsafe {
		let frags = players[player].frags;
		let killcount = players[player].killcount;
		let itemcount = players[player].itemcount;
		let secretcount = players[player].secretcount;

		let p = &raw mut players[player];
		libc::memset(p.cast(), 0, size_of::<player_t>());

		players[player].frags = frags;
		players[player].killcount = killcount;
		players[player].itemcount = itemcount;
		players[player].secretcount = secretcount;

		// don't do anything immediately
		(*p).usedown = 1;
		(*p).attackdown = 1;

		(*p).playerstate = playerstate_t::PST_LIVE;
		(*p).health = MAXHEALTH;
		(*p).readyweapon = weapontype_t::wp_pistol;
		(*p).pendingweapon = weapontype_t::wp_pistol;
		(*p).weaponowned[weapontype_t::wp_fist as usize] = 1;
		(*p).weaponowned[weapontype_t::wp_pistol as usize] = 1;
		(*p).ammo[ammotype_t::am_clip as usize] = 50;

		(*p).maxammo = maxammo;
	}
}

// G_CheckSpot
// Returns false if the player cannot be respawned
// at the given mapthing_t spot
// because something is occupying it

unsafe extern "C" {
	fn P_CheckPosition(thing: *const mobj_t, x: fixed_t, y: fixed_t) -> boolean;
	fn P_RemoveMobj(thing: *mut mobj_t);
	fn R_PointInSubsector(x: fixed_t, y: fixed_t) -> *mut subsector_t;
	fn P_SpawnMobj(x: fixed_t, y: fixed_t, floorheight: i32, mt_tfog: mobjtype_t) -> *mut mobj_t;
	fn S_StartSound(origin: *mut c_void, sound_id: sfxenum_t);
}

fn G_CheckSpot(playernum: usize, mthing: *mut mapthing_t) -> boolean {
	unsafe {
		if players[playernum].mo.is_null() {
			// first spawn of level, before corpses
			for p in &players[..playernum] {
				if (*p.mo).x == ((*mthing).x as i32) << FRACBITS
					&& (*p.mo).y == ((*mthing).y as i32) << FRACBITS
				{
					return 0;
				}
			}
			return 1;
		}

		let x = ((*mthing).x as i32) << FRACBITS;
		let y = ((*mthing).y as i32) << FRACBITS;

		if P_CheckPosition(players[playernum].mo, x, y) != 0 {
			return 0;
		}

		// flush an old corpse if needed
		if bodyqueslot >= BODYQUESIZE {
			P_RemoveMobj(bodyque[bodyqueslot % BODYQUESIZE]);
		}
		bodyque[bodyqueslot % BODYQUESIZE] = players[playernum].mo;
		bodyqueslot += 1;

		// spawn a teleport fog
		let ss = R_PointInSubsector(x, y);
		let an = (ANG45 * ((*mthing).angle as angle_t / 45)) >> ANGLETOFINESHIFT;

		let mo = P_SpawnMobj(
			x + 20 * finecos(an as usize),
			y + 20 * finesine[an as usize],
			(*(*ss).sector).floorheight,
			mobjtype_t::MT_TFOG,
		);

		if players[consoleplayer].viewz != 1 {
			S_StartSound(mo.cast(), sfxenum_t::sfx_telept); // don't start sound on first frame
		}

		1
	}
}

unsafe extern "C" {
	fn P_SpawnPlayer(thing: *mut mapthing_t);
}

// G_DeathMatchSpawnPlayer
// Spawns a player at one of the random death match spots
// called at level load and each death
#[unsafe(no_mangle)]
pub(crate) fn G_DeathMatchSpawnPlayer(playernum: usize) {
	unsafe {
		let selections = deathmatch_p.offset_from(&raw mut deathmatchstarts[0]);
		if selections < 4 {
			I_Error(c"Only %i deathmatch spots, 4 required".as_ptr(), selections);
		}

		for _ in 0..20 {
			let i = (P_Random() % selections as i32) as usize;
			if G_CheckSpot(playernum, &raw mut deathmatchstarts[i]) != 0 {
				deathmatchstarts[i].ty = playernum as i16 + 1;
				P_SpawnPlayer(&raw mut deathmatchstarts[i]);
				return;
			}
		}

		// no good spot, so the player will probably get stuck
		P_SpawnPlayer(&raw mut playerstarts[playernum]);
	}
}

// G_DoReborn
fn G_DoReborn(playernum: usize) {
	unsafe {
		if netgame == 0 {
			// reload the level from scratch
			gameaction = gameaction_t::ga_loadlevel;
		} else {
			// respawn at the start

			// first dissasociate the corpse
			(*players[playernum].mo).player = null_mut();

			// spawn at random spot if in death match
			if deathmatch != 0 {
				G_DeathMatchSpawnPlayer(playernum);
				return;
			}

			if G_CheckSpot(playernum, &raw mut playerstarts[playernum]) != 0 {
				P_SpawnPlayer(&raw mut playerstarts[playernum]);
				return;
			}

			// try to spawn at one of the other players spots
			#[allow(clippy::needless_range_loop)]
			for i in 0..MAXPLAYERS {
				if G_CheckSpot(playernum, &raw mut playerstarts[i]) != 0 {
					playerstarts[i].ty = playernum as i16 + 1; // fake as other player
					P_SpawnPlayer(&raw mut playerstarts[i]);
					playerstarts[i].ty = i as i16 + 1; // restore
					return;
				}
				// he's going to be inside something.  Too bad.
			}
			P_SpawnPlayer(&raw mut playerstarts[playernum]);
		}
	}
}

#[unsafe(no_mangle)]
pub extern "C" fn G_ScreenShot() {
	unsafe {
		gameaction = gameaction_t::ga_screenshot;
	}
}

// DOOM Par Times
#[unsafe(no_mangle)]
pub static mut pars: [[usize; 10]; 4] = [
	[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
	[0, 30, 75, 120, 90, 165, 180, 180, 30, 165],
	[0, 90, 90, 90, 120, 90, 360, 240, 30, 170],
	[0, 90, 45, 90, 150, 90, 90, 165, 30, 135],
];

// DOOM II Par Times
#[unsafe(no_mangle)]
pub static mut cpars: [usize; 32] = [
	30, 90, 120, 120, 90, 150, 120, 120, 270, 90, //  1-10
	210, 150, 150, 150, 210, 150, 420, 150, 210, 150, // 11-20
	240, 150, 180, 150, 150, 300, 330, 420, 300, 180, // 21-30
	120, 30, // 31-32
];

// G_DoCompleted
#[unsafe(no_mangle)]
pub static mut secretexit: boolean = 0;

#[unsafe(no_mangle)]
pub extern "C" fn G_ExitLevel() {
	unsafe {
		secretexit = 0;
		gameaction = gameaction_t::ga_completed;
	}
}

// Here's for the german edition.
#[unsafe(no_mangle)]
pub extern "C" fn G_SecretExitLevel() {
	unsafe {
		// IF NO WOLF3D LEVELS, NO SECRET EXIT!
		if gamemode == GameMode_t::commercial && W_CheckNumForName(c"map31".as_ptr()) < 0 {
			secretexit = 0;
		} else {
			secretexit = 1;
		}
		gameaction = gameaction_t::ga_completed;
	}
}

unsafe extern "C" {
	static mut automapactive: boolean;
	fn AM_Stop();
	fn WI_Start(wbstartstruct: *mut wbstartstruct_t);
}

fn G_DoCompleted() {
	unsafe {
		gameaction = gameaction_t::ga_nothing;

		#[allow(clippy::needless_range_loop)]
		for i in 0..MAXPLAYERS {
			if playeringame[i] != 0 {
				G_PlayerFinishLevel(i); // take away cards and stuff
			}
		}

		if automapactive != 0 {
			AM_Stop();
		}

		if gamemode != GameMode_t::commercial {
			match gamemap {
				8 => {
					gameaction = gameaction_t::ga_victory;
					return;
				}
				9 =>
				{
					#[allow(clippy::needless_range_loop)]
					for i in 0..MAXPLAYERS {
						players[i].didsecret = 1;
					}
				}
				_ => (),
			}
		}

		wminfo.didsecret = players[consoleplayer].didsecret;
		wminfo.epsd = gameepisode - 1;
		wminfo.last = gamemap - 1;

		// wminfo.next is 0 biased, unlike gamemap
		if gamemode == GameMode_t::commercial {
			if secretexit != 0 {
				match gamemap {
					15 => wminfo.next = 30,
					31 => wminfo.next = 31,
					_ => (),
				}
			} else {
				match gamemap {
					31 | 32 => wminfo.next = 15,
					_ => wminfo.next = gamemap,
				}
			}
		} else if secretexit != 0 {
			wminfo.next = 8; // go to secret level
		} else if gamemap == 9 {
			// returning from secret level
			match gameepisode {
				1 => wminfo.next = 3,
				2 => wminfo.next = 5,
				3 => wminfo.next = 6,
				4 => wminfo.next = 2,
				_ => (),
			}
		} else {
			wminfo.next = gamemap; // go to next level
		}

		wminfo.maxkills = totalkills;
		wminfo.maxitems = totalitems;
		wminfo.maxsecret = totalsecret;
		wminfo.maxfrags = 0;
		if gamemode == GameMode_t::commercial {
			wminfo.partime = 35 * cpars[gamemap - 1];
		} else {
			wminfo.partime = 35 * pars[gameepisode][gamemap];
		}
		wminfo.pnum = consoleplayer;

		for i in 0..MAXPLAYERS {
			wminfo.plyr[i].in_ = playeringame[i];
			wminfo.plyr[i].skills = players[i].killcount;
			wminfo.plyr[i].sitems = players[i].itemcount;
			wminfo.plyr[i].ssecret = players[i].secretcount;
			wminfo.plyr[i].stime = leveltime;
			wminfo.plyr[i].frags = players[i].frags;
		}

		gamestate = gamestate_t::GS_INTERMISSION;
		viewactive = 0;
		automapactive = 0;

		if !statcopy.is_null() {
			libc::memcpy(statcopy, (&raw mut wminfo).cast(), size_of::<wbstartstruct_t>());
		}

		WI_Start(&raw mut wminfo);
	}
}

// G_WorldDone
#[unsafe(no_mangle)]
pub extern "C" fn G_WorldDone() {
	unsafe {
		gameaction = gameaction_t::ga_worlddone;

		if secretexit != 0 {
			players[consoleplayer].didsecret = 1;
		}

		if gamemode == GameMode_t::commercial {
			match gamemap {
				15 | 31 if secretexit == 0 => (),
				15 | 31 | 6 | 11 | 20 | 30 => F_StartFinale(),
				_ => (),
			}
		}
	}
}

fn G_DoWorldDone() {
	unsafe {
		gamestate = gamestate_t::GS_LEVEL;
		gamemap = wminfo.next + 1;
		G_DoLoadLevel();
		gameaction = gameaction_t::ga_nothing;
		viewactive = 1;
	}
}

// G_InitFromSavegame
// Can be called by the startup code or the menu task.
unsafe extern "C" {
	static mut setsizeneeded: boolean;
}

#[unsafe(no_mangle)]
pub static mut savename: [c_char; 256] = [0; 256];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn G_LoadGame(name: *const c_char) {
	unsafe {
		libc::strcpy(&raw mut savename[0], name);
		gameaction = gameaction_t::ga_loadgame;
	}
}

pub const VERSIONSIZE: usize = 16;

unsafe extern "C" {
	static mut save_p: *mut u8;

	fn M_ReadFile(name: *const c_char, buffer: *mut *mut u8) -> i32;
	fn P_UnArchivePlayers();
	fn P_UnArchiveWorld();
	fn P_UnArchiveThinkers();
	fn P_UnArchiveSpecials();
	fn R_ExecuteSetViewSize();
	fn R_FillBackScreen();
}

fn G_DoLoadGame() {
	unsafe {
		gameaction = gameaction_t::ga_nothing;

		let _length = M_ReadFile(&raw const savename[0], &raw mut savebuffer);
		save_p = savebuffer.wrapping_add(SAVESTRINGSIZE);

		// skip the description field
		let mut vcheck = [0; VERSIONSIZE];
		libc::sprintf(&raw mut vcheck[0], c"version %i".as_ptr(), VERSION);
		if libc::strcmp(save_p.cast(), &raw const vcheck[0]) != 0 {
			return; // bad version
		}
		save_p = save_p.wrapping_add(VERSIONSIZE);

		gameskill = (*save_p).into();
		save_p = save_p.wrapping_add(1);
		gameepisode = *save_p as usize;
		save_p = save_p.wrapping_add(1);
		gamemap = *save_p as usize;
		save_p = save_p.wrapping_add(1);

		#[allow(clippy::needless_range_loop)]
		for i in 0..MAXPLAYERS {
			playeringame[i] = *save_p as boolean;
			save_p = save_p.wrapping_add(1);
		}

		// load a base level
		G_InitNew(gameskill, gameepisode, gamemap);

		// get the times
		let a = *save_p as usize;
		save_p = save_p.wrapping_add(1);
		let b = *save_p as usize;
		save_p = save_p.wrapping_add(1);
		let c = *save_p as usize;
		save_p = save_p.wrapping_add(1);
		leveltime = (a << 16) + (b << 8) + c;

		// dearchive all the modifications
		P_UnArchivePlayers();
		P_UnArchiveWorld();
		P_UnArchiveThinkers();
		P_UnArchiveSpecials();

		if *save_p != 0x1d {
			I_Error(c"Bad savegame".as_ptr());
		}

		// done
		Z_Free(savebuffer.cast());

		if setsizeneeded != 0 {
			R_ExecuteSetViewSize();
		}

		// draw the pattern into the back screen
		R_FillBackScreen();
	}
}

// G_SaveGame
// Called by the menu task.
// Description is a 24 byte text string
#[unsafe(no_mangle)]
pub unsafe extern "C" fn G_SaveGame(slot: usize, description: *const c_char) {
	unsafe {
		savegameslot = slot;
		libc::strcpy(&raw mut savedescription[0], description);
		sendsave = 1;
	}
}

unsafe extern "C" {
	fn P_ArchivePlayers();
	fn P_ArchiveWorld();
	fn P_ArchiveThinkers();
	fn P_ArchiveSpecials();
	fn M_WriteFile(name: *const c_char, source: *mut c_void, length: usize) -> boolean;

}

fn G_DoSaveGame() {
	unsafe {
		let mut name = [0; 100];

		if M_CheckParm(c"-cdrom".as_ptr()) != 0 {
			libc::sprintf(
				&raw mut name[0],
				c"c:\\doomdata\\%s%d.dsg".as_ptr(),
				SAVEGAMENAME,
				savegameslot,
			);
		} else {
			libc::sprintf(&raw mut name[0], c"%s%d.dsg".as_ptr(), SAVEGAMENAME, savegameslot);
		}
		let description = savedescription;

		savebuffer = screens[1].wrapping_add(0x4000);
		save_p = savebuffer;

		libc::memcpy(save_p.cast(), (&raw const description).cast(), SAVESTRINGSIZE);
		save_p = save_p.wrapping_add(SAVESTRINGSIZE);

		let mut name2 = [0; VERSIONSIZE];
		libc::sprintf(&raw mut name2[0], c"version %i".as_ptr(), VERSION);
		libc::memcpy(save_p.cast(), (&raw const name2[0]).cast(), VERSIONSIZE);
		save_p = save_p.wrapping_add(VERSIONSIZE);

		*save_p = gameskill as u8;
		save_p = save_p.wrapping_add(1);
		*save_p = gameepisode as u8;
		save_p = save_p.wrapping_add(1);
		*save_p = gamemap as u8;
		save_p = save_p.wrapping_add(1);

		#[allow(clippy::needless_range_loop)]
		for i in 0..MAXPLAYERS {
			*save_p = playeringame[i] as u8;
			save_p = save_p.wrapping_add(1);
		}

		*save_p = (leveltime >> 16) as u8;
		save_p = save_p.wrapping_add(1);
		*save_p = (leveltime >> 8) as u8;
		save_p = save_p.wrapping_add(1);
		*save_p = leveltime as u8;
		save_p = save_p.wrapping_add(1);

		P_ArchivePlayers();
		P_ArchiveWorld();
		P_ArchiveThinkers();
		P_ArchiveSpecials();

		*save_p = 0x1d; // consistancy marker
		save_p = save_p.wrapping_add(1);

		let length = save_p.offset_from(savebuffer);
		if length as usize > SAVEGAMESIZE {
			I_Error(c"Savegame buffer overrun".as_ptr());
		}

		M_WriteFile(&raw const name[0], savebuffer.cast(), length as usize);
		gameaction = gameaction_t::ga_nothing;
		savedescription[0] = 0;

		players[consoleplayer].message = GGSAVED.as_ptr();

		// draw the pattern into the back screen
		R_FillBackScreen();
	}
}

// G_InitNew
// Can be called by the startup code or the menu task,
// consoleplayer, displayplayer, playeringame[] should be set.
#[unsafe(no_mangle)]
pub static mut d_skill: skill_t = skill_t::sk_baby;
#[unsafe(no_mangle)]
pub static mut d_episode: usize = 0;
#[unsafe(no_mangle)]
pub static mut d_map: usize = 0;

#[unsafe(no_mangle)]
pub extern "C" fn G_DeferedInitNew(skill: skill_t, episode: usize, map: usize) {
	unsafe {
		d_skill = skill;
		d_episode = episode;
		d_map = map;
		gameaction = gameaction_t::ga_newgame;
	}
}

fn G_DoNewGame() {
	unsafe {
		demoplayback = 0;
		netdemo = 0;
		netgame = 0;
		deathmatch = 0;
		playeringame[1] = 0;
		playeringame[2] = 0;
		playeringame[3] = 0;
		respawnparm = 0;
		fastparm = 0;
		nomonsters = 0;
		consoleplayer = 0;
		G_InitNew(d_skill, d_episode, d_map);
		gameaction = gameaction_t::ga_nothing;
	}
}

pub(crate) fn G_InitNew(skill: skill_t, mut episode: usize, mut map: usize) {
	unsafe {
		if paused != 0 {
			paused = 0;
			S_ResumeSound();
		}

		// if (skill > sk_nightmare)
		// {
		// 	skill = sk_nightmare;
		// }

		// This was quite messy with SPECIAL and commented parts.
		// Supposedly hacks to make the latest edition work.
		// It might not work properly.

		if gamemode == GameMode_t::retail {
			episode = episode.clamp(1, 4);
		} else if gamemode == GameMode_t::shareware {
			episode = 1; // only start episode 1 on shareware
		} else {
			episode = episode.clamp(1, 3);
		}

		if map < 1 {
			map = 1;
		}

		if map > 9 && gamemode != GameMode_t::commercial {
			map = 9;
		}

		M_ClearRandom();

		if skill == skill_t::sk_nightmare || respawnparm != 0 {
			respawnmonsters = 1;
		} else {
			respawnmonsters = 0;
		}

		if fastparm != 0 || (skill == skill_t::sk_nightmare && gameskill != skill_t::sk_nightmare) {
			#[allow(clippy::needless_range_loop)]
			for i in statenum_t::S_SARG_RUN1 as usize..statenum_t::S_SARG_PAIN2 as usize {
				states[i].tics >>= 1;
			}
			mobjinfo[mobjtype_t::MT_BRUISERSHOT as usize].speed = 20 * FRACUNIT;
			mobjinfo[mobjtype_t::MT_HEADSHOT as usize].speed = 20 * FRACUNIT;
			mobjinfo[mobjtype_t::MT_TROOPSHOT as usize].speed = 20 * FRACUNIT;
		} else if skill != skill_t::sk_nightmare && gameskill == skill_t::sk_nightmare {
			#[allow(clippy::needless_range_loop)]
			for i in statenum_t::S_SARG_RUN1 as usize..statenum_t::S_SARG_PAIN2 as usize {
				states[i].tics <<= 1;
			}
			mobjinfo[mobjtype_t::MT_BRUISERSHOT as usize].speed = 15 * FRACUNIT;
			mobjinfo[mobjtype_t::MT_HEADSHOT as usize].speed = 10 * FRACUNIT;
			mobjinfo[mobjtype_t::MT_TROOPSHOT as usize].speed = 10 * FRACUNIT;
		}

		// force players to be initialized upon first level load
		#[allow(clippy::needless_range_loop)]
		for i in 0..MAXPLAYERS {
			players[i].playerstate = playerstate_t::PST_REBORN;
		}

		usergame = 1; // will be set false if a demo
		paused = 0;
		demoplayback = 0;
		automapactive = 0;
		viewactive = 1;
		gameepisode = episode;
		gamemap = map;
		gameskill = skill;

		viewactive = 1;

		// set the sky map for the episode
		if gamemode == GameMode_t::commercial {
			skytexture = R_TextureNumForName(c"SKY3".as_ptr());
			if gamemap < 12 {
				skytexture = R_TextureNumForName(c"SKY1".as_ptr());
			} else if gamemap < 21 {
				skytexture = R_TextureNumForName(c"SKY2".as_ptr());
			}
		} else {
			match episode {
				1 => skytexture = R_TextureNumForName(c"SKY1".as_ptr()),
				2 => skytexture = R_TextureNumForName(c"SKY2".as_ptr()),
				3 => skytexture = R_TextureNumForName(c"SKY3".as_ptr()),
				4 => skytexture = R_TextureNumForName(c"SKY4".as_ptr()), // Special Edition sky
				_ => (),
			}
		}

		G_DoLoadLevel();
	}
}

// DEMO RECORDING

pub const DEMOMARKER: u8 = 0x80;

fn G_ReadDemoTiccmd(cmd: *mut ticcmd_t) {
	unsafe {
		if *demo_p == DEMOMARKER {
			// end of demo data stream
			G_CheckDemoStatus();
			return;
		}
		(*cmd).forwardmove = *demo_p as i8;
		demo_p = demo_p.wrapping_add(1);
		(*cmd).sidemove = *demo_p as i8;
		demo_p = demo_p.wrapping_add(1);
		(*cmd).angleturn = (*demo_p as i16) << 8;
		demo_p = demo_p.wrapping_add(1);
		(*cmd).buttons = *demo_p;
		demo_p = demo_p.wrapping_add(1);
	}
}

fn G_WriteDemoTiccmd(cmd: *mut ticcmd_t) {
	unsafe {
		if gamekeydown['q' as usize] != 0
		// press q to end demo recording
		{
			G_CheckDemoStatus();
		}
		*demo_p = (*cmd).forwardmove as u8;
		demo_p = demo_p.wrapping_add(1);
		*demo_p = (*cmd).sidemove as u8;
		demo_p = demo_p.wrapping_add(1);
		*demo_p = (((*cmd).angleturn + 128) >> 8) as u8;
		demo_p = demo_p.wrapping_add(1);
		*demo_p = (*cmd).buttons;
		demo_p = demo_p.wrapping_add(1);

		demo_p = demo_p.wrapping_sub(4);

		if demoend.offset_from(demo_p) < 16 {
			// no more space
			G_CheckDemoStatus();
			return;
		}

		G_ReadDemoTiccmd(cmd); // make SURE it is exactly the same
	}
}

// G_RecordDemo
pub(crate) fn G_RecordDemo(name: *const c_char) {
	unsafe {
		usergame = 0;
		libc::strcpy(&raw mut demoname[0], name);
		libc::strcat(&raw mut demoname[0], c".lmp".as_ptr());
		let mut maxsize = 0x20000;
		let i = M_CheckParm(c"-maxdemo".as_ptr());
		if i != 0 && i < myargc - 1 {
			maxsize = libc::atoi(*myargv.wrapping_add(i + 1)) as usize * 1024;
		}
		demobuffer = Z_Malloc(maxsize, PU_STATIC, null_mut()).cast();
		demoend = demobuffer.wrapping_add(maxsize);

		demorecording = 1;
	}
}

pub(crate) fn G_BeginRecording() {
	unsafe {
		demo_p = demobuffer;

		*demo_p = VERSION as u8;
		demo_p = demo_p.wrapping_add(1);
		*demo_p = gameskill as u8;
		demo_p = demo_p.wrapping_add(1);
		*demo_p = gameepisode as u8;
		demo_p = demo_p.wrapping_add(1);
		*demo_p = gamemap as u8;
		demo_p = demo_p.wrapping_add(1);
		*demo_p = deathmatch as u8;
		demo_p = demo_p.wrapping_add(1);
		*demo_p = respawnparm as u8;
		demo_p = demo_p.wrapping_add(1);
		*demo_p = fastparm as u8;
		demo_p = demo_p.wrapping_add(1);
		*demo_p = nomonsters as u8;
		demo_p = demo_p.wrapping_add(1);
		*demo_p = consoleplayer as u8;
		demo_p = demo_p.wrapping_add(1);

		#[allow(clippy::needless_range_loop)]
		for i in 0..MAXPLAYERS {
			*demo_p = playeringame[i] as u8;
			demo_p = demo_p.wrapping_add(1);
		}
	}
}

// G_PlayDemo

#[unsafe(no_mangle)]
pub static mut defdemoname: *const c_char = null();

pub(crate) fn G_DeferedPlayDemo(name: *const c_char) {
	unsafe {
		defdemoname = name;
		gameaction = gameaction_t::ga_playdemo;
	}
}

fn G_DoPlayDemo() {
	unsafe {
		gameaction = gameaction_t::ga_nothing;
		demo_p = W_CacheLumpName(defdemoname, PU_STATIC).cast();
		demobuffer = demo_p;
		if *demo_p != VERSION as u8 {
			eprintln!(
				"Demo is from a different game version! (version = {}, demo version = {})",
				VERSION, *demo_p
			);
			gameaction = gameaction_t::ga_nothing;
			return;
		}
		demo_p = demo_p.wrapping_add(1);

		let skill = skill_t::from(*demo_p);
		demo_p = demo_p.wrapping_add(1);
		let episode = *demo_p as usize;
		demo_p = demo_p.wrapping_add(1);
		let map = *demo_p as usize;
		demo_p = demo_p.wrapping_add(1);

		deathmatch = *demo_p as boolean;
		demo_p = demo_p.wrapping_add(1);
		respawnparm = *demo_p as boolean;
		demo_p = demo_p.wrapping_add(1);
		fastparm = *demo_p as boolean;
		demo_p = demo_p.wrapping_add(1);
		nomonsters = *demo_p as boolean;
		demo_p = demo_p.wrapping_add(1);
		consoleplayer = *demo_p as usize;
		demo_p = demo_p.wrapping_add(1);

		#[allow(clippy::needless_range_loop)]
		for i in 0..MAXPLAYERS {
			playeringame[i] = *demo_p as i32;
			demo_p = demo_p.wrapping_add(1);
		}
		if playeringame[1] != 0 {
			netgame = 1;
			netdemo = 1;
		}

		// don't spend a lot of time in loadlevel
		precache = 0;
		G_InitNew(skill, episode, map);
		precache = 1;

		usergame = 0;
		demoplayback = 1;
	}
}

// G_TimeDemo
pub(crate) fn G_TimeDemo(name: *const c_char) {
	unsafe {
		nodrawers = M_CheckParm(c"-nodraw".as_ptr()) as boolean;
		noblit = M_CheckParm(c"-noblit".as_ptr()) as boolean;
		timingdemo = 1;
		singletics = 1;

		defdemoname = name;
		gameaction = gameaction_t::ga_playdemo;
	}
}

/*
===================
=
= G_CheckDemoStatus
=
= Called after a death or level completion to allow demos to be cleaned up
= Returns true if a new demo loop action will take place
===================
*/

#[unsafe(no_mangle)]
pub extern "C" fn G_CheckDemoStatus() -> boolean {
	unsafe {
		if timingdemo != 0 {
			let endtime = I_GetTime();
			I_Error(c"timed %i gametics in %i realtics".as_ptr(), gametic, endtime - starttime);
		}

		if demoplayback != 0 {
			if singledemo != 0 {
				I_Quit();
			}

			Z_ChangeTag!(demobuffer, PU_CACHE);
			demoplayback = 0;
			netdemo = 0;
			netgame = 0;
			deathmatch = 0;
			playeringame[1] = 0;
			playeringame[2] = 0;
			playeringame[3] = 0;
			respawnparm = 0;
			fastparm = 0;
			nomonsters = 0;
			consoleplayer = 0;
			D_AdvanceDemo();
			return 1;
		}

		if demorecording != 0 {
			*demo_p = DEMOMARKER;
			demo_p = demo_p.wrapping_add(1);
			M_WriteFile(
				&raw mut demoname[0],
				demobuffer.cast(),
				demo_p.offset_from(demobuffer) as usize,
			);
			Z_Free(demobuffer.cast());
			demorecording = 0;
			I_Error(c"Demo %s recorded".as_ptr(), demoname);
		}

		0
	}
}
