#![allow(clippy::as_conversions)]

use std::ffi::{CStr, c_char};

//	Printed strings for translation

// D_Main.C
pub(crate) const D_DEVSTR: &str = "Development mode ON.";
pub(crate) const D_CDROM: &str = r"CD-ROM Version: default.cfg from c:\doomdata";

macro_rules! c_concat {
	($($str:expr),* $(,)?) => {
		unsafe { CStr::from_ptr(concat!($($str),*).as_ptr().cast()) }
	}
}

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
pub(crate) const LOADNET: &CStr =
	c_concat!("you can't do load while in a net game!\n\n", PRESSKEY!());
pub(crate) const QLOADNET: &CStr =
	c_concat!("you can't quickload during a netgame!\n\n", PRESSKEY!());
pub(crate) const QSAVESPOT: &CStr =
	c_concat!("you haven't picked a quicksave slot yet!\n\n", PRESSKEY!());
pub(crate) const SAVEDEAD: &CStr =
	c_concat!("you can't save if you aren't playing!\n\n", PRESSKEY!());
pub(crate) const QSPROMPT: &CStr =
	c_concat!("quicksave over your game named\n\n'%s'?\n\n", PRESSYN!());
pub(crate) const QLPROMPT: &CStr =
	c_concat!("do you want to quickload the game named\n\n'%s'?\n\n", PRESSYN!());

pub(crate) const NEWGAME: &CStr =
	c_concat!("you can't start a new game\nwhile in a network game.\n\n", PRESSKEY!());

pub(crate) const NIGHTMARE: &CStr =
	c_concat!("are you sure? this skill level\nisn't even remotely fair.\n\n", PRESSYN!());

pub(crate) const SWSTRING: &CStr = c_concat!(
	"this is the shareware version of doom.\n\nyou need to order the entire trilogy.\n\n",
	PRESSKEY!()
);

pub(crate) const MSGOFF: &CStr = c"Messages OFF";
pub(crate) const MSGON: &CStr = c"Messages ON";
pub(crate) const NETEND: &CStr = c_concat!("you can't end a netgame!\n\n", PRESSKEY!());
pub(crate) const ENDGAME: &CStr =
	c_concat!("are you sure you want to end the game?\n\n", PRESSYN!());

macro_rules! DOSY {
	() => {
		"(press y to quit)\0"
	};
}
pub(crate) use DOSY;

// pub(crate) const DETAILHI: &CStr = c"High detail";
// pub(crate) const DETAILLO: &CStr = c"Low detail";
pub(crate) const GAMMALVL0: &CStr = c"Gamma correction OFF";
pub(crate) const GAMMALVL1: &CStr = c"Gamma correction level 1";
pub(crate) const GAMMALVL2: &CStr = c"Gamma correction level 2";
pub(crate) const GAMMALVL3: &CStr = c"Gamma correction level 3";
pub(crate) const GAMMALVL4: &CStr = c"Gamma correction level 4";
pub(crate) const EMPTYSTRING: &CStr = c"empty slot";

//	P_inter.C
pub(crate) const GOTARMOR: &CStr = c"Picked up the armor.";
pub(crate) const GOTMEGA: &CStr = c"Picked up the MegaArmor!";
pub(crate) const GOTHTHBONUS: &CStr = c"Picked up a health bonus.";
pub(crate) const GOTARMBONUS: &CStr = c"Picked up an armor bonus.";
pub(crate) const GOTSTIM: &CStr = c"Picked up a stimpack.";
pub(crate) const GOTMEDINEED: &CStr = c"Picked up a medikit that you REALLY need!";
pub(crate) const GOTMEDIKIT: &CStr = c"Picked up a medikit.";
pub(crate) const GOTSUPER: &CStr = c"Supercharge!";

pub(crate) const GOTBLUECARD: &CStr = c"Picked up a blue keycard.";
pub(crate) const GOTYELWCARD: &CStr = c"Picked up a yellow keycard.";
pub(crate) const GOTREDCARD: &CStr = c"Picked up a red keycard.";
pub(crate) const GOTBLUESKUL: &CStr = c"Picked up a blue skull key.";
pub(crate) const GOTYELWSKUL: &CStr = c"Picked up a yellow skull key.";
pub(crate) const GOTREDSKULL: &CStr = c"Picked up a red skull key.";

