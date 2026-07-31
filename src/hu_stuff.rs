#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::{
	ffi::{c_char, c_int},
	ptr::{null, null_mut},
};

use crate::{
	am_map::automapactive,
	const_conv::usize_from_u8,
	d_englsh::{
		HUSTR_1, HUSTR_2, HUSTR_3, HUSTR_4, HUSTR_5, HUSTR_6, HUSTR_7, HUSTR_8, HUSTR_9, HUSTR_10,
		HUSTR_11, HUSTR_12, HUSTR_13, HUSTR_14, HUSTR_15, HUSTR_16, HUSTR_17, HUSTR_18, HUSTR_19,
		HUSTR_20, HUSTR_21, HUSTR_22, HUSTR_23, HUSTR_24, HUSTR_25, HUSTR_26, HUSTR_27, HUSTR_28,
		HUSTR_29, HUSTR_30, HUSTR_31, HUSTR_32, HUSTR_CHATMACRO0, HUSTR_CHATMACRO1,
		HUSTR_CHATMACRO2, HUSTR_CHATMACRO3, HUSTR_CHATMACRO4, HUSTR_CHATMACRO5, HUSTR_CHATMACRO6,
		HUSTR_CHATMACRO7, HUSTR_CHATMACRO8, HUSTR_CHATMACRO9, HUSTR_E1M1, HUSTR_E1M2, HUSTR_E1M3,
		HUSTR_E1M4, HUSTR_E1M5, HUSTR_E1M6, HUSTR_E1M7, HUSTR_E1M8, HUSTR_E1M9, HUSTR_E2M1,
		HUSTR_E2M2, HUSTR_E2M3, HUSTR_E2M4, HUSTR_E2M5, HUSTR_E2M6, HUSTR_E2M7, HUSTR_E2M8,
		HUSTR_E2M9, HUSTR_E3M1, HUSTR_E3M2, HUSTR_E3M3, HUSTR_E3M4, HUSTR_E3M5, HUSTR_E3M6,
		HUSTR_E3M7, HUSTR_E3M8, HUSTR_E3M9, HUSTR_E4M1, HUSTR_E4M2, HUSTR_E4M3, HUSTR_E4M4,
		HUSTR_E4M5, HUSTR_E4M6, HUSTR_E4M7, HUSTR_E4M8, HUSTR_E4M9, HUSTR_KEYBROWN, HUSTR_KEYGREEN,
		HUSTR_KEYINDIGO, HUSTR_KEYRED, HUSTR_MSGU, HUSTR_PLRBROWN, HUSTR_PLRGREEN, HUSTR_PLRINDIGO,
		HUSTR_PLRRED, HUSTR_TALKTOSELF1, HUSTR_TALKTOSELF2, HUSTR_TALKTOSELF3, HUSTR_TALKTOSELF4,
		HUSTR_TALKTOSELF5,
	},
	d_event::{event_t, evtype_t},
	d_player::player_t,
	doomdef::{
		GameMode_t, KEY_ENTER, KEY_ESCAPE, KEY_LALT, KEY_RALT, KEY_RSHIFT, MAXPLAYERS, TICRATE,
	},
	doomstat::gamemode,
	dstrings::Smuggle,
	g_game::{consoleplayer, gameepisode, gamemap, netgame, playeringame, players},
	hu_lib::{
		HU_MAXLINELENGTH, HUlib_addCharToTextLine, HUlib_addMessageToSText, HUlib_drawIText,
		HUlib_drawSText, HUlib_drawTextLine, HUlib_eraseIText, HUlib_eraseSText,
		HUlib_eraseTextLine, HUlib_initIText, HUlib_initSText, HUlib_initTextLine,
		HUlib_keyInIText, HUlib_resetIText, hu_itext_t, hu_stext_t, hu_textline_t,
	},
	m_menu::showMessages,
	r_defs::patch_t,
	s_sound::S_StartSound,
	sounds::sfxenum_t,
	w_wad::W_CacheLumpName,
	z_zone::PU_STATIC,
};

