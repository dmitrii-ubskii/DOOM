#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

// DESCRIPTION:
//	Handling interactions (i.e., collisions).

use std::{ffi::c_void, ptr::null_mut};

use crate::{
	d_englsh::{
		GOTARMBONUS, GOTARMOR, GOTBACKPACK, GOTBERSERK, GOTBFG9000, GOTBLUECARD, GOTBLUESKUL,
		GOTCELL, GOTCELLBOX, GOTCHAINGUN, GOTCHAINSAW, GOTCLIP, GOTCLIPBOX, GOTHTHBONUS, GOTINVIS,
		GOTINVUL, GOTLAUNCHER, GOTMAP, GOTMEDIKIT, GOTMEDINEED, GOTMEGA, GOTMSPHERE, GOTPLASMA,
		GOTREDCARD, GOTREDSKULL, GOTROCKBOX, GOTROCKET, GOTSHELLBOX, GOTSHELLS, GOTSHOTGUN,
		GOTSHOTGUN2, GOTSTIM, GOTSUIT, GOTSUPER, GOTVISOR, GOTYELWCARD, GOTYELWSKUL,
	},
	d_items::weaponinfo,
	d_player::{cheat_t, player_t, playerstate_t},
	doomdef::{
		GameMode_t, INFRATICS, INVISTICS, INVULNTICS, IRONTICS, ammotype_t, card_t, powertype_t,
		skill_t, weapontype_t,
	},
	doomstat::gamemode,
	g_game::{consoleplayer, deathmatch, gameskill, netgame, players},
	i_system::{I_Error, I_Tactile},
	info::{mobjtype_t, spritenum_t, statenum_t, states},
	m_fixed::{FRACUNIT, FixedMul, fixed_t},
	m_random::P_Random,
	p_local::{BASETHRESHOLD, MAXHEALTH, ONFLOORZ},
	p_mobj::{
		MF_CORPSE, MF_COUNTITEM, MF_COUNTKILL, MF_DROPOFF, MF_DROPPED, MF_FLOAT, MF_JUSTHIT,
		MF_NOCLIP, MF_NOGRAVITY, MF_SHADOW, MF_SHOOTABLE, MF_SKULLFLY, MF_SOLID, mobj_t,
	},
	sounds::sfxenum_t,
	tables::{ANG180, ANGLETOFINESHIFT, finecos, finesine},
};

type int = i32;
type boolean = i32;

const BONUSADD: usize = 6;

// a weapon is found with two clip loads,
// a big item has five clip loads
#[unsafe(no_mangle)]
pub static mut maxammo: [int; ammotype_t::NUMAMMO as usize] = [200, 50, 300, 50];
static mut clipammo: [int; ammotype_t::NUMAMMO as usize] = [10, 4, 20, 1];

// GET STUFF

