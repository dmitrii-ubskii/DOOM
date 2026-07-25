#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]
//	DOOM graphics stuff for X11, UNIX.

use std::{
	mem::{self, MaybeUninit},
	ptr::null_mut,
};

use libc::{
	IPC_CREAT, IPC_RMID, IPC_STAT, SIGINT, getenv, getuid, shmat, shmctl, shmdt, shmget, shmid_ds,
	strcasecmp,
};
use x11::{
	keysym::{
		XK_Alt_L, XK_Alt_R, XK_BackSpace, XK_Control_L, XK_Control_R, XK_Delete, XK_Down,
		XK_Escape, XK_F1, XK_F2, XK_F3, XK_F4, XK_F5, XK_F6, XK_F7, XK_F8, XK_F9, XK_F10, XK_F11,
		XK_F12, XK_KP_Equal, XK_KP_Subtract, XK_Left, XK_Meta_L, XK_Meta_R, XK_Pause, XK_Return,
		XK_Right, XK_Shift_L, XK_Shift_R, XK_Tab, XK_Up, XK_equal, XK_minus,
	},
	xlib::{
		AllocAll, Button1, Button1Mask, Button2, Button2Mask, Button3, Button3Mask, ButtonPress,
		ButtonPressMask, ButtonRelease, ButtonReleaseMask, CWBorderPixel, CWColormap, CWEventMask,
		Colormap, ConfigureNotify, CurrentTime, Cursor, Display, DoBlue, DoGreen, DoRed, Expose,
		ExposureMask, False, GC, GCFunction, GCGraphicsExposures, GXclear, GrabModeAsync,
		InputOutput, KeyPress, KeyPressMask, KeyRelease, KeyReleaseMask, MotionNotify,
		PointerMotionMask, PseudoColor, True, Visual, Window, XColor, XCreateColormap, XCreateGC,
		XCreateImage, XCreatePixmap, XCreatePixmapCursor, XCreateWindow, XDefaultScreen,
		XDefineCursor, XEvent, XFillRectangle, XFreeGC, XFreePixmap, XGCValues, XGrabPointer,
		XImage, XInstallColormap, XKeycodeToKeysym, XMapWindow, XMatchVisualInfo, XNextEvent,
		XOpenDisplay, XPending, XPutImage, XRootWindow, XSetWindowAttributes, XStoreColors, XSync,
		XVisualInfo, XWarpPointer, ZPixmap,
	},
	xshm::{
		XShmAttach, XShmCreateImage, XShmDetach, XShmGetEventBase, XShmPutImage,
		XShmQueryExtension, XShmSegmentInfo,
	},
};

use crate::{
	d_event::{event_t, evtype_t},
	d_main::{D_PostEvent, devparm},
	doomdef::{
		KEY_BACKSPACE, KEY_DOWNARROW, KEY_ENTER, KEY_EQUALS, KEY_ESCAPE, KEY_F1, KEY_F2, KEY_F3,
		KEY_F4, KEY_F5, KEY_F6, KEY_F7, KEY_F8, KEY_F9, KEY_F10, KEY_F11, KEY_F12, KEY_LEFTARROW,
		KEY_MINUS, KEY_PAUSE, KEY_RALT, KEY_RCTRL, KEY_RIGHTARROW, KEY_RSHIFT, KEY_TAB,
		KEY_UPARROW, SCREENHEIGHT, SCREENWIDTH,
	},
	i_system::{I_Error, I_GetTime, I_Quit},
	m_argv::M_CheckParm,
	myargv,
	v_video::{gammatable, screens, usegamma},
};

const POINTER_WARP_COUNTDOWN: usize = 1;

static mut X_display: *mut Display = null_mut();
static mut X_mainWindow: Window = 0;
static mut X_cmap: Colormap = 0;
static mut X_visual: *mut Visual = null_mut();
static mut X_gc: GC = null_mut();
static mut X_event: MaybeUninit<XEvent> = MaybeUninit::uninit();
static mut X_screen: i32 = 0;
static mut X_visualinfo: MaybeUninit<XVisualInfo> = MaybeUninit::uninit();
static mut image: *mut XImage = null_mut();
static mut X_width: usize = 0;
static mut X_height: usize = 0;

