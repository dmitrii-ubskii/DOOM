#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::{
	ffi::{CStr, c_char, c_void},
	mem::MaybeUninit,
	ptr::{null, null_mut},
};

use libc::{O_CREAT, O_RDONLY, O_TRUNC, O_WRONLY};

use crate::{
	d_englsh::{
		HUSTR_CHATMACRO0, HUSTR_CHATMACRO1, HUSTR_CHATMACRO2, HUSTR_CHATMACRO3, HUSTR_CHATMACRO4,
		HUSTR_CHATMACRO5, HUSTR_CHATMACRO6, HUSTR_CHATMACRO7, HUSTR_CHATMACRO8, HUSTR_CHATMACRO9,
	},
	d_main::basedefault,
	doomdef::{
		KEY_DOWNARROW, KEY_LEFTARROW, KEY_RALT, KEY_RCTRL, KEY_RIGHTARROW, KEY_RSHIFT, KEY_UPARROW,
		SCREENHEIGHT, SCREENWIDTH,
	},
	g_game::{
		consoleplayer, joybfire, joybspeed, joybstrafe, joybuse, key_down, key_fire, key_left,
		key_right, key_speed, key_strafe, key_strafeleft, key_straferight, key_up, key_use,
		mousebfire, mousebforward, mousebstrafe, players,
	},
	hu_stuff::chat_macros,
	i_system::I_Error,
	m_argv::M_CheckParm,
	m_menu::{detailLevel, mouseSensitivity, screenblocks, showMessages},
	myargc, myargv,
	s_sound::{numChannels, snd_MusicVolume, snd_SfxVolume},
	v_video::{screens, usegamma},
	w_wad::W_CacheLumpName,
	z_zone::{PU_CACHE, PU_STATIC, Z_Free, Z_Malloc},
};

type int = i32;

// M_WriteFile
pub(crate) fn M_WriteFile(name: *const c_char, source: *mut c_void, length: usize) -> bool {
	unsafe {
		let handle = libc::open(name, O_WRONLY | O_CREAT | O_TRUNC, 0o666);

		if handle == -1 {
			return false;
		}

		let count = libc::write(handle, source, length);
		libc::close(handle);

		count >= length as isize
	}
}

// M_ReadFile
pub(crate) fn M_ReadFile(name: *const c_char, buffer: *mut *mut u8) -> usize {
	unsafe {
		let handle = libc::open(name, O_RDONLY, 0o666);
		if handle == -1 {
			I_Error(c"Couldn't read file %s".as_ptr(), name);
		}
		let mut fileinfo = MaybeUninit::uninit();
		if libc::fstat(handle, fileinfo.as_mut_ptr()) == -1 {
			I_Error(c"Couldn't read file %s".as_ptr(), name);
		}
		let length = fileinfo.assume_init().st_size as usize;
		let buf = Z_Malloc(length, PU_STATIC, null_mut());
		let count = libc::read(handle, buf, length);
		libc::close(handle);

		if count < length as isize {
			I_Error(c"Couldn't read file %s".as_ptr(), name);
		}

		*buffer = buf.cast();
		length
	}
}

// DEFAULTS
static mut usemouse: int = 0;
static mut usejoystick: int = 0;

// #ifdef LINUX
static mut mousetype: *mut c_char = null_mut();
static mut mousedev: *mut c_char = null_mut();
// #endif

struct default_t {
	pub name: *const c_char,
	pub location: *mut *const c_void,
	pub defaultvalue: *const c_void,
}

