#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::{
	env,
	ffi::{CStr, CString, c_char, c_int, c_void},
	mem::transmute,
	ptr::{null, null_mut},
	str::FromStr,
};

use libc::{
	R_OK, SEEK_END, SEEK_SET, access, atoi, fclose, fread, fseek, ftell, malloc, memset, mkdir,
	printf, sprintf, strcpy,
};

use crate::{
	d_englsh::{D_CDROM, D_DEVSTR},
	d_event::{MAXEVENTS, event_t, eventhead, events, eventtail, gameaction, gameaction_t},
	d_net::BACKUPTICS,
	d_player::{player_t, playerstate_t},
	d_ticcmd::ticcmd_t,
	doomdef::{
		GameMode_t, Language_t, MAXPLAYERS, SCREENHEIGHT, SCREENWIDTH, VERSION, gamestate_t,
		skill_t,
	},
	doomstat::{gamemode, language, modifiedgame},
	f_wipe::{wipe_EndScreen, wipe_Melt, wipe_ScreenWipe, wipe_StartScreen},
	g_game::{
		G_BeginRecording, G_DeferedPlayDemo, G_InitNew, G_RecordDemo, G_Responder, G_TimeDemo,
	},
	i_system::{I_Error, I_GetTime, I_Init},
	m_argv::M_CheckParm,
	myargc, myargv,
	p_setup::P_Init,
	p_tick::players,
	r_defs::patch_t,
	sounds::musicenum_t,
	v_video::{V_DrawPatch, V_DrawPatchDirect, V_Init},
	w_wad::W_InitMultipleFiles,
	z_zone::{PU_CACHE, Z_Init},
};

// D-DoomLoop()
// Not a globally visible function,
//  just included for source reference,
//  called by D_DoomMain, never exits.
// Manages timing and IO,
//  calls all ?_Responder, ?_Ticker, and ?_Drawer,
//  calls I_GetTime, I_StartFrame, and I_StartTic
// void D_DoomLoop (void);

const MAXWADFILES: usize = 20;

#[unsafe(no_mangle)]
pub static mut wadfiles: [*mut c_char; MAXWADFILES] = [null_mut(); MAXWADFILES];

type boolean = i32;

#[unsafe(no_mangle)]
pub static mut devparm: boolean = 0; // started game with -devparm
#[unsafe(no_mangle)]
pub static mut nomonsters: boolean = 0; // checkparm of -nomonsters
#[unsafe(no_mangle)]
pub static mut respawnparm: boolean = 0; // checkparm of -respawn
#[unsafe(no_mangle)]
pub static mut fastparm: boolean = 0; // checkparm of -fast

#[unsafe(no_mangle)]
pub static mut drone: boolean = 0;

#[unsafe(no_mangle)]
pub static mut singletics: boolean = 0; // debug flag to cancel adaptiveness

//extern int soundVolume;
//extern  int	sfxVolume;
//extern  int	musicVolume;

unsafe extern "C" {
	static mut inhelpscreens: boolean;
}

#[unsafe(no_mangle)]
pub static mut startskill: skill_t = skill_t::sk_baby;
#[unsafe(no_mangle)]
pub static mut startepisode: usize = 0;
#[unsafe(no_mangle)]
pub static mut startmap: usize = 0;
#[unsafe(no_mangle)]
pub static mut autostart: boolean = 0;

#[unsafe(no_mangle)]
pub static mut debugfile: *const libc::FILE = null();

#[unsafe(no_mangle)]
pub static mut advancedemo: boolean = 0;

#[unsafe(no_mangle)]
pub static mut wadfile: [c_char; 1024] = [0; 1024]; // primary wad file
#[unsafe(no_mangle)]
pub static mut mapdir: [c_char; 1024] = [0; 1024]; // directory of development maps
#[unsafe(no_mangle)]
pub static mut basedefault: [c_char; 1024] = [0; 1024]; // default file

// D_PostEvent
// Called by the I/O functions when input is detected
#[unsafe(no_mangle)]
pub extern "C" fn D_PostEvent(ev: &mut event_t) {
	unsafe {
		events[eventhead] = *ev;
		eventhead = (eventhead + 1) & (MAXEVENTS - 1);
	}
}

unsafe extern "C" {
	fn W_CheckNumForName(_: *const c_char) -> i32;
	fn M_Responder(ev: &mut event_t) -> i32;
}

// D_ProcessEvents
// Send all the events of the given timestamp down the responder chain
#[unsafe(no_mangle)]
pub extern "C" fn D_ProcessEvents() {
	unsafe {
		// IF STORE DEMO, DO NOT ACCEPT INPUT
		if gamemode == GameMode_t::commercial && W_CheckNumForName(c"map01".as_ptr()) < 0 {
			return;
		}

		while eventtail != eventhead {
			let ev = &mut events[eventtail];
			if M_Responder(ev) != 0 {
				eventtail = (eventtail + 1) & (MAXEVENTS - 1);
				continue; // menu ate the event
			}
			G_Responder(ev);
			eventtail = (eventtail + 1) & (MAXEVENTS - 1);
		}
	}
}

