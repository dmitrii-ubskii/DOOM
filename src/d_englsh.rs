#![allow(clippy::as_conversions)]

use std::ffi::{CStr, c_char};

use crate::dstrings::Smuggle;

//	Printed strings for translation

// D_Main.C
pub(crate) const D_DEVSTR: &CStr = c"Development mode ON.\n";
pub(crate) const D_CDROM: &CStr = c"CD-ROM Version: default.cfg from c:\\doomdata\n";

//	M_Menu.C
macro_rules! PRESSKEY {
	() => {
		"press a key.\0"
	};
}
macro_rules! PRESSYN {
	() => {
		"press y or n.\0"
	};
}
/*
macro_rules! QUITMSG {
	() => {
		"are you sure you want to\nquit this great game?\0"
	};
}
*/
pub(crate) const LOADNET: *const c_char =
	concat!("you can't do load while in a net game!\n\n", PRESSKEY!()).as_ptr().cast();
pub(crate) const QLOADNET: *const c_char =
	concat!("you can't quickload during a netgame!\n\n", PRESSKEY!()).as_ptr().cast();
pub(crate) const QSAVESPOT: *const c_char =
	concat!("you haven't picked a quicksave slot yet!\n\n", PRESSKEY!()).as_ptr().cast();
pub(crate) const SAVEDEAD: *const c_char =
	concat!("you can't save if you aren't playing!\n\n", PRESSKEY!()).as_ptr().cast();
pub(crate) const QSPROMPT: *const c_char =
	concat!("quicksave over your game named\n\n'%s'?\n\n", PRESSYN!()).as_ptr().cast();
pub(crate) const QLPROMPT: *const c_char =
	concat!("do you want to quickload the game named\n\n'%s'?\n\n", PRESSYN!()).as_ptr().cast();

pub(crate) const NEWGAME: *const c_char =
	concat!("you can't start a new game\nwhile in a network game.\n\n", PRESSKEY!())
		.as_ptr()
		.cast();

pub(crate) const NIGHTMARE: *const c_char =
	concat!("are you sure? this skill level\nisn't even remotely fair.\n\n", PRESSYN!())
		.as_ptr()
		.cast();

pub(crate) const SWSTRING: *const c_char = concat!(
	"this is the shareware version of doom.\n\nyou need to order the entire trilogy.\n\n",
	PRESSKEY!()
)
.as_ptr()
.cast();

pub(crate) const MSGOFF: *const c_char = c"Messages OFF".as_ptr();
pub(crate) const MSGON: *const c_char = c"Messages ON".as_ptr();
pub(crate) const NETEND: *const c_char =
	concat!("you can't end a netgame!\n\n", PRESSKEY!()).as_ptr().cast();
pub(crate) const ENDGAME: *const c_char =
	concat!("are you sure you want to end the game?\n\n", PRESSYN!()).as_ptr().cast();

macro_rules! DOSY {
	() => {
		"(press y to quit)\0"
	};
}
pub(crate) use DOSY;

// pub(crate) const DETAILHI: *const c_char = c"High detail".as_ptr();
// pub(crate) const DETAILLO: *const c_char = c"Low detail".as_ptr();
pub(crate) const GAMMALVL0: [u8; 26] = *b"Gamma correction OFF\0\0\0\0\0\0";
pub(crate) const GAMMALVL1: [u8; 26] = *b"Gamma correction level 1\0\0";
pub(crate) const GAMMALVL2: [u8; 26] = *b"Gamma correction level 2\0\0";
pub(crate) const GAMMALVL3: [u8; 26] = *b"Gamma correction level 3\0\0";
pub(crate) const GAMMALVL4: [u8; 26] = *b"Gamma correction level 4\0\0";
pub(crate) const EMPTYSTRING: *const c_char = c"empty slot".as_ptr();

//	P_inter.C
pub(crate) const GOTARMOR: *const c_char = c"Picked up the armor.".as_ptr();
pub(crate) const GOTMEGA: *const c_char = c"Picked up the MegaArmor!".as_ptr();
pub(crate) const GOTHTHBONUS: *const c_char = c"Picked up a health bonus.".as_ptr();
pub(crate) const GOTARMBONUS: *const c_char = c"Picked up an armor bonus.".as_ptr();
pub(crate) const GOTSTIM: *const c_char = c"Picked up a stimpack.".as_ptr();
pub(crate) const GOTMEDINEED: *const c_char = c"Picked up a medikit that you REALLY need!".as_ptr();
pub(crate) const GOTMEDIKIT: *const c_char = c"Picked up a medikit.".as_ptr();
pub(crate) const GOTSUPER: *const c_char = c"Supercharge!".as_ptr();

pub(crate) const GOTBLUECARD: *const c_char = c"Picked up a blue keycard.".as_ptr();
pub(crate) const GOTYELWCARD: *const c_char = c"Picked up a yellow keycard.".as_ptr();
pub(crate) const GOTREDCARD: *const c_char = c"Picked up a red keycard.".as_ptr();
pub(crate) const GOTBLUESKUL: *const c_char = c"Picked up a blue skull key.".as_ptr();
pub(crate) const GOTYELWSKUL: *const c_char = c"Picked up a yellow skull key.".as_ptr();
pub(crate) const GOTREDSKULL: *const c_char = c"Picked up a red skull key.".as_ptr();

