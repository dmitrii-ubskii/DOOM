#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use crate::m_fixed::fixed_t;

pub const BOXTOP: usize = 0;
pub const BOXBOTTOM: usize = 1;
pub const BOXLEFT: usize = 2;
pub const BOXRIGHT: usize = 3;

pub(crate) fn M_ClearBox(bbox: &mut [fixed_t; 4]) {
	bbox[BOXTOP] = i32::MIN;
	bbox[BOXRIGHT] = i32::MIN;
	bbox[BOXBOTTOM] = i32::MAX;
	bbox[BOXLEFT] = i32::MAX;
}

pub(crate) fn M_AddToBox(bbox: &mut [fixed_t; 4], x: fixed_t, y: fixed_t) {
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