// MIT SHared Memory extension.
static mut doShm: bool = false;

static mut X_shminfo: MaybeUninit<XShmSegmentInfo> = MaybeUninit::uninit();
static mut X_shmeventtype: i32 = 0;

// Fake mouse handling.
// This cannot work properly w/o DGA.
// Needs an invisible mouse cursor at least.
static mut grabMouse: bool = false;
static mut doPointerWarp: usize = POINTER_WARP_COUNTDOWN;

// Blocky mode,
// replace each 320x200 pixel with multiply*multiply pixels.
// According to Dave Taylor, it still is a bonehead thing
// to use ....
static mut multiply: usize = 1;

//  Translates the key currently in X_event
#[allow(static_mut_refs)]
fn xlatekey() -> u8 {
	const A: u32 = b'A' as u32;
	const Z: u32 = b'Z' as u32;
	unsafe {
		match XKeycodeToKeysym(X_display, X_event.assume_init_ref().key.keycode as u8, 0) {
			XK_Left => KEY_LEFTARROW,
			XK_Right => KEY_RIGHTARROW,
			XK_Down => KEY_DOWNARROW,
			XK_Up => KEY_UPARROW,
			XK_Escape => KEY_ESCAPE,
			XK_Return => KEY_ENTER,
			XK_Tab => KEY_TAB,
			XK_F1 => KEY_F1,
			XK_F2 => KEY_F2,
			XK_F3 => KEY_F3,
			XK_F4 => KEY_F4,
			XK_F5 => KEY_F5,
			XK_F6 => KEY_F6,
			XK_F7 => KEY_F7,
			XK_F8 => KEY_F8,
			XK_F9 => KEY_F9,
			XK_F10 => KEY_F10,
			XK_F11 => KEY_F11,
			XK_F12 => KEY_F12,
			XK_BackSpace | XK_Delete => KEY_BACKSPACE,
			XK_Pause => KEY_PAUSE,
			XK_KP_Equal | XK_equal => KEY_EQUALS,
			XK_KP_Subtract | XK_minus => KEY_MINUS,
			XK_Shift_L | XK_Shift_R => KEY_RSHIFT,
			XK_Control_L | XK_Control_R => KEY_RCTRL,
			XK_Alt_L | XK_Meta_L | XK_Alt_R | XK_Meta_R => KEY_RALT,
			rc @ A..=Z => rc as u8 - b'A' + b'a',
			rc => rc as u8,
		}
	}
}

#[allow(static_mut_refs)]
#[unsafe(no_mangle)]
pub extern "C" fn I_ShutdownGraphics() {
	unsafe {
		// Detach from X server
		if XShmDetach(X_display, X_shminfo.as_mut_ptr()) == 0 {
			I_Error(c"XShmDetach() failed in I_ShutdownGraphics()".as_ptr());
		}

		// Release shared memory.
		shmdt(X_shminfo.assume_init_ref().shmaddr.cast());
		shmctl(X_shminfo.assume_init_ref().shmid, IPC_RMID, null_mut());

		// Paranoia.
		(*image).data = null_mut();
	}
}

// I_StartFrame
pub fn I_StartFrame() {
	// er?
}

static mut lastmousex: i32 = 0;
static mut lastmousey: i32 = 0;
static mut mousemoved: bool = false;
static mut shmFinished: bool = false;

