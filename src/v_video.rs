#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::ptr::null_mut;

use libc::memcpy;

use crate::{
	doomdef::{SCREENHEIGHT, SCREENWIDTH},
	i_system::{I_AllocLow, I_Error},
	m_bbox::M_AddToBox,
	m_fixed::fixed_t,
	r_defs::{column_t, patch_t},
};

#[unsafe(no_mangle)]
pub static mut screens: [*mut u8; 5] = [null_mut(); 5];

#[unsafe(no_mangle)]
pub static mut dirtybox: [i32; 4] = [0; 4];

// Now where did these came from?
#[unsafe(no_mangle)]
pub static gammatable: [[u8; 256]; 5] = [
	[
		1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
		26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
		49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71,
		72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94,
		95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113,
		114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 128, 129, 130,
		131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148,
		149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166,
		167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184,
		185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202,
		203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220,
		221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238,
		239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255,
	],
	[
		2, 4, 5, 7, 8, 10, 11, 12, 14, 15, 16, 18, 19, 20, 21, 23, 24, 25, 26, 27, 29, 30, 31, 32,
		33, 34, 36, 37, 38, 39, 40, 41, 42, 44, 45, 46, 47, 48, 49, 50, 51, 52, 54, 55, 56, 57, 58,
		59, 60, 61, 62, 63, 64, 65, 66, 67, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82,
		83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103,
		104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121,
		122, 123, 124, 125, 126, 127, 128, 129, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138,
		139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 148, 149, 150, 151, 152, 153, 154, 155,
		156, 157, 158, 159, 160, 161, 162, 163, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172,
		173, 174, 175, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 186, 187, 188,
		189, 190, 191, 192, 193, 194, 195, 196, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205,
		205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 214, 215, 216, 217, 218, 219, 220, 221,
		222, 222, 223, 224, 225, 226, 227, 228, 229, 230, 230, 231, 232, 233, 234, 235, 236, 237,
		237, 238, 239, 240, 241, 242, 243, 244, 245, 245, 246, 247, 248, 249, 250, 251, 252, 252,
		253, 254, 255,
	],
	[
		4, 7, 9, 11, 13, 15, 17, 19, 21, 22, 24, 26, 27, 29, 30, 32, 33, 35, 36, 38, 39, 40, 42,
		43, 45, 46, 47, 48, 50, 51, 52, 54, 55, 56, 57, 59, 60, 61, 62, 63, 65, 66, 67, 68, 69, 70,
		72, 73, 74, 75, 76, 77, 78, 79, 80, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95,
		96, 97, 98, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 114,
		115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132,
		133, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 144, 145, 146, 147, 148,
		149, 150, 151, 152, 153, 153, 154, 155, 156, 157, 158, 159, 160, 160, 161, 162, 163, 164,
		165, 166, 166, 167, 168, 169, 170, 171, 172, 172, 173, 174, 175, 176, 177, 178, 178, 179,
		180, 181, 182, 183, 183, 184, 185, 186, 187, 188, 188, 189, 190, 191, 192, 193, 193, 194,
		195, 196, 197, 197, 198, 199, 200, 201, 201, 202, 203, 204, 205, 206, 206, 207, 208, 209,
		210, 210, 211, 212, 213, 213, 214, 215, 216, 217, 217, 218, 219, 220, 221, 221, 222, 223,
		224, 224, 225, 226, 227, 228, 228, 229, 230, 231, 231, 232, 233, 234, 235, 235, 236, 237,
		238, 238, 239, 240, 241, 241, 242, 243, 244, 244, 245, 246, 247, 247, 248, 249, 250, 251,
		251, 252, 253, 254, 254, 255,
	],
	[
		8, 12, 16, 19, 22, 24, 27, 29, 31, 34, 36, 38, 40, 41, 43, 45, 47, 49, 50, 52, 53, 55, 57,
		58, 60, 61, 63, 64, 65, 67, 68, 70, 71, 72, 74, 75, 76, 77, 79, 80, 81, 82, 84, 85, 86, 87,
		88, 90, 91, 92, 93, 94, 95, 96, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109,
		110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127,
		128, 129, 130, 131, 132, 133, 134, 135, 135, 136, 137, 138, 139, 140, 141, 142, 143, 143,
		144, 145, 146, 147, 148, 149, 150, 150, 151, 152, 153, 154, 155, 155, 156, 157, 158, 159,
		160, 160, 161, 162, 163, 164, 165, 165, 166, 167, 168, 169, 169, 170, 171, 172, 173, 173,
		174, 175, 176, 176, 177, 178, 179, 180, 180, 181, 182, 183, 183, 184, 185, 186, 186, 187,
		188, 189, 189, 190, 191, 192, 192, 193, 194, 195, 195, 196, 197, 197, 198, 199, 200, 200,
		201, 202, 202, 203, 204, 205, 205, 206, 207, 207, 208, 209, 210, 210, 211, 212, 212, 213,
		214, 214, 215, 216, 216, 217, 218, 219, 219, 220, 221, 221, 222, 223, 223, 224, 225, 225,
		226, 227, 227, 228, 229, 229, 230, 231, 231, 232, 233, 233, 234, 235, 235, 236, 237, 237,
		238, 238, 239, 240, 240, 241, 242, 242, 243, 244, 244, 245, 246, 246, 247, 247, 248, 249,
		249, 250, 251, 251, 252, 253, 253, 254, 254, 255,
	],
	[
		16, 23, 28, 32, 36, 39, 42, 45, 48, 50, 53, 55, 57, 60, 62, 64, 66, 68, 69, 71, 73, 75, 76,
		78, 80, 81, 83, 84, 86, 87, 89, 90, 92, 93, 94, 96, 97, 98, 100, 101, 102, 103, 105, 106,
		107, 108, 109, 110, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125,
		126, 128, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143,
		143, 144, 145, 146, 147, 148, 149, 150, 150, 151, 152, 153, 154, 155, 155, 156, 157, 158,
		159, 159, 160, 161, 162, 163, 163, 164, 165, 166, 166, 167, 168, 169, 169, 170, 171, 172,
		172, 173, 174, 175, 175, 176, 177, 177, 178, 179, 180, 180, 181, 182, 182, 183, 184, 184,
		185, 186, 187, 187, 188, 189, 189, 190, 191, 191, 192, 193, 193, 194, 195, 195, 196, 196,
		197, 198, 198, 199, 200, 200, 201, 202, 202, 203, 203, 204, 205, 205, 206, 207, 207, 208,
		208, 209, 210, 210, 211, 211, 212, 213, 213, 214, 214, 215, 216, 216, 217, 217, 218, 219,
		219, 220, 220, 221, 221, 222, 223, 223, 224, 224, 225, 225, 226, 227, 227, 228, 228, 229,
		229, 230, 230, 231, 232, 232, 233, 233, 234, 234, 235, 235, 236, 236, 237, 237, 238, 239,
		239, 240, 240, 241, 241, 242, 242, 243, 243, 244, 244, 245, 245, 246, 246, 247, 247, 248,
		248, 249, 249, 250, 250, 251, 251, 252, 252, 253, 254, 254, 255, 255,
	],
];

