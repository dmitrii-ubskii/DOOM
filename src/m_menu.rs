#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::{
	ffi::{c_char, c_void},
	mem,
	ptr::{null, null_mut},
};

use libc::O_RDONLY;

use crate::{
	am_map::automapactive,
	d_englsh::{
		DOSY, EMPTYSTRING, ENDGAME, GAMMALVL0, GAMMALVL1, GAMMALVL2, GAMMALVL3, GAMMALVL4, LOADNET,
		MSGOFF, MSGON, NETEND, NEWGAME, NIGHTMARE, QLOADNET, QLPROMPT, QSAVESPOT, QSPROMPT,
		SAVEDEAD, SWSTRING,
	},
	d_event::{event_t, evtype_t},
	d_main::{D_StartTitle, devparm},
	doomdef::{
		GameMode_t, KEY_BACKSPACE, KEY_DOWNARROW, KEY_ENTER, KEY_EQUALS, KEY_ESCAPE, KEY_F1,
		KEY_F2, KEY_F3, KEY_F4, KEY_F5, KEY_F6, KEY_F7, KEY_F8, KEY_F9, KEY_F10, KEY_F11,
		KEY_LEFTARROW, KEY_MINUS, KEY_RIGHTARROW, KEY_UPARROW, Language_t, SCREENWIDTH,
		gamestate_t, skill_t,
	},
	doomstat::{gamemode, language},
	dstrings::{NUM_QUITMESSAGES, SAVEGAMENAME, endmsg},
	g_game::{
		G_DeferedInitNew, G_LoadGame, G_SaveGame, G_ScreenShot, consoleplayer, demoplayback,
		gamestate, gametic, netgame, players, usergame,
	},
	hu_stuff::{HU_FONTSIZE, HU_FONTSTART, chat_on, hu_font, message_dontfuckwithme},
	i_system::{I_GetTime, I_Quit, I_WaitVBL},
	m_argv::M_CheckParm,
	s_sound::{S_SetMusicVolume, S_SetSfxVolume, S_StartSound, snd_MusicVolume, snd_SfxVolume},
	sounds::sfxenum_t,
	v_video::{V_DrawPatchDirect, usegamma},
	w_wad::W_CacheLumpName,
	z_zone::PU_CACHE,
};

type short = i16;
type int = i32;

// defaulted values
pub(crate) static mut mouseSensitivity: int = 0; // has default

// Show messages has default, 0 = off, 1 = on
pub(crate) static mut showMessages: int = 0;

// Blocky mode, has default, 0 = high, 1 = normal
#[unsafe(no_mangle)]
pub static mut detailLevel: int = 0;
#[unsafe(no_mangle)]
pub static mut screenblocks: int = 0; // has default

// temp for screenblocks (0-9)
static mut screenSize: int = 0;

// -1 = no quicksave slot picked!
static mut quickSaveSlot: int = 0;

// 1 = message to be printed
static mut messageToPrint: int = 0;
// ...and here is the message string!
static mut messageString: *const c_char = null_mut();

// message x & y
static mut messageLastMenuActive: bool = false;

// timed message = no input from user
static mut messageNeedsInput: bool = false;
static mut messageRoutine: Option<fn(i32)> = None;

const SAVESTRINGSIZE: usize = 24;

pub static mut gammamsg: [[u8; 26]; 5] = [GAMMALVL0, GAMMALVL1, GAMMALVL2, GAMMALVL3, GAMMALVL4];

// we are going to be entering a savegame string
static mut saveStringEnter: int = 0;
static mut saveSlot: usize = 0; // which slot to save in
static mut saveCharIndex: usize = 0; // which char we're editing
// old save description before edit
static mut saveOldString: [c_char; SAVESTRINGSIZE] = [0; SAVESTRINGSIZE];

pub(crate) static mut inhelpscreens: bool = false;
pub(crate) static mut menuactive: bool = false;

const SKULLXOFF: isize = -32;
const LINEHEIGHT: usize = 16;

static mut savegamestrings: [[c_char; SAVESTRINGSIZE]; 10] = [[0; SAVESTRINGSIZE]; 10];

static mut endstring: [c_char; 160] = [0; 160];

// MENU TYPEDEFS
#[derive(Clone, Copy)]
struct menuitem_t {
	// 0 = no cursor here, 1 = ok, 2 = arrows ok
	pub status: i16,

	pub name: [u8; 10],

	// choice = menu item #.
	// if status = 2,
	//   choice=0:leftarrow,1:rightarrow
	pub routine: Option<fn(i32)>,

	// hotkey in menu
	pub alphaKey: u8,
}

struct menu_t {
	pub numitems: short,            // # of menu items
	pub prevMenu: *mut menu_t,      // previous menu
	pub menuitems: *mut menuitem_t, // menu items
	pub routine: fn(),              // draw routine
	pub x: short,
	pub y: short,      // x,y of menu
	pub lastOn: short, // last item user was on in menu
}

unsafe impl Sync for menu_t {}
unsafe impl Send for menu_t {}

static mut itemOn: short = 0; // menu item skull is on
static mut skullAnimCounter: short = 0; // skull animation counter
static mut whichSkull: short = 0; // which skull to draw

// graphic name of skulls
// warning: initializer-string for array of chars is too long
static mut skullName: [[u8; 9]; 2] = [*b"M_SKULL1\0", *b"M_SKULL2\0"];

// current menudef
static mut currentMenu: *mut menu_t = null_mut();

// DOOM MENU
enum main_e {
	_newgame = 0,
	_options,
	_loadgame,
	_savegame,
	readthis,
	quitdoom,
	main_end,
}

static mut MainMenu: [menuitem_t; 6] = [
	menuitem_t { status: 1, name: *b"M_NGAME\0\0\0", routine: Some(M_NewGame), alphaKey: b'n' },
	menuitem_t { status: 1, name: *b"M_OPTION\0\0", routine: Some(M_Options), alphaKey: b'o' },
	menuitem_t { status: 1, name: *b"M_LOADG\0\0\0", routine: Some(M_LoadGame), alphaKey: b'l' },
	menuitem_t { status: 1, name: *b"M_SAVEG\0\0\0", routine: Some(M_SaveGame), alphaKey: b's' },
	menuitem_t { status: 1, name: *b"M_RDTHIS\0\0", routine: Some(M_ReadThis), alphaKey: b'r' },
	menuitem_t { status: 1, name: *b"M_QUITG\0\0\0", routine: Some(M_QuitDOOM), alphaKey: b'q' },
];

// Another hickup with Special edition.
#[allow(static_mut_refs)]
static mut MainDef: menu_t = menu_t {
	numitems: main_e::main_end as i16,
	prevMenu: null_mut(),
	menuitems: unsafe { MainMenu.as_mut_ptr() },
	routine: M_DrawMainMenu,
	x: 97,
	y: 64,
	lastOn: 0,
};