// D_Display
//  draw current display, possibly wiping it from the previous

// wipegamestate can be set to -1 to force a wipe on the next draw
#[unsafe(no_mangle)]
pub static mut wipegamestate: gamestate_t = gamestate_t::GS_DEMOSCREEN;

unsafe extern "C" {
	static mut automapactive: boolean;
	static mut displayplayer: usize;
	static mut gametic: i32;
	static mut menuactive: boolean;
	static mut nodrawers: boolean;
	static mut paused: boolean;
	static mut scaledviewwidth: i32;
	static mut setsizeneeded: boolean;
	static mut viewactive: boolean;
	static mut viewheight: i32;
	static mut viewwindowx: usize;
	static mut viewwindowy: usize;
	static mut gamestate: gamestate_t;

	fn AM_Drawer();
	fn F_Drawer();
	fn HU_Drawer();
	fn HU_Erase();
	fn I_FinishUpdate();
	fn I_SetPalette(palette: *mut u8);
	fn I_UpdateNoBlit();
	fn M_Drawer();
	fn NetUpdate();
	fn R_DrawViewBorder();
	fn R_ExecuteSetViewSize();
	fn R_FillBackScreen();
	fn R_RenderPlayerView(player: &mut player_t);
	fn ST_Drawer(_: boolean, redrawsbar: boolean);
	fn WI_Drawer();
	fn W_CacheLumpName(name: *const c_char, tag: usize) -> *mut c_void;
}

fn D_Display() {
	unsafe {
		static mut viewactivestate: boolean = 0;
		static mut menuactivestate: boolean = 0;
		static mut inhelpscreensstate: boolean = 0;
		static mut fullscreen: boolean = 0;
		static mut oldgamestate: gamestate_t = gamestate_t::None;
		static mut borderdrawcount: i32 = 0;
		let wipe;

		if nodrawers != 0 {
			return; // for comparative timing / profiling
		}

		let mut redrawsbar = 0;

		// change the view size if needed
		if setsizeneeded != 0 {
			R_ExecuteSetViewSize();
			oldgamestate = gamestate_t::None; // force background redraw
			borderdrawcount = 3;
		}

		// save the current screen if about to wipe
		if gamestate != wipegamestate {
			wipe = true;
			wipe_StartScreen(0, 0, SCREENWIDTH, SCREENHEIGHT);
		} else {
			wipe = false;
		}

		if gamestate == gamestate_t::GS_LEVEL && gametic != 0 {
			HU_Erase();
		}

		// do buffered drawing
		match gamestate {
			gamestate_t::GS_LEVEL => {
				if gametic != 0 {
					if automapactive != 0 {
						AM_Drawer();
					}
					if wipe || (viewheight != 200 && fullscreen != 0) {
						redrawsbar = 1;
					}
					if inhelpscreensstate != 0 && inhelpscreens == 0 {
						redrawsbar = 1; // just put away the help screen
					}
					ST_Drawer((viewheight == 200) as boolean, redrawsbar);
					fullscreen = (viewheight == 200) as boolean;
				}
			}
			gamestate_t::GS_INTERMISSION => WI_Drawer(),
			gamestate_t::GS_FINALE => F_Drawer(),
			gamestate_t::GS_DEMOSCREEN => D_PageDrawer(),
			gamestate_t::None => (),
		}

		// draw buffered stuff to screen
		I_UpdateNoBlit();

		// draw the view directly
		if gamestate == gamestate_t::GS_LEVEL && automapactive == 0 && gametic != 0 {
			R_RenderPlayerView(&mut players[displayplayer]);
		}

		if gamestate == gamestate_t::GS_LEVEL && gametic != 0 {
			HU_Drawer();
		}

		// clean up border stuff
		if gamestate != oldgamestate && gamestate != gamestate_t::GS_LEVEL {
			I_SetPalette(W_CacheLumpName(c"PLAYPAL".as_ptr(), PU_CACHE) as *mut u8);
		}

		// see if the border needs to be initially drawn
		if gamestate == gamestate_t::GS_LEVEL && oldgamestate != gamestate_t::GS_LEVEL {
			viewactivestate = 0; // view was not active
			R_FillBackScreen(); // draw the pattern into the back screen
		}

		// see if the border needs to be updated to the screen
		if gamestate == gamestate_t::GS_LEVEL && automapactive == 0 && scaledviewwidth != 320 {
			if menuactive != 0 || menuactivestate != 0 || viewactivestate == 0 {
				borderdrawcount = 3;
			}
			if borderdrawcount != 0 {
				R_DrawViewBorder(); // erase old menu stuff
				borderdrawcount -= 1;
			}
		}

		menuactivestate = menuactive;
		viewactivestate = viewactive;
		inhelpscreensstate = inhelpscreens;
		oldgamestate = gamestate;
		wipegamestate = gamestate;

		// draw pause pic
		if paused != 0 {
			let y = if automapactive != 0 { 4 } else { viewwindowy + 4 };
			let x = viewwindowx.wrapping_add_signed((scaledviewwidth as isize - 68) / 2);
			V_DrawPatchDirect(
				x,
				y,
				0,
				W_CacheLumpName(c"M_PAUSE".as_ptr(), PU_CACHE) as *mut patch_t,
			);
		}

		// menus go directly to the screen
		M_Drawer(); // menu is drawn even on top of everything
		NetUpdate(); // send out any new accumulation

		// normal update
		if !wipe {
			I_FinishUpdate(); // page flip or blit buffer
			return;
		}

		// wipe update
		wipe_EndScreen(0, 0, SCREENWIDTH, SCREENHEIGHT);

		let mut wipestart = I_GetTime() - 1;

		loop {
			let mut nowtime;
			let mut tics;
			loop {
				nowtime = I_GetTime();
				tics = nowtime - wipestart;
				if tics != 0 {
					break;
				}
			}
			wipestart = nowtime;
			let done = wipe_ScreenWipe(wipe_Melt, 0, 0, SCREENWIDTH, SCREENHEIGHT, tics as usize);
			I_UpdateNoBlit();
			M_Drawer(); // menu is drawn even on top of wipes
			I_FinishUpdate(); // page flip or blit buffer
			if done != 0 {
				break;
			}
		}
	}
}