#[allow(static_mut_refs)]
fn I_GetEvent() {
	unsafe {
		let mut event = event_t { ty: evtype_t::ev_mouse, data1: 0, data2: 0, data3: 0 };

		// put event-grabbing stuff in here
		XNextEvent(X_display, X_event.as_mut_ptr());
		let xev = X_event.assume_init_ref();
		match xev.type_ {
			KeyPress => {
				event.ty = evtype_t::ev_keydown;
				event.data1 = xlatekey() as i32;
				D_PostEvent(&mut event);
			}
			KeyRelease => {
				event.ty = evtype_t::ev_keyup;
				event.data1 = xlatekey() as i32;
				D_PostEvent(&mut event);
			}
			ButtonPress => {
				event.ty = evtype_t::ev_mouse;
				event.data1 = (if xev.button.state & Button1Mask != 0 { 1 } else { 0 })
					| (if xev.button.state & Button2Mask != 0 { 2 } else { 0 })
					| (if xev.button.state & Button3Mask != 0 { 4 } else { 0 })
					| (if xev.button.button == Button1 { 1 } else { 0 })
					| (if xev.button.button == Button2 { 2 } else { 0 })
					| (if xev.button.button == Button3 { 4 } else { 0 });
				event.data2 = 0;
				event.data3 = 0;
				D_PostEvent(&mut event);
			}
			ButtonRelease => {
				event.ty = evtype_t::ev_mouse;
				event.data1 = (if xev.button.state & Button1Mask != 0 { 1 } else { 0 })
					| (if xev.button.state & Button2Mask != 0 { 2 } else { 0 })
					| (if xev.button.state & Button3Mask != 0 { 4 } else { 0 });
				// suggest parentheses around arithmetic in operand of |
				event.data1 = event.data1
					^ (if xev.button.button == Button1 { 1 } else { 0 })
					^ (if xev.button.button == Button2 { 2 } else { 0 })
					^ (if xev.button.button == Button3 { 4 } else { 0 });
				event.data2 = 0;
				event.data3 = 0;
				D_PostEvent(&mut event);
			}
			MotionNotify => {
				event.ty = evtype_t::ev_mouse;
				event.data1 = (if xev.button.state & Button1Mask != 0 { 1 } else { 0 })
					| (if xev.button.state & Button2Mask != 0 { 2 } else { 0 })
					| (if xev.button.state & Button3Mask != 0 { 4 } else { 0 });
				event.data2 = (xev.motion.x - lastmousex) << 2;
				event.data3 = (lastmousey - xev.motion.y) << 2;

				if event.data2 != 0 || event.data3 != 0 {
					lastmousex = xev.motion.x;
					lastmousey = xev.motion.y;
					if xev.motion.x != (X_width / 2) as i32 && xev.motion.y != (X_height / 2) as i32
					{
						D_PostEvent(&mut event);
						mousemoved = false;
					} else {
						mousemoved = true;
					}
				}
			}

			Expose | ConfigureNotify => {}
			_ => {
				if doShm && xev.type_ == X_shmeventtype {
					shmFinished = true;
				}
			}
		}
	}
}

fn createnullcursor(display: *mut Display, root: Window) -> Cursor {
	unsafe {
		let cursormask = XCreatePixmap(display, root, 1, 1, 1 /*depth*/);
		let mut xgc = mem::zeroed::<XGCValues>();
		xgc.function = GXclear;
		let gc = XCreateGC(display, cursormask, GCFunction, &raw mut xgc);
		XFillRectangle(display, cursormask, gc, 0, 0, 1, 1);
		let mut dummycolour = mem::zeroed::<XColor>();
		dummycolour.pixel = 0;
		dummycolour.red = 0;
		dummycolour.flags = 4;
		let cursor = XCreatePixmapCursor(
			display,
			cursormask,
			cursormask,
			&raw mut dummycolour,
			&raw mut dummycolour,
			0,
			0,
		);
		XFreePixmap(display, cursormask);
		XFreeGC(display, gc);
		cursor
	}
}

// I_StartTic
#[unsafe(no_mangle)]
pub extern "C" fn I_StartTic() {
	unsafe {
		if X_display.is_null() {
			return;
		}

		while XPending(X_display) != 0 {
			I_GetEvent();
		}

		// Warp the pointer back to the middle of the window
		//  or it will wander off - that is, the game will
		//  loose input focus within X11.
		if grabMouse {
			doPointerWarp -= 1;
			if doPointerWarp == 0 {
				XWarpPointer(
					X_display,
					0,
					X_mainWindow,
					0,
					0,
					0,
					0,
					(X_width / 2) as i32,
					(X_height / 2) as i32,
				);
				doPointerWarp = POINTER_WARP_COUNTDOWN;
			}
		}

		mousemoved = false;
	}
}

// I_UpdateNoBlit
pub fn I_UpdateNoBlit() {
	// what is this?
}