pub(crate) const GOTINVUL: *const c_char = c"Invulnerability!".as_ptr();
pub(crate) const GOTBERSERK: *const c_char = c"Berserk!".as_ptr();
pub(crate) const GOTINVIS: *const c_char = c"Partial Invisibility".as_ptr();
pub(crate) const GOTSUIT: *const c_char = c"Radiation Shielding Suit".as_ptr();
pub(crate) const GOTMAP: *const c_char = c"Computer Area Map".as_ptr();
pub(crate) const GOTVISOR: *const c_char = c"Light Amplification Visor".as_ptr();
pub(crate) const GOTMSPHERE: *const c_char = c"MegaSphere!".as_ptr();

pub(crate) const GOTCLIP: *const c_char = c"Picked up a clip.".as_ptr();
pub(crate) const GOTCLIPBOX: *const c_char = c"Picked up a box of bullets.".as_ptr();
pub(crate) const GOTROCKET: *const c_char = c"Picked up a rocket.".as_ptr();
pub(crate) const GOTROCKBOX: *const c_char = c"Picked up a box of rockets.".as_ptr();
pub(crate) const GOTCELL: *const c_char = c"Picked up an energy cell.".as_ptr();
pub(crate) const GOTCELLBOX: *const c_char = c"Picked up an energy cell pack.".as_ptr();
pub(crate) const GOTSHELLS: *const c_char = c"Picked up 4 shotgun shells.".as_ptr();
pub(crate) const GOTSHELLBOX: *const c_char = c"Picked up a box of shotgun shells.".as_ptr();
pub(crate) const GOTBACKPACK: *const c_char = c"Picked up a backpack full of ammo!".as_ptr();

pub(crate) const GOTBFG9000: *const c_char = c"You got the BFG9000!  Oh, yes.".as_ptr();
pub(crate) const GOTCHAINGUN: *const c_char = c"You got the chaingun!".as_ptr();
pub(crate) const GOTCHAINSAW: *const c_char = c"A chainsaw!  Find some meat!".as_ptr();
pub(crate) const GOTLAUNCHER: *const c_char = c"You got the rocket launcher!".as_ptr();
pub(crate) const GOTPLASMA: *const c_char = c"You got the plasma gun!".as_ptr();
pub(crate) const GOTSHOTGUN: *const c_char = c"You got the shotgun!".as_ptr();
pub(crate) const GOTSHOTGUN2: *const c_char = c"You got the super shotgun!".as_ptr();

// P_Doors.C
pub(crate) const PD_BLUEO: *const c_char = c"You need a blue key to activate this object".as_ptr();
pub(crate) const PD_REDO: *const c_char = c"You need a red key to activate this object".as_ptr();
pub(crate) const PD_YELLOWO: *const c_char =
	c"You need a yellow key to activate this object".as_ptr();
pub(crate) const PD_BLUEK: *const c_char = c"You need a blue key to open this door".as_ptr();
pub(crate) const PD_REDK: *const c_char = c"You need a red key to open this door".as_ptr();
pub(crate) const PD_YELLOWK: *const c_char = c"You need a yellow key to open this door".as_ptr();

//	G_game.C
pub(crate) const GGSAVED: *const c_char = c"game saved.".as_ptr();

//	HU_stuff.C
pub(crate) const HUSTR_MSGU: *const c_char = c"[Message unsent]".as_ptr();

pub(crate) const HUSTR_E1M1: *const c_char = c"E1M1: Hangar".as_ptr();
pub(crate) const HUSTR_E1M2: *const c_char = c"E1M2: Nuclear Plant".as_ptr();
pub(crate) const HUSTR_E1M3: *const c_char = c"E1M3: Toxin Refinery".as_ptr();
pub(crate) const HUSTR_E1M4: *const c_char = c"E1M4: Command Control".as_ptr();
pub(crate) const HUSTR_E1M5: *const c_char = c"E1M5: Phobos Lab".as_ptr();
pub(crate) const HUSTR_E1M6: *const c_char = c"E1M6: Central Processing".as_ptr();
pub(crate) const HUSTR_E1M7: *const c_char = c"E1M7: Computer Station".as_ptr();
pub(crate) const HUSTR_E1M8: *const c_char = c"E1M8: Phobos Anomaly".as_ptr();
pub(crate) const HUSTR_E1M9: *const c_char = c"E1M9: Military Base".as_ptr();