static mut defaults: [default_t; 39] = [
	default_t {
		name: c"mouse_sensitivity".as_ptr(),
		location: (&raw mut mouseSensitivity).cast(),
		defaultvalue: 5 as *mut c_void,
	},
	default_t {
		name: c"sfx_volume".as_ptr(),
		location: (&raw mut snd_SfxVolume).cast(),
		defaultvalue: 8 as *mut c_void,
	},
	default_t {
		name: c"music_volume".as_ptr(),
		location: (&raw mut snd_MusicVolume).cast(),
		defaultvalue: 8 as *mut c_void,
	},
	default_t {
		name: c"show_messages".as_ptr(),
		location: (&raw mut showMessages).cast(),
		defaultvalue: 1 as *mut c_void,
	},
	// #ifdef NORMALUNIX
	default_t {
		name: c"key_right".as_ptr(),
		location: (&raw mut key_right).cast(),
		defaultvalue: KEY_RIGHTARROW as *mut c_void,
	},
	default_t {
		name: c"key_left".as_ptr(),
		location: (&raw mut key_left).cast(),
		defaultvalue: KEY_LEFTARROW as *mut c_void,
	},
	default_t {
		name: c"key_up".as_ptr(),
		location: (&raw mut key_up).cast(),
		defaultvalue: KEY_UPARROW as *mut c_void,
	},
	default_t {
		name: c"key_down".as_ptr(),
		location: (&raw mut key_down).cast(),
		defaultvalue: KEY_DOWNARROW as *mut c_void,
	},
	default_t {
		name: c"key_strafeleft".as_ptr(),
		location: (&raw mut key_strafeleft).cast(),
		defaultvalue: b',' as *mut c_void,
	},
	default_t {
		name: c"key_straferight".as_ptr(),
		location: (&raw mut key_straferight).cast(),
		defaultvalue: b'.' as *mut c_void,
	},
	default_t {
		name: c"key_fire".as_ptr(),
		location: (&raw mut key_fire).cast(),
		defaultvalue: KEY_RCTRL as *mut c_void,
	},
	default_t {
		name: c"key_use".as_ptr(),
		location: (&raw mut key_use).cast(),
		defaultvalue: b' ' as *mut c_void,
	},
	default_t {
		name: c"key_strafe".as_ptr(),
		location: (&raw mut key_strafe).cast(),
		defaultvalue: KEY_RALT as *mut c_void,
	},
	default_t {
		name: c"key_speed".as_ptr(),
		location: (&raw mut key_speed).cast(),
		defaultvalue: KEY_RSHIFT as *mut c_void,
	},
	// // UNIX hack, to be removed.
	// #ifdef SNDSERV
	// 	default_t { name: c"sndserver".as_ptr(), location: &raw mut sndserver_filename, defaultvalue: (int) "sndserver" },
	// 	default_t { name: c"mb_used".as_ptr(), location: &raw mut mb_used, defaultvalue: 2 },
	// #endif
	// #endif

	// #ifdef LINUX
	default_t {
		name: c"mousedev".as_ptr(),
		location: (&raw mut mousedev).cast(),
		defaultvalue: c"/dev/ttyS0".as_ptr().cast(),
	},
	default_t {
		name: c"mousetype".as_ptr(),
		location: (&raw mut mousetype).cast(),
		defaultvalue: c"microsoft".as_ptr().cast(),
	},
	// #endif
	default_t {
		name: c"use_mouse".as_ptr(),
		location: (&raw mut usemouse).cast(),
		defaultvalue: 1 as *mut c_void,
	},
	default_t {
		name: c"mouseb_fire".as_ptr(),
		location: (&raw mut mousebfire).cast(),
		defaultvalue: null(),
	},
	default_t {
		name: c"mouseb_strafe".as_ptr(),
		location: (&raw mut mousebstrafe).cast(),
		defaultvalue: 1 as *const c_void,
	},
	default_t {
		name: c"mouseb_forward".as_ptr(),
		location: (&raw mut mousebforward).cast(),
		defaultvalue: 2 as *const c_void,
	},
	default_t {
		name: c"use_joystick".as_ptr(),
		location: (&raw mut usejoystick).cast(),
		defaultvalue: null(),
	},
	default_t {
		name: c"joyb_fire".as_ptr(),
		location: (&raw mut joybfire).cast(),
		defaultvalue: null(),
	},
	default_t {
		name: c"joyb_strafe".as_ptr(),
		location: (&raw mut joybstrafe).cast(),
		defaultvalue: 1 as *const c_void,
	},
	default_t {
		name: c"joyb_use".as_ptr(),
		location: (&raw mut joybuse).cast(),
		defaultvalue: 3 as *const c_void,
	},
	default_t {
		name: c"joyb_speed".as_ptr(),
		location: (&raw mut joybspeed).cast(),
		defaultvalue: 2 as *const c_void,
	},
	default_t {
		name: c"screenblocks".as_ptr(),
		location: (&raw mut screenblocks).cast(),
		defaultvalue: 9 as *const c_void,
	},
	default_t {
		name: c"detaillevel".as_ptr(),
		location: (&raw mut detailLevel).cast(),
		defaultvalue: null(),
	},
	default_t {
		name: c"snd_channels".as_ptr(),
		location: (&raw mut numChannels).cast(),
		defaultvalue: 3 as *const c_void,
	},
	default_t {
		name: c"usegamma".as_ptr(),
		location: (&raw mut usegamma).cast(),
		defaultvalue: null(),
	},
	default_t {
		name: c"chatmacro0".as_ptr(),
		location: unsafe { { &raw mut chat_macros[0] }.cast() },
		defaultvalue: HUSTR_CHATMACRO0.0.cast(),
	},
	default_t {
		name: c"chatmacro1".as_ptr(),
		location: unsafe { { &raw mut chat_macros[1] }.cast() },
		defaultvalue: HUSTR_CHATMACRO1.0.cast(),
	},
	default_t {
		name: c"chatmacro2".as_ptr(),
		location: unsafe { { &raw mut chat_macros[2] }.cast() },
		defaultvalue: HUSTR_CHATMACRO2.0.cast(),
	},
	default_t {
		name: c"chatmacro3".as_ptr(),
		location: unsafe { { &raw mut chat_macros[3] }.cast() },
		defaultvalue: HUSTR_CHATMACRO3.0.cast(),
	},
	default_t {
		name: c"chatmacro4".as_ptr(),
		location: unsafe { { &raw mut chat_macros[4] }.cast() },
		defaultvalue: HUSTR_CHATMACRO4.0.cast(),
	},
	default_t {
		name: c"chatmacro5".as_ptr(),
		location: unsafe { { &raw mut chat_macros[5] }.cast() },
		defaultvalue: HUSTR_CHATMACRO5.0.cast(),
	},
	default_t {
		name: c"chatmacro6".as_ptr(),
		location: unsafe { { &raw mut chat_macros[6] }.cast() },
		defaultvalue: HUSTR_CHATMACRO6.0.cast(),
	},
	default_t {
		name: c"chatmacro7".as_ptr(),
		location: unsafe { { &raw mut chat_macros[7] }.cast() },
		defaultvalue: HUSTR_CHATMACRO7.0.cast(),
	},
	default_t {
		name: c"chatmacro8".as_ptr(),
		location: unsafe { { &raw mut chat_macros[8] }.cast() },
		defaultvalue: HUSTR_CHATMACRO8.0.cast(),
	},
	default_t {
		name: c"chatmacro9".as_ptr(),
		location: unsafe { { &raw mut chat_macros[9] }.cast() },
		defaultvalue: HUSTR_CHATMACRO9.0.cast(),
	},
];