//  D_DoomLoop
unsafe extern "C" {
	static mut demorecording: boolean;
	static mut consoleplayer: usize;
	static mut maketic: usize;
	static mut netcmds: [[ticcmd_t; BACKUPTICS]; MAXPLAYERS];
	fn I_InitGraphics();
	fn I_StartFrame();
	fn I_StartTic();
	fn M_Ticker();
	fn G_Ticker();
	fn TryRunTics();
	fn G_BuildTiccmd(cmd: *mut ticcmd_t);
	fn S_UpdateSounds(listener: *mut c_void);
}

#[unsafe(no_mangle)]
pub(crate) fn D_DoomLoop() {
	unsafe {
		if demorecording != 0 {
			G_BeginRecording();
		}

		if M_CheckParm(c"-debugfile".as_ptr()) != 0 {
			let mut filename = [0; 20];
			sprintf(&raw mut filename[0], c"debug%i.txt".as_ptr(), consoleplayer);
			libc::printf(c"debug output to: %s\n".as_ptr(), filename);
			debugfile = libc::fopen(&raw mut filename[0], c"w".as_ptr());
		}

		I_InitGraphics();

		loop {
			// frame syncronous IO operations
			I_StartFrame();

			// process one or more tics
			if singletics != 0 {
				I_StartTic();
				D_ProcessEvents();
				G_BuildTiccmd(&raw mut netcmds[consoleplayer][maketic % BACKUPTICS]);
				if advancedemo != 0 {
					D_DoAdvanceDemo();
				}
				M_Ticker();
				G_Ticker();
				gametic += 1;
				maketic += 1;
			} else {
				TryRunTics(); // will run at least one tic
			}

			S_UpdateSounds(players[consoleplayer].mo as *mut c_void); // move positional sounds

			// Update display, next frame, with current state.
			D_Display();

			// #ifndef SNDSERV
			// // Sound mixing for the buffer is snychronous.
			// I_UpdateSound();
			// #endif
			// // Synchronous sound output is explicitly called.
			// #ifndef SNDINTR
			// // Update sound output.
			// I_SubmitSound();
			// #endif
		}
	}
}

//  DEMO LOOP
#[unsafe(no_mangle)]
pub static mut demosequence: i32 = 0;
#[unsafe(no_mangle)]
pub static mut pagetic: i32 = 0;
#[unsafe(no_mangle)]
pub static mut pagename: *const c_char = null_mut();

// D_PageTicker
// Handles timing for warped projection
pub(crate) fn D_PageTicker() {
	unsafe {
		pagetic -= 1;
		if pagetic < 0 {
			D_AdvanceDemo();
		}
	}
}

// D_PageDrawer
fn D_PageDrawer() {
	unsafe {
		V_DrawPatch(0, 0, 0, W_CacheLumpName(pagename, PU_CACHE) as *const patch_t);
	}
}

// D_AdvanceDemo
// Called after each demo or intro demosequence finishes
#[unsafe(no_mangle)]
pub(crate) fn D_AdvanceDemo() {
	unsafe {
		advancedemo = 1;
	}
}

unsafe extern "C" {
	static mut usergame: boolean;
	fn S_StartMusic(music_id: musicenum_t);
}