// Globally visible constants.
pub(crate) const HU_FONTSTART: u8 = b'!'; // the first font characters
pub(crate) const HU_FONTEND: u8 = b'_'; // the last font characters

// Calculate # of glyphs in font.
pub(crate) const HU_FONTSIZE: u8 = HU_FONTEND - HU_FONTSTART + 1;

pub(crate) const HU_BROADCAST: u8 = 5;

pub(crate) const HU_MSGREFRESH: u8 = KEY_ENTER;
pub(crate) const HU_MSGX: usize = 0;
pub(crate) const HU_MSGY: usize = 0;
pub(crate) const HU_MSGHEIGHT: usize = 1; // in lines

pub(crate) const HU_MSGTIMEOUT: usize = 4 * TICRATE;

// Locally used constants, shortcuts.
fn HU_TITLE() -> *const c_char {
	unsafe { mapnames[(gameepisode - 1) * 9 + gamemap - 1] }
}
fn HU_TITLE2() -> *const c_char {
	unsafe { mapnames2[gamemap - 1] }
}
// fn HU_TITLEP() -> *const c_char {
// 	unsafe { mapnamesp[gamemap - 1] }
// }
// fn HU_TITLET() -> *const c_char {
// 	unsafe { mapnamest[gamemap - 1] }
// }
const HU_TITLEX: usize = 0;
fn HU_TITLEY() -> usize {
	unsafe { 167 - usize::try_from((*hu_font[0]).height).unwrap() }
}

const HU_INPUTTOGGLE: u8 = b't';
const HU_INPUTX: usize = HU_MSGX;
fn HU_INPUTY() -> usize {
	unsafe { HU_MSGY + HU_MSGHEIGHT * (usize::try_from((*hu_font[0]).height).unwrap() + 1) }
}

pub(crate) static mut chat_macros: [Smuggle<c_char>; 10] = [
	HUSTR_CHATMACRO0,
	HUSTR_CHATMACRO1,
	HUSTR_CHATMACRO2,
	HUSTR_CHATMACRO3,
	HUSTR_CHATMACRO4,
	HUSTR_CHATMACRO5,
	HUSTR_CHATMACRO6,
	HUSTR_CHATMACRO7,
	HUSTR_CHATMACRO8,
	HUSTR_CHATMACRO9,
];

pub(crate) const player_names: [*const c_char; 4] =
	[HUSTR_PLRGREEN, HUSTR_PLRINDIGO, HUSTR_PLRBROWN, HUSTR_PLRRED];

static mut plr: *mut player_t = null_mut();
#[unsafe(no_mangle)]
pub(crate) static mut hu_font: [*mut patch_t; usize_from_u8(HU_FONTSIZE)] =
	[null_mut(); usize_from_u8(HU_FONTSIZE)];
static mut w_title: hu_textline_t = hu_textline_t {
	x: 0,
	y: 0,
	f: null_mut(),
	sc: 0,
	l: [0; HU_MAXLINELENGTH + 1],
	len: 0,
	needsupdate: 0,
};
pub(crate) static mut chat_on: bool = false;
static mut w_chat: hu_itext_t = hu_itext_t {
	l: hu_textline_t {
		x: 0,
		y: 0,
		f: null_mut(),
		sc: 0,
		l: [0; HU_MAXLINELENGTH + 1],
		len: 0,
		needsupdate: 0,
	},
	lm: 0,
	on: null_mut(),
	laston: false,
};
static mut always_off: bool = false;
static mut chat_dest: [c_char; MAXPLAYERS] = [0; MAXPLAYERS];
static mut w_inputbuffer: [hu_itext_t; MAXPLAYERS] = [hu_itext_t {
	l: hu_textline_t {
		x: 0,
		y: 0,
		f: null_mut(),
		sc: 0,
		l: [0; HU_MAXLINELENGTH + 1],
		len: 0,
		needsupdate: 0,
	},
	lm: 0,
	on: null_mut(),
	laston: false,
}; MAXPLAYERS];