// EPISODE SELECT
#[repr(C)]
enum episodes_e {
	ep1,
	_ep2,
	_ep3,
	_ep4,
	ep_end,
}

static mut EpisodeMenu: [menuitem_t; 4] = [
	menuitem_t { status: 1, name: *b"M_EPI1\0\0\0\0", routine: Some(M_Episode), alphaKey: b'k' },
	menuitem_t { status: 1, name: *b"M_EPI2\0\0\0\0", routine: Some(M_Episode), alphaKey: b't' },
	menuitem_t { status: 1, name: *b"M_EPI3\0\0\0\0", routine: Some(M_Episode), alphaKey: b'i' },
	menuitem_t { status: 1, name: *b"M_EPI4\0\0\0\0", routine: Some(M_Episode), alphaKey: b't' },
];

#[allow(static_mut_refs)]
static mut EpiDef: menu_t = menu_t {
	numitems: episodes_e::ep_end as i16,            // # of menu items
	prevMenu: &raw mut MainDef,                     // previous menu
	menuitems: unsafe { EpisodeMenu.as_mut_ptr() }, // menuitem_t ->
	routine: M_DrawEpisode,                         // drawing routine ->
	x: 48,
	y: 63,                          // x,y
	lastOn: episodes_e::ep1 as i16, // lastOn
};

// NEW GAME
#[repr(C)]
#[derive(PartialEq, Eq)]
enum newgame_e {
	_killthings,
	_toorough,
	hurtme,
	_violence,
	nightmare,
	newg_end,
}

static mut NewGameMenu: [menuitem_t; 5] = [
	menuitem_t { status: 1, name: *b"M_JKILL\0\0\0", routine: Some(M_ChooseSkill), alphaKey: b'i' },
	menuitem_t { status: 1, name: *b"M_ROUGH\0\0\0", routine: Some(M_ChooseSkill), alphaKey: b'h' },
	menuitem_t {
		status: 1,
		name: *b"M_HURT\0\0\0\0",
		routine: Some(M_ChooseSkill),
		alphaKey: b'h',
	},
	menuitem_t { status: 1, name: *b"M_ULTRA\0\0\0", routine: Some(M_ChooseSkill), alphaKey: b'u' },
	menuitem_t { status: 1, name: *b"M_NMARE\0\0\0", routine: Some(M_ChooseSkill), alphaKey: b'n' },
];

#[allow(static_mut_refs)]
static mut NewDef: menu_t = menu_t {
	numitems: newgame_e::newg_end as i16, // # of menu items
	prevMenu: &raw mut EpiDef,            // previous menu
	menuitems: unsafe { NewGameMenu.as_mut_ptr() }, // menuitem_t ->
	routine: M_DrawNewGame,               // drawing routine ->
	x: 48,
	y: 63,                            // x,y
	lastOn: newgame_e::hurtme as i16, // lastOn
};

// OPTIONS MENU
#[repr(C)]
enum options_e {
	_endgame,
	messages,
	detail,
	scrnsize,
	_option_empty1,
	mousesens,
	_option_empty2,
	_soundvol,
	opt_end,
}

static mut OptionsMenu: [menuitem_t; 8] = [
	menuitem_t { status: 1, name: *b"M_ENDGAM\0\0", routine: Some(M_EndGame), alphaKey: b'e' },
	menuitem_t {
		status: 1,
		name: *b"M_MESSG\0\0\0",
		routine: Some(M_ChangeMessages),
		alphaKey: b'm',
	},
	menuitem_t { status: 1, name: *b"M_DETAIL\0\0", routine: Some(M_ChangeDetail), alphaKey: b'g' },
	menuitem_t { status: 2, name: *b"M_SCRNSZ\0\0", routine: Some(M_SizeDisplay), alphaKey: b's' },
	menuitem_t { status: -1, name: [0; 10], routine: None, alphaKey: 0 },
	menuitem_t {
		status: 2,
		name: *b"M_MSENS\0\0\0",
		routine: Some(M_ChangeSensitivity),
		alphaKey: b'm',
	},
	menuitem_t { status: -1, name: [0; 10], routine: None, alphaKey: 0 },
	menuitem_t { status: 1, name: *b"M_SVOL\0\0\0\0", routine: Some(M_Sound), alphaKey: b's' },
];

#[allow(static_mut_refs)]
static mut OptionsDef: menu_t = menu_t {
	numitems: options_e::opt_end as short,
	prevMenu: &raw mut MainDef,
	menuitems: unsafe { OptionsMenu.as_mut_ptr() },
	routine: M_DrawOptions,
	x: 60,
	y: 37,
	lastOn: 0,
};

// Read This! MENU 1 & 2
enum read_e {
	_rdthsempty1,
	read1_end,
}

static mut ReadMenu1: [menuitem_t; 1] =
	[menuitem_t { status: 1, name: [0; 10], routine: Some(M_ReadThis2), alphaKey: 0 }];

#[allow(static_mut_refs)]
static mut ReadDef1: menu_t = menu_t {
	numitems: read_e::read1_end as short,
	prevMenu: &raw mut MainDef,
	menuitems: unsafe { ReadMenu1.as_mut_ptr() },
	routine: M_DrawReadThis1,
	x: 280,
	y: 185,
	lastOn: 0,
};

enum read_e2 {
	_rdthsempty2,
	read2_end,
}

static mut ReadMenu2: [menuitem_t; 1] =
	[menuitem_t { status: 1, name: [0; 10], routine: Some(M_FinishReadThis), alphaKey: 0 }];

#[allow(static_mut_refs)]
static mut ReadDef2: menu_t = menu_t {
	numitems: read_e2::read2_end as short,
	prevMenu: &raw mut ReadDef1,
	menuitems: unsafe { ReadMenu2.as_mut_ptr() },
	routine: M_DrawReadThis2,
	x: 330,
	y: 175,
	lastOn: 0,
};

// SOUND VOLUME MENU
enum sound_e {
	sfx_vol,
	_sfx_empty1,
	music_vol,
	_sfx_empty2,
	sound_end,
}

static mut SoundMenu: [menuitem_t; 4] = [
	menuitem_t { status: 2, name: *b"M_SFXVOL\0\0", routine: Some(M_SfxVol), alphaKey: b's' },
	menuitem_t { status: -1, name: [0; 10], routine: None, alphaKey: 0 },
	menuitem_t { status: 2, name: *b"M_MUSVOL\0\0", routine: Some(M_MusicVol), alphaKey: b'm' },
	menuitem_t { status: -1, name: [0; 10], routine: None, alphaKey: 0 },
];

#[allow(static_mut_refs)]
static mut SoundDef: menu_t = menu_t {
	numitems: sound_e::sound_end as short,
	prevMenu: &raw mut OptionsDef,
	menuitems: unsafe { SoundMenu.as_mut_ptr() },
	routine: M_DrawSound,
	x: 80,
	y: 64,
	lastOn: 0,
};

