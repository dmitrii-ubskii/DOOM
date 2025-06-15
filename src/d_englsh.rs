use std::ffi::{CStr, c_char};

use crate::dstrings::Smuggle;

//	Printed strings for translation

// D_Main.C
pub const D_DEVSTR: &CStr = c"Development mode ON.\n";
pub const D_CDROM: &CStr = c"CD-ROM Version: default.cfg from c:\\doomdata\n";

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
pub const LOADNET: *const c_char =
	concat!("you can't do load while in a net game!\n\n", PRESSKEY!()).as_ptr().cast();
pub const QLOADNET: *const c_char =
	concat!("you can't quickload during a netgame!\n\n", PRESSKEY!()).as_ptr().cast();
pub const QSAVESPOT: *const c_char =
	concat!("you haven't picked a quicksave slot yet!\n\n", PRESSKEY!()).as_ptr().cast();
pub const SAVEDEAD: *const c_char =
	concat!("you can't save if you aren't playing!\n\n", PRESSKEY!()).as_ptr().cast();
pub const QSPROMPT: *const c_char =
	concat!("quicksave over your game named\n\n'%s'?\n\n", PRESSYN!()).as_ptr().cast();
pub const QLPROMPT: *const c_char =
	concat!("do you want to quickload the game named\n\n'%s'?\n\n", PRESSYN!()).as_ptr().cast();

pub const NEWGAME: *const c_char =
	concat!("you can't start a new game\nwhile in a network game.\n\n", PRESSKEY!())
		.as_ptr()
		.cast();

pub const NIGHTMARE: *const c_char =
	concat!("are you sure? this skill level\nisn't even remotely fair.\n\n", PRESSYN!())
		.as_ptr()
		.cast();

pub const SWSTRING: *const c_char = concat!(
	"this is the shareware version of doom.\n\nyou need to order the entire trilogy.\n\n",
	PRESSKEY!()
)
.as_ptr()
.cast();

pub const MSGOFF: *const c_char = c"Messages OFF".as_ptr();
pub const MSGON: *const c_char = c"Messages ON".as_ptr();
pub const NETEND: *const c_char =
	concat!("you can't end a netgame!\n\n", PRESSKEY!()).as_ptr().cast();
pub const ENDGAME: *const c_char =
	concat!("are you sure you want to end the game?\n\n", PRESSYN!()).as_ptr().cast();

macro_rules! DOSY {
	() => {
		"(press y to quit)\0"
	};
}
pub(crate) use DOSY;

pub const DETAILHI: *const c_char = c"High detail".as_ptr();
pub const DETAILLO: *const c_char = c"Low detail".as_ptr();
pub const GAMMALVL0: [u8; 26] = *b"Gamma correction OFF\0\0\0\0\0\0";
pub const GAMMALVL1: [u8; 26] = *b"Gamma correction level 1\0\0";
pub const GAMMALVL2: [u8; 26] = *b"Gamma correction level 2\0\0";
pub const GAMMALVL3: [u8; 26] = *b"Gamma correction level 3\0\0";
pub const GAMMALVL4: [u8; 26] = *b"Gamma correction level 4\0\0";
pub const EMPTYSTRING: *const c_char = c"empty slot".as_ptr();

//	P_inter.C
pub const GOTARMOR: *const c_char = c"Picked up the armor.".as_ptr();
pub const GOTMEGA: *const c_char = c"Picked up the MegaArmor!".as_ptr();
pub const GOTHTHBONUS: *const c_char = c"Picked up a health bonus.".as_ptr();
pub const GOTARMBONUS: *const c_char = c"Picked up an armor bonus.".as_ptr();
pub const GOTSTIM: *const c_char = c"Picked up a stimpack.".as_ptr();
pub const GOTMEDINEED: *const c_char = c"Picked up a medikit that you REALLY need!".as_ptr();
pub const GOTMEDIKIT: *const c_char = c"Picked up a medikit.".as_ptr();
pub const GOTSUPER: *const c_char = c"Supercharge!".as_ptr();

pub const GOTBLUECARD: *const c_char = c"Picked up a blue keycard.".as_ptr();
pub const GOTYELWCARD: *const c_char = c"Picked up a yellow keycard.".as_ptr();
pub const GOTREDCARD: *const c_char = c"Picked up a red keycard.".as_ptr();
pub const GOTBLUESKUL: *const c_char = c"Picked up a blue skull key.".as_ptr();
pub const GOTYELWSKUL: *const c_char = c"Picked up a yellow skull key.".as_ptr();
pub const GOTREDSKULL: *const c_char = c"Picked up a red skull key.".as_ptr();

