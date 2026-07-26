#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::{
	ffi::c_char,
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
	i_video::I_SetPalette,
	m_argv::M_CheckParm,
	r_main::R_SetViewSize,
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
pub static mut screenblocks: usize = 0; // has default

// temp for screenblocks (0-9)
static mut screenSize: usize = 0;

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
#[derive(Clone, Copy)]
enum main_e {
	_newgame = 0,
	_options,
	_loadgame,
	_savegame,
	readthis,
	quitdoom,
	main_end,
}

impl main_e {
	const fn to_u8(self) -> u8 {
		match self {
			main_e::_newgame => 0,
			main_e::_options => 1,
			main_e::_loadgame => 2,
			main_e::_savegame => 3,
			main_e::readthis => 4,
			main_e::quitdoom => 5,
			main_e::main_end => 6,
		}
	}

	const fn to_i16(self) -> i16 {
		match self {
			main_e::_newgame => 0,
			main_e::_options => 1,
			main_e::_loadgame => 2,
			main_e::_savegame => 3,
			main_e::readthis => 4,
			main_e::quitdoom => 5,
			main_e::main_end => 6,
		}
	}
}

impl From<main_e> for usize {
	fn from(value: main_e) -> Self {
		value.to_u8().into()
	}
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
	numitems: main_e::main_end.to_i16(),
	prevMenu: null_mut(),
	menuitems: unsafe { MainMenu.as_mut_ptr() },
	routine: M_DrawMainMenu,
	x: 97,
	y: 64,
	lastOn: 0,
};

// EPISODE SELECT
#[repr(C)]
#[derive(Clone, Copy)]
enum episodes_e {
	ep1,
	_ep2,
	_ep3,
	_ep4,
	ep_end,
}

impl episodes_e {
	const fn to_i16(self) -> i16 {
		match self {
			episodes_e::ep1 => 0,
			episodes_e::_ep2 => 1,
			episodes_e::_ep3 => 2,
			episodes_e::_ep4 => 3,
			episodes_e::ep_end => 4,
		}
	}
}

static mut EpisodeMenu: [menuitem_t; 4] = [
	menuitem_t { status: 1, name: *b"M_EPI1\0\0\0\0", routine: Some(M_Episode), alphaKey: b'k' },
	menuitem_t { status: 1, name: *b"M_EPI2\0\0\0\0", routine: Some(M_Episode), alphaKey: b't' },
	menuitem_t { status: 1, name: *b"M_EPI3\0\0\0\0", routine: Some(M_Episode), alphaKey: b'i' },
	menuitem_t { status: 1, name: *b"M_EPI4\0\0\0\0", routine: Some(M_Episode), alphaKey: b't' },
];

#[allow(static_mut_refs)]
static mut EpiDef: menu_t = menu_t {
	numitems: episodes_e::ep_end.to_i16(), // # of menu items
	prevMenu: &raw mut MainDef,            // previous menu
	menuitems: unsafe { EpisodeMenu.as_mut_ptr() }, // menuitem_t ->
	routine: M_DrawEpisode,                // drawing routine ->
	x: 48,
	y: 63,                            // x,y
	lastOn: episodes_e::ep1.to_i16(), // lastOn
};

// NEW GAME
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum newgame_e {
	_killthings,
	_toorough,
	hurtme,
	_violence,
	nightmare,
	newg_end,
}

impl newgame_e {
	const fn to_i16(self) -> i16 {
		match self {
			newgame_e::_killthings => 0,
			newgame_e::_toorough => 1,
			newgame_e::hurtme => 2,
			newgame_e::_violence => 3,
			newgame_e::nightmare => 4,
			newgame_e::newg_end => 5,
		}
	}
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
	numitems: newgame_e::newg_end.to_i16(), // # of menu items
	prevMenu: &raw mut EpiDef,              // previous menu
	menuitems: unsafe { NewGameMenu.as_mut_ptr() }, // menuitem_t ->
	routine: M_DrawNewGame,                 // drawing routine ->
	x: 48,
	y: 63,                              // x,y
	lastOn: newgame_e::hurtme.to_i16(), // lastOn
};