// LOAD GAME MENU
enum load_e {
	_load1,
	_load2,
	_load3,
	_load4,
	_load5,
	_load6,
	load_end,
}

static mut LoadMenu: [menuitem_t; 6] = [
	menuitem_t { status: 1, name: [0; 10], routine: Some(M_LoadSelect), alphaKey: b'1' },
	menuitem_t { status: 1, name: [0; 10], routine: Some(M_LoadSelect), alphaKey: b'2' },
	menuitem_t { status: 1, name: [0; 10], routine: Some(M_LoadSelect), alphaKey: b'3' },
	menuitem_t { status: 1, name: [0; 10], routine: Some(M_LoadSelect), alphaKey: b'4' },
	menuitem_t { status: 1, name: [0; 10], routine: Some(M_LoadSelect), alphaKey: b'5' },
	menuitem_t { status: 1, name: [0; 10], routine: Some(M_LoadSelect), alphaKey: b'6' },
];

#[allow(static_mut_refs)]
static mut LoadDef: menu_t = menu_t {
	numitems: load_e::load_end as short,
	prevMenu: &raw mut MainDef,
	menuitems: unsafe { LoadMenu.as_mut_ptr() },
	routine: M_DrawLoad,
	x: 80,
	y: 54,
	lastOn: 0,
};

// SAVE GAME MENU
static mut SaveMenu: [menuitem_t; 6] = [
	menuitem_t { status: 1, name: [0; 10], routine: Some(M_SaveSelect), alphaKey: b'1' },
	menuitem_t { status: 1, name: [0; 10], routine: Some(M_SaveSelect), alphaKey: b'2' },
	menuitem_t { status: 1, name: [0; 10], routine: Some(M_SaveSelect), alphaKey: b'3' },
	menuitem_t { status: 1, name: [0; 10], routine: Some(M_SaveSelect), alphaKey: b'4' },
	menuitem_t { status: 1, name: [0; 10], routine: Some(M_SaveSelect), alphaKey: b'5' },
	menuitem_t { status: 1, name: [0; 10], routine: Some(M_SaveSelect), alphaKey: b'6' },
];

#[allow(static_mut_refs)]
static mut SaveDef: menu_t = menu_t {
	numitems: load_e::load_end as short,
	prevMenu: &raw mut MainDef,
	menuitems: unsafe { SaveMenu.as_mut_ptr() },
	routine: M_DrawSave,
	x: 80,
	y: 54,
	lastOn: 0,
};

// M_ReadSaveStrings
//  read the strings from the savegame files
fn M_ReadSaveStrings() {
	unsafe {
		let mut name = [0; 256];

		for i in 0..load_e::load_end as usize {
			if M_CheckParm(c"-cdrom".as_ptr()) != 0 {
				libc::sprintf(
					name.as_mut_ptr(),
					c"c:\\doomdata\\%s%d.dsg".as_ptr(),
					SAVEGAMENAME,
					i,
				);
			} else {
				libc::sprintf(name.as_mut_ptr(), c"%s%d.dsg".as_ptr(), SAVEGAMENAME, i);
			}

			let handle = libc::open(name.as_ptr(), O_RDONLY, 0o666);
			if handle == -1 {
				libc::strcpy(savegamestrings[i].as_mut_ptr(), EMPTYSTRING);
				LoadMenu[i].status = 0;
				continue;
			}
			libc::read(handle, savegamestrings[i].as_mut_ptr().cast(), SAVESTRINGSIZE);
			libc::close(handle);
			LoadMenu[i].status = 1;
		}
	}
}

// M_LoadGame & Cie.
fn M_DrawLoad() {
	unsafe {
		V_DrawPatchDirect(72, 28, 0, W_CacheLumpName(c"M_LOADG".as_ptr(), PU_CACHE).cast());
		#[allow(clippy::needless_range_loop)]
		for i in 0..load_e::load_end as usize {
			M_DrawSaveLoadBorder(LoadDef.x as usize, LoadDef.y as usize + LINEHEIGHT * i);
			M_WriteText(
				LoadDef.x as usize,
				LoadDef.y as usize + LINEHEIGHT * i,
				savegamestrings[i].as_ptr(),
			);
		}
	}
}

// Draw border for the savegame description
fn M_DrawSaveLoadBorder(mut x: usize, y: usize) {
	unsafe {
		V_DrawPatchDirect(x - 8, y + 7, 0, W_CacheLumpName(c"M_LSLEFT".as_ptr(), PU_CACHE).cast());

		for _ in 0..24 {
			V_DrawPatchDirect(x, y + 7, 0, W_CacheLumpName(c"M_LSCNTR".as_ptr(), PU_CACHE).cast());
			x += 8;
		}

		V_DrawPatchDirect(x, y + 7, 0, W_CacheLumpName(c"M_LSRGHT".as_ptr(), PU_CACHE).cast());
	}
}

// User wants to load this game
fn M_LoadSelect(choice: i32) {
	unsafe {
		let mut name = [0; 256];

		if M_CheckParm(c"-cdrom".as_ptr()) != 0 {
			libc::sprintf(
				name.as_mut_ptr(),
				c"c:\\doomdata\\%s%d.dsg".as_ptr(),
				SAVEGAMENAME,
				choice,
			);
		} else {
			libc::sprintf(name.as_mut_ptr(), c"%s%d.dsg".as_ptr(), SAVEGAMENAME, choice);
		}
		G_LoadGame(name.as_ptr());
		M_ClearMenus();
	}
}

// Selected from DOOM menu
fn M_LoadGame(_choice: i32) {
	unsafe {
		if netgame != 0 {
			M_StartMessage(LOADNET, null_mut(), false);
			return;
		}
		M_SetupNextMenu(&raw mut LoadDef);
		M_ReadSaveStrings();
	}
}

//  M_SaveGame & Cie.
fn M_DrawSave() {
	unsafe {
		V_DrawPatchDirect(72, 28, 0, W_CacheLumpName(c"M_SAVEG".as_ptr(), PU_CACHE).cast());
		#[allow(clippy::needless_range_loop)]
		for i in 0..load_e::load_end as usize {
			M_DrawSaveLoadBorder(LoadDef.x as usize, LoadDef.y as usize + LINEHEIGHT * i);
			M_WriteText(
				LoadDef.x as usize,
				LoadDef.y as usize + LINEHEIGHT * i,
				savegamestrings[i].as_ptr(),
			);
		}

		if saveStringEnter != 0 {
			let i = M_StringWidth(savegamestrings[saveSlot].as_ptr());
			M_WriteText(
				LoadDef.x as usize + i,
				LoadDef.y as usize + LINEHEIGHT * saveSlot,
				c"_".as_ptr(),
			);
		}
	}
}

