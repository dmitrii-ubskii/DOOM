#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use crate::{
	doomdata::ML_SECRET,
	doomdef::GameMode_t,
	doomstat::gamemode,
	g_game::{G_ExitLevel, G_SecretExitLevel},
	i_system::I_Error,
	p_ceiling::EV_DoCeiling,
	p_doors::{EV_DoDoor, EV_DoLockedDoor, EV_VerticalDoor},
	p_floor::{EV_BuildStairs, EV_DoFloor},
	p_lights::EV_LightTurnOn,
	p_mobj::mobj_t,
	p_plats::EV_DoPlat,
	p_setup::sides,
	p_spec::{
		BUTTONTIME, EV_DoDonut, MAXBUTTONS, MAXSWITCHES, button_t, bwhere_e, ceiling_e, floor_e,
		plattype_e, stair_e, switchlist_t, vldoor_e,
	},
	r_data::R_TextureNumForName,
	r_defs::line_t,
	s_sound::S_StartSound,
	sounds::sfxenum_t,
};

type boolean = i32;

// CHANGE THE TEXTURE OF A WALL SWITCH TO ITS OPPOSITE
static alphSwitchList: [switchlist_t; 41] = [
	// Doom shareware episode 1 switches
	switchlist_t { name1: *b"SW1BRCOM\0", name2: *b"SW2BRCOM\0", episode: 1 },
	switchlist_t { name1: *b"SW1BRN1\0\0", name2: *b"SW2BRN1\0\0", episode: 1 },
	switchlist_t { name1: *b"SW1BRN2\0\0", name2: *b"SW2BRN2\0\0", episode: 1 },
	switchlist_t { name1: *b"SW1BRNGN\0", name2: *b"SW2BRNGN\0", episode: 1 },
	switchlist_t { name1: *b"SW1BROWN\0", name2: *b"SW2BROWN\0", episode: 1 },
	switchlist_t { name1: *b"SW1COMM\0\0", name2: *b"SW2COMM\0\0", episode: 1 },
	switchlist_t { name1: *b"SW1COMP\0\0", name2: *b"SW2COMP\0\0", episode: 1 },
	switchlist_t { name1: *b"SW1DIRT\0\0", name2: *b"SW2DIRT\0\0", episode: 1 },
	switchlist_t { name1: *b"SW1EXIT\0\0", name2: *b"SW2EXIT\0\0", episode: 1 },
	switchlist_t { name1: *b"SW1GRAY\0\0", name2: *b"SW2GRAY\0\0", episode: 1 },
	switchlist_t { name1: *b"SW1GRAY1\0", name2: *b"SW2GRAY1\0", episode: 1 },
	switchlist_t { name1: *b"SW1METAL\0", name2: *b"SW2METAL\0", episode: 1 },
	switchlist_t { name1: *b"SW1PIPE\0\0", name2: *b"SW2PIPE\0\0", episode: 1 },
	switchlist_t { name1: *b"SW1SLAD\0\0", name2: *b"SW2SLAD\0\0", episode: 1 },
	switchlist_t { name1: *b"SW1STARG\0", name2: *b"SW2STARG\0", episode: 1 },
	switchlist_t { name1: *b"SW1STON1\0", name2: *b"SW2STON1\0", episode: 1 },
	switchlist_t { name1: *b"SW1STON2\0", name2: *b"SW2STON2\0", episode: 1 },
	switchlist_t { name1: *b"SW1STONE\0", name2: *b"SW2STONE\0", episode: 1 },
	switchlist_t { name1: *b"SW1STRTN\0", name2: *b"SW2STRTN\0", episode: 1 },
	// Doom registered episodes 2&3 switches
	switchlist_t { name1: *b"SW1BLUE\0\0", name2: *b"SW2BLUE\0\0", episode: 2 },
	switchlist_t { name1: *b"SW1CMT\0\0\0", name2: *b"SW2CMT\0\0\0", episode: 2 },
	switchlist_t { name1: *b"SW1GARG\0\0", name2: *b"SW2GARG\0\0", episode: 2 },
	switchlist_t { name1: *b"SW1GSTON\0", name2: *b"SW2GSTON\0", episode: 2 },
	switchlist_t { name1: *b"SW1HOT\0\0\0", name2: *b"SW2HOT\0\0\0", episode: 2 },
	switchlist_t { name1: *b"SW1LION\0\0", name2: *b"SW2LION\0\0", episode: 2 },
	switchlist_t { name1: *b"SW1SATYR\0", name2: *b"SW2SATYR\0", episode: 2 },
	switchlist_t { name1: *b"SW1SKIN\0\0", name2: *b"SW2SKIN\0\0", episode: 2 },
	switchlist_t { name1: *b"SW1VINE\0\0", name2: *b"SW2VINE\0\0", episode: 2 },
	switchlist_t { name1: *b"SW1WOOD\0\0", name2: *b"SW2WOOD\0\0", episode: 2 },
	// Doom II switches
	switchlist_t { name1: *b"SW1PANEL\0", name2: *b"SW2PANEL\0", episode: 3 },
	switchlist_t { name1: *b"SW1ROCK\0\0", name2: *b"SW2ROCK\0\0", episode: 3 },
	switchlist_t { name1: *b"SW1MET2\0\0", name2: *b"SW2MET2\0\0", episode: 3 },
	switchlist_t { name1: *b"SW1WDMET\0", name2: *b"SW2WDMET\0", episode: 3 },
	switchlist_t { name1: *b"SW1BRIK\0\0", name2: *b"SW2BRIK\0\0", episode: 3 },
	switchlist_t { name1: *b"SW1MOD1\0\0", name2: *b"SW2MOD1\0\0", episode: 3 },
	switchlist_t { name1: *b"SW1ZIM\0\0\0", name2: *b"SW2ZIM\0\0\0", episode: 3 },
	switchlist_t { name1: *b"SW1STON6\0", name2: *b"SW2STON6\0", episode: 3 },
	switchlist_t { name1: *b"SW1TEK\0\0\0", name2: *b"SW2TEK\0\0\0", episode: 3 },
	switchlist_t { name1: *b"SW1MARB\0\0", name2: *b"SW2MARB\0\0", episode: 3 },
	switchlist_t { name1: *b"SW1SKULL\0", name2: *b"SW2SKULL\0", episode: 3 },
	switchlist_t { name1: [0; 9], name2: [0; 9], episode: 0 },
];