pub(crate) const GOTINVUL: &CStr = c"Invulnerability!";
pub(crate) const GOTBERSERK: &CStr = c"Berserk!";
pub(crate) const GOTINVIS: &CStr = c"Partial Invisibility";
pub(crate) const GOTSUIT: &CStr = c"Radiation Shielding Suit";
pub(crate) const GOTMAP: &CStr = c"Computer Area Map";
pub(crate) const GOTVISOR: &CStr = c"Light Amplification Visor";
pub(crate) const GOTMSPHERE: &CStr = c"MegaSphere!";

pub(crate) const GOTCLIP: &CStr = c"Picked up a clip.";
pub(crate) const GOTCLIPBOX: &CStr = c"Picked up a box of bullets.";
pub(crate) const GOTROCKET: &CStr = c"Picked up a rocket.";
pub(crate) const GOTROCKBOX: &CStr = c"Picked up a box of rockets.";
pub(crate) const GOTCELL: &CStr = c"Picked up an energy cell.";
pub(crate) const GOTCELLBOX: &CStr = c"Picked up an energy cell pack.";
pub(crate) const GOTSHELLS: &CStr = c"Picked up 4 shotgun shells.";
pub(crate) const GOTSHELLBOX: &CStr = c"Picked up a box of shotgun shells.";
pub(crate) const GOTBACKPACK: &CStr = c"Picked up a backpack full of ammo!";

pub(crate) const GOTBFG9000: &CStr = c"You got the BFG9000!  Oh, yes.";
pub(crate) const GOTCHAINGUN: &CStr = c"You got the chaingun!";
pub(crate) const GOTCHAINSAW: &CStr = c"A chainsaw!  Find some meat!";
pub(crate) const GOTLAUNCHER: &CStr = c"You got the rocket launcher!";
pub(crate) const GOTPLASMA: &CStr = c"You got the plasma gun!";
pub(crate) const GOTSHOTGUN: &CStr = c"You got the shotgun!";
pub(crate) const GOTSHOTGUN2: &CStr = c"You got the super shotgun!";

// P_Doors.C
pub(crate) const PD_BLUEO: &CStr = c"You need a blue key to activate this object";
pub(crate) const PD_REDO: &CStr = c"You need a red key to activate this object";
pub(crate) const PD_YELLOWO: &CStr = c"You need a yellow key to activate this object";
pub(crate) const PD_BLUEK: &CStr = c"You need a blue key to open this door";
pub(crate) const PD_REDK: &CStr = c"You need a red key to open this door";
pub(crate) const PD_YELLOWK: &CStr = c"You need a yellow key to open this door";

//	G_game.C
pub(crate) const GGSAVED: &CStr = c"game saved.";

//	HU_stuff.C
pub(crate) const HUSTR_MSGU: &CStr = c"[Message unsent]";

pub(crate) const HUSTR_E1M1: &CStr = c"E1M1: Hangar";
pub(crate) const HUSTR_E1M2: &CStr = c"E1M2: Nuclear Plant";
pub(crate) const HUSTR_E1M3: &CStr = c"E1M3: Toxin Refinery";
pub(crate) const HUSTR_E1M4: &CStr = c"E1M4: Command Control";
pub(crate) const HUSTR_E1M5: &CStr = c"E1M5: Phobos Lab";
pub(crate) const HUSTR_E1M6: &CStr = c"E1M6: Central Processing";
pub(crate) const HUSTR_E1M7: &CStr = c"E1M7: Computer Station";
pub(crate) const HUSTR_E1M8: &CStr = c"E1M8: Phobos Anomaly";
pub(crate) const HUSTR_E1M9: &CStr = c"E1M9: Military Base";

pub(crate) const HUSTR_E2M1: &CStr = c"E2M1: Deimos Anomaly";
pub(crate) const HUSTR_E2M2: &CStr = c"E2M2: Containment Area";
pub(crate) const HUSTR_E2M3: &CStr = c"E2M3: Refinery";
pub(crate) const HUSTR_E2M4: &CStr = c"E2M4: Deimos Lab";
pub(crate) const HUSTR_E2M5: &CStr = c"E2M5: Command Center";
pub(crate) const HUSTR_E2M6: &CStr = c"E2M6: Halls of the Damned";
pub(crate) const HUSTR_E2M7: &CStr = c"E2M7: Spawning Vats";
pub(crate) const HUSTR_E2M8: &CStr = c"E2M8: Tower of Babel";
pub(crate) const HUSTR_E2M9: &CStr = c"E2M9: Fortress of Mystery";