// M_Responder calls this when user is finished
fn M_DoSave(slot: usize) {
	unsafe {
		G_SaveGame(slot, savegamestrings[slot].as_ptr());
		M_ClearMenus();

		// PICK QUICKSAVE SLOT YET?
		if quickSaveSlot == -2 {
			quickSaveSlot = slot as i32;
		}
	}
}

// User wants to save. Start string input for M_Responder
#[allow(static_mut_refs)]
fn M_SaveSelect(choice: i32) {
	unsafe {
		// we are going to be intercepting all chars
		saveStringEnter = 1;

		let choice = choice as usize;
		saveSlot = choice;
		libc::strcpy(saveOldString.as_mut_ptr(), savegamestrings[choice].as_ptr());
		if libc::strcmp(savegamestrings[choice].as_ptr(), EMPTYSTRING) == 0 {
			savegamestrings[choice][0] = 0;
		}
		saveCharIndex = libc::strlen(savegamestrings[choice].as_ptr());
	}
}

// Selected from DOOM menu
fn M_SaveGame(_choice: i32) {
	unsafe {
		if usergame == 0 {
			M_StartMessage(SAVEDEAD, null_mut(), false);
			return;
		}

		if gamestate != gamestate_t::GS_LEVEL {
			return;
		}
		M_SetupNextMenu(&raw mut SaveDef);
		M_ReadSaveStrings();
	}
}

//      M_QuickSave
static mut tempstring: [c_char; 80] = [0; 80];

fn M_QuickSaveResponse(ch: u8) {
	if ch == b'y' {
		unsafe { M_DoSave(quickSaveSlot as usize) };
		S_StartSound(null_mut(), sfxenum_t::sfx_swtchx);
	}
}

#[allow(static_mut_refs)]
fn M_QuickSave() {
	unsafe {
		if usergame == 0 {
			S_StartSound(null_mut(), sfxenum_t::sfx_oof);
			return;
		}

		if gamestate != gamestate_t::GS_LEVEL {
			return;
		}

		if quickSaveSlot < 0 {
			M_StartControlPanel();
			M_ReadSaveStrings();
			M_SetupNextMenu(&raw mut SaveDef);
			quickSaveSlot = -2; // means to pick a slot now
			return;
		}
		libc::sprintf(tempstring.as_mut_ptr(), QSPROMPT, savegamestrings[quickSaveSlot as usize]);
		M_StartMessage(tempstring.as_ptr(), M_QuickSaveResponse as *mut c_void, true);
	}
}

// M_QuickLoad
fn M_QuickLoadResponse(ch: u8) {
	if ch == b'y' {
		unsafe { M_LoadSelect(quickSaveSlot) };
		S_StartSound(null_mut(), sfxenum_t::sfx_swtchx);
	}
}

#[allow(static_mut_refs)]
fn M_QuickLoad() {
	unsafe {
		if netgame != 0 {
			M_StartMessage(QLOADNET, null_mut(), false);
			return;
		}

		if quickSaveSlot < 0 {
			M_StartMessage(QSAVESPOT, null_mut(), false);
			return;
		}

		libc::sprintf(tempstring.as_mut_ptr(), QLPROMPT, savegamestrings[quickSaveSlot as usize]);
		M_StartMessage(tempstring.as_ptr(), M_QuickLoadResponse as *mut c_void, true);
	}
}

// Read This Menus
// Had a "quick hack to fix romero bug"
fn M_DrawReadThis1() {
	unsafe {
		inhelpscreens = true;
		match gamemode {
			GameMode_t::commercial => {
				V_DrawPatchDirect(0, 0, 0, W_CacheLumpName(c"HELP".as_ptr(), PU_CACHE).cast())
			}
			GameMode_t::shareware | GameMode_t::registered | GameMode_t::retail => {
				V_DrawPatchDirect(0, 0, 0, W_CacheLumpName(c"HELP1".as_ptr(), PU_CACHE).cast())
			}
			GameMode_t::indetermined => (),
		}
	}
}

// Read This Menus - optional second page.
fn M_DrawReadThis2() {
	unsafe {
		inhelpscreens = true;
		match gamemode {
			GameMode_t::retail | GameMode_t::commercial => {
				V_DrawPatchDirect(0, 0, 0, W_CacheLumpName(c"CREDIT".as_ptr(), PU_CACHE).cast())
			}
			GameMode_t::shareware | GameMode_t::registered => {
				V_DrawPatchDirect(0, 0, 0, W_CacheLumpName(c"HELP2".as_ptr(), PU_CACHE).cast())
			}
			GameMode_t::indetermined => (),
		}
	}
}

// Change Sfx & Music volumes
fn M_DrawSound() {
	unsafe {
		V_DrawPatchDirect(60, 38, 0, W_CacheLumpName(c"M_SVOL".as_ptr(), PU_CACHE).cast());

		M_DrawThermo(
			SoundDef.x as usize,
			SoundDef.y as usize + LINEHEIGHT * (sound_e::sfx_vol as usize + 1),
			16,
			snd_MusicVolume,
		);

		M_DrawThermo(
			SoundDef.x as usize,
			SoundDef.y as usize + LINEHEIGHT * (sound_e::music_vol as usize + 1),
			16,
			snd_MusicVolume,
		);
	}
}

fn M_Sound(_choice: i32) {
	M_SetupNextMenu(&raw mut SoundDef);
}

fn M_SfxVol(choice: i32) {
	unsafe {
		match choice {
			0 => snd_SfxVolume = snd_SfxVolume.saturating_sub(1),
			1 => {
				if snd_SfxVolume < 15 {
					snd_SfxVolume += 1;
				}
			}
			_ => (),
		}

		S_SetSfxVolume(snd_SfxVolume /* *8 */);
	}
}

fn M_MusicVol(choice: i32) {
	unsafe {
		match choice {
			0 => snd_MusicVolume = snd_MusicVolume.saturating_sub(1),
			1 => {
				if snd_MusicVolume < 15 {
					snd_MusicVolume += 1;
				}
			}
			_ => (),
		}

		S_SetMusicVolume(snd_MusicVolume /* *8 */);
	}
}

// M_DrawMainMenu
fn M_DrawMainMenu() {
	unsafe {
		V_DrawPatchDirect(94, 2, 0, W_CacheLumpName(c"M_DOOM".as_ptr(), PU_CACHE).cast());
	}
}

// M_NewGame
fn M_DrawNewGame() {
	unsafe {
		V_DrawPatchDirect(96, 14, 0, W_CacheLumpName(c"M_NEWG".as_ptr(), PU_CACHE).cast());
		V_DrawPatchDirect(54, 38, 0, W_CacheLumpName(c"M_SKILL".as_ptr(), PU_CACHE).cast());
	}
}