#[unsafe(no_mangle)]
pub static mut usegamma: i32 = 0;

// V_MarkRect
#[unsafe(no_mangle)]
pub extern "C" fn V_MarkRect(x: usize, y: usize, width: usize, height: usize) {
	#[allow(static_mut_refs)]
	unsafe {
		M_AddToBox(&mut dirtybox, x as fixed_t, y as fixed_t);
		M_AddToBox(&mut dirtybox, (x + width - 1) as fixed_t, (y + height - 1) as fixed_t);
	}
}

// V_CopyRect
#[unsafe(no_mangle)]
pub extern "C" fn V_CopyRect(
	srcx: usize,
	srcy: usize,
	srcscrn: usize,
	width: usize,
	height: usize,
	destx: usize,
	desty: usize,
	destscrn: usize,
) {
	unsafe {
		// #ifdef RANGECHECK
		if srcx + width > SCREENWIDTH
			|| srcy + height > SCREENHEIGHT
			|| destx + width > SCREENWIDTH
			|| desty + height > SCREENHEIGHT
			|| srcscrn > 4
			|| destscrn > 4
		{
			I_Error(c"Bad V_CopyRect".as_ptr());
		}
		// #endif
		V_MarkRect(destx, desty, width, height);

		let mut src = screens[srcscrn].wrapping_byte_add(SCREENWIDTH * srcy + srcx);
		let mut dest = screens[destscrn].wrapping_byte_add(SCREENWIDTH * desty + destx);

		for _ in 0..height {
			memcpy(dest.cast(), src.cast(), width);
			src = src.wrapping_byte_add(SCREENWIDTH);
			dest = dest.wrapping_byte_add(SCREENWIDTH);
		}
	}
}