pub(crate) const HUSTR_E2M1: *const c_char = c"E2M1: Deimos Anomaly".as_ptr();
pub(crate) const HUSTR_E2M2: *const c_char = c"E2M2: Containment Area".as_ptr();
pub(crate) const HUSTR_E2M3: *const c_char = c"E2M3: Refinery".as_ptr();
pub(crate) const HUSTR_E2M4: *const c_char = c"E2M4: Deimos Lab".as_ptr();
pub(crate) const HUSTR_E2M5: *const c_char = c"E2M5: Command Center".as_ptr();
pub(crate) const HUSTR_E2M6: *const c_char = c"E2M6: Halls of the Damned".as_ptr();
pub(crate) const HUSTR_E2M7: *const c_char = c"E2M7: Spawning Vats".as_ptr();
pub(crate) const HUSTR_E2M8: *const c_char = c"E2M8: Tower of Babel".as_ptr();
pub(crate) const HUSTR_E2M9: *const c_char = c"E2M9: Fortress of Mystery".as_ptr();

pub(crate) const HUSTR_E3M1: *const c_char = c"E3M1: Hell Keep".as_ptr();
pub(crate) const HUSTR_E3M2: *const c_char = c"E3M2: Slough of Despair".as_ptr();
pub(crate) const HUSTR_E3M3: *const c_char = c"E3M3: Pandemonium".as_ptr();
pub(crate) const HUSTR_E3M4: *const c_char = c"E3M4: House of Pain".as_ptr();
pub(crate) const HUSTR_E3M5: *const c_char = c"E3M5: Unholy Cathedral".as_ptr();
pub(crate) const HUSTR_E3M6: *const c_char = c"E3M6: Mt. Erebus".as_ptr();
pub(crate) const HUSTR_E3M7: *const c_char = c"E3M7: Limbo".as_ptr();
pub(crate) const HUSTR_E3M8: *const c_char = c"E3M8: Dis".as_ptr();
pub(crate) const HUSTR_E3M9: *const c_char = c"E3M9: Warrens".as_ptr();

pub(crate) const HUSTR_E4M1: *const c_char = c"E4M1: Hell Beneath".as_ptr();
pub(crate) const HUSTR_E4M2: *const c_char = c"E4M2: Perfect Hatred".as_ptr();
pub(crate) const HUSTR_E4M3: *const c_char = c"E4M3: Sever The Wicked".as_ptr();
pub(crate) const HUSTR_E4M4: *const c_char = c"E4M4: Unruly Evil".as_ptr();
pub(crate) const HUSTR_E4M5: *const c_char = c"E4M5: They Will Repent".as_ptr();
pub(crate) const HUSTR_E4M6: *const c_char = c"E4M6: Against Thee Wickedly".as_ptr();
pub(crate) const HUSTR_E4M7: *const c_char = c"E4M7: And Hell Followed".as_ptr();
pub(crate) const HUSTR_E4M8: *const c_char = c"E4M8: Unto The Cruel".as_ptr();
pub(crate) const HUSTR_E4M9: *const c_char = c"E4M9: Fear".as_ptr();

pub(crate) const HUSTR_1: *const c_char = c"level 1: entryway".as_ptr();
pub(crate) const HUSTR_2: *const c_char = c"level 2: underhalls".as_ptr();
pub(crate) const HUSTR_3: *const c_char = c"level 3: the gantlet".as_ptr();
pub(crate) const HUSTR_4: *const c_char = c"level 4: the focus".as_ptr();
pub(crate) const HUSTR_5: *const c_char = c"level 5: the waste tunnels".as_ptr();
pub(crate) const HUSTR_6: *const c_char = c"level 6: the crusher".as_ptr();
pub(crate) const HUSTR_7: *const c_char = c"level 7: dead simple".as_ptr();
pub(crate) const HUSTR_8: *const c_char = c"level 8: tricks and traps".as_ptr();
pub(crate) const HUSTR_9: *const c_char = c"level 9: the pit".as_ptr();
pub(crate) const HUSTR_10: *const c_char = c"level 10: refueling base".as_ptr();
pub(crate) const HUSTR_11: *const c_char = c"level 11: 'o' of destruction!".as_ptr();

pub(crate) const HUSTR_12: *const c_char = c"level 12: the factory".as_ptr();
pub(crate) const HUSTR_13: *const c_char = c"level 13: downtown".as_ptr();
pub(crate) const HUSTR_14: *const c_char = c"level 14: the inmost dens".as_ptr();
pub(crate) const HUSTR_15: *const c_char = c"level 15: industrial zone".as_ptr();
pub(crate) const HUSTR_16: *const c_char = c"level 16: suburbs".as_ptr();
pub(crate) const HUSTR_17: *const c_char = c"level 17: tenements".as_ptr();
pub(crate) const HUSTR_18: *const c_char = c"level 18: the courtyard".as_ptr();
pub(crate) const HUSTR_19: *const c_char = c"level 19: the citadel".as_ptr();
pub(crate) const HUSTR_20: *const c_char = c"level 20: gotcha!".as_ptr();