static mut numdefaults: usize = 0;
static mut defaultfile: *const c_char = null_mut();

// M_SaveDefaults
pub(crate) fn M_SaveDefaults() {
	unsafe {
		let f = libc::fopen(defaultfile, c"w".as_ptr());
		if f.is_null() {
			return; // can't write the file, but don't complain
		}

		#[allow(clippy::needless_range_loop)]
		for i in 0..numdefaults {
			if defaults[i].defaultvalue as isize > -0xfff
				&& (defaults[i].defaultvalue as isize) < 0xfff
			{
				let v = *defaults[i].location;
				libc::fprintf(f, c"%s\t\t%i\n".as_ptr(), defaults[i].name, v);
			} else {
				libc::fprintf(
					f,
					c"%s\t\t\"%s\"\n".as_ptr(),
					defaults[i].name,
					*defaults[i].location,
				);
			}
		}

		libc::fclose(f);
	}
}

// M_LoadDefaults
#[allow(static_mut_refs)]
pub(crate) fn M_LoadDefaults() {
	unsafe {
		// set everything to base values
		numdefaults = size_of_val(&defaults) / size_of_val(&defaults[0]);
		#[allow(clippy::needless_range_loop)]
		for i in 0..numdefaults {
			*defaults[i].location = defaults[i].defaultvalue;
		}

		// check for a custom default file
		let i = M_CheckParm(c"-config".as_ptr());
		if i != 0 && i < myargc - 1 {
			defaultfile = *myargv.wrapping_add(i + 1);
			println!("	default file: {}", CStr::from_ptr(defaultfile).to_str().unwrap());
		} else {
			defaultfile = basedefault.as_ptr();
		}

		// read the file in, overriding any set defaults
		let f = libc::fopen(defaultfile, c"r".as_ptr());
		if !f.is_null() {
			let mut def = [0; 80];
			let mut strparm = [0; 100];
			while libc::feof(f) == 0 {
				if libc::fscanf(
					f,
					c"%79s %[^\n]\n".as_ptr(),
					def.as_mut_ptr(),
					strparm.as_mut_ptr(),
				) == 2
				{
					let mut isstring = false;
					let mut parm: i32 = 0;
					let mut newstring = null_mut();
					if strparm[0] == b'"' {
						// get a string default
						isstring = true;
						let len = libc::strlen(strparm.as_ptr().cast());
						newstring = libc::malloc(len).cast();
						strparm[len - 1] = 0;
						libc::strcpy(newstring, strparm[1..].as_ptr().cast());
					} else if strparm[0] == b'0' && strparm[1] == b'x' {
						libc::sscanf(strparm[2..].as_ptr().cast(), c"%x".as_ptr(), &raw mut parm);
					} else {
						libc::sscanf(strparm.as_ptr().cast(), c"%i".as_ptr(), &raw mut parm);
					}
					#[allow(clippy::needless_range_loop)]
					for i in 0..numdefaults {
						if libc::strcmp(def.as_ptr(), defaults[i].name) == 0 {
							if !isstring {
								*defaults[i].location = parm as *const c_void;
							} else {
								*defaults[i].location = newstring.cast();
							}
							break;
						}
					}
				}
			}

			libc::fclose(f);
		}
	}
}