// This cycles through the demo sequences.
// FIXME - version dependend demo numbers?
#[unsafe(no_mangle)]
pub extern "C" fn D_DoAdvanceDemo() {
	unsafe {
		players[consoleplayer].playerstate = playerstate_t::PST_LIVE; // not reborn
		advancedemo = 0;
		usergame = 0; // no save / end game here
		paused = 0;
		gameaction = gameaction_t::ga_nothing;

		if gamemode == GameMode_t::retail {
			demosequence = (demosequence + 1) % 7;
		} else {
			demosequence = (demosequence + 1) % 6;
		}

		match demosequence {
			0 => {
				if gamemode == GameMode_t::commercial {
					pagetic = 35 * 11;
				} else {
					pagetic = 170;
				}
				gamestate = gamestate_t::GS_DEMOSCREEN;
				pagename = c"TITLEPIC".as_ptr();
				if gamemode == GameMode_t::commercial {
					S_StartMusic(musicenum_t::mus_dm2ttl);
				} else {
					S_StartMusic(musicenum_t::mus_intro);
				}
			}
			1 => {
				G_DeferedPlayDemo(c"demo1".as_ptr());
			}
			2 => {
				pagetic = 200;
				gamestate = gamestate_t::GS_DEMOSCREEN;
				pagename = c"CREDIT".as_ptr();
			}
			3 => {
				G_DeferedPlayDemo(c"demo2".as_ptr());
			}
			4 => {
				gamestate = gamestate_t::GS_DEMOSCREEN;
				if gamemode == GameMode_t::commercial {
					pagetic = 35 * 11;
					pagename = c"TITLEPIC".as_ptr();
					S_StartMusic(musicenum_t::mus_dm2ttl);
				} else {
					pagetic = 200;

					if gamemode == GameMode_t::retail {
						pagename = c"CREDIT".as_ptr();
					} else {
						pagename = c"HELP2".as_ptr();
					}
				}
			}
			5 => {
				G_DeferedPlayDemo(c"demo3".as_ptr());
			}
			// THE DEFINITIVE DOOM Special Edition demo
			6 => {
				G_DeferedPlayDemo(c"demo4".as_ptr());
			}
			_ => (),
		}
	}
}

// D_StartTitle
#[unsafe(no_mangle)]
pub extern "C" fn D_StartTitle() {
	unsafe {
		gameaction = gameaction_t::ga_nothing;
		demosequence = -1;
		D_AdvanceDemo();
	}
}

//      print title for every printed line
#[unsafe(no_mangle)]
pub static mut title: [c_char; 128] = [0; 128];

// D_AddFile
fn D_AddFile(file: *const c_char) {
	unsafe {
		let mut numwadfiles = 0;

		while !wadfiles[numwadfiles].is_null() {
			numwadfiles += 1;
		}

		let newfile = libc::malloc(libc::strlen(file) + 1) as *mut c_char;
		libc::strcpy(newfile, file);

		wadfiles[numwadfiles] = newfile;
	}
}

macro_rules! devdata {
	($s:literal) => {
		concat!("devdata", $s, "\0").as_ptr() as *const i8
	};
}

macro_rules! devmaps {
	($s:literal) => {
		concat!("devmaps", $s, "\0").as_ptr() as *const i8
	};
}

macro_rules! tilde_devmaps {
	($s:literal) => {
		concat!("~", "devmaps", $s, "\0").as_ptr() as *const i8
	};
}

