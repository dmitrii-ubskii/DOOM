#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::ffi::c_char;

use crate::{
	am_map::automapactive,
	doomdef::{KEY_BACKSPACE, KEY_ENTER, SCREENWIDTH},
	r_defs::patch_t,
	v_video::V_DrawPatchDirect,
};

type boolean = i32;

// background and foreground screen numbers
// different from other modules.
// const BG: usize = 1;
const FG: usize = 0;

// font stuff
const HU_MAXLINES: usize = 4;
pub(crate) const HU_MAXLINELENGTH: usize = 80;

// Typedefs of widgets

// Text Line widget
//  (parent of Scrolling Text and Input Text widgets)
#[derive(Clone, Copy)]
pub struct hu_textline_t {
	// left-justified position of scrolling text window
	pub x: usize,
	pub y: usize,

	pub f: *mut *mut patch_t,              // font
	pub sc: i32,                           // start character
	pub l: [c_char; HU_MAXLINELENGTH + 1], // line of text
	pub len: usize,                        // current line length

	// whether this line needs to be udpated
	pub needsupdate: boolean,
}

// Scrolling Text window widget
//  (child of Text Line widget)
pub struct hu_stext_t {
	pub l: [hu_textline_t; HU_MAXLINES], // text lines to draw
	pub h: usize,                        // height in lines
	pub cl: usize,                       // current line number

	//	pointer to boolean stating whether to update window
	pub on: *mut boolean,
	pub laston: boolean, // last value of *on.
}

// Input Text Line widget
//  (child of Text Line widget)
#[derive(Clone, Copy)]
pub struct hu_itext_t {
	pub l: hu_textline_t, // text line to input on

	// left margin past which I am not to delete characters
	pub lm: usize,

	// pointer to boolean stating whether to update window
	pub on: *mut boolean,
	pub laston: boolean, // last value of *on.
}

fn HUlib_clearTextLine(t: &mut hu_textline_t) {
	t.len = 0;
	t.l[0] = 0;
	t.needsupdate = 1;
}

pub(crate) fn HUlib_initTextLine(
	t: &mut hu_textline_t,
	x: usize,
	y: usize,
	f: *mut *mut patch_t,
	sc: i32,
) {
	t.x = x;
	t.y = y;
	t.f = f;
	t.sc = sc;
	HUlib_clearTextLine(t);
}

pub(crate) fn HUlib_addCharToTextLine(t: &mut hu_textline_t, ch: c_char) -> boolean {
	if t.len == HU_MAXLINELENGTH {
		0
	} else {
		t.l[t.len] = ch;
		t.len += 1;
		t.l[t.len] = 0;
		t.needsupdate = 4;
		1
	}
}

fn HUlib_delCharFromTextLine(t: &mut hu_textline_t) -> boolean {
	if t.len == 0 {
		0
	} else {
		t.len -= 1;
		t.l[t.len] = 0;
		t.needsupdate = 4;
		1
	}
}

pub(crate) fn HUlib_drawTextLine(l: &mut hu_textline_t, drawcursor: boolean) {
	unsafe {
		// draw the new stuff
		let mut x = l.x;
		for i in 0..l.len {
			let c = (l.l[i] as u8 as char).to_ascii_uppercase() as i32;
			if c != b' ' as i32 && c >= l.sc && c <= b'_' as i32 {
				let font = *l.f.wrapping_add((c - l.sc) as usize);
				let w = (*font).width as usize;
				if x + w > SCREENWIDTH {
					break;
				}

				V_DrawPatchDirect(x, l.y, FG, font);
				x += w;
			} else {
				x += 4;
				if x >= SCREENWIDTH {
					break;
				}
			}
		}

		// draw the cursor if requested
		let underscore = '_' as usize - l.sc as usize;
		if drawcursor != 0 && x + (**l.f.wrapping_add(underscore)).width as usize <= SCREENWIDTH {
			V_DrawPatchDirect(x, l.y, FG, *l.f.wrapping_add(underscore));
		}
	}
}

unsafe extern "C" {
	static mut viewwindowx: usize;
	static mut viewwindowy: usize;
	static mut viewwidth: usize;
	static mut viewheight: usize;

	fn R_VideoErase(ofs: usize, count: usize);
}

