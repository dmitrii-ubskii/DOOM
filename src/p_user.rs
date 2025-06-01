#![allow(non_snake_case, clippy::missing_safety_doc)]

use crate::{
	d_event::{BT_CHANGE, BT_SPECIAL, BT_USE, BT_WEAPONMASK, BT_WEAPONSHIFT},
	d_player::{cheat_t, player_t, playerstate_t},
	doomdef::{GameMode_t, powertype_t, weapontype_t},
	doomstat::gamemode,
	info::{state_t, statenum_t},
	m_fixed::{FRACUNIT, FixedMul, fixed_t},
	p_local::VIEWHEIGHT,
	p_mobj::{MF_JUSTATTACKED, MF_NOCLIP, MF_SHADOW, mobj_t},
	p_tick::leveltime,
	tables::{ANG90, ANG180, ANGLETOFINESHIFT, FINEANGLES, FINEMASK, angle_t, finecos, finesine},
};

// Index of the special effects (INVUL inverse) map.
const INVERSECOLORMAP: i32 = 32;

// Movement.

// 16 pixels of bob
const MAXBOB: i32 = 0x100000;

static mut onground: bool = false;

// P_Thrust
// Moves the given origin along a given angle.
fn P_Thrust(player: &mut player_t, mut angle: angle_t, mov: fixed_t) {
	angle >>= ANGLETOFINESHIFT;
	let angle = angle as usize;

	unsafe {
		(*player.mo).momx += FixedMul(mov, finecos(angle));
		(*player.mo).momy += FixedMul(mov, finesine[angle]);
	}
}

// P_CalcHeight
// Calculate the walking / running height adjustment
fn P_CalcHeight(player: &mut player_t) {
	unsafe {
		// Regular movement bobbing
		// (needs to be calculated for gun swing
		// even if not on ground)
		// OPTIMIZE: tablify angle
		// Note: a LUT allows for effects
		//  like a ramp with low health.
		let mo = &mut *player.mo;

		player.bob = FixedMul(mo.momx, mo.momx) + FixedMul(mo.momy, mo.momy);

		player.bob >>= 2;

		if player.bob > MAXBOB {
			player.bob = MAXBOB;
		}

		if (player.cheats & cheat_t::CF_NOMOMENTUM as i32 != 0) || !onground {
			player.viewz = mo.z + VIEWHEIGHT;

			if player.viewz > mo.ceilingz - 4 * FRACUNIT {
				player.viewz = mo.ceilingz - 4 * FRACUNIT;
			}

			player.viewz = mo.z + player.viewheight;
			return;
		}

		let angle = (FINEANGLES / 20 * leveltime) & FINEMASK;
		let bob = FixedMul(player.bob / 2, finesine[angle]);

		// move viewheight
		if player.playerstate == playerstate_t::PST_LIVE {
			player.viewheight += player.deltaviewheight;

			if player.viewheight > VIEWHEIGHT {
				player.viewheight = VIEWHEIGHT;
				player.deltaviewheight = 0;
			}

			if player.viewheight < VIEWHEIGHT / 2 {
				player.viewheight = VIEWHEIGHT / 2;
				if player.deltaviewheight <= 0 {
					player.deltaviewheight = 1;
				}
			}

			if player.deltaviewheight != 0 {
				player.deltaviewheight += FRACUNIT / 4;
				if player.deltaviewheight == 0 {
					player.deltaviewheight = 1;
				}
			}
		}
		player.viewz = mo.z + player.viewheight + bob;

		if player.viewz > mo.ceilingz - 4 * FRACUNIT {
			player.viewz = mo.ceilingz - 4 * FRACUNIT;
		}
	}
}

unsafe extern "C" {
	fn P_SetMobjState(mobj: *mut mobj_t, state: statenum_t) -> i32;
	static mut states: [state_t; statenum_t::NUMSTATES as usize];
}