fn M_NewGame(_choice: i32) {
	unsafe {
		if netgame != 0 && demoplayback == 0 {
			M_StartMessage(NEWGAME, null_mut(), false);
			return;
		}

		if gamemode == GameMode_t::commercial {
			M_SetupNextMenu(&raw mut NewDef);
		} else {
			M_SetupNextMenu(&raw mut EpiDef);
		}
	}
}

//      M_Episode
static mut epi: usize = 0;

fn M_DrawEpisode() {
	unsafe {
		V_DrawPatchDirect(54, 38, 0, W_CacheLumpName(c"M_EPISOD".as_ptr(), PU_CACHE).cast());
	}
}

fn M_VerifyNightmare(ch: u8) {
	if ch != b'y' {
		return;
	}

	unsafe { G_DeferedInitNew(skill_t::sk_nightmare, epi + 1, 1) };
	M_ClearMenus();
}

fn M_ChooseSkill(choice: i32) {
	unsafe {
		if choice == newgame_e::nightmare as i32 {
			M_StartMessage(NIGHTMARE, M_VerifyNightmare as *const c_void, true);
			return;
		}

		G_DeferedInitNew(skill_t::from(choice as u8), epi + 1, 1);
		M_ClearMenus();
	}
}

fn M_Episode(mut choice: i32) {
	unsafe {
		if gamemode == GameMode_t::shareware && choice != 0 {
			M_StartMessage(SWSTRING, null_mut(), false);
			M_SetupNextMenu(&raw mut ReadDef1);
			return;
		}

		// Yet another hack...
		if gamemode == GameMode_t::registered && choice > 2 {
			eprintln!("M_Episode: 4th episode requires UltimateDOOM");
			choice = 0;
		}

		epi = choice as usize;
		M_SetupNextMenu(&raw mut NewDef);
	}
}

// M_Options
static detailNames: [[u8; 9]; 2] = [*b"M_GDHIGH\0", *b"M_GDLOW\0\0"];
static msgNames: [[u8; 9]; 2] = [*b"M_MSGOFF\0", *b"M_MSGON\0\0"];

fn M_DrawOptions() {
	unsafe {
		V_DrawPatchDirect(108, 15, 0, W_CacheLumpName(c"M_OPTTTL".as_ptr(), PU_CACHE).cast());

		V_DrawPatchDirect(
			OptionsDef.x as usize + 175,
			OptionsDef.y as usize + LINEHEIGHT * options_e::detail as usize,
			0,
			W_CacheLumpName(detailNames[detailLevel as usize].as_ptr().cast(), PU_CACHE).cast(),
		);

		V_DrawPatchDirect(
			OptionsDef.x as usize + 120,
			OptionsDef.y as usize + LINEHEIGHT * options_e::messages as usize,
			0,
			W_CacheLumpName(msgNames[showMessages as usize].as_ptr().cast(), PU_CACHE).cast(),
		);

		M_DrawThermo(
			OptionsDef.x as usize,
			OptionsDef.y as usize + LINEHEIGHT * (options_e::mousesens as usize + 1),
			10,
			mouseSensitivity as usize,
		);

		M_DrawThermo(
			OptionsDef.x as usize,
			OptionsDef.y as usize + LINEHEIGHT * (options_e::scrnsize as usize + 1),
			9,
			screenSize as usize,
		);
	}
}

fn M_Options(_choice: i32) {
	M_SetupNextMenu(&raw mut OptionsDef);
}

//      Toggle messages on/off
fn M_ChangeMessages(_choice: i32) {
	unsafe {
		showMessages = 1 - showMessages;

		if showMessages == 0 {
			players[consoleplayer].message = MSGOFF;
		} else {
			players[consoleplayer].message = MSGON;
		}

		message_dontfuckwithme = true;
	}
}

// M_EndGame
fn M_EndGameResponse(ch: u8) {
	if ch != b'y' {
		return;
	}
	unsafe { (*currentMenu).lastOn = itemOn };
	M_ClearMenus();
	D_StartTitle();
}

fn M_EndGame(_choice: i32) {
	unsafe {
		if usergame == 0 {
			S_StartSound(null_mut(), sfxenum_t::sfx_oof);
			return;
		}

		if netgame != 0 {
			M_StartMessage(NETEND, null_mut(), false);
			return;
		}

		M_StartMessage(ENDGAME, M_EndGameResponse as *mut c_void, true);
	}
}

// M_ReadThis
fn M_ReadThis(_choice: i32) {
	M_SetupNextMenu(&raw mut ReadDef1);
}

fn M_ReadThis2(_choice: i32) {
	M_SetupNextMenu(&raw mut ReadDef2);
}

fn M_FinishReadThis(_choice: i32) {
	M_SetupNextMenu(&raw mut MainDef);
}

// M_QuitDOOM
static quitsounds: [sfxenum_t; 8] = [
	sfxenum_t::sfx_pldeth,
	sfxenum_t::sfx_dmpain,
	sfxenum_t::sfx_popain,
	sfxenum_t::sfx_slop,
	sfxenum_t::sfx_telept,
	sfxenum_t::sfx_posit1,
	sfxenum_t::sfx_posit3,
	sfxenum_t::sfx_sgtatk,
];

static quitsounds2: [sfxenum_t; 8] = [
	sfxenum_t::sfx_vilact,
	sfxenum_t::sfx_getpow,
	sfxenum_t::sfx_boscub,
	sfxenum_t::sfx_slop,
	sfxenum_t::sfx_skeswg,
	sfxenum_t::sfx_kntdth,
	sfxenum_t::sfx_bspact,
	sfxenum_t::sfx_sgtatk,
];

fn M_QuitResponse(ch: u8) {
	unsafe {
		if ch != b'y' {
			return;
		}
		if netgame == 0 {
			if gamemode == GameMode_t::commercial {
				S_StartSound(null_mut(), quitsounds2[(gametic as usize >> 2) & 7]);
			} else {
				S_StartSound(null_mut(), quitsounds[(gametic as usize >> 2) & 7]);
			}
			I_WaitVBL(105);
		}
		I_Quit();
	}
}

#[allow(static_mut_refs)]
fn M_QuitDOOM(_choice: i32) {
	unsafe {
		// We pick index 0 which is language sensitive,
		//  or one at random, between 1 and maximum number.
		if language != Language_t::english {
			libc::sprintf(endstring.as_mut_ptr(), c"%s\n\n%s".as_ptr(), endmsg[0], DOSY!());
		} else {
			libc::sprintf(
				endstring.as_mut_ptr(),
				c"%s\n\n%s".as_ptr(),
				endmsg[(gametic as usize % (NUM_QUITMESSAGES - 2)) + 1],
				DOSY!(),
			);
		}

		M_StartMessage(endstring.as_ptr(), M_QuitResponse as *const c_void, true);
	}
}

fn M_ChangeSensitivity(choice: i32) {
	unsafe {
		match choice {
			0 => {
				if mouseSensitivity != 0 {
					mouseSensitivity -= 1;
				}
			}
			1 => {
				if mouseSensitivity < 9 {
					mouseSensitivity += 1;
				}
			}
			_ => (),
		}
	}
}

