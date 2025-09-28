#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::{num::Wrapping, ptr::null_mut};

use crate::{
	d_event::BT_ATTACK,
	d_items::weaponinfo,
	d_player::{player_t, playerstate_t},
	doomdef::{GameMode_t, ammotype_t, powertype_t, weapontype_t},
	doomstat::gamemode,
	info::{mobjtype_t, state_t, statenum_t, states},
	m_fixed::{FRACBITS, FRACUNIT, FixedMul, fixed_t},
	m_random::P_Random,
	p_enemy::P_NoiseAlert,
	p_inter::P_DamageMobj,
	p_local::{MELEERANGE, MISSILERANGE},
	p_map::{P_AimLineAttack, P_LineAttack, linetarget},
	p_mobj::{MF_JUSTATTACKED, P_SetMobjState, P_SpawnMobj, P_SpawnPlayerMissile, mobj_t},
	p_tick::leveltime,
	s_sound::S_StartSound,
	sounds::sfxenum_t,
	tables::{ANG90, ANG180, FINEANGLES, FINEMASK, angle_t, finecos, finesine},
};

// Frame flags:
// handles maximum brightness (torches, muzzle flare, light sources)
pub const FF_FULLBRIGHT: usize = 0x8000; // flag in thing->frame
pub const FF_FRAMEMASK: usize = 0x7fff;

// Overlay psprites are scaled shapes
// drawn directly on the view screen,
// coordinates are given for a 320*200 view screen.
#[repr(C)]
pub enum psprnum_t {
	ps_weapon,
	ps_flash,
	NUMPSPRITES,
}