static mut message_on: bool = false;
pub(crate) static mut message_dontfuckwithme: bool = false;
static mut message_nottobefuckedwith: bool = false;

static mut w_message: hu_stext_t = hu_stext_t {
	l: [hu_textline_t {
		x: 0,
		y: 0,
		f: null_mut(),
		sc: 0,
		l: [0; HU_MAXLINELENGTH + 1],
		len: 0,
		needsupdate: 0,
	}; 4],
	h: 0,
	cl: 0,
	on: null_mut(),
	laston: false,
};
static mut message_counter: usize = 0;

static mut headsupactive: bool = false;

// Builtin map names.
// The actual names can be found in DStrings.h.
const mapnames: [*const c_char; 45] = // DOOM shareware/registered/retail (Ultimate) names.
	[
		HUSTR_E1M1,
		HUSTR_E1M2,
		HUSTR_E1M3,
		HUSTR_E1M4,
		HUSTR_E1M5,
		HUSTR_E1M6,
		HUSTR_E1M7,
		HUSTR_E1M8,
		HUSTR_E1M9,
		HUSTR_E2M1,
		HUSTR_E2M2,
		HUSTR_E2M3,
		HUSTR_E2M4,
		HUSTR_E2M5,
		HUSTR_E2M6,
		HUSTR_E2M7,
		HUSTR_E2M8,
		HUSTR_E2M9,
		HUSTR_E3M1,
		HUSTR_E3M2,
		HUSTR_E3M3,
		HUSTR_E3M4,
		HUSTR_E3M5,
		HUSTR_E3M6,
		HUSTR_E3M7,
		HUSTR_E3M8,
		HUSTR_E3M9,
		HUSTR_E4M1,
		HUSTR_E4M2,
		HUSTR_E4M3,
		HUSTR_E4M4,
		HUSTR_E4M5,
		HUSTR_E4M6,
		HUSTR_E4M7,
		HUSTR_E4M8,
		HUSTR_E4M9,
		c"NEWLEVEL".as_ptr(),
		c"NEWLEVEL".as_ptr(),
		c"NEWLEVEL".as_ptr(),
		c"NEWLEVEL".as_ptr(),
		c"NEWLEVEL".as_ptr(),
		c"NEWLEVEL".as_ptr(),
		c"NEWLEVEL".as_ptr(),
		c"NEWLEVEL".as_ptr(),
		c"NEWLEVEL".as_ptr(),
	];

const mapnames2: [*const c_char; 32] = // DOOM 2 map names.
	[
		HUSTR_1, HUSTR_2, HUSTR_3, HUSTR_4, HUSTR_5, HUSTR_6, HUSTR_7, HUSTR_8, HUSTR_9, HUSTR_10,
		HUSTR_11, HUSTR_12, HUSTR_13, HUSTR_14, HUSTR_15, HUSTR_16, HUSTR_17, HUSTR_18, HUSTR_19,
		HUSTR_20, HUSTR_21, HUSTR_22, HUSTR_23, HUSTR_24, HUSTR_25, HUSTR_26, HUSTR_27, HUSTR_28,
		HUSTR_29, HUSTR_30, HUSTR_31, HUSTR_32,
	];