// P_GiveAmmo
// Num is the number of clip loads,
// not the individual count (0= 1/2 clip).
// Returns false if the ammo can't be picked up at all
fn P_GiveAmmo(player: *mut player_t, ammo: ammotype_t, mut num: int) -> boolean {
	unsafe {
		if ammo == ammotype_t::am_noammo {
			return 0;
		}

		// if (ammo < 0 || ammo > NUMAMMO) {
		// 	I_Error("P_GiveAmmo: bad type %i", ammo);
		// }

		if (*player).ammo[ammo as usize] == (*player).maxammo[ammo as usize] {
			return 0;
		}

		if num != 0 {
			num *= clipammo[ammo as usize];
		} else {
			num = clipammo[ammo as usize] / 2;
		}

		if gameskill == skill_t::sk_baby || gameskill == skill_t::sk_nightmare {
			// give double ammo in trainer mode,
			// you'll need in nightmare
			num <<= 1;
		}

		let oldammo = (*player).ammo[ammo as usize];
		(*player).ammo[ammo as usize] += num;

		if (*player).ammo[ammo as usize] > (*player).maxammo[ammo as usize] {
			(*player).ammo[ammo as usize] = (*player).maxammo[ammo as usize];
		}

		// If non zero ammo,
		// don't change up weapons,
		// player was lower on purpose.
		if oldammo != 0 {
			return 1;
		}

		// We were down to zero,
		// so select a new weapon.
		// Preferences are not user selectable.
		match (ammo, (*player).readyweapon) {
			(ammotype_t::am_clip, weapontype_t::wp_fist) => {
				if (*player).weaponowned[weapontype_t::wp_chaingun as usize] != 0 {
					(*player).pendingweapon = weapontype_t::wp_chaingun;
				} else {
					(*player).pendingweapon = weapontype_t::wp_pistol;
				}
			}
			(ammotype_t::am_shell, weapontype_t::wp_fist | weapontype_t::wp_pistol) => {
				if (*player).weaponowned[weapontype_t::wp_shotgun as usize] != 0 {
					(*player).pendingweapon = weapontype_t::wp_shotgun;
				}
			}
			(ammotype_t::am_cell, weapontype_t::wp_fist | weapontype_t::wp_pistol) => {
				if (*player).weaponowned[weapontype_t::wp_plasma as usize] != 0 {
					(*player).pendingweapon = weapontype_t::wp_plasma;
				}
			}
			(ammotype_t::am_misl, weapontype_t::wp_fist) => {
				if (*player).weaponowned[weapontype_t::wp_missile as usize] != 0 {
					(*player).pendingweapon = weapontype_t::wp_missile;
				}
			}
			_ => (),
		}

		1
	}
}

unsafe extern "C" {
	fn S_StartSound(origin: *mut c_void, sound_id: sfxenum_t);
}

// P_GiveWeapon
// The weapon name may have a MF_DROPPED flag ored in.
fn P_GiveWeapon(player: *mut player_t, weapon: weapontype_t, dropped: boolean) -> boolean {
	unsafe {
		if netgame != 0 && deathmatch != 2 && dropped == 0 {
			// leave placed weapons forever on net games
			if (*player).weaponowned[weapon as usize] != 0 {
				return 0;
			}

			(*player).bonuscount += BONUSADD;
			(*player).weaponowned[weapon as usize] = 1;

			if deathmatch != 0 {
				P_GiveAmmo(player, weaponinfo[weapon as usize].ammo, 5);
			} else {
				P_GiveAmmo(player, weaponinfo[weapon as usize].ammo, 2);
			}
			(*player).pendingweapon = weapon;

			if std::ptr::eq(player, &raw mut players[consoleplayer]) {
				S_StartSound(null_mut(), sfxenum_t::sfx_wpnup);
			}
			return 0;
		}

		let gaveammo;
		if weaponinfo[weapon as usize].ammo != ammotype_t::am_noammo {
			// give one clip with a dropped weapon,
			// two clips with a found weapon
			if dropped != 0 {
				gaveammo = P_GiveAmmo(player, weaponinfo[weapon as usize].ammo, 1);
			} else {
				gaveammo = P_GiveAmmo(player, weaponinfo[weapon as usize].ammo, 2);
			}
		} else {
			gaveammo = 0;
		}

		let gaveweapon;
		if (*player).weaponowned[weapon as usize] != 0 {
			gaveweapon = 0;
		} else {
			gaveweapon = 1;
			(*player).weaponowned[weapon as usize] = 1;
			(*player).pendingweapon = weapon;
		}

		(gaveweapon != 0 || gaveammo != 0) as boolean
	}
}

// P_GiveBody
// Returns false if the body isn't needed at all
fn P_GiveBody(player: &mut player_t, num: int) -> boolean {
	unsafe {
		if player.health >= MAXHEALTH {
			return 0;
		}
		player.health += num;
		if player.health > MAXHEALTH {
			player.health = MAXHEALTH;
		}
		(*player.mo).health = player.health;
		1
	}
}

