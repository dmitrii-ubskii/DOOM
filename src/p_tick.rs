#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::ffi::c_void;

use crate::{
	d_player::player_t, d_think::thinker_t, doomdef::MAXPLAYERS, p_local::thinkercap,
	p_mobj::P_RespawnSpecials, p_user::P_PlayerThink, z_zone::Z_Free,
};

#[unsafe(no_mangle)]
pub static mut leveltime: usize = 0;

// THINKERS
// All thinkers should be allocated by Z_Malloc
// so they can be operated on uniformly.
// The actual structures will vary in size,
// but the first element must be thinker_t.

// P_InitThinkers
pub(crate) fn P_InitThinkers() {
	unsafe {
		thinkercap.prev = &raw mut thinkercap;
		thinkercap.next = &raw mut thinkercap;
	}
}

// P_AddThinker
// Adds a new thinker at the end of the list.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn P_AddThinker(thinker: *mut thinker_t) {
	unsafe {
		(*thinkercap.prev).next = thinker;
		let thinker_ref = &mut *thinker;
		thinker_ref.next = &raw mut thinkercap;
		thinker_ref.prev = thinkercap.prev;
		thinkercap.prev = thinker;
	}
}

// P_RemoveThinker
// Deallocation is lazy -- it will not actually be freed
// until its thinking turn comes up.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn P_RemoveThinker(thinker: &mut thinker_t) {
	thinker.function.acv = None;
}

// P_RunThinkers
fn run_thinkers() {
	unsafe {
		let mut currentthinker = &mut *thinkercap.next;

		while !std::ptr::eq(currentthinker, &raw mut thinkercap) {
			if currentthinker.function.acv.is_none() {
				// time to remove it
				(*currentthinker.next).prev = currentthinker.prev;
				(*currentthinker.prev).next = currentthinker.next;
				Z_Free(currentthinker as *mut thinker_t as *mut c_void);
			} else if let Some(acp1) = currentthinker.function.acp1 {
				acp1(currentthinker as *mut _ as _);
			}
			currentthinker = &mut *currentthinker.next;
		}
	}
}

unsafe extern "C" {
	static paused: i32;
	static netgame: i32;
	static menuactive: i32;
	static demoplayback: i32;
	pub static mut players: [player_t; MAXPLAYERS];
	pub static playeringame: [i32; MAXPLAYERS];
	static mut consoleplayer: i32;
	fn P_UpdateSpecials();
}

// P_Ticker
#[unsafe(no_mangle)]
pub extern "C" fn P_Ticker() {
	unsafe {
		// run the tic
		if paused != 0 {
			return;
		}

		// pause if in menu and at least one tic has been run
		if netgame == 0
			&& menuactive != 0
			&& demoplayback == 0
			&& players[consoleplayer as usize].viewz != 1
		{
			return;
		}

		for i in 0..MAXPLAYERS {
			if playeringame[i] != 0 {
				P_PlayerThink(&mut players[i]);
			}
		}

		run_thinkers();
		P_UpdateSpecials();
		P_RespawnSpecials();

		// for par times
		leveltime += 1;
	}
}