pub(crate) const HUSTR_21: *const c_char = c"level 21: nirvana".as_ptr();
pub(crate) const HUSTR_22: *const c_char = c"level 22: the catacombs".as_ptr();
pub(crate) const HUSTR_23: *const c_char = c"level 23: barrels o' fun".as_ptr();
pub(crate) const HUSTR_24: *const c_char = c"level 24: the chasm".as_ptr();
pub(crate) const HUSTR_25: *const c_char = c"level 25: bloodfalls".as_ptr();
pub(crate) const HUSTR_26: *const c_char = c"level 26: the abandoned mines".as_ptr();
pub(crate) const HUSTR_27: *const c_char = c"level 27: monster condo".as_ptr();
pub(crate) const HUSTR_28: *const c_char = c"level 28: the spirit world".as_ptr();
pub(crate) const HUSTR_29: *const c_char = c"level 29: the living end".as_ptr();
pub(crate) const HUSTR_30: *const c_char = c"level 30: icon of sin".as_ptr();

pub(crate) const HUSTR_31: *const c_char = c"level 31: wolfenstein".as_ptr();
pub(crate) const HUSTR_32: *const c_char = c"level 32: grosse".as_ptr();

// pub(crate) const PHUSTR_1: *const c_char = c"level 1: congo".as_ptr();
// pub(crate) const PHUSTR_2: *const c_char = c"level 2: well of souls".as_ptr();
// pub(crate) const PHUSTR_3: *const c_char = c"level 3: aztec".as_ptr();
// pub(crate) const PHUSTR_4: *const c_char = c"level 4: caged".as_ptr();
// pub(crate) const PHUSTR_5: *const c_char = c"level 5: ghost town".as_ptr();
// pub(crate) const PHUSTR_6: *const c_char = c"level 6: baron's lair".as_ptr();
// pub(crate) const PHUSTR_7: *const c_char = c"level 7: caughtyard".as_ptr();
// pub(crate) const PHUSTR_8: *const c_char = c"level 8: realm".as_ptr();
// pub(crate) const PHUSTR_9: *const c_char = c"level 9: abattoire".as_ptr();
// pub(crate) const PHUSTR_10: *const c_char = c"level 10: onslaught".as_ptr();
// pub(crate) const PHUSTR_11: *const c_char = c"level 11: hunted".as_ptr();

// pub(crate) const PHUSTR_12: *const c_char = c"level 12: speed".as_ptr();
// pub(crate) const PHUSTR_13: *const c_char = c"level 13: the crypt".as_ptr();
// pub(crate) const PHUSTR_14: *const c_char = c"level 14: genesis".as_ptr();
// pub(crate) const PHUSTR_15: *const c_char = c"level 15: the twilight".as_ptr();
// pub(crate) const PHUSTR_16: *const c_char = c"level 16: the omen".as_ptr();
// pub(crate) const PHUSTR_17: *const c_char = c"level 17: compound".as_ptr();
// pub(crate) const PHUSTR_18: *const c_char = c"level 18: neurosphere".as_ptr();
// pub(crate) const PHUSTR_19: *const c_char = c"level 19: nme".as_ptr();
// pub(crate) const PHUSTR_20: *const c_char = c"level 20: the death domain".as_ptr();

// pub(crate) const PHUSTR_21: *const c_char = c"level 21: slayer".as_ptr();
// pub(crate) const PHUSTR_22: *const c_char = c"level 22: impossible mission".as_ptr();
// pub(crate) const PHUSTR_23: *const c_char = c"level 23: tombstone".as_ptr();
// pub(crate) const PHUSTR_24: *const c_char = c"level 24: the final frontier".as_ptr();
// pub(crate) const PHUSTR_25: *const c_char = c"level 25: the temple of darkness".as_ptr();
// pub(crate) const PHUSTR_26: *const c_char = c"level 26: bunker".as_ptr();
// pub(crate) const PHUSTR_27: *const c_char = c"level 27: anti-christ".as_ptr();
// pub(crate) const PHUSTR_28: *const c_char = c"level 28: the sewers".as_ptr();
// pub(crate) const PHUSTR_29: *const c_char = c"level 29: odyssey of noises".as_ptr();
// pub(crate) const PHUSTR_30: *const c_char = c"level 30: the gateway of hell".as_ptr();

// pub(crate) const PHUSTR_31: *const c_char = c"level 31: cyberden".as_ptr();
// pub(crate) const PHUSTR_32: *const c_char = c"level 32: go 2 it".as_ptr();

// pub(crate) const THUSTR_1: *const c_char = c"level 1: system control".as_ptr();
// pub(crate) const THUSTR_2: *const c_char = c"level 2: human bbq".as_ptr();
// pub(crate) const THUSTR_3: *const c_char = c"level 3: power control".as_ptr();
// pub(crate) const THUSTR_4: *const c_char = c"level 4: wormhole".as_ptr();
// pub(crate) const THUSTR_5: *const c_char = c"level 5: hanger".as_ptr();
// pub(crate) const THUSTR_6: *const c_char = c"level 6: open season".as_ptr();
// pub(crate) const THUSTR_7: *const c_char = c"level 7: prison".as_ptr();
// pub(crate) const THUSTR_8: *const c_char = c"level 8: metal".as_ptr();
// pub(crate) const THUSTR_9: *const c_char = c"level 9: stronghold".as_ptr();
// pub(crate) const THUSTR_10: *const c_char = c"level 10: redemption".as_ptr();
// pub(crate) const THUSTR_11: *const c_char = c"level 11: storage facility".as_ptr();