pub(crate) const HUSTR_E3M1: &CStr = c"E3M1: Hell Keep";
pub(crate) const HUSTR_E3M2: &CStr = c"E3M2: Slough of Despair";
pub(crate) const HUSTR_E3M3: &CStr = c"E3M3: Pandemonium";
pub(crate) const HUSTR_E3M4: &CStr = c"E3M4: House of Pain";
pub(crate) const HUSTR_E3M5: &CStr = c"E3M5: Unholy Cathedral";
pub(crate) const HUSTR_E3M6: &CStr = c"E3M6: Mt. Erebus";
pub(crate) const HUSTR_E3M7: &CStr = c"E3M7: Limbo";
pub(crate) const HUSTR_E3M8: &CStr = c"E3M8: Dis";
pub(crate) const HUSTR_E3M9: &CStr = c"E3M9: Warrens";

pub(crate) const HUSTR_E4M1: &CStr = c"E4M1: Hell Beneath";
pub(crate) const HUSTR_E4M2: &CStr = c"E4M2: Perfect Hatred";
pub(crate) const HUSTR_E4M3: &CStr = c"E4M3: Sever The Wicked";
pub(crate) const HUSTR_E4M4: &CStr = c"E4M4: Unruly Evil";
pub(crate) const HUSTR_E4M5: &CStr = c"E4M5: They Will Repent";
pub(crate) const HUSTR_E4M6: &CStr = c"E4M6: Against Thee Wickedly";
pub(crate) const HUSTR_E4M7: &CStr = c"E4M7: And Hell Followed";
pub(crate) const HUSTR_E4M8: &CStr = c"E4M8: Unto The Cruel";
pub(crate) const HUSTR_E4M9: &CStr = c"E4M9: Fear";

pub(crate) const HUSTR_1: &CStr = c"level 1: entryway";
pub(crate) const HUSTR_2: &CStr = c"level 2: underhalls";
pub(crate) const HUSTR_3: &CStr = c"level 3: the gantlet";
pub(crate) const HUSTR_4: &CStr = c"level 4: the focus";
pub(crate) const HUSTR_5: &CStr = c"level 5: the waste tunnels";
pub(crate) const HUSTR_6: &CStr = c"level 6: the crusher";
pub(crate) const HUSTR_7: &CStr = c"level 7: dead simple";
pub(crate) const HUSTR_8: &CStr = c"level 8: tricks and traps";
pub(crate) const HUSTR_9: &CStr = c"level 9: the pit";
pub(crate) const HUSTR_10: &CStr = c"level 10: refueling base";
pub(crate) const HUSTR_11: &CStr = c"level 11: 'o' of destruction!";

pub(crate) const HUSTR_12: &CStr = c"level 12: the factory";
pub(crate) const HUSTR_13: &CStr = c"level 13: downtown";
pub(crate) const HUSTR_14: &CStr = c"level 14: the inmost dens";
pub(crate) const HUSTR_15: &CStr = c"level 15: industrial zone";
pub(crate) const HUSTR_16: &CStr = c"level 16: suburbs";
pub(crate) const HUSTR_17: &CStr = c"level 17: tenements";
pub(crate) const HUSTR_18: &CStr = c"level 18: the courtyard";
pub(crate) const HUSTR_19: &CStr = c"level 19: the citadel";
pub(crate) const HUSTR_20: &CStr = c"level 20: gotcha!";

pub(crate) const HUSTR_21: &CStr = c"level 21: nirvana";
pub(crate) const HUSTR_22: &CStr = c"level 22: the catacombs";
pub(crate) const HUSTR_23: &CStr = c"level 23: barrels o' fun";
pub(crate) const HUSTR_24: &CStr = c"level 24: the chasm";
pub(crate) const HUSTR_25: &CStr = c"level 25: bloodfalls";
pub(crate) const HUSTR_26: &CStr = c"level 26: the abandoned mines";
pub(crate) const HUSTR_27: &CStr = c"level 27: monster condo";
pub(crate) const HUSTR_28: &CStr = c"level 28: the spirit world";
pub(crate) const HUSTR_29: &CStr = c"level 29: the living end";
pub(crate) const HUSTR_30: &CStr = c"level 30: icon of sin";