fn M_ChangeDetail(_choice: i32) {
	unsafe {
		detailLevel = 1 - detailLevel;
	}

	// FIXME - does not work. Remove anyway?
	eprintln!("M_ChangeDetail: low detail mode n.a.");
}

unsafe extern "C" {
	fn R_SetViewSize(blocks: i32, detail: i32);
}

fn M_SizeDisplay(choice: i32) {
	unsafe {
		match choice {
			0 => {
				if screenSize > 0 {
					screenblocks -= 1;
					screenSize -= 1;
				}
			}
			1 => {
				if screenSize < 8 {
					screenblocks += 1;
					screenSize += 1;
				}
			}
			_ => (),
		}

		R_SetViewSize(screenblocks, detailLevel);
	}
}

//      Menu Functions
fn M_DrawThermo(x: usize, y: usize, thermWidth: usize, thermDot: usize) {
	unsafe {
		let mut xx = x;
		V_DrawPatchDirect(xx, y, 0, W_CacheLumpName(c"M_THERML".as_ptr(), PU_CACHE).cast());
		xx += 8;
		for _ in 0..thermWidth {
			V_DrawPatchDirect(xx, y, 0, W_CacheLumpName(c"M_THERMM".as_ptr(), PU_CACHE).cast());
			xx += 8;
		}
		V_DrawPatchDirect(xx, y, 0, W_CacheLumpName(c"M_THERMR".as_ptr(), PU_CACHE).cast());

		V_DrawPatchDirect(
			(x + 8) + thermDot * 8,
			y,
			0,
			W_CacheLumpName(c"M_THERMO".as_ptr(), PU_CACHE).cast(),
		);
	}
}

fn M_StartMessage(string: *const c_char, routine: *const c_void, input: bool) {
	unsafe {
		messageLastMenuActive = menuactive;
		messageToPrint = 1;
		messageString = string;
		messageRoutine = mem::transmute::<*const c_void, Option<fn(i32)>>(routine);
		messageNeedsInput = input;
		menuactive = true;
	}
}

// Find string width from hu_font chars
fn M_StringWidth(string: *const c_char) -> usize {
	unsafe {
		let mut w = 0;

		for i in 0..libc::strlen(string) {
			let c = libc::toupper(*string.wrapping_add(i) as i32) - HU_FONTSTART as i32;
			if c < 0 || c >= HU_FONTSIZE as i32 {
				w += 4;
			} else {
				w += (*hu_font[c as usize]).width as usize;
			}
		}

		w
	}
}

//      Find string height from hu_font chars
fn M_StringHeight(string: *const c_char) -> usize {
	unsafe {
		let height = (*hu_font[0]).height as usize;

		let mut h = height;
		for i in 0..libc::strlen(string) {
			if *string.wrapping_add(i) == b'\n' as c_char {
				h += height;
			}
		}

		h
	}
}

//      Write a string using the hu_font
fn M_WriteText(x: usize, y: usize, string: *const c_char) {
	unsafe {
		let mut ch = string;
		let mut cx = x;
		let mut cy = y;

		loop {
			let c = *ch;
			ch = ch.wrapping_add(1);
			if c == 0 {
				break;
			}
			if c == b'\n' as c_char {
				cx = x;
				cy += 12;
				continue;
			}

			let c = libc::toupper(c as i32) - HU_FONTSTART as i32;
			if c < 0 || c >= HU_FONTSIZE as i32 {
				cx += 4;
				continue;
			}

			let w = (*hu_font[c as usize]).width as usize;
			if cx + w > SCREENWIDTH {
				break;
			}
			V_DrawPatchDirect(cx, cy, 0, hu_font[c as usize]);
			cx += w;
		}
	}
}

// CONTROL PANEL

unsafe extern "C" {
	fn I_SetPalette(palette: *mut u8);
}