// pub(crate) const THUSTR_12: *const c_char = c"level 12: crater".as_ptr();
// pub(crate) const THUSTR_13: *const c_char = c"level 13: nukage processing".as_ptr();
// pub(crate) const THUSTR_14: *const c_char = c"level 14: steel works".as_ptr();
// pub(crate) const THUSTR_15: *const c_char = c"level 15: dead zone".as_ptr();
// pub(crate) const THUSTR_16: *const c_char = c"level 16: deepest reaches".as_ptr();
// pub(crate) const THUSTR_17: *const c_char = c"level 17: processing area".as_ptr();
// pub(crate) const THUSTR_18: *const c_char = c"level 18: mill".as_ptr();
// pub(crate) const THUSTR_19: *const c_char = c"level 19: shipping/respawning".as_ptr();
// pub(crate) const THUSTR_20: *const c_char = c"level 20: central processing".as_ptr();

// pub(crate) const THUSTR_21: *const c_char = c"level 21: administration center".as_ptr();
// pub(crate) const THUSTR_22: *const c_char = c"level 22: habitat".as_ptr();
// pub(crate) const THUSTR_23: *const c_char = c"level 23: lunar mining project".as_ptr();
// pub(crate) const THUSTR_24: *const c_char = c"level 24: quarry".as_ptr();
// pub(crate) const THUSTR_25: *const c_char = c"level 25: baron's den".as_ptr();
// pub(crate) const THUSTR_26: *const c_char = c"level 26: ballistyx".as_ptr();
// pub(crate) const THUSTR_27: *const c_char = c"level 27: mount pain".as_ptr();
// pub(crate) const THUSTR_28: *const c_char = c"level 28: heck".as_ptr();
// pub(crate) const THUSTR_29: *const c_char = c"level 29: river styx".as_ptr();
// pub(crate) const THUSTR_30: *const c_char = c"level 30: last call".as_ptr();

// pub(crate) const THUSTR_31: *const c_char = c"level 31: pharaoh".as_ptr();
// pub(crate) const THUSTR_32: *const c_char = c"level 32: caribbean".as_ptr();

pub(crate) const HUSTR_CHATMACRO1: Smuggle<c_char> = Smuggle(c"I'm ready to kick butt!".as_ptr());
pub(crate) const HUSTR_CHATMACRO2: Smuggle<c_char> = Smuggle(c"I'm OK.".as_ptr());
pub(crate) const HUSTR_CHATMACRO3: Smuggle<c_char> = Smuggle(c"I'm not looking too good!".as_ptr());
pub(crate) const HUSTR_CHATMACRO4: Smuggle<c_char> = Smuggle(c"Help!".as_ptr());
pub(crate) const HUSTR_CHATMACRO5: Smuggle<c_char> = Smuggle(c"You suck!".as_ptr());
pub(crate) const HUSTR_CHATMACRO6: Smuggle<c_char> = Smuggle(c"Next time, scumbag...".as_ptr());
pub(crate) const HUSTR_CHATMACRO7: Smuggle<c_char> = Smuggle(c"Come here!".as_ptr());
pub(crate) const HUSTR_CHATMACRO8: Smuggle<c_char> = Smuggle(c"I'll take care of it.".as_ptr());
pub(crate) const HUSTR_CHATMACRO9: Smuggle<c_char> = Smuggle(c"Yes".as_ptr());
pub(crate) const HUSTR_CHATMACRO0: Smuggle<c_char> = Smuggle(c"No".as_ptr());

pub(crate) const HUSTR_TALKTOSELF1: *const c_char = c"You mumble to yourself".as_ptr();
pub(crate) const HUSTR_TALKTOSELF2: *const c_char = c"Who's there?".as_ptr();
pub(crate) const HUSTR_TALKTOSELF3: *const c_char = c"You scare yourself".as_ptr();
pub(crate) const HUSTR_TALKTOSELF4: *const c_char = c"You start to rave".as_ptr();
pub(crate) const HUSTR_TALKTOSELF5: *const c_char = c"You've lost it...".as_ptr();

// pub(crate) const HUSTR_MESSAGESENT: *const c_char = c"[Message Sent]".as_ptr();

// The following should NOT be changed unless it seems
// just AWFULLY necessary

pub(crate) const HUSTR_PLRGREEN: *const c_char = c"Green: ".as_ptr();
pub(crate) const HUSTR_PLRINDIGO: *const c_char = c"Indigo: ".as_ptr();
pub(crate) const HUSTR_PLRBROWN: *const c_char = c"Brown: ".as_ptr();
pub(crate) const HUSTR_PLRRED: *const c_char = c"Red: ".as_ptr();

pub(crate) const HUSTR_KEYGREEN: c_char = b'g' as c_char;
pub(crate) const HUSTR_KEYINDIGO: c_char = b'i' as c_char;
pub(crate) const HUSTR_KEYBROWN: c_char = b'b' as c_char;
pub(crate) const HUSTR_KEYRED: c_char = b'r' as c_char;

