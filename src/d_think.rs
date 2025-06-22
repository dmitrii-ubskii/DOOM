#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::ffi::c_void;

use crate::{
	d_player::player_t,
	p_ceiling::T_MoveCeiling,
	p_doors::T_VerticalDoor,
	p_enemy::{
		A_BabyMetal, A_BossDeath, A_BrainAwake, A_BrainDie, A_BrainExplode, A_BrainPain,
		A_BrainScream, A_BrainSpit, A_BruisAttack, A_BspiAttack, A_CPosAttack, A_CPosRefire,
		A_Chase, A_CloseShotgun2, A_CyberAttack, A_Explode, A_FaceTarget, A_Fall, A_FatAttack1,
		A_FatAttack2, A_FatAttack3, A_FatRaise, A_Fire, A_FireCrackle, A_HeadAttack, A_Hoof,
		A_KeenDie, A_LoadShotgun2, A_Look, A_Metal, A_OpenShotgun2, A_Pain, A_PainAttack,
		A_PainDie, A_PlayerScream, A_PosAttack, A_SPosAttack, A_SargAttack, A_Scream, A_SkelFist,
		A_SkelMissile, A_SkelWhoosh, A_SkullAttack, A_SpawnFly, A_SpawnSound, A_SpidRefire,
		A_StartFire, A_Tracer, A_TroopAttack, A_VileAttack, A_VileChase, A_VileStart, A_VileTarget,
		A_XScream,
	},
	p_floor::T_MoveFloor,
	p_lights::{T_FireFlicker, T_Glow, T_LightFlash, T_StrobeFlash},
	p_mobj::mobj_t,
	p_plats::T_PlatRaise,
	p_pspr::{
		A_BFGSpray, A_BFGsound, A_CheckReload, A_FireBFG, A_FireCGun, A_FireMissile, A_FirePistol,
		A_FirePlasma, A_FireShotgun, A_FireShotgun2, A_GunFlash, A_Light0, A_Light1, A_Light2,
		A_Lower, A_Punch, A_Raise, A_ReFire, A_Saw, A_WeaponReady, pspdef_t,
	},
};

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum think_t {
	null,
	mobj,
	// p_pspr
	A_Light0,
	A_WeaponReady,
	A_Lower,
	A_Raise,
	A_Punch,
	A_ReFire,
	A_FirePistol,
	A_Light1,
	A_FireShotgun,
	A_Light2,
	A_FireShotgun2,
	A_CheckReload,
	A_OpenShotgun2,
	A_LoadShotgun2,
	A_CloseShotgun2,
	A_FireCGun,
	A_GunFlash,
	A_FireMissile,
	A_Saw,
	A_FirePlasma,
	A_BFGsound,
	A_FireBFG,
	// p_enemy
	A_BFGSpray,
	A_Explode,
	A_Pain,
	A_PlayerScream,
	A_Fall,
	A_XScream,
	A_Look,
	A_Chase,
	A_FaceTarget,
	A_PosAttack,
	A_Scream,
	A_SPosAttack,
	A_VileChase,
	A_VileStart,
	A_VileTarget,
	A_VileAttack,
	A_StartFire,
	A_Fire,
	A_FireCrackle,
	A_Tracer,
	A_SkelWhoosh,
	A_SkelFist,
	A_SkelMissile,
	A_FatRaise,
	A_FatAttack1,
	A_FatAttack2,
	A_FatAttack3,
	A_BossDeath,
	A_CPosAttack,
	A_CPosRefire,
	A_TroopAttack,
	A_SargAttack,
	A_HeadAttack,
	A_BruisAttack,
	A_SkullAttack,
	A_Metal,
	A_SpidRefire,
	A_BabyMetal,
	A_BspiAttack,
	A_Hoof,
	A_CyberAttack,
	A_PainAttack,
	A_PainDie,
	A_KeenDie,
	A_BrainPain,
	A_BrainScream,
	A_BrainDie,
	A_BrainAwake,
	A_BrainSpit,
	A_SpawnSound,
	A_SpawnFly,
	A_BrainExplode,
	// other
	T_MoveFloor,
	T_PlatRaise,
	T_MoveCeiling,
	T_Glow,
	T_VerticalDoor,
	T_LightFlash,
	T_FireFlicker,
	T_StrobeFlash,
}

impl think_t {
	/// Returns `true` if the think t is [`null`].
	///
	/// [`null`]: think_t::null
	#[must_use]
	pub fn is_null(self) -> bool {
		matches!(self, Self::null)
	}

	/// Returns `true` if the think t is [`mobj`].
	///
	/// [`mobj`]: think_t::mobj
	#[must_use]
	pub fn is_mobj(self) -> bool {
		matches!(self, Self::mobj)
	}