// const mapnamesp: [*const c_char; 32] = // Plutonia WAD map names.
// 	[
// 		PHUSTR_1, PHUSTR_2, PHUSTR_3, PHUSTR_4, PHUSTR_5, PHUSTR_6, PHUSTR_7, PHUSTR_8, PHUSTR_9,
// 		PHUSTR_10, PHUSTR_11, PHUSTR_12, PHUSTR_13, PHUSTR_14, PHUSTR_15, PHUSTR_16, PHUSTR_17,
// 		PHUSTR_18, PHUSTR_19, PHUSTR_20, PHUSTR_21, PHUSTR_22, PHUSTR_23, PHUSTR_24, PHUSTR_25,
// 		PHUSTR_26, PHUSTR_27, PHUSTR_28, PHUSTR_29, PHUSTR_30, PHUSTR_31, PHUSTR_32,
// 	];
//
// const mapnamest: [*const c_char; 32] = // TNT WAD map names.
// 	[
// 		THUSTR_1, THUSTR_2, THUSTR_3, THUSTR_4, THUSTR_5, THUSTR_6, THUSTR_7, THUSTR_8, THUSTR_9,
// 		THUSTR_10, THUSTR_11, THUSTR_12, THUSTR_13, THUSTR_14, THUSTR_15, THUSTR_16, THUSTR_17,
// 		THUSTR_18, THUSTR_19, THUSTR_20, THUSTR_21, THUSTR_22, THUSTR_23, THUSTR_24, THUSTR_25,
// 		THUSTR_26, THUSTR_27, THUSTR_28, THUSTR_29, THUSTR_30, THUSTR_31, THUSTR_32,
// 	];

static mut shiftxform: *const u8 = null();

// #[rustfmt::skip]
// const french_shiftxform:[u8; 128] = [
// 	0,
// 	1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
// 	11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
// 	21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
// 	31,
// 	b' ', b'!', b'"', b'#', b'$', b'%', b'&',
// 	b'"', // shift-'
// 	b'(', b')', b'*', b'+',
// 	b'?', // shift-,
// 	b'_', // shift--
// 	b'>', // shift-.
// 	b'?', // shift-/
// 	b'0', // shift-0
// 	b'1', // shift-1
// 	b'2', // shift-2
// 	b'3', // shift-3
// 	b'4', // shift-4
// 	b'5', // shift-5
// 	b'6', // shift-6
// 	b'7', // shift-7
// 	b'8', // shift-8
// 	b'9', // shift-9
// 	b'/',
// 	b'.', // shift-;
// 	b'<',
// 	b'+', // shift-=
// 	b'>', b'?', b'@',
// 	b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N',
// 	b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z',
// 	b'[', // shift-[
// 	b'!', // shift-backslash - OH MY GOD DOES WATCOM SUCK
// 	b']', // shift-]
// 	b'"', b'_',
//     b'\'', // shift-`
//     b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N',
//     b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z',
//     b'{', b'|', b'}', b'~', 127
// ];

#[rustfmt::skip]
const english_shiftxform:[u8; 128] = [
	0,
	1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
	11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
	21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
	31,
	b' ', b'!', b'"', b'#', b'$', b'%', b'&',
	b'"', // shift-'
	b'(', b')', b'*', b'+',
	b'<', // shift-,
	b'_', // shift--
	b'>', // shift-.
	b'?', // shift-/
	b')', // shift-0
	b'!', // shift-1
	b'@', // shift-2
	b'#', // shift-3
	b'$', // shift-4
	b'%', // shift-5
	b'^', // shift-6
	b'&', // shift-7
	b'*', // shift-8
	b'(', // shift-9
	b':',
	b':', // shift-;
	b'<',
	b'+', // shift-=
	b'>', b'?', b'@',
	b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N',
	b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z',
	b'[', // shift-[
	b'!', // shift-backslash - OH MY GOD DOES WATCOM SUCK
	b']', // shift-]
	b'"', b'_',
	b'\'', // shift-`
	b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N',
	b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z',
	b'{', b'|', b'}', b'~', 127
];