static mut switchlist: [i16; MAXSWITCHES * 2] = [0; MAXSWITCHES * 2];
static mut numswitches: usize = 0;
pub(crate) static mut buttonlist: [button_t; MAXBUTTONS] = [button_t::new(); MAXBUTTONS];

// P_InitSwitchList
// Only called at game initialization.
pub(crate) fn P_InitSwitchList() {
	unsafe {
		let mut episode = 1;

		if gamemode == GameMode_t::registered {
			episode = 2;
		} else if gamemode == GameMode_t::commercial {
			episode = 3;
		}

		let mut index = 0;
		for i in 0..MAXSWITCHES {
			if alphSwitchList[i].episode == 0 {
				numswitches = index / 2;
				switchlist[index] = -1;
				break;
			}

			if alphSwitchList[i].episode <= episode {
				switchlist[index] =
					R_TextureNumForName(alphSwitchList[i].name1.as_ptr().cast()) as i16;
				index += 1;
				switchlist[index] =
					R_TextureNumForName(alphSwitchList[i].name2.as_ptr().cast()) as i16;
				index += 1;
			}
		}
	}
}

// Start a button counting down till it turns off.
fn P_StartButton(line: *mut line_t, w: bwhere_e, texture: i32, time: u32) {
	unsafe {
		// See if button is already pressed
		for i in 0..MAXBUTTONS {
			if buttonlist[i].btimer != 0 && buttonlist[i].line == line {
				return;
			}
		}

		for i in 0..MAXBUTTONS {
			if buttonlist[i].btimer == 0 {
				buttonlist[i].line = line;
				buttonlist[i].where_ = w;
				buttonlist[i].btexture = texture;
				buttonlist[i].btimer = time;
				buttonlist[i].soundorg = (&raw mut (*(*line).frontsector).soundorg).cast();
				return;
			}
		}

		I_Error(c"P_StartButton: no button slots left!".as_ptr());
	}
}