// P_MovePlayer
fn P_MovePlayer(player: &mut player_t) {
	unsafe {
		let mo = &mut *player.mo;
		let cmd = &player.cmd;

		mo.angle = mo.angle.wrapping_add_signed((cmd.angleturn as i32) << 16);

		// Do not let the player control movement
		//  if not onground.
		onground = mo.z <= mo.floorz;

		if cmd.forwardmove != 0 && onground {
			let angle = mo.angle;
			let mov = cmd.forwardmove as i32 * 2048;
			P_Thrust(player, angle, mov);
		}

		let mo = &mut *player.mo;
		let cmd = &mut player.cmd;
		if cmd.sidemove != 0 && onground {
			let angle = mo.angle.wrapping_sub(ANG90);
			let mov = cmd.sidemove as i32 * 2048;
			P_Thrust(player, angle, mov);
		}

		let mo = &mut *player.mo;
		let cmd = &mut player.cmd;
		if (cmd.forwardmove != 0 || cmd.sidemove != 0)
			&& mo.state == &raw mut states[statenum_t::S_PLAY as usize]
		{
			P_SetMobjState(mo, statenum_t::S_PLAY_RUN1);
		}
	}
}

unsafe extern "C" {
	fn P_MovePsprites(player: &mut player_t);
	fn R_PointToAngle2(x_1: i32, y_1: i32, x_2: i32, y_2: i32) -> u32;
}

// P_DeathThink
// Fall on your face when dying.
// Decrease POV height to floor height.
const ANG5: angle_t = ANG90 / 18;

fn P_DeathThink(player: &mut player_t) {
	unsafe {
		P_MovePsprites(player);

		// fall to the ground
		if player.viewheight > 6 * FRACUNIT {
			player.viewheight -= FRACUNIT;
		}

		if player.viewheight < 6 * FRACUNIT {
			player.viewheight = 6 * FRACUNIT;
		}

		let mo = &mut *player.mo;

		player.deltaviewheight = 0;
		onground = mo.z <= mo.floorz;
		P_CalcHeight(player);

		if player.attacker != mo {
			let attacker = &mut *player.attacker;
			let angle = R_PointToAngle2(mo.x, mo.y, attacker.x, attacker.y);

			let delta = angle.wrapping_sub(mo.angle);

			if delta < ANG5 || delta > -(ANG5 as i32) as u32 {
				// Looking at killer,
				//  so fade damage flash down.
				mo.angle = angle;

				if player.damagecount != 0 {
					player.damagecount -= 1;
				}
			} else if delta < ANG180 {
				mo.angle += ANG5;
			} else {
				mo.angle -= ANG5;
			}
		} else if player.damagecount != 0 {
			player.damagecount -= 1;
		}

		if player.cmd.buttons & BT_USE != 0 {
			player.playerstate = playerstate_t::PST_REBORN;
		}
	}
}

unsafe extern "C" {
	fn P_PlayerInSpecialSector(player: &mut player_t);
	fn P_UseLines(player: &mut player_t);
}