// OPTIONS MENU
#[repr(C)]
#[derive(Clone, Copy)]
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

impl options_e {
	const fn to_u8(self) -> u8 {
		match self {
			options_e::_endgame => 0,
			options_e::messages => 1,
			options_e::detail => 2,
			options_e::scrnsize => 3,
			options_e::_option_empty1 => 4,
			options_e::mousesens => 5,
			options_e::_option_empty2 => 6,
			options_e::_soundvol => 7,
			options_e::opt_end => 8,
		}
	}

	const fn to_i16(self) -> i16 {
		match self {
			options_e::_endgame => 0,
			options_e::messages => 1,
			options_e::detail => 2,
			options_e::scrnsize => 3,
			options_e::_option_empty1 => 4,
			options_e::mousesens => 5,
			options_e::_option_empty2 => 6,
			options_e::_soundvol => 7,
			options_e::opt_end => 8,
		}
	}
}

impl From<options_e> for usize {
	fn from(value: options_e) -> Self {
		value.to_u8().into()
	}
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
	numitems: options_e::opt_end.to_i16(),
	prevMenu: &raw mut MainDef,
	menuitems: unsafe { OptionsMenu.as_mut_ptr() },
	routine: M_DrawOptions,
	x: 60,
	y: 37,
	lastOn: 0,
};

// Read This! MENU 1 & 2
#[derive(Clone, Copy)]
enum read_e {
	_rdthsempty1,
	read1_end,
}

impl read_e {
	const fn to_i16(self) -> i16 {
		match self {
			read_e::_rdthsempty1 => 0,
			read_e::read1_end => 1,
		}
	}
}

static mut ReadMenu1: [menuitem_t; 1] =
	[menuitem_t { status: 1, name: [0; 10], routine: Some(M_ReadThis2), alphaKey: 0 }];

#[allow(static_mut_refs)]
static mut ReadDef1: menu_t = menu_t {
	numitems: read_e::read1_end.to_i16(),
	prevMenu: &raw mut MainDef,
	menuitems: unsafe { ReadMenu1.as_mut_ptr() },
	routine: M_DrawReadThis1,
	x: 280,
	y: 185,
	lastOn: 0,
};

#[derive(Clone, Copy)]
enum read_e2 {
	_rdthsempty2,
	read2_end,
}

impl read_e2 {
	const fn to_i16(self) -> i16 {
		match self {
			read_e2::_rdthsempty2 => 0,
			read_e2::read2_end => 1,
		}
	}
}

static mut ReadMenu2: [menuitem_t; 1] =
	[menuitem_t { status: 1, name: [0; 10], routine: Some(M_FinishReadThis), alphaKey: 0 }];

#[allow(static_mut_refs)]
static mut ReadDef2: menu_t = menu_t {
	numitems: read_e2::read2_end.to_i16(),
	prevMenu: &raw mut ReadDef1,
	menuitems: unsafe { ReadMenu2.as_mut_ptr() },
	routine: M_DrawReadThis2,
	x: 330,
	y: 175,
	lastOn: 0,
};

// SOUND VOLUME MENU
#[derive(Clone, Copy)]
enum sound_e {
	sfx_vol,
	_sfx_empty1,
	music_vol,
	_sfx_empty2,
	sound_end,
}

impl sound_e {
	const fn to_i16(self) -> i16 {
		match self {
			sound_e::sfx_vol => 0,
			sound_e::_sfx_empty1 => 1,
			sound_e::music_vol => 2,
			sound_e::_sfx_empty2 => 3,
			sound_e::sound_end => 4,
		}
	}

	const fn to_usize(self) -> usize {
		match self {
			sound_e::sfx_vol => 0,
			sound_e::_sfx_empty1 => 1,
			sound_e::music_vol => 2,
			sound_e::_sfx_empty2 => 3,
			sound_e::sound_end => 4,
		}
	}
}

impl From<sound_e> for i16 {
	fn from(value: sound_e) -> Self {
		value.to_i16()
	}
}

impl From<sound_e> for usize {
	fn from(value: sound_e) -> Self {
		value.to_usize()
	}
}