// #[rustfmt::skip]
// const frenchKeyMap:[u8; 128] = [
// 	0,
// 	1,2,3,4,5,6,7,8,9,10,
// 	11,12,13,14,15,16,17,18,19,20,
// 	21,22,23,24,25,26,27,28,29,30,
// 	31,
// 	b' ',b'!',b'"',b'#',b'$',b'%',b'&',b'%',b'(',b')',b'*',b'+',b';',b'-',b':',b'!',
//     b'0',b'1',b'2',b'3',b'4',b'5',b'6',b'7',b'8',b'9',b':',b'M',b'<',b'=',b'>',b'?',
//     b'@',b'Q',b'B',b'C',b'D',b'E',b'F',b'G',b'H',b'I',b'J',b'K',b'L',b',',b'N',b'O',
//     b'P',b'A',b'R',b'S',b'T',b'U',b'V',b'Z',b'X',b'Y',b'W',b'^',b'\\',b'$',b'^',b'_',
//     b'@',b'Q',b'B',b'C',b'D',b'E',b'F',b'G',b'H',b'I',b'J',b'K',b'L',b',',b'N',b'O',
//     b'P',b'A',b'R',b'S',b'T',b'U',b'V',b'Z',b'X',b'Y',b'W',b'^',b'\\',b'$',b'^',127
// ];
//
// fn ForeignTranslation(ch: u8) -> c_char {
// 	(if ch < 128 { frenchKeyMap[ch as usize] } else { ch }) as c_char
// }

pub(crate) fn HU_Init() {
	unsafe {
		let mut buffer = [0; 9];

		// if french != 0 {
		//     shiftxform = french_shiftxform.as_ptr();
		// } else {
		shiftxform = english_shiftxform.as_ptr();
		// }

		// load the heads-up font
		let mut j = HU_FONTSTART;
		#[allow(clippy::needless_range_loop)]
		#[allow(clippy::explicit_counter_loop)]
		for i in 0..usize::from(HU_FONTSIZE) {
			libc::sprintf(buffer.as_mut_ptr(), c"STCFN%.3d".as_ptr(), c_int::from(j));
			j += 1;
			hu_font[i] = W_CacheLumpName(buffer.as_mut_ptr(), PU_STATIC).cast();
		}
	}
}

fn HU_Stop() {
	unsafe {
		headsupactive = false;
	}
}

#[allow(static_mut_refs)]
pub(crate) fn HU_Start() {
	unsafe {
		if headsupactive {
			HU_Stop();
		}

		plr = &raw mut players[consoleplayer];
		message_on = false;
		message_dontfuckwithme = false;
		message_nottobefuckedwith = false;
		chat_on = false;

		// create the message widget
		HUlib_initSText(
			&mut w_message,
			HU_MSGX,
			HU_MSGY,
			HU_MSGHEIGHT,
			hu_font.as_mut_ptr(),
			i32::from(HU_FONTSTART),
			&raw mut message_on,
		);

		// create the map title widget
		HUlib_initTextLine(
			&mut w_title,
			HU_TITLEX,
			HU_TITLEY(),
			hu_font.as_mut_ptr(),
			i32::from(HU_FONTSTART),
		);

		let mut s = match gamemode {
			GameMode_t::shareware | GameMode_t::registered | GameMode_t::retail => HU_TITLE(),

			/* FIXME
			case pack_plut:
			s = HU_TITLEP;
			break;
			case pack_tnt:
			s = HU_TITLET;
			break;
			*/
			/*GameMode_t::commercial |*/
			_ => HU_TITLE2(),
		};

		while *s != 0 {
			HUlib_addCharToTextLine(&mut w_title, *s);
			s = s.wrapping_add(1);
		}

		// create the chat widget
		HUlib_initIText(
			&mut w_chat,
			HU_INPUTX,
			HU_INPUTY(),
			hu_font.as_mut_ptr(),
			i32::from(HU_FONTSTART),
			&raw mut chat_on,
		);

		// create the inputbuffer widgets
		#[allow(clippy::needless_range_loop)]
		for i in 0..MAXPLAYERS {
			HUlib_initIText(&mut w_inputbuffer[i], 0, 0, null_mut(), 0, &mut always_off);
		}

		headsupactive = true;
	}
}

#[allow(static_mut_refs)]
pub(crate) fn HU_Drawer() {
	unsafe {
		HUlib_drawSText(&mut w_message);
		HUlib_drawIText(&mut w_chat);
		if automapactive {
			HUlib_drawTextLine(&mut w_title, false);
		}
	}
}