//	AM_map.C

pub(crate) const AMSTR_FOLLOWON: *const c_char = c"Follow Mode ON".as_ptr();
pub(crate) const AMSTR_FOLLOWOFF: *const c_char = c"Follow Mode OFF".as_ptr();

pub(crate) const AMSTR_GRIDON: *const c_char = c"Grid ON".as_ptr();
pub(crate) const AMSTR_GRIDOFF: *const c_char = c"Grid OFF".as_ptr();

pub(crate) const AMSTR_MARKEDSPOT: *const c_char = c"Marked Spot".as_ptr();
pub(crate) const AMSTR_MARKSCLEARED: *const c_char = c"All Marks Cleared".as_ptr();

//	ST_stuff.C

pub(crate) const STSTR_MUS: *const c_char = c"Music Change".as_ptr();
pub(crate) const STSTR_NOMUS: *const c_char = c"IMPOSSIBLE SELECTION".as_ptr();
pub(crate) const STSTR_DQDON: *const c_char = c"Degreelessness Mode On".as_ptr();
pub(crate) const STSTR_DQDOFF: *const c_char = c"Degreelessness Mode Off".as_ptr();

pub(crate) const STSTR_KFAADDED: *const c_char = c"Very Happy Ammo Added".as_ptr();
pub(crate) const STSTR_FAADDED: *const c_char = c"Ammo (no keys) Added".as_ptr();

pub(crate) const STSTR_NCON: *const c_char = c"No Clipping Mode ON".as_ptr();
pub(crate) const STSTR_NCOFF: *const c_char = c"No Clipping Mode OFF".as_ptr();

pub(crate) const STSTR_BEHOLD: *const c_char =
	c"inVuln, Str, Inviso, Rad, Allmap, or Lite-amp".as_ptr();
pub(crate) const STSTR_BEHOLDX: *const c_char = c"Power-up Toggled".as_ptr();

pub(crate) const STSTR_CHOPPERS: *const c_char = c"... doesn't suck - GM".as_ptr();
pub(crate) const STSTR_CLEV: *const c_char = c"Changing Level...".as_ptr();

//	F_Finale.C
pub(crate) const E1TEXT: *const c_char = c"Once you beat the big badasses and
clean out the moon base you're supposed
to win, aren't you? Aren't you? Where's
your fat reward and ticket home? What
the hell is this? It's not supposed to
end this way!

It stinks like rotten meat, but looks
like the lost Deimos base.  Looks like
you're stuck on The Shores of Hell.
The only way out is through.

To continue the DOOM experience, play
The Shores of Hell and its amazing
sequel, Inferno!\n"
	.as_ptr();

pub(crate) const E2TEXT: *const c_char = c"You've done it! The hideous cyber-
demon lord that ruled the lost Deimos
moon base has been slain and you
are triumphant! But ... where are
you? You clamber to the edge of the
moon and look down to see the awful
truth.

Deimos floats above Hell itself!
You've never heard of anyone escaping
from Hell, but you'll make the bastards
sorry they ever heard of you! Quickly,
you rappel down to  the surface of
Hell.

Now, it's on to the final chapter of
DOOM! -- Inferno."
	.as_ptr();

pub(crate) const E3TEXT: *const c_char = c"The loathsome spiderdemon that
masterminded the invasion of the moon
bases and caused so much death has had
its ass kicked for all time.

A hidden doorway opens and you enter.
You've proven too tough for Hell to
contain, and now Hell at last plays
fair -- for you emerge from the door
to see the green fields of Earth!
Home at last.

You wonder what's been happening on
Earth while you were battling evil
unleashed. It's good that no Hell-
spawn could have come through that
door with you ..."
	.as_ptr();

pub(crate) const E4TEXT: *const c_char = c"the spider mastermind must have sent forth
its legions of hellspawn before your
final confrontation with that terrible
beast from hell.  but you stepped forward
and brought forth eternal damnation and
suffering upon the horde as a true hero
would in the face of something so evil.

besides, someone was gonna pay for what
happened to daisy, your pet rabbit.

but now, you see spread before you more
potential pain and gibbitude as a nation
of demons run amok among our cities.

next stop, hell on earth!"
	.as_ptr();

// after level 6, put this:

pub(crate) const C1TEXT: *const c_char = c"YOU HAVE ENTERED DEEPLY INTO THE INFESTED
STARPORT. BUT SOMETHING IS WRONG. THE
MONSTERS HAVE BROUGHT THEIR OWN REALITY
WITH THEM, AND THE STARPORT'S TECHNOLOGY
IS BEING SUBVERTED BY THEIR PRESENCE.

AHEAD, YOU SEE AN OUTPOST OF HELL, A
FORTIFIED ZONE. IF YOU CAN GET PAST IT,
YOU CAN PENETRATE INTO THE HAUNTED HEART
OF THE STARBASE AND FIND THE CONTROLLING
SWITCH WHICH HOLDS EARTH'S POPULATION
HOSTAGE."
	.as_ptr();