// P_PlayerThink
pub fn P_PlayerThink(player: &mut player_t) {
	unsafe {
		// fixme: do this in the cheat code
		if player.cheats & cheat_t::CF_NOCLIP as i32 != 0 {
			(*player.mo).flags |= MF_NOCLIP;
		} else {
			(*player.mo).flags &= !MF_NOCLIP;
		}

		// chain saw run forward
		let cmd = &mut player.cmd;
		if (*player.mo).flags & MF_JUSTATTACKED != 0 {
			cmd.angleturn = 0;
			cmd.forwardmove = 100;
			cmd.sidemove = 0;
			(*player.mo).flags &= !MF_JUSTATTACKED;
		}

		if player.playerstate == playerstate_t::PST_DEAD {
			P_DeathThink(player);
			return;
		}

		// Move around.
		// Reactiontime is used to prevent movement
		//  for a bit after a teleport.
		if (*player.mo).reactiontime != 0 {
			(*player.mo).reactiontime -= 1;
		} else {
			P_MovePlayer(player);
		}

		P_CalcHeight(player);

		if (*(*(*player.mo).subsector).sector).special != 0 {
			P_PlayerInSpecialSector(player);
		}

		// Check for weapon change.
		let cmd = &mut player.cmd;

		// A special event has no other buttons.
		if cmd.buttons & BT_SPECIAL != 0 {
			cmd.buttons = 0;
		}

		if cmd.buttons & BT_CHANGE != 0 {
			// The actual changing of the weapon is done
			//  when the weapon psprite can do it
			//  (read: not in the middle of an attack).
			let mut newweapon = (cmd.buttons & BT_WEAPONMASK) >> BT_WEAPONSHIFT;

			if newweapon == weapontype_t::wp_fist as u8
				&& player.weaponowned[weapontype_t::wp_chainsaw as usize] != 0
				&& !(player.readyweapon == weapontype_t::wp_chainsaw
					&& player.powers[powertype_t::pw_strength as usize] != 0)
			{
				newweapon = weapontype_t::wp_chainsaw as u8;
			}

			if (gamemode == GameMode_t::commercial)
				&& newweapon == weapontype_t::wp_shotgun as u8
				&& player.weaponowned[weapontype_t::wp_supershotgun as usize] != 0
				&& player.readyweapon != weapontype_t::wp_supershotgun
			{
				newweapon = weapontype_t::wp_supershotgun as u8;
			}

			if player.weaponowned[newweapon as usize] != 0 && newweapon != player.readyweapon as u8
			{
				// Do not go to plasma or BFG in shareware,
				//  even if cheated.
				if (newweapon != weapontype_t::wp_plasma as u8
					&& newweapon != weapontype_t::wp_bfg as u8)
					|| (gamemode != GameMode_t::shareware)
				{
					player.pendingweapon =
						std::mem::transmute::<u32, weapontype_t>(newweapon as u32);
				}
			}
		}

		// check for use
		if cmd.buttons & BT_USE != 0 {
			if player.usedown == 0 {
				P_UseLines(player);
				player.usedown = 1;
			}
		} else {
			player.usedown = 0;
		}

		// cycle psprites
		P_MovePsprites(player);

		// Counters, time dependend power ups.

		// Strength counts up to diminish fade.
		if player.powers[powertype_t::pw_strength as usize] != 0 {
			player.powers[powertype_t::pw_strength as usize] += 1;
		}

		if player.powers[powertype_t::pw_invulnerability as usize] != 0 {
			player.powers[powertype_t::pw_invulnerability as usize] -= 1;
		}

		if player.powers[powertype_t::pw_invisibility as usize] != 0 {
			player.powers[powertype_t::pw_invisibility as usize] -= 1;
			if player.powers[powertype_t::pw_invisibility as usize] == 0 {
				(*player.mo).flags &= !MF_SHADOW;
			}
		}

		if player.powers[powertype_t::pw_infrared as usize] != 0 {
			player.powers[powertype_t::pw_infrared as usize] -= 1;
		}

		if player.powers[powertype_t::pw_ironfeet as usize] != 0 {
			player.powers[powertype_t::pw_ironfeet as usize] -= 1;
		}

		if player.damagecount != 0 {
			player.damagecount -= 1;
		}

		if player.bonuscount != 0 {
			player.bonuscount -= 1;
		}

		// Handling colormaps.
		if player.powers[powertype_t::pw_invulnerability as usize] != 0 {
			if player.powers[powertype_t::pw_invulnerability as usize] > 4 * 32
				|| (player.powers[powertype_t::pw_invulnerability as usize] & 8) != 0
			{
				player.fixedcolormap = INVERSECOLORMAP;
			} else {
				player.fixedcolormap = 0;
			}
		} else if player.powers[powertype_t::pw_infrared as usize] != 0 {
			if player.powers[powertype_t::pw_infrared as usize] > 4 * 32
				|| player.powers[powertype_t::pw_infrared as usize] & 8 != 0
			{
				// almost full bright
				player.fixedcolormap = 1;
			} else {
				player.fixedcolormap = 0;
			}
		} else {
			player.fixedcolormap = 0;
		}
	}
}
