#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

// DESCRIPTION:
//	Lookup tables.
//	Do not try to look them up :-).
//	In the order of appearance:
//
//	int finetangent[4096]	- Tangens LUT.
//	 Should work with BAM fairly well (12 of 16bit,
//      effectively, by shifting).
//
//	int finesine[10240]		- Sine lookup.
//	 Guess what, serves as cosine, too.
//	 Remarkable thing is, how to use BAMs with this?
//
//	int tantoangle[2049]	- ArcTan LUT,
//	  maps tan(angle) to angle fast. Gotta search.
//
//-----------------------------------------------------------------------------

use crate::m_fixed::{FRACBITS, fixed_t};

pub const FINEANGLES: usize = 8192;
pub const FINEMASK: usize = FINEANGLES - 1;

// 0x100000000 to 0x2000
pub const ANGLETOFINESHIFT: usize = 19;

unsafe extern "C" {
	// Effective size is 10240.
	pub static finesine: [fixed_t; 5 * FINEANGLES / 4];

	// Re-use data, is just PI/2 pahse shift.
	pub static finecosine: [fixed_t; 5 * FINEANGLES / 4];

	// Effective size is 4096.
	pub static finetangent: [fixed_t; FINEANGLES / 2];
}

// Binary Angle Measument, BAM.
pub const ANG45: u32 = 0x20000000;
pub const ANG90: u32 = 0x40000000;
pub const ANG180: u32 = 0x80000000;
pub const ANG270: u32 = 0xc0000000;

pub const SLOPERANGE: usize = 2048;
pub const SLOPEBITS: i32 = 11;
pub const DBITS: i32 = FRACBITS - SLOPEBITS;

pub type angle_t = u32;

/*

// Effective size is 2049;
// The +1 size is to handle the case when x==y
//  without additional checking.
extern angle_t		tantoangle[SLOPERANGE+1];

*/