// Function that changes wall texture.
// Tell it if switch is ok to use again (1=yes, it's a button).
pub(crate) fn P_ChangeSwitchTexture(line: &mut line_t, useAgain: bool) {
	unsafe {
		if !useAgain {
			line.special = 0;
		}

		let side_0 = sides.wrapping_add(line.sidenum[0] as usize);
		let texTop = (*side_0).toptexture;
		let texMid = (*side_0).midtexture;
		let texBot = (*side_0).bottomtexture;

		let mut sound = sfxenum_t::sfx_swtchn;

		// EXIT SWITCH?
		if line.special == 11 {
			sound = sfxenum_t::sfx_swtchx;
		}

		for i in 0..numswitches * 2 {
			if switchlist[i] == texTop {
				S_StartSound(buttonlist[0].soundorg.cast(), sound);
				(*side_0).toptexture = switchlist[i ^ 1];

				if useAgain {
					P_StartButton(line, bwhere_e::top, switchlist[i] as i32, BUTTONTIME);
				}

				return;
			} else if switchlist[i] == texMid {
				S_StartSound(buttonlist[0].soundorg.cast(), sound);
				(*side_0).midtexture = switchlist[i ^ 1];

				if useAgain {
					P_StartButton(line, bwhere_e::middle, switchlist[i] as i32, BUTTONTIME);
				}

				return;
			} else if switchlist[i] == texBot {
				S_StartSound(buttonlist[0].soundorg.cast(), sound);
				(*side_0).bottomtexture = switchlist[i ^ 1];

				if useAgain {
					P_StartButton(line, bwhere_e::bottom, switchlist[i] as i32, BUTTONTIME);
				}

				return;
			}
		}
	}
}