pub const GOTINVUL: *const c_char = c"Invulnerability!".as_ptr();
pub const GOTBERSERK: *const c_char = c"Berserk!".as_ptr();
pub const GOTINVIS: *const c_char = c"Partial Invisibility".as_ptr();
pub const GOTSUIT: *const c_char = c"Radiation Shielding Suit".as_ptr();
pub const GOTMAP: *const c_char = c"Computer Area Map".as_ptr();
pub const GOTVISOR: *const c_char = c"Light Amplification Visor".as_ptr();
pub const GOTMSPHERE: *const c_char = c"MegaSphere!".as_ptr();

pub const GOTCLIP: *const c_char = c"Picked up a clip.".as_ptr();
pub const GOTCLIPBOX: *const c_char = c"Picked up a box of bullets.".as_ptr();
pub const GOTROCKET: *const c_char = c"Picked up a rocket.".as_ptr();
pub const GOTROCKBOX: *const c_char = c"Picked up a box of rockets.".as_ptr();
pub const GOTCELL: *const c_char = c"Picked up an energy cell.".as_ptr();
pub const GOTCELLBOX: *const c_char = c"Picked up an energy cell pack.".as_ptr();
pub const GOTSHELLS: *const c_char = c"Picked up 4 shotgun shells.".as_ptr();
pub const GOTSHELLBOX: *const c_char = c"Picked up a box of shotgun shells.".as_ptr();
pub const GOTBACKPACK: *const c_char = c"Picked up a backpack full of ammo!".as_ptr();

pub const GOTBFG9000: *const c_char = c"You got the BFG9000!  Oh, yes.".as_ptr();
pub const GOTCHAINGUN: *const c_char = c"You got the chaingun!".as_ptr();
pub const GOTCHAINSAW: *const c_char = c"A chainsaw!  Find some meat!".as_ptr();
pub const GOTLAUNCHER: *const c_char = c"You got the rocket launcher!".as_ptr();
pub const GOTPLASMA: *const c_char = c"You got the plasma gun!".as_ptr();
pub const GOTSHOTGUN: *const c_char = c"You got the shotgun!".as_ptr();
pub const GOTSHOTGUN2: *const c_char = c"You got the super shotgun!".as_ptr();

// P_Doors.C
pub const PD_BLUEO: *const c_char = c"You need a blue key to activate this object".as_ptr();
pub const PD_REDO: *const c_char = c"You need a red key to activate this object".as_ptr();
pub const PD_YELLOWO: *const c_char = c"You need a yellow key to activate this object".as_ptr();
pub const PD_BLUEK: *const c_char = c"You need a blue key to open this door".as_ptr();
pub const PD_REDK: *const c_char = c"You need a red key to open this door".as_ptr();
pub const PD_YELLOWK: *const c_char = c"You need a yellow key to open this door".as_ptr();

//	G_game.C
pub const GGSAVED: *const c_char = c"game saved.".as_ptr();

//	HU_stuff.C
pub const HUSTR_MSGU: *const c_char = c"[Message unsent]".as_ptr();

pub const HUSTR_E1M1: *const c_char = c"E1M1: Hangar".as_ptr();
pub const HUSTR_E1M2: *const c_char = c"E1M2: Nuclear Plant".as_ptr();
pub const HUSTR_E1M3: *const c_char = c"E1M3: Toxin Refinery".as_ptr();
pub const HUSTR_E1M4: *const c_char = c"E1M4: Command Control".as_ptr();
pub const HUSTR_E1M5: *const c_char = c"E1M5: Phobos Lab".as_ptr();
pub const HUSTR_E1M6: *const c_char = c"E1M6: Central Processing".as_ptr();
pub const HUSTR_E1M7: *const c_char = c"E1M7: Computer Station".as_ptr();
pub const HUSTR_E1M8: *const c_char = c"E1M8: Phobos Anomaly".as_ptr();
pub const HUSTR_E1M9: *const c_char = c"E1M9: Military Base".as_ptr();

pub const HUSTR_E2M1: *const c_char = c"E2M1: Deimos Anomaly".as_ptr();
pub const HUSTR_E2M2: *const c_char = c"E2M2: Containment Area".as_ptr();
pub const HUSTR_E2M3: *const c_char = c"E2M3: Refinery".as_ptr();
pub const HUSTR_E2M4: *const c_char = c"E2M4: Deimos Lab".as_ptr();
pub const HUSTR_E2M5: *const c_char = c"E2M5: Command Center".as_ptr();
pub const HUSTR_E2M6: *const c_char = c"E2M6: Halls of the Damned".as_ptr();
pub const HUSTR_E2M7: *const c_char = c"E2M7: Spawning Vats".as_ptr();
pub const HUSTR_E2M8: *const c_char = c"E2M8: Tower of Babel".as_ptr();
pub const HUSTR_E2M9: *const c_char = c"E2M9: Fortress of Mystery".as_ptr();