// I_FinishUpdate
pub fn I_FinishUpdate() {
	unsafe {
		static mut lasttic: usize = 0;
		// UNUSED static unsigned char *bigscreen=0;

		// draws little dots on the bottom of the screen
		if devparm != 0 {
			let i = I_GetTime();
			let tics = usize::min(i - lasttic, 20);
			lasttic = i;

			for i in (0..tics * 2).step_by(2) {
				*screens[0].wrapping_add((SCREENHEIGHT - 1) * SCREENWIDTH + i) = 0xff;
			}
			for i in (tics * 2..20 * 2).step_by(2) {
				*screens[0].wrapping_add((SCREENHEIGHT - 1) * SCREENWIDTH + i) = 0x0;
			}
		}

		// scales the screen size before blitting it
		if multiply == 2 {
			let mut ilineptr: *mut u32 = (screens[0]).cast();
			let mut olineptrs = [null_mut(); 2];
			for (i, olineptr) in olineptrs.iter_mut().enumerate() {
				*olineptr = (*image).data.wrapping_add(i * X_width).cast();
			}

			let mut y = SCREENHEIGHT;
			while y != 0 {
				y -= 1;
				let mut x = SCREENWIDTH;
				loop {
					let fouripixels = *ilineptr;
					ilineptr = ilineptr.wrapping_add(1);
					let twoopixels = (fouripixels & 0xff000000)
						| ((fouripixels >> 8) & 0xffff00)
						| ((fouripixels >> 16) & 0xff);
					let twomoreopixels = ((fouripixels << 16) & 0xff000000)
						| ((fouripixels << 8) & 0xffff00)
						| (fouripixels & 0xff);
					*olineptrs[0] = twomoreopixels;
					olineptrs[0] = olineptrs[0].wrapping_add(1);
					*olineptrs[1] = twomoreopixels;
					olineptrs[1] = olineptrs[1].wrapping_add(1);
					*olineptrs[0] = twoopixels;
					olineptrs[0] = olineptrs[0].wrapping_add(1);
					*olineptrs[1] = twoopixels;
					olineptrs[1] = olineptrs[1].wrapping_add(1);
					x -= 4;
					if x == 0 {
						break;
					}
				}

				olineptrs[0] = olineptrs[0].wrapping_add(X_width / 4);
				olineptrs[1] = olineptrs[1].wrapping_add(X_width / 4);
			}
		} else if multiply == 3 {
			let mut ilineptr: *mut u32 = screens[0].cast();
			let mut olineptrs = [null_mut(); 3];
			for (i, olineptr) in olineptrs.iter_mut().enumerate() {
				*olineptr = (*image).data.wrapping_add(i * X_width).cast();
			}

			let mut y = SCREENHEIGHT;
			while y != 0 {
				y -= 1;
				let mut x = SCREENWIDTH;
				loop {
					let fouripixels = *ilineptr;
					ilineptr = ilineptr.wrapping_add(1);
					let mut fouropixels = [0; 3];
					fouropixels[0] = (fouripixels & 0xff000000)
						| ((fouripixels >> 8) & 0xff0000)
						| ((fouripixels >> 16) & 0xffff);
					fouropixels[1] = ((fouripixels << 8) & 0xff000000)
						| (fouripixels & 0xffff00)
						| ((fouripixels >> 8) & 0xff);
					fouropixels[2] = ((fouripixels << 16) & 0xffff0000)
						| ((fouripixels << 8) & 0xff00)
						| (fouripixels & 0xff);
					*olineptrs[0] = fouropixels[2];
					olineptrs[0] = olineptrs[0].wrapping_add(1);
					*olineptrs[1] = fouropixels[2];
					olineptrs[1] = olineptrs[1].wrapping_add(1);
					*olineptrs[2] = fouropixels[2];
					olineptrs[2] = olineptrs[2].wrapping_add(1);
					*olineptrs[0] = fouropixels[1];
					olineptrs[0] = olineptrs[0].wrapping_add(1);
					*olineptrs[1] = fouropixels[1];
					olineptrs[1] = olineptrs[1].wrapping_add(1);
					*olineptrs[2] = fouropixels[1];
					olineptrs[2] = olineptrs[2].wrapping_add(1);
					*olineptrs[0] = fouropixels[0];
					olineptrs[0] = olineptrs[0].wrapping_add(1);
					*olineptrs[1] = fouropixels[0];
					olineptrs[1] = olineptrs[1].wrapping_add(1);
					*olineptrs[2] = fouropixels[0];
					olineptrs[2] = olineptrs[2].wrapping_add(1);
					x -= 4;
					if x == 0 {
						break;
					}
				}
				olineptrs[0] = olineptrs[0].wrapping_add(2 * X_width / 4);
				olineptrs[1] = olineptrs[1].wrapping_add(2 * X_width / 4);
				olineptrs[2] = olineptrs[2].wrapping_add(2 * X_width / 4);
			}
		} else if multiply == 4 {
			// Broken. Gotta fix this some day.
			Expand4(screens[0].cast(), (*image).data.cast());
		}

		if doShm {
			if XShmPutImage(
				X_display,
				X_mainWindow,
				X_gc,
				image,
				0,
				0,
				0,
				0,
				X_width as u32,
				X_height as u32,
				True,
			) == 0
			{
				I_Error(c"XShmPutImage() failed\n".as_ptr());
			}

			// wait for it to finish and processes all input events
			shmFinished = false;
			loop {
				I_GetEvent();
				if shmFinished {
					break;
				}
			}
		} else {
			// draw the image
			XPutImage(
				X_display,
				X_mainWindow,
				X_gc,
				image,
				0,
				0,
				0,
				0,
				X_width as u32,
				X_height as u32,
			);

			// sync up with server
			XSync(X_display, False);
		}
	}
}

