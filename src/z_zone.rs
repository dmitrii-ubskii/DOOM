#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::{ffi::c_void, io::Write, os::fd::FromRawFd, ptr::null_mut};

use libc::{FILE, fileno};

use crate::i_system::{I_Error, I_ZoneBase};

// ZONE MEMORY
// PU - purge tags.
// Tags < 100 are not overwritten until freed.
pub const PU_STATIC: usize = 1; // static entire execution time
pub const PU_SOUND: usize = 2; // static while playing
pub const PU_MUSIC: usize = 3; // static while playing
pub const PU_DAVE: usize = 4; // anything else Dave wants static
pub const PU_LEVEL: usize = 50; // static until level exited
pub const PU_LEVSPEC: usize = 51; // a special thinker in a level
// Tags >= 100 are purgable whenever needed.
pub const PU_PURGELEVEL: usize = 100;
pub const PU_CACHE: usize = 101;

#[repr(C)]
pub struct memblock_t {
	pub size: usize,            // including the header and possibly tiny fragments
	pub user: *mut *mut c_void, // NULL if a free block
	pub tag: usize,             // purgelevel
	pub id: usize,              // should be ZONEID
	pub next: *mut memblock_t,
	pub prev: *mut memblock_t,
}

// This is used to get the local FILE:LINE info from CPP
// prior to really call the function in question.
macro_rules! Z_ChangeTag {
	($p:expr, $t: expr) => {
		let block = $p.wrapping_byte_sub(size_of::<$crate::z_zone::memblock_t>())
			as *mut $crate::z_zone::memblock_t;
		if (*block).id != 0x1d4a11 {
			I_Error(concat!("Z_CT at ", file!(), ":%i", line!(), "\0").as_ptr() as *const i8);
		}
		crate::z_zone::Z_ChangeTag2($p, $t);
	};
}
pub(crate) use Z_ChangeTag;

// ZONE MEMORY ALLOCATION
//
// There is never any space between memblocks,
//  and there will never be two contiguous free memblocks.
// The rover can be left pointing at a non-empty block.
//
// It is of no value to free a cachable block,
//  because it will get overwritten automatically if needed.

const ZONEID: usize = 0x1d4a11;

#[repr(C)]
pub struct memzone_t {
	// total bytes malloced, including header
	pub size: usize,

	// start / end cap for linked list
	pub blocklist: memblock_t,

	pub rover: *mut memblock_t,
}

static mut mainzone: *mut memzone_t = null_mut();

// Z_ClearZone
fn Z_ClearZone(zone: *mut memzone_t) {
	unsafe {
		let block = zone.wrapping_byte_add(size_of::<memzone_t>()) as *mut memblock_t;

		let zone = &mut *zone;
		// set the entire zone to one free block
		zone.blocklist.next = block;
		zone.blocklist.prev = block;

		zone.blocklist.user = zone as *mut memzone_t as *mut *mut c_void;
		zone.blocklist.tag = PU_STATIC;
		zone.rover = block;

		let block = &mut *block;

		block.prev = &raw mut zone.blocklist;
		block.next = &raw mut zone.blocklist;

		// NULL indicates a free block.
		block.user = null_mut();

		block.size = zone.size - size_of::<memzone_t>();
	}
}

// Z_Init
pub(crate) fn Z_Init() {
	unsafe {
		let mut size = 0;

		mainzone = I_ZoneBase(&mut size) as *mut memzone_t;
		(*mainzone).size = size;

		Z_ClearZone(mainzone);
	}
}