// P_GiveArmor
// Returns false if the armor is worse
// than the current armor.
fn P_GiveArmor(player: &mut player_t, armortype: int) -> boolean {
	let hits = armortype * 100;
	if player.armorpoints >= hits {
		0 // don't pick up
	} else {
		player.armortype = armortype;
		player.armorpoints = hits;
		1
	}
}

// P_GiveCard
fn P_GiveCard(player: &mut player_t, card: card_t) {
	if player.cards[card as usize] != 0 {
		return;
	}

	player.bonuscount = BONUSADD;
	player.cards[card as usize] = 1;
}

// P_GivePower
#[unsafe(no_mangle)]
pub extern "C" fn P_GivePower(player: &mut player_t, power: powertype_t) -> boolean {
	match power {
		powertype_t::pw_invulnerability => {
			player.powers[power as usize] = INVULNTICS;
			return 1;
		}
		powertype_t::pw_invisibility => {
			player.powers[power as usize] = INVISTICS;
			unsafe {
				(*player.mo).flags |= MF_SHADOW;
			}
			return 1;
		}
		powertype_t::pw_infrared => {
			player.powers[power as usize] = INFRATICS;
			return 1;
		}
		powertype_t::pw_ironfeet => {
			player.powers[power as usize] = IRONTICS;
			return 1;
		}

		powertype_t::pw_strength => {
			P_GiveBody(player, 100);
			player.powers[power as usize] = 1;
			return 1;
		}

		powertype_t::pw_allmap => {
			if player.powers[power as usize] != 0 {
				return 0; // already got it
			}

			player.powers[power as usize] = 1;
		}

		_ => unreachable!(),
	}

	1
}

unsafe extern "C" {
	fn P_RemoveMobj(thing: *mut mobj_t);
}