// I_ReadScreen
pub fn I_ReadScreen(scr: *mut u8) {
	unsafe { libc::memcpy(scr.cast(), screens[0].cast(), SCREENWIDTH * SCREENHEIGHT) };
}

// Palette stuff.

static mut colors: [XColor; 256] =
	[XColor { pixel: 0, red: 0, green: 0, blue: 0, flags: 0, pad: 0 }; 256];

#[allow(static_mut_refs)]
fn UploadNewPalette(cmap: Colormap, mut palette: *mut u8) {
	unsafe {
		static mut firstcall: bool = true;

		if X_visualinfo.assume_init_ref().class == PseudoColor
			&& X_visualinfo.assume_init_ref().depth == 8
		{
			// initialize the colormap
			if firstcall {
				firstcall = false;
				#[allow(clippy::needless_range_loop)]
				for i in 0..256 {
					colors[i].pixel = i as u32;
					colors[i].flags = DoRed | DoGreen | DoBlue;
				}
			}

			// set the X colormap entries
			#[allow(clippy::needless_range_loop)]
			for i in 0..256 {
				let c = gammatable[usegamma][*palette as usize] as u16;
				palette = palette.wrapping_add(1);
				colors[i].red = (c << 8) + c;
				let c = gammatable[usegamma][*palette as usize] as u16;
				palette = palette.wrapping_add(1);
				colors[i].green = (c << 8) + c;
				let c = gammatable[usegamma][*palette as usize] as u16;
				palette = palette.wrapping_add(1);
				colors[i].blue = (c << 8) + c;
			}

			// store the colors to the current colormap
			XStoreColors(X_display, cmap, colors.as_mut_ptr(), 256);
		}
	}
}

// I_SetPalette
pub fn I_SetPalette(palette: *mut u8) {
	unsafe {
		UploadNewPalette(X_cmap, palette);
	}
}