// IdentifyVersion
// Checks availability of IWAD files by name,
// to determine whether registered/commercial features
// should be executed (notably loading PWAD's).
fn IdentifyVersion() {
	unsafe {
		//#ifdef NORMALUNIX
		let doomwaddir = env::var("DOOMWADDIR").unwrap_or_else(|_| ".".to_owned());

		// Commercial.
		let doom2wad = format!("{doomwaddir}/doom2.wad\0");

		// Retail.
		let doomuwad = format!("{doomwaddir}/doomu.wad\0");

		// Registered.
		let doomwad = format!("{doomwaddir}/doom.wad\0");

		// Shareware.
		let doom1wad = format!("{doomwaddir}/doom1.wad\0");

		// Bug, dear Shawn.
		// Insufficient malloc, caused spurious realloc errors.
		let plutoniawad = format!("{doomwaddir}/plutonia.wad\0");

		let tntwad = format!("{doomwaddir}/tnt.wad\0");

		// French stuff.
		let doom2fwad = format!("{doomwaddir}/doom2f.wad\0");

		let Ok(home) = env::var("HOME") else {
			I_Error(c"Please set $HOME to your home directory".as_ptr());
		};
		let home = CString::from_str(&home).unwrap();
		sprintf(&raw mut basedefault[0], c"%s/.doomrc".as_ptr(), home.as_ptr());
		//#endif

		if M_CheckParm(c"-shdev".as_ptr()) != 0 {
			gamemode = GameMode_t::shareware;
			devparm = 1;
			D_AddFile(devdata!("doom1.wad"));
			D_AddFile(devmaps!("data_se/texture1.lmp"));
			D_AddFile(devmaps!("data_se/pnames.lmp"));
			libc::strcpy(&raw mut basedefault[0], devdata!("default.cfg"));
			return;
		}

		if M_CheckParm(c"-regdev".as_ptr()) != 0 {
			gamemode = GameMode_t::registered;
			devparm = 1;
			D_AddFile(devdata!("doom.wad"));
			D_AddFile(devmaps!("data_se/texture1.lmp"));
			D_AddFile(devmaps!("data_se/texture2.lmp"));
			D_AddFile(devmaps!("data_se/pnames.lmp"));
			libc::strcpy(&raw mut basedefault[0], devdata!("default.cfg"));
			return;
		}

		if M_CheckParm(c"-comdev".as_ptr()) != 0 {
			gamemode = GameMode_t::commercial;
			devparm = 1;
			D_AddFile(devdata!("doom2.wad"));

			D_AddFile(devmaps!("cdata/texture1.lmp"));
			D_AddFile(devmaps!("cdata/pnames.lmp"));
			libc::strcpy(&raw mut basedefault[0], devdata!("default.cfg"));
			return;
		}

		if access(doom2fwad.as_ptr() as *const i8, R_OK) == 0 {
			gamemode = GameMode_t::commercial;
			// C'est ridicule!
			// Let's handle languages in config files, okay?
			language = Language_t::french;
			println!("French version");
			D_AddFile(doom2fwad.as_ptr() as *const i8);
			return;
		}

		if access(doom2wad.as_ptr() as *const i8, R_OK) == 0 {
			gamemode = GameMode_t::commercial;
			D_AddFile(doom2wad.as_ptr() as *const i8);
			return;
		}

		if (access(plutoniawad.as_ptr() as *const i8, R_OK)) == 0 {
			gamemode = GameMode_t::commercial;
			D_AddFile(plutoniawad.as_ptr() as *const i8);
			return;
		}

		if (access(tntwad.as_ptr() as *const i8, R_OK)) == 0 {
			gamemode = GameMode_t::commercial;
			D_AddFile(tntwad.as_ptr() as *const i8);
			return;
		}

		if (access(doomuwad.as_ptr() as *const i8, R_OK)) == 0 {
			gamemode = GameMode_t::retail;
			D_AddFile(doomuwad.as_ptr() as *const i8);
			return;
		}

		if (access(doomwad.as_ptr() as *const i8, R_OK)) == 0 {
			gamemode = GameMode_t::registered;
			D_AddFile(doomwad.as_ptr() as *const i8);
			return;
		}

		if (access(doom1wad.as_ptr() as *const i8, R_OK)) == 0 {
			gamemode = GameMode_t::shareware;
			D_AddFile(doom1wad.as_ptr() as *const i8);
			return;
		}

		println!("Game mode indeterminate.");
		gamemode = GameMode_t::indetermined;

		// We don't abort. Let's see what the PWAD contains.
		//exit(1);
		//I_Error ("Game mode indeterminate\n");
	}
}

// Find a Response File
fn FindResponseFile() {
	unsafe {
		const MAXARGVS: usize = 100;

		for i in 1..myargc {
			let arg = *myargv.add(i);
			if *arg == b'@' as i8 {
				// READ THE RESPONSE FILE INTO MEMORY
				let response_file = arg.wrapping_add(1);
				let handle = libc::fopen(response_file, c"rb".as_ptr());
				if handle.is_null() {
					println!("\nNo such response file!");
					libc::exit(1);
				}
				println!(
					"Found response file {}!",
					CStr::from_ptr(response_file).to_str().unwrap()
				);
				fseek(handle, 0, SEEK_END);
				let size = ftell(handle) as usize;
				fseek(handle, 0, SEEK_SET);
				let file = libc::malloc(size) as *mut c_char;
				fread(file as *mut c_void, size, 1, handle);
				fclose(handle);

				// KEEP ALL CMDLINE ARGS FOLLOWING @RESPONSEFILE ARG
				let mut index = 0;
				let mut moreargs = [null_mut(); 20];
				for k in i + 1..myargc {
					moreargs[index] = *myargv.wrapping_add(k);
					index += 1;
				}

				let firstargv = *myargv.wrapping_add(0);
				myargv = malloc(size_of::<*const char>() * MAXARGVS) as *mut *mut c_char;
				memset(myargv as *mut c_void, 0, size_of::<*const char>() * MAXARGVS);
				*myargv = firstargv;

				let infile = file;
				let mut k = 0;
				let mut indexinfile = 1; // SKIP PAST ARGV[0] (KEEP IT)
				loop {
					*myargv.wrapping_add(indexinfile) = infile.wrapping_add(k);
					indexinfile += 1;
					while k < size && (b' '..=b'z').contains(&(*infile.wrapping_add(k) as u8)) {
						k += 1;
					}
					*infile.wrapping_add(k) = 0;
					while k < size && !(b' '..=b'z').contains(&(*infile.wrapping_add(k) as u8)) {
						k += 1;
					}
					if k >= size {
						break;
					}
				}

				for arg in moreargs.into_iter().take(index) {
					*myargv.wrapping_add(indexinfile) = arg;
					indexinfile += 1;
				}
				myargc = indexinfile;

				// DISPLAY ARGS
				#[allow(static_mut_refs)]
				{
					println!("{} command-line args:", myargc);
				}
				for k in 1..myargc {
					println!("{}", CStr::from_ptr(*myargv.wrapping_add(k)).to_str().unwrap());
				}

				break;
			}
		}
	}
}