// P_TouchSpecialThing
#[unsafe(no_mangle)]
pub extern "C" fn P_TouchSpecialThing(special: &mut mobj_t, toucher: &mut mobj_t) {
	let delta = special.z - toucher.z;

	if delta > toucher.height || delta < -8 * FRACUNIT {
		// out of reach
		return;
	}

	let mut sound = sfxenum_t::sfx_itemup;
	let player = unsafe { &mut *toucher.player };

	// Dead thing touching.
	// Can happen with a sliding player corpse.
	if toucher.health <= 0 {
		return;
	}

	// Identify by sprite.
	match special.sprite {
		// armor
		spritenum_t::SPR_ARM1 => {
			if P_GiveArmor(player, 1) == 0 {
				return;
			}
			player.message = GOTARMOR;
		}

		spritenum_t::SPR_ARM2 => {
			if P_GiveArmor(player, 2) == 0 {
				return;
			}
			player.message = GOTMEGA;
		}

		// bonus items
		spritenum_t::SPR_BON1 => {
			player.health += 1; // can go over 100%
			if player.health > 200 {
				player.health = 200;
			}
			unsafe {
				(*player.mo).health = player.health;
			}
			player.message = GOTHTHBONUS;
		}

		spritenum_t::SPR_BON2 => {
			player.armorpoints += 1; // can go over 100%
			if player.armorpoints > 200 {
				player.armorpoints = 200;
			}
			if player.armortype == 0 {
				player.armortype = 1;
			}
			player.message = GOTARMBONUS;
		}

		spritenum_t::SPR_SOUL => {
			player.health += 100;
			if player.health > 200 {
				player.health = 200;
			}
			unsafe {
				(*player.mo).health = player.health;
			}
			player.message = GOTSUPER;
			sound = sfxenum_t::sfx_getpow;
		}

		spritenum_t::SPR_MEGA => unsafe {
			if gamemode != GameMode_t::commercial {
				return;
			}
			player.health = 200;
			(*player.mo).health = player.health;
			P_GiveArmor(player, 2);
			player.message = GOTMSPHERE;
			sound = sfxenum_t::sfx_getpow;
		},

		// cards
		// leave cards for everyone
		spritenum_t::SPR_BKEY => unsafe {
			if (player.cards[card_t::it_bluecard as usize]) == 0 {
				player.message = GOTBLUECARD;
			}
			P_GiveCard(player, card_t::it_bluecard);
			if netgame != 0 {
				return;
			}
		},

		spritenum_t::SPR_YKEY => unsafe {
			if player.cards[card_t::it_yellowcard as usize] == 0 {
				player.message = GOTYELWCARD;
			}
			P_GiveCard(player, card_t::it_yellowcard);
			if netgame != 0 {
				return;
			}
		},

		spritenum_t::SPR_RKEY => unsafe {
			if (player.cards[card_t::it_redcard as usize]) == 0 {
				player.message = GOTREDCARD;
			}
			P_GiveCard(player, card_t::it_redcard);
			if netgame != 0 {
				return;
			}
		},

		spritenum_t::SPR_BSKU => unsafe {
			if player.cards[card_t::it_blueskull as usize] == 0 {
				player.message = GOTBLUESKUL;
			}
			P_GiveCard(player, card_t::it_blueskull);
			if netgame != 0 {
				return;
			}
		},

		spritenum_t::SPR_YSKU => unsafe {
			if player.cards[card_t::it_yellowskull as usize] == 0 {
				player.message = GOTYELWSKUL;
			}
			P_GiveCard(player, card_t::it_yellowskull);
			if netgame != 0 {
				return;
			}
		},

		spritenum_t::SPR_RSKU => unsafe {
			if player.cards[card_t::it_redskull as usize] == 0 {
				player.message = GOTREDSKULL;
			}
			P_GiveCard(player, card_t::it_redskull);
			if netgame != 0 {
				return;
			}
		},

		// medikits, heals
		spritenum_t::SPR_STIM => {
			if P_GiveBody(player, 10) == 0 {
				return;
			}
			player.message = GOTSTIM;
		}

		spritenum_t::SPR_MEDI => {
			if P_GiveBody(player, 25) == 0 {
				return;
			}

			if player.health < 25 {
				player.message = GOTMEDINEED;
			} else {
				player.message = GOTMEDIKIT;
			}
		}

		// power ups
		spritenum_t::SPR_PINV => {
			if P_GivePower(player, powertype_t::pw_invulnerability) == 0 {
				return;
			}
			player.message = GOTINVUL;
			sound = sfxenum_t::sfx_getpow;
		}

		spritenum_t::SPR_PSTR => {
			if P_GivePower(player, powertype_t::pw_strength) == 0 {
				return;
			}
			player.message = GOTBERSERK;
			if player.readyweapon != weapontype_t::wp_fist {
				player.pendingweapon = weapontype_t::wp_fist;
			}
			sound = sfxenum_t::sfx_getpow;
		}

		spritenum_t::SPR_PINS => {
			if P_GivePower(player, powertype_t::pw_invisibility) == 0 {
				return;
			}
			player.message = GOTINVIS;
			sound = sfxenum_t::sfx_getpow;
		}

		spritenum_t::SPR_SUIT => {
			if P_GivePower(player, powertype_t::pw_ironfeet) == 0 {
				return;
			}
			player.message = GOTSUIT;
			sound = sfxenum_t::sfx_getpow;
		}

		spritenum_t::SPR_PMAP => {
			if P_GivePower(player, powertype_t::pw_allmap) == 0 {
				return;
			}
			player.message = GOTMAP;
			sound = sfxenum_t::sfx_getpow;
		}

		spritenum_t::SPR_PVIS => {
			if P_GivePower(player, powertype_t::pw_infrared) == 0 {
				return;
			}
			player.message = GOTVISOR;
			sound = sfxenum_t::sfx_getpow;
		}

		// ammo
		spritenum_t::SPR_CLIP => {
			let num = if special.flags & MF_DROPPED != 0 { 0 } else { 1 };
			if P_GiveAmmo(player, ammotype_t::am_clip, num) == 0 {
				return;
			}
			player.message = GOTCLIP;
		}

		spritenum_t::SPR_AMMO => {
			if P_GiveAmmo(player, ammotype_t::am_clip, 5) == 0 {
				return;
			}
			player.message = GOTCLIPBOX;
		}

		spritenum_t::SPR_ROCK => {
			if P_GiveAmmo(player, ammotype_t::am_misl, 1) == 0 {
				return;
			}
			player.message = GOTROCKET;
		}

		spritenum_t::SPR_BROK => {
			if P_GiveAmmo(player, ammotype_t::am_misl, 5) == 0 {
				return;
			}
			player.message = GOTROCKBOX;
		}

		spritenum_t::SPR_CELL => {
			if P_GiveAmmo(player, ammotype_t::am_cell, 1) == 0 {
				return;
			}
			player.message = GOTCELL;
		}

		spritenum_t::SPR_CELP => {
			if P_GiveAmmo(player, ammotype_t::am_cell, 5) == 0 {
				return;
			}
			player.message = GOTCELLBOX;
		}

		spritenum_t::SPR_SHEL => {
			if P_GiveAmmo(player, ammotype_t::am_shell, 1) == 0 {
				return;
			}
			player.message = GOTSHELLS;
		}

		spritenum_t::SPR_SBOX => {
			if P_GiveAmmo(player, ammotype_t::am_shell, 5) == 0 {
				return;
			}
			player.message = GOTSHELLBOX;
		}

		spritenum_t::SPR_BPAK => {
			if player.backpack == 0 {
				for i in 0..ammotype_t::NUMAMMO as usize {
					player.maxammo[i] *= 2;
				}
				player.backpack = 1;
			}
			for i in 0..ammotype_t::NUMAMMO as u8 {
				P_GiveAmmo(player, ammotype_t::from(i), 1);
			}
			player.message = GOTBACKPACK;
		}

		// weapons
		spritenum_t::SPR_BFUG => {
			if P_GiveWeapon(player, weapontype_t::wp_bfg, 0) == 0 {
				return;
			}
			player.message = GOTBFG9000;
			sound = sfxenum_t::sfx_wpnup;
		}

		spritenum_t::SPR_MGUN => {
			if P_GiveWeapon(
				player,
				weapontype_t::wp_chaingun,
				(special.flags & MF_DROPPED) as boolean,
			) == 0
			{
				return;
			}
			player.message = GOTCHAINGUN;
			sound = sfxenum_t::sfx_wpnup;
		}

		spritenum_t::SPR_CSAW => {
			if P_GiveWeapon(player, weapontype_t::wp_chainsaw, 0) == 0 {
				return;
			}
			player.message = GOTCHAINSAW;
			sound = sfxenum_t::sfx_wpnup;
		}

		spritenum_t::SPR_LAUN => {
			if P_GiveWeapon(player, weapontype_t::wp_missile, 0) == 0 {
				return;
			}
			player.message = GOTLAUNCHER;
			sound = sfxenum_t::sfx_wpnup;
		}

		spritenum_t::SPR_PLAS => {
			if P_GiveWeapon(player, weapontype_t::wp_plasma, 0) == 0 {
				return;
			}
			player.message = GOTPLASMA;
			sound = sfxenum_t::sfx_wpnup;
		}

		spritenum_t::SPR_SHOT => {
			if P_GiveWeapon(
				player,
				weapontype_t::wp_shotgun,
				(special.flags & MF_DROPPED) as boolean,
			) == 0
			{
				return;
			}
			player.message = GOTSHOTGUN;
			sound = sfxenum_t::sfx_wpnup;
		}

		spritenum_t::SPR_SGN2 => {
			if P_GiveWeapon(
				player,
				weapontype_t::wp_supershotgun,
				(special.flags & MF_DROPPED) as boolean,
			) == 0
			{
				return;
			}
			player.message = GOTSHOTGUN2;
			sound = sfxenum_t::sfx_wpnup;
		}

		_ => unsafe {
			I_Error(c"P_SpecialThing: Unknown gettable thing".as_ptr());
		},
	}

	if special.flags & MF_COUNTITEM != 0 {
		player.itemcount += 1;
	}
	unsafe { P_RemoveMobj(special) };
	player.bonuscount += BONUSADD;
	unsafe {
		if std::ptr::eq(player, &raw const players[consoleplayer]) {
			S_StartSound(null_mut(), sound);
		}
	}
}

