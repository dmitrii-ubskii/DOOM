#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::{
	alloc::{Layout, alloc},
	ffi::{CStr, c_void},
	fs::File,
	io::{Read, Seek, SeekFrom},
	mem::{self, MaybeUninit},
	os::{
		fd::{FromRawFd, IntoRawFd, RawFd},
		unix::ffi::OsStrExt,
	},
	path::Path,
	ptr::{self, null_mut},
	slice,
};

use crate::{
	i_system::I_Error,
	z_zone::{Z_ChangeTag, Z_Free, Z_Malloc},
};

// TYPES
#[repr(C)]
pub(crate) struct wadinfo_t {
	// Should be "IWAD" or "PWAD".
	pub(crate) identification: [u8; 4],
	pub(crate) numlumps: usize,
	pub(crate) infotableofs: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct filelump_t {
	pub(crate) filepos: u32,
	pub(crate) size: usize,
	pub(crate) name: [u8; 8],
}

// WADFILE I/O related stuff.
#[repr(C)]
pub(crate) struct lumpinfo_t {
	pub(crate) name: [u8; 8],
	pub(crate) handle: Option<RawFd>,
	pub(crate) position: u32,
	pub(crate) size: usize,
}

// GLOBALS

// Location of each lump on disk.
pub(crate) static mut lumpinfo: *mut lumpinfo_t = null_mut();
pub(crate) static mut numlumps: usize = 0;

static mut lumpcache: *mut *mut c_void = null_mut();

fn filelength(handle: &File) -> usize {
	let Ok(metadata) = handle.metadata() else {
		I_Error("Error fstating");
	};

	usize::try_from(metadata.len()).unwrap()
}

fn ExtractFileBase(path: &Path, dest: &mut [u8; 8]) {
	*dest = [0; 8];

	let Some(stem) = path.file_stem() else { return };
	for (index, &c) in stem.as_bytes().iter().enumerate() {
		if c == b'.' {
			break;
		}

		if index >= 8 {
			I_Error(format_args!("Filename base of {} >8 chars", path.display()));
		}

		dest[index] = c.to_ascii_uppercase();
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
static mut reloadname: Option<&str> = None;

#[allow(static_mut_refs)]
fn W_AddFile(mut filename: &'static str) {
	unsafe {
		// open the file and add to directory

		// handle reload indicator.
		if filename.starts_with('~') {
			filename = &filename[1..];
			reloadname = Some(filename);
			reloadlump = numlumps;
		}

		let Ok(mut handle) = File::open(filename) else {
			println!(" couldn't open {}", filename);
			return;
		};

		println!(" adding {}", filename);
		let startlump = numlumps;

		let mut fileinfo;
		let mut singleinfo = filelump_t { filepos: 0, size: 0, name: [0; 8] };
		let mut lumps;

		if !filename[filename.len() - 3..].eq_ignore_ascii_case("wad") {
			// single lump file
			fileinfo = &raw mut singleinfo;
			singleinfo.filepos = 0;
			singleinfo.size = filelength(&handle);
			ExtractFileBase(Path::new(filename), &mut singleinfo.name);
			numlumps += 1;
		} else {
			// WAD file
			let mut header = MaybeUninit::<wadinfo_t>::uninit();
			handle
				.read_exact(slice::from_raw_parts_mut(
					header.as_mut_ptr().cast(),
					size_of_val(&header),
				))
				.unwrap();
			let header = header.assume_init();
			if header.identification != *b"IWAD" {
				// Homebrew levels?
				if header.identification != *b"PWAD" {
					I_Error(format_args!("Wad file {} doesn't have IWAD or PWAD id\n", filename));
				}

				// ???modifiedgame = true;
			}
			let length = header.numlumps * size_of::<filelump_t>();
			lumps = vec![filelump_t { filepos: 0, size: 0, name: [0; 8] }; length];
			fileinfo = lumps.as_mut_ptr();
			handle.seek(SeekFrom::Start(u64::from(header.infotableofs))).unwrap();
			handle.read_exact(slice::from_raw_parts_mut(fileinfo.cast(), length)).unwrap();
			numlumps += header.numlumps;
		}

		// Fill in lumpinfo
		lumpinfo = alloc(Layout::array::<lumpinfo_t>(numlumps).unwrap()).cast();

		if lumpinfo.is_null() {
			I_Error("Couldn't realloc lumpinfo");
		}

		let mut lump_p = lumpinfo.wrapping_add(startlump);

		let storehandle = if reloadname.is_none() { Some(handle.into_raw_fd()) } else { None };

		for _ in startlump..numlumps {
			(*lump_p).handle = storehandle;
			(*lump_p).position = (*fileinfo).filepos;
			(*lump_p).size = (*fileinfo).size;
			(*lump_p).name = (*fileinfo).name;
			lump_p = lump_p.wrapping_add(1);
			fileinfo = fileinfo.wrapping_add(1);
		}
	}
}

// W_Reload
// Flushes any of the reloadable lumps in memory
//  and reloads the directory.
pub(crate) fn W_Reload() {
	unsafe {
		let Some(reloadname_) = reloadname else {
			return;
		};

		let Ok(mut handle) = File::open(reloadname_) else {
			I_Error(format_args!("W_Reload: couldn't open {}", reloadname_));
		};

		let mut header = MaybeUninit::<wadinfo_t>::uninit();
		handle
			.read_exact(slice::from_raw_parts_mut(header.as_mut_ptr().cast(), size_of_val(&header)))
			.unwrap();
		let header = header.assume_init();
		let lumpcount = header.numlumps;
		let length = lumpcount * size_of::<filelump_t>();
		let mut fileinfo = vec![filelump_t { filepos: 0, size: 0, name: [0; 8] }; length];
		let mut fileinfo = fileinfo.as_mut_ptr();
		handle.seek(SeekFrom::Start(u64::from(header.infotableofs))).unwrap();
		handle.read_exact(slice::from_raw_parts_mut(fileinfo.cast(), length)).unwrap();

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
pub(crate) fn W_InitMultipleFiles(mut filenames: *const &'static str) {
	unsafe {
		// open all the files, load headers, and count lumps
		numlumps = 0;

		// will be realloced as lumps are added
		lumpinfo = null_mut();

		while !(&*filenames).is_empty() {
			W_AddFile(*filenames);
			filenames = filenames.wrapping_add(1);
		}

		if numlumps == 0 {
			I_Error("W_InitFiles: no files found");
		}

		// set up caching
		lumpcache = alloc(Layout::array::<*mut c_void>(numlumps).unwrap()).cast();

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

		for (i, &c) in name.to_bytes().iter().enumerate().take(8) {
			// make the name into two integers for easy compares
			name8[i] = c.to_ascii_uppercase();
		}

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
pub(crate) unsafe fn W_ReadLump(lump: usize, dest: *mut u8) {
	unsafe {
		if lump >= numlumps {
			I_Error(format_args!("W_ReadLump: {} >= numlumps", lump));
		}

		let l = lumpinfo.wrapping_add(lump);

		// ??? I_BeginRead ();
		let mut handle = if let Some(handle) = (*l).handle {
			File::from_raw_fd(handle)
		} else {
			// reloadable file, so use open / read / close
			File::open(reloadname.unwrap()).unwrap_or_else(|_| {
				I_Error(format_args!("W_ReadLump: couldn't open {}", reloadname.unwrap()));
			})
		};

		handle.seek(SeekFrom::Start(u64::from((*l).position))).unwrap();
		let dest = slice::from_raw_parts_mut(dest, (*l).size);
		let c = handle.read(dest).unwrap();

		if c < (*l).size {
			I_Error(format_args!("W_ReadLump: only read {} of {} on lump {}", c, (*l).size, lump));
		}

		if (*l).handle.is_some() {
			// don't close a shared handle
			mem::forget(handle);
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
			W_ReadLump(lump, (*lump_p).cast());
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