static mut SoundMenu: [menuitem_t; 4] = [
	menuitem_t { status: 2, name: *b"M_SFXVOL\0\0", routine: Some(M_SfxVol), alphaKey: b's' },
	menuitem_t { status: -1, name: [0; 10], routine: None, alphaKey: 0 },
	menuitem_t { status: 2, name: *b"M_MUSVOL\0\0", routine: Some(M_MusicVol), alphaKey: b'm' },
	menuitem_t { status: -1, name: [0; 10], routine: None, alphaKey: 0 },
];

#[allow(static_mut_refs)]
static mut SoundDef: menu_t = menu_t {
	numitems: sound_e::sound_end.to_i16(),
	prevMenu: &raw mut OptionsDef,
	menuitems: unsafe { SoundMenu.as_mut_ptr() },
	routine: M_DrawSound,
	x: 80,
	y: 64,
	lastOn: 0,
};

// LOAD GAME MENU
#[derive(Clone, Copy)]
enum load_e {
	_load1,
	_load2,
	_load3,
	_load4,
	_load5,
	_load6,
	load_end,
}

impl load_e {
	const fn to_u8(self) -> u8 {
		match self {
			load_e::_load1 => 0,
			load_e::_load2 => 1,
			load_e::_load3 => 2,
			load_e::_load4 => 3,
			load_e::_load5 => 4,
			load_e::_load6 => 5,
			load_e::load_end => 6,
		}
	}

	const fn to_i16(self) -> i16 {
		match self {
			load_e::_load1 => 0,
			load_e::_load2 => 1,
			load_e::_load3 => 2,
			load_e::_load4 => 3,
			load_e::_load5 => 4,
			load_e::_load6 => 5,
			load_e::load_end => 6,
		}
	}
}

impl From<load_e> for usize {
	fn from(value: load_e) -> Self {
		value.to_u8().into()
	}
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
	numitems: load_e::load_end.to_i16(),
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
	numitems: load_e::load_end.to_i16(),
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

