#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::{
	ffi::{CStr, c_char, c_void},
	mem::MaybeUninit,
	ptr::null_mut,
};

use libc::{O_RDONLY, SEEK_SET, memset, open};

use crate::{
	i_system::I_Error,
	z_zone::{Z_ChangeTag, Z_Free, Z_Malloc},
};

type int = i32;

// TYPES
#[repr(C)]
pub struct wadinfo_t {
	// Should be "IWAD" or "PWAD".
	pub identification: [c_char; 4],
	pub numlumps: usize,
	pub infotableofs: int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct filelump_t {
	pub filepos: int,
	pub size: int,
	pub name: [c_char; 8],
}

// WADFILE I/O related stuff.
#[repr(C)]
pub struct lumpinfo_t {
	pub name: [c_char; 8],
	pub handle: int,
	pub position: int,
	pub size: i32,
}

// GLOBALS

// Location of each lump on disk.
#[unsafe(no_mangle)]
pub static mut lumpinfo: *mut lumpinfo_t = null_mut();
pub(crate) static mut numlumps: usize = 0;

static mut lumpcache: *mut *mut c_void = null_mut();

// #define strcmpi	strcasecmp
const strcmpi: unsafe extern "C" fn(*const i8, *const i8) -> i32 = libc::strcasecmp;

fn toupper(c: c_char) -> i8 {
	(c as u8 as char).to_ascii_uppercase() as c_char
}

fn strupr(mut s: *mut c_char) {
	unsafe {
		while *s != 0 {
			*s = toupper(*s);
			s = s.wrapping_byte_add(1);
		}
	}
}

fn filelength(handle: i32) -> i32 {
	unsafe {
		let mut fileinfo = MaybeUninit::uninit();

		if libc::fstat(handle, fileinfo.as_mut_ptr()) == -1 {
			I_Error(c"Error fstating".as_ptr());
		}

		fileinfo.assume_init().st_size
	}
}

fn ExtractFileBase(path: *const c_char, mut dest: *mut c_char) {
	unsafe {
		let mut src = path.wrapping_byte_add(libc::strlen(path) - 1);

		// back up until a \ or the start
		while src != path
			&& *(src.wrapping_sub(1)) != b'\\' as i8
			&& *(src.wrapping_sub(1)) != b'/' as i8
		{
			src = src.wrapping_byte_offset(-1);
		}

		// copy up to eight characters
		memset(dest.cast(), 0, 8);
		let mut length = 0;

		while *src != 0 && *src != b'.' as i8 {
			length += 1;
			if length == 9 {
				I_Error(c"Filename base of %s >8 chars".as_ptr(), path);
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
		// wadinfo_t		header;
		// lumpinfo_t*		lump_p;
		// unsigned		i;
		// int			handle;
		// int			length;
		// int			startlump;
		// filelump_t*		fileinfo;
		// filelump_t		singleinfo;
		// int			storehandle;

		// open the file and add to directory

		// handle reload indicator.
		if *filename == b'~' as c_char {
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
					I_Error(c"Wad file %s doesn't have IWAD or PWAD id\n".as_ptr(), filename);
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
			I_Error(c"Couldn't realloc lumpinfo".as_ptr());
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
			I_Error(c"W_Reload: couldn't open %s".as_ptr(), reloadname);
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
			I_Error(c"W_InitFiles: no files found".as_ptr());
		}

		// set up caching
		let size = numlumps * size_of::<*mut c_void>();
		lumpcache = libc::malloc(size) as *mut *mut c_void;

		if lumpcache.is_null() {
			I_Error(c"Couldn't allocate lumpcache".as_ptr());
		}

		memset(lumpcache.cast(), 0, size);
	}
}

// W_CheckNumForName
// Returns -1 if name not found.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn W_CheckNumForName(name: *const c_char) -> isize {
	unsafe {
		let mut name8 = [0; 9];

		// make the name into two integers for easy compares
		libc::strncpy(name8.as_mut_ptr(), name, 8);

		// in case the name was a fill 8 chars
		name8[8] = 0;

		// case insensitive
		strupr(name8.as_mut_ptr());

		// scan backwards so patch lump files take precedence
		let mut lump_p = lumpinfo.wrapping_add(numlumps);

		while lump_p != lumpinfo {
			lump_p = lump_p.wrapping_sub(1);
			if (*lump_p).name[..] == name8[..8] {
				return lump_p.offset_from(lumpinfo);
			}
		}

		// TFB. Not found.
		-1
	}
}

// W_GetNumForName
// Calls W_CheckNumForName, but bombs out if not found.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn W_GetNumForName(name: *const c_char) -> isize {
	unsafe {
		match W_CheckNumForName(name) {
			-1 => I_Error(c"W_GetNumForName: %s not found!".as_ptr(), name),
			i => i,
		}
	}
}

// W_LumpLength
// Returns the buffer size needed to load the given lump.
#[unsafe(no_mangle)]
pub extern "C" fn W_LumpLength(lump: usize) -> usize {
	unsafe {
		if lump >= numlumps {
			I_Error(c"W_LumpLength: %i >= numlumps".as_ptr(), lump);
		}

		(*lumpinfo.wrapping_add(lump)).size as usize
	}
}

// W_ReadLump
// Loads the lump into the given buffer,
//  which must be >= W_LumpLength().
#[unsafe(no_mangle)]
pub unsafe extern "C" fn W_ReadLump(lump: usize, dest: *mut c_void) {
	unsafe {
		if lump >= numlumps {
			I_Error(c"W_ReadLump: %i >= numlumps".as_ptr(), lump);
		}

		let l = lumpinfo.wrapping_add(lump);

		// ??? I_BeginRead ();
		let handle;

		if (*l).handle == -1 {
			// reloadable file, so use open / read / close
			handle = open(reloadname, O_RDONLY /*| O_BINARY*/);
			if handle == -1 {
				I_Error(c"W_ReadLump: couldn't open %s".as_ptr(), reloadname);
			}
		} else {
			handle = (*l).handle;
		}

		libc::lseek(handle, (*l).position, SEEK_SET);
		let c = libc::read(handle, dest, (*l).size as usize);

		if c < (*l).size as isize {
			I_Error(c"W_ReadLump: only read %i of %i on lump %i".as_ptr(), c, (*l).size, lump);
		}

		if (*l).handle == -1 {
			libc::close(handle);
		}

		// ??? I_EndRead ();
	}
}

// W_CacheLumpNum
#[unsafe(no_mangle)]
pub extern "C" fn W_CacheLumpNum(lump: usize, tag: usize) -> *mut c_void {
	unsafe {
		if lump >= numlumps {
			I_Error(c"W_CacheLumpNum: %i >= numlumps".as_ptr(), lump);
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn W_CacheLumpName(name: *const c_char, tag: usize) -> *mut c_void {
	unsafe { W_CacheLumpNum(W_GetNumForName(name) as usize, tag) }
}