pub const HUSTR_E3M1: *const c_char = c"E3M1: Hell Keep".as_ptr();
pub const HUSTR_E3M2: *const c_char = c"E3M2: Slough of Despair".as_ptr();
pub const HUSTR_E3M3: *const c_char = c"E3M3: Pandemonium".as_ptr();
pub const HUSTR_E3M4: *const c_char = c"E3M4: House of Pain".as_ptr();
pub const HUSTR_E3M5: *const c_char = c"E3M5: Unholy Cathedral".as_ptr();
pub const HUSTR_E3M6: *const c_char = c"E3M6: Mt. Erebus".as_ptr();
pub const HUSTR_E3M7: *const c_char = c"E3M7: Limbo".as_ptr();
pub const HUSTR_E3M8: *const c_char = c"E3M8: Dis".as_ptr();
pub const HUSTR_E3M9: *const c_char = c"E3M9: Warrens".as_ptr();

pub const HUSTR_E4M1: *const c_char = c"E4M1: Hell Beneath".as_ptr();
pub const HUSTR_E4M2: *const c_char = c"E4M2: Perfect Hatred".as_ptr();
pub const HUSTR_E4M3: *const c_char = c"E4M3: Sever The Wicked".as_ptr();
pub const HUSTR_E4M4: *const c_char = c"E4M4: Unruly Evil".as_ptr();
pub const HUSTR_E4M5: *const c_char = c"E4M5: They Will Repent".as_ptr();
pub const HUSTR_E4M6: *const c_char = c"E4M6: Against Thee Wickedly".as_ptr();
pub const HUSTR_E4M7: *const c_char = c"E4M7: And Hell Followed".as_ptr();
pub const HUSTR_E4M8: *const c_char = c"E4M8: Unto The Cruel".as_ptr();
pub const HUSTR_E4M9: *const c_char = c"E4M9: Fear".as_ptr();

pub const HUSTR_1: *const c_char = c"level 1: entryway".as_ptr();
pub const HUSTR_2: *const c_char = c"level 2: underhalls".as_ptr();
pub const HUSTR_3: *const c_char = c"level 3: the gantlet".as_ptr();
pub const HUSTR_4: *const c_char = c"level 4: the focus".as_ptr();
pub const HUSTR_5: *const c_char = c"level 5: the waste tunnels".as_ptr();
pub const HUSTR_6: *const c_char = c"level 6: the crusher".as_ptr();
pub const HUSTR_7: *const c_char = c"level 7: dead simple".as_ptr();
pub const HUSTR_8: *const c_char = c"level 8: tricks and traps".as_ptr();
pub const HUSTR_9: *const c_char = c"level 9: the pit".as_ptr();
pub const HUSTR_10: *const c_char = c"level 10: refueling base".as_ptr();
pub const HUSTR_11: *const c_char = c"level 11: 'o' of destruction!".as_ptr();

pub const HUSTR_12: *const c_char = c"level 12: the factory".as_ptr();
pub const HUSTR_13: *const c_char = c"level 13: downtown".as_ptr();
pub const HUSTR_14: *const c_char = c"level 14: the inmost dens".as_ptr();
pub const HUSTR_15: *const c_char = c"level 15: industrial zone".as_ptr();
pub const HUSTR_16: *const c_char = c"level 16: suburbs".as_ptr();
pub const HUSTR_17: *const c_char = c"level 17: tenements".as_ptr();
pub const HUSTR_18: *const c_char = c"level 18: the courtyard".as_ptr();
pub const HUSTR_19: *const c_char = c"level 19: the citadel".as_ptr();
pub const HUSTR_20: *const c_char = c"level 20: gotcha!".as_ptr();

pub const HUSTR_21: *const c_char = c"level 21: nirvana".as_ptr();
pub const HUSTR_22: *const c_char = c"level 22: the catacombs".as_ptr();
pub const HUSTR_23: *const c_char = c"level 23: barrels o' fun".as_ptr();
pub const HUSTR_24: *const c_char = c"level 24: the chasm".as_ptr();
pub const HUSTR_25: *const c_char = c"level 25: bloodfalls".as_ptr();
pub const HUSTR_26: *const c_char = c"level 26: the abandoned mines".as_ptr();
pub const HUSTR_27: *const c_char = c"level 27: monster condo".as_ptr();
pub const HUSTR_28: *const c_char = c"level 28: the spirit world".as_ptr();
pub const HUSTR_29: *const c_char = c"level 29: the living end".as_ptr();
pub const HUSTR_30: *const c_char = c"level 30: icon of sin".as_ptr();

pub const HUSTR_31: *const c_char = c"level 31: wolfenstein".as_ptr();
pub const HUSTR_32: *const c_char = c"level 32: grosse".as_ptr();

