#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::ffi::{c_char, c_void};

use crate::{
	d_think::actionf_t,
	m_fixed::FRACUNIT,
	p_mobj::{
		MF_COUNTITEM, MF_COUNTKILL, MF_DROPOFF, MF_FLOAT, MF_MISSILE, MF_NOBLOCKMAP, MF_NOBLOOD,
		MF_NOCLIP, MF_NOGRAVITY, MF_NOSECTOR, MF_NOTDMATCH, MF_PICKUP, MF_SHADOW, MF_SHOOTABLE,
		MF_SOLID, MF_SPAWNCEILING, MF_SPECIAL,
	},
	sounds::sfxenum_t,
};

#[repr(C)]
#[rustfmt::skip]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spritenum_t {
	SPR_TROO, SPR_SHTG, SPR_PUNG, SPR_PISG, SPR_PISF, SPR_SHTF, SPR_SHT2, SPR_CHGG, SPR_CHGF,
	SPR_MISG, SPR_MISF, SPR_SAWG, SPR_PLSG, SPR_PLSF, SPR_BFGG, SPR_BFGF, SPR_BLUD, SPR_PUFF,
	SPR_BAL1, SPR_BAL2, SPR_PLSS, SPR_PLSE, SPR_MISL, SPR_BFS1, SPR_BFE1, SPR_BFE2, SPR_TFOG,
	SPR_IFOG, SPR_PLAY, SPR_POSS, SPR_SPOS, SPR_VILE, SPR_FIRE, SPR_FATB, SPR_FBXP, SPR_SKEL,
	SPR_MANF, SPR_FATT, SPR_CPOS, SPR_SARG, SPR_HEAD, SPR_BAL7, SPR_BOSS, SPR_BOS2, SPR_SKUL,
	SPR_SPID, SPR_BSPI, SPR_APLS, SPR_APBX, SPR_CYBR, SPR_PAIN, SPR_SSWV, SPR_KEEN, SPR_BBRN,
	SPR_BOSF, SPR_ARM1, SPR_ARM2, SPR_BAR1, SPR_BEXP, SPR_FCAN, SPR_BON1, SPR_BON2, SPR_BKEY,
	SPR_RKEY, SPR_YKEY, SPR_BSKU, SPR_RSKU, SPR_YSKU, SPR_STIM, SPR_MEDI, SPR_SOUL, SPR_PINV,
	SPR_PSTR, SPR_PINS, SPR_MEGA, SPR_SUIT, SPR_PMAP, SPR_PVIS, SPR_CLIP, SPR_AMMO, SPR_ROCK,
	SPR_BROK, SPR_CELL, SPR_CELP, SPR_SHEL, SPR_SBOX, SPR_BPAK, SPR_BFUG, SPR_MGUN, SPR_CSAW,
	SPR_LAUN, SPR_PLAS, SPR_SHOT, SPR_SGN2, SPR_COLU, SPR_SMT2, SPR_GOR1, SPR_POL2, SPR_POL5,
	SPR_POL4, SPR_POL3, SPR_POL1, SPR_POL6, SPR_GOR2, SPR_GOR3, SPR_GOR4, SPR_GOR5, SPR_SMIT,
	SPR_COL1, SPR_COL2, SPR_COL3, SPR_COL4, SPR_CAND, SPR_CBRA, SPR_COL6, SPR_TRE1, SPR_TRE2,
	SPR_ELEC, SPR_CEYE, SPR_FSKU, SPR_COL5, SPR_TBLU, SPR_TGRN, SPR_TRED, SPR_SMBT, SPR_SMGT,
	SPR_SMRT, SPR_HDB1, SPR_HDB2, SPR_HDB3, SPR_HDB4, SPR_HDB5, SPR_HDB6, SPR_POB1, SPR_POB2,
	SPR_BRS1, SPR_TLMP, SPR_TLP2,
	NUMSPRITES,
}

#[repr(C)]
#[rustfmt::skip]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum statenum_t {
	S_NULL, S_LIGHTDONE,
	S_PUNCH, S_PUNCHDOWN, S_PUNCHUP, S_PUNCH1, S_PUNCH2, S_PUNCH3, S_PUNCH4, S_PUNCH5,
	S_PISTOL, S_PISTOLDOWN, S_PISTOLUP, S_PISTOL1, S_PISTOL2, S_PISTOL3, S_PISTOL4, S_PISTOLFLASH,
	S_SGUN, S_SGUNDOWN, S_SGUNUP, S_SGUN1, S_SGUN2, S_SGUN3, S_SGUN4,
	S_SGUN5, S_SGUN6, S_SGUN7, S_SGUN8, S_SGUN9, S_SGUNFLASH1, S_SGUNFLASH2,
	S_DSGUN, S_DSGUNDOWN, S_DSGUNUP, S_DSGUN1, S_DSGUN2, S_DSGUN3, S_DSGUN4,
	S_DSGUN5, S_DSGUN6, S_DSGUN7, S_DSGUN8, S_DSGUN9, S_DSGUN10,
	S_DSNR1, S_DSNR2,
	S_DSGUNFLASH1, S_DSGUNFLASH2,
	S_CHAIN, S_CHAINDOWN, S_CHAINUP, S_CHAIN1, S_CHAIN2, S_CHAIN3, S_CHAINFLASH1, S_CHAINFLASH2,
	S_MISSILE, S_MISSILEDOWN, S_MISSILEUP, S_MISSILE1, S_MISSILE2, S_MISSILE3,
	S_MISSILEFLASH1, S_MISSILEFLASH2, S_MISSILEFLASH3, S_MISSILEFLASH4,
	S_SAW, S_SAWB, S_SAWDOWN, S_SAWUP, S_SAW1, S_SAW2, S_SAW3,
	S_PLASMA, S_PLASMADOWN, S_PLASMAUP, S_PLASMA1, S_PLASMA2, S_PLASMAFLASH1, S_PLASMAFLASH2,
	S_BFG, S_BFGDOWN, S_BFGUP, S_BFG1, S_BFG2, S_BFG3, S_BFG4, S_BFGFLASH1, S_BFGFLASH2,
	S_BLOOD1, S_BLOOD2, S_BLOOD3,
	S_PUFF1, S_PUFF2, S_PUFF3, S_PUFF4,
	S_TBALL1, S_TBALL2, S_TBALLX1, S_TBALLX2, S_TBALLX3,
	S_RBALL1, S_RBALL2, S_RBALLX1, S_RBALLX2, S_RBALLX3,
	S_PLASBALL, S_PLASBALL2,
	S_PLASEXP, S_PLASEXP2, S_PLASEXP3, S_PLASEXP4, S_PLASEXP5,
	S_ROCKET,
	S_BFGSHOT, S_BFGSHOT2,
	S_BFGLAND, S_BFGLAND2, S_BFGLAND3, S_BFGLAND4, S_BFGLAND5, S_BFGLAND6,
	S_BFGEXP, S_BFGEXP2, S_BFGEXP3, S_BFGEXP4,
	S_EXPLODE1, S_EXPLODE2, S_EXPLODE3,
	S_TFOG, S_TFOG01, S_TFOG02, S_TFOG2, S_TFOG3, S_TFOG4,
	S_TFOG5, S_TFOG6, S_TFOG7, S_TFOG8, S_TFOG9, S_TFOG10,
	S_IFOG, S_IFOG01, S_IFOG02, S_IFOG2, S_IFOG3, S_IFOG4, S_IFOG5,
	S_PLAY,
	S_PLAY_RUN1, S_PLAY_RUN2, S_PLAY_RUN3, S_PLAY_RUN4,
	S_PLAY_ATK1, S_PLAY_ATK2,
	S_PLAY_PAIN, S_PLAY_PAIN2,
	S_PLAY_DIE1, S_PLAY_DIE2, S_PLAY_DIE3, S_PLAY_DIE4, S_PLAY_DIE5, S_PLAY_DIE6, S_PLAY_DIE7,
	S_PLAY_XDIE1, S_PLAY_XDIE2, S_PLAY_XDIE3, S_PLAY_XDIE4, S_PLAY_XDIE5,
	S_PLAY_XDIE6, S_PLAY_XDIE7, S_PLAY_XDIE8, S_PLAY_XDIE9,
	S_POSS_STND, S_POSS_STND2,
	S_POSS_RUN1, S_POSS_RUN2, S_POSS_RUN3, S_POSS_RUN4,
	S_POSS_RUN5, S_POSS_RUN6, S_POSS_RUN7, S_POSS_RUN8,
	S_POSS_ATK1, S_POSS_ATK2, S_POSS_ATK3,
	S_POSS_PAIN, S_POSS_PAIN2,
	S_POSS_DIE1, S_POSS_DIE2, S_POSS_DIE3, S_POSS_DIE4, S_POSS_DIE5,
	S_POSS_XDIE1, S_POSS_XDIE2, S_POSS_XDIE3, S_POSS_XDIE4, S_POSS_XDIE5,
	S_POSS_XDIE6, S_POSS_XDIE7, S_POSS_XDIE8, S_POSS_XDIE9,
	S_POSS_RAISE1, S_POSS_RAISE2, S_POSS_RAISE3, S_POSS_RAISE4,
	S_SPOS_STND, S_SPOS_STND2,
	S_SPOS_RUN1, S_SPOS_RUN2, S_SPOS_RUN3, S_SPOS_RUN4,
	S_SPOS_RUN5, S_SPOS_RUN6, S_SPOS_RUN7, S_SPOS_RUN8,
	S_SPOS_ATK1, S_SPOS_ATK2, S_SPOS_ATK3,
	S_SPOS_PAIN, S_SPOS_PAIN2,
	S_SPOS_DIE1, S_SPOS_DIE2, S_SPOS_DIE3, S_SPOS_DIE4, S_SPOS_DIE5, S_SPOS_XDIE1,
	S_SPOS_XDIE2, S_SPOS_XDIE3, S_SPOS_XDIE4, S_SPOS_XDIE5,
	S_SPOS_XDIE6, S_SPOS_XDIE7, S_SPOS_XDIE8, S_SPOS_XDIE9,
	S_SPOS_RAISE1, S_SPOS_RAISE2, S_SPOS_RAISE3, S_SPOS_RAISE4, S_SPOS_RAISE5,
	S_VILE_STND, S_VILE_STND2,
	S_VILE_RUN1, S_VILE_RUN2, S_VILE_RUN3, S_VILE_RUN4, S_VILE_RUN5, S_VILE_RUN6,
	S_VILE_RUN7, S_VILE_RUN8, S_VILE_RUN9, S_VILE_RUN10, S_VILE_RUN11, S_VILE_RUN12,
	S_VILE_ATK1, S_VILE_ATK2, S_VILE_ATK3, S_VILE_ATK4, S_VILE_ATK5, S_VILE_ATK6,
	S_VILE_ATK7, S_VILE_ATK8, S_VILE_ATK9, S_VILE_ATK10, S_VILE_ATK11,
	S_VILE_HEAL1, S_VILE_HEAL2, S_VILE_HEAL3,
	S_VILE_PAIN, S_VILE_PAIN2,
	S_VILE_DIE1, S_VILE_DIE2, S_VILE_DIE3, S_VILE_DIE4, S_VILE_DIE5,
	S_VILE_DIE6, S_VILE_DIE7, S_VILE_DIE8, S_VILE_DIE9, S_VILE_DIE10,
	S_FIRE1, S_FIRE2, S_FIRE3, S_FIRE4, S_FIRE5, S_FIRE6, S_FIRE7, S_FIRE8, S_FIRE9,
	S_FIRE10, S_FIRE11, S_FIRE12, S_FIRE13, S_FIRE14, S_FIRE15, S_FIRE16, S_FIRE17, S_FIRE18,
	S_FIRE19, S_FIRE20, S_FIRE21, S_FIRE22, S_FIRE23, S_FIRE24, S_FIRE25, S_FIRE26, S_FIRE27,
	S_FIRE28, S_FIRE29, S_FIRE30,
	S_SMOKE1, S_SMOKE2, S_SMOKE3, S_SMOKE4, S_SMOKE5, S_TRACER,
	S_TRACER2,
	S_TRACEEXP1, S_TRACEEXP2, S_TRACEEXP3,
	S_SKEL_STND, S_SKEL_STND2,
	S_SKEL_RUN1, S_SKEL_RUN2, S_SKEL_RUN3, S_SKEL_RUN4, S_SKEL_RUN5, S_SKEL_RUN6,
	S_SKEL_RUN7, S_SKEL_RUN8, S_SKEL_RUN9, S_SKEL_RUN10, S_SKEL_RUN11, S_SKEL_RUN12,
	S_SKEL_FIST1, S_SKEL_FIST2, S_SKEL_FIST3, S_SKEL_FIST4,
	S_SKEL_MISS1, S_SKEL_MISS2, S_SKEL_MISS3, S_SKEL_MISS4,
	S_SKEL_PAIN, S_SKEL_PAIN2,
	S_SKEL_DIE1, S_SKEL_DIE2, S_SKEL_DIE3, S_SKEL_DIE4, S_SKEL_DIE5, S_SKEL_DIE6,
	S_SKEL_RAISE1, S_SKEL_RAISE2, S_SKEL_RAISE3, S_SKEL_RAISE4, S_SKEL_RAISE5, S_SKEL_RAISE6,
	S_FATSHOT1, S_FATSHOT2,
	S_FATSHOTX1, S_FATSHOTX2, S_FATSHOTX3,
	S_FATT_STND, S_FATT_STND2,
	S_FATT_RUN1, S_FATT_RUN2, S_FATT_RUN3, S_FATT_RUN4, S_FATT_RUN5, S_FATT_RUN6,
	S_FATT_RUN7, S_FATT_RUN8, S_FATT_RUN9, S_FATT_RUN10, S_FATT_RUN11, S_FATT_RUN12,
	S_FATT_ATK1, S_FATT_ATK2, S_FATT_ATK3, S_FATT_ATK4, S_FATT_ATK5,
	S_FATT_ATK6, S_FATT_ATK7, S_FATT_ATK8, S_FATT_ATK9, S_FATT_ATK10,
	S_FATT_PAIN, S_FATT_PAIN2,
	S_FATT_DIE1, S_FATT_DIE2, S_FATT_DIE3, S_FATT_DIE4, S_FATT_DIE5,
	S_FATT_DIE6, S_FATT_DIE7, S_FATT_DIE8, S_FATT_DIE9, S_FATT_DIE10,
	S_FATT_RAISE1, S_FATT_RAISE2, S_FATT_RAISE3, S_FATT_RAISE4,
	S_FATT_RAISE5, S_FATT_RAISE6, S_FATT_RAISE7, S_FATT_RAISE8,
	S_CPOS_STND, S_CPOS_STND2,
	S_CPOS_RUN1, S_CPOS_RUN2, S_CPOS_RUN3, S_CPOS_RUN4,
	S_CPOS_RUN5, S_CPOS_RUN6, S_CPOS_RUN7, S_CPOS_RUN8,
	S_CPOS_ATK1, S_CPOS_ATK2, S_CPOS_ATK3, S_CPOS_ATK4,
	S_CPOS_PAIN, S_CPOS_PAIN2,
	S_CPOS_DIE1, S_CPOS_DIE2, S_CPOS_DIE3, S_CPOS_DIE4, S_CPOS_DIE5, S_CPOS_DIE6, S_CPOS_DIE7,
	S_CPOS_XDIE1, S_CPOS_XDIE2, S_CPOS_XDIE3, S_CPOS_XDIE4, S_CPOS_XDIE5, S_CPOS_XDIE6,
	S_CPOS_RAISE1, S_CPOS_RAISE2, S_CPOS_RAISE3, S_CPOS_RAISE4,
	S_CPOS_RAISE5, S_CPOS_RAISE6, S_CPOS_RAISE7,
	S_TROO_STND, S_TROO_STND2,
	S_TROO_RUN1, S_TROO_RUN2, S_TROO_RUN3, S_TROO_RUN4,
	S_TROO_RUN5, S_TROO_RUN6, S_TROO_RUN7, S_TROO_RUN8,
	S_TROO_ATK1, S_TROO_ATK2, S_TROO_ATK3,
	S_TROO_PAIN, S_TROO_PAIN2,
	S_TROO_DIE1, S_TROO_DIE2, S_TROO_DIE3, S_TROO_DIE4, S_TROO_DIE5,
	S_TROO_XDIE1, S_TROO_XDIE2, S_TROO_XDIE3, S_TROO_XDIE4,
	S_TROO_XDIE5, S_TROO_XDIE6, S_TROO_XDIE7, S_TROO_XDIE8,
	S_TROO_RAISE1, S_TROO_RAISE2, S_TROO_RAISE3, S_TROO_RAISE4, S_TROO_RAISE5,
	S_SARG_STND, S_SARG_STND2,
	S_SARG_RUN1, S_SARG_RUN2, S_SARG_RUN3, S_SARG_RUN4,
	S_SARG_RUN5, S_SARG_RUN6, S_SARG_RUN7, S_SARG_RUN8,
	S_SARG_ATK1, S_SARG_ATK2, S_SARG_ATK3,
	S_SARG_PAIN, S_SARG_PAIN2,
	S_SARG_DIE1, S_SARG_DIE2, S_SARG_DIE3, S_SARG_DIE4, S_SARG_DIE5, S_SARG_DIE6,
	S_SARG_RAISE1, S_SARG_RAISE2, S_SARG_RAISE3, S_SARG_RAISE4, S_SARG_RAISE5, S_SARG_RAISE6,
	S_HEAD_STND,
	S_HEAD_RUN1,
	S_HEAD_ATK1, S_HEAD_ATK2, S_HEAD_ATK3,
	S_HEAD_PAIN, S_HEAD_PAIN2, S_HEAD_PAIN3,
	S_HEAD_DIE1, S_HEAD_DIE2, S_HEAD_DIE3, S_HEAD_DIE4, S_HEAD_DIE5, S_HEAD_DIE6,
	S_HEAD_RAISE1, S_HEAD_RAISE2, S_HEAD_RAISE3, S_HEAD_RAISE4, S_HEAD_RAISE5, S_HEAD_RAISE6,
	S_BRBALL1, S_BRBALL2,
	S_BRBALLX1, S_BRBALLX2, S_BRBALLX3,
	S_BOSS_STND, S_BOSS_STND2,
	S_BOSS_RUN1, S_BOSS_RUN2, S_BOSS_RUN3, S_BOSS_RUN4,
	S_BOSS_RUN5, S_BOSS_RUN6, S_BOSS_RUN7, S_BOSS_RUN8,
	S_BOSS_ATK1, S_BOSS_ATK2, S_BOSS_ATK3,
	S_BOSS_PAIN, S_BOSS_PAIN2,
	S_BOSS_DIE1, S_BOSS_DIE2, S_BOSS_DIE3, S_BOSS_DIE4, S_BOSS_DIE5, S_BOSS_DIE6, S_BOSS_DIE7,
	S_BOSS_RAISE1, S_BOSS_RAISE2, S_BOSS_RAISE3, S_BOSS_RAISE4,
	S_BOSS_RAISE5, S_BOSS_RAISE6, S_BOSS_RAISE7,
	S_BOS2_STND, S_BOS2_STND2,
	S_BOS2_RUN1, S_BOS2_RUN2, S_BOS2_RUN3, S_BOS2_RUN4,
	S_BOS2_RUN5, S_BOS2_RUN6, S_BOS2_RUN7, S_BOS2_RUN8,
	S_BOS2_ATK1, S_BOS2_ATK2, S_BOS2_ATK3,
	S_BOS2_PAIN, S_BOS2_PAIN2,
	S_BOS2_DIE1, S_BOS2_DIE2, S_BOS2_DIE3, S_BOS2_DIE4, S_BOS2_DIE5, S_BOS2_DIE6, S_BOS2_DIE7,
	S_BOS2_RAISE1, S_BOS2_RAISE2, S_BOS2_RAISE3, S_BOS2_RAISE4,
	S_BOS2_RAISE5, S_BOS2_RAISE6, S_BOS2_RAISE7,
	S_SKULL_STND, S_SKULL_STND2,
	S_SKULL_RUN1, S_SKULL_RUN2,
	S_SKULL_ATK1, S_SKULL_ATK2, S_SKULL_ATK3, S_SKULL_ATK4,
	S_SKULL_PAIN, S_SKULL_PAIN2,
	S_SKULL_DIE1, S_SKULL_DIE2, S_SKULL_DIE3, S_SKULL_DIE4, S_SKULL_DIE5, S_SKULL_DIE6,
	S_SPID_STND, S_SPID_STND2,
	S_SPID_RUN1, S_SPID_RUN2, S_SPID_RUN3, S_SPID_RUN4, S_SPID_RUN5, S_SPID_RUN6,
	S_SPID_RUN7, S_SPID_RUN8, S_SPID_RUN9, S_SPID_RUN10, S_SPID_RUN11, S_SPID_RUN12,
	S_SPID_ATK1, S_SPID_ATK2, S_SPID_ATK3, S_SPID_ATK4,
	S_SPID_PAIN, S_SPID_PAIN2,
	S_SPID_DIE1, S_SPID_DIE2, S_SPID_DIE3, S_SPID_DIE4, S_SPID_DIE5, S_SPID_DIE6,
	S_SPID_DIE7, S_SPID_DIE8, S_SPID_DIE9, S_SPID_DIE10, S_SPID_DIE11,
	S_BSPI_STND, S_BSPI_STND2,
	S_BSPI_SIGHT,
	S_BSPI_RUN1, S_BSPI_RUN2, S_BSPI_RUN3, S_BSPI_RUN4, S_BSPI_RUN5, S_BSPI_RUN6,
	S_BSPI_RUN7, S_BSPI_RUN8, S_BSPI_RUN9, S_BSPI_RUN10, S_BSPI_RUN11, S_BSPI_RUN12,
	S_BSPI_ATK1, S_BSPI_ATK2, S_BSPI_ATK3, S_BSPI_ATK4,
	S_BSPI_PAIN, S_BSPI_PAIN2,
	S_BSPI_DIE1, S_BSPI_DIE2, S_BSPI_DIE3, S_BSPI_DIE4, S_BSPI_DIE5, S_BSPI_DIE6, S_BSPI_DIE7,
	S_BSPI_RAISE1, S_BSPI_RAISE2, S_BSPI_RAISE3, S_BSPI_RAISE4,
	S_BSPI_RAISE5, S_BSPI_RAISE6, S_BSPI_RAISE7,
	S_ARACH_PLAZ, S_ARACH_PLAZ2,
	S_ARACH_PLEX, S_ARACH_PLEX2, S_ARACH_PLEX3, S_ARACH_PLEX4, S_ARACH_PLEX5,
	S_CYBER_STND, S_CYBER_STND2,
	S_CYBER_RUN1, S_CYBER_RUN2, S_CYBER_RUN3, S_CYBER_RUN4,
	S_CYBER_RUN5, S_CYBER_RUN6, S_CYBER_RUN7, S_CYBER_RUN8,
	S_CYBER_ATK1, S_CYBER_ATK2, S_CYBER_ATK3, S_CYBER_ATK4, S_CYBER_ATK5, S_CYBER_ATK6,
	S_CYBER_PAIN,
	S_CYBER_DIE1, S_CYBER_DIE2, S_CYBER_DIE3, S_CYBER_DIE4, S_CYBER_DIE5,
	S_CYBER_DIE6, S_CYBER_DIE7, S_CYBER_DIE8, S_CYBER_DIE9, S_CYBER_DIE10,
	S_PAIN_STND,
	S_PAIN_RUN1, S_PAIN_RUN2, S_PAIN_RUN3, S_PAIN_RUN4, S_PAIN_RUN5, S_PAIN_RUN6,
	S_PAIN_ATK1, S_PAIN_ATK2, S_PAIN_ATK3, S_PAIN_ATK4,
	S_PAIN_PAIN, S_PAIN_PAIN2,
	S_PAIN_DIE1, S_PAIN_DIE2, S_PAIN_DIE3, S_PAIN_DIE4, S_PAIN_DIE5, S_PAIN_DIE6,
	S_PAIN_RAISE1, S_PAIN_RAISE2, S_PAIN_RAISE3, S_PAIN_RAISE4, S_PAIN_RAISE5, S_PAIN_RAISE6, 
	S_SSWV_STND, S_SSWV_STND2,
	S_SSWV_RUN1, S_SSWV_RUN2, S_SSWV_RUN3, S_SSWV_RUN4, 
	S_SSWV_RUN5, S_SSWV_RUN6, S_SSWV_RUN7, S_SSWV_RUN8,
	S_SSWV_ATK1, S_SSWV_ATK2, S_SSWV_ATK3, S_SSWV_ATK4, S_SSWV_ATK5, S_SSWV_ATK6,
	S_SSWV_PAIN, S_SSWV_PAIN2, 
	S_SSWV_DIE1, S_SSWV_DIE2, S_SSWV_DIE3, S_SSWV_DIE4, S_SSWV_DIE5,
	S_SSWV_XDIE1, S_SSWV_XDIE2, S_SSWV_XDIE3, S_SSWV_XDIE4, S_SSWV_XDIE5,
	S_SSWV_XDIE6, S_SSWV_XDIE7, S_SSWV_XDIE8, S_SSWV_XDIE9,
	S_SSWV_RAISE1, S_SSWV_RAISE2, S_SSWV_RAISE3, S_SSWV_RAISE4, S_SSWV_RAISE5,
	S_KEENSTND,
	S_COMMKEEN, S_COMMKEEN2, S_COMMKEEN3, S_COMMKEEN4, S_COMMKEEN5, S_COMMKEEN6,
	S_COMMKEEN7, S_COMMKEEN8, S_COMMKEEN9, S_COMMKEEN10, S_COMMKEEN11, S_COMMKEEN12,
	S_KEENPAIN, S_KEENPAIN2,
	S_BRAIN,
	S_BRAIN_PAIN,
	S_BRAIN_DIE1, S_BRAIN_DIE2, S_BRAIN_DIE3, S_BRAIN_DIE4,
	S_BRAINEYE,
	S_BRAINEYESEE,
	S_BRAINEYE1,
	S_SPAWN1, S_SPAWN2, S_SPAWN3, S_SPAWN4,
	S_SPAWNFIRE1, S_SPAWNFIRE2, S_SPAWNFIRE3, S_SPAWNFIRE4, 
	S_SPAWNFIRE5, S_SPAWNFIRE6, S_SPAWNFIRE7, S_SPAWNFIRE8,
	S_BRAINEXPLODE1, S_BRAINEXPLODE2, S_BRAINEXPLODE3,
	S_ARM1, S_ARM1A, S_ARM2, S_ARM2A,
	S_BAR1, S_BAR2,
	S_BEXP, S_BEXP2, S_BEXP3, S_BEXP4, S_BEXP5,
	S_BBAR1, S_BBAR2, S_BBAR3, 
	S_BON1, S_BON1A, S_BON1B, S_BON1C, S_BON1D, S_BON1E,
	S_BON2, S_BON2A, S_BON2B, S_BON2C, S_BON2D, S_BON2E,
	S_BKEY, S_BKEY2, S_RKEY, S_RKEY2, S_YKEY, S_YKEY2, 
	S_BSKULL, S_BSKULL2, S_RSKULL, S_RSKULL2, S_YSKULL, S_YSKULL2,
	S_STIM, S_MEDI, 
	S_SOUL, S_SOUL2, S_SOUL3, S_SOUL4, S_SOUL5, S_SOUL6,
	S_PINV, S_PINV2, S_PINV3, S_PINV4,
	S_PSTR,
	S_PINS, S_PINS2, S_PINS3, S_PINS4,
	S_MEGA, S_MEGA2, S_MEGA3, S_MEGA4,
	S_SUIT,
	S_PMAP, S_PMAP2, S_PMAP3, S_PMAP4, S_PMAP5, S_PMAP6,
	S_PVIS, S_PVIS2,
	S_CLIP, S_AMMO, S_ROCK, S_BROK, S_CELL, S_CELP, S_SHEL, S_SBOX, S_BPAK,
	S_BFUG, S_MGUN, S_CSAW, S_LAUN, S_PLAS, S_SHOT, S_SHOT2, S_COLU, 
	S_STALAG,
	S_BLOODYTWITCH, S_BLOODYTWITCH2, S_BLOODYTWITCH3, S_BLOODYTWITCH4, 
	S_DEADTORSO, S_DEADBOTTOM, S_HEADSONSTICK, S_GIBS, S_HEADONASTICK, 
	S_HEADCANDLES, S_HEADCANDLES2, S_DEADSTICK, S_LIVESTICK, S_LIVESTICK2,
	S_MEAT2, S_MEAT3, S_MEAT4, S_MEAT5,
	S_STALAGTITE,
	S_TALLGRNCOL, S_SHRTGRNCOL, S_TALLREDCOL, S_SHRTREDCOL,
	S_CANDLESTIK, S_CANDELABRA, S_SKULLCOL, S_TORCHTREE, S_BIGTREE, S_TECHPILLAR,
	S_EVILEYE, S_EVILEYE2, S_EVILEYE3, S_EVILEYE4,
	S_FLOATSKULL, S_FLOATSKULL2, S_FLOATSKULL3,
	S_HEARTCOL, S_HEARTCOL2,
	S_BLUETORCH, S_BLUETORCH2, S_BLUETORCH3, S_BLUETORCH4, 
	S_GREENTORCH, S_GREENTORCH2, S_GREENTORCH3, S_GREENTORCH4,
	S_REDTORCH, S_REDTORCH2, S_REDTORCH3, S_REDTORCH4,
	S_BTORCHSHRT, S_BTORCHSHRT2, S_BTORCHSHRT3, S_BTORCHSHRT4,
	S_GTORCHSHRT, S_GTORCHSHRT2, S_GTORCHSHRT3, S_GTORCHSHRT4,
	S_RTORCHSHRT, S_RTORCHSHRT2, S_RTORCHSHRT3, S_RTORCHSHRT4, 
	S_HANGNOGUTS, S_HANGBNOBRAIN, S_HANGTLOOKDN, S_HANGTSKULL, S_HANGTLOOKUP, 
	S_HANGTNOBRAIN, S_COLONGIBS, S_SMALLPOOL, S_BRAINSTEM,
	S_TECHLAMP, S_TECHLAMP2, S_TECHLAMP3, S_TECHLAMP4,
	S_TECH2LAMP, S_TECH2LAMP2, S_TECH2LAMP3, S_TECH2LAMP4,
	NUMSTATES,
}