pub(crate) const HUSTR_31: &CStr = c"level 31: wolfenstein";
pub(crate) const HUSTR_32: &CStr = c"level 32: grosse";

// pub(crate) const PHUSTR_1: &CStr = c"level 1: congo";
// pub(crate) const PHUSTR_2: &CStr = c"level 2: well of souls";
// pub(crate) const PHUSTR_3: &CStr = c"level 3: aztec";
// pub(crate) const PHUSTR_4: &CStr = c"level 4: caged";
// pub(crate) const PHUSTR_5: &CStr = c"level 5: ghost town";
// pub(crate) const PHUSTR_6: &CStr = c"level 6: baron's lair";
// pub(crate) const PHUSTR_7: &CStr = c"level 7: caughtyard";
// pub(crate) const PHUSTR_8: &CStr = c"level 8: realm";
// pub(crate) const PHUSTR_9: &CStr = c"level 9: abattoire";
// pub(crate) const PHUSTR_10: &CStr = c"level 10: onslaught";
// pub(crate) const PHUSTR_11: &CStr = c"level 11: hunted";

// pub(crate) const PHUSTR_12: &CStr = c"level 12: speed";
// pub(crate) const PHUSTR_13: &CStr = c"level 13: the crypt";
// pub(crate) const PHUSTR_14: &CStr = c"level 14: genesis";
// pub(crate) const PHUSTR_15: &CStr = c"level 15: the twilight";
// pub(crate) const PHUSTR_16: &CStr = c"level 16: the omen";
// pub(crate) const PHUSTR_17: &CStr = c"level 17: compound";
// pub(crate) const PHUSTR_18: &CStr = c"level 18: neurosphere";
// pub(crate) const PHUSTR_19: &CStr = c"level 19: nme";
// pub(crate) const PHUSTR_20: &CStr = c"level 20: the death domain";

// pub(crate) const PHUSTR_21: &CStr = c"level 21: slayer";
// pub(crate) const PHUSTR_22: &CStr = c"level 22: impossible mission";
// pub(crate) const PHUSTR_23: &CStr = c"level 23: tombstone";
// pub(crate) const PHUSTR_24: &CStr = c"level 24: the final frontier";
// pub(crate) const PHUSTR_25: &CStr = c"level 25: the temple of darkness";
// pub(crate) const PHUSTR_26: &CStr = c"level 26: bunker";
// pub(crate) const PHUSTR_27: &CStr = c"level 27: anti-christ";
// pub(crate) const PHUSTR_28: &CStr = c"level 28: the sewers";
// pub(crate) const PHUSTR_29: &CStr = c"level 29: odyssey of noises";
// pub(crate) const PHUSTR_30: &CStr = c"level 30: the gateway of hell";

// pub(crate) const PHUSTR_31: &CStr = c"level 31: cyberden";
// pub(crate) const PHUSTR_32: &CStr = c"level 32: go 2 it";

// pub(crate) const THUSTR_1: &CStr = c"level 1: system control";
// pub(crate) const THUSTR_2: &CStr = c"level 2: human bbq";
// pub(crate) const THUSTR_3: &CStr = c"level 3: power control";
// pub(crate) const THUSTR_4: &CStr = c"level 4: wormhole";
// pub(crate) const THUSTR_5: &CStr = c"level 5: hanger";
// pub(crate) const THUSTR_6: &CStr = c"level 6: open season";
// pub(crate) const THUSTR_7: &CStr = c"level 7: prison";
// pub(crate) const THUSTR_8: &CStr = c"level 8: metal";
// pub(crate) const THUSTR_9: &CStr = c"level 9: stronghold";
// pub(crate) const THUSTR_10: &CStr = c"level 10: redemption";
// pub(crate) const THUSTR_11: &CStr = c"level 11: storage facility";

