#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::ffi::c_void;

use crate::{
	d_think::{actionf_p1, thinker_t},
	info::mobjtype_t,
	m_fixed::fixed_t,
	p_mobj::{MF_MISSILE, mobj_t},
	r_defs::{line_t, sector_t},
	sounds::sfxenum_t,
	tables::{ANGLETOFINESHIFT, finecosine, finesine},
};

unsafe extern "C" {
	static numsectors: i32;
	static sectors: *mut sector_t;
	static thinkercap: thinker_t;
	fn P_MobjThinker(_: *mut c_void);
	fn P_TeleportMove(thing: *mut mobj_t, x: fixed_t, y: fixed_t) -> bool;
	fn P_SpawnMobj(x: fixed_t, y: fixed_t, z: fixed_t, ty: mobjtype_t) -> *mut mobj_t;
	fn S_StartSound(origin: *mut c_void, sound_id: sfxenum_t);
}

// TELEPORTATION
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EV_Teleport(line: &mut line_t, side: i32, thing: &mut mobj_t) -> bool {
	// don't teleport missiles
	if thing.flags & MF_MISSILE != 0 {
		return false;
	}

	// Don't teleport if hit back of line,
	//  so you can get out of teleporter.
	if side == 1 {
		return false;
	}

	let tag = line.tag;

	unsafe {
		for i in 0..numsectors {
			if (*sectors.add(i as usize)).tag == tag {
				let mut thinker = &mut *thinkercap.next;
				while !std::ptr::eq(thinker, &thinkercap) {
					// not a mobj
					if (thinker.function.acp1)
						.is_none_or(|f| !std::ptr::fn_addr_eq(f, P_MobjThinker as actionf_p1))
					{
						thinker = &mut *thinker.next;
						continue;
					}

					let m = &mut *(thinker as *mut _ as *mut mobj_t);

					// not a teleportman
					if m.ty != mobjtype_t::MT_TELEPORTMAN {
						thinker = &mut *thinker.next;
						continue;
					}

					let sector = (*m.subsector).sector;
					// wrong sectori32
					if sector.offset_from(sectors) != i as isize {
						thinker = &mut *thinker.next;
						continue;
					}

					let oldx = thing.x;
					let oldy = thing.y;
					let oldz = thing.z;

					if !P_TeleportMove(thing, m.x, m.y) {
						return false;
					}

					thing.z = thing.floorz; //fixme: not needed?
					if !thing.player.is_null() {
						let player = &mut *thing.player;
						player.viewz = thing.z + player.viewheight;
					}

					// spawn teleport fog at source and destination
					let mut fog = P_SpawnMobj(oldx, oldy, oldz, mobjtype_t::MT_TFOG);
					S_StartSound(fog as _, sfxenum_t::sfx_telept);
					let an = m.angle >> ANGLETOFINESHIFT;
					fog = P_SpawnMobj(
						m.x + 20 * finecosine[an as usize],
						m.y + 20 * finesine[an as usize],
						thing.z,
						mobjtype_t::MT_TFOG,
					);

					// emit sound, where?
					S_StartSound(fog as _, sfxenum_t::sfx_telept);

					// don't move for a bit
					if !thing.player.is_null() {
						thing.reactiontime = 18;
					}

					thing.angle = m.angle;
					thing.momx = 0;
					thing.momy = 0;
					thing.momz = 0;
					return true;
				}
			}
		}
	}

	false
}