// M_Responder
#[allow(static_mut_refs)]
pub(crate) fn M_Responder(ev: &mut event_t) -> bool {
	unsafe {
		static mut joywait: usize = 0;
		static mut mousewait: usize = 0;
		static mut mousey: i32 = 0;
		static mut lasty: i32 = 0;
		static mut mousex: i32 = 0;
		static mut lastx: i32 = 0;

		let mut ch = -1;

		if ev.ty == evtype_t::ev_joystick && joywait < I_GetTime() {
			if ev.data3 == -1 {
				ch = KEY_UPARROW as i32;
				joywait = I_GetTime() + 5;
			} else if ev.data3 == 1 {
				ch = KEY_DOWNARROW as i32;
				joywait = I_GetTime() + 5;
			}

			if ev.data2 == -1 {
				ch = KEY_LEFTARROW as i32;
				joywait = I_GetTime() + 2;
			} else if ev.data2 == 1 {
				ch = KEY_RIGHTARROW as i32;
				joywait = I_GetTime() + 2;
			}

			if ev.data1 & 1 != 0 {
				ch = KEY_ENTER as i32;
				joywait = I_GetTime() + 5;
			}
			if ev.data1 & 2 != 0 {
				ch = KEY_BACKSPACE as i32;
				joywait = I_GetTime() + 5;
			}
		} else if ev.ty == evtype_t::ev_mouse && mousewait < I_GetTime() {
			mousey += ev.data3;
			if mousey < lasty - 30 {
				ch = KEY_DOWNARROW as i32;
				mousewait = I_GetTime() + 5;
				lasty -= 30;
				mousey = lasty;
			} else if mousey > lasty + 30 {
				ch = KEY_UPARROW as i32;
				mousewait = I_GetTime() + 5;
				lasty += 30;
				mousey = lasty;
			}

			mousex += ev.data2;
			if mousex < lastx - 30 {
				ch = KEY_LEFTARROW as i32;
				mousewait = I_GetTime() + 5;
				lastx -= 30;
				mousex = lastx;
			} else if mousex > lastx + 30 {
				ch = KEY_RIGHTARROW as i32;
				mousewait = I_GetTime() + 5;
				lastx += 30;
				mousex = lastx;
			}

			if ev.data1 & 1 != 0 {
				ch = KEY_ENTER as i32;
				mousewait = I_GetTime() + 15;
			}

			if ev.data1 & 2 != 0 {
				ch = KEY_BACKSPACE as i32;
				mousewait = I_GetTime() + 15;
			}
		} else if ev.ty == evtype_t::ev_keydown {
			ch = ev.data1;
		}

		if ch == -1 {
			return false;
		}

		// Save Game string input
		if saveStringEnter != 0 {
			match ch {
				_ if ch == KEY_BACKSPACE as i32 => {
					if saveCharIndex > 0 {
						saveCharIndex -= 1;
						savegamestrings[saveSlot][saveCharIndex] = 0;
					}
				}

				_ if ch == KEY_ESCAPE as i32 => {
					saveStringEnter = 0;
					libc::strcpy(savegamestrings[saveSlot].as_mut_ptr(), saveOldString.as_ptr());
				}

				_ if ch == KEY_ENTER as i32 => {
					saveStringEnter = 0;
					if savegamestrings[saveSlot][0] != 0 {
						M_DoSave(saveSlot);
					}
				}

				_ => {
					ch = libc::toupper(ch);
					if ch >= HU_FONTSTART as i32
						&& ch < (HU_FONTSTART + HU_FONTSIZE) as i32
						&& (32..=127).contains(&ch)
						&& saveCharIndex < SAVESTRINGSIZE - 1
						&& M_StringWidth(savegamestrings[saveSlot].as_ptr())
							< (SAVESTRINGSIZE - 2) * 8
					{
						savegamestrings[saveSlot][saveCharIndex] = ch as c_char;
						saveCharIndex += 1;
						savegamestrings[saveSlot][saveCharIndex] = 0;
					}
				}
			}
			return true;
		}

		// Take care of any messages that need input
		if messageToPrint != 0 {
			if messageNeedsInput
				&& !(ch == b' ' as i32
					|| ch == b'n' as i32
					|| ch == b'y' as i32
					|| ch == KEY_ESCAPE as i32)
			{
				return false;
			}

			menuactive = messageLastMenuActive;
			messageToPrint = 0;
			if let Some(routine) = messageRoutine {
				routine(ch);
			}

			menuactive = false;
			S_StartSound(null_mut(), sfxenum_t::sfx_swtchx);
			return true;
		}

		if devparm != 0 && ch == KEY_F1 as i32 {
			G_ScreenShot();
			return true;
		}

		// F-Keys
		if !menuactive {
			match ch {
				_ if ch == KEY_MINUS as i32 => {
					// Screen size down
					if automapactive != 0 || chat_on != 0 {
						return false;
					}
					M_SizeDisplay(0);
					S_StartSound(null_mut(), sfxenum_t::sfx_stnmov);
					return true;
				}

				_ if ch == KEY_EQUALS as i32 => {
					// Screen size up
					if automapactive != 0 || chat_on != 0 {
						return false;
					}
					M_SizeDisplay(1);
					S_StartSound(null_mut(), sfxenum_t::sfx_stnmov);
					return true;
				}

				_ if ch == KEY_F1 as i32 => {
					// Help key
					M_StartControlPanel();

					if gamemode == GameMode_t::retail {
						currentMenu = &raw mut ReadDef2;
					} else {
						currentMenu = &raw mut ReadDef1;
					}

					itemOn = 0;
					S_StartSound(null_mut(), sfxenum_t::sfx_swtchn);
					return true;
				}

				_ if ch == KEY_F2 as i32 => {
					// Save
					M_StartControlPanel();
					S_StartSound(null_mut(), sfxenum_t::sfx_swtchn);
					M_SaveGame(0);
					return true;
				}

				_ if ch == KEY_F3 as i32 => {
					// Load
					M_StartControlPanel();
					S_StartSound(null_mut(), sfxenum_t::sfx_swtchn);
					M_LoadGame(0);
					return true;
				}

				_ if ch == KEY_F4 as i32 => {
					// Sound Volume
					M_StartControlPanel();
					currentMenu = &raw mut SoundDef;
					itemOn = sound_e::sfx_vol as short;
					S_StartSound(null_mut(), sfxenum_t::sfx_swtchn);
					return true;
				}

				_ if ch == KEY_F5 as i32 => {
					// Detail toggle
					M_ChangeDetail(0);
					S_StartSound(null_mut(), sfxenum_t::sfx_swtchn);
					return true;
				}

				_ if ch == KEY_F6 as i32 => {
					// Quicksave
					S_StartSound(null_mut(), sfxenum_t::sfx_swtchn);
					M_QuickSave();
					return true;
				}

				_ if ch == KEY_F7 as i32 => {
					// End game
					S_StartSound(null_mut(), sfxenum_t::sfx_swtchn);
					M_EndGame(0);
					return true;
				}

				_ if ch == KEY_F8 as i32 => {
					// Toggle messages
					M_ChangeMessages(0);
					S_StartSound(null_mut(), sfxenum_t::sfx_swtchn);
					return true;
				}

				_ if ch == KEY_F9 as i32 => {
					// Quickload
					S_StartSound(null_mut(), sfxenum_t::sfx_swtchn);
					M_QuickLoad();
					return true;
				}

				_ if ch == KEY_F10 as i32 => {
					// Quit DOOM
					S_StartSound(null_mut(), sfxenum_t::sfx_swtchn);
					M_QuitDOOM(0);
					return true;
				}

				_ if ch == KEY_F11 as i32 => {
					// gamma toggle
					usegamma += 1;
					if usegamma > 4 {
						usegamma = 0;
					}
					players[consoleplayer].message = gammamsg[usegamma as usize].as_ptr().cast();
					I_SetPalette(W_CacheLumpName(c"PLAYPAL".as_ptr(), PU_CACHE).cast());
					return true;
				}

				_ => (),
			}
		}

		// Pop-up menu?
		if !menuactive {
			if ch == KEY_ESCAPE as i32 {
				M_StartControlPanel();
				S_StartSound(null_mut(), sfxenum_t::sfx_swtchn);
				return true;
			}
			return false;
		}

		// Keys usable within menu
		match ch {
			_ if ch == KEY_DOWNARROW as i32 => {
				loop {
					if itemOn + 1 > (*currentMenu).numitems - 1 {
						itemOn = 0;
					} else {
						itemOn += 1;
					}
					S_StartSound(null_mut(), sfxenum_t::sfx_pstop);
					if (*(*currentMenu).menuitems.wrapping_add(itemOn as usize)).status != -1 {
						break;
					}
				}
				return true;
			}

			_ if ch == KEY_UPARROW as i32 => {
				loop {
					if itemOn == 0 {
						itemOn = (*currentMenu).numitems - 1;
					} else {
						itemOn -= 1;
					}
					S_StartSound(null_mut(), sfxenum_t::sfx_pstop);
					if (*(*currentMenu).menuitems.wrapping_add(itemOn as usize)).status != -1 {
						break;
					}
				}
				return true;
			}

			_ if ch == KEY_LEFTARROW as i32 => {
				if let Some(routine) =
					(*(*currentMenu).menuitems.wrapping_add(itemOn as usize)).routine
				{
					if (*(*currentMenu).menuitems.wrapping_add(itemOn as usize)).status == 2 {
						S_StartSound(null_mut(), sfxenum_t::sfx_stnmov);
						routine(0);
					}
				}
				return true;
			}

			_ if ch == KEY_RIGHTARROW as i32 => {
				if let Some(routine) =
					(*(*currentMenu).menuitems.wrapping_add(itemOn as usize)).routine
				{
					if (*(*currentMenu).menuitems.wrapping_add(itemOn as usize)).status == 2 {
						S_StartSound(null_mut(), sfxenum_t::sfx_stnmov);
						routine(1);
					}
				}
				return true;
			}

			_ if ch == KEY_ENTER as i32 => {
				if let Some(routine) =
					(*(*currentMenu).menuitems.wrapping_add(itemOn as usize)).routine
				{
					if let status @ 1.. =
						(*(*currentMenu).menuitems.wrapping_add(itemOn as usize)).status
					{
						(*currentMenu).lastOn = itemOn;
						if status == 2 {
							routine(1); // right arrow
							S_StartSound(null_mut(), sfxenum_t::sfx_stnmov);
						} else {
							routine(itemOn as i32);
							S_StartSound(null_mut(), sfxenum_t::sfx_pistol);
						}
					}
				}
				return true;
			}

			_ if ch == KEY_ESCAPE as i32 => {
				(*currentMenu).lastOn = itemOn;
				M_ClearMenus();
				S_StartSound(null_mut(), sfxenum_t::sfx_swtchx);
				return true;
			}

			_ if ch == KEY_BACKSPACE as i32 => {
				(*currentMenu).lastOn = itemOn;
				if !(*currentMenu).prevMenu.is_null() {
					currentMenu = (*currentMenu).prevMenu;
					itemOn = (*currentMenu).lastOn;
					S_StartSound(null_mut(), sfxenum_t::sfx_swtchn);
				}
				return true;
			}

			_ => {
				for i in itemOn as usize + 1..(*currentMenu).numitems as usize {
					if (*(*currentMenu).menuitems.wrapping_add(i)).alphaKey as i32 == ch {
						itemOn = i as short;
						S_StartSound(null_mut(), sfxenum_t::sfx_pstop);
						return true;
					}
				}
				for i in 0..itemOn as usize {
					if (*(*currentMenu).menuitems.wrapping_add(i)).alphaKey as i32 == ch {
						itemOn = i as short;
						S_StartSound(null_mut(), sfxenum_t::sfx_pstop);
						return true;
					}
				}
			}
		}

		false
	}
}

