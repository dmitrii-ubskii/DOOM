#![allow(non_camel_case_types, non_upper_case_globals)]

use crate::{
	doomdef::{ammotype_t, weapontype_t},
	info::statenum_t,
};

// Weapon info: sprite frames, ammunition use.
#[repr(C)]
pub struct weaponinfo_t {
	ammo: ammotype_t,
	upstate: statenum_t,
	downstate: statenum_t,
	readystate: statenum_t,
	atkstate: statenum_t,
	flashstate: statenum_t,
}

// PSPRITE ACTIONS for waepons.
// This struct controls the weapon animations.
//
// Each entry is:
//   ammo/amunition type
//  upstate
//  downstate
// readystate
// atkstate, i.e. attack/fire/hit frame
// flashstate, muzzle flash
#[unsafe(no_mangle)]
pub static weaponinfo: [weaponinfo_t; weapontype_t::NUMWEAPONS as usize] = [
	weaponinfo_t {
		// fist
		ammo: ammotype_t::am_noammo,
		upstate: statenum_t::S_PUNCHUP,
		downstate: statenum_t::S_PUNCHDOWN,
		readystate: statenum_t::S_PUNCH,
		atkstate: statenum_t::S_PUNCH1,
		flashstate: statenum_t::S_NULL,
	},
	weaponinfo_t {
		// pistol
		ammo: ammotype_t::am_clip,
		upstate: statenum_t::S_PISTOLUP,
		downstate: statenum_t::S_PISTOLDOWN,
		readystate: statenum_t::S_PISTOL,
		atkstate: statenum_t::S_PISTOL1,
		flashstate: statenum_t::S_PISTOLFLASH,
	},
	weaponinfo_t {
		// shotgun
		ammo: ammotype_t::am_shell,
		upstate: statenum_t::S_SGUNUP,
		downstate: statenum_t::S_SGUNDOWN,
		readystate: statenum_t::S_SGUN,
		atkstate: statenum_t::S_SGUN1,
		flashstate: statenum_t::S_SGUNFLASH1,
	},
	weaponinfo_t {
		// chaingun
		ammo: ammotype_t::am_clip,
		upstate: statenum_t::S_CHAINUP,
		downstate: statenum_t::S_CHAINDOWN,
		readystate: statenum_t::S_CHAIN,
		atkstate: statenum_t::S_CHAIN1,
		flashstate: statenum_t::S_CHAINFLASH1,
	},
	weaponinfo_t {
		// missile launcher
		ammo: ammotype_t::am_misl,
		upstate: statenum_t::S_MISSILEUP,
		downstate: statenum_t::S_MISSILEDOWN,
		readystate: statenum_t::S_MISSILE,
		atkstate: statenum_t::S_MISSILE1,
		flashstate: statenum_t::S_MISSILEFLASH1,
	},
	weaponinfo_t {
		// plasma rifle
		ammo: ammotype_t::am_cell,
		upstate: statenum_t::S_PLASMAUP,
		downstate: statenum_t::S_PLASMADOWN,
		readystate: statenum_t::S_PLASMA,
		atkstate: statenum_t::S_PLASMA1,
		flashstate: statenum_t::S_PLASMAFLASH1,
	},
	weaponinfo_t {
		// bfg 9000
		ammo: ammotype_t::am_cell,
		upstate: statenum_t::S_BFGUP,
		downstate: statenum_t::S_BFGDOWN,
		readystate: statenum_t::S_BFG,
		atkstate: statenum_t::S_BFG1,
		flashstate: statenum_t::S_BFGFLASH1,
	},
	weaponinfo_t {
		// chainsaw
		ammo: ammotype_t::am_noammo,
		upstate: statenum_t::S_SAWUP,
		downstate: statenum_t::S_SAWDOWN,
		readystate: statenum_t::S_SAW,
		atkstate: statenum_t::S_SAW1,
		flashstate: statenum_t::S_NULL,
	},
	weaponinfo_t {
		// super shotgun
		ammo: ammotype_t::am_shell,
		upstate: statenum_t::S_DSGUNUP,
		downstate: statenum_t::S_DSGUNDOWN,
		readystate: statenum_t::S_DSGUN,
		atkstate: statenum_t::S_DSGUN1,
		flashstate: statenum_t::S_DSGUNFLASH1,
	},
];
