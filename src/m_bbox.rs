#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use crate::m_fixed::fixed_t;

const BOXTOP: usize = 0;
const BOXBOTTOM: usize = 1;
const BOXLEFT: usize = 2;
const BOXRIGHT: usize = 3;

#[unsafe(no_mangle)]
pub extern "C" fn M_ClearBox(bbox: &mut [fixed_t; 4]) {
	bbox[BOXTOP] = i32::MIN;
	bbox[BOXRIGHT] = i32::MIN;
	bbox[BOXBOTTOM] = i32::MAX;
	bbox[BOXLEFT] = i32::MAX;
}

#[unsafe(no_mangle)]
pub extern "C" fn M_AddToBox(bbox: &mut [fixed_t; 4], x: fixed_t, y: fixed_t) {
	if x < bbox[BOXLEFT] {
		bbox[BOXLEFT] = x;
	} else if x > bbox[BOXRIGHT] {
		bbox[BOXRIGHT] = x;
	}
	if y < bbox[BOXBOTTOM] {
		bbox[BOXBOTTOM] = y;
	} else if y > bbox[BOXTOP] {
		bbox[BOXTOP] = y;
	}
}
