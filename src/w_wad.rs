#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::{
	ffi::{CStr, c_char, c_void},
	mem::MaybeUninit,
	ptr::{self, null_mut},
};

use libc::{O_RDONLY, SEEK_SET, open};

use crate::{
	i_system::I_Error,
	z_zone::{Z_ChangeTag, Z_Free, Z_Malloc},
};

type int = i32;

// TYPES
#[repr(C)]
pub(crate) struct wadinfo_t {
	// Should be "IWAD" or "PWAD".
	pub(crate) identification: [c_char; 4],
	pub(crate) numlumps: usize,
	pub(crate) infotableofs: int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct filelump_t {
	pub(crate) filepos: int,
	pub(crate) size: usize,
	pub(crate) name: [c_char; 8],
}

// WADFILE I/O related stuff.
#[repr(C)]
pub(crate) struct lumpinfo_t {
	pub(crate) name: [c_char; 8],
	pub(crate) handle: int,
	pub(crate) position: int,
	pub(crate) size: usize,
}

// GLOBALS

// Location of each lump on disk.
pub(crate) static mut lumpinfo: *mut lumpinfo_t = null_mut();
pub(crate) static mut numlumps: usize = 0;

static mut lumpcache: *mut *mut c_void = null_mut();

// #define strcmpi	strcasecmp
const strcmpi: unsafe extern "C" fn(*const i8, *const i8) -> i32 = libc::strcasecmp;

fn toupper(c: c_char) -> i8 {
	c_char::try_from(u32::from(char::from(u8::try_from(c).unwrap()).to_ascii_uppercase())).unwrap()
}

fn strupr(mut s: *mut c_char) {
	unsafe {
		while *s != 0 {
			*s = toupper(*s);
			s = s.wrapping_byte_add(1);
		}
	}
}

fn filelength(handle: i32) -> usize {
	unsafe {
		let mut fileinfo = MaybeUninit::uninit();

		if libc::fstat(handle, fileinfo.as_mut_ptr()) == -1 {
			I_Error("Error fstating");
		}

		usize::try_from(fileinfo.assume_init().st_size).unwrap()
	}
}

fn ExtractFileBase(path: *const c_char, mut dest: *mut c_char) {
	unsafe {
		let mut src = path.wrapping_byte_add(libc::strlen(path) - 1);

		// back up until a \ or the start
		while src != path
			&& *(src.wrapping_sub(1)) != i8::try_from(b'\\').unwrap()
			&& *(src.wrapping_sub(1)) != i8::try_from(b'/').unwrap()
		{
			src = src.wrapping_byte_offset(-1);
		}

		// copy up to eight characters
		ptr::write_bytes(dest, 0, 8);
		let mut length = 0;

		while *src != 0 && *src != i8::try_from(b'.').unwrap() {
			length += 1;
			if length == 9 {
				I_Error(format_args!(
					"Filename base of {} >8 chars",
					CStr::from_ptr(path).to_str().unwrap(),
				));
			}

			*dest = toupper(*src);
			dest = dest.wrapping_byte_add(1);
			src = src.wrapping_byte_add(1);
		}
	}
}

// LUMP BASED ROUTINES.

// W_AddFile
// All files are optional, but at least one file must be
//  found (PWAD, if all required lumps are present).
// Files with a .wad extension are wadlink files
//  with multiple lumps.
// Other files are single lumps with the base filename
//  for the lump name.
//
// If filename starts with a tilde, the file is handled
//  specially to allow map reloads.
// But: the reload feature is a fragile hack...

static mut reloadlump: usize = 0;
static mut reloadname: *const c_char = null_mut();

fn W_AddFile(mut filename: *const c_char) {
	unsafe {
		// open the file and add to directory

		// handle reload indicator.
		if *filename == c_char::try_from(b'~').unwrap() {
			filename = filename.wrapping_byte_add(1);
			reloadname = filename;
			reloadlump = numlumps;
		}
		let handle = open(filename, O_RDONLY /*| O_BINARY*/);
		if handle == -1 {
			println!(" couldn't open {}", CStr::from_ptr(filename).to_str().unwrap());
			return;
		}

		println!(" adding {}", CStr::from_ptr(filename).to_str().unwrap());
		let startlump = numlumps;

		let mut fileinfo;
		let mut singleinfo = filelump_t { filepos: 0, size: 0, name: [0; 8] };
		let mut lumps;

		if strcmpi(filename.wrapping_add((libc::strlen(filename)) - 3), c"wad".as_ptr()) != 0 {
			// single lump file
			fileinfo = &raw mut singleinfo;
			singleinfo.filepos = 0;
			singleinfo.size = filelength(handle);
			ExtractFileBase(filename, singleinfo.name.as_mut_ptr());
			numlumps += 1;
		} else {
			// WAD file
			let mut header = MaybeUninit::<wadinfo_t>::uninit();
			libc::read(handle, header.as_mut_ptr().cast(), size_of_val(&header));
			let header = header.assume_init();
			if libc::strncmp(header.identification.as_ptr(), c"IWAD".as_ptr(), 4) != 0 {
				// Homebrew levels?
				if libc::strncmp(header.identification.as_ptr(), c"PWAD".as_ptr(), 4) != 0 {
					I_Error(format_args!(
						"Wad file {} doesn't have IWAD or PWAD id\n",
						CStr::from_ptr(filename).to_str().unwrap(),
					));
				}

				// ???modifiedgame = true;
			}
			let length = header.numlumps * size_of::<filelump_t>();
			lumps = vec![filelump_t { filepos: 0, size: 0, name: [0; 8] }; length];
			fileinfo = lumps.as_mut_ptr();
			libc::lseek(handle, header.infotableofs, libc::SEEK_SET);
			libc::read(handle, fileinfo.cast(), length);
			numlumps += header.numlumps;
		}

		// Fill in lumpinfo
		lumpinfo = libc::realloc(lumpinfo.cast(), numlumps * size_of::<lumpinfo_t>()).cast();

		if lumpinfo.is_null() {
			I_Error("Couldn't realloc lumpinfo");
		}

		let mut lump_p = lumpinfo.wrapping_add(startlump);

		let storehandle = if reloadname.is_null() { handle } else { -1 };

		for _ in startlump..numlumps {
			(*lump_p).handle = storehandle;
			(*lump_p).position = (*fileinfo).filepos;
			(*lump_p).size = (*fileinfo).size;
			libc::strncpy((*lump_p).name.as_mut_ptr(), (*fileinfo).name.as_ptr(), 8);
			lump_p = lump_p.wrapping_add(1);
			fileinfo = fileinfo.wrapping_add(1);
		}

		if !reloadname.is_null() {
			libc::close(handle);
		}
	}
}

// W_Reload
// Flushes any of the reloadable lumps in memory
//  and reloads the directory.
pub(crate) fn W_Reload() {
	unsafe {
		if reloadname.is_null() {
			return;
		}

		let handle = open(reloadname, O_RDONLY /*| O_BINARY*/);
		if handle == -1 {
			I_Error(format_args!(
				"W_Reload: couldn't open {}",
				CStr::from_ptr(reloadname).to_str().unwrap(),
			));
		}

		let mut header = MaybeUninit::<wadinfo_t>::uninit();
		libc::read(handle, header.as_mut_ptr().cast(), size_of_val(&header));
		let header = header.assume_init();
		let lumpcount = header.numlumps;
		let length = lumpcount * size_of::<filelump_t>();
		let mut fileinfo = vec![filelump_t { filepos: 0, size: 0, name: [0; 8] }; length];
		let mut fileinfo = fileinfo.as_mut_ptr();
		libc::lseek(handle, header.infotableofs, SEEK_SET);
		libc::read(handle, fileinfo.cast(), length);

		// Fill in lumpinfo
		let mut lump_p = lumpinfo.wrapping_add(reloadlump);

		for i in reloadlump..reloadlump + lumpcount {
			if !lumpcache.wrapping_add(i).is_null() {
				Z_Free(lumpcache.wrapping_add(i).cast());
			}

			(*lump_p).position = (*fileinfo).filepos;
			(*lump_p).size = (*fileinfo).size;
			lump_p = lump_p.wrapping_add(1);
			fileinfo = fileinfo.wrapping_add(1)
		}

		libc::close(handle);
	}
}

// W_InitMultipleFiles
// Pass a null terminated list of files to use.
// All files are optional, but at least one file
//  must be found.
// Files with a .wad extension are idlink files
//  with multiple lumps.
// Other files are single lumps with the base filename
//  for the lump name.
// Lump names can appear multiple times.
// The name searcher looks backwards, so a later file
//  does override all earlier ones.
pub(crate) fn W_InitMultipleFiles(mut filenames: *const *const c_char) {
	unsafe {
		// open all the files, load headers, and count lumps
		numlumps = 0;

		// will be realloced as lumps are added
		lumpinfo = libc::malloc(1).cast();

		while !(*filenames).is_null() {
			W_AddFile(*filenames);
			filenames = filenames.wrapping_add(1);
		}

		if numlumps == 0 {
			I_Error("W_InitFiles: no files found");
		}

		// set up caching
		let size = numlumps * size_of::<*mut c_void>();
		lumpcache = libc::malloc(size).cast();

		if lumpcache.is_null() {
			I_Error("Couldn't allocate lumpcache");
		}

		ptr::write_bytes(lumpcache, 0, numlumps);
	}
}

// W_CheckNumForName
// Returns -1 if name not found.
pub(crate) fn W_CheckNumForName(name: &CStr) -> Option<usize> {
	unsafe {
		let mut name8 = [0; 9];

		// make the name into two integers for easy compares
		libc::strncpy(name8.as_mut_ptr(), name.as_ptr(), 8);

		// in case the name was a fill 8 chars
		name8[8] = 0;

		// case insensitive
		strupr(name8.as_mut_ptr());

		// scan backwards so patch lump files take precedence
		let mut lump_p = lumpinfo.wrapping_add(numlumps);

		while lump_p != lumpinfo {
			lump_p = lump_p.wrapping_sub(1);
			if (&*lump_p).name[..] == name8[..8] {
				return Some(lump_p.offset_from_unsigned(lumpinfo));
			}
		}

		// TFB. Not found.
		None
	}
}

// W_GetNumForName
// Calls W_CheckNumForName, but bombs out if not found.
pub(crate) fn W_GetNumForName(name: &CStr) -> usize {
	let Some(num) = W_CheckNumForName(name) else {
		I_Error(format_args!("W_GetNumForName: {} not found!", name.to_str().unwrap()));
	};
	num
}

// W_LumpLength
// Returns the buffer size needed to load the given lump.
pub(crate) fn W_LumpLength(lump: usize) -> usize {
	unsafe {
		if lump >= numlumps {
			I_Error(format_args!("W_LumpLength: {} >= numlumps", lump));
		}

		(*lumpinfo.wrapping_add(lump)).size
	}
}

// W_ReadLump
// Loads the lump into the given buffer,
//  which must be >= W_LumpLength().
pub(crate) unsafe fn W_ReadLump(lump: usize, dest: *mut c_void) {
	unsafe {
		if lump >= numlumps {
			I_Error(format_args!("W_ReadLump: {} >= numlumps", lump));
		}

		let l = lumpinfo.wrapping_add(lump);

		// ??? I_BeginRead ();
		let handle;

		if (*l).handle == -1 {
			// reloadable file, so use open / read / close
			handle = open(reloadname, O_RDONLY /*| O_BINARY*/);
			if handle == -1 {
				I_Error(format_args!(
					"W_ReadLump: couldn't open {}",
					CStr::from_ptr(reloadname).to_str().unwrap(),
				));
			}
		} else {
			handle = (*l).handle;
		}

		libc::lseek(handle, (*l).position, SEEK_SET);
		let c = libc::read(handle, dest, (*l).size);

		if c < isize::try_from((*l).size).unwrap() {
			I_Error(format_args!("W_ReadLump: only read {} of {} on lump {}", c, (*l).size, lump));
		}

		if (*l).handle == -1 {
			libc::close(handle);
		}

		// ??? I_EndRead ();
	}
}

// W_CacheLumpNum
pub(crate) fn W_CacheLumpNum(lump: usize, tag: usize) -> *mut c_void {
	unsafe {
		if lump >= numlumps {
			I_Error(format_args!("W_CacheLumpNum: {} >= numlumps", lump));
		}

		let lump_p = lumpcache.wrapping_add(lump);
		if (*lump_p).is_null() {
			// read the lump in

			//printf ("cache miss on lump %i\n",lump);
			// FIXME unused???
			let _ptr = Z_Malloc(W_LumpLength(lump), tag, lump_p.cast());
			W_ReadLump(lump, *lump_p);
		} else {
			//printf ("cache hit on lump %i\n",lump);
			Z_ChangeTag!(*lump_p, tag);
		}

		*lump_p
	}
}

// W_CacheLumpName
pub(crate) unsafe fn W_CacheLumpName(name: &CStr, tag: usize) -> *mut c_void {
	W_CacheLumpNum(W_GetNumForName(name), tag)
}