#[repr(C)]
pub struct state_t {
	pub sprite: spritenum_t,
	pub frame: i32,
	pub tics: i32,
	pub action: actionf_t,
	pub nextstate: statenum_t,
	pub misc1: i32,
	pub misc2: i32,
}

#[repr(C)]
#[rustfmt::skip]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum mobjtype_t {
	MT_PLAYER, MT_POSSESSED, MT_SHOTGUY, MT_VILE, MT_FIRE, MT_UNDEAD, 
	MT_TRACER, MT_SMOKE, MT_FATSO, MT_FATSHOT, MT_CHAINGUY, MT_TROOP, MT_SERGEANT, 
	MT_SHADOWS, MT_HEAD, MT_BRUISER, MT_BRUISERSHOT, MT_KNIGHT, MT_SKULL, 
	MT_SPIDER, MT_BABY, MT_CYBORG, MT_PAIN, MT_WOLFSS, MT_KEEN, MT_BOSSBRAIN, 
	MT_BOSSSPIT, MT_BOSSTARGET, MT_SPAWNSHOT, MT_SPAWNFIRE, MT_BARREL, 
	MT_TROOPSHOT, MT_HEADSHOT, MT_ROCKET, MT_PLASMA, MT_BFG, MT_ARACHPLAZ, MT_PUFF, 
	MT_BLOOD, MT_TFOG, MT_IFOG, MT_TELEPORTMAN, MT_EXTRABFG, MT_MISC0, MT_MISC1, 
	MT_MISC2, MT_MISC3, MT_MISC4, MT_MISC5, MT_MISC6, MT_MISC7, MT_MISC8, MT_MISC9, 
	MT_MISC10, MT_MISC11, MT_MISC12, MT_INV, MT_MISC13, MT_INS, MT_MISC14, 
	MT_MISC15, MT_MISC16, MT_MEGA, MT_CLIP, MT_MISC17, MT_MISC18, MT_MISC19, 
	MT_MISC20, MT_MISC21, MT_MISC22, MT_MISC23, MT_MISC24, MT_MISC25, MT_CHAINGUN, 
	MT_MISC26, MT_MISC27, MT_MISC28, MT_SHOTGUN, MT_SUPERSHOTGUN, MT_MISC29, 
	MT_MISC30, MT_MISC31, MT_MISC32, MT_MISC33, MT_MISC34, MT_MISC35, MT_MISC36, 
	MT_MISC37, MT_MISC38, MT_MISC39, MT_MISC40, MT_MISC41, MT_MISC42, MT_MISC43, 
	MT_MISC44, MT_MISC45, MT_MISC46, MT_MISC47, MT_MISC48, MT_MISC49, MT_MISC50, 
	MT_MISC51, MT_MISC52, MT_MISC53, MT_MISC54, MT_MISC55, MT_MISC56, MT_MISC57, 
	MT_MISC58, MT_MISC59, MT_MISC60, MT_MISC61, MT_MISC62, MT_MISC63, MT_MISC64, 
	MT_MISC65, MT_MISC66, MT_MISC67, MT_MISC68, MT_MISC69, MT_MISC70, MT_MISC71, 
	MT_MISC72, MT_MISC73, MT_MISC74, MT_MISC75, MT_MISC76, MT_MISC77, MT_MISC78, 
	MT_MISC79, MT_MISC80, MT_MISC81, MT_MISC82, MT_MISC83, MT_MISC84, MT_MISC85, 
	MT_MISC86,
	NUMMOBJTYPES,
}

impl From<usize> for mobjtype_t {
	fn from(value: usize) -> Self {
		if value > Self::NUMMOBJTYPES as usize {
			panic!("mobjtype_t out of bounds");
		}
		unsafe { std::mem::transmute(value) }
	}
}

#[repr(C)]
pub struct mobjinfo_t {
	pub doomednum: i32,
	pub spawnstate: statenum_t,
	pub spawnhealth: i32,
	pub seestate: statenum_t,
	pub seesound: sfxenum_t,
	pub reactiontime: i32,
	pub attacksound: sfxenum_t,
	pub painstate: statenum_t,
	pub painchance: i32,
	pub painsound: sfxenum_t,
	pub meleestate: statenum_t,
	pub missilestate: statenum_t,
	pub deathstate: statenum_t,
	pub xdeathstate: statenum_t,
	pub deathsound: sfxenum_t,
	pub speed: i32,
	pub radius: i32,
	pub height: i32,
	pub mass: i32,
	pub damage: i32,
	pub activesound: sfxenum_t,
	pub flags: u32,
	pub raisestate: statenum_t,
}

#[unsafe(no_mangle)]
#[rustfmt::skip]
pub static mut sprnames: [*const c_char; spritenum_t::NUMSPRITES as usize] = [
	c"TROO".as_ptr(), c"SHTG".as_ptr(), c"PUNG".as_ptr(), c"PISG".as_ptr(), 
	c"PISF".as_ptr(), c"SHTF".as_ptr(), c"SHT2".as_ptr(), c"CHGG".as_ptr(), c"CHGF".as_ptr(), 
	c"MISG".as_ptr(), c"MISF".as_ptr(), c"SAWG".as_ptr(), c"PLSG".as_ptr(), c"PLSF".as_ptr(), 
	c"BFGG".as_ptr(), c"BFGF".as_ptr(), c"BLUD".as_ptr(), c"PUFF".as_ptr(), c"BAL1".as_ptr(), 
	c"BAL2".as_ptr(), c"PLSS".as_ptr(), c"PLSE".as_ptr(), c"MISL".as_ptr(), c"BFS1".as_ptr(), 
	c"BFE1".as_ptr(), c"BFE2".as_ptr(), c"TFOG".as_ptr(), c"IFOG".as_ptr(), c"PLAY".as_ptr(), 
	c"POSS".as_ptr(), c"SPOS".as_ptr(), c"VILE".as_ptr(), c"FIRE".as_ptr(), c"FATB".as_ptr(), 
	c"FBXP".as_ptr(), c"SKEL".as_ptr(), c"MANF".as_ptr(), c"FATT".as_ptr(), c"CPOS".as_ptr(), 
	c"SARG".as_ptr(), c"HEAD".as_ptr(), c"BAL7".as_ptr(), c"BOSS".as_ptr(), c"BOS2".as_ptr(), 
	c"SKUL".as_ptr(), c"SPID".as_ptr(), c"BSPI".as_ptr(), c"APLS".as_ptr(), c"APBX".as_ptr(), 
	c"CYBR".as_ptr(), c"PAIN".as_ptr(), c"SSWV".as_ptr(), c"KEEN".as_ptr(), c"BBRN".as_ptr(), 
	c"BOSF".as_ptr(), c"ARM1".as_ptr(), c"ARM2".as_ptr(), c"BAR1".as_ptr(), c"BEXP".as_ptr(), 
	c"FCAN".as_ptr(), c"BON1".as_ptr(), c"BON2".as_ptr(), c"BKEY".as_ptr(), c"RKEY".as_ptr(), 
	c"YKEY".as_ptr(), c"BSKU".as_ptr(), c"RSKU".as_ptr(), c"YSKU".as_ptr(), c"STIM".as_ptr(), 
	c"MEDI".as_ptr(), c"SOUL".as_ptr(), c"PINV".as_ptr(), c"PSTR".as_ptr(), c"PINS".as_ptr(), 
	c"MEGA".as_ptr(), c"SUIT".as_ptr(), c"PMAP".as_ptr(), c"PVIS".as_ptr(), c"CLIP".as_ptr(), 
	c"AMMO".as_ptr(), c"ROCK".as_ptr(), c"BROK".as_ptr(), c"CELL".as_ptr(), c"CELP".as_ptr(), 
	c"SHEL".as_ptr(), c"SBOX".as_ptr(), c"BPAK".as_ptr(), c"BFUG".as_ptr(), c"MGUN".as_ptr(), 
	c"CSAW".as_ptr(), c"LAUN".as_ptr(), c"PLAS".as_ptr(), c"SHOT".as_ptr(), c"SGN2".as_ptr(), 
	c"COLU".as_ptr(), c"SMT2".as_ptr(), c"GOR1".as_ptr(), c"POL2".as_ptr(), c"POL5".as_ptr(), 
	c"POL4".as_ptr(), c"POL3".as_ptr(), c"POL1".as_ptr(), c"POL6".as_ptr(), c"GOR2".as_ptr(), 
	c"GOR3".as_ptr(), c"GOR4".as_ptr(), c"GOR5".as_ptr(), c"SMIT".as_ptr(), c"COL1".as_ptr(), 
	c"COL2".as_ptr(), c"COL3".as_ptr(), c"COL4".as_ptr(), c"CAND".as_ptr(), c"CBRA".as_ptr(), 
	c"COL6".as_ptr(), c"TRE1".as_ptr(), c"TRE2".as_ptr(), c"ELEC".as_ptr(), c"CEYE".as_ptr(), 
	c"FSKU".as_ptr(), c"COL5".as_ptr(), c"TBLU".as_ptr(), c"TGRN".as_ptr(), c"TRED".as_ptr(), 
	c"SMBT".as_ptr(), c"SMGT".as_ptr(), c"SMRT".as_ptr(), c"HDB1".as_ptr(), c"HDB2".as_ptr(), 
	c"HDB3".as_ptr(), c"HDB4".as_ptr(), c"HDB5".as_ptr(), c"HDB6".as_ptr(), c"POB1".as_ptr(), 
	c"POB2".as_ptr(), c"BRS1".as_ptr(), c"TLMP".as_ptr(), c"TLP2".as_ptr(),
];

unsafe extern "C" {
	// Doesn't work with g++, needs actionf_p1
	fn A_Light0(_: *mut c_void);
	fn A_WeaponReady(_: *mut c_void);
	fn A_Lower(_: *mut c_void);
	fn A_Raise(_: *mut c_void);
	fn A_Punch(_: *mut c_void);
	fn A_ReFire(_: *mut c_void);
	fn A_FirePistol(_: *mut c_void);
	fn A_Light1(_: *mut c_void);
	fn A_FireShotgun(_: *mut c_void);
	fn A_Light2(_: *mut c_void);
	fn A_FireShotgun2(_: *mut c_void);
	fn A_CheckReload(_: *mut c_void);
	fn A_OpenShotgun2(_: *mut c_void);
	fn A_LoadShotgun2(_: *mut c_void);
	fn A_CloseShotgun2(_: *mut c_void);
	fn A_FireCGun(_: *mut c_void);
	fn A_GunFlash(_: *mut c_void);
	fn A_FireMissile(_: *mut c_void);
	fn A_Saw(_: *mut c_void);
	fn A_FirePlasma(_: *mut c_void);
	fn A_BFGsound(_: *mut c_void);
	fn A_FireBFG(_: *mut c_void);
	fn A_BFGSpray(_: *mut c_void);
	fn A_Explode(_: *mut c_void);
	fn A_Pain(_: *mut c_void);
	fn A_PlayerScream(_: *mut c_void);
	fn A_Fall(_: *mut c_void);
	fn A_XScream(_: *mut c_void);
	fn A_Look(_: *mut c_void);
	fn A_Chase(_: *mut c_void);
	fn A_FaceTarget(_: *mut c_void);
	fn A_PosAttack(_: *mut c_void);
	fn A_Scream(_: *mut c_void);
	fn A_SPosAttack(_: *mut c_void);
	fn A_VileChase(_: *mut c_void);
	fn A_VileStart(_: *mut c_void);
	fn A_VileTarget(_: *mut c_void);
	fn A_VileAttack(_: *mut c_void);
	fn A_StartFire(_: *mut c_void);
	fn A_Fire(_: *mut c_void);
	fn A_FireCrackle(_: *mut c_void);
	fn A_Tracer(_: *mut c_void);
	fn A_SkelWhoosh(_: *mut c_void);
	fn A_SkelFist(_: *mut c_void);
	fn A_SkelMissile(_: *mut c_void);
	fn A_FatRaise(_: *mut c_void);
	fn A_FatAttack1(_: *mut c_void);
	fn A_FatAttack2(_: *mut c_void);
	fn A_FatAttack3(_: *mut c_void);
	fn A_BossDeath(_: *mut c_void);
	fn A_CPosAttack(_: *mut c_void);
	fn A_CPosRefire(_: *mut c_void);
	fn A_TroopAttack(_: *mut c_void);
	fn A_SargAttack(_: *mut c_void);
	fn A_HeadAttack(_: *mut c_void);
	fn A_BruisAttack(_: *mut c_void);
	fn A_SkullAttack(_: *mut c_void);
	fn A_Metal(_: *mut c_void);
	fn A_SpidRefire(_: *mut c_void);
	fn A_BabyMetal(_: *mut c_void);
	fn A_BspiAttack(_: *mut c_void);
	fn A_Hoof(_: *mut c_void);
	fn A_CyberAttack(_: *mut c_void);
	fn A_PainAttack(_: *mut c_void);
	fn A_PainDie(_: *mut c_void);
	fn A_KeenDie(_: *mut c_void);
	fn A_BrainPain(_: *mut c_void);
	fn A_BrainScream(_: *mut c_void);
	fn A_BrainDie(_: *mut c_void);
	fn A_BrainAwake(_: *mut c_void);
	fn A_BrainSpit(_: *mut c_void);
	fn A_SpawnSound(_: *mut c_void);
	fn A_SpawnFly(_: *mut c_void);
	fn A_BrainExplode(_: *mut c_void);
}

unsafe extern "C" fn NULL(_: *mut c_void) {}