// SCREEN SHOTS
struct pcx_t {
	manufacturer: u8,
	version: u8,
	encoding: u8,
	bits_per_pixel: u8,

	xmin: u16,
	ymin: u16,
	xmax: u16,
	ymax: u16,

	hres: u16,
	vres: u16,

	palette: [u8; 48],

	_reserved: u8,
	color_planes: u8,

	bytes_per_line: u16,
	palette_type: u16,

	filler: [u8; 58],
	data: u8, // unbounded
}

// WritePCXfile
fn WritePCXfile(
	filename: *const c_char,
	mut data: *const u8,
	width: usize,
	height: usize,
	mut palette: *const u8,
) {
	unsafe {
		let pcx = Z_Malloc(width * height * 2 + 1000, PU_STATIC, null_mut()).cast::<pcx_t>();

		(*pcx).manufacturer = 0x0a; // PCX id
		(*pcx).version = 5; // 256 color
		(*pcx).encoding = 1; // uncompressed
		(*pcx).bits_per_pixel = 8; // 256 color
		(*pcx).xmin = 0;
		(*pcx).ymin = 0;
		(*pcx).xmax = (width - 1) as u16;
		(*pcx).ymax = (height - 1) as u16;
		(*pcx).hres = (width) as u16;
		(*pcx).vres = (height) as u16;
		(*pcx).palette = [0; 48];
		(*pcx).color_planes = 1; // chunky image
		(*pcx).bytes_per_line = width as u16;
		(*pcx).palette_type = 2; // not a grey scale
		(*pcx).filler = [0; 58];

		// pack the image
		let mut pack = &raw mut (*pcx).data;

		for _ in 0..width * height {
			if (*data & 0xc0) != 0xc0 {
				*pack = *data;
				pack = pack.wrapping_add(1);
				data = data.wrapping_add(1);
			} else {
				*pack = 0xc1;
				pack = pack.wrapping_add(1);
				*pack = *data;
				pack = pack.wrapping_add(1);
				data = data.wrapping_add(1);
			}
		}

		// write the palette
		*pack = 0x0c; // palette ID byte
		pack = pack.wrapping_add(1);
		for _ in 0..768 {
			*pack = *palette;
			pack = pack.wrapping_add(1);
			palette = palette.wrapping_add(1);
		}

		// write output file
		let length = pack.offset_from(pcx.cast());
		M_WriteFile(filename, pcx.cast(), length as usize);

		Z_Free(pcx.cast());
	}
}

unsafe extern "C" {
	fn I_ReadScreen(scr: *mut u8);
}

// M_ScreenShot
pub(crate) fn M_ScreenShot() {
	unsafe {
		let mut lbmname = [0; 12];

		// munge planar buffer to linear
		let linear = screens[2];
		I_ReadScreen(linear);

		// find a file name to save it to
		libc::strcpy(lbmname.as_mut_ptr(), c"DOOM00.pcx".as_ptr());

		let mut i = 0;
		for _ in 0..=99 {
			lbmname[4] = i / 10 + b'0' as c_char;
			lbmname[5] = i % 10 + b'0' as c_char;
			if libc::access(lbmname.as_ptr(), 0) == -1 {
				break; // file doesn't exist
			}
			i += 1;
		}
		if i == 100 {
			I_Error(c"M_ScreenShot: Couldn't create a PCX".as_ptr());
		}

		// save the pcx file
		WritePCXfile(
			lbmname.as_ptr(),
			linear,
			SCREENWIDTH,
			SCREENHEIGHT,
			W_CacheLumpName(c"PLAYPAL".as_ptr(), PU_CACHE).cast(),
		);

		players[consoleplayer].message = c"screen shot".as_ptr();
	}
}