// Z_Free
#[unsafe(no_mangle)]
pub extern "C" fn Z_Free(ptr: *mut c_void) {
	unsafe {
		let mut block = &mut *(ptr.wrapping_byte_sub(size_of::<memblock_t>()) as *mut memblock_t);

		if block.id != ZONEID {
			I_Error(c"Z_Free: freed a pointer without ZONEID".as_ptr());
		}

		if block.user as usize > 0x100 {
			// smaller values are not pointers
			// Note: OS-dependend?

			// clear the user's mark
			*block.user = null_mut();
		}

		// mark as free
		block.user = null_mut();
		block.tag = 0;
		block.id = 0;

		let other = &mut *block.prev;

		if other.user.is_null() {
			// merge with previous free block
			other.size += block.size;
			other.next = block.next;
			(*other.next).prev = other;

			if std::ptr::eq(block, (*mainzone).rover) {
				(*mainzone).rover = other;
			}

			block = other;
		}

		let other = &mut *block.next;
		if other.user.is_null() {
			// merge the next free block onto the end
			block.size += other.size;
			block.next = other.next;
			(*block.next).prev = block;

			if std::ptr::eq(other, (*mainzone).rover) {
				(*mainzone).rover = block;
			}
		}
	}
}

// Z_Malloc
// You can pass a NULL user if the tag is < PU_PURGELEVEL.
const MINFRAGMENT: usize = 64;

#[unsafe(no_mangle)]
pub extern "C" fn Z_Malloc(size: usize, tag: usize, user: *mut c_void) -> *mut c_void {
	unsafe {
		// int		extra;
		// memblock_t*	start;
		// memblock_t* rover;
		// memblock_t* newblock;
		// memblock_t*	base;

		let size = (size + 3) & !3;

		// scan through the block list,
		// looking for the first free block
		// of sufficient size,
		// throwing out any purgable blocks along the way.

		// account for size of block header
		let size = size + size_of::<memblock_t>();

		// if there is a free block behind the rover,
		//  back up over them
		let mut base = (*mainzone).rover;

		if (*(*base).prev).user.is_null() {
			base = (*base).prev;
		}

		let mut rover = base;
		let start = (*base).prev;

		loop {
			if std::ptr::eq(rover, start) {
				// scanned all the way around the list
				I_Error(c"Z_Malloc: failed on allocation of %i bytes".as_ptr(), size);
			}

			if !(*rover).user.is_null() {
				if (*rover).tag < PU_PURGELEVEL {
					// hit a block that can't be purged,
					//  so move base past it
					base = (*rover).next;
					rover = (*rover).next;
				} else {
					// free the rover block (adding the size to base)

					// the rover can be the base block
					base = (*base).prev;
					Z_Free((rover as *mut c_void).wrapping_byte_add(size_of::<memblock_t>()));
					base = (*base).next;
					rover = (*base).next;
				}
			} else {
				rover = (*rover).next;
			}
			if (*base).user.is_null() && (*base).size >= size {
				break;
			}
		}

		// found a block big enough
		let extra = (*base).size - size;

		if extra > MINFRAGMENT {
			// there will be a free fragment after the allocated block
			let newblock = &mut *base.wrapping_byte_add(size);
			newblock.size = extra;

			// NULL indicates free block.
			newblock.user = null_mut();
			newblock.tag = 0;
			newblock.prev = base;
			newblock.next = (*base).next;
			(*newblock.next).prev = newblock;

			(*base).next = newblock;
			(*base).size = size;
		}

		if !user.is_null() {
			// mark as an in use block
			(*base).user = user as *mut *mut c_void;
			*(user as *mut *mut c_void) =
				(base.wrapping_byte_add(size_of::<memblock_t>())) as *mut c_void;
		} else {
			if tag >= PU_PURGELEVEL {
				I_Error(c"Z_Malloc: an owner is required for purgable blocks".as_ptr());
			}

			// mark as in use, but unowned
			(*base).user = 2 as *mut *mut c_void;
		}
		(*base).tag = tag;

		// next allocation will start looking here
		(*mainzone).rover = (*base).next;

		(*base).id = ZONEID;

		(base.wrapping_byte_add(size_of::<memblock_t>())) as *mut c_void
	}
}

