// CHEAT SEQUENCE PACKAGE

#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

#[unsafe(no_mangle)]
pub static mut firsttime: i32 = 1;
pub static mut cheat_xlate_table: [u8; 256] = [0; 256];

#[repr(C)]
pub struct cheatseq_t {
	sequence: *mut u8,
	p: *mut u8,
}

fn scramble(a: u8) -> u8 {
	((a & 0b1) << 7)
		+ ((a & 2) << 5)
		+ (a & 4)
		+ ((a & 8) << 1)
		+ ((a & 16) >> 1)
		+ (a & 32)
		+ ((a & 64) >> 5)
		+ ((a & 128) >> 7)
}

// Called in st_stuff module, which handles the input.
// Returns a 1 if the cheat was successful, 0 if failed.
#[unsafe(no_mangle)]
pub unsafe fn cht_CheckCheat(cht: *mut cheatseq_t, key: u8) -> i32 {
	unsafe {
		let cht = &mut *cht;
		let mut rc = 0;

		if firsttime != 0 {
			firsttime = 0;
			for i in 0..=255 {
				cheat_xlate_table[i as usize] = scramble(i);
			}
		}

		if cht.p.is_null() {
			cht.p = cht.sequence; // initialize if first time
		}

		if *cht.p == 0 {
			*cht.p = key;
			cht.p = cht.p.add(1);
		} else if cheat_xlate_table[key as usize] == *cht.p {
			cht.p = cht.p.add(1)
		} else {
			cht.p = cht.sequence;
		}

		if *cht.p == 1 {
			cht.p = cht.p.add(1);
		} else if *cht.p == 0xff {
			// end of sequence character
			cht.p = cht.sequence;
			rc = 1;
		}

		rc
	}
}

#[unsafe(no_mangle)]
pub unsafe fn cht_GetParam(cht: *mut cheatseq_t, mut buffer: *mut u8) {
	unsafe {
		let mut p = (*cht).sequence;

		while *p != 1 {
			p = p.add(1);
		}
		p = p.add(1);

		let mut c = *p;
		*buffer = c;
		buffer = buffer.add(1);
		*p = 0;
		p = p.add(1);

		while c != 0 && *p != 0xff {
			c = *p;
			*buffer = c;
			buffer = buffer.add(1);
			*p = 0;
			p = p.add(1);
		}

		if *p == 0xff {
			*buffer = 0;
		}
	}
}
