#![allow(non_camel_case_types, non_upper_case_globals)]

use crate::{
	doomdef::{NUMWEAPONS, ammotype_t},
	info::statenum_t,
};

// Weapon info: sprite frames, ammunition use.
#[repr(C)]
pub(crate) struct weaponinfo_t {
	pub(crate) ammo: Option<ammotype_t>,
	pub(crate) upstate: statenum_t,
	pub(crate) downstate: statenum_t,
	pub(crate) readystate: statenum_t,
	pub(crate) atkstate: statenum_t,
	pub(crate) flashstate: statenum_t,
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
pub(crate) static weaponinfo: [weaponinfo_t; NUMWEAPONS] = [
	weaponinfo_t {
		// fist
		ammo: None,
		upstate: statenum_t::S_PUNCHUP,
		downstate: statenum_t::S_PUNCHDOWN,
		readystate: statenum_t::S_PUNCH,
		atkstate: statenum_t::S_PUNCH1,
		flashstate: statenum_t::S_NULL,
	},
	weaponinfo_t {
		// pistol
		ammo: Some(ammotype_t::am_clip),
		upstate: statenum_t::S_PISTOLUP,
		downstate: statenum_t::S_PISTOLDOWN,
		readystate: statenum_t::S_PISTOL,
		atkstate: statenum_t::S_PISTOL1,
		flashstate: statenum_t::S_PISTOLFLASH,
	},
	weaponinfo_t {
		// shotgun
		ammo: Some(ammotype_t::am_shell),
		upstate: statenum_t::S_SGUNUP,
		downstate: statenum_t::S_SGUNDOWN,
		readystate: statenum_t::S_SGUN,
		atkstate: statenum_t::S_SGUN1,
		flashstate: statenum_t::S_SGUNFLASH1,
	},
	weaponinfo_t {
		// chaingun
		ammo: Some(ammotype_t::am_clip),
		upstate: statenum_t::S_CHAINUP,
		downstate: statenum_t::S_CHAINDOWN,
		readystate: statenum_t::S_CHAIN,
		atkstate: statenum_t::S_CHAIN1,
		flashstate: statenum_t::S_CHAINFLASH1,
	},
	weaponinfo_t {
		// missile launcher
		ammo: Some(ammotype_t::am_misl),
		upstate: statenum_t::S_MISSILEUP,
		downstate: statenum_t::S_MISSILEDOWN,
		readystate: statenum_t::S_MISSILE,
		atkstate: statenum_t::S_MISSILE1,
		flashstate: statenum_t::S_MISSILEFLASH1,
	},
	weaponinfo_t {
		// plasma rifle
		ammo: Some(ammotype_t::am_cell),
		upstate: statenum_t::S_PLASMAUP,
		downstate: statenum_t::S_PLASMADOWN,
		readystate: statenum_t::S_PLASMA,
		atkstate: statenum_t::S_PLASMA1,
		flashstate: statenum_t::S_PLASMAFLASH1,
	},
	weaponinfo_t {
		// bfg 9000
		ammo: Some(ammotype_t::am_cell),
		upstate: statenum_t::S_BFGUP,
		downstate: statenum_t::S_BFGDOWN,
		readystate: statenum_t::S_BFG,
		atkstate: statenum_t::S_BFG1,
		flashstate: statenum_t::S_BFGFLASH1,
	},
	weaponinfo_t {
		// chainsaw
		ammo: None,
		upstate: statenum_t::S_SAWUP,
		downstate: statenum_t::S_SAWDOWN,
		readystate: statenum_t::S_SAW,
		atkstate: statenum_t::S_SAW1,
		flashstate: statenum_t::S_NULL,
	},
	weaponinfo_t {
		// super shotgun
		ammo: Some(ammotype_t::am_shell),
		upstate: statenum_t::S_DSGUNUP,
		downstate: statenum_t::S_DSGUNDOWN,
		readystate: statenum_t::S_DSGUN,
		atkstate: statenum_t::S_DSGUN1,
		flashstate: statenum_t::S_DSGUNFLASH1,
	},
];