unsafe extern "C" {
	static mut automapactive: boolean;
	fn P_DropWeapon(player: *mut player_t);
	fn AM_Stop();
	fn P_SetMobjState(mobj: *mut mobj_t, state: statenum_t) -> boolean;
	fn P_SpawnMobj(x: fixed_t, y: fixed_t, floorheight: i32, mt_tfog: mobjtype_t) -> *mut mobj_t;
}

// KillMobj
fn P_KillMobj(source: *mut mobj_t, target: &mut mobj_t) {
	target.flags &= !(MF_SHOOTABLE | MF_FLOAT | MF_SKULLFLY);

	if target.ty != mobjtype_t::MT_SKULL {
		target.flags &= !MF_NOGRAVITY;
	}

	target.flags |= MF_CORPSE | MF_DROPOFF;
	target.height >>= 2;
	unsafe {
		if !source.is_null() && !(*source).player.is_null() {
			// count for intermission
			if target.flags & MF_COUNTKILL != 0 {
				(*(*source).player).killcount += 1;
			}

			if !target.player.is_null() {
				(*(*source).player).frags
					[target.player.offset_from(&raw const players[0]) as usize] += 1;
			}
		} else if netgame == 0 && (target.flags & MF_COUNTKILL != 0) {
			// count all monster deaths,
			// even those caused by other monsters
			players[0].killcount += 1;
		}
	}

	if !target.player.is_null() {
		unsafe {
			// count environment kills against you
			if source.is_null() {
				(*target.player).frags
					[target.player.offset_from(&raw const players[0]) as usize] += 1;
			}

			target.flags &= !MF_SOLID;
			(*target.player).playerstate = playerstate_t::PST_DEAD;
			P_DropWeapon(target.player);

			if std::ptr::eq(target.player, &raw const players[consoleplayer]) && automapactive != 0
			{
				// don't die in auto map,
				// switch view prior to dying
				AM_Stop();
			}
		}
	}

	unsafe {
		if target.health < -(*target.info).spawnhealth
			&& (*target.info).xdeathstate != statenum_t::S_NULL
		{
			P_SetMobjState(target, (*target.info).xdeathstate);
		} else {
			P_SetMobjState(target, (*target.info).deathstate);
		}
	}

	target.tics -= P_Random() & 3;

	if target.tics < 1 {
		target.tics = 1;
	}

	//	I_StartSound (&(*actor).r, (*actor).(*info).deathsound);

	// Drop stuff.
	// This determines the kind of object spawned
	// during the death frame of a thing.
	let item = match target.ty {
		mobjtype_t::MT_WOLFSS | mobjtype_t::MT_POSSESSED => mobjtype_t::MT_CLIP,
		mobjtype_t::MT_SHOTGUY => mobjtype_t::MT_SHOTGUN,
		mobjtype_t::MT_CHAINGUY => mobjtype_t::MT_CHAINGUN,
		_ => return,
	};

	unsafe {
		let mo = P_SpawnMobj(target.x, target.y, ONFLOORZ, item);
		(*mo).flags |= MF_DROPPED; // special versions of items
	}
}