// pub(crate) const THUSTR_12: &CStr = c"level 12: crater";
// pub(crate) const THUSTR_13: &CStr = c"level 13: nukage processing";
// pub(crate) const THUSTR_14: &CStr = c"level 14: steel works";
// pub(crate) const THUSTR_15: &CStr = c"level 15: dead zone";
// pub(crate) const THUSTR_16: &CStr = c"level 16: deepest reaches";
// pub(crate) const THUSTR_17: &CStr = c"level 17: processing area";
// pub(crate) const THUSTR_18: &CStr = c"level 18: mill";
// pub(crate) const THUSTR_19: &CStr = c"level 19: shipping/respawning";
// pub(crate) const THUSTR_20: &CStr = c"level 20: central processing";

// pub(crate) const THUSTR_21: &CStr = c"level 21: administration center";
// pub(crate) const THUSTR_22: &CStr = c"level 22: habitat";
// pub(crate) const THUSTR_23: &CStr = c"level 23: lunar mining project";
// pub(crate) const THUSTR_24: &CStr = c"level 24: quarry";
// pub(crate) const THUSTR_25: &CStr = c"level 25: baron's den";
// pub(crate) const THUSTR_26: &CStr = c"level 26: ballistyx";
// pub(crate) const THUSTR_27: &CStr = c"level 27: mount pain";
// pub(crate) const THUSTR_28: &CStr = c"level 28: heck";
// pub(crate) const THUSTR_29: &CStr = c"level 29: river styx";
// pub(crate) const THUSTR_30: &CStr = c"level 30: last call";

// pub(crate) const THUSTR_31: &CStr = c"level 31: pharaoh";
// pub(crate) const THUSTR_32: &CStr = c"level 32: caribbean";

pub(crate) const HUSTR_CHATMACRO1: &CStr = c"I'm ready to kick butt!";
pub(crate) const HUSTR_CHATMACRO2: &CStr = c"I'm OK.";
pub(crate) const HUSTR_CHATMACRO3: &CStr = c"I'm not looking too good!";
pub(crate) const HUSTR_CHATMACRO4: &CStr = c"Help!";
pub(crate) const HUSTR_CHATMACRO5: &CStr = c"You suck!";
pub(crate) const HUSTR_CHATMACRO6: &CStr = c"Next time, scumbag...";
pub(crate) const HUSTR_CHATMACRO7: &CStr = c"Come here!";
pub(crate) const HUSTR_CHATMACRO8: &CStr = c"I'll take care of it.";
pub(crate) const HUSTR_CHATMACRO9: &CStr = c"Yes";
pub(crate) const HUSTR_CHATMACRO0: &CStr = c"No";

pub(crate) const HUSTR_TALKTOSELF1: &CStr = c"You mumble to yourself";
pub(crate) const HUSTR_TALKTOSELF2: &CStr = c"Who's there?";
pub(crate) const HUSTR_TALKTOSELF3: &CStr = c"You scare yourself";
pub(crate) const HUSTR_TALKTOSELF4: &CStr = c"You start to rave";
pub(crate) const HUSTR_TALKTOSELF5: &CStr = c"You've lost it...";

// pub(crate) const HUSTR_MESSAGESENT: *const c_char = c"[Message Sent]".as_ptr();

// The following should NOT be changed unless it seems
// just AWFULLY necessary

pub(crate) const HUSTR_PLRGREEN: &CStr = c"Green: ";
pub(crate) const HUSTR_PLRINDIGO: &CStr = c"Indigo: ";
pub(crate) const HUSTR_PLRBROWN: &CStr = c"Brown: ";
pub(crate) const HUSTR_PLRRED: &CStr = c"Red: ";

pub(crate) const HUSTR_KEYGREEN: c_char = b'g' as c_char;
pub(crate) const HUSTR_KEYINDIGO: c_char = b'i' as c_char;
pub(crate) const HUSTR_KEYBROWN: c_char = b'b' as c_char;
pub(crate) const HUSTR_KEYRED: c_char = b'r' as c_char;

//	AM_map.C

pub(crate) const AMSTR_FOLLOWON: &CStr = c"Follow Mode ON";
pub(crate) const AMSTR_FOLLOWOFF: &CStr = c"Follow Mode OFF";

pub(crate) const AMSTR_GRIDON: &CStr = c"Grid ON";
pub(crate) const AMSTR_GRIDOFF: &CStr = c"Grid OFF";