	pub fn as_ac_pspr(self) -> Option<fn(&mut player_t, &mut pspdef_t)> {
		match self {
			think_t::A_Light0 => Some(A_Light0),
			think_t::A_WeaponReady => Some(A_WeaponReady),
			think_t::A_Lower => Some(A_Lower),
			think_t::A_Raise => Some(A_Raise),
			think_t::A_Punch => Some(A_Punch),
			think_t::A_ReFire => Some(A_ReFire),
			think_t::A_FirePistol => Some(A_FirePistol),
			think_t::A_Light1 => Some(A_Light1),
			think_t::A_FireShotgun => Some(A_FireShotgun),
			think_t::A_Light2 => Some(A_Light2),
			think_t::A_FireShotgun2 => Some(A_FireShotgun2),
			think_t::A_CheckReload => Some(A_CheckReload),
			think_t::A_OpenShotgun2 => Some(A_OpenShotgun2),
			think_t::A_LoadShotgun2 => Some(A_LoadShotgun2),
			think_t::A_CloseShotgun2 => Some(A_CloseShotgun2),
			think_t::A_FireCGun => Some(A_FireCGun),
			think_t::A_GunFlash => Some(A_GunFlash),
			think_t::A_FireMissile => Some(A_FireMissile),
			think_t::A_Saw => Some(A_Saw),
			think_t::A_FirePlasma => Some(A_FirePlasma),
			think_t::A_BFGsound => Some(A_BFGsound),
			think_t::A_FireBFG => Some(A_FireBFG),
			_ => None,
		}
	}

	pub fn as_ac_mobj(self) -> Option<fn(&mut mobj_t)> {
		match self {
			think_t::A_BFGSpray => Some(A_BFGSpray),
			think_t::A_Explode => Some(A_Explode),
			think_t::A_Pain => Some(A_Pain),
			think_t::A_PlayerScream => Some(A_PlayerScream),
			think_t::A_Fall => Some(A_Fall),
			think_t::A_XScream => Some(A_XScream),
			think_t::A_Look => Some(A_Look),
			think_t::A_Chase => Some(A_Chase),
			think_t::A_FaceTarget => Some(A_FaceTarget),
			think_t::A_PosAttack => Some(A_PosAttack),
			think_t::A_Scream => Some(A_Scream),
			think_t::A_SPosAttack => Some(A_SPosAttack),
			think_t::A_VileChase => Some(A_VileChase),
			think_t::A_VileStart => Some(A_VileStart),
			think_t::A_VileTarget => Some(A_VileTarget),
			think_t::A_VileAttack => Some(A_VileAttack),
			think_t::A_StartFire => Some(A_StartFire),
			think_t::A_Fire => Some(A_Fire),
			think_t::A_FireCrackle => Some(A_FireCrackle),
			think_t::A_Tracer => Some(A_Tracer),
			think_t::A_SkelWhoosh => Some(A_SkelWhoosh),
			think_t::A_SkelFist => Some(A_SkelFist),
			think_t::A_SkelMissile => Some(A_SkelMissile),
			think_t::A_FatRaise => Some(A_FatRaise),
			think_t::A_FatAttack1 => Some(A_FatAttack1),
			think_t::A_FatAttack2 => Some(A_FatAttack2),
			think_t::A_FatAttack3 => Some(A_FatAttack3),
			think_t::A_BossDeath => Some(A_BossDeath),
			think_t::A_CPosAttack => Some(A_CPosAttack),
			think_t::A_CPosRefire => Some(A_CPosRefire),
			think_t::A_TroopAttack => Some(A_TroopAttack),
			think_t::A_SargAttack => Some(A_SargAttack),
			think_t::A_HeadAttack => Some(A_HeadAttack),
			think_t::A_BruisAttack => Some(A_BruisAttack),
			think_t::A_SkullAttack => Some(A_SkullAttack),
			think_t::A_Metal => Some(A_Metal),
			think_t::A_SpidRefire => Some(A_SpidRefire),
			think_t::A_BabyMetal => Some(A_BabyMetal),
			think_t::A_BspiAttack => Some(A_BspiAttack),
			think_t::A_Hoof => Some(A_Hoof),
			think_t::A_CyberAttack => Some(A_CyberAttack),
			think_t::A_PainAttack => Some(A_PainAttack),
			think_t::A_PainDie => Some(A_PainDie),
			think_t::A_KeenDie => Some(A_KeenDie),
			think_t::A_BrainPain => Some(A_BrainPain),
			think_t::A_BrainScream => Some(A_BrainScream),
			think_t::A_BrainDie => Some(A_BrainDie),
			think_t::A_BrainAwake => Some(A_BrainAwake),
			think_t::A_BrainSpit => Some(A_BrainSpit),
			think_t::A_SpawnSound => Some(A_SpawnSound),
			think_t::A_SpawnFly => Some(A_SpawnFly),
			think_t::A_BrainExplode => Some(A_BrainExplode),
			_ => None,
		}
	}

	pub fn as_acp1(self) -> Option<fn(*mut c_void)> {
		match self {
			think_t::T_MoveFloor => Some(|p| unsafe { T_MoveFloor(&mut *p.cast()) }),
			think_t::T_PlatRaise => Some(|p| unsafe { T_PlatRaise(&mut *p.cast()) }),
			think_t::T_MoveCeiling => Some(|p| unsafe { T_MoveCeiling(&mut *p.cast()) }),
			think_t::T_Glow => Some(|p| unsafe { T_Glow(&mut *p.cast()) }),
			think_t::T_VerticalDoor => Some(|p| unsafe { T_VerticalDoor(&mut *p.cast()) }),
			think_t::T_LightFlash => Some(|p| unsafe { T_LightFlash(&mut *p.cast()) }),
			think_t::T_FireFlicker => Some(|p| unsafe { T_FireFlicker(&mut *p.cast()) }),
			think_t::T_StrobeFlash => Some(|p| unsafe { T_StrobeFlash(&mut *p.cast()) }),
			_ => None,
		}
	}
}

// Doubly linked list of actors.
#[repr(C)]
pub struct thinker_t {
	pub prev: *mut thinker_t,
	pub next: *mut thinker_t,
	pub function: think_t,
}