// sorta called by HU_Erase and just better darn get things straight
pub(crate) fn HUlib_eraseTextLine(l: &mut hu_textline_t) {
	unsafe {
		// Only erases when NOT in automap and the screen is reduced,
		// and the text must either need updating or refreshing
		// (because of a recent change back from the automap)
		if automapactive != 0 && viewwindowx != 0 && l.needsupdate != 0 {
			let lh = (**l.f).height as usize + 1;
			for y in l.y..l.y + lh {
				let yoffset = y * SCREENWIDTH;
				if y < viewwindowy || y >= viewwindowy + viewheight {
					R_VideoErase(yoffset, SCREENWIDTH); // erase entire line
				} else {
					R_VideoErase(yoffset, viewwindowx); // erase left border
					R_VideoErase(yoffset + viewwindowx + viewwidth, viewwindowx);
					// erase right border
				}
			}
		}

		if l.needsupdate != 0 {
			l.needsupdate -= 1;
		}
	}
}

pub(crate) fn HUlib_initSText(
	s: &mut hu_stext_t,
	x: usize,
	y: usize,
	h: usize,
	font: *mut *mut patch_t,
	startchar: i32,
	on: *mut boolean,
) {
	unsafe {
		s.h = h;
		s.on = on;
		s.laston = 1;
		s.cl = 0;
		for i in 0..h {
			HUlib_initTextLine(
				&mut s.l[i],
				x,
				y - i * ((**font).height as usize + 1),
				font,
				startchar,
			);
		}
	}
}

fn HUlib_addLineToSText(s: &mut hu_stext_t) {
	// add a clear line
	s.cl += 1;
	if s.cl == s.h {
		s.cl = 0;
	}
	HUlib_clearTextLine(&mut s.l[s.cl]);

	// everything needs updating
	for i in 0..s.h {
		s.l[i].needsupdate = 4;
	}
}

pub(crate) fn HUlib_addMessageToSText(
	s: &mut hu_stext_t,
	mut prefix: *const c_char,
	mut msg: *const c_char,
) {
	unsafe {
		HUlib_addLineToSText(s);
		if !prefix.is_null() {
			while *prefix != 0 {
				HUlib_addCharToTextLine(&mut s.l[s.cl], *prefix);
				prefix = prefix.wrapping_add(1);
			}
		}
		while *msg != 0 {
			HUlib_addCharToTextLine(&mut s.l[s.cl], *msg);
			msg = msg.wrapping_add(1);
		}
	}
}

pub(crate) fn HUlib_drawSText(s: &mut hu_stext_t) {
	unsafe {
		if *s.on == 0 {
			return; // if not on, don't draw
		}

		// draw everything
		for i in 0..s.h {
			let idx = if s.cl < i {
				s.cl + s.h - i // handle queue of lines
			} else {
				s.cl - i
			};

			// need a decision made here on whether to skip the draw
			HUlib_drawTextLine(&mut s.l[idx], 0); // no cursor, please
		}
	}
}

pub(crate) fn HUlib_eraseSText(s: &mut hu_stext_t) {
	unsafe {
		for i in 0..s.h {
			if s.laston != 0 && *s.on == 0 {
				s.l[i].needsupdate = 4;
			}
			HUlib_eraseTextLine(&mut s.l[i]);
		}
		s.laston = *s.on;
	}
}

pub(crate) fn HUlib_initIText(
	it: &mut hu_itext_t,
	x: usize,
	y: usize,
	font: *mut *mut patch_t,
	startchar: i32,
	on: *mut boolean,
) {
	it.lm = 0; // default left margin is start of text
	it.on = on;
	it.laston = 1;
	HUlib_initTextLine(&mut it.l, x, y, font, startchar);
}

// The following deletion routines adhere to the left margin restriction
fn HUlib_delCharFromIText(it: &mut hu_itext_t) {
	if it.l.len != it.lm {
		HUlib_delCharFromTextLine(&mut it.l);
	}
}

// Resets left margin as well
pub(crate) fn HUlib_resetIText(it: &mut hu_itext_t) {
	it.lm = 0;
	HUlib_clearTextLine(&mut it.l);
}

// wrapper function for handling general keyed input.
// returns true if it ate the key
pub(crate) fn HUlib_keyInIText(it: &mut hu_itext_t, ch: u8) -> boolean {
	if (b' '..=b'_').contains(&ch) {
		HUlib_addCharToTextLine(&mut it.l, ch as c_char);
	} else if ch == KEY_BACKSPACE {
		HUlib_delCharFromIText(it);
	} else if ch != KEY_ENTER {
		return 0; // did not eat key
	}

	1 // ate the key
}

pub(crate) fn HUlib_drawIText(it: &mut hu_itext_t) {
	unsafe {
		if *it.on == 0 {
			return;
		}
		HUlib_drawTextLine(&mut it.l, 1); // draw the line w/ cursor
	}
}

pub(crate) fn HUlib_eraseIText(it: &mut hu_itext_t) {
	unsafe {
		if it.laston != 0 && *it.on == 0 {
			it.l.needsupdate = 4;
		}
		HUlib_eraseTextLine(&mut it.l);
		it.laston = *it.on;
	}
}
