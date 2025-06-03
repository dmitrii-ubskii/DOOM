#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

// DESCRIPTION:
//	Mission start screen wipe/melt, special effects.

use std::{ffi::c_void, ptr::null_mut, slice};

use libc::memcpy;

use crate::{m_random::M_Random, z_zone::PU_STATIC};

// simple gradual pixel change for 8-bit only
pub const wipe_ColorXForm: usize = 0;

// weird screen melt
pub const wipe_Melt: usize = 1;

pub const wipe_NUMWIPES: usize = 2;

// when zero, stop the wipe
static mut go: bool = false;

static mut wipe_scr_start: *mut u8 = null_mut();
static mut wipe_scr_end: *mut u8 = null_mut();
static mut wipe_scr: *mut u8 = null_mut();

unsafe extern "C" {
	fn Z_Malloc(size: usize, tag: usize, ptr: *mut c_void) -> *mut c_void;
	fn Z_Free(ptr: *mut c_void);
}

fn wipe_shittyColMajorXform(array: *mut i16, width: usize, height: usize) {
	unsafe {
		let dest = Z_Malloc(width * height * 2, PU_STATIC, null_mut()) as *mut i16;

		for y_ in 0..height {
			for x in 0..width {
				*dest.wrapping_add(x * height + y_) = *array.wrapping_add(y_ * width + x);
			}
		}

		memcpy(array as *mut c_void, dest as *mut c_void, width * height * 2);

		Z_Free(dest as *mut c_void);
	}
}

fn wipe_initColorXForm(width: usize, height: usize, _ticks: usize) -> i32 {
	unsafe { memcpy(wipe_scr as *mut c_void, wipe_scr_start as *mut c_void, width * height) };
	0
}

fn wipe_doColorXForm(width: usize, height: usize, ticks: usize) -> i32 {
	unsafe {
		let mut changed = false;
		let mut w = wipe_scr;
		let mut e = wipe_scr_end;

		while w != wipe_scr.wrapping_add(width * height) {
			if *w != *e {
				if *w > *e {
					let newval = *w - ticks as u8;
					if newval < *e {
						*w = *e;
					} else {
						*w = newval;
					}
					changed = true;
				} else if *w < *e {
					let newval = *w + ticks as u8;
					if newval > *e {
						*w = *e;
					} else {
						*w = newval;
					}
					changed = true;
				}
			}
			w = w.wrapping_add(1);
			e = e.wrapping_add(1);
		}

		(!changed) as i32
	}
}

fn wipe_exitColorXForm(_width: usize, _height: usize, _ticks: usize) -> i32 {
	0
}

static mut y: *mut i32 = null_mut();

fn wipe_initMelt(width: usize, height: usize, _ticks: usize) -> i32 {
	unsafe {
		// copy start screen to main screen
		memcpy(wipe_scr as *mut c_void, wipe_scr_start as *mut c_void, width * height);

		// makes this wipe faster (in theory)
		// to have stuff in column-major format
		wipe_shittyColMajorXform(wipe_scr_start as *mut i16, width / 2, height);
		wipe_shittyColMajorXform(wipe_scr_end as *mut i16, width / 2, height);

		// setup initial column positions
		// (y<0 => not ready to scroll yet)
		y = Z_Malloc(width * size_of::<i32>(), PU_STATIC, null_mut()) as *mut i32;

		let y_slice = slice::from_raw_parts_mut(y, width);
		y_slice[0] = -(M_Random() % 16);
		for i in 1..width {
			let r = (M_Random() % 3) - 1;
			y_slice[i] = y_slice[i - 1] + r;
			if y_slice[i] > 0 {
				y_slice[i] = 0;
			} else if y_slice[i] == -16 {
				y_slice[i] = -15;
			}
		}

		0
	}
}

fn wipe_doMelt(mut width: usize, height: usize, ticks: usize) -> i32 {
	unsafe {
		let mut done = true;

		width /= 2;
		let y_slice = slice::from_raw_parts_mut(y, width);

		for _ in 0..ticks {
			for (i, item) in y_slice.iter_mut().enumerate() {
				if *item < 0 {
					*item += 1;
					done = false;
				} else if *item < height as i32 {
					let mut dy = if *item < 16 { *item + 1 } else { 8 };
					if *item + dy >= height as i32 {
						dy = height as i32 - *item;
					}
					let mut s =
						(wipe_scr_end as *mut i16).wrapping_add(i * height + *item as usize);
					let d = (wipe_scr as *mut i16).wrapping_add(*item as usize * width + i);
					let mut idx = 0;
					for _ in (1..=dy).rev() {
						*d.wrapping_add(idx) = *s;
						s = s.wrapping_add(1);
						idx += width;
					}
					*item += dy;
					let mut s = (wipe_scr_start as *mut i16).wrapping_add(i * height);
					let d = (wipe_scr as *mut i16).wrapping_add(*item as usize * width + i);
					let mut idx = 0;
					for _ in 0..height - *item as usize {
						*d.wrapping_add(idx) = *s;
						s = s.wrapping_add(1);
						idx += width;
					}
					done = false;
				}
			}
		}

		done as i32
	}
}

fn wipe_exitMelt(_width: usize, _height: usize, _ticks: usize) -> i32 {
	unsafe { Z_Free(y as *mut c_void) };
	0
}

unsafe extern "C" {
	static mut screens: [*mut u8; 5];
	fn I_ReadScreen(scr: *mut u8);
	fn V_DrawBlock(x: i32, y_: i32, scrn: i32, width: usize, height: usize, src: *mut u8);
}

pub(crate) fn wipe_StartScreen(_x: i32, _y: i32, _width: usize, _height: usize) -> i32 {
	unsafe {
		wipe_scr_start = screens[2];
		I_ReadScreen(wipe_scr_start);
	}
	0
}

pub(crate) fn wipe_EndScreen(x: i32, y_: i32, width: usize, height: usize) -> i32 {
	unsafe {
		wipe_scr_end = screens[3];
		I_ReadScreen(wipe_scr_end);
		V_DrawBlock(x, y_, 0, width, height, wipe_scr_start); // restore start scr.
	}
	0
}

pub(crate) fn wipe_ScreenWipe(
	wipeno: usize,
	_x: i32,
	_y: i32,
	width: usize,
	height: usize,
	ticks: usize,
) -> i32 {
	unsafe {
		static mut wipes: [fn(usize, usize, usize) -> i32; 6] = [
			wipe_initColorXForm,
			wipe_doColorXForm,
			wipe_exitColorXForm,
			wipe_initMelt,
			wipe_doMelt,
			wipe_exitMelt,
		];

		unsafe extern "C" {
			fn V_MarkRect(_: i32, _: i32, _: usize, _: usize);
		}

		// initial stuff
		if !go {
			go = true;
			// wipe_scr = (byte *) Z_Malloc(width*height, PU_STATIC, 0); // DEBUG
			wipe_scr = screens[0];
			(wipes[wipeno * 3])(width, height, ticks);
		}

		// do a piece of wipe-in
		V_MarkRect(0, 0, width, height);
		let rc = wipes[wipeno * 3 + 1](width, height, ticks);
		//  V_DrawBlock(x, y, 0, width, height, wipe_scr); // DEBUG

		// final stuff
		if rc != 0 {
			go = false;
			wipes[wipeno * 3 + 2](width, height, ticks);
		}

		(!go) as i32
	}
}