pub const PHUSTR_1: *const c_char = c"level 1: congo".as_ptr();
pub const PHUSTR_2: *const c_char = c"level 2: well of souls".as_ptr();
pub const PHUSTR_3: *const c_char = c"level 3: aztec".as_ptr();
pub const PHUSTR_4: *const c_char = c"level 4: caged".as_ptr();
pub const PHUSTR_5: *const c_char = c"level 5: ghost town".as_ptr();
pub const PHUSTR_6: *const c_char = c"level 6: baron's lair".as_ptr();
pub const PHUSTR_7: *const c_char = c"level 7: caughtyard".as_ptr();
pub const PHUSTR_8: *const c_char = c"level 8: realm".as_ptr();
pub const PHUSTR_9: *const c_char = c"level 9: abattoire".as_ptr();
pub const PHUSTR_10: *const c_char = c"level 10: onslaught".as_ptr();
pub const PHUSTR_11: *const c_char = c"level 11: hunted".as_ptr();

pub const PHUSTR_12: *const c_char = c"level 12: speed".as_ptr();
pub const PHUSTR_13: *const c_char = c"level 13: the crypt".as_ptr();
pub const PHUSTR_14: *const c_char = c"level 14: genesis".as_ptr();
pub const PHUSTR_15: *const c_char = c"level 15: the twilight".as_ptr();
pub const PHUSTR_16: *const c_char = c"level 16: the omen".as_ptr();
pub const PHUSTR_17: *const c_char = c"level 17: compound".as_ptr();
pub const PHUSTR_18: *const c_char = c"level 18: neurosphere".as_ptr();
pub const PHUSTR_19: *const c_char = c"level 19: nme".as_ptr();
pub const PHUSTR_20: *const c_char = c"level 20: the death domain".as_ptr();

pub const PHUSTR_21: *const c_char = c"level 21: slayer".as_ptr();
pub const PHUSTR_22: *const c_char = c"level 22: impossible mission".as_ptr();
pub const PHUSTR_23: *const c_char = c"level 23: tombstone".as_ptr();
pub const PHUSTR_24: *const c_char = c"level 24: the final frontier".as_ptr();
pub const PHUSTR_25: *const c_char = c"level 25: the temple of darkness".as_ptr();
pub const PHUSTR_26: *const c_char = c"level 26: bunker".as_ptr();
pub const PHUSTR_27: *const c_char = c"level 27: anti-christ".as_ptr();
pub const PHUSTR_28: *const c_char = c"level 28: the sewers".as_ptr();
pub const PHUSTR_29: *const c_char = c"level 29: odyssey of noises".as_ptr();
pub const PHUSTR_30: *const c_char = c"level 30: the gateway of hell".as_ptr();

pub const PHUSTR_31: *const c_char = c"level 31: cyberden".as_ptr();
pub const PHUSTR_32: *const c_char = c"level 32: go 2 it".as_ptr();

pub const THUSTR_1: *const c_char = c"level 1: system control".as_ptr();
pub const THUSTR_2: *const c_char = c"level 2: human bbq".as_ptr();
pub const THUSTR_3: *const c_char = c"level 3: power control".as_ptr();
pub const THUSTR_4: *const c_char = c"level 4: wormhole".as_ptr();
pub const THUSTR_5: *const c_char = c"level 5: hanger".as_ptr();
pub const THUSTR_6: *const c_char = c"level 6: open season".as_ptr();
pub const THUSTR_7: *const c_char = c"level 7: prison".as_ptr();
pub const THUSTR_8: *const c_char = c"level 8: metal".as_ptr();
pub const THUSTR_9: *const c_char = c"level 9: stronghold".as_ptr();
pub const THUSTR_10: *const c_char = c"level 10: redemption".as_ptr();
pub const THUSTR_11: *const c_char = c"level 11: storage facility".as_ptr();

pub const THUSTR_12: *const c_char = c"level 12: crater".as_ptr();
pub const THUSTR_13: *const c_char = c"level 13: nukage processing".as_ptr();
pub const THUSTR_14: *const c_char = c"level 14: steel works".as_ptr();
pub const THUSTR_15: *const c_char = c"level 15: dead zone".as_ptr();
pub const THUSTR_16: *const c_char = c"level 16: deepest reaches".as_ptr();
pub const THUSTR_17: *const c_char = c"level 17: processing area".as_ptr();
pub const THUSTR_18: *const c_char = c"level 18: mill".as_ptr();
pub const THUSTR_19: *const c_char = c"level 19: shipping/respawning".as_ptr();
pub const THUSTR_20: *const c_char = c"level 20: central processing".as_ptr();