#[allow(static_mut_refs)]
pub(crate) fn HU_Erase() {
	unsafe {
		HUlib_eraseSText(&mut w_message);
		HUlib_eraseIText(&mut w_chat);
		HUlib_eraseTextLine(&mut w_title);
	}
}

#[allow(static_mut_refs)]
pub(crate) fn HU_Ticker() {
	unsafe {
		// tick down message counter if message is up
		if message_counter != 0 {
			message_counter -= 1;
			if message_counter == 0 {
				message_on = false;
				message_nottobefuckedwith = false;
			}
		}

		if showMessages != 0 || message_dontfuckwithme {
			// display message if necessary
			if !(*plr).message.is_null() && (!message_nottobefuckedwith || message_dontfuckwithme) {
				HUlib_addMessageToSText(&mut w_message, null(), (*plr).message);
				(*plr).message = null();
				message_on = true;
				message_counter = HU_MSGTIMEOUT;
				message_nottobefuckedwith = message_dontfuckwithme;
				message_dontfuckwithme = false;
			}
		} // else message_on = false;

		// check for incoming chat characters
		if netgame {
			for i in 0..MAXPLAYERS {
				if !playeringame[i] {
					continue;
				}
				let mut c = players[i].cmd.chatchar;
				if i != consoleplayer && c != 0 {
					if c <= HU_BROADCAST {
						chat_dest[i] = c_char::try_from(c).unwrap();
					} else {
						if c.is_ascii_lowercase() {
							c = *shiftxform.wrapping_add(usize::from(c));
						}
						let rc = HUlib_keyInIText(&mut w_inputbuffer[i], c);
						if rc && c == KEY_ENTER {
							if w_inputbuffer[i].l.len != 0
								&& (chat_dest[i] == c_char::try_from(consoleplayer).unwrap() + 1
									|| chat_dest[i] == i8::try_from(HU_BROADCAST).unwrap())
							{
								HUlib_addMessageToSText(
									&mut w_message,
									player_names[i],
									w_inputbuffer[i].l.l.as_ptr(),
								);

								message_nottobefuckedwith = true;
								message_on = true;
								message_counter = HU_MSGTIMEOUT;
								if gamemode == GameMode_t::commercial {
									S_StartSound(null_mut(), sfxenum_t::sfx_radio);
								} else {
									S_StartSound(null_mut(), sfxenum_t::sfx_tink);
								}
							}
							HUlib_resetIText(&mut w_inputbuffer[i]);
						}
					}
					players[i].cmd.chatchar = 0;
				}
			}
		}
	}
}

pub(crate) const QUEUESIZE: usize = 128;

static mut chatchars: [c_char; QUEUESIZE] = [0; QUEUESIZE];
static mut head: usize = 0;
static mut tail: usize = 0;

fn HU_queueChatChar(c: c_char) {
	unsafe {
		if ((head + 1) & (QUEUESIZE - 1)) == tail {
			(*plr).message = HUSTR_MSGU;
		} else {
			chatchars[head] = c;
			head = (head + 1) & (QUEUESIZE - 1);
		}
	}
}

pub(crate) fn HU_dequeueChatChar() -> c_char {
	unsafe {
		if head != tail {
			let c = chatchars[tail];
			tail = (tail + 1) & (QUEUESIZE - 1);
			c
		} else {
			0
		}
	}
}