pub(crate) const AMSTR_MARKEDSPOT: &CStr = c"Marked Spot";
pub(crate) const AMSTR_MARKSCLEARED: &CStr = c"All Marks Cleared";

//	ST_stuff.C

pub(crate) const STSTR_MUS: &CStr = c"Music Change";
pub(crate) const STSTR_NOMUS: &CStr = c"IMPOSSIBLE SELECTION";
pub(crate) const STSTR_DQDON: &CStr = c"Degreelessness Mode On";
pub(crate) const STSTR_DQDOFF: &CStr = c"Degreelessness Mode Off";

pub(crate) const STSTR_KFAADDED: &CStr = c"Very Happy Ammo Added";
pub(crate) const STSTR_FAADDED: &CStr = c"Ammo (no keys) Added";

pub(crate) const STSTR_NCON: &CStr = c"No Clipping Mode ON";
pub(crate) const STSTR_NCOFF: &CStr = c"No Clipping Mode OFF";

pub(crate) const STSTR_BEHOLD: &CStr = c"inVuln, Str, Inviso, Rad, Allmap, or Lite-amp";
pub(crate) const STSTR_BEHOLDX: &CStr = c"Power-up Toggled";

pub(crate) const STSTR_CHOPPERS: &CStr = c"... doesn't suck - GM";
pub(crate) const STSTR_CLEV: &CStr = c"Changing Level...";

//	F_Finale.C
pub(crate) const E1TEXT: &CStr = c"Once you beat the big badasses and
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
sequel, Inferno!\n";

pub(crate) const E2TEXT: &CStr = c"You've done it! The hideous cyber-
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
DOOM! -- Inferno.";

pub(crate) const E3TEXT: &CStr = c"The loathsome spiderdemon that
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
door with you ...";

pub(crate) const E4TEXT: &CStr = c"the spider mastermind must have sent forth
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

next stop, hell on earth!";

// after level 6, put this:

pub(crate) const C1TEXT: &CStr = c"YOU HAVE ENTERED DEEPLY INTO THE INFESTED
STARPORT. BUT SOMETHING IS WRONG. THE
MONSTERS HAVE BROUGHT THEIR OWN REALITY
WITH THEM, AND THE STARPORT'S TECHNOLOGY
IS BEING SUBVERTED BY THEIR PRESENCE.

AHEAD, YOU SEE AN OUTPOST OF HELL, A
FORTIFIED ZONE. IF YOU CAN GET PAST IT,
YOU CAN PENETRATE INTO THE HAUNTED HEART
OF THE STARBASE AND FIND THE CONTROLLING
SWITCH WHICH HOLDS EARTH'S POPULATION
HOSTAGE.";

// After level 11, put this:

pub(crate) const C2TEXT: &CStr = c"YOU HAVE WON! YOUR VICTORY HAS ENABLED
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
UP AND RETURN TO THE FRAY.";

// After level 20, put this:

pub(crate) const C3TEXT: &CStr = c"YOU ARE AT THE CORRUPT HEART OF THE CITY,
SURROUNDED BY THE CORPSES OF YOUR ENEMIES.
YOU SEE NO WAY TO DESTROY THE CREATURES'
ENTRYWAY ON THIS SIDE, SO YOU CLENCH YOUR
TEETH AND PLUNGE THROUGH IT.

THERE MUST BE A WAY TO CLOSE IT ON THE
OTHER SIDE. WHAT DO YOU CARE IF YOU'VE
GOT TO GO THROUGH HELL TO GET TO IT?";

// After level 29, put this:

pub(crate) const C4TEXT: &CStr = c"THE HORRENDOUS VISAGE OF THE BIGGEST
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
LOT MORE FUN THAN RUINING IT WAS.\n";

// Before level 31, put this:

pub(crate) const C5TEXT: &CStr = c"CONGRATULATIONS, YOU'VE FOUND THE SECRET
LEVEL! LOOKS LIKE IT'S BEEN BUILT BY
HUMANS, RATHER THAN DEMONS. YOU WONDER
WHO THE INMATES OF THIS CORNER OF HELL
WILL BE.";

// Before level 32, put this:

pub(crate) const C6TEXT: &CStr = c"CONGRATULATIONS, YOU'VE FOUND THE
SUPER SECRET LEVEL!  YOU'D BETTER
BLAZE THROUGH THIS ONE!\n";

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