pub const THUSTR_21: *const c_char = c"level 21: administration center".as_ptr();
pub const THUSTR_22: *const c_char = c"level 22: habitat".as_ptr();
pub const THUSTR_23: *const c_char = c"level 23: lunar mining project".as_ptr();
pub const THUSTR_24: *const c_char = c"level 24: quarry".as_ptr();
pub const THUSTR_25: *const c_char = c"level 25: baron's den".as_ptr();
pub const THUSTR_26: *const c_char = c"level 26: ballistyx".as_ptr();
pub const THUSTR_27: *const c_char = c"level 27: mount pain".as_ptr();
pub const THUSTR_28: *const c_char = c"level 28: heck".as_ptr();
pub const THUSTR_29: *const c_char = c"level 29: river styx".as_ptr();
pub const THUSTR_30: *const c_char = c"level 30: last call".as_ptr();

pub const THUSTR_31: *const c_char = c"level 31: pharaoh".as_ptr();
pub const THUSTR_32: *const c_char = c"level 32: caribbean".as_ptr();

pub static HUSTR_CHATMACRO1: Smuggle<c_char> = Smuggle(c"I'm ready to kick butt!".as_ptr());
pub static HUSTR_CHATMACRO2: Smuggle<c_char> = Smuggle(c"I'm OK.".as_ptr());
pub static HUSTR_CHATMACRO3: Smuggle<c_char> = Smuggle(c"I'm not looking too good!".as_ptr());
pub static HUSTR_CHATMACRO4: Smuggle<c_char> = Smuggle(c"Help!".as_ptr());
pub static HUSTR_CHATMACRO5: Smuggle<c_char> = Smuggle(c"You suck!".as_ptr());
pub static HUSTR_CHATMACRO6: Smuggle<c_char> = Smuggle(c"Next time, scumbag...".as_ptr());
pub static HUSTR_CHATMACRO7: Smuggle<c_char> = Smuggle(c"Come here!".as_ptr());
pub static HUSTR_CHATMACRO8: Smuggle<c_char> = Smuggle(c"I'll take care of it.".as_ptr());
pub static HUSTR_CHATMACRO9: Smuggle<c_char> = Smuggle(c"Yes".as_ptr());
pub static HUSTR_CHATMACRO0: Smuggle<c_char> = Smuggle(c"No".as_ptr());

pub const HUSTR_TALKTOSELF1: *const c_char = c"You mumble to yourself".as_ptr();
pub const HUSTR_TALKTOSELF2: *const c_char = c"Who's there?".as_ptr();
pub const HUSTR_TALKTOSELF3: *const c_char = c"You scare yourself".as_ptr();
pub const HUSTR_TALKTOSELF4: *const c_char = c"You start to rave".as_ptr();
pub const HUSTR_TALKTOSELF5: *const c_char = c"You've lost it...".as_ptr();

pub const HUSTR_MESSAGESENT: *const c_char = c"[Message Sent]".as_ptr();

// The following should NOT be changed unless it seems
// just AWFULLY necessary

pub const HUSTR_PLRGREEN: *const c_char = c"Green: ".as_ptr();
pub const HUSTR_PLRINDIGO: *const c_char = c"Indigo: ".as_ptr();
pub const HUSTR_PLRBROWN: *const c_char = c"Brown: ".as_ptr();
pub const HUSTR_PLRRED: *const c_char = c"Red: ".as_ptr();

pub const HUSTR_KEYGREEN: c_char = b'g' as c_char;
pub const HUSTR_KEYINDIGO: c_char = b'i' as c_char;
pub const HUSTR_KEYBROWN: c_char = b'b' as c_char;
pub const HUSTR_KEYRED: c_char = b'r' as c_char;

//	AM_map.C

pub const AMSTR_FOLLOWON: *const c_char = c"Follow Mode ON".as_ptr();
pub const AMSTR_FOLLOWOFF: *const c_char = c"Follow Mode OFF".as_ptr();

pub const AMSTR_GRIDON: *const c_char = c"Grid ON".as_ptr();
pub const AMSTR_GRIDOFF: *const c_char = c"Grid OFF".as_ptr();

pub const AMSTR_MARKEDSPOT: *const c_char = c"Marked Spot".as_ptr();
pub const AMSTR_MARKSCLEARED: *const c_char = c"All Marks Cleared".as_ptr();