// This function is probably redundant,
//  if XShmDetach works properly.
// ddt never detached the XShm memory,
//  thus there might have been stale
//  handles accumulating.
#[allow(static_mut_refs)]
fn grabsharedmemory(size: i32) {
	let size = size as usize;
	unsafe {
		let mut key = i32::from_be_bytes(*b"doom");
		// struct shmid_ds	shminfo;
		let minsize = 320 * 200;
		let mut id;
		// int			rc;
		let mut pollution = 5;

		// try to use what was here before
		loop {
			id = shmget(key, minsize, 0o777); // just get the id
			if id != -1 {
				let mut shminfo = mem::zeroed::<shmid_ds>();
				let mut rc = shmctl(id, IPC_STAT, &raw mut shminfo); // get stats on it
				if rc == 0 {
					if shminfo.shm_nattch != 0 {
						eprintln!(
							"User {} appears to be running DOOM.  Is that wise?",
							shminfo.shm_cpid
						);
						key += 1;
					} else {
						if getuid() == shminfo.shm_perm.cuid {
							rc = shmctl(id, IPC_RMID, null_mut());
							if rc == 0 {
								eprintln!("Was able to kill my old shared memory");
							} else {
								I_Error(c"Was NOT able to kill my old shared memory".as_ptr());
							}

							id = shmget(key, size, IPC_CREAT | 0o777);
							if id == -1 {
								I_Error(c"Could not get shared memory".as_ptr());
							}

							_ = shmctl(id, IPC_STAT, &raw mut shminfo);

							break;
						}
						if size >= shminfo.shm_segsz {
							eprintln!("will use {}'s stale shared memory", shminfo.shm_cpid);
							break;
						} else {
							eprintln!(
								"warning: can't use stale shared memory belonging to id {}, key=0x{:x}\n",
								shminfo.shm_cpid, key
							);
							key += 1;
						}
					}
				} else {
					I_Error(c"could not get stats on key=%d".as_ptr(), key);
				}
			} else {
				id = shmget(key, size, IPC_CREAT | 0o777);
				if id == -1 {
					// eprintln!("errno={}", errno);
					I_Error(c"Could not get any shared memory".as_ptr());
				}
				break;
			}
			pollution -= 1;
			if pollution == 0 {
				break;
			}
		}

		if pollution == 0 {
			I_Error(c"Sorry, system too polluted with stale shared memory segments.\n".as_ptr());
		}

		X_shminfo.assume_init_mut().shmid = id;

		// attach to the shared memory segment
		(*image).data = shmat(id, null_mut(), 0).cast();
		X_shminfo.assume_init_mut().shmaddr = (*image).data;

		eprintln!("shared memory id={}, addr={:p}", id, (*image).data);
	}
}