// P_UseSpecialLine
// Called when a thing uses a special line.
// Only the front sides of lines are usable.
#[unsafe(no_mangle)]
pub extern "C" fn P_UseSpecialLine(thing: &mut mobj_t, line: &mut line_t, side: i32) -> boolean {
	// Err...
	// Use the back sides of VERY SPECIAL lines...
	if side != 0 {
		match line.special {
			124 => (), // Sliding door open&close / UNUSED?
			_ => return 0,
		}
	}

	// Switches that other things can activate.
	if thing.player.is_null() {
		// never open secret doors
		if line.flags as usize & ML_SECRET != 0 {
			return 0;
		}

		match line.special {
			// MANUAL DOOR RAISE | MANUAL BLUE | MANUAL RED | MANUAL YELLOW
			1 | 32 | 33 | 34 => (),
			_ => return 0,
		}
	}

	// do something
	match line.special {
		// MANUALS
		// Vertical Door | Blue Door/Locked |
		// Yellow Door /Locked | Red Door /Locked |
		// Manual door open | Blue locked door open |
		// Red locked door open | Yellow locked door open |
		// Blazing door raise | Blazing door open
		1 | 26 | 27 | 28 | 31 | 32 | 33 | 34 | 117 | 118 => EV_VerticalDoor(line, thing),

		// SWITCHES
		7 => {
			// Build Stairs
			if EV_BuildStairs(line, stair_e::build8) {
				P_ChangeSwitchTexture(line, false);
			}
		}

		9 => {
			// Change Donut
			if EV_DoDonut(line) {
				P_ChangeSwitchTexture(line, false);
			}
		}

		11 => {
			// Exit level
			P_ChangeSwitchTexture(line, false);
			G_ExitLevel();
		}

		14 => {
			// Raise Floor 32 and change texture
			if EV_DoPlat(line, plattype_e::raiseAndChange, 32) {
				P_ChangeSwitchTexture(line, false);
			}
		}

		15 => {
			// Raise Floor 24 and change texture
			if EV_DoPlat(line, plattype_e::raiseAndChange, 24) {
				P_ChangeSwitchTexture(line, false);
			}
		}

		18 => {
			// Raise Floor to next highest floor
			if EV_DoFloor(line, floor_e::raiseFloorToNearest) {
				P_ChangeSwitchTexture(line, false);
			}
		}

		20 => {
			// Raise Plat next highest floor and change texture
			if EV_DoPlat(line, plattype_e::raiseToNearestAndChange, 0) {
				P_ChangeSwitchTexture(line, false);
			}
		}

		21 => {
			// PlatDownWaitUpStay
			if EV_DoPlat(line, plattype_e::downWaitUpStay, 0) {
				P_ChangeSwitchTexture(line, false);
			}
		}

		23 => {
			// Lower Floor to Lowest
			if EV_DoFloor(line, floor_e::lowerFloorToLowest) {
				P_ChangeSwitchTexture(line, false);
			}
		}

		29 => {
			// Raise Door
			if EV_DoDoor(line, vldoor_e::normal) {
				P_ChangeSwitchTexture(line, false);
			}
		}

		41 => {
			// Lower Ceiling to Floor
			if EV_DoCeiling(line, ceiling_e::lowerToFloor) {
				P_ChangeSwitchTexture(line, false);
			}
		}

		71 => {
			// Turbo Lower Floor
			if EV_DoFloor(line, floor_e::turboLower) {
				P_ChangeSwitchTexture(line, false);
			}
		}

		49 => {
			// Ceiling Crush And Raise
			if EV_DoCeiling(line, ceiling_e::crushAndRaise) {
				P_ChangeSwitchTexture(line, false);
			}
		}

		50 => {
			// Close Door
			if EV_DoDoor(line, vldoor_e::close) {
				P_ChangeSwitchTexture(line, false);
			}
		}

		51 => {
			// Secret EXIT
			P_ChangeSwitchTexture(line, false);
			G_SecretExitLevel();
		}

		55 => {
			// Raise Floor Crush
			if EV_DoFloor(line, floor_e::raiseFloorCrush) {
				P_ChangeSwitchTexture(line, false);
			}
		}

		101 => {
			// Raise Floor
			if EV_DoFloor(line, floor_e::raiseFloor) {
				P_ChangeSwitchTexture(line, false);
			}
		}

		102 => {
			// Lower Floor to Surrounding floor height
			if EV_DoFloor(line, floor_e::lowerFloor) {
				P_ChangeSwitchTexture(line, false);
			}
		}

		103 => {
			// Open Door
			if EV_DoDoor(line, vldoor_e::open) {
				P_ChangeSwitchTexture(line, false);
			}
		}

		111 => {
			// Blazing Door Raise (faster than TURBO!)
			if EV_DoDoor(line, vldoor_e::blazeRaise) {
				P_ChangeSwitchTexture(line, false);
			}
		}

		112 => {
			// Blazing Door Open (faster than TURBO!)
			if EV_DoDoor(line, vldoor_e::blazeOpen) {
				P_ChangeSwitchTexture(line, false);
			}
		}

		113 => {
			// Blazing Door Close (faster than TURBO!)
			if EV_DoDoor(line, vldoor_e::blazeClose) {
				P_ChangeSwitchTexture(line, false);
			}
		}

		122 => {
			// Blazing PlatDownWaitUpStay
			if EV_DoPlat(line, plattype_e::blazeDWUS, 0) {
				P_ChangeSwitchTexture(line, false);
			}
		}

		127 => {
			// Build Stairs Turbo 16
			if EV_BuildStairs(line, stair_e::turbo16) {
				P_ChangeSwitchTexture(line, false);
			}
		}

		131 => {
			// Raise Floor Turbo
			if EV_DoFloor(line, floor_e::raiseFloorTurbo) {
				P_ChangeSwitchTexture(line, false);
			}
		}

		// BlzOpenDoor BLUE | BlzOpenDoor RED | BlzOpenDoor YELLOW
		133 | 135 | 137 => {
			if EV_DoLockedDoor(line, vldoor_e::blazeOpen, thing) {
				P_ChangeSwitchTexture(line, false);
			}
		}

		140 => {
			// Raise Floor 512
			if EV_DoFloor(line, floor_e::raiseFloor512) {
				P_ChangeSwitchTexture(line, false);
			}
		}

		// BUTTONS
		42 => {
			// Close Door
			if EV_DoDoor(line, vldoor_e::close) {
				P_ChangeSwitchTexture(line, true);
			}
		}

		43 => {
			// Lower Ceiling to Floor
			if EV_DoCeiling(line, ceiling_e::lowerToFloor) {
				P_ChangeSwitchTexture(line, true);
			}
		}

		45 => {
			// Lower Floor to Surrounding floor height
			if EV_DoFloor(line, floor_e::lowerFloor) {
				P_ChangeSwitchTexture(line, true);
			}
		}

		60 => {
			// Lower Floor to Lowest
			if EV_DoFloor(line, floor_e::lowerFloorToLowest) {
				P_ChangeSwitchTexture(line, true);
			}
		}

		61 => {
			// Open Door
			if EV_DoDoor(line, vldoor_e::open) {
				P_ChangeSwitchTexture(line, true);
			}
		}

		62 => {
			// PlatDownWaitUpStay
			if EV_DoPlat(line, plattype_e::downWaitUpStay, 1) {
				P_ChangeSwitchTexture(line, true);
			}
		}

		63 => {
			// Raise Door
			if EV_DoDoor(line, vldoor_e::normal) {
				P_ChangeSwitchTexture(line, true);
			}
		}

		64 => {
			// Raise Floor to ceiling
			if EV_DoFloor(line, floor_e::raiseFloor) {
				P_ChangeSwitchTexture(line, true);
			}
		}

		66 => {
			// Raise Floor 24 and change texture
			if EV_DoPlat(line, plattype_e::raiseAndChange, 24) {
				P_ChangeSwitchTexture(line, true);
			}
		}

		67 => {
			// Raise Floor 32 and change texture
			if EV_DoPlat(line, plattype_e::raiseAndChange, 32) {
				P_ChangeSwitchTexture(line, true);
			}
		}

		65 => {
			// Raise Floor Crush
			if EV_DoFloor(line, floor_e::raiseFloorCrush) {
				P_ChangeSwitchTexture(line, true);
			}
		}

		68 => {
			// Raise Plat to next highest floor and change texture
			if EV_DoPlat(line, plattype_e::raiseToNearestAndChange, 0) {
				P_ChangeSwitchTexture(line, true);
			}
		}

		69 => {
			// Raise Floor to next highest floor
			if EV_DoFloor(line, floor_e::raiseFloorToNearest) {
				P_ChangeSwitchTexture(line, true);
			}
		}

		70 => {
			// Turbo Lower Floor
			if EV_DoFloor(line, floor_e::turboLower) {
				P_ChangeSwitchTexture(line, true);
			}
		}

		114 => {
			// Blazing Door Raise (faster than TURBO!)
			if EV_DoDoor(line, vldoor_e::blazeRaise) {
				P_ChangeSwitchTexture(line, true);
			}
		}

		115 => {
			// Blazing Door Open (faster than TURBO!)
			if EV_DoDoor(line, vldoor_e::blazeOpen) {
				P_ChangeSwitchTexture(line, true);
			}
		}

		116 => {
			// Blazing Door Close (faster than TURBO!)
			if EV_DoDoor(line, vldoor_e::blazeClose) {
				P_ChangeSwitchTexture(line, true);
			}
		}

		123 => {
			// Blazing PlatDownWaitUpStay
			if EV_DoPlat(line, plattype_e::blazeDWUS, 0) {
				P_ChangeSwitchTexture(line, true);
			}
		}

		132 => {
			// Raise Floor Turbo
			if EV_DoFloor(line, floor_e::raiseFloorTurbo) {
				P_ChangeSwitchTexture(line, true);
			}
		}

		// BlzOpenDoor BLUE | BlzOpenDoor RED | BlzOpenDoor YELLOW
		99 | 134 | 136 => {
			if EV_DoLockedDoor(line, vldoor_e::blazeOpen, thing) {
				P_ChangeSwitchTexture(line, true);
			}
		}

		138 => {
			// Light Turn On
			EV_LightTurnOn(line, 255);
			P_ChangeSwitchTexture(line, true);
		}

		139 => {
			// Light Turn Off
			EV_LightTurnOn(line, 35);
			P_ChangeSwitchTexture(line, true);
		}

		_ => (),
	}

	1
}
