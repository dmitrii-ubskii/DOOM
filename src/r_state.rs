#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

// Refresh internal data structures,
//  for rendering.

use crate::r_defs::{node_t, sector_t, seg_t, subsector_t};

unsafe extern "C" {
	pub static mut segs: *mut seg_t;

	pub static mut numsectors: i32;
	pub static mut sectors: *mut sector_t;

	pub static mut numsubsectors: i32;
	pub static mut subsectors: *mut subsector_t;

	pub static mut numnodes: i32;
	pub static mut nodes: *mut node_t;
}