// After level 11, put this:

pub(crate) const C2TEXT: *const c_char = c"YOU HAVE WON! YOUR VICTORY HAS ENABLED
HUMANKIND TO EVACUATE EARTH AND ESCAPE
THE NIGHTMARE.  NOW YOU ARE THE ONLY
HUMAN LEFT ON THE FACE OF THE PLANET.
CANNIBAL MUTATIONS, CARNIVOROUS ALIENS,
AND EVIL SPIRITS ARE YOUR ONLY NEIGHBORS.
YOU SIT BACK AND WAIT FOR DEATH, CONTENT
THAT YOU HAVE SAVED YOUR SPECIES.

BUT THEN, EARTH CONTROL BEAMS DOWN A
MESSAGE FROM SPACE: \"SENSORS HAVE LOCATED
THE SOURCE OF THE ALIEN INVASION. IF YOU
GO THERE, YOU MAY BE ABLE TO BLOCK THEIR
ENTRY.  THE ALIEN BASE IS IN THE HEART OF
YOUR OWN HOME CITY, NOT FAR FROM THE
STARPORT.\" SLOWLY AND PAINFULLY YOU GET
UP AND RETURN TO THE FRAY."
	.as_ptr();

// After level 20, put this:

pub(crate) const C3TEXT: *const c_char = c"YOU ARE AT THE CORRUPT HEART OF THE CITY,
SURROUNDED BY THE CORPSES OF YOUR ENEMIES.
YOU SEE NO WAY TO DESTROY THE CREATURES'
ENTRYWAY ON THIS SIDE, SO YOU CLENCH YOUR
TEETH AND PLUNGE THROUGH IT.

THERE MUST BE A WAY TO CLOSE IT ON THE
OTHER SIDE. WHAT DO YOU CARE IF YOU'VE
GOT TO GO THROUGH HELL TO GET TO IT?"
	.as_ptr();

// After level 29, put this:

pub(crate) const C4TEXT: *const c_char = c"THE HORRENDOUS VISAGE OF THE BIGGEST
DEMON YOU'VE EVER SEEN CRUMBLES BEFORE
YOU, AFTER YOU PUMP YOUR ROCKETS INTO
HIS EXPOSED BRAIN. THE MONSTER SHRIVELS
UP AND DIES, ITS THRASHING LIMBS
DEVASTATING UNTOLD MILES OF HELL'S
SURFACE.

YOU'VE DONE IT. THE INVASION IS OVER.
EARTH IS SAVED. HELL IS A WRECK. YOU
WONDER WHERE BAD FOLKS WILL GO WHEN THEY
DIE, NOW. WIPING THE SWEAT FROM YOUR
FOREHEAD YOU BEGIN THE LONG TREK BACK
HOME. REBUILDING EARTH OUGHT TO BE A
LOT MORE FUN THAN RUINING IT WAS.\n"
	.as_ptr();

// Before level 31, put this:

pub(crate) const C5TEXT: *const c_char = c"CONGRATULATIONS, YOU'VE FOUND THE SECRET
LEVEL! LOOKS LIKE IT'S BEEN BUILT BY
HUMANS, RATHER THAN DEMONS. YOU WONDER
WHO THE INMATES OF THIS CORNER OF HELL
WILL BE."
	.as_ptr();

// Before level 32, put this:

pub(crate) const C6TEXT: *const c_char = c"CONGRATULATIONS, YOU'VE FOUND THE
SUPER SECRET LEVEL!  YOU'D BETTER
BLAZE THROUGH THIS ONE!\n"
	.as_ptr();

// after map 06