impl From<usize> for psprnum_t {
	fn from(value: usize) -> Self {
		match value {
			0 => Self::ps_weapon,
			1 => Self::ps_flash,
			_ => unreachable!("{value} out of bounds for psprnum_t"),
		}
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct pspdef_t {
	pub state: *mut state_t, // a NULL state means not active
	pub tics: i32,
	pub sx: fixed_t,
	pub sy: fixed_t,
}

pub const LOWERSPEED: fixed_t = FRACUNIT * 6;
pub const RAISESPEED: fixed_t = FRACUNIT * 6;

pub const WEAPONBOTTOM: fixed_t = 128 * FRACUNIT;
pub const WEAPONTOP: fixed_t = 32 * FRACUNIT;

// plasma cells for a bfg attack
pub const BFGCELLS: usize = 40;

// P_SetPsprite
fn P_SetPsprite(player: &mut player_t, position: psprnum_t, mut stnum: statenum_t) {
	unsafe {
		let player_p = player as *mut player_t;
		let psp = &mut player.psprites[position as usize];

		loop {
			if stnum == statenum_t::S_NULL {
				// object removed itself
				psp.state = null_mut();
				break;
			}

			let state = &raw mut states[stnum as usize];
			psp.state = state;
			psp.tics = (*state).tics; // could be 0

			if (*state).misc1 != 0 {
				// coordinate set
				psp.sx = (*state).misc1 << FRACBITS;
				psp.sy = (*state).misc2 << FRACBITS;
			}

			// Call action routine.
			// Modified handling.
			if let Some(action) = (*state).action.as_ac_pspr() {
				action(&mut *player_p, psp);
				if psp.state.is_null() {
					break;
				}
			}

			stnum = (*psp.state).nextstate;

			if psp.tics != 0 {
				break;
			}
		}
		// an initial state of 0 could cycle through
	}
}

// P_BringUpWeapon
// Starts bringing the pending weapon up
// from the bottom of the screen.
// Uses player
fn P_BringUpWeapon(player: &mut player_t) {
	if player.pendingweapon == weapontype_t::wp_nochange {
		player.pendingweapon = player.readyweapon;
	}

	if player.pendingweapon == weapontype_t::wp_chainsaw {
		S_StartSound(player.mo.cast(), sfxenum_t::sfx_sawup);
	}

	let newstate = weaponinfo[player.pendingweapon as usize].upstate;

	player.pendingweapon = weapontype_t::wp_nochange;
	player.psprites[psprnum_t::ps_weapon as usize].sy = WEAPONBOTTOM;

	P_SetPsprite(player, psprnum_t::ps_weapon, newstate);
}

// P_CheckAmmo
// Returns true if there is enough ammo to shoot.
// If not, selects the next weapon to use.
fn P_CheckAmmo(player: &mut player_t) -> bool {
	unsafe {
		let ammo = weaponinfo[player.readyweapon as usize].ammo;

		// Minimal amount for one shot varies.
		let count = match player.readyweapon {
			weapontype_t::wp_bfg => BFGCELLS,
			weapontype_t::wp_supershotgun => 2, // Double barrel.
			_ => 1,                             // Regular.
		};

		// Some do not need ammunition anyway.
		// Return if current ammunition sufficient.
		if ammo == ammotype_t::am_noammo || player.ammo[ammo as usize] >= count {
			return true;
		}

		// Out of ammo, pick a weapon to change to.
		// Preferences are set here.
		loop {
			if player.weaponowned[weapontype_t::wp_plasma as usize] != 0
				&& player.ammo[ammotype_t::am_cell as usize] != 0
				&& gamemode != GameMode_t::shareware
			{
				player.pendingweapon = weapontype_t::wp_plasma;
			} else if player.weaponowned[weapontype_t::wp_supershotgun as usize] != 0
				&& player.ammo[ammotype_t::am_shell as usize] > 2
				&& gamemode == GameMode_t::commercial
			{
				player.pendingweapon = weapontype_t::wp_supershotgun;
			} else if player.weaponowned[weapontype_t::wp_chaingun as usize] != 0
				&& player.ammo[ammotype_t::am_clip as usize] != 0
			{
				player.pendingweapon = weapontype_t::wp_chaingun;
			} else if player.weaponowned[weapontype_t::wp_shotgun as usize] != 0
				&& player.ammo[ammotype_t::am_shell as usize] != 0
			{
				player.pendingweapon = weapontype_t::wp_shotgun;
			} else if player.ammo[ammotype_t::am_clip as usize] != 0 {
				player.pendingweapon = weapontype_t::wp_pistol;
			} else if player.weaponowned[weapontype_t::wp_chainsaw as usize] != 0 {
				player.pendingweapon = weapontype_t::wp_chainsaw;
			} else if player.weaponowned[weapontype_t::wp_missile as usize] != 0
				&& player.ammo[ammotype_t::am_misl as usize] != 0
			{
				player.pendingweapon = weapontype_t::wp_missile;
			} else if player.weaponowned[weapontype_t::wp_bfg as usize] != 0
				&& player.ammo[ammotype_t::am_cell as usize] > BFGCELLS
				&& gamemode != GameMode_t::shareware
			{
				player.pendingweapon = weapontype_t::wp_bfg;
			} else {
				// If everything fails.
				player.pendingweapon = weapontype_t::wp_fist;
			}

			if player.pendingweapon != weapontype_t::wp_nochange {
				break;
			}
		}

		// Now set appropriate weapon overlay.
		P_SetPsprite(
			player,
			psprnum_t::ps_weapon,
			weaponinfo[player.readyweapon as usize].downstate,
		);

		false
	}
}

// P_FireWeapon.
fn P_FireWeapon(player: &mut player_t) {
	if !P_CheckAmmo(player) {
		return;
	}

	unsafe { P_SetMobjState(&mut *player.mo, statenum_t::S_PLAY_ATK1) };
	let newstate = weaponinfo[player.readyweapon as usize].atkstate;
	P_SetPsprite(player, psprnum_t::ps_weapon, newstate);
	unsafe { P_NoiseAlert(player.mo, &mut *player.mo) };
}

// P_DropWeapon
// Player died, so put the weapon away.
#[unsafe(no_mangle)]
pub extern "C" fn P_DropWeapon(player: &mut player_t) {
	P_SetPsprite(player, psprnum_t::ps_weapon, weaponinfo[player.readyweapon as usize].downstate);
}

// A_WeaponReady
// The player can fire the weapon
// or change to another weapon at this time.
// Follows after getting weapon up,
// or after previous attack/fire sequence.
pub(crate) fn A_WeaponReady(player: &mut player_t, psp: &mut pspdef_t) {
	unsafe {
		// get out of attack state
		if (*player.mo).state == &raw mut states[statenum_t::S_PLAY_ATK1 as usize]
			|| (*player.mo).state == &raw mut states[statenum_t::S_PLAY_ATK2 as usize]
		{
			P_SetMobjState(&mut *player.mo, statenum_t::S_PLAY);
		}

		if player.readyweapon == weapontype_t::wp_chainsaw
			&& std::ptr::eq(psp.state, &raw const states[statenum_t::S_SAW as usize])
		{
			S_StartSound(player.mo.cast(), sfxenum_t::sfx_sawidl);
		}

		// check for change
		//  if player is dead, put the weapon away
		if player.pendingweapon != weapontype_t::wp_nochange || player.health == 0 {
			// change weapon
			//  (pending weapon should allready be validated)
			let newstate = weaponinfo[player.readyweapon as usize].downstate;
			P_SetPsprite(player, psprnum_t::ps_weapon, newstate);
			return;
		}

		// check for fire
		//  the missile launcher and bfg do not auto fire
		if player.cmd.buttons & BT_ATTACK != 0 {
			if player.attackdown == 0
				|| (player.readyweapon != weapontype_t::wp_missile
					&& player.readyweapon != weapontype_t::wp_bfg)
			{
				player.attackdown = 1;
				P_FireWeapon(player);
				return;
			}
		} else {
			player.attackdown = 0;
		}

		// bob the weapon based on movement speed
		let mut angle = (128 * leveltime) & FINEMASK;
		psp.sx = FRACUNIT + FixedMul(player.bob, finecos(angle));
		angle &= FINEANGLES / 2 - 1;
		psp.sy = WEAPONTOP + FixedMul(player.bob, finesine[angle]);
	}
}

// A_ReFire
// The player can re-fire the weapon
// without lowering it entirely.
pub(crate) fn A_ReFire(player: &mut player_t, _psp: &mut pspdef_t) {
	// check for fire
	//  (if a weaponchange is pending, let it go through instead)
	if player.cmd.buttons & BT_ATTACK != 0
		&& player.pendingweapon == weapontype_t::wp_nochange
		&& player.health != 0
	{
		player.refire += 1;
		P_FireWeapon(player);
	} else {
		player.refire = 0;
		P_CheckAmmo(player);
	}
}

pub(crate) fn A_CheckReload(player: &mut player_t, _psp: &mut pspdef_t) {
	P_CheckAmmo(player);
}

// A_Lower
// Lowers current weapon,
//  and changes weapon at bottom.
pub(crate) fn A_Lower(player: &mut player_t, psp: &mut pspdef_t) {
	psp.sy += LOWERSPEED;

	// Is already down.
	if psp.sy < WEAPONBOTTOM {
		return;
	}

	// Player is dead.
	if player.playerstate == playerstate_t::PST_DEAD {
		psp.sy = WEAPONBOTTOM;

		// don't bring weapon back up
		return;
	}

	// The old weapon has been lowered off the screen,
	// so change the weapon and start raising it
	if player.health == 0 {
		// Player is dead, so keep the weapon off screen.
		P_SetPsprite(player, psprnum_t::ps_weapon, statenum_t::S_NULL);
		return;
	}

	player.readyweapon = player.pendingweapon;

	P_BringUpWeapon(player);
}

// A_Raise
pub(crate) fn A_Raise(player: &mut player_t, psp: &mut pspdef_t) {
	psp.sy -= RAISESPEED;

	if psp.sy > WEAPONTOP {
		return;
	}

	psp.sy = WEAPONTOP;

	// The weapon has been raised all the way,
	//  so change to the ready state.
	let newstate = weaponinfo[player.readyweapon as usize].readystate;

	P_SetPsprite(player, psprnum_t::ps_weapon, newstate);
}

// A_GunFlash
pub(crate) fn A_GunFlash(player: &mut player_t, _psp: &mut pspdef_t) {
	unsafe {
		P_SetMobjState(&mut *player.mo, statenum_t::S_PLAY_ATK2);
		P_SetPsprite(
			player,
			psprnum_t::ps_flash,
			weaponinfo[player.readyweapon as usize].flashstate,
		);
	}
}

// WEAPON ATTACKS

unsafe extern "C" {
	fn R_PointToAngle2(x_1: i32, y_1: i32, x_2: i32, y_2: i32) -> angle_t;
}

// A_Punch
pub(crate) fn A_Punch(player: &mut player_t, _psp: &mut pspdef_t) {
	unsafe {
		let mut damage = (P_Random() % 10 + 1) << 1;

		if player.powers[powertype_t::pw_strength as usize] != 0 {
			damage *= 10;
		}

		let mut angle = (*player.mo).angle;
		angle += ((P_Random() - P_Random()) << 18) as usize;
		let slope = P_AimLineAttack(&mut *player.mo, angle, MELEERANGE);
		P_LineAttack(&mut *player.mo, angle, MELEERANGE, slope, damage);

		// turn to face target
		if !linetarget.is_null() {
			S_StartSound(player.mo.cast(), sfxenum_t::sfx_punch);
			(*player.mo).angle =
				R_PointToAngle2((*player.mo).x, (*player.mo).y, (*linetarget).x, (*linetarget).y);
		}
	}
}

// A_Saw
pub(crate) fn A_Saw(player: &mut player_t, _psp: &mut pspdef_t) {
	unsafe {
		let damage = 2 * (P_Random() % 10 + 1);
		let mut angle = (*player.mo).angle;
		angle += ((P_Random() - P_Random()) << 18) as usize;

		// use meleerange + 1 se the puff doesn't skip the flash
		let slope = P_AimLineAttack(&mut *player.mo, angle, MELEERANGE + 1);
		P_LineAttack(&mut *player.mo, angle, MELEERANGE + 1, slope, damage);

		if linetarget.is_null() {
			S_StartSound(player.mo.cast(), sfxenum_t::sfx_sawful);
			return;
		}
		S_StartSound(player.mo.cast(), sfxenum_t::sfx_sawhit);

		// turn to face target
		angle = R_PointToAngle2((*player.mo).x, (*player.mo).y, (*linetarget).x, (*linetarget).y);
		if angle - (*player.mo).angle > ANG180 {
			if angle - (*player.mo).angle < -ANG90 / Wrapping(20) {
				(*player.mo).angle = angle + ANG90 / Wrapping(21);
			} else {
				(*player.mo).angle -= ANG90 / Wrapping(20);
			}
		} else if angle - (*player.mo).angle > ANG90 / Wrapping(20) {
			(*player.mo).angle = angle - ANG90 / Wrapping(21);
		} else {
			(*player.mo).angle += ANG90 / Wrapping(20);
		}
		(*player.mo).flags |= MF_JUSTATTACKED;
	}
}

// A_FireMissile
pub(crate) fn A_FireMissile(player: &mut player_t, _psp: &mut pspdef_t) {
	player.ammo[weaponinfo[player.readyweapon as usize].ammo as usize] -= 1;
	unsafe { P_SpawnPlayerMissile(&mut *player.mo, mobjtype_t::MT_ROCKET) };
}

// A_FireBFG
pub(crate) fn A_FireBFG(player: &mut player_t, _psp: &mut pspdef_t) {
	player.ammo[weaponinfo[player.readyweapon as usize].ammo as usize] -= BFGCELLS;
	unsafe { P_SpawnPlayerMissile(&mut *player.mo, mobjtype_t::MT_BFG) };
}

// A_FirePlasma
pub(crate) fn A_FirePlasma(player: &mut player_t, _psp: &mut pspdef_t) {
	player.ammo[weaponinfo[player.readyweapon as usize].ammo as usize] -= 1;

	P_SetPsprite(
		player,
		psprnum_t::ps_flash,
		statenum_t::from(
			weaponinfo[player.readyweapon as usize].flashstate as usize + (P_Random() & 1) as usize,
		),
	);

	P_SpawnPlayerMissile(unsafe { &mut *player.mo }, mobjtype_t::MT_PLASMA);
}

// P_BulletSlope
// Sets a slope so a near miss is at aproximately
// the height of the intended target
static mut bulletslope: fixed_t = 0;

fn P_BulletSlope(mo: &mut mobj_t) {
	unsafe {
		// see which target is to be aimed at
		let mut an = mo.angle;
		bulletslope = P_AimLineAttack(mo, an, 16 * 64 * FRACUNIT);

		if linetarget.is_null() {
			an += 1 << 26;
			bulletslope = P_AimLineAttack(mo, an, 16 * 64 * FRACUNIT);
			if linetarget.is_null() {
				an -= 2 << 26;
				bulletslope = P_AimLineAttack(mo, an, 16 * 64 * FRACUNIT);
			}
		}
	}
}

// P_GunShot
fn P_GunShot(mo: &mut mobj_t, accurate: bool) {
	let damage = 5 * (P_Random() % 3 + 1);
	let mut angle = mo.angle;

	if !accurate {
		angle += ((P_Random() - P_Random()) << 18) as usize;
	}

	unsafe { P_LineAttack(mo, angle, MISSILERANGE, bulletslope, damage) };
}

// A_FirePistol
pub(crate) fn A_FirePistol(player: &mut player_t, _psp: &mut pspdef_t) {
	unsafe {
		S_StartSound(player.mo.cast(), sfxenum_t::sfx_pistol);

		P_SetMobjState(&mut *player.mo, statenum_t::S_PLAY_ATK2);
		player.ammo[weaponinfo[player.readyweapon as usize].ammo as usize] -= 1;

		P_SetPsprite(
			player,
			psprnum_t::ps_flash,
			weaponinfo[player.readyweapon as usize].flashstate,
		);

		P_BulletSlope(&mut *player.mo);
		P_GunShot(&mut *player.mo, player.refire == 0);
	}
}

// A_FireShotgun
pub(crate) fn A_FireShotgun(player: &mut player_t, _psp: &mut pspdef_t) {
	unsafe {
		S_StartSound(player.mo.cast(), sfxenum_t::sfx_shotgn);
		P_SetMobjState(&mut *player.mo, statenum_t::S_PLAY_ATK2);

		player.ammo[weaponinfo[player.readyweapon as usize].ammo as usize] -= 1;

		P_SetPsprite(
			player,
			psprnum_t::ps_flash,
			weaponinfo[player.readyweapon as usize].flashstate,
		);

		P_BulletSlope(&mut *player.mo);

		for _ in 0..7 {
			P_GunShot(&mut *player.mo, false);
		}
	}
}

// A_FireShotgun2
pub(crate) fn A_FireShotgun2(player: &mut player_t, _psp: &mut pspdef_t) {
	unsafe {
		S_StartSound(player.mo.cast(), sfxenum_t::sfx_dshtgn);
		P_SetMobjState(&mut *player.mo, statenum_t::S_PLAY_ATK2);

		player.ammo[weaponinfo[player.readyweapon as usize].ammo as usize] -= 2;

		P_SetPsprite(
			player,
			psprnum_t::ps_flash,
			weaponinfo[player.readyweapon as usize].flashstate,
		);

		P_BulletSlope(&mut *player.mo);

		for _ in 0..20 {
			let damage = 5 * (P_Random() % 3 + 1);
			let mut angle = (*player.mo).angle;
			angle += ((P_Random() - P_Random()) << 19) as usize;
			P_LineAttack(
				&mut *player.mo,
				angle,
				MISSILERANGE,
				bulletslope + ((P_Random() - P_Random()) << 5),
				damage,
			);
		}
	}
}

// A_FireCGun
pub(crate) fn A_FireCGun(player: &mut player_t, psp: &mut pspdef_t) {
	unsafe {
		S_StartSound(player.mo.cast(), sfxenum_t::sfx_pistol);

		if player.ammo[weaponinfo[player.readyweapon as usize].ammo as usize] == 0 {
			return;
		}

		P_SetMobjState(&mut *player.mo, statenum_t::S_PLAY_ATK2);
		player.ammo[weaponinfo[player.readyweapon as usize].ammo as usize] -= 1;

		P_SetPsprite(
			player,
			psprnum_t::ps_flash,
			statenum_t::from(
				weaponinfo[player.readyweapon as usize].flashstate as usize
					+ psp.state.offset_from(&raw const states[statenum_t::S_CHAIN1 as usize])
						as usize,
			),
		);

		P_BulletSlope(&mut *player.mo);

		P_GunShot(&mut *player.mo, player.refire == 0);
	}
}

// ?
pub(crate) fn A_Light0(player: &mut player_t, _psp: &mut pspdef_t) {
	player.extralight = 0;
}

pub(crate) fn A_Light1(player: &mut player_t, _psp: &mut pspdef_t) {
	player.extralight = 1;
}

pub(crate) fn A_Light2(player: &mut player_t, _psp: &mut pspdef_t) {
	player.extralight = 2;
}

// A_BFGSpray
// Spawn a BFG explosion on every monster in view
pub(crate) fn A_BFGSpray(mo: &mut mobj_t) {
	unsafe {
		// offset angles from its attack angle
		for i in 0..40 {
			let an = mo.angle - Wrapping(ANG90.0 / 2 - ANG90.0 / 40 * i);

			// mo.target is the originator (player)
			//  of the missile
			P_AimLineAttack(&mut *mo.target, an, 16 * 64 * FRACUNIT);

			if linetarget.is_null() {
				continue;
			}

			P_SpawnMobj(
				(*linetarget).x,
				(*linetarget).y,
				(*linetarget).z + ((*linetarget).height >> 2),
				mobjtype_t::MT_EXTRABFG,
			);

			let mut damage = 0;
			for _ in 0..15 {
				damage += (P_Random() & 7) + 1;
			}

			P_DamageMobj(&mut *linetarget, mo.target, mo.target, damage);
		}
	}
}

// A_BFGsound
pub(crate) fn A_BFGsound(player: &mut player_t, _psp: &mut pspdef_t) {
	S_StartSound(player.mo.cast(), sfxenum_t::sfx_bfg);
}

// P_SetupPsprites
// Called at start of level for each player.
pub(crate) fn P_SetupPsprites(player: &mut player_t) {
	// remove all psprites
	for i in 0..psprnum_t::NUMPSPRITES as usize {
		player.psprites[i].state = null_mut();
	}

	// spawn the gun
	player.pendingweapon = player.readyweapon;
	P_BringUpWeapon(player);
}

// P_MovePsprites
// Called every tic by player thinking routine.
pub(crate) fn P_MovePsprites(player: &mut player_t) {
	for i in 0..psprnum_t::NUMPSPRITES as usize {
		let psp = &mut player.psprites[i];
		// a null state means not active
		let state = psp.state;
		if !state.is_null() {
			// drop tic count and possibly change state

			// a -1 tic count never changes
			if psp.tics != -1 {
				psp.tics -= 1;
				if psp.tics == 0 {
					let nextstate = unsafe { (*psp.state).nextstate };
					P_SetPsprite(player, i.into(), nextstate);
				}
			}
		}
	}

	player.psprites[psprnum_t::ps_flash as usize].sx =
		player.psprites[psprnum_t::ps_weapon as usize].sx;
	player.psprites[psprnum_t::ps_flash as usize].sy =
		player.psprites[psprnum_t::ps_weapon as usize].sy;
}