#[allow(static_mut_refs)]
pub fn I_InitGraphics() {
	unsafe {
		static mut firsttime: bool = true;

		if !firsttime {
			return;
		}
		firsttime = false;

		libc::signal(SIGINT, I_Quit as *const () as _);

		if M_CheckParm(c"-2".as_ptr()) != 0 {
			multiply = 2;
		}

		if M_CheckParm(c"-3".as_ptr()) != 0 {
			multiply = 3;
		}

		if M_CheckParm(c"-4".as_ptr()) != 0 {
			multiply = 4;
		}

		X_width = SCREENWIDTH * multiply;
		X_height = SCREENHEIGHT * multiply;

		// check for command-line display name
		let mut displayname = if let pnum @ 1.. = M_CheckParm(c"-disp".as_ptr()) {
			*myargv.wrapping_add(pnum + 1)
		} else {
			null_mut()
		};

		// check if the user wants to grab the mouse (quite unnice)
		grabMouse = M_CheckParm(c"-grabmouse".as_ptr()) != 0;

		let mut x: i32 = 0;
		let mut y: i32 = 0;

		// warning: char format, different type arg
		let mut xsign: u8 = b' ';
		let mut ysign: u8 = b' ';

		// check for command-line geometry
		if let pnum @ 1.. = M_CheckParm(c"-geom".as_ptr()) {
			// warning: char format, different type arg 3,5
			let n = libc::sscanf(
				*myargv.wrapping_add(pnum + 1),
				c"%c%d%c%d".as_ptr(),
				&raw mut xsign,
				&raw mut x,
				&raw mut ysign,
				&raw mut y,
			);

			if n == 2 {
				x = 0;
				y = 0;
			} else if n == 6 {
				if xsign == b'-' {
					x = -x;
				}
				if ysign == b'-' {
					y = -y;
				}
			} else {
				I_Error(c"bad -geom parameter".as_ptr());
			}
		}

		// open the display
		X_display = XOpenDisplay(displayname);
		if X_display.is_null() {
			if !displayname.is_null() {
				I_Error(c"Could not open display [%s]".as_ptr(), displayname);
			} else {
				I_Error(
					c"Could not open display (DISPLAY=[%s])".as_ptr(),
					getenv(c"DISPLAY".as_ptr()),
				);
			}
		}

		// use the default visual
		X_screen = XDefaultScreen(X_display);
		if XMatchVisualInfo(X_display, X_screen, 8, PseudoColor, X_visualinfo.as_mut_ptr()) == 0 {
			I_Error(c"xdoom currently only supports 256-color PseudoColor screens".as_ptr());
		}
		X_visual = X_visualinfo.assume_init_ref().visual;

		// check for the MITSHM extension
		doShm = XShmQueryExtension(X_display) != 0;

		// even if it's available, make sure it's a local connection
		if doShm {
			if displayname.is_null() {
				displayname = getenv(c"DISPLAY".as_ptr());
			}
			if !displayname.is_null() {
				let mut d = displayname;
				while *d != 0 && *d != b':' as i8 {
					d = d.wrapping_add(1);
				}
				if *d != 0 {
					*d = 0;
				}
				if strcasecmp(displayname, c"unix".as_ptr()) != 0 && *displayname != 0 {
					doShm = false;
				}
			}
		}

		eprintln!("Using MITSHM extension");

		// create the colormap
		X_cmap = XCreateColormap(X_display, XRootWindow(X_display, X_screen), X_visual, AllocAll);

		// setup attributes for main window
		let attribmask = CWEventMask | CWColormap | CWBorderPixel;
		let mut attribs = mem::zeroed::<XSetWindowAttributes>();
		attribs.event_mask = KeyPressMask | KeyReleaseMask | ExposureMask;

		attribs.colormap = X_cmap;
		attribs.border_pixel = 0;

		// create the main window
		X_mainWindow = XCreateWindow(
			X_display,
			XRootWindow(X_display, X_screen),
			x,
			y,
			X_width as u32,
			X_height as u32,
			0, // borderwidth
			8, // depth
			InputOutput as u32,
			X_visual,
			attribmask,
			&raw mut attribs,
		);

		XInstallColormap(X_display, X_cmap);
		XDefineCursor(X_display, X_mainWindow, createnullcursor(X_display, X_mainWindow));

		// create the GC
		let valuemask = GCGraphicsExposures;
		let mut xgcvalues = mem::zeroed::<XGCValues>();
		xgcvalues.graphics_exposures = False;
		X_gc = XCreateGC(X_display, X_mainWindow, valuemask, &raw mut xgcvalues);

		// map the window
		XMapWindow(X_display, X_mainWindow);

		// wait until it is OK to draw
		let mut oktodraw = false;
		while !oktodraw {
			XNextEvent(X_display, X_event.as_mut_ptr());
			if X_event.assume_init_ref().type_ == Expose
				&& X_event.assume_init_ref().expose.count == 0
			{
				oktodraw = true;
			}
		}

		// grabs the pointer so it is restricted to this window
		if grabMouse {
			XGrabPointer(
				X_display,
				X_mainWindow,
				True,
				(ButtonPressMask | ButtonReleaseMask | PointerMotionMask) as u32,
				GrabModeAsync,
				GrabModeAsync,
				X_mainWindow,
				0,
				CurrentTime,
			);
		}

		if doShm {
			X_shmeventtype = XShmGetEventBase(X_display);

			// create the image
			image = XShmCreateImage(
				X_display,
				X_visual,
				8,
				ZPixmap,
				null_mut(),
				X_shminfo.as_mut_ptr(),
				X_width as u32,
				X_height as u32,
			);

			grabsharedmemory((*image).bytes_per_line * (*image).height);

			if (*image).data.is_null() {
				I_Error(c"shmat() failed in InitGraphics()".as_ptr());
			}

			// get the X server to attach to it
			if XShmAttach(X_display, X_shminfo.as_mut_ptr()) == 0 {
				I_Error(c"XShmAttach() failed in InitGraphics()".as_ptr());
			}
		} else {
			image = XCreateImage(
				X_display,
				X_visual,
				8,
				ZPixmap,
				0,
				libc::malloc(X_width * X_height).cast(),
				X_width as u32,
				X_height as u32,
				8,
				X_width as i32,
			);
		}

		if multiply == 1 {
			screens[0] = (*image).data.cast();
		} else {
			screens[0] = libc::malloc(SCREENWIDTH * SCREENHEIGHT).cast();
		}
	}
}