#[allow(static_mut_refs)]
pub(crate) fn HU_Responder(ev: &mut event_t) -> bool {
	unsafe {
		static mut lastmessage: [c_char; HU_MAXLINELENGTH + 1] = [0; HU_MAXLINELENGTH + 1];
		static mut shiftdown: bool = false;
		static mut altdown: bool = false;

		const destination_keys: [c_char; MAXPLAYERS] =
			[HUSTR_KEYGREEN, HUSTR_KEYINDIGO, HUSTR_KEYBROWN, HUSTR_KEYRED];

		static mut num_nobrainers: i32 = 0;

		let mut numplayers = 0;
		#[allow(clippy::needless_range_loop)]
		for i in 0..MAXPLAYERS {
			if playeringame[i] {
				numplayers += 1;
			}
		}

		if ev.data1 == i32::from(KEY_RSHIFT) {
			shiftdown = ev.ty == evtype_t::ev_keydown;
			return false;
		} else if ev.data1 == i32::from(KEY_RALT) || ev.data1 == i32::from(KEY_LALT) {
			altdown = ev.ty == evtype_t::ev_keydown;
			return false;
		}

		if ev.ty != evtype_t::ev_keydown {
			return false;
		}

		let mut eatkey = false;
		if !chat_on {
			if ev.data1 == i32::from(HU_MSGREFRESH) {
				message_on = true;
				message_counter = HU_MSGTIMEOUT;
				eatkey = true;
			} else if netgame && ev.data1 == i32::from(HU_INPUTTOGGLE) {
				eatkey = true;
				chat_on = true;
				HUlib_resetIText(&mut w_chat);
				HU_queueChatChar(c_char::try_from(HU_BROADCAST).unwrap());
			} else if netgame && numplayers > 2 {
				for i in 0..MAXPLAYERS {
					if ev.data1 == i32::from(destination_keys[i]) {
						if playeringame[i] && i != consoleplayer {
							eatkey = true;
							chat_on = true;
							HUlib_resetIText(&mut w_chat);
							HU_queueChatChar(c_char::try_from(i).unwrap() + 1);
							break;
						} else if i == consoleplayer {
							num_nobrainers += 1;
							if num_nobrainers < 3 {
								(*plr).message = HUSTR_TALKTOSELF1;
							} else if num_nobrainers < 6 {
								(*plr).message = HUSTR_TALKTOSELF2;
							} else if num_nobrainers < 9 {
								(*plr).message = HUSTR_TALKTOSELF3;
							} else if num_nobrainers < 32 {
								(*plr).message = HUSTR_TALKTOSELF4;
							} else {
								(*plr).message = HUSTR_TALKTOSELF5;
							}
						}
					}
				}
			}
		} else {
			let mut c = u8::try_from(ev.data1).unwrap();
			// send a macro
			if altdown {
				c -= b'0';
				if c > 9 {
					return false;
				}
				// fprintf(stderr, "got here\n");
				let Smuggle(mut macromessage) = chat_macros[usize::from(c)];

				// kill last message with a '\n'
				HU_queueChatChar(c_char::try_from(KEY_ENTER).unwrap()); // DEBUG!!!

				// send the macro message
				while *macromessage != 0 {
					HU_queueChatChar(*macromessage);
					macromessage = macromessage.wrapping_add(1);
				}
				HU_queueChatChar(c_char::try_from(KEY_ENTER).unwrap());

				// leave chat mode and notify that it was sent
				chat_on = false;
				libc::strcpy(lastmessage.as_mut_ptr(), chat_macros[usize::from(c)].u());
				(*plr).message = lastmessage.as_mut_ptr();
				eatkey = true;
			} else {
				// if french {
				// 	c = ForeignTranslation(c);
				// }
				if shiftdown || c.is_ascii_lowercase() {
					c = *shiftxform.wrapping_add(usize::from(c));
				}
				eatkey = HUlib_keyInIText(&mut w_chat, c);
				if eatkey {
					// static unsigned char buf[20]; // DEBUG
					HU_queueChatChar(c_char::try_from(c).unwrap());

					// sprintf(buf, "KEY: %d => %d", ev.data1, c);
					//      plr.message = buf;
				}
				if c == KEY_ENTER {
					chat_on = false;
					if w_chat.l.len != 0 {
						libc::strcpy(lastmessage.as_mut_ptr(), w_chat.l.l.as_ptr());
						(*plr).message = lastmessage.as_mut_ptr();
					}
				} else if c == KEY_ESCAPE {
					chat_on = false;
				}
			}
		}

		eatkey
	}
}