// V_DrawPatch
// Masks a column based masked pic to the screen.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn V_DrawPatch(
	mut x: usize,
	mut y: usize,
	scrn: usize,
	patch: *const patch_t,
) {
	unsafe {
		y = y.checked_add_signed(-(*patch).topoffset as isize).unwrap();
		x = x.checked_add_signed(-(*patch).leftoffset as isize).unwrap();
		// #ifdef RANGECHECK
		if x + (*patch).width as usize > SCREENWIDTH
			|| y + (*patch).height as usize > SCREENHEIGHT
			|| scrn > 4
		{
			eprintln!("Patch at {x},{y} exceeds LFB");
			// No I_Error abort - what is up with TNT.WAD?
			eprintln!("V_DrawPatch: bad patch (ignored)");
			return;
		}
		// #endif

		if scrn == 0 {
			V_MarkRect(x, y, (*patch).width as usize, (*patch).height as usize);
		}

		let mut col = 0;
		let mut desttop = screens[scrn].wrapping_byte_add(y * SCREENWIDTH + x);

		let w = (*patch).width as usize;
		while col < w {
			let count = *((*patch).columnofs.as_ptr()).wrapping_add(col);
			let mut column = patch.wrapping_byte_add(count) as *mut column_t;

			// step through the posts in a column
			while (*column).topdelta != 0xff {
				let mut source = (column as *mut u8).wrapping_byte_add(3);
				let mut dest = desttop.wrapping_byte_add((*column).topdelta as usize * SCREENWIDTH);
				let count = (*column).length;

				for _ in 0..count {
					*dest = *source;
					source = source.wrapping_byte_add(1);
					dest = dest.wrapping_byte_add(SCREENWIDTH);
				}
				column = column.wrapping_byte_add((*column).length as usize + 4);
			}
			x += 1;
			col += 1;
			desttop = desttop.wrapping_byte_add(1);
		}
	}
}

// V_DrawPatchFlipped
// Masks a column based masked pic to the screen.
// Flips horizontally, e.g. to mirror face.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn V_DrawPatchFlipped(
	mut x: usize,
	mut y: usize,
	scrn: usize,
	patch: *mut patch_t,
) {
	unsafe {
		y = y.checked_add_signed(-(*patch).topoffset as isize).unwrap();
		x = x.checked_add_signed(-(*patch).leftoffset as isize).unwrap();
		// #ifdef RANGECHECK
		if x + (*patch).width as usize > SCREENWIDTH
			|| y + (*patch).height as usize > SCREENHEIGHT
			|| scrn > 4
		{
			eprintln!("Patch origin {x},{y} exceeds LFB",);
			I_Error(c"Bad V_DrawPatch in V_DrawPatchFlipped".as_ptr());
		}
		// #endif

		if scrn == 0 {
			V_MarkRect(x, y, (*patch).width as usize, (*patch).height as usize);
		}

		let mut col = 0;
		let mut desttop = screens[scrn].wrapping_byte_add(y * SCREENWIDTH + x);

		let w = (*patch).width as usize;

		while col < w {
			let count = *((*patch).columnofs.as_ptr()).wrapping_add(w - 1 - col);
			let mut column = patch.wrapping_byte_add(count) as *mut column_t;

			// step through the posts in a column
			while (*column).topdelta != 0xff {
				let mut source = (column as *mut u8).wrapping_byte_add(3);
				let mut dest = desttop.wrapping_byte_add((*column).topdelta as usize * SCREENWIDTH);
				let count = (*column).length;

				for _ in 0..count {
					*dest = *source;
					source = source.wrapping_byte_add(1);
					dest = dest.wrapping_byte_add(SCREENWIDTH);
				}
				column = column.wrapping_byte_add((*column).length as usize + 4);
			}
			x += 1;
			col += 1;
			desttop = desttop.wrapping_byte_add(1);
		}
	}
}

// V_DrawPatchDirect
// Draws directly to the screen on the pc.
pub(crate) unsafe fn V_DrawPatchDirect(x: usize, y: usize, scrn: usize, patch: *mut patch_t) {
	unsafe {
		V_DrawPatch(x, y, scrn, patch);
	}
}

// V_DrawBlock
// Draw a linear block of pixels into the view buffer.
pub(crate) fn V_DrawBlock(
	x: usize,
	y: usize,
	scrn: usize,
	width: usize,
	height: usize,
	mut src: *mut u8,
) {
	unsafe {
		// #ifdef RANGECHECK
		if x + width > SCREENWIDTH || y + height > SCREENHEIGHT || scrn > 4 {
			I_Error(c"Bad V_DrawBlock".as_ptr());
		}
		// #endif

		V_MarkRect(x, y, width, height);

		let mut dest = screens[scrn].wrapping_byte_add(y * SCREENWIDTH + x);

		for _ in 0..height {
			memcpy(dest.cast(), src.cast(), width);
			src = src.wrapping_byte_add(width);
			dest = dest.wrapping_byte_add(SCREENWIDTH);
		}
	}
}

// V_Init
pub(crate) fn V_Init() {
	unsafe {
		// stick these in low dos memory on PCs

		let base = I_AllocLow(SCREENWIDTH * SCREENHEIGHT * 4);

		#[allow(static_mut_refs)]
		for (i, screen) in screens.iter_mut().enumerate() {
			*screen = base.wrapping_byte_add(i * SCREENWIDTH * SCREENHEIGHT);
		}
	}
}