// Z_FreeTags
#[unsafe(no_mangle)]
pub extern "C" fn Z_FreeTags(lowtag: usize, hightag: usize) {
	unsafe {
		let mut block = (*mainzone).blocklist.next;
		while !std::ptr::eq(block, (*mainzone).blocklist.next) {
			// get link before freeing
			let next = (*block).next;

			// free block?
			if !(*block).user.is_null() && lowtag <= (*block).tag && (*block).tag <= hightag {
				Z_Free(block.wrapping_byte_add(size_of::<memblock_t>()) as *mut c_void);
			}

			block = next;
		}
	}
}

// Z_FileDumpHeap
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Z_FileDumpHeap(f: *mut FILE) {
	unsafe {
		let mut f = std::fs::File::from_raw_fd(fileno(f));

		#[allow(static_mut_refs)]
		{
			writeln!(f, "zone size: {}  location: {:p}", (*mainzone).size, mainzone).unwrap();
		}

		let mut block = (*mainzone).blocklist.next;
		loop {
			writeln!(
				f,
				"block:{:p}	size:{:7}	user:{:p}	tag:{:3}",
				block,
				(*block).size,
				(*block).user,
				(*block).tag
			)
			.unwrap();

			if std::ptr::eq((*block).next, &(*mainzone).blocklist) {
				// all blocks have been hit
				break;
			}

			if !std::ptr::eq(block.wrapping_byte_add((*block).size), (*block).next) {
				writeln!(f, "ERROR: block size does not touch the next block").unwrap();
			}

			if !std::ptr::eq((*(*block).next).prev, block) {
				writeln!(f, "ERROR: next block doesn't have proper back link").unwrap();
			}

			if (*block).user.is_null() && (*(*block).next).user.is_null() {
				writeln!(f, "ERROR: two consecutive free blocks").unwrap();
			}

			block = (*block).next;
		}
	}
}

// Z_CheckHeap
#[unsafe(no_mangle)]
pub extern "C" fn Z_CheckHeap() {
	unsafe {
		let mut block = (*mainzone).blocklist.next;
		loop {
			if std::ptr::eq((*block).next, &(*mainzone).blocklist) {
				// all blocks have been hit
				break;
			}

			if !std::ptr::eq(block.wrapping_byte_add((*block).size), (*block).next) {
				I_Error(c"Z_CheckHeap: block size does not touch the next block\n".as_ptr());
			}

			if !std::ptr::eq((*(*block).next).prev, block) {
				I_Error(c"Z_CheckHeap: next block doesn't have proper back link\n".as_ptr());
			}

			if (*block).user.is_null() && (*(*block).next).user.is_null() {
				I_Error(c"Z_CheckHeap: two consecutive free blocks\n".as_ptr());
			}
			block = (*block).next;
		}
	}
}

// Z_ChangeTag
#[unsafe(no_mangle)]
pub extern "C" fn Z_ChangeTag2(ptr: *mut c_void, tag: usize) {
	unsafe {
		let block = ptr.wrapping_byte_sub(size_of::<memblock_t>()) as *mut memblock_t;

		if (*block).id != ZONEID {
			I_Error(c"Z_ChangeTag: freed a pointer without ZONEID".as_ptr());
		}

		if tag >= PU_PURGELEVEL && ((*block).user as usize) < 0x100 {
			I_Error(c"Z_ChangeTag: an owner is required for purgable blocks".as_ptr());
		}

		(*block).tag = tag;
	}
}

// Z_FreeMemory
#[unsafe(no_mangle)]
pub extern "C" fn Z_FreeMemory() -> usize {
	unsafe {
		let mut free = 0;

		let mut block = (*mainzone).blocklist.next;
		loop {
			if std::ptr::eq((*block).next, &(*mainzone).blocklist) {
				break;
			}
			if (*block).user.is_null() || (*block).tag >= PU_PURGELEVEL {
				free += (*block).size;
			}
			block = (*block).next;
		}
		free
	}
}