// pub(crate) const P1TEXT: *const c_char = c"You gloat over the steaming carcass of the
// Guardian.  With its death, you've wrested
// the Accelerator from the stinking claws
// of Hell.  You relax and glance around the
// room.  Damn!  There was supposed to be at
// least one working prototype, but you can't
// see it. The demons must have taken it.
//
// You must find the prototype, or all your
// struggles will have been wasted. Keep
// moving, keep fighting, keep killing.
// Oh yes, keep living, too."
// 	.as_ptr();
//
// // after map 11
//
// pub(crate) const P2TEXT: *const c_char = c"Even the deadly Arch-Vile labyrinth could
// not stop you, and you've gotten to the
// prototype Accelerator which is soon
// efficiently and permanently deactivated.
//
// You're good at that kind of thing."
// 	.as_ptr();
//
// // after map 20
//
// pub(crate) const P3TEXT: *const c_char = c"You've bashed and battered your way into
// the heart of the devil-hive.  Time for a
// Search-and-Destroy mission, aimed at the
// Gatekeeper, whose foul offspring is
// cascading to Earth.  Yeah, he's bad. But
// you know who's worse!
//
// Grinning evilly, you check your gear, and
// get ready to give the bastard a little Hell
// of your own making!"
// 	.as_ptr();
//
// // after map 30
//
// pub(crate) const P4TEXT: *const c_char = c"The Gatekeeper's evil face is splattered
// all over the place.  As its tattered corpse
// collapses, an inverted Gate forms and
// sucks down the shards of the last
// prototype Accelerator, not to mention the
// few remaining demons.  You're done. Hell
// has gone back to pounding bad dead folks
// instead of good live ones.  Remember to
// tell your grandkids to put a rocket
// launcher in your coffin. If you go to Hell
// when you die, you'll need it for some
// final cleaning-up ..."
// 	.as_ptr();
//
// // before map 31
//
// pub(crate) const P5TEXT: *const c_char = c"You've found the second-hardest level we
// got. Hope you have a saved game a level or
// two previous.  If not, be prepared to die
// aplenty. For master marines only."
// 	.as_ptr();
//
// // before map 32
//
// pub(crate) const P6TEXT: *const c_char = c"Betcha wondered just what WAS the hardest
// level we had ready for ya?  Now you know.
// No one gets out alive."
// 	.as_ptr();
//
// pub(crate) const T1TEXT: *const c_char = c"You've fought your way out of the infested
// experimental labs.   It seems that UAC has
// once again gulped it down.  With their
// high turnover, it must be hard for poor
// old UAC to buy corporate health insurance
// nowadays..
//
// Ahead lies the military complex, now
// swarming with diseased horrors hot to get
// their teeth into you. With luck, the
// complex still has some warlike ordnance
// laying around."
// 	.as_ptr();
//
// pub(crate) const T2TEXT: *const c_char = c"You hear the grinding of heavy machinery
// ahead.  You sure hope they're not stamping
// out new hellspawn, but you're ready to
// ream out a whole herd if you have to.
// They might be planning a blood feast, but
// you feel about as mean as two thousand
// maniacs packed into one mad killer.
//
// You don't plan to go down easy."
// 	.as_ptr();
//
// pub(crate) const T3TEXT: *const c_char = c"The vista opening ahead looks real damn
// familiar. Smells familiar, too -- like
// fried excrement. You didn't like this
// place before, and you sure as hell ain't
// planning to like it now. The more you
// brood on it, the madder you get.
// Hefting your gun, an evil grin trickles
// onto your face. Time to take some names."
// 	.as_ptr();
//
// pub(crate) const T4TEXT: *const c_char = c"Suddenly, all is silent, from one horizon
// to the other. The agonizing echo of Hell
// fades away, the nightmare sky turns to
// blue, the heaps of monster corpses start
// to evaporate along with the evil stench
// that filled the air. Jeeze, maybe you've
// done it. Have you really won?
//
// Something rumbles in the distance.
// A blue light begins to glow inside the
// ruined skull of the demon-spitter."
// 	.as_ptr();
//
// pub(crate) const T5TEXT: *const c_char = c"What now? Looks totally different. Kind
// of like King Tut's condo. Well,
// whatever's here can't be any worse
// than usual. Can it?  Or maybe it's best
// to let sleeping gods lie.."
// 	.as_ptr();
//
// pub(crate) const T6TEXT: *const c_char = c"Time for a vacation. You've burst the
// bowels of hell and by golly you're ready
// for a break. You mutter to yourself,
// Maybe someone else can kick Hell's ass
// next time around. Ahead lies a quiet town,
// with peaceful flowing water, quaint
// buildings, and presumably no Hellspawn.
//
// As you step off the transport, you hear
// the stomp of a cyberdemon's iron shoe."
// 	.as_ptr();

// Character cast strings F_FINALE.C
pub(crate) const CC_ZOMBIE: *const c_char = c"ZOMBIEMAN".as_ptr();
pub(crate) const CC_SHOTGUN: *const c_char = c"SHOTGUN GUY".as_ptr();
pub(crate) const CC_HEAVY: *const c_char = c"HEAVY WEAPON DUDE".as_ptr();
pub(crate) const CC_IMP: *const c_char = c"IMP".as_ptr();
pub(crate) const CC_DEMON: *const c_char = c"DEMON".as_ptr();
pub(crate) const CC_LOST: *const c_char = c"LOST SOUL".as_ptr();
pub(crate) const CC_CACO: *const c_char = c"CACODEMON".as_ptr();
pub(crate) const CC_HELL: *const c_char = c"HELL KNIGHT".as_ptr();
pub(crate) const CC_BARON: *const c_char = c"BARON OF HELL".as_ptr();
pub(crate) const CC_ARACH: *const c_char = c"ARACHNOTRON".as_ptr();
pub(crate) const CC_PAIN: *const c_char = c"PAIN ELEMENTAL".as_ptr();
pub(crate) const CC_REVEN: *const c_char = c"REVENANT".as_ptr();
pub(crate) const CC_MANCU: *const c_char = c"MANCUBUS".as_ptr();
pub(crate) const CC_ARCH: *const c_char = c"ARCH-VILE".as_ptr();
pub(crate) const CC_SPIDER: *const c_char = c"THE SPIDER MASTERMIND".as_ptr();
pub(crate) const CC_CYBER: *const c_char = c"THE CYBERDEMON".as_ptr();
pub(crate) const CC_HERO: *const c_char = c"OUR HERO".as_ptr();