		for i in 0..usize::from(load_e::load_end) {
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
		for i in 0..usize::from(load_e::load_end) {
			M_DrawSaveLoadBorder(
				usize::try_from(LoadDef.x).unwrap(),
				usize::try_from(LoadDef.y).unwrap() + LINEHEIGHT * i,
			);
			M_WriteText(
				usize::try_from(LoadDef.x).unwrap(),
				usize::try_from(LoadDef.y).unwrap() + LINEHEIGHT * i,
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
			M_StartMessage(LOADNET, None, false);
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
		for i in 0..usize::from(load_e::load_end) {
			M_DrawSaveLoadBorder(
				usize::try_from(LoadDef.x).unwrap(),
				usize::try_from(LoadDef.y).unwrap() + LINEHEIGHT * i,
			);
			M_WriteText(
				usize::try_from(LoadDef.x).unwrap(),
				usize::try_from(LoadDef.y).unwrap() + LINEHEIGHT * i,
				savegamestrings[i].as_ptr(),
			);
		}

		if saveStringEnter != 0 {
			let i = M_StringWidth(savegamestrings[saveSlot].as_ptr());
			M_WriteText(
				usize::try_from(LoadDef.x).unwrap() + i,
				usize::try_from(LoadDef.y).unwrap() + LINEHEIGHT * saveSlot,
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
			quickSaveSlot = i32::try_from(slot).unwrap();
		}
	}
}

// User wants to save. Start string input for M_Responder
#[allow(static_mut_refs)]
fn M_SaveSelect(choice: i32) {
	unsafe {
		// we are going to be intercepting all chars
		saveStringEnter = 1;

		let choice = usize::try_from(choice).unwrap();
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
			M_StartMessage(SAVEDEAD, None, false);
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

fn M_QuickSaveResponse(ch: i32) {
	if ch == i32::from(b'y') {
		unsafe { M_DoSave(usize::try_from(quickSaveSlot).unwrap()) };
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
		libc::sprintf(
			tempstring.as_mut_ptr(),
			QSPROMPT,
			savegamestrings[usize::try_from(quickSaveSlot).unwrap()],
		);
		M_StartMessage(tempstring.as_ptr(), Some(M_QuickSaveResponse), true);
	}
}

// M_QuickLoad
fn M_QuickLoadResponse(ch: i32) {
	if ch == i32::from(b'y') {
		unsafe { M_LoadSelect(quickSaveSlot) };
		S_StartSound(null_mut(), sfxenum_t::sfx_swtchx);
	}
}

#[allow(static_mut_refs)]
fn M_QuickLoad() {
	unsafe {
		if netgame != 0 {
			M_StartMessage(QLOADNET, None, false);
			return;
		}

		if quickSaveSlot < 0 {
			M_StartMessage(QSAVESPOT, None, false);
			return;
		}

		libc::sprintf(
			tempstring.as_mut_ptr(),
			QLPROMPT,
			savegamestrings[usize::try_from(quickSaveSlot).unwrap()],
		);
		M_StartMessage(tempstring.as_ptr(), Some(M_QuickLoadResponse), true);
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
			usize::try_from(SoundDef.x).unwrap(),
			usize::try_from(SoundDef.y).unwrap() + LINEHEIGHT * (usize::from(sound_e::sfx_vol) + 1),
			16,
			usize::try_from(snd_SfxVolume).unwrap(),
		);

		M_DrawThermo(
			usize::try_from(SoundDef.x).unwrap(),
			usize::try_from(SoundDef.y).unwrap()
				+ LINEHEIGHT * (usize::from(sound_e::music_vol) + 1),
			16,
			usize::try_from(snd_MusicVolume).unwrap(),
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
			1 if snd_SfxVolume < 15 => snd_SfxVolume += 1,
			_ => (),
		}

		S_SetSfxVolume(snd_SfxVolume /* *8 */);
	}
}

fn M_MusicVol(choice: i32) {
	unsafe {
		match choice {
			0 => snd_MusicVolume = snd_MusicVolume.saturating_sub(1),
			1 if snd_MusicVolume < 15 => snd_MusicVolume += 1,
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
			M_StartMessage(NEWGAME, None, false);
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

fn M_VerifyNightmare(ch: i32) {
	if ch != i32::from(b'y') {
		return;
	}

	unsafe { G_DeferedInitNew(skill_t::sk_nightmare, epi + 1, 1) };
	M_ClearMenus();
}

fn M_ChooseSkill(choice: i32) {
	unsafe {
		if choice == i32::from(newgame_e::nightmare.to_i16()) {
			M_StartMessage(NIGHTMARE, Some(M_VerifyNightmare), true);
			return;
		}

		G_DeferedInitNew(skill_t::from(u8::try_from(choice).unwrap()), epi + 1, 1);
		M_ClearMenus();
	}
}

fn M_Episode(mut choice: i32) {
	unsafe {
		if gamemode == GameMode_t::shareware && choice != 0 {
			M_StartMessage(SWSTRING, None, false);
			M_SetupNextMenu(&raw mut ReadDef1);
			return;
		}

		// Yet another hack...
		if gamemode == GameMode_t::registered && choice > 2 {
			eprintln!("M_Episode: 4th episode requires UltimateDOOM");
			choice = 0;
		}

		epi = usize::try_from(choice).unwrap();
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
			usize::try_from(OptionsDef.x).unwrap() + 175,
			usize::try_from(OptionsDef.y).unwrap() + LINEHEIGHT * usize::from(options_e::detail),
			0,
			W_CacheLumpName(
				detailNames[usize::try_from(detailLevel).unwrap()].as_ptr().cast(),
				PU_CACHE,
			)
			.cast(),
		);

		V_DrawPatchDirect(
			usize::try_from(OptionsDef.x).unwrap() + 120,
			usize::try_from(OptionsDef.y).unwrap() + LINEHEIGHT * usize::from(options_e::messages),
			0,
			W_CacheLumpName(
				msgNames[usize::try_from(showMessages).unwrap()].as_ptr().cast(),
				PU_CACHE,
			)
			.cast(),
		);

		M_DrawThermo(
			usize::try_from(OptionsDef.x).unwrap(),
			usize::try_from(OptionsDef.y).unwrap()
				+ LINEHEIGHT * (usize::from(options_e::mousesens) + 1),
			10,
			usize::try_from(mouseSensitivity).unwrap(),
		);

		M_DrawThermo(
			usize::try_from(OptionsDef.x).unwrap(),
			usize::try_from(OptionsDef.y).unwrap()
				+ LINEHEIGHT * (usize::from(options_e::scrnsize) + 1),
			9,
			screenSize,
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
fn M_EndGameResponse(ch: i32) {
	if ch != i32::from(b'y') {
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
			M_StartMessage(NETEND, None, false);
			return;
		}

		M_StartMessage(ENDGAME, Some(M_EndGameResponse), true);
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

fn M_QuitResponse(ch: i32) {
	unsafe {
		if ch != i32::from(b'y') {
			return;
		}
		if netgame == 0 {
			if gamemode == GameMode_t::commercial {
				S_StartSound(null_mut(), quitsounds2[(usize::try_from(gametic).unwrap() >> 2) & 7]);
			} else {
				S_StartSound(null_mut(), quitsounds[(usize::try_from(gametic).unwrap() >> 2) & 7]);
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
				endmsg[(usize::try_from(gametic).unwrap() % (NUM_QUITMESSAGES - 2)) + 1],
				DOSY!(),
			);
		}

		M_StartMessage(endstring.as_ptr(), Some(M_QuitResponse), true);
	}
}

fn M_ChangeSensitivity(choice: i32) {
	unsafe {
		match choice {
			0 if mouseSensitivity != 0 => mouseSensitivity -= 1,
			1 if mouseSensitivity < 9 => mouseSensitivity += 1,
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

fn M_SizeDisplay(choice: i32) {
	unsafe {
		match choice {
			0 if screenSize > 0 => {
				screenblocks -= 1;
				screenSize -= 1;
			}
			1 if screenSize < 8 => {
				screenblocks += 1;
				screenSize += 1;
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

fn M_StartMessage(string: *const c_char, routine: Option<fn(i32)>, input: bool) {
	unsafe {
		messageLastMenuActive = menuactive;
		messageToPrint = 1;
		messageString = string;
		messageRoutine = routine;
		messageNeedsInput = input;
		menuactive = true;
	}
}

// Find string width from hu_font chars
fn M_StringWidth(string: *const c_char) -> usize {
	unsafe {
		let mut w = 0;

		for i in 0..libc::strlen(string) {
			let c = libc::toupper(i32::from(*string.wrapping_add(i))) - i32::from(HU_FONTSTART);
			if c < 0 || c >= i32::from(HU_FONTSIZE) {
				w += 4;
			} else {
				w += usize::try_from((*hu_font[usize::try_from(c).unwrap()]).width).unwrap();
			}
		}

		w
	}
}

//      Find string height from hu_font chars
fn M_StringHeight(string: *const c_char) -> usize {
	unsafe {
		let height = usize::try_from((*hu_font[0]).height).unwrap();

		let mut h = height;
		for i in 0..libc::strlen(string) {
			if *string.wrapping_add(i) == c_char::try_from(b'\n').unwrap() {
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
			if c == c_char::try_from(b'\n').unwrap() {
				cx = x;
				cy += 12;
				continue;
			}

			let c = libc::toupper(i32::from(c)) - i32::from(HU_FONTSTART);
			if c < 0 || c >= i32::from(HU_FONTSIZE) {
				cx += 4;
				continue;
			}

			let w = usize::try_from((*hu_font[usize::try_from(c).unwrap()]).width).unwrap();
			if cx + w > SCREENWIDTH {
				break;
			}
			V_DrawPatchDirect(cx, cy, 0, hu_font[usize::try_from(c).unwrap()]);
			cx += w;
		}
	}
}

// CONTROL PANEL

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
				ch = i32::from(KEY_UPARROW);
				joywait = I_GetTime() + 5;
			} else if ev.data3 == 1 {
				ch = i32::from(KEY_DOWNARROW);
				joywait = I_GetTime() + 5;
			}

			if ev.data2 == -1 {
				ch = i32::from(KEY_LEFTARROW);
				joywait = I_GetTime() + 2;
			} else if ev.data2 == 1 {
				ch = i32::from(KEY_RIGHTARROW);
				joywait = I_GetTime() + 2;
			}

			if ev.data1 & 1 != 0 {
				ch = i32::from(KEY_ENTER);
				joywait = I_GetTime() + 5;
			}
			if ev.data1 & 2 != 0 {
				ch = i32::from(KEY_BACKSPACE);
				joywait = I_GetTime() + 5;
			}
		} else if ev.ty == evtype_t::ev_mouse && mousewait < I_GetTime() {
			mousey += ev.data3;
			if mousey < lasty - 30 {
				ch = i32::from(KEY_DOWNARROW);
				mousewait = I_GetTime() + 5;
				lasty -= 30;
				mousey = lasty;
			} else if mousey > lasty + 30 {
				ch = i32::from(KEY_UPARROW);
				mousewait = I_GetTime() + 5;
				lasty += 30;
				mousey = lasty;
			}

			mousex += ev.data2;
			if mousex < lastx - 30 {
				ch = i32::from(KEY_LEFTARROW);
				mousewait = I_GetTime() + 5;
				lastx -= 30;
				mousex = lastx;
			} else if mousex > lastx + 30 {
				ch = i32::from(KEY_RIGHTARROW);
				mousewait = I_GetTime() + 5;
				lastx += 30;
				mousex = lastx;
			}

			if ev.data1 & 1 != 0 {
				ch = i32::from(KEY_ENTER);
				mousewait = I_GetTime() + 15;
			}

			if ev.data1 & 2 != 0 {
				ch = i32::from(KEY_BACKSPACE);
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
				_ if ch == i32::from(KEY_BACKSPACE) => {
					if saveCharIndex > 0 {
						saveCharIndex -= 1;
						savegamestrings[saveSlot][saveCharIndex] = 0;
					}
				}

				_ if ch == i32::from(KEY_ESCAPE) => {
					saveStringEnter = 0;
					libc::strcpy(savegamestrings[saveSlot].as_mut_ptr(), saveOldString.as_ptr());
				}

				_ if ch == i32::from(KEY_ENTER) => {
					saveStringEnter = 0;
					if savegamestrings[saveSlot][0] != 0 {
						M_DoSave(saveSlot);
					}
				}

				_ => {
					ch = libc::toupper(ch);
					if ch >= i32::from(HU_FONTSTART)
						&& ch < i32::from(HU_FONTSTART + HU_FONTSIZE)
						&& (32..=127).contains(&ch)
						&& saveCharIndex < SAVESTRINGSIZE - 1
						&& M_StringWidth(savegamestrings[saveSlot].as_ptr())
							< (SAVESTRINGSIZE - 2) * 8
					{
						savegamestrings[saveSlot][saveCharIndex] = c_char::try_from(ch).unwrap();
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
				&& !(ch == i32::from(b' ')
					|| ch == i32::from(b'n')
					|| ch == i32::from(b'y')
					|| ch == i32::from(KEY_ESCAPE))
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

		if devparm != 0 && ch == i32::from(KEY_F1) {
			G_ScreenShot();
			return true;
		}

		// F-Keys
		if !menuactive {
			match ch {
				_ if ch == i32::from(KEY_MINUS) => {
					// Screen size down
					if automapactive || chat_on != 0 {
						return false;
					}
					M_SizeDisplay(0);
					S_StartSound(null_mut(), sfxenum_t::sfx_stnmov);
					return true;
				}

				_ if ch == i32::from(KEY_EQUALS) => {
					// Screen size up
					if automapactive || chat_on != 0 {
						return false;
					}
					M_SizeDisplay(1);
					S_StartSound(null_mut(), sfxenum_t::sfx_stnmov);
					return true;
				}

				_ if ch == i32::from(KEY_F1) => {
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

				_ if ch == i32::from(KEY_F2) => {
					// Save
					M_StartControlPanel();
					S_StartSound(null_mut(), sfxenum_t::sfx_swtchn);
					M_SaveGame(0);
					return true;
				}

				_ if ch == i32::from(KEY_F3) => {
					// Load
					M_StartControlPanel();
					S_StartSound(null_mut(), sfxenum_t::sfx_swtchn);
					M_LoadGame(0);
					return true;
				}

				_ if ch == i32::from(KEY_F4) => {
					// Sound Volume
					M_StartControlPanel();
					currentMenu = &raw mut SoundDef;
					itemOn = short::from(sound_e::sfx_vol);
					S_StartSound(null_mut(), sfxenum_t::sfx_swtchn);
					return true;
				}

				_ if ch == i32::from(KEY_F5) => {
					// Detail toggle
					M_ChangeDetail(0);
					S_StartSound(null_mut(), sfxenum_t::sfx_swtchn);
					return true;
				}

				_ if ch == i32::from(KEY_F6) => {
					// Quicksave
					S_StartSound(null_mut(), sfxenum_t::sfx_swtchn);
					M_QuickSave();
					return true;
				}

				_ if ch == i32::from(KEY_F7) => {
					// End game
					S_StartSound(null_mut(), sfxenum_t::sfx_swtchn);
					M_EndGame(0);
					return true;
				}

				_ if ch == i32::from(KEY_F8) => {
					// Toggle messages
					M_ChangeMessages(0);
					S_StartSound(null_mut(), sfxenum_t::sfx_swtchn);
					return true;
				}

				_ if ch == i32::from(KEY_F9) => {
					// Quickload
					S_StartSound(null_mut(), sfxenum_t::sfx_swtchn);
					M_QuickLoad();
					return true;
				}

				_ if ch == i32::from(KEY_F10) => {
					// Quit DOOM
					S_StartSound(null_mut(), sfxenum_t::sfx_swtchn);
					M_QuitDOOM(0);
					return true;
				}

				_ if ch == i32::from(KEY_F11) => {
					// gamma toggle
					usegamma += 1;
					if usegamma > 4 {
						usegamma = 0;
					}
					players[consoleplayer].message = gammamsg[usegamma].as_ptr().cast();
					I_SetPalette(W_CacheLumpName(c"PLAYPAL".as_ptr(), PU_CACHE).cast());
					return true;
				}

				_ => (),
			}
		}

		// Pop-up menu?
		if !menuactive {
			if ch == i32::from(KEY_ESCAPE) {
				M_StartControlPanel();
				S_StartSound(null_mut(), sfxenum_t::sfx_swtchn);
				return true;
			}
			return false;
		}

		// Keys usable within menu
		match ch {
			_ if ch == i32::from(KEY_DOWNARROW) => {
				loop {
					if itemOn + 1 > (*currentMenu).numitems - 1 {
						itemOn = 0;
					} else {
						itemOn += 1;
					}
					S_StartSound(null_mut(), sfxenum_t::sfx_pstop);
					if (*(*currentMenu).menuitems.wrapping_add(usize::try_from(itemOn).unwrap()))
						.status != -1
					{
						break;
					}
				}
				return true;
			}

			_ if ch == i32::from(KEY_UPARROW) => {
				loop {
					if itemOn == 0 {
						itemOn = (*currentMenu).numitems - 1;
					} else {
						itemOn -= 1;
					}
					S_StartSound(null_mut(), sfxenum_t::sfx_pstop);
					if (*(*currentMenu).menuitems.wrapping_add(usize::try_from(itemOn).unwrap()))
						.status != -1
					{
						break;
					}
				}
				return true;
			}

			_ if ch == i32::from(KEY_LEFTARROW) => {
				if let Some(routine) =
					(*(*currentMenu).menuitems.wrapping_add(usize::try_from(itemOn).unwrap()))
						.routine
				{
					if (*(*currentMenu).menuitems.wrapping_add(usize::try_from(itemOn).unwrap()))
						.status == 2
					{
						S_StartSound(null_mut(), sfxenum_t::sfx_stnmov);
						routine(0);
					}
				}
				return true;
			}

			_ if ch == i32::from(KEY_RIGHTARROW) => {
				if let Some(routine) =
					(*(*currentMenu).menuitems.wrapping_add(usize::try_from(itemOn).unwrap()))
						.routine
				{
					if (*(*currentMenu).menuitems.wrapping_add(usize::try_from(itemOn).unwrap()))
						.status == 2
					{
						S_StartSound(null_mut(), sfxenum_t::sfx_stnmov);
						routine(1);
					}
				}
				return true;
			}

			_ if ch == i32::from(KEY_ENTER) => {
				if let Some(routine) =
					(*(*currentMenu).menuitems.wrapping_add(usize::try_from(itemOn).unwrap()))
						.routine
				{
					if let status @ 1.. =
						(*(*currentMenu).menuitems.wrapping_add(usize::try_from(itemOn).unwrap()))
							.status
					{
						(*currentMenu).lastOn = itemOn;
						if status == 2 {
							routine(1); // right arrow
							S_StartSound(null_mut(), sfxenum_t::sfx_stnmov);
						} else {
							routine(i32::from(itemOn));
							S_StartSound(null_mut(), sfxenum_t::sfx_pistol);
						}
					}
				}
				return true;
			}

			_ if ch == i32::from(KEY_ESCAPE) => {
				(*currentMenu).lastOn = itemOn;
				M_ClearMenus();
				S_StartSound(null_mut(), sfxenum_t::sfx_swtchx);
				return true;
			}

			_ if ch == i32::from(KEY_BACKSPACE) => {
				(*currentMenu).lastOn = itemOn;
				if !(*currentMenu).prevMenu.is_null() {
					currentMenu = (*currentMenu).prevMenu;
					itemOn = (*currentMenu).lastOn;
					S_StartSound(null_mut(), sfxenum_t::sfx_swtchn);
				}
				return true;
			}

			_ => {
				for i in usize::try_from(itemOn).unwrap() + 1
					..usize::try_from((*currentMenu).numitems).unwrap()
				{
					if i32::from((*(*currentMenu).menuitems.wrapping_add(i)).alphaKey) == ch {
						itemOn = short::try_from(i).unwrap();
						S_StartSound(null_mut(), sfxenum_t::sfx_pstop);
						return true;
					}
				}
				for i in 0..usize::try_from(itemOn).unwrap() {
					if i32::from((*(*currentMenu).menuitems.wrapping_add(i)).alphaKey) == ch {
						itemOn = short::try_from(i).unwrap();
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
			y = 100 - i16::try_from(M_StringHeight(messageString) / 2).unwrap();
			while *messageString.wrapping_add(start) != 0 {
				let mut i = 0;
				for _ in 0..libc::strlen(messageString.wrapping_add(start)) {
					if *(messageString.wrapping_add(start + i)) == c_char::try_from(b'\n').unwrap()
					{
						string = [0; 40];
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

				x = 160 - i16::try_from(M_StringWidth(string.as_ptr()) / 2).unwrap();
				M_WriteText(
					usize::try_from(i16::max(x, 0)).unwrap(),
					usize::try_from(i16::max(y, 0)).unwrap(),
					string.as_ptr(),
				);
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

		for i in 0..usize::try_from(max).unwrap() {
			if (*(*currentMenu).menuitems.wrapping_add(i)).name[0] != 0 {
				V_DrawPatchDirect(
					usize::try_from(x).unwrap(),
					usize::try_from(y).unwrap(),
					0,
					W_CacheLumpName(
						(*(*currentMenu).menuitems.wrapping_add(i)).name.as_ptr().cast(),
						PU_CACHE,
					)
					.cast(),
				);
			}
			y += i16::try_from(LINEHEIGHT).unwrap();
		}

		// DRAW SKULL
		V_DrawPatchDirect(
			usize::try_from(x).unwrap().wrapping_add_signed(SKULLXOFF),
			usize::try_from((*currentMenu).y).unwrap() - 5
				+ usize::try_from(itemOn).unwrap() * LINEHEIGHT,
			0,
			W_CacheLumpName(
				skullName[usize::try_from(whichSkull).unwrap()].as_ptr().cast(),
				PU_CACHE,
			)
			.cast(),
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
pub fn M_Ticker() {
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
			MainMenu[usize::from(main_e::readthis)] = MainMenu[usize::from(main_e::quitdoom)];
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