/*
//	ST_stuff.C
//

pub const STSTR_MUS: &CStr = c"Music Change";
pub const STSTR_NOMUS: &CStr = c"IMPOSSIBLE SELECTION";
pub const STSTR_DQDON: &CStr = c"Degreelessness Mode On";
pub const STSTR_DQDOFF: &CStr = c"Degreelessness Mode Off";

pub const STSTR_KFAADDED: &CStr = c"Very Happy Ammo Added";
pub const STSTR_FAADDED: &CStr = c"Ammo (no keys) Added";

pub const STSTR_NCON: &CStr = c"No Clipping Mode ON";
pub const STSTR_NCOFF: &CStr = c"No Clipping Mode OFF";

pub const STSTR_BEHOLD: &CStr = c"inVuln, Str, Inviso, Rad, Allmap, or Lite-amp";
pub const STSTR_BEHOLDX: &CStr = c"Power-up Toggled";

pub const STSTR_CHOPPERS: &CStr = c"... doesn't suck - GM";
pub const STSTR_CLEV: &CStr = c"Changing Level...";

//
//	F_Finale.C
//
#define E1TEXT \
"Once you beat the big badasses and\n"\
"clean out the moon base you're supposed\n"\
"to win, aren't you? Aren't you? Where's\n"\
"your fat reward and ticket home? What\n"\
"the hell is this? It's not supposed to\n"\
"end this way!\n"\
"\n" \
"It stinks like rotten meat, but looks\n"\
"like the lost Deimos base.  Looks like\n"\
"you're stuck on The Shores of Hell.\n"\
"The only way out is through.\n"\
"\n"\
"To continue the DOOM experience, play\n"\
"The Shores of Hell and its amazing\n"\
"sequel, Inferno!\n"


#define E2TEXT \
"You've done it! The hideous cyber-\n"\
"demon lord that ruled the lost Deimos\n"\
"moon base has been slain and you\n"\
"are triumphant! But ... where are\n"\
"you? You clamber to the edge of the\n"\
"moon and look down to see the awful\n"\
"truth.\n" \
"\n"\
"Deimos floats above Hell itself!\n"\
"You've never heard of anyone escaping\n"\
"from Hell, but you'll make the bastards\n"\
"sorry they ever heard of you! Quickly,\n"\
"you rappel down to  the surface of\n"\
"Hell.\n"\
"\n" \
"Now, it's on to the final chapter of\n"\
"DOOM! -- Inferno."


#define E3TEXT \
"The loathsome spiderdemon that\n"\
"masterminded the invasion of the moon\n"\
"bases and caused so much death has had\n"\
"its ass kicked for all time.\n"\
"\n"\
"A hidden doorway opens and you enter.\n"\
"You've proven too tough for Hell to\n"\
"contain, and now Hell at last plays\n"\
"fair -- for you emerge from the door\n"\
"to see the green fields of Earth!\n"\
"Home at last.\n" \
"\n"\
"You wonder what's been happening on\n"\
"Earth while you were battling evil\n"\
"unleashed. It's good that no Hell-\n"\
"spawn could have come through that\n"\
"door with you ..."


#define E4TEXT \
"the spider mastermind must have sent forth\n"\
"its legions of hellspawn before your\n"\
"final confrontation with that terrible\n"\
"beast from hell.  but you stepped forward\n"\
"and brought forth eternal damnation and\n"\
"suffering upon the horde as a true hero\n"\
"would in the face of something so evil.\n"\
"\n"\
"besides, someone was gonna pay for what\n"\
"happened to daisy, your pet rabbit.\n"\
"\n"\
"but now, you see spread before you more\n"\
"potential pain and gibbitude as a nation\n"\
"of demons run amok among our cities.\n"\
"\n"\
"next stop, hell on earth!"


// after level 6, put this:

#define C1TEXT \
"YOU HAVE ENTERED DEEPLY INTO THE INFESTED\n" \
"STARPORT. BUT SOMETHING IS WRONG. THE\n" \
"MONSTERS HAVE BROUGHT THEIR OWN REALITY\n" \
"WITH THEM, AND THE STARPORT'S TECHNOLOGY\n" \
"IS BEING SUBVERTED BY THEIR PRESENCE.\n" \
"\n"\
"AHEAD, YOU SEE AN OUTPOST OF HELL, A\n" \
"FORTIFIED ZONE. IF YOU CAN GET PAST IT,\n" \
"YOU CAN PENETRATE INTO THE HAUNTED HEART\n" \
"OF THE STARBASE AND FIND THE CONTROLLING\n" \
"SWITCH WHICH HOLDS EARTH'S POPULATION\n" \
"HOSTAGE."

// After level 11, put this:

#define C2TEXT \
"YOU HAVE WON! YOUR VICTORY HAS ENABLED\n" \
"HUMANKIND TO EVACUATE EARTH AND ESCAPE\n"\
"THE NIGHTMARE.  NOW YOU ARE THE ONLY\n"\
"HUMAN LEFT ON THE FACE OF THE PLANET.\n"\
"CANNIBAL MUTATIONS, CARNIVOROUS ALIENS,\n"\
"AND EVIL SPIRITS ARE YOUR ONLY NEIGHBORS.\n"\
"YOU SIT BACK AND WAIT FOR DEATH, CONTENT\n"\
"THAT YOU HAVE SAVED YOUR SPECIES.\n"\
"\n"\
"BUT THEN, EARTH CONTROL BEAMS DOWN A\n"\
"MESSAGE FROM SPACE: \"SENSORS HAVE LOCATED\n"\
"THE SOURCE OF THE ALIEN INVASION. IF YOU\n"\
"GO THERE, YOU MAY BE ABLE TO BLOCK THEIR\n"\
"ENTRY.  THE ALIEN BASE IS IN THE HEART OF\n"\
"YOUR OWN HOME CITY, NOT FAR FROM THE\n"\
"STARPORT.\" SLOWLY AND PAINFULLY YOU GET\n"\
"UP AND RETURN TO THE FRAY."


// After level 20, put this:

#define C3TEXT \
"YOU ARE AT THE CORRUPT HEART OF THE CITY,\n"\
"SURROUNDED BY THE CORPSES OF YOUR ENEMIES.\n"\
"YOU SEE NO WAY TO DESTROY THE CREATURES'\n"\
"ENTRYWAY ON THIS SIDE, SO YOU CLENCH YOUR\n"\
"TEETH AND PLUNGE THROUGH IT.\n"\
"\n"\
"THERE MUST BE A WAY TO CLOSE IT ON THE\n"\
"OTHER SIDE. WHAT DO YOU CARE IF YOU'VE\n"\
"GOT TO GO THROUGH HELL TO GET TO IT?"


// After level 29, put this:

#define C4TEXT \
"THE HORRENDOUS VISAGE OF THE BIGGEST\n"\
"DEMON YOU'VE EVER SEEN CRUMBLES BEFORE\n"\
"YOU, AFTER YOU PUMP YOUR ROCKETS INTO\n"\
"HIS EXPOSED BRAIN. THE MONSTER SHRIVELS\n"\
"UP AND DIES, ITS THRASHING LIMBS\n"\
"DEVASTATING UNTOLD MILES OF HELL'S\n"\
"SURFACE.\n"\
"\n"\
"YOU'VE DONE IT. THE INVASION IS OVER.\n"\
"EARTH IS SAVED. HELL IS A WRECK. YOU\n"\
"WONDER WHERE BAD FOLKS WILL GO WHEN THEY\n"\
"DIE, NOW. WIPING THE SWEAT FROM YOUR\n"\
"FOREHEAD YOU BEGIN THE LONG TREK BACK\n"\
"HOME. REBUILDING EARTH OUGHT TO BE A\n"\
"LOT MORE FUN THAN RUINING IT WAS.\n"



// Before level 31, put this:

#define C5TEXT \
"CONGRATULATIONS, YOU'VE FOUND THE SECRET\n"\
"LEVEL! LOOKS LIKE IT'S BEEN BUILT BY\n"\
"HUMANS, RATHER THAN DEMONS. YOU WONDER\n"\
"WHO THE INMATES OF THIS CORNER OF HELL\n"\
"WILL BE."


// Before level 32, put this:

#define C6TEXT \
"CONGRATULATIONS, YOU'VE FOUND THE\n"\
"SUPER SECRET LEVEL!  YOU'D BETTER\n"\
"BLAZE THROUGH THIS ONE!\n"


// after map 06

#define P1TEXT  \
"You gloat over the steaming carcass of the\n"\
"Guardian.  With its death, you've wrested\n"\
"the Accelerator from the stinking claws\n"\
"of Hell.  You relax and glance around the\n"\
"room.  Damn!  There was supposed to be at\n"\
"least one working prototype, but you can't\n"\
"see it. The demons must have taken it.\n"\
"\n"\
"You must find the prototype, or all your\n"\
"struggles will have been wasted. Keep\n"\
"moving, keep fighting, keep killing.\n"\
"Oh yes, keep living, too."


// after map 11

#define P2TEXT \
"Even the deadly Arch-Vile labyrinth could\n"\
"not stop you, and you've gotten to the\n"\
"prototype Accelerator which is soon\n"\
"efficiently and permanently deactivated.\n"\
"\n"\
"You're good at that kind of thing."


// after map 20

#define P3TEXT \
"You've bashed and battered your way into\n"\
"the heart of the devil-hive.  Time for a\n"\
"Search-and-Destroy mission, aimed at the\n"\
"Gatekeeper, whose foul offspring is\n"\
"cascading to Earth.  Yeah, he's bad. But\n"\
"you know who's worse!\n"\
"\n"\
"Grinning evilly, you check your gear, and\n"\
"get ready to give the bastard a little Hell\n"\
"of your own making!"

// after map 30

#define P4TEXT \
"The Gatekeeper's evil face is splattered\n"\
"all over the place.  As its tattered corpse\n"\
"collapses, an inverted Gate forms and\n"\
"sucks down the shards of the last\n"\
"prototype Accelerator, not to mention the\n"\
"few remaining demons.  You're done. Hell\n"\
"has gone back to pounding bad dead folks \n"\
"instead of good live ones.  Remember to\n"\
"tell your grandkids to put a rocket\n"\
"launcher in your coffin. If you go to Hell\n"\
"when you die, you'll need it for some\n"\
"final cleaning-up ..."

// before map 31

#define P5TEXT \
"You've found the second-hardest level we\n"\
"got. Hope you have a saved game a level or\n"\
"two previous.  If not, be prepared to die\n"\
"aplenty. For master marines only."

// before map 32

#define P6TEXT \
"Betcha wondered just what WAS the hardest\n"\
"level we had ready for ya?  Now you know.\n"\
"No one gets out alive."


#define T1TEXT \
"You've fought your way out of the infested\n"\
"experimental labs.   It seems that UAC has\n"\
"once again gulped it down.  With their\n"\
"high turnover, it must be hard for poor\n"\
"old UAC to buy corporate health insurance\n"\
"nowadays..\n"\
"\n"\
"Ahead lies the military complex, now\n"\
"swarming with diseased horrors hot to get\n"\
"their teeth into you. With luck, the\n"\
"complex still has some warlike ordnance\n"\
"laying around."


#define T2TEXT \
"You hear the grinding of heavy machinery\n"\
"ahead.  You sure hope they're not stamping\n"\
"out new hellspawn, but you're ready to\n"\
"ream out a whole herd if you have to.\n"\
"They might be planning a blood feast, but\n"\
"you feel about as mean as two thousand\n"\
"maniacs packed into one mad killer.\n"\
"\n"\
"You don't plan to go down easy."


#define T3TEXT \
"The vista opening ahead looks real damn\n"\
"familiar. Smells familiar, too -- like\n"\
"fried excrement. You didn't like this\n"\
"place before, and you sure as hell ain't\n"\
"planning to like it now. The more you\n"\
"brood on it, the madder you get.\n"\
"Hefting your gun, an evil grin trickles\n"\
"onto your face. Time to take some names."

#define T4TEXT \
"Suddenly, all is silent, from one horizon\n"\
"to the other. The agonizing echo of Hell\n"\
"fades away, the nightmare sky turns to\n"\
"blue, the heaps of monster corpses start \n"\
"to evaporate along with the evil stench \n"\
"that filled the air. Jeeze, maybe you've\n"\
"done it. Have you really won?\n"\
"\n"\
"Something rumbles in the distance.\n"\
"A blue light begins to glow inside the\n"\
"ruined skull of the demon-spitter."


#define T5TEXT \
"What now? Looks totally different. Kind\n"\
"of like King Tut's condo. Well,\n"\
"whatever's here can't be any worse\n"\
"than usual. Can it?  Or maybe it's best\n"\
"to let sleeping gods lie.."


#define T6TEXT \
"Time for a vacation. You've burst the\n"\
"bowels of hell and by golly you're ready\n"\
"for a break. You mutter to yourself,\n"\
"Maybe someone else can kick Hell's ass\n"\
"next time around. Ahead lies a quiet town,\n"\
"with peaceful flowing water, quaint\n"\
"buildings, and presumably no Hellspawn.\n"\
"\n"\
"As you step off the transport, you hear\n"\
"the stomp of a cyberdemon's iron shoe."
*/