unsafe extern "C" {
	fn R_PointToAngle2(x_1: i32, y_1: i32, x_2: i32, y_2: i32) -> u32;
}

// P_DamageMobj
// Damages both enemies and players
// "inflictor" is the thing that caused the damage
//  creature or missile, can be NULL (slime, etc)
// "source" is the thing to target after taking damage
//  creature or NULL
// Source and inflictor are the same for melee attacks.
// Source can be NULL for slime, barrel explosions
// and other environmental stuff.
#[unsafe(no_mangle)]
pub extern "C" fn P_DamageMobj(
	target: &mut mobj_t,
	inflictor: *mut mobj_t,
	source: *mut mobj_t,
	mut damage: i32,
) {
	unsafe {
		if target.flags & MF_SHOOTABLE == 0 {
			return; // shouldn't happen...
		}

		if target.health <= 0 {
			return;
		}

		if target.flags & MF_SKULLFLY != 0 {
			target.momx = 0;
			target.momy = 0;
			target.momz = 0;
		}

		let player = target.player;
		if !player.is_null() && gameskill == skill_t::sk_baby {
			damage >>= 1; // take half damage in trainer mode
		}

		// Some close combat weapons should not
		// inflict thrust and push the victim out of reach,
		// thus kick away unless using the chainsaw.
		if !inflictor.is_null()
			&& target.flags & MF_NOCLIP == 0
			&& (source.is_null()
				|| (*source).player.is_null()
				|| (*(*source).player).readyweapon != weapontype_t::wp_chainsaw)
		{
			let mut ang = R_PointToAngle2((*inflictor).x, (*inflictor).y, target.x, target.y);

			let mut thrust = damage * (FRACUNIT >> 3) * 100 / (*target.info).mass;

			// make fall forwards sometimes
			if damage < 40
				&& damage > target.health
				&& target.z - (*inflictor).z > 64 * FRACUNIT
				&& P_Random() & 1 == 1
			{
				ang += ANG180;
				thrust *= 4;
			}

			ang >>= ANGLETOFINESHIFT;
			let ang = ang as usize;
			target.momx += FixedMul(thrust, finecos(ang));
			target.momy += FixedMul(thrust, finesine[ang]);
		}

		// player specific
		if !player.is_null() {
			// end of game hell hack
			if (*(*target.subsector).sector).special == 11 && damage >= target.health {
				damage = target.health - 1;
			}

			// Below certain threshold,
			// ignore damage in GOD mode, or with INVUL power.
			if damage < 1000
				&& ((*player).cheats & cheat_t::CF_GODMODE as i32 != 0
					|| (*player).powers[powertype_t::pw_invulnerability as usize] != 0)
			{
				return;
			}

			if (*player).armortype != 0 {
				let mut saved = if (*player).armortype == 1 { damage / 3 } else { damage / 2 };

				if (*player).armorpoints <= saved {
					// armor is used up
					saved = (*player).armorpoints;
					(*player).armortype = 0;
				}
				(*player).armorpoints -= saved;
				damage -= saved;
			}
			(*player).health -= damage; // mirror mobj health here for Dave
			if (*player).health < 0 {
				(*player).health = 0;
			}

			(*player).attacker = source;
			(*player).damagecount += damage; // add damage after armor / invuln

			if (*player).damagecount > 100 {
				(*player).damagecount = 100; // teleport stomp does 10k points...
			}

			let temp = if damage < 100 { damage } else { 100 };

			if std::ptr::eq(player, &raw const players[consoleplayer]) {
				I_Tactile(40, 10, 40 + temp * 2);
			}
		}

		// do the damage
		target.health -= damage;
		if target.health <= 0 {
			P_KillMobj(source, target);
			return;
		}

		if P_Random() < (*target.info).painchance && target.flags & MF_SKULLFLY == 0 {
			target.flags |= MF_JUSTHIT; // fight back!

			P_SetMobjState(target, (*target.info).painstate);
		}

		target.reactiontime = 0; // we're awake now...

		if (target.threshold == 0 || target.ty == mobjtype_t::MT_VILE)
			&& !source.is_null()
			&& source != target
			&& (*source).ty != mobjtype_t::MT_VILE
		{
			// if not intent on another player,
			// chase after this one
			target.target = source;
			target.threshold = BASETHRESHOLD;
			if std::ptr::eq(target.state, &raw const states[(*target.info).spawnstate as usize])
				&& (*target.info).seestate != statenum_t::S_NULL
			{
				P_SetMobjState(target, (*target.info).seestate);
			}
		}
	}
}