unsafe extern "C" {
	static mut deathmatch: boolean;
	static mut singledemo: boolean;
	static mut netgame: boolean;
	static mut snd_SfxVolume: i32;
	static mut snd_MusicVolume: i32;
	fn M_LoadDefaults();
	fn M_Init();
	fn R_Init();
	fn D_CheckNetGame();
	fn S_Init(snd_SfxVolume: i32, snd_MusicVolume: i32);
	fn HU_Init();
	fn ST_Init();
	fn G_LoadGame(name: *mut c_char);
}

macro_rules! cdrom_savegamename {
	($s:literal) => {
		concat!("c:\\doomdata\\doomsav", $s, "\0").as_ptr() as *const i8
	};
}

macro_rules! savegamename {
	($s:literal) => {
		concat!("doomsav", $s, "\0").as_ptr() as *const i8
	};
}

// D_DoomMain
#[unsafe(no_mangle)]
pub extern "C" fn D_DoomMain() {
	unsafe {
		FindResponseFile();

		IdentifyVersion();

		//setbuf(stdout, NULL);
		modifiedgame = 0;

		nomonsters = M_CheckParm(c"-nomonsters".as_ptr()) as boolean;
		respawnparm = M_CheckParm(c"-respawn".as_ptr()) as boolean;
		fastparm = M_CheckParm(c"-fast".as_ptr()) as boolean;
		devparm = M_CheckParm(c"-devparm".as_ptr()) as boolean;
		if M_CheckParm(c"-altdeath".as_ptr()) != 0 {
			deathmatch = 2;
		} else if M_CheckParm(c"-deathmatch".as_ptr()) != 0 {
			deathmatch = 1;
		}

		match gamemode {
			GameMode_t::retail => {
				sprintf(
					&raw mut title[0],
					c"						 The Ultimate DOOM Startup v%i.%i						   ".as_ptr(),
					VERSION / 100,
					VERSION % 100,
				);
			}
			GameMode_t::shareware => {
				sprintf(
					&raw mut title[0],
					c"							DOOM Shareware Startup v%i.%i						   ".as_ptr(),
					VERSION / 100,
					VERSION % 100,
				);
			}
			GameMode_t::registered => {
				sprintf(
					&raw mut title[0],
					c"							DOOM Registered Startup v%i.%i						   ".as_ptr(),
					VERSION / 100,
					VERSION % 100,
				);
			}
			GameMode_t::commercial => {
				sprintf(
					&raw mut title[0],
					c"						 DOOM 2: Hell on Earth v%i.%i						   ".as_ptr(),
					VERSION / 100,
					VERSION % 100,
				);
				/*FIXME
				} GameMode_t::pack_plut => {
				sprintf (&raw mut title[0],
				c"				   ".as_ptr()
				c"DOOM 2: Plutonia Experiment v%i.%i".as_ptr()
				c"						   ".as_ptr(),
				VERSION/100,VERSION%100);
				break;
				} GameMode_t::pack_tnt => {
				sprintf (&raw mut title[0],
				c"					 ".as_ptr()
				c"DOOM 2: TNT - Evilution v%i.%i".as_ptr()
				c"						   ".as_ptr(),
				VERSION/100,VERSION%100);
				break;
				*/
			}
			_ => {
				sprintf(
					&raw mut title[0],
					c"					 Public DOOM - v%i.%i						   ".as_ptr(),
					VERSION / 100,
					VERSION % 100,
				);
			}
		}

		printf(c"%s\n".as_ptr(), &raw const title[0]);

		if devparm != 0 {
			printf(D_DEVSTR.as_ptr());
		}

		if M_CheckParm(c"-cdrom".as_ptr()) != 0 {
			printf(D_CDROM.as_ptr());
			mkdir(c"c:\\doomdata".as_ptr(), 0);
			strcpy(&raw mut basedefault[0], c"c:/doomdata/default.cfg".as_ptr());
		}

		// turbo option
		if let p @ 1.. = M_CheckParm(c"-turbo".as_ptr()) {
			let mut scale = 200;

			unsafe extern "C" {
				static mut forwardmove: [i32; 2];
				static mut sidemove: [i32; 2];
			}

			if p < myargc - 1 {
				scale = atoi(*myargv.wrapping_add(p + 1));
			}
			scale = scale.clamp(10, 400);

			printf(c"turbo scale: %i%%\n".as_ptr(), scale);
			forwardmove[0] = forwardmove[0] * scale / 100;
			forwardmove[1] = forwardmove[1] * scale / 100;
			sidemove[0] = sidemove[0] * scale / 100;
			sidemove[1] = sidemove[1] * scale / 100;
		}

		let mut file: [c_char; 256] = [0; 256];

		// add any files specified on the command line with -file wadfile
		// to the wad list
		//
		// convenience hack to allow -wart e m to add a wad file
		// prepend a tilde to the filename so wadfile will be reloadable
		let p = M_CheckParm(c"-wart".as_ptr());
		if p != 0 {
			*(*myargv.wrapping_add(p)).wrapping_add(4) = b'p' as i8; // big hack, change to -warp

			let argvp1 = *myargv.wrapping_add(p + 1);
			let argvp2 = *myargv.wrapping_add(p + 2);

			// Map name handling.
			#[allow(clippy::wildcard_in_or_patterns)]
			match gamemode {
				GameMode_t::shareware | GameMode_t::retail | GameMode_t::registered => {
					sprintf(
						&raw mut file[0],
						tilde_devmaps!("E%cM%c.wad"),
						*argvp1 as c_int,
						*argvp2 as c_int,
					);
					printf(c"Warping to Episode %s, Map %s.\n".as_ptr(), argvp1, argvp2);
				}
				GameMode_t::commercial | _ => {
					let p = atoi(argvp1) as usize;
					if p < 10 {
						sprintf(&raw mut file[0], tilde_devmaps!("cdata/map0%i.wad"), p);
					} else {
						sprintf(&raw mut file[0], tilde_devmaps!("cdata/map%i.wad"), p);
					}
				}
			}
			D_AddFile(&raw const file[0]);
		}

		let mut p = M_CheckParm(c"-file".as_ptr());
		if p != 0 {
			// the parms after p are wadfile/lump names,
			// until end of parms or another - preceded parm
			modifiedgame = 1; // homebrew levels
			loop {
				p += 1;
				if p == myargc || **myargv.wrapping_add(p) == b'-' as i8 {
					break;
				}
				D_AddFile(*myargv.wrapping_add(p));
			}
		}

		let mut p = M_CheckParm(c"-playdemo".as_ptr());

		if p == 0 {
			p = M_CheckParm(c"-timedemo".as_ptr());
		}

		if p > 0 && p < myargc - 1 {
			let argvp1 = *myargv.wrapping_add(p + 1);
			sprintf(&raw mut file[0], c"%s.lmp".as_ptr(), argvp1);
			D_AddFile(&raw const file[0]);
			printf(c"Playing demo %s.lmp.\n".as_ptr(), argvp1);
		}

		// get skill / episode / map from parms
		startskill = skill_t::sk_medium;
		startepisode = 1;
		startmap = 1;
		autostart = 0;

		let p = M_CheckParm(c"-skill".as_ptr());
		if p != 0 && p < myargc - 1 {
			let argvp1 = *myargv.wrapping_add(p + 1);
			startskill = transmute::<i32, skill_t>(*argvp1 as i32 - b'1' as i32);
			autostart = 1;
		}

		let p = M_CheckParm(c"-episode".as_ptr());
		if p != 0 && p < myargc - 1 {
			let argvp1 = *myargv.wrapping_add(p + 1);
			startepisode = *argvp1 as usize - b'0' as usize;
			startmap = 1;
			autostart = 1;
		}

		let p = M_CheckParm(c"-timer".as_ptr());
		if p != 0 && p < myargc - 1 && deathmatch != 0 {
			let argvp1 = *myargv.wrapping_add(p + 1);
			let time = atoi(argvp1);
			printf(c"Levels will end after %d minute".as_ptr(), time);
			if time > 1 {
				printf(c"s".as_ptr());
			}
			printf(c".\n".as_ptr());
		}

		let p = M_CheckParm(c"-avg".as_ptr());
		if p != 0 && p < myargc - 1 && deathmatch != 0 {
			printf(c"Austin Virtual Gaming: Levels will end after 20 minutes\n".as_ptr());
		}

		let p = M_CheckParm(c"-warp".as_ptr());
		if p != 0 && p < myargc - 1 {
			let argvp1 = *myargv.wrapping_add(p + 1);
			let argvp2 = *myargv.wrapping_add(p + 2);
			if gamemode == GameMode_t::commercial {
				startmap = atoi(argvp1) as usize;
			} else {
				startepisode = *argvp1 as usize - b'0' as usize;
				startmap = *argvp2 as usize - b'0' as usize;
			}
			autostart = 1;
		}

		// init subsystems
		printf(c"V_Init: allocate screens.\n".as_ptr());
		V_Init();

		printf(c"M_LoadDefaults: Load system defaults.\n".as_ptr());
		M_LoadDefaults(); // load before initing other systems

		printf(c"Z_Init: Init zone memory allocation daemon. \n".as_ptr());
		Z_Init();

		printf(c"W_Init: Init WADfiles.\n".as_ptr());
		W_InitMultipleFiles((&raw const wadfiles[0]).cast());

		// Check for -file in shareware
		if modifiedgame != 0 {
			// These are the lumps that will be checked in IWAD,
			// if any one is not present, execution will be aborted.
			const name: [*const c_char; 23] = [
				c"e2m1".as_ptr(),
				c"e2m2".as_ptr(),
				c"e2m3".as_ptr(),
				c"e2m4".as_ptr(),
				c"e2m5".as_ptr(),
				c"e2m6".as_ptr(),
				c"e2m7".as_ptr(),
				c"e2m8".as_ptr(),
				c"e2m9".as_ptr(),
				c"e3m1".as_ptr(),
				c"e3m3".as_ptr(),
				c"e3m3".as_ptr(),
				c"e3m4".as_ptr(),
				c"e3m5".as_ptr(),
				c"e3m6".as_ptr(),
				c"e3m7".as_ptr(),
				c"e3m8".as_ptr(),
				c"e3m9".as_ptr(),
				c"dphoof".as_ptr(),
				c"bfgga0".as_ptr(),
				c"heada1".as_ptr(),
				c"cybra1".as_ptr(),
				c"spida1d1".as_ptr(),
			];

			if gamemode == GameMode_t::shareware {
				I_Error(c"\nYou cannot -file with the shareware version. Register!".as_ptr());
			}

			// Check for fake IWAD with right name,
			// but w/o all the lumps of the registered version.
			if gamemode == GameMode_t::registered {
				for n in name {
					if W_CheckNumForName(n) < 0 {
						I_Error(c"\nThis is not the registered version.".as_ptr());
					}
				}
			}
		}

		// Iff additonal PWAD files are used, print modified banner
		if modifiedgame != 0 {
			println!("===========================================================================");
			println!("ATTENTION:  This version of DOOM has been modified.  If you would like to");
			println!("get a copy of the original game, call 1-800-IDGAMES or see the readme file.");
			println!("		You will not receive technical support for modified games.");
			println!("					  press enter to continue");
			println!("===========================================================================");
			// getchar ();
		}

		// Check and print which version is executed.
		match gamemode {
			GameMode_t::shareware | GameMode_t::indetermined => {
				println!(
					"==========================================================================="
				);
				println!("								Shareware!");
				println!(
					"==========================================================================="
				);
			}
			GameMode_t::registered | GameMode_t::retail | GameMode_t::commercial => {
				println!(
					"==========================================================================="
				);
				println!("				 Commercial product - do not distribute!");
				println!("		 Please report software piracy to the SPA: 1-800-388-PIR8");
				println!(
					"==========================================================================="
				);
			}
		}

		printf(c"M_Init: Init miscellaneous info.\n".as_ptr());
		M_Init();

		printf(c"R_Init: Init DOOM refresh daemon - ".as_ptr());
		R_Init();

		printf(c"\nP_Init: Init Playloop state.\n".as_ptr());
		P_Init();

		printf(c"I_Init: Setting up machine state.\n".as_ptr());
		I_Init();

		printf(c"D_CheckNetGame: Checking network game status.\n".as_ptr());
		D_CheckNetGame();

		printf(c"S_Init: Setting up sound.\n".as_ptr());
		S_Init(snd_SfxVolume /* *8 */, snd_MusicVolume /* *8*/);

		printf(c"HU_Init: Setting up heads up display.\n".as_ptr());
		HU_Init();

		printf(c"ST_Init: Init status bar.\n".as_ptr());
		ST_Init();

		// check for a driver that wants intermission stats
		let p = M_CheckParm(c"-statcopy".as_ptr());
		if p != 0 && p < myargc - 1 {
			// for statistics driver
			unsafe extern "C" {
				static mut statcopy: *mut c_void;
			}

			let argvp1 = *myargv.wrapping_add(p + 1);
			statcopy = atoi(argvp1) as *mut c_void;
			printf(c"External statistics registered.\n".as_ptr());
		}

		// start the apropriate game based on parms
		let p = M_CheckParm(c"-record".as_ptr());

		if p != 0 && p < myargc - 1 {
			let argvp1 = *myargv.wrapping_add(p + 1);
			G_RecordDemo(argvp1);
			autostart = 1;
		}

		let p = M_CheckParm(c"-playdemo".as_ptr());
		if p != 0 && p < myargc - 1 {
			singledemo = 1; // quit after one demo
			let argvp1 = *myargv.wrapping_add(p + 1);
			G_DeferedPlayDemo(argvp1);
			D_DoomLoop(); // never returns
		}

		let p = M_CheckParm(c"-timedemo".as_ptr());
		if p != 0 && p < myargc - 1 {
			let argvp1 = *myargv.wrapping_add(p + 1);
			G_TimeDemo(argvp1);
			D_DoomLoop(); // never returns
		}

		let p = M_CheckParm(c"-loadgame".as_ptr());
		if p != 0 && p < myargc - 1 {
			let argvp1 = *myargv.wrapping_add(p + 1);
			if M_CheckParm(c"-cdrom".as_ptr()) != 0 {
				sprintf(&raw mut file[0], cdrom_savegamename!("%c.dsg"), *argvp1 as c_int);
			} else {
				sprintf(&raw mut file[0], savegamename!("%c.dsg"), *argvp1 as c_int);
			}
			G_LoadGame(&raw mut file[0]);
		}

		if gameaction != gameaction_t::ga_loadgame {
			if autostart != 0 || netgame != 0 {
				G_InitNew(startskill, startepisode, startmap);
			} else {
				D_StartTitle(); // start up intro loop
			}
		}

		D_DoomLoop(); // never returns
	}
}