// Character cast strings F_FINALE.C
pub const CC_ZOMBIE: *const c_char = c"ZOMBIEMAN".as_ptr();
pub const CC_SHOTGUN: *const c_char = c"SHOTGUN GUY".as_ptr();
pub const CC_HEAVY: *const c_char = c"HEAVY WEAPON DUDE".as_ptr();
pub const CC_IMP: *const c_char = c"IMP".as_ptr();
pub const CC_DEMON: *const c_char = c"DEMON".as_ptr();
pub const CC_LOST: *const c_char = c"LOST SOUL".as_ptr();
pub const CC_CACO: *const c_char = c"CACODEMON".as_ptr();
pub const CC_HELL: *const c_char = c"HELL KNIGHT".as_ptr();
pub const CC_BARON: *const c_char = c"BARON OF HELL".as_ptr();
pub const CC_ARACH: *const c_char = c"ARACHNOTRON".as_ptr();
pub const CC_PAIN: *const c_char = c"PAIN ELEMENTAL".as_ptr();
pub const CC_REVEN: *const c_char = c"REVENANT".as_ptr();
pub const CC_MANCU: *const c_char = c"MANCUBUS".as_ptr();
pub const CC_ARCH: *const c_char = c"ARCH-VILE".as_ptr();
pub const CC_SPIDER: *const c_char = c"THE SPIDER MASTERMIND".as_ptr();
pub const CC_CYBER: *const c_char = c"THE CYBERDEMON".as_ptr();
pub const CC_HERO: *const c_char = c"OUR HERO".as_ptr();