#[unsafe(no_mangle)]
#[rustfmt::skip]
pub static mut 	states:[state_t; statenum_t::NUMSTATES as usize] = [	
	state_t { sprite: spritenum_t::SPR_TROO, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_NULL
	state_t { sprite: spritenum_t::SPR_SHTG, frame: 4, tics: 0, action: actionf_t{acp1: Some(A_Light0)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_LIGHTDONE
	state_t { sprite: spritenum_t::SPR_PUNG, frame: 0, tics: 1, action: actionf_t{acp1: Some(A_WeaponReady)}, nextstate: statenum_t::S_PUNCH, misc1: 0, misc2: 0},	// S_PUNCH
	state_t { sprite: spritenum_t::SPR_PUNG, frame: 0, tics: 1, action: actionf_t{acp1: Some(A_Lower)}, nextstate: statenum_t::S_PUNCHDOWN, misc1: 0, misc2: 0},	// S_PUNCHDOWN
	state_t { sprite: spritenum_t::SPR_PUNG, frame: 0, tics: 1, action: actionf_t{acp1: Some(A_Raise)}, nextstate: statenum_t::S_PUNCHUP, misc1: 0, misc2: 0},	// S_PUNCHUP
	state_t { sprite: spritenum_t::SPR_PUNG, frame: 1, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PUNCH2, misc1: 0, misc2: 0},		// S_PUNCH1
	state_t { sprite: spritenum_t::SPR_PUNG, frame: 2, tics: 4, action: actionf_t{acp1: Some(A_Punch)}, nextstate: statenum_t::S_PUNCH3, misc1: 0, misc2: 0},	// S_PUNCH2
	state_t { sprite: spritenum_t::SPR_PUNG, frame: 3, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PUNCH4, misc1: 0, misc2: 0},		// S_PUNCH3
	state_t { sprite: spritenum_t::SPR_PUNG, frame: 2, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PUNCH5, misc1: 0, misc2: 0},		// S_PUNCH4
	state_t { sprite: spritenum_t::SPR_PUNG, frame: 1, tics: 5, action: actionf_t{acp1: Some(A_ReFire)}, nextstate: statenum_t::S_PUNCH, misc1: 0, misc2: 0},	// S_PUNCH5
	state_t { sprite: spritenum_t::SPR_PISG, frame: 0, tics: 1, action: actionf_t{acp1: Some(A_WeaponReady)}, nextstate: statenum_t::S_PISTOL, misc1: 0, misc2: 0},// S_PISTOL
	state_t { sprite: spritenum_t::SPR_PISG, frame: 0, tics: 1, action: actionf_t{acp1: Some(A_Lower)}, nextstate: statenum_t::S_PISTOLDOWN, misc1: 0, misc2: 0},	// S_PISTOLDOWN
	state_t { sprite: spritenum_t::SPR_PISG, frame: 0, tics: 1, action: actionf_t{acp1: Some(A_Raise)}, nextstate: statenum_t::S_PISTOLUP, misc1: 0, misc2: 0},	// S_PISTOLUP
	state_t { sprite: spritenum_t::SPR_PISG, frame: 0, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PISTOL2, misc1: 0, misc2: 0},	// S_PISTOL1
	state_t { sprite: spritenum_t::SPR_PISG, frame: 1, tics: 6, action: actionf_t{acp1: Some(A_FirePistol)}, nextstate: statenum_t::S_PISTOL3, misc1: 0, misc2: 0},// S_PISTOL2
	state_t { sprite: spritenum_t::SPR_PISG, frame: 2, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PISTOL4, misc1: 0, misc2: 0},	// S_PISTOL3
	state_t { sprite: spritenum_t::SPR_PISG, frame: 1, tics: 5, action: actionf_t{acp1: Some(A_ReFire)}, nextstate: statenum_t::S_PISTOL, misc1: 0, misc2: 0},	// S_PISTOL4
	state_t { sprite: spritenum_t::SPR_PISF, frame: 32768, tics: 7, action: actionf_t{acp1: Some(A_Light1)}, nextstate: statenum_t::S_LIGHTDONE, misc1: 0, misc2: 0},	// S_PISTOLFLASH
	state_t { sprite: spritenum_t::SPR_SHTG, frame: 0, tics: 1, action: actionf_t{acp1: Some(A_WeaponReady)}, nextstate: statenum_t::S_SGUN, misc1: 0, misc2: 0},	// S_SGUN
	state_t { sprite: spritenum_t::SPR_SHTG, frame: 0, tics: 1, action: actionf_t{acp1: Some(A_Lower)}, nextstate: statenum_t::S_SGUNDOWN, misc1: 0, misc2: 0},	// S_SGUNDOWN
	state_t { sprite: spritenum_t::SPR_SHTG, frame: 0, tics: 1, action: actionf_t{acp1: Some(A_Raise)}, nextstate: statenum_t::S_SGUNUP, misc1: 0, misc2: 0},	// S_SGUNUP
	state_t { sprite: spritenum_t::SPR_SHTG, frame: 0, tics: 3, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SGUN2, misc1: 0, misc2: 0},	// S_SGUN1
	state_t { sprite: spritenum_t::SPR_SHTG, frame: 0, tics: 7, action: actionf_t{acp1: Some(A_FireShotgun)}, nextstate: statenum_t::S_SGUN3, misc1: 0, misc2: 0},	// S_SGUN2
	state_t { sprite: spritenum_t::SPR_SHTG, frame: 1, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SGUN4, misc1: 0, misc2: 0},	// S_SGUN3
	state_t { sprite: spritenum_t::SPR_SHTG, frame: 2, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SGUN5, misc1: 0, misc2: 0},	// S_SGUN4
	state_t { sprite: spritenum_t::SPR_SHTG, frame: 3, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SGUN6, misc1: 0, misc2: 0},	// S_SGUN5
	state_t { sprite: spritenum_t::SPR_SHTG, frame: 2, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SGUN7, misc1: 0, misc2: 0},	// S_SGUN6
	state_t { sprite: spritenum_t::SPR_SHTG, frame: 1, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SGUN8, misc1: 0, misc2: 0},	// S_SGUN7
	state_t { sprite: spritenum_t::SPR_SHTG, frame: 0, tics: 3, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SGUN9, misc1: 0, misc2: 0},	// S_SGUN8
	state_t { sprite: spritenum_t::SPR_SHTG, frame: 0, tics: 7, action: actionf_t{acp1: Some(A_ReFire)}, nextstate: statenum_t::S_SGUN, misc1: 0, misc2: 0},	// S_SGUN9
	state_t { sprite: spritenum_t::SPR_SHTF, frame: 32768, tics: 4, action: actionf_t{acp1: Some(A_Light1)}, nextstate: statenum_t::S_SGUNFLASH2, misc1: 0, misc2: 0},	// S_SGUNFLASH1
	state_t { sprite: spritenum_t::SPR_SHTF, frame: 32769, tics: 3, action: actionf_t{acp1: Some(A_Light2)}, nextstate: statenum_t::S_LIGHTDONE, misc1: 0, misc2: 0},	// S_SGUNFLASH2
	state_t { sprite: spritenum_t::SPR_SHT2, frame: 0, tics: 1, action: actionf_t{acp1: Some(A_WeaponReady)}, nextstate: statenum_t::S_DSGUN, misc1: 0, misc2: 0},	// S_DSGUN
	state_t { sprite: spritenum_t::SPR_SHT2, frame: 0, tics: 1, action: actionf_t{acp1: Some(A_Lower)}, nextstate: statenum_t::S_DSGUNDOWN, misc1: 0, misc2: 0},	// S_DSGUNDOWN
	state_t { sprite: spritenum_t::SPR_SHT2, frame: 0, tics: 1, action: actionf_t{acp1: Some(A_Raise)}, nextstate: statenum_t::S_DSGUNUP, misc1: 0, misc2: 0},	// S_DSGUNUP
	state_t { sprite: spritenum_t::SPR_SHT2, frame: 0, tics: 3, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_DSGUN2, misc1: 0, misc2: 0},	// S_DSGUN1
	state_t { sprite: spritenum_t::SPR_SHT2, frame: 0, tics: 7, action: actionf_t{acp1: Some(A_FireShotgun2)}, nextstate: statenum_t::S_DSGUN3, misc1: 0, misc2: 0},	// S_DSGUN2
	state_t { sprite: spritenum_t::SPR_SHT2, frame: 1, tics: 7, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_DSGUN4, misc1: 0, misc2: 0},	// S_DSGUN3
	state_t { sprite: spritenum_t::SPR_SHT2, frame: 2, tics: 7, action: actionf_t{acp1: Some(A_CheckReload)}, nextstate: statenum_t::S_DSGUN5, misc1: 0, misc2: 0},	// S_DSGUN4
	state_t { sprite: spritenum_t::SPR_SHT2, frame: 3, tics: 7, action: actionf_t{acp1: Some(A_OpenShotgun2)}, nextstate: statenum_t::S_DSGUN6, misc1: 0, misc2: 0},	// S_DSGUN5
	state_t { sprite: spritenum_t::SPR_SHT2, frame: 4, tics: 7, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_DSGUN7, misc1: 0, misc2: 0},	// S_DSGUN6
	state_t { sprite: spritenum_t::SPR_SHT2, frame: 5, tics: 7, action: actionf_t{acp1: Some(A_LoadShotgun2)}, nextstate: statenum_t::S_DSGUN8, misc1: 0, misc2: 0},	// S_DSGUN7
	state_t { sprite: spritenum_t::SPR_SHT2, frame: 6, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_DSGUN9, misc1: 0, misc2: 0},	// S_DSGUN8
	state_t { sprite: spritenum_t::SPR_SHT2, frame: 7, tics: 6, action: actionf_t{acp1: Some(A_CloseShotgun2)}, nextstate: statenum_t::S_DSGUN10, misc1: 0, misc2: 0},	// S_DSGUN9
	state_t { sprite: spritenum_t::SPR_SHT2, frame: 0, tics: 5, action: actionf_t{acp1: Some(A_ReFire)}, nextstate: statenum_t::S_DSGUN, misc1: 0, misc2: 0},	// S_DSGUN10
	state_t { sprite: spritenum_t::SPR_SHT2, frame: 1, tics: 7, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_DSNR2, misc1: 0, misc2: 0},	// S_DSNR1
	state_t { sprite: spritenum_t::SPR_SHT2, frame: 0, tics: 3, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_DSGUNDOWN, misc1: 0, misc2: 0},	// S_DSNR2
	state_t { sprite: spritenum_t::SPR_SHT2, frame: 32776, tics: 5, action: actionf_t{acp1: Some(A_Light1)}, nextstate: statenum_t::S_DSGUNFLASH2, misc1: 0, misc2: 0},	// S_DSGUNFLASH1
	state_t { sprite: spritenum_t::SPR_SHT2, frame: 32777, tics: 4, action: actionf_t{acp1: Some(A_Light2)}, nextstate: statenum_t::S_LIGHTDONE, misc1: 0, misc2: 0},	// S_DSGUNFLASH2
	state_t { sprite: spritenum_t::SPR_CHGG, frame: 0, tics: 1, action: actionf_t{acp1: Some(A_WeaponReady)}, nextstate: statenum_t::S_CHAIN, misc1: 0, misc2: 0},	// S_CHAIN
	state_t { sprite: spritenum_t::SPR_CHGG, frame: 0, tics: 1, action: actionf_t{acp1: Some(A_Lower)}, nextstate: statenum_t::S_CHAINDOWN, misc1: 0, misc2: 0},	// S_CHAINDOWN
	state_t { sprite: spritenum_t::SPR_CHGG, frame: 0, tics: 1, action: actionf_t{acp1: Some(A_Raise)}, nextstate: statenum_t::S_CHAINUP, misc1: 0, misc2: 0},	// S_CHAINUP
	state_t { sprite: spritenum_t::SPR_CHGG, frame: 0, tics: 4, action: actionf_t{acp1: Some(A_FireCGun)}, nextstate: statenum_t::S_CHAIN2, misc1: 0, misc2: 0},	// S_CHAIN1
	state_t { sprite: spritenum_t::SPR_CHGG, frame: 1, tics: 4, action: actionf_t{acp1: Some(A_FireCGun)}, nextstate: statenum_t::S_CHAIN3, misc1: 0, misc2: 0},	// S_CHAIN2
	state_t { sprite: spritenum_t::SPR_CHGG, frame: 1, tics: 0, action: actionf_t{acp1: Some(A_ReFire)}, nextstate: statenum_t::S_CHAIN, misc1: 0, misc2: 0},	// S_CHAIN3
	state_t { sprite: spritenum_t::SPR_CHGF, frame: 32768, tics: 5, action: actionf_t{acp1: Some(A_Light1)}, nextstate: statenum_t::S_LIGHTDONE, misc1: 0, misc2: 0},	// S_CHAINFLASH1
	state_t { sprite: spritenum_t::SPR_CHGF, frame: 32769, tics: 5, action: actionf_t{acp1: Some(A_Light2)}, nextstate: statenum_t::S_LIGHTDONE, misc1: 0, misc2: 0},	// S_CHAINFLASH2
	state_t { sprite: spritenum_t::SPR_MISG, frame: 0, tics: 1, action: actionf_t{acp1: Some(A_WeaponReady)}, nextstate: statenum_t::S_MISSILE, misc1: 0, misc2: 0},	// S_MISSILE
	state_t { sprite: spritenum_t::SPR_MISG, frame: 0, tics: 1, action: actionf_t{acp1: Some(A_Lower)}, nextstate: statenum_t::S_MISSILEDOWN, misc1: 0, misc2: 0},	// S_MISSILEDOWN
	state_t { sprite: spritenum_t::SPR_MISG, frame: 0, tics: 1, action: actionf_t{acp1: Some(A_Raise)}, nextstate: statenum_t::S_MISSILEUP, misc1: 0, misc2: 0},	// S_MISSILEUP
	state_t { sprite: spritenum_t::SPR_MISG, frame: 1, tics: 8, action: actionf_t{acp1: Some(A_GunFlash)}, nextstate: statenum_t::S_MISSILE2, misc1: 0, misc2: 0},	// S_MISSILE1
	state_t { sprite: spritenum_t::SPR_MISG, frame: 1, tics: 12, action: actionf_t{acp1: Some(A_FireMissile)}, nextstate: statenum_t::S_MISSILE3, misc1: 0, misc2: 0},	// S_MISSILE2
	state_t { sprite: spritenum_t::SPR_MISG, frame: 1, tics: 0, action: actionf_t{acp1: Some(A_ReFire)}, nextstate: statenum_t::S_MISSILE, misc1: 0, misc2: 0},	// S_MISSILE3
	state_t { sprite: spritenum_t::SPR_MISF, frame: 32768, tics: 3, action: actionf_t{acp1: Some(A_Light1)}, nextstate: statenum_t::S_MISSILEFLASH2, misc1: 0, misc2: 0},	// S_MISSILEFLASH1
	state_t { sprite: spritenum_t::SPR_MISF, frame: 32769, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_MISSILEFLASH3, misc1: 0, misc2: 0},	// S_MISSILEFLASH2
	state_t { sprite: spritenum_t::SPR_MISF, frame: 32770, tics: 4, action: actionf_t{acp1: Some(A_Light2)}, nextstate: statenum_t::S_MISSILEFLASH4, misc1: 0, misc2: 0},	// S_MISSILEFLASH3
	state_t { sprite: spritenum_t::SPR_MISF, frame: 32771, tics: 4, action: actionf_t{acp1: Some(A_Light2)}, nextstate: statenum_t::S_LIGHTDONE, misc1: 0, misc2: 0},	// S_MISSILEFLASH4
	state_t { sprite: spritenum_t::SPR_SAWG, frame: 2, tics: 4, action: actionf_t{acp1: Some(A_WeaponReady)}, nextstate: statenum_t::S_SAWB, misc1: 0, misc2: 0},	// S_SAW
	state_t { sprite: spritenum_t::SPR_SAWG, frame: 3, tics: 4, action: actionf_t{acp1: Some(A_WeaponReady)}, nextstate: statenum_t::S_SAW, misc1: 0, misc2: 0},	// S_SAWB
	state_t { sprite: spritenum_t::SPR_SAWG, frame: 2, tics: 1, action: actionf_t{acp1: Some(A_Lower)}, nextstate: statenum_t::S_SAWDOWN, misc1: 0, misc2: 0},	// S_SAWDOWN
	state_t { sprite: spritenum_t::SPR_SAWG, frame: 2, tics: 1, action: actionf_t{acp1: Some(A_Raise)}, nextstate: statenum_t::S_SAWUP, misc1: 0, misc2: 0},	// S_SAWUP
	state_t { sprite: spritenum_t::SPR_SAWG, frame: 0, tics: 4, action: actionf_t{acp1: Some(A_Saw)}, nextstate: statenum_t::S_SAW2, misc1: 0, misc2: 0},	// S_SAW1
	state_t { sprite: spritenum_t::SPR_SAWG, frame: 1, tics: 4, action: actionf_t{acp1: Some(A_Saw)}, nextstate: statenum_t::S_SAW3, misc1: 0, misc2: 0},	// S_SAW2
	state_t { sprite: spritenum_t::SPR_SAWG, frame: 1, tics: 0, action: actionf_t{acp1: Some(A_ReFire)}, nextstate: statenum_t::S_SAW, misc1: 0, misc2: 0},	// S_SAW3
	state_t { sprite: spritenum_t::SPR_PLSG, frame: 0, tics: 1, action: actionf_t{acp1: Some(A_WeaponReady)}, nextstate: statenum_t::S_PLASMA, misc1: 0, misc2: 0},	// S_PLASMA
	state_t { sprite: spritenum_t::SPR_PLSG, frame: 0, tics: 1, action: actionf_t{acp1: Some(A_Lower)}, nextstate: statenum_t::S_PLASMADOWN, misc1: 0, misc2: 0},	// S_PLASMADOWN
	state_t { sprite: spritenum_t::SPR_PLSG, frame: 0, tics: 1, action: actionf_t{acp1: Some(A_Raise)}, nextstate: statenum_t::S_PLASMAUP, misc1: 0, misc2: 0},	// S_PLASMAUP
	state_t { sprite: spritenum_t::SPR_PLSG, frame: 0, tics: 3, action: actionf_t{acp1: Some(A_FirePlasma)}, nextstate: statenum_t::S_PLASMA2, misc1: 0, misc2: 0},	// S_PLASMA1
	state_t { sprite: spritenum_t::SPR_PLSG, frame: 1, tics: 20, action: actionf_t{acp1: Some(A_ReFire)}, nextstate: statenum_t::S_PLASMA, misc1: 0, misc2: 0},	// S_PLASMA2
	state_t { sprite: spritenum_t::SPR_PLSF, frame: 32768, tics: 4, action: actionf_t{acp1: Some(A_Light1)}, nextstate: statenum_t::S_LIGHTDONE, misc1: 0, misc2: 0},	// S_PLASMAFLASH1
	state_t { sprite: spritenum_t::SPR_PLSF, frame: 32769, tics: 4, action: actionf_t{acp1: Some(A_Light1)}, nextstate: statenum_t::S_LIGHTDONE, misc1: 0, misc2: 0},	// S_PLASMAFLASH2
	state_t { sprite: spritenum_t::SPR_BFGG, frame: 0, tics: 1, action: actionf_t{acp1: Some(A_WeaponReady)}, nextstate: statenum_t::S_BFG, misc1: 0, misc2: 0},	// S_BFG
	state_t { sprite: spritenum_t::SPR_BFGG, frame: 0, tics: 1, action: actionf_t{acp1: Some(A_Lower)}, nextstate: statenum_t::S_BFGDOWN, misc1: 0, misc2: 0},	// S_BFGDOWN
	state_t { sprite: spritenum_t::SPR_BFGG, frame: 0, tics: 1, action: actionf_t{acp1: Some(A_Raise)}, nextstate: statenum_t::S_BFGUP, misc1: 0, misc2: 0},	// S_BFGUP
	state_t { sprite: spritenum_t::SPR_BFGG, frame: 0, tics: 20, action: actionf_t{acp1: Some(A_BFGsound)}, nextstate: statenum_t::S_BFG2, misc1: 0, misc2: 0},	// S_BFG1
	state_t { sprite: spritenum_t::SPR_BFGG, frame: 1, tics: 10, action: actionf_t{acp1: Some(A_GunFlash)}, nextstate: statenum_t::S_BFG3, misc1: 0, misc2: 0},	// S_BFG2
	state_t { sprite: spritenum_t::SPR_BFGG, frame: 1, tics: 10, action: actionf_t{acp1: Some(A_FireBFG)}, nextstate: statenum_t::S_BFG4, misc1: 0, misc2: 0},	// S_BFG3
	state_t { sprite: spritenum_t::SPR_BFGG, frame: 1, tics: 20, action: actionf_t{acp1: Some(A_ReFire)}, nextstate: statenum_t::S_BFG, misc1: 0, misc2: 0},	// S_BFG4
	state_t { sprite: spritenum_t::SPR_BFGF, frame: 32768, tics: 11, action: actionf_t{acp1: Some(A_Light1)}, nextstate: statenum_t::S_BFGFLASH2, misc1: 0, misc2: 0},	// S_BFGFLASH1
	state_t { sprite: spritenum_t::SPR_BFGF, frame: 32769, tics: 6, action: actionf_t{acp1: Some(A_Light2)}, nextstate: statenum_t::S_LIGHTDONE, misc1: 0, misc2: 0},	// S_BFGFLASH2
	state_t { sprite: spritenum_t::SPR_BLUD, frame: 2, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BLOOD2, misc1: 0, misc2: 0},	// S_BLOOD1
	state_t { sprite: spritenum_t::SPR_BLUD, frame: 1, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BLOOD3, misc1: 0, misc2: 0},	// S_BLOOD2
	state_t { sprite: spritenum_t::SPR_BLUD, frame: 0, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_BLOOD3
	state_t { sprite: spritenum_t::SPR_PUFF, frame: 32768, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PUFF2, misc1: 0, misc2: 0},	// S_PUFF1
	state_t { sprite: spritenum_t::SPR_PUFF, frame: 1, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PUFF3, misc1: 0, misc2: 0},	// S_PUFF2
	state_t { sprite: spritenum_t::SPR_PUFF, frame: 2, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PUFF4, misc1: 0, misc2: 0},	// S_PUFF3
	state_t { sprite: spritenum_t::SPR_PUFF, frame: 3, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_PUFF4
	state_t { sprite: spritenum_t::SPR_BAL1, frame: 32768, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TBALL2, misc1: 0, misc2: 0},	// S_TBALL1
	state_t { sprite: spritenum_t::SPR_BAL1, frame: 32769, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TBALL1, misc1: 0, misc2: 0},	// S_TBALL2
	state_t { sprite: spritenum_t::SPR_BAL1, frame: 32770, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TBALLX2, misc1: 0, misc2: 0},	// S_TBALLX1
	state_t { sprite: spritenum_t::SPR_BAL1, frame: 32771, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TBALLX3, misc1: 0, misc2: 0},	// S_TBALLX2
	state_t { sprite: spritenum_t::SPR_BAL1, frame: 32772, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_TBALLX3
	state_t { sprite: spritenum_t::SPR_BAL2, frame: 32768, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_RBALL2, misc1: 0, misc2: 0},	// S_RBALL1
	state_t { sprite: spritenum_t::SPR_BAL2, frame: 32769, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_RBALL1, misc1: 0, misc2: 0},	// S_RBALL2
	state_t { sprite: spritenum_t::SPR_BAL2, frame: 32770, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_RBALLX2, misc1: 0, misc2: 0},	// S_RBALLX1
	state_t { sprite: spritenum_t::SPR_BAL2, frame: 32771, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_RBALLX3, misc1: 0, misc2: 0},	// S_RBALLX2
	state_t { sprite: spritenum_t::SPR_BAL2, frame: 32772, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_RBALLX3
	state_t { sprite: spritenum_t::SPR_PLSS, frame: 32768, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PLASBALL2, misc1: 0, misc2: 0},	// S_PLASBALL
	state_t { sprite: spritenum_t::SPR_PLSS, frame: 32769, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PLASBALL, misc1: 0, misc2: 0},	// S_PLASBALL2
	state_t { sprite: spritenum_t::SPR_PLSE, frame: 32768, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PLASEXP2, misc1: 0, misc2: 0},	// S_PLASEXP
	state_t { sprite: spritenum_t::SPR_PLSE, frame: 32769, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PLASEXP3, misc1: 0, misc2: 0},	// S_PLASEXP2
	state_t { sprite: spritenum_t::SPR_PLSE, frame: 32770, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PLASEXP4, misc1: 0, misc2: 0},	// S_PLASEXP3
	state_t { sprite: spritenum_t::SPR_PLSE, frame: 32771, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PLASEXP5, misc1: 0, misc2: 0},	// S_PLASEXP4
	state_t { sprite: spritenum_t::SPR_PLSE, frame: 32772, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_PLASEXP5
	state_t { sprite: spritenum_t::SPR_MISL, frame: 32768, tics: 1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_ROCKET, misc1: 0, misc2: 0},	// S_ROCKET
	state_t { sprite: spritenum_t::SPR_BFS1, frame: 32768, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BFGSHOT2, misc1: 0, misc2: 0},	// S_BFGSHOT
	state_t { sprite: spritenum_t::SPR_BFS1, frame: 32769, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BFGSHOT, misc1: 0, misc2: 0},	// S_BFGSHOT2
	state_t { sprite: spritenum_t::SPR_BFE1, frame: 32768, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BFGLAND2, misc1: 0, misc2: 0},	// S_BFGLAND
	state_t { sprite: spritenum_t::SPR_BFE1, frame: 32769, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BFGLAND3, misc1: 0, misc2: 0},	// S_BFGLAND2
	state_t { sprite: spritenum_t::SPR_BFE1, frame: 32770, tics: 8, action: actionf_t{acp1: Some(A_BFGSpray)}, nextstate: statenum_t::S_BFGLAND4, misc1: 0, misc2: 0},	// S_BFGLAND3
	state_t { sprite: spritenum_t::SPR_BFE1, frame: 32771, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BFGLAND5, misc1: 0, misc2: 0},	// S_BFGLAND4
	state_t { sprite: spritenum_t::SPR_BFE1, frame: 32772, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BFGLAND6, misc1: 0, misc2: 0},	// S_BFGLAND5
	state_t { sprite: spritenum_t::SPR_BFE1, frame: 32773, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_BFGLAND6
	state_t { sprite: spritenum_t::SPR_BFE2, frame: 32768, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BFGEXP2, misc1: 0, misc2: 0},	// S_BFGEXP
	state_t { sprite: spritenum_t::SPR_BFE2, frame: 32769, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BFGEXP3, misc1: 0, misc2: 0},	// S_BFGEXP2
	state_t { sprite: spritenum_t::SPR_BFE2, frame: 32770, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BFGEXP4, misc1: 0, misc2: 0},	// S_BFGEXP3
	state_t { sprite: spritenum_t::SPR_BFE2, frame: 32771, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_BFGEXP4
	state_t { sprite: spritenum_t::SPR_MISL, frame: 32769, tics: 8, action: actionf_t{acp1: Some(A_Explode)}, nextstate: statenum_t::S_EXPLODE2, misc1: 0, misc2: 0},	// S_EXPLODE1
	state_t { sprite: spritenum_t::SPR_MISL, frame: 32770, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_EXPLODE3, misc1: 0, misc2: 0},	// S_EXPLODE2
	state_t { sprite: spritenum_t::SPR_MISL, frame: 32771, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_EXPLODE3
	state_t { sprite: spritenum_t::SPR_TFOG, frame: 32768, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TFOG01, misc1: 0, misc2: 0},	// S_TFOG
	state_t { sprite: spritenum_t::SPR_TFOG, frame: 32769, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TFOG02, misc1: 0, misc2: 0},	// S_TFOG01
	state_t { sprite: spritenum_t::SPR_TFOG, frame: 32768, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TFOG2, misc1: 0, misc2: 0},	// S_TFOG02
	state_t { sprite: spritenum_t::SPR_TFOG, frame: 32769, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TFOG3, misc1: 0, misc2: 0},	// S_TFOG2
	state_t { sprite: spritenum_t::SPR_TFOG, frame: 32770, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TFOG4, misc1: 0, misc2: 0},	// S_TFOG3
	state_t { sprite: spritenum_t::SPR_TFOG, frame: 32771, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TFOG5, misc1: 0, misc2: 0},	// S_TFOG4
	state_t { sprite: spritenum_t::SPR_TFOG, frame: 32772, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TFOG6, misc1: 0, misc2: 0},	// S_TFOG5
	state_t { sprite: spritenum_t::SPR_TFOG, frame: 32773, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TFOG7, misc1: 0, misc2: 0},	// S_TFOG6
	state_t { sprite: spritenum_t::SPR_TFOG, frame: 32774, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TFOG8, misc1: 0, misc2: 0},	// S_TFOG7
	state_t { sprite: spritenum_t::SPR_TFOG, frame: 32775, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TFOG9, misc1: 0, misc2: 0},	// S_TFOG8
	state_t { sprite: spritenum_t::SPR_TFOG, frame: 32776, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TFOG10, misc1: 0, misc2: 0},	// S_TFOG9
	state_t { sprite: spritenum_t::SPR_TFOG, frame: 32777, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_TFOG10
	state_t { sprite: spritenum_t::SPR_IFOG, frame: 32768, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_IFOG01, misc1: 0, misc2: 0},	// S_IFOG
	state_t { sprite: spritenum_t::SPR_IFOG, frame: 32769, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_IFOG02, misc1: 0, misc2: 0},	// S_IFOG01
	state_t { sprite: spritenum_t::SPR_IFOG, frame: 32768, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_IFOG2, misc1: 0, misc2: 0},	// S_IFOG02
	state_t { sprite: spritenum_t::SPR_IFOG, frame: 32769, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_IFOG3, misc1: 0, misc2: 0},	// S_IFOG2
	state_t { sprite: spritenum_t::SPR_IFOG, frame: 32770, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_IFOG4, misc1: 0, misc2: 0},	// S_IFOG3
	state_t { sprite: spritenum_t::SPR_IFOG, frame: 32771, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_IFOG5, misc1: 0, misc2: 0},	// S_IFOG4
	state_t { sprite: spritenum_t::SPR_IFOG, frame: 32772, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_IFOG5
	state_t { sprite: spritenum_t::SPR_PLAY, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_PLAY
	state_t { sprite: spritenum_t::SPR_PLAY, frame: 0, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PLAY_RUN2, misc1: 0, misc2: 0},	// S_PLAY_RUN1
	state_t { sprite: spritenum_t::SPR_PLAY, frame: 1, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PLAY_RUN3, misc1: 0, misc2: 0},	// S_PLAY_RUN2
	state_t { sprite: spritenum_t::SPR_PLAY, frame: 2, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PLAY_RUN4, misc1: 0, misc2: 0},	// S_PLAY_RUN3
	state_t { sprite: spritenum_t::SPR_PLAY, frame: 3, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PLAY_RUN1, misc1: 0, misc2: 0},	// S_PLAY_RUN4
	state_t { sprite: spritenum_t::SPR_PLAY, frame: 4, tics: 12, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PLAY, misc1: 0, misc2: 0},	// S_PLAY_ATK1
	state_t { sprite: spritenum_t::SPR_PLAY, frame: 32773, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PLAY_ATK1, misc1: 0, misc2: 0},	// S_PLAY_ATK2
	state_t { sprite: spritenum_t::SPR_PLAY, frame: 6, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PLAY_PAIN2, misc1: 0, misc2: 0},	// S_PLAY_PAIN
	state_t { sprite: spritenum_t::SPR_PLAY, frame: 6, tics: 4, action: actionf_t{acp1: Some(A_Pain)}, nextstate: statenum_t::S_PLAY, misc1: 0, misc2: 0},	// S_PLAY_PAIN2
	state_t { sprite: spritenum_t::SPR_PLAY, frame: 7, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PLAY_DIE2, misc1: 0, misc2: 0},	// S_PLAY_DIE1
	state_t { sprite: spritenum_t::SPR_PLAY, frame: 8, tics: 10, action: actionf_t{acp1: Some(A_PlayerScream)}, nextstate: statenum_t::S_PLAY_DIE3, misc1: 0, misc2: 0},	// S_PLAY_DIE2
	state_t { sprite: spritenum_t::SPR_PLAY, frame: 9, tics: 10, action: actionf_t{acp1: Some(A_Fall)}, nextstate: statenum_t::S_PLAY_DIE4, misc1: 0, misc2: 0},	// S_PLAY_DIE3
	state_t { sprite: spritenum_t::SPR_PLAY, frame: 10, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PLAY_DIE5, misc1: 0, misc2: 0},	// S_PLAY_DIE4
	state_t { sprite: spritenum_t::SPR_PLAY, frame: 11, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PLAY_DIE6, misc1: 0, misc2: 0},	// S_PLAY_DIE5
	state_t { sprite: spritenum_t::SPR_PLAY, frame: 12, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PLAY_DIE7, misc1: 0, misc2: 0},	// S_PLAY_DIE6
	state_t { sprite: spritenum_t::SPR_PLAY, frame: 13, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_PLAY_DIE7
	state_t { sprite: spritenum_t::SPR_PLAY, frame: 14, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PLAY_XDIE2, misc1: 0, misc2: 0},	// S_PLAY_XDIE1
	state_t { sprite: spritenum_t::SPR_PLAY, frame: 15, tics: 5, action: actionf_t{acp1: Some(A_XScream)}, nextstate: statenum_t::S_PLAY_XDIE3, misc1: 0, misc2: 0},	// S_PLAY_XDIE2
	state_t { sprite: spritenum_t::SPR_PLAY, frame: 16, tics: 5, action: actionf_t{acp1: Some(A_Fall)}, nextstate: statenum_t::S_PLAY_XDIE4, misc1: 0, misc2: 0},	// S_PLAY_XDIE3
	state_t { sprite: spritenum_t::SPR_PLAY, frame: 17, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PLAY_XDIE5, misc1: 0, misc2: 0},	// S_PLAY_XDIE4
	state_t { sprite: spritenum_t::SPR_PLAY, frame: 18, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PLAY_XDIE6, misc1: 0, misc2: 0},	// S_PLAY_XDIE5
	state_t { sprite: spritenum_t::SPR_PLAY, frame: 19, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PLAY_XDIE7, misc1: 0, misc2: 0},	// S_PLAY_XDIE6
	state_t { sprite: spritenum_t::SPR_PLAY, frame: 20, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PLAY_XDIE8, misc1: 0, misc2: 0},	// S_PLAY_XDIE7
	state_t { sprite: spritenum_t::SPR_PLAY, frame: 21, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PLAY_XDIE9, misc1: 0, misc2: 0},	// S_PLAY_XDIE8
	state_t { sprite: spritenum_t::SPR_PLAY, frame: 22, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_PLAY_XDIE9
	state_t { sprite: spritenum_t::SPR_POSS, frame: 0, tics: 10, action: actionf_t{acp1: Some(A_Look)}, nextstate: statenum_t::S_POSS_STND2, misc1: 0, misc2: 0},	// S_POSS_STND
	state_t { sprite: spritenum_t::SPR_POSS, frame: 1, tics: 10, action: actionf_t{acp1: Some(A_Look)}, nextstate: statenum_t::S_POSS_STND, misc1: 0, misc2: 0},	// S_POSS_STND2
	state_t { sprite: spritenum_t::SPR_POSS, frame: 0, tics: 4, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_POSS_RUN2, misc1: 0, misc2: 0},	// S_POSS_RUN1
	state_t { sprite: spritenum_t::SPR_POSS, frame: 0, tics: 4, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_POSS_RUN3, misc1: 0, misc2: 0},	// S_POSS_RUN2
	state_t { sprite: spritenum_t::SPR_POSS, frame: 1, tics: 4, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_POSS_RUN4, misc1: 0, misc2: 0},	// S_POSS_RUN3
	state_t { sprite: spritenum_t::SPR_POSS, frame: 1, tics: 4, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_POSS_RUN5, misc1: 0, misc2: 0},	// S_POSS_RUN4
	state_t { sprite: spritenum_t::SPR_POSS, frame: 2, tics: 4, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_POSS_RUN6, misc1: 0, misc2: 0},	// S_POSS_RUN5
	state_t { sprite: spritenum_t::SPR_POSS, frame: 2, tics: 4, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_POSS_RUN7, misc1: 0, misc2: 0},	// S_POSS_RUN6
	state_t { sprite: spritenum_t::SPR_POSS, frame: 3, tics: 4, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_POSS_RUN8, misc1: 0, misc2: 0},	// S_POSS_RUN7
	state_t { sprite: spritenum_t::SPR_POSS, frame: 3, tics: 4, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_POSS_RUN1, misc1: 0, misc2: 0},	// S_POSS_RUN8
	state_t { sprite: spritenum_t::SPR_POSS, frame: 4, tics: 10, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_POSS_ATK2, misc1: 0, misc2: 0},	// S_POSS_ATK1
	state_t { sprite: spritenum_t::SPR_POSS, frame: 5, tics: 8, action: actionf_t{acp1: Some(A_PosAttack)}, nextstate: statenum_t::S_POSS_ATK3, misc1: 0, misc2: 0},	// S_POSS_ATK2
	state_t { sprite: spritenum_t::SPR_POSS, frame: 4, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_POSS_RUN1, misc1: 0, misc2: 0},	// S_POSS_ATK3
	state_t { sprite: spritenum_t::SPR_POSS, frame: 6, tics: 3, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_POSS_PAIN2, misc1: 0, misc2: 0},	// S_POSS_PAIN
	state_t { sprite: spritenum_t::SPR_POSS, frame: 6, tics: 3, action: actionf_t{acp1: Some(A_Pain)}, nextstate: statenum_t::S_POSS_RUN1, misc1: 0, misc2: 0},	// S_POSS_PAIN2
	state_t { sprite: spritenum_t::SPR_POSS, frame: 7, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_POSS_DIE2, misc1: 0, misc2: 0},	// S_POSS_DIE1
	state_t { sprite: spritenum_t::SPR_POSS, frame: 8, tics: 5, action: actionf_t{acp1: Some(A_Scream)}, nextstate: statenum_t::S_POSS_DIE3, misc1: 0, misc2: 0},	// S_POSS_DIE2
	state_t { sprite: spritenum_t::SPR_POSS, frame: 9, tics: 5, action: actionf_t{acp1: Some(A_Fall)}, nextstate: statenum_t::S_POSS_DIE4, misc1: 0, misc2: 0},	// S_POSS_DIE3
	state_t { sprite: spritenum_t::SPR_POSS, frame: 10, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_POSS_DIE5, misc1: 0, misc2: 0},	// S_POSS_DIE4
	state_t { sprite: spritenum_t::SPR_POSS, frame: 11, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_POSS_DIE5
	state_t { sprite: spritenum_t::SPR_POSS, frame: 12, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_POSS_XDIE2, misc1: 0, misc2: 0},	// S_POSS_XDIE1
	state_t { sprite: spritenum_t::SPR_POSS, frame: 13, tics: 5, action: actionf_t{acp1: Some(A_XScream)}, nextstate: statenum_t::S_POSS_XDIE3, misc1: 0, misc2: 0},	// S_POSS_XDIE2
	state_t { sprite: spritenum_t::SPR_POSS, frame: 14, tics: 5, action: actionf_t{acp1: Some(A_Fall)}, nextstate: statenum_t::S_POSS_XDIE4, misc1: 0, misc2: 0},	// S_POSS_XDIE3
	state_t { sprite: spritenum_t::SPR_POSS, frame: 15, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_POSS_XDIE5, misc1: 0, misc2: 0},	// S_POSS_XDIE4
	state_t { sprite: spritenum_t::SPR_POSS, frame: 16, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_POSS_XDIE6, misc1: 0, misc2: 0},	// S_POSS_XDIE5
	state_t { sprite: spritenum_t::SPR_POSS, frame: 17, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_POSS_XDIE7, misc1: 0, misc2: 0},	// S_POSS_XDIE6
	state_t { sprite: spritenum_t::SPR_POSS, frame: 18, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_POSS_XDIE8, misc1: 0, misc2: 0},	// S_POSS_XDIE7
	state_t { sprite: spritenum_t::SPR_POSS, frame: 19, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_POSS_XDIE9, misc1: 0, misc2: 0},	// S_POSS_XDIE8
	state_t { sprite: spritenum_t::SPR_POSS, frame: 20, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_POSS_XDIE9
	state_t { sprite: spritenum_t::SPR_POSS, frame: 10, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_POSS_RAISE2, misc1: 0, misc2: 0},	// S_POSS_RAISE1
	state_t { sprite: spritenum_t::SPR_POSS, frame: 9, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_POSS_RAISE3, misc1: 0, misc2: 0},	// S_POSS_RAISE2
	state_t { sprite: spritenum_t::SPR_POSS, frame: 8, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_POSS_RAISE4, misc1: 0, misc2: 0},	// S_POSS_RAISE3
	state_t { sprite: spritenum_t::SPR_POSS, frame: 7, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_POSS_RUN1, misc1: 0, misc2: 0},	// S_POSS_RAISE4
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 0, tics: 10, action: actionf_t{acp1: Some(A_Look)}, nextstate: statenum_t::S_SPOS_STND2, misc1: 0, misc2: 0},	// S_SPOS_STND
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 1, tics: 10, action: actionf_t{acp1: Some(A_Look)}, nextstate: statenum_t::S_SPOS_STND, misc1: 0, misc2: 0},	// S_SPOS_STND2
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 0, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SPOS_RUN2, misc1: 0, misc2: 0},	// S_SPOS_RUN1
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 0, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SPOS_RUN3, misc1: 0, misc2: 0},	// S_SPOS_RUN2
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 1, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SPOS_RUN4, misc1: 0, misc2: 0},	// S_SPOS_RUN3
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 1, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SPOS_RUN5, misc1: 0, misc2: 0},	// S_SPOS_RUN4
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 2, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SPOS_RUN6, misc1: 0, misc2: 0},	// S_SPOS_RUN5
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 2, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SPOS_RUN7, misc1: 0, misc2: 0},	// S_SPOS_RUN6
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 3, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SPOS_RUN8, misc1: 0, misc2: 0},	// S_SPOS_RUN7
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 3, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SPOS_RUN1, misc1: 0, misc2: 0},	// S_SPOS_RUN8
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 4, tics: 10, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_SPOS_ATK2, misc1: 0, misc2: 0},	// S_SPOS_ATK1
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 32773, tics: 10, action: actionf_t{acp1: Some(A_SPosAttack)}, nextstate: statenum_t::S_SPOS_ATK3, misc1: 0, misc2: 0},	// S_SPOS_ATK2
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 4, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SPOS_RUN1, misc1: 0, misc2: 0},	// S_SPOS_ATK3
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 6, tics: 3, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SPOS_PAIN2, misc1: 0, misc2: 0},	// S_SPOS_PAIN
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 6, tics: 3, action: actionf_t{acp1: Some(A_Pain)}, nextstate: statenum_t::S_SPOS_RUN1, misc1: 0, misc2: 0},	// S_SPOS_PAIN2
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 7, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SPOS_DIE2, misc1: 0, misc2: 0},	// S_SPOS_DIE1
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 8, tics: 5, action: actionf_t{acp1: Some(A_Scream)}, nextstate: statenum_t::S_SPOS_DIE3, misc1: 0, misc2: 0},	// S_SPOS_DIE2
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 9, tics: 5, action: actionf_t{acp1: Some(A_Fall)}, nextstate: statenum_t::S_SPOS_DIE4, misc1: 0, misc2: 0},	// S_SPOS_DIE3
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 10, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SPOS_DIE5, misc1: 0, misc2: 0},	// S_SPOS_DIE4
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 11, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_SPOS_DIE5
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 12, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SPOS_XDIE2, misc1: 0, misc2: 0},	// S_SPOS_XDIE1
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 13, tics: 5, action: actionf_t{acp1: Some(A_XScream)}, nextstate: statenum_t::S_SPOS_XDIE3, misc1: 0, misc2: 0},	// S_SPOS_XDIE2
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 14, tics: 5, action: actionf_t{acp1: Some(A_Fall)}, nextstate: statenum_t::S_SPOS_XDIE4, misc1: 0, misc2: 0},	// S_SPOS_XDIE3
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 15, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SPOS_XDIE5, misc1: 0, misc2: 0},	// S_SPOS_XDIE4
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 16, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SPOS_XDIE6, misc1: 0, misc2: 0},	// S_SPOS_XDIE5
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 17, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SPOS_XDIE7, misc1: 0, misc2: 0},	// S_SPOS_XDIE6
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 18, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SPOS_XDIE8, misc1: 0, misc2: 0},	// S_SPOS_XDIE7
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 19, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SPOS_XDIE9, misc1: 0, misc2: 0},	// S_SPOS_XDIE8
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 20, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_SPOS_XDIE9
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 11, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SPOS_RAISE2, misc1: 0, misc2: 0},	// S_SPOS_RAISE1
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 10, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SPOS_RAISE3, misc1: 0, misc2: 0},	// S_SPOS_RAISE2
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 9, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SPOS_RAISE4, misc1: 0, misc2: 0},	// S_SPOS_RAISE3
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 8, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SPOS_RAISE5, misc1: 0, misc2: 0},	// S_SPOS_RAISE4
	state_t { sprite: spritenum_t::SPR_SPOS, frame: 7, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SPOS_RUN1, misc1: 0, misc2: 0},	// S_SPOS_RAISE5
	state_t { sprite: spritenum_t::SPR_VILE, frame: 0, tics: 10, action: actionf_t{acp1: Some(A_Look)}, nextstate: statenum_t::S_VILE_STND2, misc1: 0, misc2: 0},	// S_VILE_STND
	state_t { sprite: spritenum_t::SPR_VILE, frame: 1, tics: 10, action: actionf_t{acp1: Some(A_Look)}, nextstate: statenum_t::S_VILE_STND, misc1: 0, misc2: 0},	// S_VILE_STND2
	state_t { sprite: spritenum_t::SPR_VILE, frame: 0, tics: 2, action: actionf_t{acp1: Some(A_VileChase)}, nextstate: statenum_t::S_VILE_RUN2, misc1: 0, misc2: 0},	// S_VILE_RUN1
	state_t { sprite: spritenum_t::SPR_VILE, frame: 0, tics: 2, action: actionf_t{acp1: Some(A_VileChase)}, nextstate: statenum_t::S_VILE_RUN3, misc1: 0, misc2: 0},	// S_VILE_RUN2
	state_t { sprite: spritenum_t::SPR_VILE, frame: 1, tics: 2, action: actionf_t{acp1: Some(A_VileChase)}, nextstate: statenum_t::S_VILE_RUN4, misc1: 0, misc2: 0},	// S_VILE_RUN3
	state_t { sprite: spritenum_t::SPR_VILE, frame: 1, tics: 2, action: actionf_t{acp1: Some(A_VileChase)}, nextstate: statenum_t::S_VILE_RUN5, misc1: 0, misc2: 0},	// S_VILE_RUN4
	state_t { sprite: spritenum_t::SPR_VILE, frame: 2, tics: 2, action: actionf_t{acp1: Some(A_VileChase)}, nextstate: statenum_t::S_VILE_RUN6, misc1: 0, misc2: 0},	// S_VILE_RUN5
	state_t { sprite: spritenum_t::SPR_VILE, frame: 2, tics: 2, action: actionf_t{acp1: Some(A_VileChase)}, nextstate: statenum_t::S_VILE_RUN7, misc1: 0, misc2: 0},	// S_VILE_RUN6
	state_t { sprite: spritenum_t::SPR_VILE, frame: 3, tics: 2, action: actionf_t{acp1: Some(A_VileChase)}, nextstate: statenum_t::S_VILE_RUN8, misc1: 0, misc2: 0},	// S_VILE_RUN7
	state_t { sprite: spritenum_t::SPR_VILE, frame: 3, tics: 2, action: actionf_t{acp1: Some(A_VileChase)}, nextstate: statenum_t::S_VILE_RUN9, misc1: 0, misc2: 0},	// S_VILE_RUN8
	state_t { sprite: spritenum_t::SPR_VILE, frame: 4, tics: 2, action: actionf_t{acp1: Some(A_VileChase)}, nextstate: statenum_t::S_VILE_RUN10, misc1: 0, misc2: 0},	// S_VILE_RUN9
	state_t { sprite: spritenum_t::SPR_VILE, frame: 4, tics: 2, action: actionf_t{acp1: Some(A_VileChase)}, nextstate: statenum_t::S_VILE_RUN11, misc1: 0, misc2: 0},	// S_VILE_RUN10
	state_t { sprite: spritenum_t::SPR_VILE, frame: 5, tics: 2, action: actionf_t{acp1: Some(A_VileChase)}, nextstate: statenum_t::S_VILE_RUN12, misc1: 0, misc2: 0},	// S_VILE_RUN11
	state_t { sprite: spritenum_t::SPR_VILE, frame: 5, tics: 2, action: actionf_t{acp1: Some(A_VileChase)}, nextstate: statenum_t::S_VILE_RUN1, misc1: 0, misc2: 0},	// S_VILE_RUN12
	state_t { sprite: spritenum_t::SPR_VILE, frame: 32774, tics: 0, action: actionf_t{acp1: Some(A_VileStart)}, nextstate: statenum_t::S_VILE_ATK2, misc1: 0, misc2: 0},	// S_VILE_ATK1
	state_t { sprite: spritenum_t::SPR_VILE, frame: 32774, tics: 10, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_VILE_ATK3, misc1: 0, misc2: 0},	// S_VILE_ATK2
	state_t { sprite: spritenum_t::SPR_VILE, frame: 32775, tics: 8, action: actionf_t{acp1: Some(A_VileTarget)}, nextstate: statenum_t::S_VILE_ATK4, misc1: 0, misc2: 0},	// S_VILE_ATK3
	state_t { sprite: spritenum_t::SPR_VILE, frame: 32776, tics: 8, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_VILE_ATK5, misc1: 0, misc2: 0},	// S_VILE_ATK4
	state_t { sprite: spritenum_t::SPR_VILE, frame: 32777, tics: 8, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_VILE_ATK6, misc1: 0, misc2: 0},	// S_VILE_ATK5
	state_t { sprite: spritenum_t::SPR_VILE, frame: 32778, tics: 8, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_VILE_ATK7, misc1: 0, misc2: 0},	// S_VILE_ATK6
	state_t { sprite: spritenum_t::SPR_VILE, frame: 32779, tics: 8, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_VILE_ATK8, misc1: 0, misc2: 0},	// S_VILE_ATK7
	state_t { sprite: spritenum_t::SPR_VILE, frame: 32780, tics: 8, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_VILE_ATK9, misc1: 0, misc2: 0},	// S_VILE_ATK8
	state_t { sprite: spritenum_t::SPR_VILE, frame: 32781, tics: 8, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_VILE_ATK10, misc1: 0, misc2: 0},	// S_VILE_ATK9
	state_t { sprite: spritenum_t::SPR_VILE, frame: 32782, tics: 8, action: actionf_t{acp1: Some(A_VileAttack)}, nextstate: statenum_t::S_VILE_ATK11, misc1: 0, misc2: 0},	// S_VILE_ATK10
	state_t { sprite: spritenum_t::SPR_VILE, frame: 32783, tics: 20, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_VILE_RUN1, misc1: 0, misc2: 0},	// S_VILE_ATK11
	state_t { sprite: spritenum_t::SPR_VILE, frame: 32794, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_VILE_HEAL2, misc1: 0, misc2: 0},	// S_VILE_HEAL1
	state_t { sprite: spritenum_t::SPR_VILE, frame: 32795, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_VILE_HEAL3, misc1: 0, misc2: 0},	// S_VILE_HEAL2
	state_t { sprite: spritenum_t::SPR_VILE, frame: 32796, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_VILE_RUN1, misc1: 0, misc2: 0},	// S_VILE_HEAL3
	state_t { sprite: spritenum_t::SPR_VILE, frame: 16, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_VILE_PAIN2, misc1: 0, misc2: 0},	// S_VILE_PAIN
	state_t { sprite: spritenum_t::SPR_VILE, frame: 16, tics: 5, action: actionf_t{acp1: Some(A_Pain)}, nextstate: statenum_t::S_VILE_RUN1, misc1: 0, misc2: 0},	// S_VILE_PAIN2
	state_t { sprite: spritenum_t::SPR_VILE, frame: 16, tics: 7, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_VILE_DIE2, misc1: 0, misc2: 0},	// S_VILE_DIE1
	state_t { sprite: spritenum_t::SPR_VILE, frame: 17, tics: 7, action: actionf_t{acp1: Some(A_Scream)}, nextstate: statenum_t::S_VILE_DIE3, misc1: 0, misc2: 0},	// S_VILE_DIE2
	state_t { sprite: spritenum_t::SPR_VILE, frame: 18, tics: 7, action: actionf_t{acp1: Some(A_Fall)}, nextstate: statenum_t::S_VILE_DIE4, misc1: 0, misc2: 0},	// S_VILE_DIE3
	state_t { sprite: spritenum_t::SPR_VILE, frame: 19, tics: 7, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_VILE_DIE5, misc1: 0, misc2: 0},	// S_VILE_DIE4
	state_t { sprite: spritenum_t::SPR_VILE, frame: 20, tics: 7, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_VILE_DIE6, misc1: 0, misc2: 0},	// S_VILE_DIE5
	state_t { sprite: spritenum_t::SPR_VILE, frame: 21, tics: 7, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_VILE_DIE7, misc1: 0, misc2: 0},	// S_VILE_DIE6
	state_t { sprite: spritenum_t::SPR_VILE, frame: 22, tics: 7, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_VILE_DIE8, misc1: 0, misc2: 0},	// S_VILE_DIE7
	state_t { sprite: spritenum_t::SPR_VILE, frame: 23, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_VILE_DIE9, misc1: 0, misc2: 0},	// S_VILE_DIE8
	state_t { sprite: spritenum_t::SPR_VILE, frame: 24, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_VILE_DIE10, misc1: 0, misc2: 0},	// S_VILE_DIE9
	state_t { sprite: spritenum_t::SPR_VILE, frame: 25, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_VILE_DIE10
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32768, tics: 2, action: actionf_t{acp1: Some(A_StartFire)}, nextstate: statenum_t::S_FIRE2, misc1: 0, misc2: 0},	// S_FIRE1
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32769, tics: 2, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_FIRE3, misc1: 0, misc2: 0},	// S_FIRE2
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32768, tics: 2, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_FIRE4, misc1: 0, misc2: 0},	// S_FIRE3
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32769, tics: 2, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_FIRE5, misc1: 0, misc2: 0},	// S_FIRE4
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32770, tics: 2, action: actionf_t{acp1: Some(A_FireCrackle)}, nextstate: statenum_t::S_FIRE6, misc1: 0, misc2: 0},	// S_FIRE5
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32769, tics: 2, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_FIRE7, misc1: 0, misc2: 0},	// S_FIRE6
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32770, tics: 2, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_FIRE8, misc1: 0, misc2: 0},	// S_FIRE7
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32769, tics: 2, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_FIRE9, misc1: 0, misc2: 0},	// S_FIRE8
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32770, tics: 2, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_FIRE10, misc1: 0, misc2: 0},	// S_FIRE9
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32771, tics: 2, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_FIRE11, misc1: 0, misc2: 0},	// S_FIRE10
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32770, tics: 2, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_FIRE12, misc1: 0, misc2: 0},	// S_FIRE11
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32771, tics: 2, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_FIRE13, misc1: 0, misc2: 0},	// S_FIRE12
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32770, tics: 2, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_FIRE14, misc1: 0, misc2: 0},	// S_FIRE13
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32771, tics: 2, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_FIRE15, misc1: 0, misc2: 0},	// S_FIRE14
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32772, tics: 2, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_FIRE16, misc1: 0, misc2: 0},	// S_FIRE15
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32771, tics: 2, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_FIRE17, misc1: 0, misc2: 0},	// S_FIRE16
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32772, tics: 2, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_FIRE18, misc1: 0, misc2: 0},	// S_FIRE17
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32771, tics: 2, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_FIRE19, misc1: 0, misc2: 0},	// S_FIRE18
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32772, tics: 2, action: actionf_t{acp1: Some(A_FireCrackle)}, nextstate: statenum_t::S_FIRE20, misc1: 0, misc2: 0},	// S_FIRE19
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32773, tics: 2, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_FIRE21, misc1: 0, misc2: 0},	// S_FIRE20
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32772, tics: 2, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_FIRE22, misc1: 0, misc2: 0},	// S_FIRE21
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32773, tics: 2, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_FIRE23, misc1: 0, misc2: 0},	// S_FIRE22
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32772, tics: 2, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_FIRE24, misc1: 0, misc2: 0},	// S_FIRE23
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32773, tics: 2, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_FIRE25, misc1: 0, misc2: 0},	// S_FIRE24
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32774, tics: 2, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_FIRE26, misc1: 0, misc2: 0},	// S_FIRE25
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32775, tics: 2, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_FIRE27, misc1: 0, misc2: 0},	// S_FIRE26
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32774, tics: 2, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_FIRE28, misc1: 0, misc2: 0},	// S_FIRE27
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32775, tics: 2, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_FIRE29, misc1: 0, misc2: 0},	// S_FIRE28
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32774, tics: 2, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_FIRE30, misc1: 0, misc2: 0},	// S_FIRE29
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32775, tics: 2, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_FIRE30
	state_t { sprite: spritenum_t::SPR_PUFF, frame: 1, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SMOKE2, misc1: 0, misc2: 0},	// S_SMOKE1
	state_t { sprite: spritenum_t::SPR_PUFF, frame: 2, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SMOKE3, misc1: 0, misc2: 0},	// S_SMOKE2
	state_t { sprite: spritenum_t::SPR_PUFF, frame: 1, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SMOKE4, misc1: 0, misc2: 0},	// S_SMOKE3
	state_t { sprite: spritenum_t::SPR_PUFF, frame: 2, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SMOKE5, misc1: 0, misc2: 0},	// S_SMOKE4
	state_t { sprite: spritenum_t::SPR_PUFF, frame: 3, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_SMOKE5
	state_t { sprite: spritenum_t::SPR_FATB, frame: 32768, tics: 2, action: actionf_t{acp1: Some(A_Tracer)}, nextstate: statenum_t::S_TRACER2, misc1: 0, misc2: 0},	// S_TRACER
	state_t { sprite: spritenum_t::SPR_FATB, frame: 32769, tics: 2, action: actionf_t{acp1: Some(A_Tracer)}, nextstate: statenum_t::S_TRACER, misc1: 0, misc2: 0},	// S_TRACER2
	state_t { sprite: spritenum_t::SPR_FBXP, frame: 32768, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TRACEEXP2, misc1: 0, misc2: 0},	// S_TRACEEXP1
	state_t { sprite: spritenum_t::SPR_FBXP, frame: 32769, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TRACEEXP3, misc1: 0, misc2: 0},	// S_TRACEEXP2
	state_t { sprite: spritenum_t::SPR_FBXP, frame: 32770, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_TRACEEXP3
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 0, tics: 10, action: actionf_t{acp1: Some(A_Look)}, nextstate: statenum_t::S_SKEL_STND2, misc1: 0, misc2: 0},	// S_SKEL_STND
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 1, tics: 10, action: actionf_t{acp1: Some(A_Look)}, nextstate: statenum_t::S_SKEL_STND, misc1: 0, misc2: 0},	// S_SKEL_STND2
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 0, tics: 2, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SKEL_RUN2, misc1: 0, misc2: 0},	// S_SKEL_RUN1
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 0, tics: 2, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SKEL_RUN3, misc1: 0, misc2: 0},	// S_SKEL_RUN2
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 1, tics: 2, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SKEL_RUN4, misc1: 0, misc2: 0},	// S_SKEL_RUN3
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 1, tics: 2, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SKEL_RUN5, misc1: 0, misc2: 0},	// S_SKEL_RUN4
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 2, tics: 2, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SKEL_RUN6, misc1: 0, misc2: 0},	// S_SKEL_RUN5
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 2, tics: 2, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SKEL_RUN7, misc1: 0, misc2: 0},	// S_SKEL_RUN6
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 3, tics: 2, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SKEL_RUN8, misc1: 0, misc2: 0},	// S_SKEL_RUN7
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 3, tics: 2, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SKEL_RUN9, misc1: 0, misc2: 0},	// S_SKEL_RUN8
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 4, tics: 2, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SKEL_RUN10, misc1: 0, misc2: 0},	// S_SKEL_RUN9
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 4, tics: 2, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SKEL_RUN11, misc1: 0, misc2: 0},	// S_SKEL_RUN10
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 5, tics: 2, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SKEL_RUN12, misc1: 0, misc2: 0},	// S_SKEL_RUN11
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 5, tics: 2, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SKEL_RUN1, misc1: 0, misc2: 0},	// S_SKEL_RUN12
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 6, tics: 0, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_SKEL_FIST2, misc1: 0, misc2: 0},	// S_SKEL_FIST1
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 6, tics: 6, action: actionf_t{acp1: Some(A_SkelWhoosh)}, nextstate: statenum_t::S_SKEL_FIST3, misc1: 0, misc2: 0},	// S_SKEL_FIST2
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 7, tics: 6, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_SKEL_FIST4, misc1: 0, misc2: 0},	// S_SKEL_FIST3
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 8, tics: 6, action: actionf_t{acp1: Some(A_SkelFist)}, nextstate: statenum_t::S_SKEL_RUN1, misc1: 0, misc2: 0},	// S_SKEL_FIST4
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 32777, tics: 0, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_SKEL_MISS2, misc1: 0, misc2: 0},	// S_SKEL_MISS1
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 32777, tics: 10, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_SKEL_MISS3, misc1: 0, misc2: 0},	// S_SKEL_MISS2
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 10, tics: 10, action: actionf_t{acp1: Some(A_SkelMissile)}, nextstate: statenum_t::S_SKEL_MISS4, misc1: 0, misc2: 0},	// S_SKEL_MISS3
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 10, tics: 10, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_SKEL_RUN1, misc1: 0, misc2: 0},	// S_SKEL_MISS4
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 11, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SKEL_PAIN2, misc1: 0, misc2: 0},	// S_SKEL_PAIN
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 11, tics: 5, action: actionf_t{acp1: Some(A_Pain)}, nextstate: statenum_t::S_SKEL_RUN1, misc1: 0, misc2: 0},	// S_SKEL_PAIN2
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 11, tics: 7, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SKEL_DIE2, misc1: 0, misc2: 0},	// S_SKEL_DIE1
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 12, tics: 7, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SKEL_DIE3, misc1: 0, misc2: 0},	// S_SKEL_DIE2
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 13, tics: 7, action: actionf_t{acp1: Some(A_Scream)}, nextstate: statenum_t::S_SKEL_DIE4, misc1: 0, misc2: 0},	// S_SKEL_DIE3
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 14, tics: 7, action: actionf_t{acp1: Some(A_Fall)}, nextstate: statenum_t::S_SKEL_DIE5, misc1: 0, misc2: 0},	// S_SKEL_DIE4
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 15, tics: 7, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SKEL_DIE6, misc1: 0, misc2: 0},	// S_SKEL_DIE5
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 16, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_SKEL_DIE6
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 16, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SKEL_RAISE2, misc1: 0, misc2: 0},	// S_SKEL_RAISE1
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 15, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SKEL_RAISE3, misc1: 0, misc2: 0},	// S_SKEL_RAISE2
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 14, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SKEL_RAISE4, misc1: 0, misc2: 0},	// S_SKEL_RAISE3
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 13, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SKEL_RAISE5, misc1: 0, misc2: 0},	// S_SKEL_RAISE4
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 12, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SKEL_RAISE6, misc1: 0, misc2: 0},	// S_SKEL_RAISE5
	state_t { sprite: spritenum_t::SPR_SKEL, frame: 11, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SKEL_RUN1, misc1: 0, misc2: 0},	// S_SKEL_RAISE6
	state_t { sprite: spritenum_t::SPR_MANF, frame: 32768, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_FATSHOT2, misc1: 0, misc2: 0},	// S_FATSHOT1
	state_t { sprite: spritenum_t::SPR_MANF, frame: 32769, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_FATSHOT1, misc1: 0, misc2: 0},	// S_FATSHOT2
	state_t { sprite: spritenum_t::SPR_MISL, frame: 32769, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_FATSHOTX2, misc1: 0, misc2: 0},	// S_FATSHOTX1
	state_t { sprite: spritenum_t::SPR_MISL, frame: 32770, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_FATSHOTX3, misc1: 0, misc2: 0},	// S_FATSHOTX2
	state_t { sprite: spritenum_t::SPR_MISL, frame: 32771, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_FATSHOTX3
	state_t { sprite: spritenum_t::SPR_FATT, frame: 0, tics: 15, action: actionf_t{acp1: Some(A_Look)}, nextstate: statenum_t::S_FATT_STND2, misc1: 0, misc2: 0},	// S_FATT_STND
	state_t { sprite: spritenum_t::SPR_FATT, frame: 1, tics: 15, action: actionf_t{acp1: Some(A_Look)}, nextstate: statenum_t::S_FATT_STND, misc1: 0, misc2: 0},	// S_FATT_STND2
	state_t { sprite: spritenum_t::SPR_FATT, frame: 0, tics: 4, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_FATT_RUN2, misc1: 0, misc2: 0},	// S_FATT_RUN1
	state_t { sprite: spritenum_t::SPR_FATT, frame: 0, tics: 4, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_FATT_RUN3, misc1: 0, misc2: 0},	// S_FATT_RUN2
	state_t { sprite: spritenum_t::SPR_FATT, frame: 1, tics: 4, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_FATT_RUN4, misc1: 0, misc2: 0},	// S_FATT_RUN3
	state_t { sprite: spritenum_t::SPR_FATT, frame: 1, tics: 4, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_FATT_RUN5, misc1: 0, misc2: 0},	// S_FATT_RUN4
	state_t { sprite: spritenum_t::SPR_FATT, frame: 2, tics: 4, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_FATT_RUN6, misc1: 0, misc2: 0},	// S_FATT_RUN5
	state_t { sprite: spritenum_t::SPR_FATT, frame: 2, tics: 4, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_FATT_RUN7, misc1: 0, misc2: 0},	// S_FATT_RUN6
	state_t { sprite: spritenum_t::SPR_FATT, frame: 3, tics: 4, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_FATT_RUN8, misc1: 0, misc2: 0},	// S_FATT_RUN7
	state_t { sprite: spritenum_t::SPR_FATT, frame: 3, tics: 4, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_FATT_RUN9, misc1: 0, misc2: 0},	// S_FATT_RUN8
	state_t { sprite: spritenum_t::SPR_FATT, frame: 4, tics: 4, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_FATT_RUN10, misc1: 0, misc2: 0},	// S_FATT_RUN9
	state_t { sprite: spritenum_t::SPR_FATT, frame: 4, tics: 4, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_FATT_RUN11, misc1: 0, misc2: 0},	// S_FATT_RUN10
	state_t { sprite: spritenum_t::SPR_FATT, frame: 5, tics: 4, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_FATT_RUN12, misc1: 0, misc2: 0},	// S_FATT_RUN11
	state_t { sprite: spritenum_t::SPR_FATT, frame: 5, tics: 4, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_FATT_RUN1, misc1: 0, misc2: 0},	// S_FATT_RUN12
	state_t { sprite: spritenum_t::SPR_FATT, frame: 6, tics: 20, action: actionf_t{acp1: Some(A_FatRaise)}, nextstate: statenum_t::S_FATT_ATK2, misc1: 0, misc2: 0},	// S_FATT_ATK1
	state_t { sprite: spritenum_t::SPR_FATT, frame: 32775, tics: 10, action: actionf_t{acp1: Some(A_FatAttack1)}, nextstate: statenum_t::S_FATT_ATK3, misc1: 0, misc2: 0},	// S_FATT_ATK2
	state_t { sprite: spritenum_t::SPR_FATT, frame: 8, tics: 5, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_FATT_ATK4, misc1: 0, misc2: 0},	// S_FATT_ATK3
	state_t { sprite: spritenum_t::SPR_FATT, frame: 6, tics: 5, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_FATT_ATK5, misc1: 0, misc2: 0},	// S_FATT_ATK4
	state_t { sprite: spritenum_t::SPR_FATT, frame: 32775, tics: 10, action: actionf_t{acp1: Some(A_FatAttack2)}, nextstate: statenum_t::S_FATT_ATK6, misc1: 0, misc2: 0},	// S_FATT_ATK5
	state_t { sprite: spritenum_t::SPR_FATT, frame: 8, tics: 5, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_FATT_ATK7, misc1: 0, misc2: 0},	// S_FATT_ATK6
	state_t { sprite: spritenum_t::SPR_FATT, frame: 6, tics: 5, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_FATT_ATK8, misc1: 0, misc2: 0},	// S_FATT_ATK7
	state_t { sprite: spritenum_t::SPR_FATT, frame: 32775, tics: 10, action: actionf_t{acp1: Some(A_FatAttack3)}, nextstate: statenum_t::S_FATT_ATK9, misc1: 0, misc2: 0},	// S_FATT_ATK8
	state_t { sprite: spritenum_t::SPR_FATT, frame: 8, tics: 5, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_FATT_ATK10, misc1: 0, misc2: 0},	// S_FATT_ATK9
	state_t { sprite: spritenum_t::SPR_FATT, frame: 6, tics: 5, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_FATT_RUN1, misc1: 0, misc2: 0},	// S_FATT_ATK10
	state_t { sprite: spritenum_t::SPR_FATT, frame: 9, tics: 3, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_FATT_PAIN2, misc1: 0, misc2: 0},	// S_FATT_PAIN
	state_t { sprite: spritenum_t::SPR_FATT, frame: 9, tics: 3, action: actionf_t{acp1: Some(A_Pain)}, nextstate: statenum_t::S_FATT_RUN1, misc1: 0, misc2: 0},	// S_FATT_PAIN2
	state_t { sprite: spritenum_t::SPR_FATT, frame: 10, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_FATT_DIE2, misc1: 0, misc2: 0},	// S_FATT_DIE1
	state_t { sprite: spritenum_t::SPR_FATT, frame: 11, tics: 6, action: actionf_t{acp1: Some(A_Scream)}, nextstate: statenum_t::S_FATT_DIE3, misc1: 0, misc2: 0},	// S_FATT_DIE2
	state_t { sprite: spritenum_t::SPR_FATT, frame: 12, tics: 6, action: actionf_t{acp1: Some(A_Fall)}, nextstate: statenum_t::S_FATT_DIE4, misc1: 0, misc2: 0},	// S_FATT_DIE3
	state_t { sprite: spritenum_t::SPR_FATT, frame: 13, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_FATT_DIE5, misc1: 0, misc2: 0},	// S_FATT_DIE4
	state_t { sprite: spritenum_t::SPR_FATT, frame: 14, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_FATT_DIE6, misc1: 0, misc2: 0},	// S_FATT_DIE5
	state_t { sprite: spritenum_t::SPR_FATT, frame: 15, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_FATT_DIE7, misc1: 0, misc2: 0},	// S_FATT_DIE6
	state_t { sprite: spritenum_t::SPR_FATT, frame: 16, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_FATT_DIE8, misc1: 0, misc2: 0},	// S_FATT_DIE7
	state_t { sprite: spritenum_t::SPR_FATT, frame: 17, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_FATT_DIE9, misc1: 0, misc2: 0},	// S_FATT_DIE8
	state_t { sprite: spritenum_t::SPR_FATT, frame: 18, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_FATT_DIE10, misc1: 0, misc2: 0},	// S_FATT_DIE9
	state_t { sprite: spritenum_t::SPR_FATT, frame: 19, tics: -1, action: actionf_t{acp1: Some(A_BossDeath)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_FATT_DIE10
	state_t { sprite: spritenum_t::SPR_FATT, frame: 17, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_FATT_RAISE2, misc1: 0, misc2: 0},	// S_FATT_RAISE1
	state_t { sprite: spritenum_t::SPR_FATT, frame: 16, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_FATT_RAISE3, misc1: 0, misc2: 0},	// S_FATT_RAISE2
	state_t { sprite: spritenum_t::SPR_FATT, frame: 15, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_FATT_RAISE4, misc1: 0, misc2: 0},	// S_FATT_RAISE3
	state_t { sprite: spritenum_t::SPR_FATT, frame: 14, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_FATT_RAISE5, misc1: 0, misc2: 0},	// S_FATT_RAISE4
	state_t { sprite: spritenum_t::SPR_FATT, frame: 13, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_FATT_RAISE6, misc1: 0, misc2: 0},	// S_FATT_RAISE5
	state_t { sprite: spritenum_t::SPR_FATT, frame: 12, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_FATT_RAISE7, misc1: 0, misc2: 0},	// S_FATT_RAISE6
	state_t { sprite: spritenum_t::SPR_FATT, frame: 11, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_FATT_RAISE8, misc1: 0, misc2: 0},	// S_FATT_RAISE7
	state_t { sprite: spritenum_t::SPR_FATT, frame: 10, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_FATT_RUN1, misc1: 0, misc2: 0},	// S_FATT_RAISE8
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 0, tics: 10, action: actionf_t{acp1: Some(A_Look)}, nextstate: statenum_t::S_CPOS_STND2, misc1: 0, misc2: 0},	// S_CPOS_STND
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 1, tics: 10, action: actionf_t{acp1: Some(A_Look)}, nextstate: statenum_t::S_CPOS_STND, misc1: 0, misc2: 0},	// S_CPOS_STND2
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 0, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_CPOS_RUN2, misc1: 0, misc2: 0},	// S_CPOS_RUN1
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 0, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_CPOS_RUN3, misc1: 0, misc2: 0},	// S_CPOS_RUN2
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 1, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_CPOS_RUN4, misc1: 0, misc2: 0},	// S_CPOS_RUN3
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 1, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_CPOS_RUN5, misc1: 0, misc2: 0},	// S_CPOS_RUN4
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 2, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_CPOS_RUN6, misc1: 0, misc2: 0},	// S_CPOS_RUN5
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 2, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_CPOS_RUN7, misc1: 0, misc2: 0},	// S_CPOS_RUN6
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 3, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_CPOS_RUN8, misc1: 0, misc2: 0},	// S_CPOS_RUN7
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 3, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_CPOS_RUN1, misc1: 0, misc2: 0},	// S_CPOS_RUN8
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 4, tics: 10, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_CPOS_ATK2, misc1: 0, misc2: 0},	// S_CPOS_ATK1
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 32773, tics: 4, action: actionf_t{acp1: Some(A_CPosAttack)}, nextstate: statenum_t::S_CPOS_ATK3, misc1: 0, misc2: 0},	// S_CPOS_ATK2
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 32772, tics: 4, action: actionf_t{acp1: Some(A_CPosAttack)}, nextstate: statenum_t::S_CPOS_ATK4, misc1: 0, misc2: 0},	// S_CPOS_ATK3
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 5, tics: 1, action: actionf_t{acp1: Some(A_CPosRefire)}, nextstate: statenum_t::S_CPOS_ATK2, misc1: 0, misc2: 0},	// S_CPOS_ATK4
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 6, tics: 3, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_CPOS_PAIN2, misc1: 0, misc2: 0},	// S_CPOS_PAIN
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 6, tics: 3, action: actionf_t{acp1: Some(A_Pain)}, nextstate: statenum_t::S_CPOS_RUN1, misc1: 0, misc2: 0},	// S_CPOS_PAIN2
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 7, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_CPOS_DIE2, misc1: 0, misc2: 0},	// S_CPOS_DIE1
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 8, tics: 5, action: actionf_t{acp1: Some(A_Scream)}, nextstate: statenum_t::S_CPOS_DIE3, misc1: 0, misc2: 0},	// S_CPOS_DIE2
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 9, tics: 5, action: actionf_t{acp1: Some(A_Fall)}, nextstate: statenum_t::S_CPOS_DIE4, misc1: 0, misc2: 0},	// S_CPOS_DIE3
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 10, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_CPOS_DIE5, misc1: 0, misc2: 0},	// S_CPOS_DIE4
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 11, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_CPOS_DIE6, misc1: 0, misc2: 0},	// S_CPOS_DIE5
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 12, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_CPOS_DIE7, misc1: 0, misc2: 0},	// S_CPOS_DIE6
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 13, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_CPOS_DIE7
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 14, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_CPOS_XDIE2, misc1: 0, misc2: 0},	// S_CPOS_XDIE1
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 15, tics: 5, action: actionf_t{acp1: Some(A_XScream)}, nextstate: statenum_t::S_CPOS_XDIE3, misc1: 0, misc2: 0},	// S_CPOS_XDIE2
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 16, tics: 5, action: actionf_t{acp1: Some(A_Fall)}, nextstate: statenum_t::S_CPOS_XDIE4, misc1: 0, misc2: 0},	// S_CPOS_XDIE3
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 17, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_CPOS_XDIE5, misc1: 0, misc2: 0},	// S_CPOS_XDIE4
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 18, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_CPOS_XDIE6, misc1: 0, misc2: 0},	// S_CPOS_XDIE5
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 19, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_CPOS_XDIE6
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 13, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_CPOS_RAISE2, misc1: 0, misc2: 0},	// S_CPOS_RAISE1
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 12, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_CPOS_RAISE3, misc1: 0, misc2: 0},	// S_CPOS_RAISE2
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 11, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_CPOS_RAISE4, misc1: 0, misc2: 0},	// S_CPOS_RAISE3
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 10, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_CPOS_RAISE5, misc1: 0, misc2: 0},	// S_CPOS_RAISE4
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 9, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_CPOS_RAISE6, misc1: 0, misc2: 0},	// S_CPOS_RAISE5
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 8, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_CPOS_RAISE7, misc1: 0, misc2: 0},	// S_CPOS_RAISE6
	state_t { sprite: spritenum_t::SPR_CPOS, frame: 7, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_CPOS_RUN1, misc1: 0, misc2: 0},	// S_CPOS_RAISE7
	state_t { sprite: spritenum_t::SPR_TROO, frame: 0, tics: 10, action: actionf_t{acp1: Some(A_Look)}, nextstate: statenum_t::S_TROO_STND2, misc1: 0, misc2: 0},	// S_TROO_STND
	state_t { sprite: spritenum_t::SPR_TROO, frame: 1, tics: 10, action: actionf_t{acp1: Some(A_Look)}, nextstate: statenum_t::S_TROO_STND, misc1: 0, misc2: 0},	// S_TROO_STND2
	state_t { sprite: spritenum_t::SPR_TROO, frame: 0, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_TROO_RUN2, misc1: 0, misc2: 0},	// S_TROO_RUN1
	state_t { sprite: spritenum_t::SPR_TROO, frame: 0, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_TROO_RUN3, misc1: 0, misc2: 0},	// S_TROO_RUN2
	state_t { sprite: spritenum_t::SPR_TROO, frame: 1, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_TROO_RUN4, misc1: 0, misc2: 0},	// S_TROO_RUN3
	state_t { sprite: spritenum_t::SPR_TROO, frame: 1, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_TROO_RUN5, misc1: 0, misc2: 0},	// S_TROO_RUN4
	state_t { sprite: spritenum_t::SPR_TROO, frame: 2, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_TROO_RUN6, misc1: 0, misc2: 0},	// S_TROO_RUN5
	state_t { sprite: spritenum_t::SPR_TROO, frame: 2, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_TROO_RUN7, misc1: 0, misc2: 0},	// S_TROO_RUN6
	state_t { sprite: spritenum_t::SPR_TROO, frame: 3, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_TROO_RUN8, misc1: 0, misc2: 0},	// S_TROO_RUN7
	state_t { sprite: spritenum_t::SPR_TROO, frame: 3, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_TROO_RUN1, misc1: 0, misc2: 0},	// S_TROO_RUN8
	state_t { sprite: spritenum_t::SPR_TROO, frame: 4, tics: 8, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_TROO_ATK2, misc1: 0, misc2: 0},	// S_TROO_ATK1
	state_t { sprite: spritenum_t::SPR_TROO, frame: 5, tics: 8, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_TROO_ATK3, misc1: 0, misc2: 0},	// S_TROO_ATK2
	state_t { sprite: spritenum_t::SPR_TROO, frame: 6, tics: 6, action: actionf_t{acp1: Some(A_TroopAttack)}, nextstate: statenum_t::S_TROO_RUN1, misc1: 0, misc2: 0},	// S_TROO_ATK3
	state_t { sprite: spritenum_t::SPR_TROO, frame: 7, tics: 2, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TROO_PAIN2, misc1: 0, misc2: 0},	// S_TROO_PAIN
	state_t { sprite: spritenum_t::SPR_TROO, frame: 7, tics: 2, action: actionf_t{acp1: Some(A_Pain)}, nextstate: statenum_t::S_TROO_RUN1, misc1: 0, misc2: 0},	// S_TROO_PAIN2
	state_t { sprite: spritenum_t::SPR_TROO, frame: 8, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TROO_DIE2, misc1: 0, misc2: 0},	// S_TROO_DIE1
	state_t { sprite: spritenum_t::SPR_TROO, frame: 9, tics: 8, action: actionf_t{acp1: Some(A_Scream)}, nextstate: statenum_t::S_TROO_DIE3, misc1: 0, misc2: 0},	// S_TROO_DIE2
	state_t { sprite: spritenum_t::SPR_TROO, frame: 10, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TROO_DIE4, misc1: 0, misc2: 0},	// S_TROO_DIE3
	state_t { sprite: spritenum_t::SPR_TROO, frame: 11, tics: 6, action: actionf_t{acp1: Some(A_Fall)}, nextstate: statenum_t::S_TROO_DIE5, misc1: 0, misc2: 0},	// S_TROO_DIE4
	state_t { sprite: spritenum_t::SPR_TROO, frame: 12, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_TROO_DIE5
	state_t { sprite: spritenum_t::SPR_TROO, frame: 13, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TROO_XDIE2, misc1: 0, misc2: 0},	// S_TROO_XDIE1
	state_t { sprite: spritenum_t::SPR_TROO, frame: 14, tics: 5, action: actionf_t{acp1: Some(A_XScream)}, nextstate: statenum_t::S_TROO_XDIE3, misc1: 0, misc2: 0},	// S_TROO_XDIE2
	state_t { sprite: spritenum_t::SPR_TROO, frame: 15, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TROO_XDIE4, misc1: 0, misc2: 0},	// S_TROO_XDIE3
	state_t { sprite: spritenum_t::SPR_TROO, frame: 16, tics: 5, action: actionf_t{acp1: Some(A_Fall)}, nextstate: statenum_t::S_TROO_XDIE5, misc1: 0, misc2: 0},	// S_TROO_XDIE4
	state_t { sprite: spritenum_t::SPR_TROO, frame: 17, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TROO_XDIE6, misc1: 0, misc2: 0},	// S_TROO_XDIE5
	state_t { sprite: spritenum_t::SPR_TROO, frame: 18, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TROO_XDIE7, misc1: 0, misc2: 0},	// S_TROO_XDIE6
	state_t { sprite: spritenum_t::SPR_TROO, frame: 19, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TROO_XDIE8, misc1: 0, misc2: 0},	// S_TROO_XDIE7
	state_t { sprite: spritenum_t::SPR_TROO, frame: 20, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_TROO_XDIE8
	state_t { sprite: spritenum_t::SPR_TROO, frame: 12, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TROO_RAISE2, misc1: 0, misc2: 0},	// S_TROO_RAISE1
	state_t { sprite: spritenum_t::SPR_TROO, frame: 11, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TROO_RAISE3, misc1: 0, misc2: 0},	// S_TROO_RAISE2
	state_t { sprite: spritenum_t::SPR_TROO, frame: 10, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TROO_RAISE4, misc1: 0, misc2: 0},	// S_TROO_RAISE3
	state_t { sprite: spritenum_t::SPR_TROO, frame: 9, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TROO_RAISE5, misc1: 0, misc2: 0},	// S_TROO_RAISE4
	state_t { sprite: spritenum_t::SPR_TROO, frame: 8, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TROO_RUN1, misc1: 0, misc2: 0},	// S_TROO_RAISE5
	state_t { sprite: spritenum_t::SPR_SARG, frame: 0, tics: 10, action: actionf_t{acp1: Some(A_Look)}, nextstate: statenum_t::S_SARG_STND2, misc1: 0, misc2: 0},	// S_SARG_STND
	state_t { sprite: spritenum_t::SPR_SARG, frame: 1, tics: 10, action: actionf_t{acp1: Some(A_Look)}, nextstate: statenum_t::S_SARG_STND, misc1: 0, misc2: 0},	// S_SARG_STND2
	state_t { sprite: spritenum_t::SPR_SARG, frame: 0, tics: 2, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SARG_RUN2, misc1: 0, misc2: 0},	// S_SARG_RUN1
	state_t { sprite: spritenum_t::SPR_SARG, frame: 0, tics: 2, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SARG_RUN3, misc1: 0, misc2: 0},	// S_SARG_RUN2
	state_t { sprite: spritenum_t::SPR_SARG, frame: 1, tics: 2, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SARG_RUN4, misc1: 0, misc2: 0},	// S_SARG_RUN3
	state_t { sprite: spritenum_t::SPR_SARG, frame: 1, tics: 2, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SARG_RUN5, misc1: 0, misc2: 0},	// S_SARG_RUN4
	state_t { sprite: spritenum_t::SPR_SARG, frame: 2, tics: 2, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SARG_RUN6, misc1: 0, misc2: 0},	// S_SARG_RUN5
	state_t { sprite: spritenum_t::SPR_SARG, frame: 2, tics: 2, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SARG_RUN7, misc1: 0, misc2: 0},	// S_SARG_RUN6
	state_t { sprite: spritenum_t::SPR_SARG, frame: 3, tics: 2, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SARG_RUN8, misc1: 0, misc2: 0},	// S_SARG_RUN7
	state_t { sprite: spritenum_t::SPR_SARG, frame: 3, tics: 2, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SARG_RUN1, misc1: 0, misc2: 0},	// S_SARG_RUN8
	state_t { sprite: spritenum_t::SPR_SARG, frame: 4, tics: 8, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_SARG_ATK2, misc1: 0, misc2: 0},	// S_SARG_ATK1
	state_t { sprite: spritenum_t::SPR_SARG, frame: 5, tics: 8, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_SARG_ATK3, misc1: 0, misc2: 0},	// S_SARG_ATK2
	state_t { sprite: spritenum_t::SPR_SARG, frame: 6, tics: 8, action: actionf_t{acp1: Some(A_SargAttack)}, nextstate: statenum_t::S_SARG_RUN1, misc1: 0, misc2: 0},	// S_SARG_ATK3
	state_t { sprite: spritenum_t::SPR_SARG, frame: 7, tics: 2, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SARG_PAIN2, misc1: 0, misc2: 0},	// S_SARG_PAIN
	state_t { sprite: spritenum_t::SPR_SARG, frame: 7, tics: 2, action: actionf_t{acp1: Some(A_Pain)}, nextstate: statenum_t::S_SARG_RUN1, misc1: 0, misc2: 0},	// S_SARG_PAIN2
	state_t { sprite: spritenum_t::SPR_SARG, frame: 8, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SARG_DIE2, misc1: 0, misc2: 0},	// S_SARG_DIE1
	state_t { sprite: spritenum_t::SPR_SARG, frame: 9, tics: 8, action: actionf_t{acp1: Some(A_Scream)}, nextstate: statenum_t::S_SARG_DIE3, misc1: 0, misc2: 0},	// S_SARG_DIE2
	state_t { sprite: spritenum_t::SPR_SARG, frame: 10, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SARG_DIE4, misc1: 0, misc2: 0},	// S_SARG_DIE3
	state_t { sprite: spritenum_t::SPR_SARG, frame: 11, tics: 4, action: actionf_t{acp1: Some(A_Fall)}, nextstate: statenum_t::S_SARG_DIE5, misc1: 0, misc2: 0},	// S_SARG_DIE4
	state_t { sprite: spritenum_t::SPR_SARG, frame: 12, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SARG_DIE6, misc1: 0, misc2: 0},	// S_SARG_DIE5
	state_t { sprite: spritenum_t::SPR_SARG, frame: 13, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_SARG_DIE6
	state_t { sprite: spritenum_t::SPR_SARG, frame: 13, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SARG_RAISE2, misc1: 0, misc2: 0},	// S_SARG_RAISE1
	state_t { sprite: spritenum_t::SPR_SARG, frame: 12, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SARG_RAISE3, misc1: 0, misc2: 0},	// S_SARG_RAISE2
	state_t { sprite: spritenum_t::SPR_SARG, frame: 11, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SARG_RAISE4, misc1: 0, misc2: 0},	// S_SARG_RAISE3
	state_t { sprite: spritenum_t::SPR_SARG, frame: 10, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SARG_RAISE5, misc1: 0, misc2: 0},	// S_SARG_RAISE4
	state_t { sprite: spritenum_t::SPR_SARG, frame: 9, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SARG_RAISE6, misc1: 0, misc2: 0},	// S_SARG_RAISE5
	state_t { sprite: spritenum_t::SPR_SARG, frame: 8, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SARG_RUN1, misc1: 0, misc2: 0},	// S_SARG_RAISE6
	state_t { sprite: spritenum_t::SPR_HEAD, frame: 0, tics: 10, action: actionf_t{acp1: Some(A_Look)}, nextstate: statenum_t::S_HEAD_STND, misc1: 0, misc2: 0},	// S_HEAD_STND
	state_t { sprite: spritenum_t::SPR_HEAD, frame: 0, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_HEAD_RUN1, misc1: 0, misc2: 0},	// S_HEAD_RUN1
	state_t { sprite: spritenum_t::SPR_HEAD, frame: 1, tics: 5, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_HEAD_ATK2, misc1: 0, misc2: 0},	// S_HEAD_ATK1
	state_t { sprite: spritenum_t::SPR_HEAD, frame: 2, tics: 5, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_HEAD_ATK3, misc1: 0, misc2: 0},	// S_HEAD_ATK2
	state_t { sprite: spritenum_t::SPR_HEAD, frame: 32771, tics: 5, action: actionf_t{acp1: Some(A_HeadAttack)}, nextstate: statenum_t::S_HEAD_RUN1, misc1: 0, misc2: 0},	// S_HEAD_ATK3
	state_t { sprite: spritenum_t::SPR_HEAD, frame: 4, tics: 3, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_HEAD_PAIN2, misc1: 0, misc2: 0},	// S_HEAD_PAIN
	state_t { sprite: spritenum_t::SPR_HEAD, frame: 4, tics: 3, action: actionf_t{acp1: Some(A_Pain)}, nextstate: statenum_t::S_HEAD_PAIN3, misc1: 0, misc2: 0},	// S_HEAD_PAIN2
	state_t { sprite: spritenum_t::SPR_HEAD, frame: 5, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_HEAD_RUN1, misc1: 0, misc2: 0},	// S_HEAD_PAIN3
	state_t { sprite: spritenum_t::SPR_HEAD, frame: 6, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_HEAD_DIE2, misc1: 0, misc2: 0},	// S_HEAD_DIE1
	state_t { sprite: spritenum_t::SPR_HEAD, frame: 7, tics: 8, action: actionf_t{acp1: Some(A_Scream)}, nextstate: statenum_t::S_HEAD_DIE3, misc1: 0, misc2: 0},	// S_HEAD_DIE2
	state_t { sprite: spritenum_t::SPR_HEAD, frame: 8, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_HEAD_DIE4, misc1: 0, misc2: 0},	// S_HEAD_DIE3
	state_t { sprite: spritenum_t::SPR_HEAD, frame: 9, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_HEAD_DIE5, misc1: 0, misc2: 0},	// S_HEAD_DIE4
	state_t { sprite: spritenum_t::SPR_HEAD, frame: 10, tics: 8, action: actionf_t{acp1: Some(A_Fall)}, nextstate: statenum_t::S_HEAD_DIE6, misc1: 0, misc2: 0},	// S_HEAD_DIE5
	state_t { sprite: spritenum_t::SPR_HEAD, frame: 11, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_HEAD_DIE6
	state_t { sprite: spritenum_t::SPR_HEAD, frame: 11, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_HEAD_RAISE2, misc1: 0, misc2: 0},	// S_HEAD_RAISE1
	state_t { sprite: spritenum_t::SPR_HEAD, frame: 10, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_HEAD_RAISE3, misc1: 0, misc2: 0},	// S_HEAD_RAISE2
	state_t { sprite: spritenum_t::SPR_HEAD, frame: 9, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_HEAD_RAISE4, misc1: 0, misc2: 0},	// S_HEAD_RAISE3
	state_t { sprite: spritenum_t::SPR_HEAD, frame: 8, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_HEAD_RAISE5, misc1: 0, misc2: 0},	// S_HEAD_RAISE4
	state_t { sprite: spritenum_t::SPR_HEAD, frame: 7, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_HEAD_RAISE6, misc1: 0, misc2: 0},	// S_HEAD_RAISE5
	state_t { sprite: spritenum_t::SPR_HEAD, frame: 6, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_HEAD_RUN1, misc1: 0, misc2: 0},	// S_HEAD_RAISE6
	state_t { sprite: spritenum_t::SPR_BAL7, frame: 32768, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BRBALL2, misc1: 0, misc2: 0},	// S_BRBALL1
	state_t { sprite: spritenum_t::SPR_BAL7, frame: 32769, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BRBALL1, misc1: 0, misc2: 0},	// S_BRBALL2
	state_t { sprite: spritenum_t::SPR_BAL7, frame: 32770, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BRBALLX2, misc1: 0, misc2: 0},	// S_BRBALLX1
	state_t { sprite: spritenum_t::SPR_BAL7, frame: 32771, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BRBALLX3, misc1: 0, misc2: 0},	// S_BRBALLX2
	state_t { sprite: spritenum_t::SPR_BAL7, frame: 32772, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_BRBALLX3
	state_t { sprite: spritenum_t::SPR_BOSS, frame: 0, tics: 10, action: actionf_t{acp1: Some(A_Look)}, nextstate: statenum_t::S_BOSS_STND2, misc1: 0, misc2: 0},	// S_BOSS_STND
	state_t { sprite: spritenum_t::SPR_BOSS, frame: 1, tics: 10, action: actionf_t{acp1: Some(A_Look)}, nextstate: statenum_t::S_BOSS_STND, misc1: 0, misc2: 0},	// S_BOSS_STND2
	state_t { sprite: spritenum_t::SPR_BOSS, frame: 0, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_BOSS_RUN2, misc1: 0, misc2: 0},	// S_BOSS_RUN1
	state_t { sprite: spritenum_t::SPR_BOSS, frame: 0, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_BOSS_RUN3, misc1: 0, misc2: 0},	// S_BOSS_RUN2
	state_t { sprite: spritenum_t::SPR_BOSS, frame: 1, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_BOSS_RUN4, misc1: 0, misc2: 0},	// S_BOSS_RUN3
	state_t { sprite: spritenum_t::SPR_BOSS, frame: 1, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_BOSS_RUN5, misc1: 0, misc2: 0},	// S_BOSS_RUN4
	state_t { sprite: spritenum_t::SPR_BOSS, frame: 2, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_BOSS_RUN6, misc1: 0, misc2: 0},	// S_BOSS_RUN5
	state_t { sprite: spritenum_t::SPR_BOSS, frame: 2, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_BOSS_RUN7, misc1: 0, misc2: 0},	// S_BOSS_RUN6
	state_t { sprite: spritenum_t::SPR_BOSS, frame: 3, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_BOSS_RUN8, misc1: 0, misc2: 0},	// S_BOSS_RUN7
	state_t { sprite: spritenum_t::SPR_BOSS, frame: 3, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_BOSS_RUN1, misc1: 0, misc2: 0},	// S_BOSS_RUN8
	state_t { sprite: spritenum_t::SPR_BOSS, frame: 4, tics: 8, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_BOSS_ATK2, misc1: 0, misc2: 0},	// S_BOSS_ATK1
	state_t { sprite: spritenum_t::SPR_BOSS, frame: 5, tics: 8, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_BOSS_ATK3, misc1: 0, misc2: 0},	// S_BOSS_ATK2
	state_t { sprite: spritenum_t::SPR_BOSS, frame: 6, tics: 8, action: actionf_t{acp1: Some(A_BruisAttack)}, nextstate: statenum_t::S_BOSS_RUN1, misc1: 0, misc2: 0},	// S_BOSS_ATK3
	state_t { sprite: spritenum_t::SPR_BOSS, frame: 7, tics: 2, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BOSS_PAIN2, misc1: 0, misc2: 0},	// S_BOSS_PAIN
	state_t { sprite: spritenum_t::SPR_BOSS, frame: 7, tics: 2, action: actionf_t{acp1: Some(A_Pain)}, nextstate: statenum_t::S_BOSS_RUN1, misc1: 0, misc2: 0},	// S_BOSS_PAIN2
	state_t { sprite: spritenum_t::SPR_BOSS, frame: 8, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BOSS_DIE2, misc1: 0, misc2: 0},	// S_BOSS_DIE1
	state_t { sprite: spritenum_t::SPR_BOSS, frame: 9, tics: 8, action: actionf_t{acp1: Some(A_Scream)}, nextstate: statenum_t::S_BOSS_DIE3, misc1: 0, misc2: 0},	// S_BOSS_DIE2
	state_t { sprite: spritenum_t::SPR_BOSS, frame: 10, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BOSS_DIE4, misc1: 0, misc2: 0},	// S_BOSS_DIE3
	state_t { sprite: spritenum_t::SPR_BOSS, frame: 11, tics: 8, action: actionf_t{acp1: Some(A_Fall)}, nextstate: statenum_t::S_BOSS_DIE5, misc1: 0, misc2: 0},	// S_BOSS_DIE4
	state_t { sprite: spritenum_t::SPR_BOSS, frame: 12, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BOSS_DIE6, misc1: 0, misc2: 0},	// S_BOSS_DIE5
	state_t { sprite: spritenum_t::SPR_BOSS, frame: 13, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BOSS_DIE7, misc1: 0, misc2: 0},	// S_BOSS_DIE6
	state_t { sprite: spritenum_t::SPR_BOSS, frame: 14, tics: -1, action: actionf_t{acp1: Some(A_BossDeath)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_BOSS_DIE7
	state_t { sprite: spritenum_t::SPR_BOSS, frame: 14, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BOSS_RAISE2, misc1: 0, misc2: 0},	// S_BOSS_RAISE1
	state_t { sprite: spritenum_t::SPR_BOSS, frame: 13, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BOSS_RAISE3, misc1: 0, misc2: 0},	// S_BOSS_RAISE2
	state_t { sprite: spritenum_t::SPR_BOSS, frame: 12, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BOSS_RAISE4, misc1: 0, misc2: 0},	// S_BOSS_RAISE3
	state_t { sprite: spritenum_t::SPR_BOSS, frame: 11, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BOSS_RAISE5, misc1: 0, misc2: 0},	// S_BOSS_RAISE4
	state_t { sprite: spritenum_t::SPR_BOSS, frame: 10, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BOSS_RAISE6, misc1: 0, misc2: 0},	// S_BOSS_RAISE5
	state_t { sprite: spritenum_t::SPR_BOSS, frame: 9, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BOSS_RAISE7, misc1: 0, misc2: 0},	// S_BOSS_RAISE6
	state_t { sprite: spritenum_t::SPR_BOSS, frame: 8, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BOSS_RUN1, misc1: 0, misc2: 0},	// S_BOSS_RAISE7
	state_t { sprite: spritenum_t::SPR_BOS2, frame: 0, tics: 10, action: actionf_t{acp1: Some(A_Look)}, nextstate: statenum_t::S_BOS2_STND2, misc1: 0, misc2: 0},	// S_BOS2_STND
	state_t { sprite: spritenum_t::SPR_BOS2, frame: 1, tics: 10, action: actionf_t{acp1: Some(A_Look)}, nextstate: statenum_t::S_BOS2_STND, misc1: 0, misc2: 0},	// S_BOS2_STND2
state_t { sprite: spritenum_t::SPR_BOS2, frame: 0, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_BOS2_RUN2, misc1: 0, misc2: 0},	// S_BOS2_RUN1
	state_t { sprite: spritenum_t::SPR_BOS2, frame: 0, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_BOS2_RUN3, misc1: 0, misc2: 0},	// S_BOS2_RUN2
	state_t { sprite: spritenum_t::SPR_BOS2, frame: 1, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_BOS2_RUN4, misc1: 0, misc2: 0},	// S_BOS2_RUN3
	state_t { sprite: spritenum_t::SPR_BOS2, frame: 1, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_BOS2_RUN5, misc1: 0, misc2: 0},	// S_BOS2_RUN4
	state_t { sprite: spritenum_t::SPR_BOS2, frame: 2, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_BOS2_RUN6, misc1: 0, misc2: 0},	// S_BOS2_RUN5
	state_t { sprite: spritenum_t::SPR_BOS2, frame: 2, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_BOS2_RUN7, misc1: 0, misc2: 0},	// S_BOS2_RUN6
	state_t { sprite: spritenum_t::SPR_BOS2, frame: 3, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_BOS2_RUN8, misc1: 0, misc2: 0},	// S_BOS2_RUN7
	state_t { sprite: spritenum_t::SPR_BOS2, frame: 3, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_BOS2_RUN1, misc1: 0, misc2: 0},	// S_BOS2_RUN8
	state_t { sprite: spritenum_t::SPR_BOS2, frame: 4, tics: 8, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_BOS2_ATK2, misc1: 0, misc2: 0},	// S_BOS2_ATK1
	state_t { sprite: spritenum_t::SPR_BOS2, frame: 5, tics: 8, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_BOS2_ATK3, misc1: 0, misc2: 0},	// S_BOS2_ATK2
	state_t { sprite: spritenum_t::SPR_BOS2, frame: 6, tics: 8, action: actionf_t{acp1: Some(A_BruisAttack)}, nextstate: statenum_t::S_BOS2_RUN1, misc1: 0, misc2: 0},	// S_BOS2_ATK3
	state_t { sprite: spritenum_t::SPR_BOS2, frame: 7, tics: 2, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BOS2_PAIN2, misc1: 0, misc2: 0},	// S_BOS2_PAIN
	state_t { sprite: spritenum_t::SPR_BOS2, frame: 7, tics: 2, action: actionf_t{acp1: Some(A_Pain)}, nextstate: statenum_t::S_BOS2_RUN1, misc1: 0, misc2: 0},	// S_BOS2_PAIN2
	state_t { sprite: spritenum_t::SPR_BOS2, frame: 8, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BOS2_DIE2, misc1: 0, misc2: 0},	// S_BOS2_DIE1
	state_t { sprite: spritenum_t::SPR_BOS2, frame: 9, tics: 8, action: actionf_t{acp1: Some(A_Scream)}, nextstate: statenum_t::S_BOS2_DIE3, misc1: 0, misc2: 0},	// S_BOS2_DIE2
	state_t { sprite: spritenum_t::SPR_BOS2, frame: 10, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BOS2_DIE4, misc1: 0, misc2: 0},	// S_BOS2_DIE3
	state_t { sprite: spritenum_t::SPR_BOS2, frame: 11, tics: 8, action: actionf_t{acp1: Some(A_Fall)}, nextstate: statenum_t::S_BOS2_DIE5, misc1: 0, misc2: 0},	// S_BOS2_DIE4
	state_t { sprite: spritenum_t::SPR_BOS2, frame: 12, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BOS2_DIE6, misc1: 0, misc2: 0},	// S_BOS2_DIE5
	state_t { sprite: spritenum_t::SPR_BOS2, frame: 13, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BOS2_DIE7, misc1: 0, misc2: 0},	// S_BOS2_DIE6
	state_t { sprite: spritenum_t::SPR_BOS2, frame: 14, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_BOS2_DIE7
	state_t { sprite: spritenum_t::SPR_BOS2, frame: 14, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BOS2_RAISE2, misc1: 0, misc2: 0},	// S_BOS2_RAISE1
	state_t { sprite: spritenum_t::SPR_BOS2, frame: 13, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BOS2_RAISE3, misc1: 0, misc2: 0},	// S_BOS2_RAISE2
	state_t { sprite: spritenum_t::SPR_BOS2, frame: 12, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BOS2_RAISE4, misc1: 0, misc2: 0},	// S_BOS2_RAISE3
	state_t { sprite: spritenum_t::SPR_BOS2, frame: 11, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BOS2_RAISE5, misc1: 0, misc2: 0},	// S_BOS2_RAISE4
	state_t { sprite: spritenum_t::SPR_BOS2, frame: 10, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BOS2_RAISE6, misc1: 0, misc2: 0},	// S_BOS2_RAISE5
	state_t { sprite: spritenum_t::SPR_BOS2, frame: 9, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BOS2_RAISE7, misc1: 0, misc2: 0},	// S_BOS2_RAISE6
	state_t { sprite: spritenum_t::SPR_BOS2, frame: 8, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BOS2_RUN1, misc1: 0, misc2: 0},	// S_BOS2_RAISE7
	state_t { sprite: spritenum_t::SPR_SKUL, frame: 32768, tics: 10, action: actionf_t{acp1: Some(A_Look)}, nextstate: statenum_t::S_SKULL_STND2, misc1: 0, misc2: 0},	// S_SKULL_STND
	state_t { sprite: spritenum_t::SPR_SKUL, frame: 32769, tics: 10, action: actionf_t{acp1: Some(A_Look)}, nextstate: statenum_t::S_SKULL_STND, misc1: 0, misc2: 0},	// S_SKULL_STND2
	state_t { sprite: spritenum_t::SPR_SKUL, frame: 32768, tics: 6, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SKULL_RUN2, misc1: 0, misc2: 0},	// S_SKULL_RUN1
	state_t { sprite: spritenum_t::SPR_SKUL, frame: 32769, tics: 6, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SKULL_RUN1, misc1: 0, misc2: 0},	// S_SKULL_RUN2
	state_t { sprite: spritenum_t::SPR_SKUL, frame: 32770, tics: 10, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_SKULL_ATK2, misc1: 0, misc2: 0},	// S_SKULL_ATK1
	state_t { sprite: spritenum_t::SPR_SKUL, frame: 32771, tics: 4, action: actionf_t{acp1: Some(A_SkullAttack)}, nextstate: statenum_t::S_SKULL_ATK3, misc1: 0, misc2: 0},	// S_SKULL_ATK2
	state_t { sprite: spritenum_t::SPR_SKUL, frame: 32770, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SKULL_ATK4, misc1: 0, misc2: 0},	// S_SKULL_ATK3
	state_t { sprite: spritenum_t::SPR_SKUL, frame: 32771, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SKULL_ATK3, misc1: 0, misc2: 0},	// S_SKULL_ATK4
	state_t { sprite: spritenum_t::SPR_SKUL, frame: 32772, tics: 3, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SKULL_PAIN2, misc1: 0, misc2: 0},	// S_SKULL_PAIN
	state_t { sprite: spritenum_t::SPR_SKUL, frame: 32772, tics: 3, action: actionf_t{acp1: Some(A_Pain)}, nextstate: statenum_t::S_SKULL_RUN1, misc1: 0, misc2: 0},	// S_SKULL_PAIN2
	state_t { sprite: spritenum_t::SPR_SKUL, frame: 32773, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SKULL_DIE2, misc1: 0, misc2: 0},	// S_SKULL_DIE1
	state_t { sprite: spritenum_t::SPR_SKUL, frame: 32774, tics: 6, action: actionf_t{acp1: Some(A_Scream)}, nextstate: statenum_t::S_SKULL_DIE3, misc1: 0, misc2: 0},	// S_SKULL_DIE2
	state_t { sprite: spritenum_t::SPR_SKUL, frame: 32775, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SKULL_DIE4, misc1: 0, misc2: 0},	// S_SKULL_DIE3
	state_t { sprite: spritenum_t::SPR_SKUL, frame: 32776, tics: 6, action: actionf_t{acp1: Some(A_Fall)}, nextstate: statenum_t::S_SKULL_DIE5, misc1: 0, misc2: 0},	// S_SKULL_DIE4
	state_t { sprite: spritenum_t::SPR_SKUL, frame: 9, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SKULL_DIE6, misc1: 0, misc2: 0},	// S_SKULL_DIE5
	state_t { sprite: spritenum_t::SPR_SKUL, frame: 10, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_SKULL_DIE6
	state_t { sprite: spritenum_t::SPR_SPID, frame: 0, tics: 10, action: actionf_t{acp1: Some(A_Look)}, nextstate: statenum_t::S_SPID_STND2, misc1: 0, misc2: 0},	// S_SPID_STND
	state_t { sprite: spritenum_t::SPR_SPID, frame: 1, tics: 10, action: actionf_t{acp1: Some(A_Look)}, nextstate: statenum_t::S_SPID_STND, misc1: 0, misc2: 0},	// S_SPID_STND2
	state_t { sprite: spritenum_t::SPR_SPID, frame: 0, tics: 3, action: actionf_t{acp1: Some(A_Metal)}, nextstate: statenum_t::S_SPID_RUN2, misc1: 0, misc2: 0},	// S_SPID_RUN1
	state_t { sprite: spritenum_t::SPR_SPID, frame: 0, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SPID_RUN3, misc1: 0, misc2: 0},	// S_SPID_RUN2
	state_t { sprite: spritenum_t::SPR_SPID, frame: 1, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SPID_RUN4, misc1: 0, misc2: 0},	// S_SPID_RUN3
	state_t { sprite: spritenum_t::SPR_SPID, frame: 1, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SPID_RUN5, misc1: 0, misc2: 0},	// S_SPID_RUN4
	state_t { sprite: spritenum_t::SPR_SPID, frame: 2, tics: 3, action: actionf_t{acp1: Some(A_Metal)}, nextstate: statenum_t::S_SPID_RUN6, misc1: 0, misc2: 0},	// S_SPID_RUN5
	state_t { sprite: spritenum_t::SPR_SPID, frame: 2, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SPID_RUN7, misc1: 0, misc2: 0},	// S_SPID_RUN6
	state_t { sprite: spritenum_t::SPR_SPID, frame: 3, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SPID_RUN8, misc1: 0, misc2: 0},	// S_SPID_RUN7
	state_t { sprite: spritenum_t::SPR_SPID, frame: 3, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SPID_RUN9, misc1: 0, misc2: 0},	// S_SPID_RUN8
	state_t { sprite: spritenum_t::SPR_SPID, frame: 4, tics: 3, action: actionf_t{acp1: Some(A_Metal)}, nextstate: statenum_t::S_SPID_RUN10, misc1: 0, misc2: 0},	// S_SPID_RUN9
	state_t { sprite: spritenum_t::SPR_SPID, frame: 4, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SPID_RUN11, misc1: 0, misc2: 0},	// S_SPID_RUN10
	state_t { sprite: spritenum_t::SPR_SPID, frame: 5, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SPID_RUN12, misc1: 0, misc2: 0},	// S_SPID_RUN11
	state_t { sprite: spritenum_t::SPR_SPID, frame: 5, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SPID_RUN1, misc1: 0, misc2: 0},	// S_SPID_RUN12
	state_t { sprite: spritenum_t::SPR_SPID, frame: 32768, tics: 20, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_SPID_ATK2, misc1: 0, misc2: 0},	// S_SPID_ATK1
	state_t { sprite: spritenum_t::SPR_SPID, frame: 32774, tics: 4, action: actionf_t{acp1: Some(A_SPosAttack)}, nextstate: statenum_t::S_SPID_ATK3, misc1: 0, misc2: 0},	// S_SPID_ATK2
	state_t { sprite: spritenum_t::SPR_SPID, frame: 32775, tics: 4, action: actionf_t{acp1: Some(A_SPosAttack)}, nextstate: statenum_t::S_SPID_ATK4, misc1: 0, misc2: 0},	// S_SPID_ATK3
	state_t { sprite: spritenum_t::SPR_SPID, frame: 32775, tics: 1, action: actionf_t{acp1: Some(A_SpidRefire)}, nextstate: statenum_t::S_SPID_ATK2, misc1: 0, misc2: 0},	// S_SPID_ATK4
	state_t { sprite: spritenum_t::SPR_SPID, frame: 8, tics: 3, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SPID_PAIN2, misc1: 0, misc2: 0},	// S_SPID_PAIN
	state_t { sprite: spritenum_t::SPR_SPID, frame: 8, tics: 3, action: actionf_t{acp1: Some(A_Pain)}, nextstate: statenum_t::S_SPID_RUN1, misc1: 0, misc2: 0},	// S_SPID_PAIN2
	state_t { sprite: spritenum_t::SPR_SPID, frame: 9, tics: 20, action: actionf_t{acp1: Some(A_Scream)}, nextstate: statenum_t::S_SPID_DIE2, misc1: 0, misc2: 0},	// S_SPID_DIE1
	state_t { sprite: spritenum_t::SPR_SPID, frame: 10, tics: 10, action: actionf_t{acp1: Some(A_Fall)}, nextstate: statenum_t::S_SPID_DIE3, misc1: 0, misc2: 0},	// S_SPID_DIE2
	state_t { sprite: spritenum_t::SPR_SPID, frame: 11, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SPID_DIE4, misc1: 0, misc2: 0},	// S_SPID_DIE3
	state_t { sprite: spritenum_t::SPR_SPID, frame: 12, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SPID_DIE5, misc1: 0, misc2: 0},	// S_SPID_DIE4
	state_t { sprite: spritenum_t::SPR_SPID, frame: 13, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SPID_DIE6, misc1: 0, misc2: 0},	// S_SPID_DIE5
	state_t { sprite: spritenum_t::SPR_SPID, frame: 14, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SPID_DIE7, misc1: 0, misc2: 0},	// S_SPID_DIE6
	state_t { sprite: spritenum_t::SPR_SPID, frame: 15, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SPID_DIE8, misc1: 0, misc2: 0},	// S_SPID_DIE7
	state_t { sprite: spritenum_t::SPR_SPID, frame: 16, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SPID_DIE9, misc1: 0, misc2: 0},	// S_SPID_DIE8
	state_t { sprite: spritenum_t::SPR_SPID, frame: 17, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SPID_DIE10, misc1: 0, misc2: 0},	// S_SPID_DIE9
	state_t { sprite: spritenum_t::SPR_SPID, frame: 18, tics: 30, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SPID_DIE11, misc1: 0, misc2: 0},	// S_SPID_DIE10
	state_t { sprite: spritenum_t::SPR_SPID, frame: 18, tics: -1, action: actionf_t{acp1: Some(A_BossDeath)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_SPID_DIE11
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 0, tics: 10, action: actionf_t{acp1: Some(A_Look)}, nextstate: statenum_t::S_BSPI_STND2, misc1: 0, misc2: 0},	// S_BSPI_STND
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 1, tics: 10, action: actionf_t{acp1: Some(A_Look)}, nextstate: statenum_t::S_BSPI_STND, misc1: 0, misc2: 0},	// S_BSPI_STND2
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 0, tics: 20, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BSPI_RUN1, misc1: 0, misc2: 0},	// S_BSPI_SIGHT
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 0, tics: 3, action: actionf_t{acp1: Some(A_BabyMetal)}, nextstate: statenum_t::S_BSPI_RUN2, misc1: 0, misc2: 0},	// S_BSPI_RUN1
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 0, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_BSPI_RUN3, misc1: 0, misc2: 0},	// S_BSPI_RUN2
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 1, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_BSPI_RUN4, misc1: 0, misc2: 0},	// S_BSPI_RUN3
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 1, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_BSPI_RUN5, misc1: 0, misc2: 0},	// S_BSPI_RUN4
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 2, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_BSPI_RUN6, misc1: 0, misc2: 0},	// S_BSPI_RUN5
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 2, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_BSPI_RUN7, misc1: 0, misc2: 0},	// S_BSPI_RUN6
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 3, tics: 3, action: actionf_t{acp1: Some(A_BabyMetal)}, nextstate: statenum_t::S_BSPI_RUN8, misc1: 0, misc2: 0},	// S_BSPI_RUN7
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 3, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_BSPI_RUN9, misc1: 0, misc2: 0},	// S_BSPI_RUN8
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 4, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_BSPI_RUN10, misc1: 0, misc2: 0},	// S_BSPI_RUN9
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 4, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_BSPI_RUN11, misc1: 0, misc2: 0},	// S_BSPI_RUN10
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 5, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_BSPI_RUN12, misc1: 0, misc2: 0},	// S_BSPI_RUN11
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 5, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_BSPI_RUN1, misc1: 0, misc2: 0},	// S_BSPI_RUN12
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 32768, tics: 20, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_BSPI_ATK2, misc1: 0, misc2: 0},	// S_BSPI_ATK1
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 32774, tics: 4, action: actionf_t{acp1: Some(A_BspiAttack)}, nextstate: statenum_t::S_BSPI_ATK3, misc1: 0, misc2: 0},	// S_BSPI_ATK2
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 32775, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BSPI_ATK4, misc1: 0, misc2: 0},	// S_BSPI_ATK3
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 32775, tics: 1, action: actionf_t{acp1: Some(A_SpidRefire)}, nextstate: statenum_t::S_BSPI_ATK2, misc1: 0, misc2: 0},	// S_BSPI_ATK4
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 8, tics: 3, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BSPI_PAIN2, misc1: 0, misc2: 0},	// S_BSPI_PAIN
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 8, tics: 3, action: actionf_t{acp1: Some(A_Pain)}, nextstate: statenum_t::S_BSPI_RUN1, misc1: 0, misc2: 0},	// S_BSPI_PAIN2
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 9, tics: 20, action: actionf_t{acp1: Some(A_Scream)}, nextstate: statenum_t::S_BSPI_DIE2, misc1: 0, misc2: 0},	// S_BSPI_DIE1
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 10, tics: 7, action: actionf_t{acp1: Some(A_Fall)}, nextstate: statenum_t::S_BSPI_DIE3, misc1: 0, misc2: 0},	// S_BSPI_DIE2
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 11, tics: 7, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BSPI_DIE4, misc1: 0, misc2: 0},	// S_BSPI_DIE3
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 12, tics: 7, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BSPI_DIE5, misc1: 0, misc2: 0},	// S_BSPI_DIE4
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 13, tics: 7, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BSPI_DIE6, misc1: 0, misc2: 0},	// S_BSPI_DIE5
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 14, tics: 7, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BSPI_DIE7, misc1: 0, misc2: 0},	// S_BSPI_DIE6
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 15, tics: -1, action: actionf_t{acp1: Some(A_BossDeath)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_BSPI_DIE7
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 15, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BSPI_RAISE2, misc1: 0, misc2: 0},	// S_BSPI_RAISE1
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 14, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BSPI_RAISE3, misc1: 0, misc2: 0},	// S_BSPI_RAISE2
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 13, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BSPI_RAISE4, misc1: 0, misc2: 0},	// S_BSPI_RAISE3
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 12, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BSPI_RAISE5, misc1: 0, misc2: 0},	// S_BSPI_RAISE4
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 11, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BSPI_RAISE6, misc1: 0, misc2: 0},	// S_BSPI_RAISE5
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 10, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BSPI_RAISE7, misc1: 0, misc2: 0},	// S_BSPI_RAISE6
	state_t { sprite: spritenum_t::SPR_BSPI, frame: 9, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BSPI_RUN1, misc1: 0, misc2: 0},	// S_BSPI_RAISE7
	state_t { sprite: spritenum_t::SPR_APLS, frame: 32768, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_ARACH_PLAZ2, misc1: 0, misc2: 0},	// S_ARACH_PLAZ
	state_t { sprite: spritenum_t::SPR_APLS, frame: 32769, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_ARACH_PLAZ, misc1: 0, misc2: 0},	// S_ARACH_PLAZ2
	state_t { sprite: spritenum_t::SPR_APBX, frame: 32768, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_ARACH_PLEX2, misc1: 0, misc2: 0},	// S_ARACH_PLEX
	state_t { sprite: spritenum_t::SPR_APBX, frame: 32769, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_ARACH_PLEX3, misc1: 0, misc2: 0},	// S_ARACH_PLEX2
	state_t { sprite: spritenum_t::SPR_APBX, frame: 32770, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_ARACH_PLEX4, misc1: 0, misc2: 0},	// S_ARACH_PLEX3
	state_t { sprite: spritenum_t::SPR_APBX, frame: 32771, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_ARACH_PLEX5, misc1: 0, misc2: 0},	// S_ARACH_PLEX4
	state_t { sprite: spritenum_t::SPR_APBX, frame: 32772, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_ARACH_PLEX5
	state_t { sprite: spritenum_t::SPR_CYBR, frame: 0, tics: 10, action: actionf_t{acp1: Some(A_Look)}, nextstate: statenum_t::S_CYBER_STND2, misc1: 0, misc2: 0},	// S_CYBER_STND
	state_t { sprite: spritenum_t::SPR_CYBR, frame: 1, tics: 10, action: actionf_t{acp1: Some(A_Look)}, nextstate: statenum_t::S_CYBER_STND, misc1: 0, misc2: 0},	// S_CYBER_STND2
	state_t { sprite: spritenum_t::SPR_CYBR, frame: 0, tics: 3, action: actionf_t{acp1: Some(A_Hoof)}, nextstate: statenum_t::S_CYBER_RUN2, misc1: 0, misc2: 0},	// S_CYBER_RUN1
	state_t { sprite: spritenum_t::SPR_CYBR, frame: 0, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_CYBER_RUN3, misc1: 0, misc2: 0},	// S_CYBER_RUN2
	state_t { sprite: spritenum_t::SPR_CYBR, frame: 1, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_CYBER_RUN4, misc1: 0, misc2: 0},	// S_CYBER_RUN3
	state_t { sprite: spritenum_t::SPR_CYBR, frame: 1, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_CYBER_RUN5, misc1: 0, misc2: 0},	// S_CYBER_RUN4
	state_t { sprite: spritenum_t::SPR_CYBR, frame: 2, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_CYBER_RUN6, misc1: 0, misc2: 0},	// S_CYBER_RUN5
	state_t { sprite: spritenum_t::SPR_CYBR, frame: 2, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_CYBER_RUN7, misc1: 0, misc2: 0},	// S_CYBER_RUN6
	state_t { sprite: spritenum_t::SPR_CYBR, frame: 3, tics: 3, action: actionf_t{acp1: Some(A_Metal)}, nextstate: statenum_t::S_CYBER_RUN8, misc1: 0, misc2: 0},	// S_CYBER_RUN7
	state_t { sprite: spritenum_t::SPR_CYBR, frame: 3, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_CYBER_RUN1, misc1: 0, misc2: 0},	// S_CYBER_RUN8
	state_t { sprite: spritenum_t::SPR_CYBR, frame: 4, tics: 6, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_CYBER_ATK2, misc1: 0, misc2: 0},	// S_CYBER_ATK1
	state_t { sprite: spritenum_t::SPR_CYBR, frame: 5, tics: 12, action: actionf_t{acp1: Some(A_CyberAttack)}, nextstate: statenum_t::S_CYBER_ATK3, misc1: 0, misc2: 0},	// S_CYBER_ATK2
	state_t { sprite: spritenum_t::SPR_CYBR, frame: 4, tics: 12, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_CYBER_ATK4, misc1: 0, misc2: 0},	// S_CYBER_ATK3
	state_t { sprite: spritenum_t::SPR_CYBR, frame: 5, tics: 12, action: actionf_t{acp1: Some(A_CyberAttack)}, nextstate: statenum_t::S_CYBER_ATK5, misc1: 0, misc2: 0},	// S_CYBER_ATK4
	state_t { sprite: spritenum_t::SPR_CYBR, frame: 4, tics: 12, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_CYBER_ATK6, misc1: 0, misc2: 0},	// S_CYBER_ATK5
	state_t { sprite: spritenum_t::SPR_CYBR, frame: 5, tics: 12, action: actionf_t{acp1: Some(A_CyberAttack)}, nextstate: statenum_t::S_CYBER_RUN1, misc1: 0, misc2: 0},	// S_CYBER_ATK6
	state_t { sprite: spritenum_t::SPR_CYBR, frame: 6, tics: 10, action: actionf_t{acp1: Some(A_Pain)}, nextstate: statenum_t::S_CYBER_RUN1, misc1: 0, misc2: 0},	// S_CYBER_PAIN
	state_t { sprite: spritenum_t::SPR_CYBR, frame: 7, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_CYBER_DIE2, misc1: 0, misc2: 0},	// S_CYBER_DIE1
	state_t { sprite: spritenum_t::SPR_CYBR, frame: 8, tics: 10, action: actionf_t{acp1: Some(A_Scream)}, nextstate: statenum_t::S_CYBER_DIE3, misc1: 0, misc2: 0},	// S_CYBER_DIE2
	state_t { sprite: spritenum_t::SPR_CYBR, frame: 9, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_CYBER_DIE4, misc1: 0, misc2: 0},	// S_CYBER_DIE3
	state_t { sprite: spritenum_t::SPR_CYBR, frame: 10, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_CYBER_DIE5, misc1: 0, misc2: 0},	// S_CYBER_DIE4
	state_t { sprite: spritenum_t::SPR_CYBR, frame: 11, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_CYBER_DIE6, misc1: 0, misc2: 0},	// S_CYBER_DIE5
	state_t { sprite: spritenum_t::SPR_CYBR, frame: 12, tics: 10, action: actionf_t{acp1: Some(A_Fall)}, nextstate: statenum_t::S_CYBER_DIE7, misc1: 0, misc2: 0},	// S_CYBER_DIE6
	state_t { sprite: spritenum_t::SPR_CYBR, frame: 13, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_CYBER_DIE8, misc1: 0, misc2: 0},	// S_CYBER_DIE7
	state_t { sprite: spritenum_t::SPR_CYBR, frame: 14, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_CYBER_DIE9, misc1: 0, misc2: 0},	// S_CYBER_DIE8
	state_t { sprite: spritenum_t::SPR_CYBR, frame: 15, tics: 30, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_CYBER_DIE10, misc1: 0, misc2: 0},	// S_CYBER_DIE9
	state_t { sprite: spritenum_t::SPR_CYBR, frame: 15, tics: -1, action: actionf_t{acp1: Some(A_BossDeath)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_CYBER_DIE10
	state_t { sprite: spritenum_t::SPR_PAIN, frame: 0, tics: 10, action: actionf_t{acp1: Some(A_Look)}, nextstate: statenum_t::S_PAIN_STND, misc1: 0, misc2: 0},	// S_PAIN_STND
	state_t { sprite: spritenum_t::SPR_PAIN, frame: 0, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_PAIN_RUN2, misc1: 0, misc2: 0},	// S_PAIN_RUN1
	state_t { sprite: spritenum_t::SPR_PAIN, frame: 0, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_PAIN_RUN3, misc1: 0, misc2: 0},	// S_PAIN_RUN2
	state_t { sprite: spritenum_t::SPR_PAIN, frame: 1, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_PAIN_RUN4, misc1: 0, misc2: 0},	// S_PAIN_RUN3
	state_t { sprite: spritenum_t::SPR_PAIN, frame: 1, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_PAIN_RUN5, misc1: 0, misc2: 0},	// S_PAIN_RUN4
	state_t { sprite: spritenum_t::SPR_PAIN, frame: 2, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_PAIN_RUN6, misc1: 0, misc2: 0},	// S_PAIN_RUN5
	state_t { sprite: spritenum_t::SPR_PAIN, frame: 2, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_PAIN_RUN1, misc1: 0, misc2: 0},	// S_PAIN_RUN6
	state_t { sprite: spritenum_t::SPR_PAIN, frame: 3, tics: 5, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_PAIN_ATK2, misc1: 0, misc2: 0},	// S_PAIN_ATK1
	state_t { sprite: spritenum_t::SPR_PAIN, frame: 4, tics: 5, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_PAIN_ATK3, misc1: 0, misc2: 0},	// S_PAIN_ATK2
	state_t { sprite: spritenum_t::SPR_PAIN, frame: 32773, tics: 5, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_PAIN_ATK4, misc1: 0, misc2: 0},	// S_PAIN_ATK3
	state_t { sprite: spritenum_t::SPR_PAIN, frame: 32773, tics: 0, action: actionf_t{acp1: Some(A_PainAttack)}, nextstate: statenum_t::S_PAIN_RUN1, misc1: 0, misc2: 0},	// S_PAIN_ATK4
	state_t { sprite: spritenum_t::SPR_PAIN, frame: 6, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PAIN_PAIN2, misc1: 0, misc2: 0},	// S_PAIN_PAIN
	state_t { sprite: spritenum_t::SPR_PAIN, frame: 6, tics: 6, action: actionf_t{acp1: Some(A_Pain)}, nextstate: statenum_t::S_PAIN_RUN1, misc1: 0, misc2: 0},	// S_PAIN_PAIN2
	state_t { sprite: spritenum_t::SPR_PAIN, frame: 32775, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PAIN_DIE2, misc1: 0, misc2: 0},	// S_PAIN_DIE1
	state_t { sprite: spritenum_t::SPR_PAIN, frame: 32776, tics: 8, action: actionf_t{acp1: Some(A_Scream)}, nextstate: statenum_t::S_PAIN_DIE3, misc1: 0, misc2: 0},	// S_PAIN_DIE2
	state_t { sprite: spritenum_t::SPR_PAIN, frame: 32777, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PAIN_DIE4, misc1: 0, misc2: 0},	// S_PAIN_DIE3
	state_t { sprite: spritenum_t::SPR_PAIN, frame: 32778, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PAIN_DIE5, misc1: 0, misc2: 0},	// S_PAIN_DIE4
	state_t { sprite: spritenum_t::SPR_PAIN, frame: 32779, tics: 8, action: actionf_t{acp1: Some(A_PainDie)}, nextstate: statenum_t::S_PAIN_DIE6, misc1: 0, misc2: 0},	// S_PAIN_DIE5
	state_t { sprite: spritenum_t::SPR_PAIN, frame: 32780, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_PAIN_DIE6
	state_t { sprite: spritenum_t::SPR_PAIN, frame: 12, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PAIN_RAISE2, misc1: 0, misc2: 0},	// S_PAIN_RAISE1
	state_t { sprite: spritenum_t::SPR_PAIN, frame: 11, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PAIN_RAISE3, misc1: 0, misc2: 0},	// S_PAIN_RAISE2
	state_t { sprite: spritenum_t::SPR_PAIN, frame: 10, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PAIN_RAISE4, misc1: 0, misc2: 0},	// S_PAIN_RAISE3
	state_t { sprite: spritenum_t::SPR_PAIN, frame: 9, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PAIN_RAISE5, misc1: 0, misc2: 0},	// S_PAIN_RAISE4
	state_t { sprite: spritenum_t::SPR_PAIN, frame: 8, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PAIN_RAISE6, misc1: 0, misc2: 0},	// S_PAIN_RAISE5
	state_t { sprite: spritenum_t::SPR_PAIN, frame: 7, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PAIN_RUN1, misc1: 0, misc2: 0},	// S_PAIN_RAISE6
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 0, tics: 10, action: actionf_t{acp1: Some(A_Look)}, nextstate: statenum_t::S_SSWV_STND2, misc1: 0, misc2: 0},	// S_SSWV_STND
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 1, tics: 10, action: actionf_t{acp1: Some(A_Look)}, nextstate: statenum_t::S_SSWV_STND, misc1: 0, misc2: 0},	// S_SSWV_STND2
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 0, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SSWV_RUN2, misc1: 0, misc2: 0},	// S_SSWV_RUN1
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 0, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SSWV_RUN3, misc1: 0, misc2: 0},	// S_SSWV_RUN2
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 1, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SSWV_RUN4, misc1: 0, misc2: 0},	// S_SSWV_RUN3
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 1, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SSWV_RUN5, misc1: 0, misc2: 0},	// S_SSWV_RUN4
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 2, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SSWV_RUN6, misc1: 0, misc2: 0},	// S_SSWV_RUN5
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 2, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SSWV_RUN7, misc1: 0, misc2: 0},	// S_SSWV_RUN6
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 3, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SSWV_RUN8, misc1: 0, misc2: 0},	// S_SSWV_RUN7
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 3, tics: 3, action: actionf_t{acp1: Some(A_Chase)}, nextstate: statenum_t::S_SSWV_RUN1, misc1: 0, misc2: 0},	// S_SSWV_RUN8
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 4, tics: 10, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_SSWV_ATK2, misc1: 0, misc2: 0},	// S_SSWV_ATK1
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 5, tics: 10, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_SSWV_ATK3, misc1: 0, misc2: 0},	// S_SSWV_ATK2
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 32774, tics: 4, action: actionf_t{acp1: Some(A_CPosAttack)}, nextstate: statenum_t::S_SSWV_ATK4, misc1: 0, misc2: 0},	// S_SSWV_ATK3
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 5, tics: 6, action: actionf_t{acp1: Some(A_FaceTarget)}, nextstate: statenum_t::S_SSWV_ATK5, misc1: 0, misc2: 0},	// S_SSWV_ATK4
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 32774, tics: 4, action: actionf_t{acp1: Some(A_CPosAttack)}, nextstate: statenum_t::S_SSWV_ATK6, misc1: 0, misc2: 0},	// S_SSWV_ATK5
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 5, tics: 1, action: actionf_t{acp1: Some(A_CPosRefire)}, nextstate: statenum_t::S_SSWV_ATK2, misc1: 0, misc2: 0},	// S_SSWV_ATK6
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 7, tics: 3, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SSWV_PAIN2, misc1: 0, misc2: 0},	// S_SSWV_PAIN
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 7, tics: 3, action: actionf_t{acp1: Some(A_Pain)}, nextstate: statenum_t::S_SSWV_RUN1, misc1: 0, misc2: 0},	// S_SSWV_PAIN2
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 8, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SSWV_DIE2, misc1: 0, misc2: 0},	// S_SSWV_DIE1
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 9, tics: 5, action: actionf_t{acp1: Some(A_Scream)}, nextstate: statenum_t::S_SSWV_DIE3, misc1: 0, misc2: 0},	// S_SSWV_DIE2
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 10, tics: 5, action: actionf_t{acp1: Some(A_Fall)}, nextstate: statenum_t::S_SSWV_DIE4, misc1: 0, misc2: 0},	// S_SSWV_DIE3
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 11, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SSWV_DIE5, misc1: 0, misc2: 0},	// S_SSWV_DIE4
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 12, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_SSWV_DIE5
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 13, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SSWV_XDIE2, misc1: 0, misc2: 0},	// S_SSWV_XDIE1
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 14, tics: 5, action: actionf_t{acp1: Some(A_XScream)}, nextstate: statenum_t::S_SSWV_XDIE3, misc1: 0, misc2: 0},	// S_SSWV_XDIE2
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 15, tics: 5, action: actionf_t{acp1: Some(A_Fall)}, nextstate: statenum_t::S_SSWV_XDIE4, misc1: 0, misc2: 0},	// S_SSWV_XDIE3
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 16, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SSWV_XDIE5, misc1: 0, misc2: 0},	// S_SSWV_XDIE4
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 17, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SSWV_XDIE6, misc1: 0, misc2: 0},	// S_SSWV_XDIE5
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 18, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SSWV_XDIE7, misc1: 0, misc2: 0},	// S_SSWV_XDIE6
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 19, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SSWV_XDIE8, misc1: 0, misc2: 0},	// S_SSWV_XDIE7
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 20, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SSWV_XDIE9, misc1: 0, misc2: 0},	// S_SSWV_XDIE8
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 21, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_SSWV_XDIE9
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 12, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SSWV_RAISE2, misc1: 0, misc2: 0},	// S_SSWV_RAISE1
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 11, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SSWV_RAISE3, misc1: 0, misc2: 0},	// S_SSWV_RAISE2
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 10, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SSWV_RAISE4, misc1: 0, misc2: 0},	// S_SSWV_RAISE3
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 9, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SSWV_RAISE5, misc1: 0, misc2: 0},	// S_SSWV_RAISE4
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 8, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SSWV_RUN1, misc1: 0, misc2: 0},	// S_SSWV_RAISE5
	state_t { sprite: spritenum_t::SPR_KEEN, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_KEENSTND, misc1: 0, misc2: 0},	// S_KEENSTND
	state_t { sprite: spritenum_t::SPR_KEEN, frame: 0, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_COMMKEEN2, misc1: 0, misc2: 0},	// S_COMMKEEN
	state_t { sprite: spritenum_t::SPR_KEEN, frame: 1, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_COMMKEEN3, misc1: 0, misc2: 0},	// S_COMMKEEN2
	state_t { sprite: spritenum_t::SPR_KEEN, frame: 2, tics: 6, action: actionf_t{acp1: Some(A_Scream)}, nextstate: statenum_t::S_COMMKEEN4, misc1: 0, misc2: 0},	// S_COMMKEEN3
	state_t { sprite: spritenum_t::SPR_KEEN, frame: 3, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_COMMKEEN5, misc1: 0, misc2: 0},	// S_COMMKEEN4
	state_t { sprite: spritenum_t::SPR_KEEN, frame: 4, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_COMMKEEN6, misc1: 0, misc2: 0},	// S_COMMKEEN5
	state_t { sprite: spritenum_t::SPR_KEEN, frame: 5, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_COMMKEEN7, misc1: 0, misc2: 0},	// S_COMMKEEN6
	state_t { sprite: spritenum_t::SPR_KEEN, frame: 6, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_COMMKEEN8, misc1: 0, misc2: 0},	// S_COMMKEEN7
	state_t { sprite: spritenum_t::SPR_KEEN, frame: 7, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_COMMKEEN9, misc1: 0, misc2: 0},	// S_COMMKEEN8
	state_t { sprite: spritenum_t::SPR_KEEN, frame: 8, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_COMMKEEN10, misc1: 0, misc2: 0},	// S_COMMKEEN9
	state_t { sprite: spritenum_t::SPR_KEEN, frame: 9, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_COMMKEEN11, misc1: 0, misc2: 0},	// S_COMMKEEN10
	state_t { sprite: spritenum_t::SPR_KEEN, frame: 10, tics: 6, action: actionf_t{acp1: Some(A_KeenDie)}, nextstate: statenum_t::S_COMMKEEN12, misc1: 0, misc2: 0},// S_COMMKEEN11
	state_t { sprite: spritenum_t::SPR_KEEN, frame: 11, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},		// S_COMMKEEN12
	state_t { sprite: spritenum_t::SPR_KEEN, frame: 12, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_KEENPAIN2, misc1: 0, misc2: 0},	// S_KEENPAIN
	state_t { sprite: spritenum_t::SPR_KEEN, frame: 12, tics: 8, action: actionf_t{acp1: Some(A_Pain)}, nextstate: statenum_t::S_KEENSTND, misc1: 0, misc2: 0},	// S_KEENPAIN2
	state_t { sprite: spritenum_t::SPR_BBRN, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},		// S_BRAIN
	state_t { sprite: spritenum_t::SPR_BBRN, frame: 1, tics: 36, action: actionf_t{acp1: Some(A_BrainPain)}, nextstate: statenum_t::S_BRAIN, misc1: 0, misc2: 0},	// S_BRAIN_PAIN
	state_t { sprite: spritenum_t::SPR_BBRN, frame: 0, tics: 100, action: actionf_t{acp1: Some(A_BrainScream)}, nextstate: statenum_t::S_BRAIN_DIE2, misc1: 0, misc2: 0},	// S_BRAIN_DIE1
	state_t { sprite: spritenum_t::SPR_BBRN, frame: 0, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BRAIN_DIE3, misc1: 0, misc2: 0},	// S_BRAIN_DIE2
	state_t { sprite: spritenum_t::SPR_BBRN, frame: 0, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BRAIN_DIE4, misc1: 0, misc2: 0},	// S_BRAIN_DIE3
	state_t { sprite: spritenum_t::SPR_BBRN, frame: 0, tics: -1, action: actionf_t{acp1: Some(A_BrainDie)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_BRAIN_DIE4
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 0, tics: 10, action: actionf_t{acp1: Some(A_Look)}, nextstate: statenum_t::S_BRAINEYE, misc1: 0, misc2: 0},	// S_BRAINEYE
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 0, tics: 181, action: actionf_t{acp1: Some(A_BrainAwake)}, nextstate: statenum_t::S_BRAINEYE1, misc1: 0, misc2: 0},	// S_BRAINEYESEE
	state_t { sprite: spritenum_t::SPR_SSWV, frame: 0, tics: 150, action: actionf_t{acp1: Some(A_BrainSpit)}, nextstate: statenum_t::S_BRAINEYE1, misc1: 0, misc2: 0},	// S_BRAINEYE1
	state_t { sprite: spritenum_t::SPR_BOSF, frame: 32768, tics: 3, action: actionf_t{acp1: Some(A_SpawnSound)}, nextstate: statenum_t::S_SPAWN2, misc1: 0, misc2: 0},	// S_SPAWN1
	state_t { sprite: spritenum_t::SPR_BOSF, frame: 32769, tics: 3, action: actionf_t{acp1: Some(A_SpawnFly)}, nextstate: statenum_t::S_SPAWN3, misc1: 0, misc2: 0},	// S_SPAWN2
	state_t { sprite: spritenum_t::SPR_BOSF, frame: 32770, tics: 3, action: actionf_t{acp1: Some(A_SpawnFly)}, nextstate: statenum_t::S_SPAWN4, misc1: 0, misc2: 0},	// S_SPAWN3
	state_t { sprite: spritenum_t::SPR_BOSF, frame: 32771, tics: 3, action: actionf_t{acp1: Some(A_SpawnFly)}, nextstate: statenum_t::S_SPAWN1, misc1: 0, misc2: 0},	// S_SPAWN4
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32768, tics: 4, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_SPAWNFIRE2, misc1: 0, misc2: 0},	// S_SPAWNFIRE1
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32769, tics: 4, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_SPAWNFIRE3, misc1: 0, misc2: 0},	// S_SPAWNFIRE2
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32770, tics: 4, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_SPAWNFIRE4, misc1: 0, misc2: 0},	// S_SPAWNFIRE3
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32771, tics: 4, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_SPAWNFIRE5, misc1: 0, misc2: 0},	// S_SPAWNFIRE4
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32772, tics: 4, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_SPAWNFIRE6, misc1: 0, misc2: 0},	// S_SPAWNFIRE5
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32773, tics: 4, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_SPAWNFIRE7, misc1: 0, misc2: 0},	// S_SPAWNFIRE6
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32774, tics: 4, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_SPAWNFIRE8, misc1: 0, misc2: 0},	// S_SPAWNFIRE7
	state_t { sprite: spritenum_t::SPR_FIRE, frame: 32775, tics: 4, action: actionf_t{acp1: Some(A_Fire)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},		// S_SPAWNFIRE8
	state_t { sprite: spritenum_t::SPR_MISL, frame: 32769, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BRAINEXPLODE2, misc1: 0, misc2: 0},	// S_BRAINEXPLODE1
	state_t { sprite: spritenum_t::SPR_MISL, frame: 32770, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BRAINEXPLODE3, misc1: 0, misc2: 0},	// S_BRAINEXPLODE2
	state_t { sprite: spritenum_t::SPR_MISL, frame: 32771, tics: 10, action: actionf_t{acp1: Some(A_BrainExplode)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_BRAINEXPLODE3
	state_t { sprite: spritenum_t::SPR_ARM1, frame: 0, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_ARM1A, misc1: 0, misc2: 0},	// S_ARM1
	state_t { sprite: spritenum_t::SPR_ARM1, frame: 32769, tics: 7, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_ARM1, misc1: 0, misc2: 0},	// S_ARM1A
	state_t { sprite: spritenum_t::SPR_ARM2, frame: 0, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_ARM2A, misc1: 0, misc2: 0},	// S_ARM2
	state_t { sprite: spritenum_t::SPR_ARM2, frame: 32769, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_ARM2, misc1: 0, misc2: 0},	// S_ARM2A
	state_t { sprite: spritenum_t::SPR_BAR1, frame: 0, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BAR2, misc1: 0, misc2: 0},	// S_BAR1
	state_t { sprite: spritenum_t::SPR_BAR1, frame: 1, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BAR1, misc1: 0, misc2: 0},	// S_BAR2
	state_t { sprite: spritenum_t::SPR_BEXP, frame: 32768, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BEXP2, misc1: 0, misc2: 0},	// S_BEXP
	state_t { sprite: spritenum_t::SPR_BEXP, frame: 32769, tics: 5, action: actionf_t{acp1: Some(A_Scream)}, nextstate: statenum_t::S_BEXP3, misc1: 0, misc2: 0},	// S_BEXP2
	state_t { sprite: spritenum_t::SPR_BEXP, frame: 32770, tics: 5, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BEXP4, misc1: 0, misc2: 0},	// S_BEXP3
	state_t { sprite: spritenum_t::SPR_BEXP, frame: 32771, tics: 10, action: actionf_t{acp1: Some(A_Explode)}, nextstate: statenum_t::S_BEXP5, misc1: 0, misc2: 0},	// S_BEXP4
	state_t { sprite: spritenum_t::SPR_BEXP, frame: 32772, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_BEXP5
	state_t { sprite: spritenum_t::SPR_FCAN, frame: 32768, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BBAR2, misc1: 0, misc2: 0},	// S_BBAR1
	state_t { sprite: spritenum_t::SPR_FCAN, frame: 32769, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BBAR3, misc1: 0, misc2: 0},	// S_BBAR2
	state_t { sprite: spritenum_t::SPR_FCAN, frame: 32770, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BBAR1, misc1: 0, misc2: 0},	// S_BBAR3
	state_t { sprite: spritenum_t::SPR_BON1, frame: 0, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BON1A, misc1: 0, misc2: 0},	// S_BON1
	state_t { sprite: spritenum_t::SPR_BON1, frame: 1, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BON1B, misc1: 0, misc2: 0},	// S_BON1A
	state_t { sprite: spritenum_t::SPR_BON1, frame: 2, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BON1C, misc1: 0, misc2: 0},	// S_BON1B
	state_t { sprite: spritenum_t::SPR_BON1, frame: 3, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BON1D, misc1: 0, misc2: 0},	// S_BON1C
	state_t { sprite: spritenum_t::SPR_BON1, frame: 2, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BON1E, misc1: 0, misc2: 0},	// S_BON1D
	state_t { sprite: spritenum_t::SPR_BON1, frame: 1, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BON1, misc1: 0, misc2: 0},	// S_BON1E
	state_t { sprite: spritenum_t::SPR_BON2, frame: 0, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BON2A, misc1: 0, misc2: 0},	// S_BON2
	state_t { sprite: spritenum_t::SPR_BON2, frame: 1, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BON2B, misc1: 0, misc2: 0},	// S_BON2A
	state_t { sprite: spritenum_t::SPR_BON2, frame: 2, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BON2C, misc1: 0, misc2: 0},	// S_BON2B
	state_t { sprite: spritenum_t::SPR_BON2, frame: 3, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BON2D, misc1: 0, misc2: 0},	// S_BON2C
	state_t { sprite: spritenum_t::SPR_BON2, frame: 2, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BON2E, misc1: 0, misc2: 0},	// S_BON2D
	state_t { sprite: spritenum_t::SPR_BON2, frame: 1, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BON2, misc1: 0, misc2: 0},	// S_BON2E
	state_t { sprite: spritenum_t::SPR_BKEY, frame: 0, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BKEY2, misc1: 0, misc2: 0},	// S_BKEY
	state_t { sprite: spritenum_t::SPR_BKEY, frame: 32769, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BKEY, misc1: 0, misc2: 0},	// S_BKEY2
	state_t { sprite: spritenum_t::SPR_RKEY, frame: 0, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_RKEY2, misc1: 0, misc2: 0},	// S_RKEY
	state_t { sprite: spritenum_t::SPR_RKEY, frame: 32769, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_RKEY, misc1: 0, misc2: 0},	// S_RKEY2
	state_t { sprite: spritenum_t::SPR_YKEY, frame: 0, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_YKEY2, misc1: 0, misc2: 0},	// S_YKEY
	state_t { sprite: spritenum_t::SPR_YKEY, frame: 32769, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_YKEY, misc1: 0, misc2: 0},	// S_YKEY2
	state_t { sprite: spritenum_t::SPR_BSKU, frame: 0, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BSKULL2, misc1: 0, misc2: 0},	// S_BSKULL
	state_t { sprite: spritenum_t::SPR_BSKU, frame: 32769, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BSKULL, misc1: 0, misc2: 0},	// S_BSKULL2
	state_t { sprite: spritenum_t::SPR_RSKU, frame: 0, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_RSKULL2, misc1: 0, misc2: 0},	// S_RSKULL
	state_t { sprite: spritenum_t::SPR_RSKU, frame: 32769, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_RSKULL, misc1: 0, misc2: 0},	// S_RSKULL2
	state_t { sprite: spritenum_t::SPR_YSKU, frame: 0, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_YSKULL2, misc1: 0, misc2: 0},	// S_YSKULL
	state_t { sprite: spritenum_t::SPR_YSKU, frame: 32769, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_YSKULL, misc1: 0, misc2: 0},	// S_YSKULL2
	state_t { sprite: spritenum_t::SPR_STIM, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_STIM
	state_t { sprite: spritenum_t::SPR_MEDI, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_MEDI
	state_t { sprite: spritenum_t::SPR_SOUL, frame: 32768, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SOUL2, misc1: 0, misc2: 0},	// S_SOUL
	state_t { sprite: spritenum_t::SPR_SOUL, frame: 32769, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SOUL3, misc1: 0, misc2: 0},	// S_SOUL2
	state_t { sprite: spritenum_t::SPR_SOUL, frame: 32770, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SOUL4, misc1: 0, misc2: 0},	// S_SOUL3
	state_t { sprite: spritenum_t::SPR_SOUL, frame: 32771, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SOUL5, misc1: 0, misc2: 0},	// S_SOUL4
	state_t { sprite: spritenum_t::SPR_SOUL, frame: 32770, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SOUL6, misc1: 0, misc2: 0},	// S_SOUL5
	state_t { sprite: spritenum_t::SPR_SOUL, frame: 32769, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_SOUL, misc1: 0, misc2: 0},	// S_SOUL6
	state_t { sprite: spritenum_t::SPR_PINV, frame: 32768, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PINV2, misc1: 0, misc2: 0},	// S_PINV
	state_t { sprite: spritenum_t::SPR_PINV, frame: 32769, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PINV3, misc1: 0, misc2: 0},	// S_PINV2
	state_t { sprite: spritenum_t::SPR_PINV, frame: 32770, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PINV4, misc1: 0, misc2: 0},	// S_PINV3
	state_t { sprite: spritenum_t::SPR_PINV, frame: 32771, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PINV, misc1: 0, misc2: 0},	// S_PINV4
	state_t { sprite: spritenum_t::SPR_PSTR, frame: 32768, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_PSTR
	state_t { sprite: spritenum_t::SPR_PINS, frame: 32768, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PINS2, misc1: 0, misc2: 0},	// S_PINS
	state_t { sprite: spritenum_t::SPR_PINS, frame: 32769, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PINS3, misc1: 0, misc2: 0},	// S_PINS2
	state_t { sprite: spritenum_t::SPR_PINS, frame: 32770, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PINS4, misc1: 0, misc2: 0},	// S_PINS3
	state_t { sprite: spritenum_t::SPR_PINS, frame: 32771, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PINS, misc1: 0, misc2: 0},	// S_PINS4
	state_t { sprite: spritenum_t::SPR_MEGA, frame: 32768, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_MEGA2, misc1: 0, misc2: 0},	// S_MEGA
	state_t { sprite: spritenum_t::SPR_MEGA, frame: 32769, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_MEGA3, misc1: 0, misc2: 0},	// S_MEGA2
	state_t { sprite: spritenum_t::SPR_MEGA, frame: 32770, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_MEGA4, misc1: 0, misc2: 0},	// S_MEGA3
	state_t { sprite: spritenum_t::SPR_MEGA, frame: 32771, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_MEGA, misc1: 0, misc2: 0},	// S_MEGA4
	state_t { sprite: spritenum_t::SPR_SUIT, frame: 32768, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_SUIT
	state_t { sprite: spritenum_t::SPR_PMAP, frame: 32768, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PMAP2, misc1: 0, misc2: 0},	// S_PMAP
	state_t { sprite: spritenum_t::SPR_PMAP, frame: 32769, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PMAP3, misc1: 0, misc2: 0},	// S_PMAP2
	state_t { sprite: spritenum_t::SPR_PMAP, frame: 32770, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PMAP4, misc1: 0, misc2: 0},	// S_PMAP3
	state_t { sprite: spritenum_t::SPR_PMAP, frame: 32771, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PMAP5, misc1: 0, misc2: 0},	// S_PMAP4
	state_t { sprite: spritenum_t::SPR_PMAP, frame: 32770, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PMAP6, misc1: 0, misc2: 0},	// S_PMAP5
	state_t { sprite: spritenum_t::SPR_PMAP, frame: 32769, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PMAP, misc1: 0, misc2: 0},	// S_PMAP6
	state_t { sprite: spritenum_t::SPR_PVIS, frame: 32768, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PVIS2, misc1: 0, misc2: 0},	// S_PVIS
	state_t { sprite: spritenum_t::SPR_PVIS, frame: 1, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_PVIS, misc1: 0, misc2: 0},	// S_PVIS2
	state_t { sprite: spritenum_t::SPR_CLIP, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_CLIP
	state_t { sprite: spritenum_t::SPR_AMMO, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_AMMO
	state_t { sprite: spritenum_t::SPR_ROCK, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_ROCK
	state_t { sprite: spritenum_t::SPR_BROK, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_BROK
	state_t { sprite: spritenum_t::SPR_CELL, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_CELL
	state_t { sprite: spritenum_t::SPR_CELP, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_CELP
	state_t { sprite: spritenum_t::SPR_SHEL, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_SHEL
	state_t { sprite: spritenum_t::SPR_SBOX, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_SBOX
	state_t { sprite: spritenum_t::SPR_BPAK, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_BPAK
	state_t { sprite: spritenum_t::SPR_BFUG, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_BFUG
	state_t { sprite: spritenum_t::SPR_MGUN, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_MGUN
	state_t { sprite: spritenum_t::SPR_CSAW, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_CSAW
	state_t { sprite: spritenum_t::SPR_LAUN, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_LAUN
	state_t { sprite: spritenum_t::SPR_PLAS, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_PLAS
	state_t { sprite: spritenum_t::SPR_SHOT, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_SHOT
	state_t { sprite: spritenum_t::SPR_SGN2, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_SHOT2
	state_t { sprite: spritenum_t::SPR_COLU, frame: 32768, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_COLU
	state_t { sprite: spritenum_t::SPR_SMT2, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_STALAG
	state_t { sprite: spritenum_t::SPR_GOR1, frame: 0, tics: 10, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BLOODYTWITCH2, misc1: 0, misc2: 0},	// S_BLOODYTWITCH
	state_t { sprite: spritenum_t::SPR_GOR1, frame: 1, tics: 15, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BLOODYTWITCH3, misc1: 0, misc2: 0},	// S_BLOODYTWITCH2
	state_t { sprite: spritenum_t::SPR_GOR1, frame: 2, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BLOODYTWITCH4, misc1: 0, misc2: 0},	// S_BLOODYTWITCH3
	state_t { sprite: spritenum_t::SPR_GOR1, frame: 1, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BLOODYTWITCH, misc1: 0, misc2: 0},	// S_BLOODYTWITCH4
	state_t { sprite: spritenum_t::SPR_PLAY, frame: 13, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_DEADTORSO
	state_t { sprite: spritenum_t::SPR_PLAY, frame: 18, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_DEADBOTTOM
	state_t { sprite: spritenum_t::SPR_POL2, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_HEADSONSTICK
	state_t { sprite: spritenum_t::SPR_POL5, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_GIBS
	state_t { sprite: spritenum_t::SPR_POL4, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_HEADONASTICK
	state_t { sprite: spritenum_t::SPR_POL3, frame: 32768, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_HEADCANDLES2, misc1: 0, misc2: 0},	// S_HEADCANDLES
	state_t { sprite: spritenum_t::SPR_POL3, frame: 32769, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_HEADCANDLES, misc1: 0, misc2: 0},	// S_HEADCANDLES2
	state_t { sprite: spritenum_t::SPR_POL1, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_DEADSTICK
	state_t { sprite: spritenum_t::SPR_POL6, frame: 0, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_LIVESTICK2, misc1: 0, misc2: 0},	// S_LIVESTICK
	state_t { sprite: spritenum_t::SPR_POL6, frame: 1, tics: 8, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_LIVESTICK, misc1: 0, misc2: 0},	// S_LIVESTICK2
	state_t { sprite: spritenum_t::SPR_GOR2, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_MEAT2
	state_t { sprite: spritenum_t::SPR_GOR3, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_MEAT3
	state_t { sprite: spritenum_t::SPR_GOR4, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_MEAT4
	state_t { sprite: spritenum_t::SPR_GOR5, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_MEAT5
	state_t { sprite: spritenum_t::SPR_SMIT, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_STALAGTITE
	state_t { sprite: spritenum_t::SPR_COL1, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_TALLGRNCOL
	state_t { sprite: spritenum_t::SPR_COL2, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_SHRTGRNCOL
	state_t { sprite: spritenum_t::SPR_COL3, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_TALLREDCOL
	state_t { sprite: spritenum_t::SPR_COL4, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_SHRTREDCOL
	state_t { sprite: spritenum_t::SPR_CAND, frame: 32768, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_CANDLESTIK
	state_t { sprite: spritenum_t::SPR_CBRA, frame: 32768, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_CANDELABRA
	state_t { sprite: spritenum_t::SPR_COL6, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_SKULLCOL
	state_t { sprite: spritenum_t::SPR_TRE1, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_TORCHTREE
	state_t { sprite: spritenum_t::SPR_TRE2, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_BIGTREE
	state_t { sprite: spritenum_t::SPR_ELEC, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_TECHPILLAR
	state_t { sprite: spritenum_t::SPR_CEYE, frame: 32768, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_EVILEYE2, misc1: 0, misc2: 0},	// S_EVILEYE
	state_t { sprite: spritenum_t::SPR_CEYE, frame: 32769, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_EVILEYE3, misc1: 0, misc2: 0},	// S_EVILEYE2
	state_t { sprite: spritenum_t::SPR_CEYE, frame: 32770, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_EVILEYE4, misc1: 0, misc2: 0},	// S_EVILEYE3
	state_t { sprite: spritenum_t::SPR_CEYE, frame: 32769, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_EVILEYE, misc1: 0, misc2: 0},	// S_EVILEYE4
	state_t { sprite: spritenum_t::SPR_FSKU, frame: 32768, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_FLOATSKULL2, misc1: 0, misc2: 0},	// S_FLOATSKULL
	state_t { sprite: spritenum_t::SPR_FSKU, frame: 32769, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_FLOATSKULL3, misc1: 0, misc2: 0},	// S_FLOATSKULL2
	state_t { sprite: spritenum_t::SPR_FSKU, frame: 32770, tics: 6, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_FLOATSKULL, misc1: 0, misc2: 0},	// S_FLOATSKULL3
	state_t { sprite: spritenum_t::SPR_COL5, frame: 0, tics: 14, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_HEARTCOL2, misc1: 0, misc2: 0},	// S_HEARTCOL
	state_t { sprite: spritenum_t::SPR_COL5, frame: 1, tics: 14, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_HEARTCOL, misc1: 0, misc2: 0},	// S_HEARTCOL2
	state_t { sprite: spritenum_t::SPR_TBLU, frame: 32768, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BLUETORCH2, misc1: 0, misc2: 0},	// S_BLUETORCH
	state_t { sprite: spritenum_t::SPR_TBLU, frame: 32769, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BLUETORCH3, misc1: 0, misc2: 0},	// S_BLUETORCH2
	state_t { sprite: spritenum_t::SPR_TBLU, frame: 32770, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BLUETORCH4, misc1: 0, misc2: 0},	// S_BLUETORCH3
	state_t { sprite: spritenum_t::SPR_TBLU, frame: 32771, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BLUETORCH, misc1: 0, misc2: 0},	// S_BLUETORCH4
	state_t { sprite: spritenum_t::SPR_TGRN, frame: 32768, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_GREENTORCH2, misc1: 0, misc2: 0},	// S_GREENTORCH
	state_t { sprite: spritenum_t::SPR_TGRN, frame: 32769, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_GREENTORCH3, misc1: 0, misc2: 0},	// S_GREENTORCH2
	state_t { sprite: spritenum_t::SPR_TGRN, frame: 32770, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_GREENTORCH4, misc1: 0, misc2: 0},	// S_GREENTORCH3
	state_t { sprite: spritenum_t::SPR_TGRN, frame: 32771, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_GREENTORCH, misc1: 0, misc2: 0},	// S_GREENTORCH4
	state_t { sprite: spritenum_t::SPR_TRED, frame: 32768, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_REDTORCH2, misc1: 0, misc2: 0},	// S_REDTORCH
	state_t { sprite: spritenum_t::SPR_TRED, frame: 32769, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_REDTORCH3, misc1: 0, misc2: 0},	// S_REDTORCH2
	state_t { sprite: spritenum_t::SPR_TRED, frame: 32770, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_REDTORCH4, misc1: 0, misc2: 0},	// S_REDTORCH3
	state_t { sprite: spritenum_t::SPR_TRED, frame: 32771, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_REDTORCH, misc1: 0, misc2: 0},	// S_REDTORCH4
	state_t { sprite: spritenum_t::SPR_SMBT, frame: 32768, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BTORCHSHRT2, misc1: 0, misc2: 0},	// S_BTORCHSHRT
	state_t { sprite: spritenum_t::SPR_SMBT, frame: 32769, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BTORCHSHRT3, misc1: 0, misc2: 0},	// S_BTORCHSHRT2
	state_t { sprite: spritenum_t::SPR_SMBT, frame: 32770, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BTORCHSHRT4, misc1: 0, misc2: 0},	// S_BTORCHSHRT3
	state_t { sprite: spritenum_t::SPR_SMBT, frame: 32771, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_BTORCHSHRT, misc1: 0, misc2: 0},	// S_BTORCHSHRT4
	state_t { sprite: spritenum_t::SPR_SMGT, frame: 32768, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_GTORCHSHRT2, misc1: 0, misc2: 0},	// S_GTORCHSHRT
	state_t { sprite: spritenum_t::SPR_SMGT, frame: 32769, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_GTORCHSHRT3, misc1: 0, misc2: 0},	// S_GTORCHSHRT2
	state_t { sprite: spritenum_t::SPR_SMGT, frame: 32770, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_GTORCHSHRT4, misc1: 0, misc2: 0},	// S_GTORCHSHRT3
	state_t { sprite: spritenum_t::SPR_SMGT, frame: 32771, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_GTORCHSHRT, misc1: 0, misc2: 0},	// S_GTORCHSHRT4
	state_t { sprite: spritenum_t::SPR_SMRT, frame: 32768, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_RTORCHSHRT2, misc1: 0, misc2: 0},	// S_RTORCHSHRT
	state_t { sprite: spritenum_t::SPR_SMRT, frame: 32769, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_RTORCHSHRT3, misc1: 0, misc2: 0},	// S_RTORCHSHRT2
	state_t { sprite: spritenum_t::SPR_SMRT, frame: 32770, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_RTORCHSHRT4, misc1: 0, misc2: 0},	// S_RTORCHSHRT3
	state_t { sprite: spritenum_t::SPR_SMRT, frame: 32771, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_RTORCHSHRT, misc1: 0, misc2: 0},	// S_RTORCHSHRT4
	state_t { sprite: spritenum_t::SPR_HDB1, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_HANGNOGUTS
	state_t { sprite: spritenum_t::SPR_HDB2, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_HANGBNOBRAIN
	state_t { sprite: spritenum_t::SPR_HDB3, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_HANGTLOOKDN
	state_t { sprite: spritenum_t::SPR_HDB4, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_HANGTSKULL
	state_t { sprite: spritenum_t::SPR_HDB5, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_HANGTLOOKUP
	state_t { sprite: spritenum_t::SPR_HDB6, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_HANGTNOBRAIN
	state_t { sprite: spritenum_t::SPR_POB1, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_COLONGIBS
	state_t { sprite: spritenum_t::SPR_POB2, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},	// S_SMALLPOOL
	state_t { sprite: spritenum_t::SPR_BRS1, frame: 0, tics: -1, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_NULL, misc1: 0, misc2: 0},		// S_BRAINSTEM
	state_t { sprite: spritenum_t::SPR_TLMP, frame: 32768, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TECHLAMP2, misc1: 0, misc2: 0},	// S_TECHLAMP
	state_t { sprite: spritenum_t::SPR_TLMP, frame: 32769, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TECHLAMP3, misc1: 0, misc2: 0},	// S_TECHLAMP2
	state_t { sprite: spritenum_t::SPR_TLMP, frame: 32770, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TECHLAMP4, misc1: 0, misc2: 0},	// S_TECHLAMP3
	state_t { sprite: spritenum_t::SPR_TLMP, frame: 32771, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TECHLAMP, misc1: 0, misc2: 0},	// S_TECHLAMP4
	state_t { sprite: spritenum_t::SPR_TLP2, frame: 32768, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TECH2LAMP2, misc1: 0, misc2: 0},	// S_TECH2LAMP
	state_t { sprite: spritenum_t::SPR_TLP2, frame: 32769, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TECH2LAMP3, misc1: 0, misc2: 0},	// S_TECH2LAMP2
	state_t { sprite: spritenum_t::SPR_TLP2, frame: 32770, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TECH2LAMP4, misc1: 0, misc2: 0},	// S_TECH2LAMP3
	state_t { sprite: spritenum_t::SPR_TLP2, frame: 32771, tics: 4, action: actionf_t{acp1: Some(NULL)}, nextstate: statenum_t::S_TECH2LAMP, misc1: 0, misc2: 0},	// S_TECH2LAMP4
];

#[unsafe(no_mangle)]
pub static mut mobjinfo: [mobjinfo_t; mobjtype_t::NUMMOBJTYPES as usize] = [
	mobjinfo_t {
		// MT_PLAYER
		doomednum: -1,
		spawnstate: statenum_t::S_PLAY,
		spawnhealth: 100,
		seestate: statenum_t::S_PLAY_RUN1,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 0,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_PLAY_PAIN,
		painchance: 255,
		painsound: sfxenum_t::sfx_plpain,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_PLAY_ATK1,
		deathstate: statenum_t::S_PLAY_DIE1,
		xdeathstate: statenum_t::S_PLAY_XDIE1,
		deathsound: sfxenum_t::sfx_pldeth,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 56 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID | MF_SHOOTABLE | MF_DROPOFF | MF_PICKUP | MF_NOTDMATCH,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_POSSESSED
		doomednum: 3004,
		spawnstate: statenum_t::S_POSS_STND,
		spawnhealth: 20,
		seestate: statenum_t::S_POSS_RUN1,
		seesound: sfxenum_t::sfx_posit1,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_pistol,
		painstate: statenum_t::S_POSS_PAIN,
		painchance: 200,
		painsound: sfxenum_t::sfx_popain,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_POSS_ATK1,
		deathstate: statenum_t::S_POSS_DIE1,
		xdeathstate: statenum_t::S_POSS_XDIE1,
		deathsound: sfxenum_t::sfx_podth1,
		speed: 8,
		radius: 20 * FRACUNIT,
		height: 56 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_posact,
		flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL,
		raisestate: statenum_t::S_POSS_RAISE1,
	},
	mobjinfo_t {
		// MT_SHOTGUY
		doomednum: 9,
		spawnstate: statenum_t::S_SPOS_STND,
		spawnhealth: 30,
		seestate: statenum_t::S_SPOS_RUN1,
		seesound: sfxenum_t::sfx_posit2,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_SPOS_PAIN,
		painchance: 170,
		painsound: sfxenum_t::sfx_popain,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_SPOS_ATK1,
		deathstate: statenum_t::S_SPOS_DIE1,
		xdeathstate: statenum_t::S_SPOS_XDIE1,
		deathsound: sfxenum_t::sfx_podth2,
		speed: 8,
		radius: 20 * FRACUNIT,
		height: 56 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_posact,
		flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL,
		raisestate: statenum_t::S_SPOS_RAISE1,
	},
	mobjinfo_t {
		// MT_VILE
		doomednum: 64,
		spawnstate: statenum_t::S_VILE_STND,
		spawnhealth: 700,
		seestate: statenum_t::S_VILE_RUN1,
		seesound: sfxenum_t::sfx_vilsit,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_VILE_PAIN,
		painchance: 10,
		painsound: sfxenum_t::sfx_vipain,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_VILE_ATK1,
		deathstate: statenum_t::S_VILE_DIE1,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_vildth,
		speed: 15,
		radius: 20 * FRACUNIT,
		height: 56 * FRACUNIT,
		mass: 500,
		damage: 0,
		activesound: sfxenum_t::sfx_vilact,
		flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_FIRE
		doomednum: -1,
		spawnstate: statenum_t::S_FIRE1,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_NOBLOCKMAP | MF_NOGRAVITY,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_UNDEAD
		doomednum: 66,
		spawnstate: statenum_t::S_SKEL_STND,
		spawnhealth: 300,
		seestate: statenum_t::S_SKEL_RUN1,
		seesound: sfxenum_t::sfx_skesit,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_SKEL_PAIN,
		painchance: 100,
		painsound: sfxenum_t::sfx_popain,
		meleestate: statenum_t::S_SKEL_FIST1,
		missilestate: statenum_t::S_SKEL_MISS1,
		deathstate: statenum_t::S_SKEL_DIE1,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_skedth,
		speed: 10,
		radius: 20 * FRACUNIT,
		height: 56 * FRACUNIT,
		mass: 500,
		damage: 0,
		activesound: sfxenum_t::sfx_skeact,
		flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL,
		raisestate: statenum_t::S_SKEL_RAISE1,
	},
	mobjinfo_t {
		// MT_TRACER
		doomednum: -1,
		spawnstate: statenum_t::S_TRACER,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_skeatk,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_TRACEEXP1,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_barexp,
		speed: 10 * FRACUNIT,
		radius: 11 * FRACUNIT,
		height: 8 * FRACUNIT,
		mass: 100,
		damage: 10,
		activesound: sfxenum_t::sfx_None,
		flags: MF_NOBLOCKMAP | MF_MISSILE | MF_DROPOFF | MF_NOGRAVITY,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_SMOKE
		doomednum: -1,
		spawnstate: statenum_t::S_SMOKE1,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_NOBLOCKMAP | MF_NOGRAVITY,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_FATSO
		doomednum: 67,
		spawnstate: statenum_t::S_FATT_STND,
		spawnhealth: 600,
		seestate: statenum_t::S_FATT_RUN1,
		seesound: sfxenum_t::sfx_mansit,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_FATT_PAIN,
		painchance: 80,
		painsound: sfxenum_t::sfx_mnpain,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_FATT_ATK1,
		deathstate: statenum_t::S_FATT_DIE1,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_mandth,
		speed: 8,
		radius: 48 * FRACUNIT,
		height: 64 * FRACUNIT,
		mass: 1000,
		damage: 0,
		activesound: sfxenum_t::sfx_posact,
		flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL,
		raisestate: statenum_t::S_FATT_RAISE1,
	},
	mobjinfo_t {
		// MT_FATSHOT
		doomednum: -1,
		spawnstate: statenum_t::S_FATSHOT1,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_firsht,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_FATSHOTX1,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_firxpl,
		speed: 20 * FRACUNIT,
		radius: 6 * FRACUNIT,
		height: 8 * FRACUNIT,
		mass: 100,
		damage: 8,
		activesound: sfxenum_t::sfx_None,
		flags: MF_NOBLOCKMAP | MF_MISSILE | MF_DROPOFF | MF_NOGRAVITY,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_CHAINGUY
		doomednum: 65,
		spawnstate: statenum_t::S_CPOS_STND,
		spawnhealth: 70,
		seestate: statenum_t::S_CPOS_RUN1,
		seesound: sfxenum_t::sfx_posit2,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_CPOS_PAIN,
		painchance: 170,
		painsound: sfxenum_t::sfx_popain,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_CPOS_ATK1,
		deathstate: statenum_t::S_CPOS_DIE1,
		xdeathstate: statenum_t::S_CPOS_XDIE1,
		deathsound: sfxenum_t::sfx_podth2,
		speed: 8,
		radius: 20 * FRACUNIT,
		height: 56 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_posact,
		flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL,
		raisestate: statenum_t::S_CPOS_RAISE1,
	},
	mobjinfo_t {
		// MT_TROOP
		doomednum: 3001,
		spawnstate: statenum_t::S_TROO_STND,
		spawnhealth: 60,
		seestate: statenum_t::S_TROO_RUN1,
		seesound: sfxenum_t::sfx_bgsit1,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_TROO_PAIN,
		painchance: 200,
		painsound: sfxenum_t::sfx_popain,
		meleestate: statenum_t::S_TROO_ATK1,
		missilestate: statenum_t::S_TROO_ATK1,
		deathstate: statenum_t::S_TROO_DIE1,
		xdeathstate: statenum_t::S_TROO_XDIE1,
		deathsound: sfxenum_t::sfx_bgdth1,
		speed: 8,
		radius: 20 * FRACUNIT,
		height: 56 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_bgact,
		flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL,
		raisestate: statenum_t::S_TROO_RAISE1,
	},
	mobjinfo_t {
		// MT_SERGEANT
		doomednum: 3002,
		spawnstate: statenum_t::S_SARG_STND,
		spawnhealth: 150,
		seestate: statenum_t::S_SARG_RUN1,
		seesound: sfxenum_t::sfx_sgtsit,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_sgtatk,
		painstate: statenum_t::S_SARG_PAIN,
		painchance: 180,
		painsound: sfxenum_t::sfx_dmpain,
		meleestate: statenum_t::S_SARG_ATK1,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_SARG_DIE1,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_sgtdth,
		speed: 10,
		radius: 30 * FRACUNIT,
		height: 56 * FRACUNIT,
		mass: 400,
		damage: 0,
		activesound: sfxenum_t::sfx_dmact,
		flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL,
		raisestate: statenum_t::S_SARG_RAISE1,
	},
	mobjinfo_t {
		// MT_SHADOWS
		doomednum: 58,
		spawnstate: statenum_t::S_SARG_STND,
		spawnhealth: 150,
		seestate: statenum_t::S_SARG_RUN1,
		seesound: sfxenum_t::sfx_sgtsit,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_sgtatk,
		painstate: statenum_t::S_SARG_PAIN,
		painchance: 180,
		painsound: sfxenum_t::sfx_dmpain,
		meleestate: statenum_t::S_SARG_ATK1,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_SARG_DIE1,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_sgtdth,
		speed: 10,
		radius: 30 * FRACUNIT,
		height: 56 * FRACUNIT,
		mass: 400,
		damage: 0,
		activesound: sfxenum_t::sfx_dmact,
		flags: MF_SOLID | MF_SHOOTABLE | MF_SHADOW | MF_COUNTKILL,
		raisestate: statenum_t::S_SARG_RAISE1,
	},
	mobjinfo_t {
		// MT_HEAD
		doomednum: 3005,
		spawnstate: statenum_t::S_HEAD_STND,
		spawnhealth: 400,
		seestate: statenum_t::S_HEAD_RUN1,
		seesound: sfxenum_t::sfx_cacsit,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_HEAD_PAIN,
		painchance: 128,
		painsound: sfxenum_t::sfx_dmpain,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_HEAD_ATK1,
		deathstate: statenum_t::S_HEAD_DIE1,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_cacdth,
		speed: 8,
		radius: 31 * FRACUNIT,
		height: 56 * FRACUNIT,
		mass: 400,
		damage: 0,
		activesound: sfxenum_t::sfx_dmact,
		flags: MF_SOLID | MF_SHOOTABLE | MF_FLOAT | MF_NOGRAVITY | MF_COUNTKILL,
		raisestate: statenum_t::S_HEAD_RAISE1,
	},
	mobjinfo_t {
		// MT_BRUISER
		doomednum: 3003,
		spawnstate: statenum_t::S_BOSS_STND,
		spawnhealth: 1000,
		seestate: statenum_t::S_BOSS_RUN1,
		seesound: sfxenum_t::sfx_brssit,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_BOSS_PAIN,
		painchance: 50,
		painsound: sfxenum_t::sfx_dmpain,
		meleestate: statenum_t::S_BOSS_ATK1,
		missilestate: statenum_t::S_BOSS_ATK1,
		deathstate: statenum_t::S_BOSS_DIE1,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_brsdth,
		speed: 8,
		radius: 24 * FRACUNIT,
		height: 64 * FRACUNIT,
		mass: 1000,
		damage: 0,
		activesound: sfxenum_t::sfx_dmact,
		flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL,
		raisestate: statenum_t::S_BOSS_RAISE1,
	},
	mobjinfo_t {
		// MT_BRUISERSHOT
		doomednum: -1,
		spawnstate: statenum_t::S_BRBALL1,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_firsht,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_BRBALLX1,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_firxpl,
		speed: 15 * FRACUNIT,
		radius: 6 * FRACUNIT,
		height: 8 * FRACUNIT,
		mass: 100,
		damage: 8,
		activesound: sfxenum_t::sfx_None,
		flags: MF_NOBLOCKMAP | MF_MISSILE | MF_DROPOFF | MF_NOGRAVITY,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_KNIGHT
		doomednum: 69,
		spawnstate: statenum_t::S_BOS2_STND,
		spawnhealth: 500,
		seestate: statenum_t::S_BOS2_RUN1,
		seesound: sfxenum_t::sfx_kntsit,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_BOS2_PAIN,
		painchance: 50,
		painsound: sfxenum_t::sfx_dmpain,
		meleestate: statenum_t::S_BOS2_ATK1,
		missilestate: statenum_t::S_BOS2_ATK1,
		deathstate: statenum_t::S_BOS2_DIE1,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_kntdth,
		speed: 8,
		radius: 24 * FRACUNIT,
		height: 64 * FRACUNIT,
		mass: 1000,
		damage: 0,
		activesound: sfxenum_t::sfx_dmact,
		flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL,
		raisestate: statenum_t::S_BOS2_RAISE1,
	},
	mobjinfo_t {
		// MT_SKULL
		doomednum: 3006,
		spawnstate: statenum_t::S_SKULL_STND,
		spawnhealth: 100,
		seestate: statenum_t::S_SKULL_RUN1,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_sklatk,
		painstate: statenum_t::S_SKULL_PAIN,
		painchance: 256,
		painsound: sfxenum_t::sfx_dmpain,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_SKULL_ATK1,
		deathstate: statenum_t::S_SKULL_DIE1,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_firxpl,
		speed: 8,
		radius: 16 * FRACUNIT,
		height: 56 * FRACUNIT,
		mass: 50,
		damage: 3,
		activesound: sfxenum_t::sfx_dmact,
		flags: MF_SOLID | MF_SHOOTABLE | MF_FLOAT | MF_NOGRAVITY,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_SPIDER
		doomednum: 7,
		spawnstate: statenum_t::S_SPID_STND,
		spawnhealth: 3000,
		seestate: statenum_t::S_SPID_RUN1,
		seesound: sfxenum_t::sfx_spisit,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_shotgn,
		painstate: statenum_t::S_SPID_PAIN,
		painchance: 40,
		painsound: sfxenum_t::sfx_dmpain,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_SPID_ATK1,
		deathstate: statenum_t::S_SPID_DIE1,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_spidth,
		speed: 12,
		radius: 128 * FRACUNIT,
		height: 100 * FRACUNIT,
		mass: 1000,
		damage: 0,
		activesound: sfxenum_t::sfx_dmact,
		flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_BABY
		doomednum: 68,
		spawnstate: statenum_t::S_BSPI_STND,
		spawnhealth: 500,
		seestate: statenum_t::S_BSPI_SIGHT,
		seesound: sfxenum_t::sfx_bspsit,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_BSPI_PAIN,
		painchance: 128,
		painsound: sfxenum_t::sfx_dmpain,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_BSPI_ATK1,
		deathstate: statenum_t::S_BSPI_DIE1,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_bspdth,
		speed: 12,
		radius: 64 * FRACUNIT,
		height: 64 * FRACUNIT,
		mass: 600,
		damage: 0,
		activesound: sfxenum_t::sfx_bspact,
		flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL,
		raisestate: statenum_t::S_BSPI_RAISE1,
	},
	mobjinfo_t {
		// MT_CYBORG
		doomednum: 16,
		spawnstate: statenum_t::S_CYBER_STND,
		spawnhealth: 4000,
		seestate: statenum_t::S_CYBER_RUN1,
		seesound: sfxenum_t::sfx_cybsit,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_CYBER_PAIN,
		painchance: 20,
		painsound: sfxenum_t::sfx_dmpain,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_CYBER_ATK1,
		deathstate: statenum_t::S_CYBER_DIE1,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_cybdth,
		speed: 16,
		radius: 40 * FRACUNIT,
		height: 110 * FRACUNIT,
		mass: 1000,
		damage: 0,
		activesound: sfxenum_t::sfx_dmact,
		flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_PAIN
		doomednum: 71,
		spawnstate: statenum_t::S_PAIN_STND,
		spawnhealth: 400,
		seestate: statenum_t::S_PAIN_RUN1,
		seesound: sfxenum_t::sfx_pesit,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_PAIN_PAIN,
		painchance: 128,
		painsound: sfxenum_t::sfx_pepain,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_PAIN_ATK1,
		deathstate: statenum_t::S_PAIN_DIE1,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_pedth,
		speed: 8,
		radius: 31 * FRACUNIT,
		height: 56 * FRACUNIT,
		mass: 400,
		damage: 0,
		activesound: sfxenum_t::sfx_dmact,
		flags: MF_SOLID | MF_SHOOTABLE | MF_FLOAT | MF_NOGRAVITY | MF_COUNTKILL,
		raisestate: statenum_t::S_PAIN_RAISE1,
	},
	mobjinfo_t {
		// MT_WOLFSS
		doomednum: 84,
		spawnstate: statenum_t::S_SSWV_STND,
		spawnhealth: 50,
		seestate: statenum_t::S_SSWV_RUN1,
		seesound: sfxenum_t::sfx_sssit,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_SSWV_PAIN,
		painchance: 170,
		painsound: sfxenum_t::sfx_popain,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_SSWV_ATK1,
		deathstate: statenum_t::S_SSWV_DIE1,
		xdeathstate: statenum_t::S_SSWV_XDIE1,
		deathsound: sfxenum_t::sfx_ssdth,
		speed: 8,
		radius: 20 * FRACUNIT,
		height: 56 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_posact,
		flags: MF_SOLID | MF_SHOOTABLE | MF_COUNTKILL,
		raisestate: statenum_t::S_SSWV_RAISE1,
	},
	mobjinfo_t {
		// MT_KEEN
		doomednum: 72,
		spawnstate: statenum_t::S_KEENSTND,
		spawnhealth: 100,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_KEENPAIN,
		painchance: 256,
		painsound: sfxenum_t::sfx_keenpn,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_COMMKEEN,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_keendt,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 72 * FRACUNIT,
		mass: 10000000,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY | MF_SHOOTABLE | MF_COUNTKILL,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_BOSSBRAIN
		doomednum: 88,
		spawnstate: statenum_t::S_BRAIN,
		spawnhealth: 250,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_BRAIN_PAIN,
		painchance: 255,
		painsound: sfxenum_t::sfx_bospn,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_BRAIN_DIE1,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_bosdth,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 10000000,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID | MF_SHOOTABLE,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_BOSSSPIT
		doomednum: 89,
		spawnstate: statenum_t::S_BRAINEYE,
		spawnhealth: 1000,
		seestate: statenum_t::S_BRAINEYESEE,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 32 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_NOBLOCKMAP | MF_NOSECTOR,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_BOSSTARGET
		doomednum: 87,
		spawnstate: statenum_t::S_NULL,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 32 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_NOBLOCKMAP | MF_NOSECTOR,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_SPAWNSHOT
		doomednum: -1,
		spawnstate: statenum_t::S_SPAWN1,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_bospit,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_firxpl,
		speed: 10 * FRACUNIT,
		radius: 6 * FRACUNIT,
		height: 32 * FRACUNIT,
		mass: 100,
		damage: 3,
		activesound: sfxenum_t::sfx_None,
		flags: MF_NOBLOCKMAP | MF_MISSILE | MF_DROPOFF | MF_NOGRAVITY | MF_NOCLIP,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_SPAWNFIRE
		doomednum: -1,
		spawnstate: statenum_t::S_SPAWNFIRE1,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_NOBLOCKMAP | MF_NOGRAVITY,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_BARREL
		doomednum: 2035,
		spawnstate: statenum_t::S_BAR1,
		spawnhealth: 20,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_BEXP,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_barexp,
		speed: 0,
		radius: 10 * FRACUNIT,
		height: 42 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID | MF_SHOOTABLE | MF_NOBLOOD,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_TROOPSHOT
		doomednum: -1,
		spawnstate: statenum_t::S_TBALL1,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_firsht,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_TBALLX1,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_firxpl,
		speed: 10 * FRACUNIT,
		radius: 6 * FRACUNIT,
		height: 8 * FRACUNIT,
		mass: 100,
		damage: 3,
		activesound: sfxenum_t::sfx_None,
		flags: MF_NOBLOCKMAP | MF_MISSILE | MF_DROPOFF | MF_NOGRAVITY,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_HEADSHOT
		doomednum: -1,
		spawnstate: statenum_t::S_RBALL1,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_firsht,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_RBALLX1,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_firxpl,
		speed: 10 * FRACUNIT,
		radius: 6 * FRACUNIT,
		height: 8 * FRACUNIT,
		mass: 100,
		damage: 5,
		activesound: sfxenum_t::sfx_None,
		flags: MF_NOBLOCKMAP | MF_MISSILE | MF_DROPOFF | MF_NOGRAVITY,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_ROCKET
		doomednum: -1,
		spawnstate: statenum_t::S_ROCKET,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_rlaunc,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_EXPLODE1,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_barexp,
		speed: 20 * FRACUNIT,
		radius: 11 * FRACUNIT,
		height: 8 * FRACUNIT,
		mass: 100,
		damage: 20,
		activesound: sfxenum_t::sfx_None,
		flags: MF_NOBLOCKMAP | MF_MISSILE | MF_DROPOFF | MF_NOGRAVITY,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_PLASMA
		doomednum: -1,
		spawnstate: statenum_t::S_PLASBALL,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_plasma,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_PLASEXP,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_firxpl,
		speed: 25 * FRACUNIT,
		radius: 13 * FRACUNIT,
		height: 8 * FRACUNIT,
		mass: 100,
		damage: 5,
		activesound: sfxenum_t::sfx_None,
		flags: MF_NOBLOCKMAP | MF_MISSILE | MF_DROPOFF | MF_NOGRAVITY,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_BFG
		doomednum: -1,
		spawnstate: statenum_t::S_BFGSHOT,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_BFGLAND,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_rxplod,
		speed: 25 * FRACUNIT,
		radius: 13 * FRACUNIT,
		height: 8 * FRACUNIT,
		mass: 100,
		damage: 100,
		activesound: sfxenum_t::sfx_None,
		flags: MF_NOBLOCKMAP | MF_MISSILE | MF_DROPOFF | MF_NOGRAVITY,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_ARACHPLAZ
		doomednum: -1,
		spawnstate: statenum_t::S_ARACH_PLAZ,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_plasma,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_ARACH_PLEX,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_firxpl,
		speed: 25 * FRACUNIT,
		radius: 13 * FRACUNIT,
		height: 8 * FRACUNIT,
		mass: 100,
		damage: 5,
		activesound: sfxenum_t::sfx_None,
		flags: MF_NOBLOCKMAP | MF_MISSILE | MF_DROPOFF | MF_NOGRAVITY,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_PUFF
		doomednum: -1,
		spawnstate: statenum_t::S_PUFF1,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_NOBLOCKMAP | MF_NOGRAVITY,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_BLOOD
		doomednum: -1,
		spawnstate: statenum_t::S_BLOOD1,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_NOBLOCKMAP,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_TFOG
		doomednum: -1,
		spawnstate: statenum_t::S_TFOG,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_NOBLOCKMAP | MF_NOGRAVITY,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_IFOG
		doomednum: -1,
		spawnstate: statenum_t::S_IFOG,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_NOBLOCKMAP | MF_NOGRAVITY,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_TELEPORTMAN
		doomednum: 14,
		spawnstate: statenum_t::S_NULL,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_NOBLOCKMAP | MF_NOSECTOR,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_EXTRABFG
		doomednum: -1,
		spawnstate: statenum_t::S_BFGEXP,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_NOBLOCKMAP | MF_NOGRAVITY,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC0
		doomednum: 2018,
		spawnstate: statenum_t::S_ARM1,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC1
		doomednum: 2019,
		spawnstate: statenum_t::S_ARM2,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC2
		doomednum: 2014,
		spawnstate: statenum_t::S_BON1,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL | MF_COUNTITEM,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC3
		doomednum: 2015,
		spawnstate: statenum_t::S_BON2,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL | MF_COUNTITEM,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC4
		doomednum: 5,
		spawnstate: statenum_t::S_BKEY,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL | MF_NOTDMATCH,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC5
		doomednum: 13,
		spawnstate: statenum_t::S_RKEY,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL | MF_NOTDMATCH,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC6
		doomednum: 6,
		spawnstate: statenum_t::S_YKEY,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL | MF_NOTDMATCH,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC7
		doomednum: 39,
		spawnstate: statenum_t::S_YSKULL,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL | MF_NOTDMATCH,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC8
		doomednum: 38,
		spawnstate: statenum_t::S_RSKULL,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL | MF_NOTDMATCH,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC9
		doomednum: 40,
		spawnstate: statenum_t::S_BSKULL,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL | MF_NOTDMATCH,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC10
		doomednum: 2011,
		spawnstate: statenum_t::S_STIM,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC11
		doomednum: 2012,
		spawnstate: statenum_t::S_MEDI,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC12
		doomednum: 2013,
		spawnstate: statenum_t::S_SOUL,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL | MF_COUNTITEM,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_INV
		doomednum: 2022,
		spawnstate: statenum_t::S_PINV,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL | MF_COUNTITEM,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC13
		doomednum: 2023,
		spawnstate: statenum_t::S_PSTR,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL | MF_COUNTITEM,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_INS
		doomednum: 2024,
		spawnstate: statenum_t::S_PINS,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL | MF_COUNTITEM,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC14
		doomednum: 2025,
		spawnstate: statenum_t::S_SUIT,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC15
		doomednum: 2026,
		spawnstate: statenum_t::S_PMAP,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL | MF_COUNTITEM,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC16
		doomednum: 2045,
		spawnstate: statenum_t::S_PVIS,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL | MF_COUNTITEM,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MEGA
		doomednum: 83,
		spawnstate: statenum_t::S_MEGA,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL | MF_COUNTITEM,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_CLIP
		doomednum: 2007,
		spawnstate: statenum_t::S_CLIP,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC17
		doomednum: 2048,
		spawnstate: statenum_t::S_AMMO,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC18
		doomednum: 2010,
		spawnstate: statenum_t::S_ROCK,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC19
		doomednum: 2046,
		spawnstate: statenum_t::S_BROK,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC20
		doomednum: 2047,
		spawnstate: statenum_t::S_CELL,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC21
		doomednum: 17,
		spawnstate: statenum_t::S_CELP,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC22
		doomednum: 2008,
		spawnstate: statenum_t::S_SHEL,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC23
		doomednum: 2049,
		spawnstate: statenum_t::S_SBOX,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC24
		doomednum: 8,
		spawnstate: statenum_t::S_BPAK,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC25
		doomednum: 2006,
		spawnstate: statenum_t::S_BFUG,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_CHAINGUN
		doomednum: 2002,
		spawnstate: statenum_t::S_MGUN,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC26
		doomednum: 2005,
		spawnstate: statenum_t::S_CSAW,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC27
		doomednum: 2003,
		spawnstate: statenum_t::S_LAUN,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC28
		doomednum: 2004,
		spawnstate: statenum_t::S_PLAS,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_SHOTGUN
		doomednum: 2001,
		spawnstate: statenum_t::S_SHOT,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_SUPERSHOTGUN
		doomednum: 82,
		spawnstate: statenum_t::S_SHOT2,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPECIAL,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC29
		doomednum: 85,
		spawnstate: statenum_t::S_TECHLAMP,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC30
		doomednum: 86,
		spawnstate: statenum_t::S_TECH2LAMP,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC31
		doomednum: 2028,
		spawnstate: statenum_t::S_COLU,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC32
		doomednum: 30,
		spawnstate: statenum_t::S_TALLGRNCOL,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC33
		doomednum: 31,
		spawnstate: statenum_t::S_SHRTGRNCOL,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC34
		doomednum: 32,
		spawnstate: statenum_t::S_TALLREDCOL,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC35
		doomednum: 33,
		spawnstate: statenum_t::S_SHRTREDCOL,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC36
		doomednum: 37,
		spawnstate: statenum_t::S_SKULLCOL,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC37
		doomednum: 36,
		spawnstate: statenum_t::S_HEARTCOL,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC38
		doomednum: 41,
		spawnstate: statenum_t::S_EVILEYE,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC39
		doomednum: 42,
		spawnstate: statenum_t::S_FLOATSKULL,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC40
		doomednum: 43,
		spawnstate: statenum_t::S_TORCHTREE,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC41
		doomednum: 44,
		spawnstate: statenum_t::S_BLUETORCH,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC42
		doomednum: 45,
		spawnstate: statenum_t::S_GREENTORCH,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC43
		doomednum: 46,
		spawnstate: statenum_t::S_REDTORCH,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC44
		doomednum: 55,
		spawnstate: statenum_t::S_BTORCHSHRT,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC45
		doomednum: 56,
		spawnstate: statenum_t::S_GTORCHSHRT,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC46
		doomednum: 57,
		spawnstate: statenum_t::S_RTORCHSHRT,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC47
		doomednum: 47,
		spawnstate: statenum_t::S_STALAGTITE,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC48
		doomednum: 48,
		spawnstate: statenum_t::S_TECHPILLAR,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC49
		doomednum: 34,
		spawnstate: statenum_t::S_CANDLESTIK,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: 0,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC50
		doomednum: 35,
		spawnstate: statenum_t::S_CANDELABRA,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC51
		doomednum: 49,
		spawnstate: statenum_t::S_BLOODYTWITCH,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 68 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC52
		doomednum: 50,
		spawnstate: statenum_t::S_MEAT2,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 84 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC53
		doomednum: 51,
		spawnstate: statenum_t::S_MEAT3,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 84 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC54
		doomednum: 52,
		spawnstate: statenum_t::S_MEAT4,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 68 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC55
		doomednum: 53,
		spawnstate: statenum_t::S_MEAT5,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 52 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC56
		doomednum: 59,
		spawnstate: statenum_t::S_MEAT2,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 84 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPAWNCEILING | MF_NOGRAVITY,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC57
		doomednum: 60,
		spawnstate: statenum_t::S_MEAT4,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 68 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPAWNCEILING | MF_NOGRAVITY,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC58
		doomednum: 61,
		spawnstate: statenum_t::S_MEAT3,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 52 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPAWNCEILING | MF_NOGRAVITY,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC59
		doomednum: 62,
		spawnstate: statenum_t::S_MEAT5,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 52 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPAWNCEILING | MF_NOGRAVITY,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC60
		doomednum: 63,
		spawnstate: statenum_t::S_BLOODYTWITCH,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 68 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SPAWNCEILING | MF_NOGRAVITY,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC61
		doomednum: 22,
		spawnstate: statenum_t::S_HEAD_DIE6,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: 0,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC62
		doomednum: 15,
		spawnstate: statenum_t::S_PLAY_DIE7,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: 0,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC63
		doomednum: 18,
		spawnstate: statenum_t::S_POSS_DIE5,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: 0,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC64
		doomednum: 21,
		spawnstate: statenum_t::S_SARG_DIE6,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: 0,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC65
		doomednum: 23,
		spawnstate: statenum_t::S_SKULL_DIE6,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: 0,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC66
		doomednum: 20,
		spawnstate: statenum_t::S_TROO_DIE5,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: 0,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC67
		doomednum: 19,
		spawnstate: statenum_t::S_SPOS_DIE5,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: 0,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC68
		doomednum: 10,
		spawnstate: statenum_t::S_PLAY_XDIE9,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: 0,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC69
		doomednum: 12,
		spawnstate: statenum_t::S_PLAY_XDIE9,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: 0,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC70
		doomednum: 28,
		spawnstate: statenum_t::S_HEADSONSTICK,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC71
		doomednum: 24,
		spawnstate: statenum_t::S_GIBS,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: 0,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC72
		doomednum: 27,
		spawnstate: statenum_t::S_HEADONASTICK,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC73
		doomednum: 29,
		spawnstate: statenum_t::S_HEADCANDLES,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC74
		doomednum: 25,
		spawnstate: statenum_t::S_DEADSTICK,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC75
		doomednum: 26,
		spawnstate: statenum_t::S_LIVESTICK,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC76
		doomednum: 54,
		spawnstate: statenum_t::S_BIGTREE,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 32 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC77
		doomednum: 70,
		spawnstate: statenum_t::S_BBAR1,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC78
		doomednum: 73,
		spawnstate: statenum_t::S_HANGNOGUTS,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 88 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC79
		doomednum: 74,
		spawnstate: statenum_t::S_HANGBNOBRAIN,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 88 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC80
		doomednum: 75,
		spawnstate: statenum_t::S_HANGTLOOKDN,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 64 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC81
		doomednum: 76,
		spawnstate: statenum_t::S_HANGTSKULL,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 64 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC82
		doomednum: 77,
		spawnstate: statenum_t::S_HANGTLOOKUP,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 64 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC83
		doomednum: 78,
		spawnstate: statenum_t::S_HANGTNOBRAIN,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 16 * FRACUNIT,
		height: 64 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_SOLID | MF_SPAWNCEILING | MF_NOGRAVITY,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC84
		doomednum: 79,
		spawnstate: statenum_t::S_COLONGIBS,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_NOBLOCKMAP,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC85
		doomednum: 80,
		spawnstate: statenum_t::S_SMALLPOOL,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_NOBLOCKMAP,
		raisestate: statenum_t::S_NULL,
	},
	mobjinfo_t {
		// MT_MISC86
		doomednum: 81,
		spawnstate: statenum_t::S_BRAINSTEM,
		spawnhealth: 1000,
		seestate: statenum_t::S_NULL,
		seesound: sfxenum_t::sfx_None,
		reactiontime: 8,
		attacksound: sfxenum_t::sfx_None,
		painstate: statenum_t::S_NULL,
		painchance: 0,
		painsound: sfxenum_t::sfx_None,
		meleestate: statenum_t::S_NULL,
		missilestate: statenum_t::S_NULL,
		deathstate: statenum_t::S_NULL,
		xdeathstate: statenum_t::S_NULL,
		deathsound: sfxenum_t::sfx_None,
		speed: 0,
		radius: 20 * FRACUNIT,
		height: 16 * FRACUNIT,
		mass: 100,
		damage: 0,
		activesound: sfxenum_t::sfx_None,
		flags: MF_NOBLOCKMAP,
		raisestate: statenum_t::S_NULL,
	},
];
