// CHEAT SEQUENCE PACKAGE

#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use crate::const_conv::usize_from_u8;

const cheat_xlate_table: [u8; 256] = const {
	let mut table = [0; 256];
	let mut i = 0;
	loop {
		table[usize_from_u8(i)] = scramble(i);
		if i == 255 {
			break;
		}
		i += 1;
	}
	table
};

#[repr(C)]
pub(crate) struct cheatseq_t {
	pub(crate) sequence: *mut u8,
	pub(crate) p: *mut u8,
}

const fn scramble(a: u8) -> u8 {
	(a & 0b_0010_0100) + (a & 0b_1101_1011).reverse_bits()
}

// Called in st_stuff module, which handles the input.
// Returns a 1 if the cheat was successful, 0 if failed.
pub(crate) fn cht_CheckCheat(cht: &mut cheatseq_t, key: u8) -> bool {
	unsafe {
		if cht.p.is_null() {
			cht.p = cht.sequence; // initialize if first time
		}

		if *cht.p == 0 {
			*cht.p = key;
			cht.p = cht.p.add(1);
		} else if cheat_xlate_table[usize::from(key)] == *cht.p {
			cht.p = cht.p.add(1)
		} else {
			cht.p = cht.sequence;
		}

		if *cht.p == 1 {
			cht.p = cht.p.add(1);
		} else if *cht.p == 0xff {
			// end of sequence character
			cht.p = cht.sequence;
			return true;
		}

		false
	}
}

pub(crate) unsafe fn cht_GetParam(cht: &mut cheatseq_t, buffer: &mut [u8]) {
	unsafe {
		let mut p = cht.sequence;

		while *p != 1 {
			p = p.add(1);
		}
		p = p.add(1);

		let mut i = 0;

		loop {
			let c = *p;
			buffer[i] = c;
			i += 1;
			*p = 0;
			p = p.add(1);

			if c == 0 || *p == 0xff {
				break;
			}
		}

		if *p == 0xff {
			buffer[i] = 0;
		}
	}
}