static mut exptable2: [f64; 256 * 256] = [0.0; 256 * 256];

#[allow(static_mut_refs)]
fn InitExpand2() {
	unsafe {
		println!("building exptable2...");
		let mut exp = exptable2.as_mut_ptr();
		for i in 0..=255 {
			for j in 0..=255 {
				*exp = f64::from_le_bytes([i, i, i, i, j, j, j, j]);
				exp = exp.wrapping_add(1);
			}
		}
		println!("done.");
	}
}

static mut inited: bool = false;

#[allow(static_mut_refs)]
fn Expand4(mut lineptr: *mut usize, mut xline: *mut f64) {
	unsafe {
		let exp = exptable2.as_mut_ptr();
		if !inited {
			inited = true;
			InitExpand2();
		}

		let step = 3 * SCREENWIDTH / 2;

		let mut y = SCREENHEIGHT - 1;
		loop {
			let mut x = SCREENWIDTH;

			loop {
				let fourpixels = *lineptr;

				let dpixel = *exp.wrapping_byte_add((fourpixels & 0xffff0000) >> 13).cast();
				*xline.wrapping_add(0) = dpixel;
				*xline.wrapping_add(160) = dpixel;
				*xline.wrapping_add(320) = dpixel;
				*xline.wrapping_add(480) = dpixel;

				let dpixel = *(exp.wrapping_byte_add((fourpixels & 0xffff) << 3)).cast();
				*xline.wrapping_add(1) = dpixel;
				*xline.wrapping_add(161) = dpixel;
				*xline.wrapping_add(321) = dpixel;
				*xline.wrapping_add(481) = dpixel;

				let fourpixels = *lineptr.wrapping_add(1);

				let dpixel = *(exp.wrapping_byte_add((fourpixels & 0xffff0000) >> 13)).cast();
				*xline.wrapping_add(2) = dpixel;
				*xline.wrapping_add(162) = dpixel;
				*xline.wrapping_add(322) = dpixel;
				*xline.wrapping_add(482) = dpixel;

				let dpixel = *(exp.wrapping_byte_add((fourpixels & 0xffff) << 3)).cast();
				*xline.wrapping_add(3) = dpixel;
				*xline.wrapping_add(163) = dpixel;
				*xline.wrapping_add(323) = dpixel;
				*xline.wrapping_add(483) = dpixel;

				let fourpixels = *lineptr.wrapping_add(2);

				let dpixel = *(exp.wrapping_byte_add((fourpixels & 0xffff0000) >> 13)).cast();
				*xline.wrapping_add(4) = dpixel;
				*xline.wrapping_add(164) = dpixel;
				*xline.wrapping_add(324) = dpixel;
				*xline.wrapping_add(484) = dpixel;

				let dpixel = *(exp.wrapping_byte_add((fourpixels & 0xffff) << 3)).cast();
				*xline.wrapping_add(5) = dpixel;
				*xline.wrapping_add(165) = dpixel;
				*xline.wrapping_add(325) = dpixel;
				*xline.wrapping_add(485) = dpixel;

				let fourpixels = *lineptr.wrapping_add(3);

				let dpixel = *(exp.wrapping_byte_add((fourpixels & 0xffff0000) >> 13)).cast();
				*xline.wrapping_add(6) = dpixel;
				*xline.wrapping_add(166) = dpixel;
				*xline.wrapping_add(326) = dpixel;
				*xline.wrapping_add(486) = dpixel;

				let dpixel = *(exp.wrapping_byte_add((fourpixels & 0xffff) << 3)).cast();
				*xline.wrapping_add(7) = dpixel;
				*xline.wrapping_add(167) = dpixel;
				*xline.wrapping_add(327) = dpixel;
				*xline.wrapping_add(487) = dpixel;

				lineptr = lineptr.wrapping_add(4);
				xline = xline.wrapping_add(8);
				x -= 16;
				if x == 0 {
					break;
				}
			}
			xline = xline.wrapping_add(step);
			y -= 1;
			if y == 0 {
				break;
			}
		}
	}
}