// M_StartControlPanel
pub(crate) fn M_StartControlPanel() {
	unsafe {
		// intro might call this repeatedly
		if menuactive {
			return;
		}

		menuactive = true;
		currentMenu = &raw mut MainDef; // JDC
		itemOn = (*currentMenu).lastOn; // JDC
	}
}

// M_Drawer
// Called after the view has been rendered,
// but before it has been blitted.
pub(crate) fn M_Drawer() {
	unsafe {
		static mut x: short = 0;
		static mut y: short = 0;
		let mut string = [0; 40];

		inhelpscreens = false;

		// Horiz. & Vertically center string and print it.
		if messageToPrint != 0 {
			let mut start = 0;
			y = 100 - (M_StringHeight(messageString) / 2) as i16;
			while *messageString.wrapping_add(start) != 0 {
				let mut i = 0;
				for _ in 0..libc::strlen(messageString.wrapping_add(start)) {
					if *(messageString.wrapping_add(start + i)) == b'\n' as c_char {
						libc::memset(string.as_mut_ptr().cast(), 0, 40);
						libc::strncpy(string.as_mut_ptr(), messageString.wrapping_add(start), i);
						start += i + 1;
						break;
					}
					i += 1;
				}
				if i == libc::strlen(messageString.wrapping_add(start)) {
					libc::strcpy(string.as_mut_ptr(), messageString.wrapping_add(start));
					start += i;
				}

				x = 160 - (M_StringWidth(string.as_ptr()) / 2) as i16;
				M_WriteText(i16::max(x, 0) as usize, i16::max(y, 0) as usize, string.as_ptr());
				y += (*hu_font[0]).height;
			}
			return;
		}

		if !menuactive {
			return;
		}

		((*currentMenu).routine)();

		// DRAW MENU
		x = (*currentMenu).x;
		y = (*currentMenu).y;
		let max = (*currentMenu).numitems;

		for i in 0..max as usize {
			if (*(*currentMenu).menuitems.wrapping_add(i)).name[0] != 0 {
				V_DrawPatchDirect(
					x as usize,
					y as usize,
					0,
					W_CacheLumpName(
						(*(*currentMenu).menuitems.wrapping_add(i)).name.as_ptr().cast(),
						PU_CACHE,
					)
					.cast(),
				);
			}
			y += LINEHEIGHT as i16;
		}

		// DRAW SKULL
		V_DrawPatchDirect(
			(x as usize).wrapping_add_signed(SKULLXOFF),
			(*currentMenu).y as usize - 5 + itemOn as usize * LINEHEIGHT,
			0,
			W_CacheLumpName(skullName[whichSkull as usize].as_ptr().cast(), PU_CACHE).cast(),
		);
	}
}

// M_ClearMenus
fn M_ClearMenus() {
	unsafe {
		menuactive = false;
	}
}

// M_SetupNextMenu
fn M_SetupNextMenu(menudef: *mut menu_t) {
	unsafe {
		currentMenu = menudef;
		itemOn = (*currentMenu).lastOn;
	}
}

// M_Ticker
#[unsafe(no_mangle)]
pub extern "C" fn M_Ticker() {
	unsafe {
		skullAnimCounter -= 1;
		if skullAnimCounter <= 0 {
			whichSkull ^= 1;
			skullAnimCounter = 8;
		}
	}
}

// M_Init
pub(crate) fn M_Init() {
	unsafe {
		currentMenu = &raw mut MainDef;
		menuactive = false;
		itemOn = (*currentMenu).lastOn;
		whichSkull = 0;
		skullAnimCounter = 10;
		screenSize = screenblocks - 3;
		messageToPrint = 0;
		messageString = null();
		messageLastMenuActive = menuactive;
		quickSaveSlot = -1;

		// Here we could catch other version dependencies,
		//  like HELP1/2, and four episodes.

		match  gamemode
		{
			GameMode_t::commercial => {
			// This is used because DOOM 2 had only one HELP
			//  page. I use CREDIT as second page now, but
			//  kept this hack for educational purposes.
			MainMenu[main_e::readthis as usize] = MainMenu[main_e::quitdoom as usize];
			MainDef.numitems-=1;
			MainDef.y += 8;
			NewDef.prevMenu = &raw mut MainDef;
			ReadDef1.routine = M_DrawReadThis1;
			ReadDef1.x = 330;
			ReadDef1.y = 165;
			ReadMenu1[0].routine = Some(M_FinishReadThis);
			}
			GameMode_t::shareware |
			// Episode 2 and 3 are handled,
			//  branching to an ad screen.
			GameMode_t::registered =>
			// We need to remove the fourth episode.
			EpiDef.numitems-=1,
			// We are fine.
			_ => ()
		}
	}
}
