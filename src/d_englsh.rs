use std::ffi::{CStr, c_char};

//	Printed strings for translation

// D_Main.C
pub const D_DEVSTR: &CStr = c"Development mode ON.\n";
pub const D_CDROM: &CStr = c"CD-ROM Version: default.cfg from c:\\doomdata\n";

/*
//	M_Menu.C
pub const PRESSKEY: &CStr = c"press a key.";
pub const PRESSYN: &CStr = c"press y or n.";
pub const QUITMSG: &CStr = c"are you sure you want to\nquit this great game?";
#define LOADNET 	"you can't do load while in a net game!\n\n"PRESSKEY
#define QLOADNET	"you can't quickload during a netgame!\n\n"PRESSKEY
#define QSAVESPOT	"you haven't picked a quicksave slot yet!\n\n"PRESSKEY
#define SAVEDEAD 	"you can't save if you aren't playing!\n\n"PRESSKEY
#define QSPROMPT 	"quicksave over your game named\n\n'%s'?\n\n"PRESSYN
#define QLPROMPT	"do you want to quickload the game named\n\n'%s'?\n\n"PRESSYN

#define NEWGAME	"you can't start a new game\nwhile in a network game.\n\n"PRESSKEY

#define NIGHTMARE	"are you sure? this skill level\nisn't even remotely fair.\n\n"PRESSYN

#define SWSTRING	"this is the shareware version of doom.\n\nyou need to order the entire trilogy.\n\n"PRESSKEY

pub const MSGOFF: &CStr = c"Messages OFF";
pub const MSGON: &CStr = c"Messages ON";
#define NETEND	"you can't end a netgame!\n\n"PRESSKEY
#define ENDGAME	"are you sure you want to end the game?\n\n"PRESSYN

pub const DOSY: &CStr = c"(press y to quit)";

pub const DETAILHI: &CStr = c"High detail";
pub const DETAILLO: &CStr = c"Low detail";
pub const GAMMALVL0: &CStr = c"Gamma correction OFF";
pub const GAMMALVL1: &CStr = c"Gamma correction level 1";
pub const GAMMALVL2: &CStr = c"Gamma correction level 2";
pub const GAMMALVL3: &CStr = c"Gamma correction level 3";
pub const GAMMALVL4: &CStr = c"Gamma correction level 4";
pub const EMPTYSTRING: &CStr = c"empty slot";
*/

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

/*
//	HU_stuff.C
//
pub const HUSTR_MSGU: &CStr = c"[Message unsent]";

pub const HUSTR_E1M1: &CStr = c"E1M1: Hangar";
pub const HUSTR_E1M2: &CStr = c"E1M2: Nuclear Plant";
pub const HUSTR_E1M3: &CStr = c"E1M3: Toxin Refinery";
pub const HUSTR_E1M4: &CStr = c"E1M4: Command Control";
pub const HUSTR_E1M5: &CStr = c"E1M5: Phobos Lab";
pub const HUSTR_E1M6: &CStr = c"E1M6: Central Processing";
pub const HUSTR_E1M7: &CStr = c"E1M7: Computer Station";
pub const HUSTR_E1M8: &CStr = c"E1M8: Phobos Anomaly";
pub const HUSTR_E1M9: &CStr = c"E1M9: Military Base";

pub const HUSTR_E2M1: &CStr = c"E2M1: Deimos Anomaly";
pub const HUSTR_E2M2: &CStr = c"E2M2: Containment Area";
pub const HUSTR_E2M3: &CStr = c"E2M3: Refinery";
pub const HUSTR_E2M4: &CStr = c"E2M4: Deimos Lab";
pub const HUSTR_E2M5: &CStr = c"E2M5: Command Center";
pub const HUSTR_E2M6: &CStr = c"E2M6: Halls of the Damned";
pub const HUSTR_E2M7: &CStr = c"E2M7: Spawning Vats";
pub const HUSTR_E2M8: &CStr = c"E2M8: Tower of Babel";
pub const HUSTR_E2M9: &CStr = c"E2M9: Fortress of Mystery";

pub const HUSTR_E3M1: &CStr = c"E3M1: Hell Keep";
pub const HUSTR_E3M2: &CStr = c"E3M2: Slough of Despair";
pub const HUSTR_E3M3: &CStr = c"E3M3: Pandemonium";
pub const HUSTR_E3M4: &CStr = c"E3M4: House of Pain";
pub const HUSTR_E3M5: &CStr = c"E3M5: Unholy Cathedral";
pub const HUSTR_E3M6: &CStr = c"E3M6: Mt. Erebus";
pub const HUSTR_E3M7: &CStr = c"E3M7: Limbo";
pub const HUSTR_E3M8: &CStr = c"E3M8: Dis";
pub const HUSTR_E3M9: &CStr = c"E3M9: Warrens";

pub const HUSTR_E4M1: &CStr = c"E4M1: Hell Beneath";
pub const HUSTR_E4M2: &CStr = c"E4M2: Perfect Hatred";
pub const HUSTR_E4M3: &CStr = c"E4M3: Sever The Wicked";
pub const HUSTR_E4M4: &CStr = c"E4M4: Unruly Evil";
pub const HUSTR_E4M5: &CStr = c"E4M5: They Will Repent";
pub const HUSTR_E4M6: &CStr = c"E4M6: Against Thee Wickedly";
pub const HUSTR_E4M7: &CStr = c"E4M7: And Hell Followed";
pub const HUSTR_E4M8: &CStr = c"E4M8: Unto The Cruel";
pub const HUSTR_E4M9: &CStr = c"E4M9: Fear";

pub const HUSTR_1: &CStr = c"level 1: entryway";
pub const HUSTR_2: &CStr = c"level 2: underhalls";
pub const HUSTR_3: &CStr = c"level 3: the gantlet";
pub const HUSTR_4: &CStr = c"level 4: the focus";
pub const HUSTR_5: &CStr = c"level 5: the waste tunnels";
pub const HUSTR_6: &CStr = c"level 6: the crusher";
pub const HUSTR_7: &CStr = c"level 7: dead simple";
pub const HUSTR_8: &CStr = c"level 8: tricks and traps";
pub const HUSTR_9: &CStr = c"level 9: the pit";
pub const HUSTR_10: &CStr = c"level 10: refueling base";
pub const HUSTR_11: &CStr = c"level 11: 'o' of destruction!";

pub const HUSTR_12: &CStr = c"level 12: the factory";
pub const HUSTR_13: &CStr = c"level 13: downtown";
pub const HUSTR_14: &CStr = c"level 14: the inmost dens";
pub const HUSTR_15: &CStr = c"level 15: industrial zone";
pub const HUSTR_16: &CStr = c"level 16: suburbs";
pub const HUSTR_17: &CStr = c"level 17: tenements";
pub const HUSTR_18: &CStr = c"level 18: the courtyard";
pub const HUSTR_19: &CStr = c"level 19: the citadel";
pub const HUSTR_20: &CStr = c"level 20: gotcha!";

pub const HUSTR_21: &CStr = c"level 21: nirvana";
pub const HUSTR_22: &CStr = c"level 22: the catacombs";
pub const HUSTR_23: &CStr = c"level 23: barrels o' fun";
pub const HUSTR_24: &CStr = c"level 24: the chasm";
pub const HUSTR_25: &CStr = c"level 25: bloodfalls";
pub const HUSTR_26: &CStr = c"level 26: the abandoned mines";
pub const HUSTR_27: &CStr = c"level 27: monster condo";
pub const HUSTR_28: &CStr = c"level 28: the spirit world";
pub const HUSTR_29: &CStr = c"level 29: the living end";
pub const HUSTR_30: &CStr = c"level 30: icon of sin";

pub const HUSTR_31: &CStr = c"level 31: wolfenstein";
pub const HUSTR_32: &CStr = c"level 32: grosse";

pub const PHUSTR_1: &CStr = c"level 1: congo";
pub const PHUSTR_2: &CStr = c"level 2: well of souls";
pub const PHUSTR_3: &CStr = c"level 3: aztec";
pub const PHUSTR_4: &CStr = c"level 4: caged";
pub const PHUSTR_5: &CStr = c"level 5: ghost town";
pub const PHUSTR_6: &CStr = c"level 6: baron's lair";
pub const PHUSTR_7: &CStr = c"level 7: caughtyard";
pub const PHUSTR_8: &CStr = c"level 8: realm";
pub const PHUSTR_9: &CStr = c"level 9: abattoire";
pub const PHUSTR_10: &CStr = c"level 10: onslaught";
pub const PHUSTR_11: &CStr = c"level 11: hunted";

pub const PHUSTR_12: &CStr = c"level 12: speed";
pub const PHUSTR_13: &CStr = c"level 13: the crypt";
pub const PHUSTR_14: &CStr = c"level 14: genesis";
pub const PHUSTR_15: &CStr = c"level 15: the twilight";
pub const PHUSTR_16: &CStr = c"level 16: the omen";
pub const PHUSTR_17: &CStr = c"level 17: compound";
pub const PHUSTR_18: &CStr = c"level 18: neurosphere";
pub const PHUSTR_19: &CStr = c"level 19: nme";
pub const PHUSTR_20: &CStr = c"level 20: the death domain";

pub const PHUSTR_21: &CStr = c"level 21: slayer";
pub const PHUSTR_22: &CStr = c"level 22: impossible mission";
pub const PHUSTR_23: &CStr = c"level 23: tombstone";
pub const PHUSTR_24: &CStr = c"level 24: the final frontier";
pub const PHUSTR_25: &CStr = c"level 25: the temple of darkness";
pub const PHUSTR_26: &CStr = c"level 26: bunker";
pub const PHUSTR_27: &CStr = c"level 27: anti-christ";
pub const PHUSTR_28: &CStr = c"level 28: the sewers";
pub const PHUSTR_29: &CStr = c"level 29: odyssey of noises";
pub const PHUSTR_30: &CStr = c"level 30: the gateway of hell";

pub const PHUSTR_31: &CStr = c"level 31: cyberden";
pub const PHUSTR_32: &CStr = c"level 32: go 2 it";

pub const THUSTR_1: &CStr = c"level 1: system control";
pub const THUSTR_2: &CStr = c"level 2: human bbq";
pub const THUSTR_3: &CStr = c"level 3: power control";
pub const THUSTR_4: &CStr = c"level 4: wormhole";
pub const THUSTR_5: &CStr = c"level 5: hanger";
pub const THUSTR_6: &CStr = c"level 6: open season";
pub const THUSTR_7: &CStr = c"level 7: prison";
pub const THUSTR_8: &CStr = c"level 8: metal";
pub const THUSTR_9: &CStr = c"level 9: stronghold";
pub const THUSTR_10: &CStr = c"level 10: redemption";
pub const THUSTR_11: &CStr = c"level 11: storage facility";

pub const THUSTR_12: &CStr = c"level 12: crater";
pub const THUSTR_13: &CStr = c"level 13: nukage processing";
pub const THUSTR_14: &CStr = c"level 14: steel works";
pub const THUSTR_15: &CStr = c"level 15: dead zone";
pub const THUSTR_16: &CStr = c"level 16: deepest reaches";
pub const THUSTR_17: &CStr = c"level 17: processing area";
pub const THUSTR_18: &CStr = c"level 18: mill";
pub const THUSTR_19: &CStr = c"level 19: shipping/respawning";
pub const THUSTR_20: &CStr = c"level 20: central processing";

pub const THUSTR_21: &CStr = c"level 21: administration center";
pub const THUSTR_22: &CStr = c"level 22: habitat";
pub const THUSTR_23: &CStr = c"level 23: lunar mining project";
pub const THUSTR_24: &CStr = c"level 24: quarry";
pub const THUSTR_25: &CStr = c"level 25: baron's den";
pub const THUSTR_26: &CStr = c"level 26: ballistyx";
pub const THUSTR_27: &CStr = c"level 27: mount pain";
pub const THUSTR_28: &CStr = c"level 28: heck";
pub const THUSTR_29: &CStr = c"level 29: river styx";
pub const THUSTR_30: &CStr = c"level 30: last call";

pub const THUSTR_31: &CStr = c"level 31: pharaoh";
pub const THUSTR_32: &CStr = c"level 32: caribbean";

pub const HUSTR_CHATMACRO1: &CStr = c"I'm ready to kick butt!";
pub const HUSTR_CHATMACRO2: &CStr = c"I'm OK.";
pub const HUSTR_CHATMACRO3: &CStr = c"I'm not looking too good!";
pub const HUSTR_CHATMACRO4: &CStr = c"Help!";
pub const HUSTR_CHATMACRO5: &CStr = c"You suck!";
pub const HUSTR_CHATMACRO6: &CStr = c"Next time, scumbag...";
pub const HUSTR_CHATMACRO7: &CStr = c"Come here!";
pub const HUSTR_CHATMACRO8: &CStr = c"I'll take care of it.";
pub const HUSTR_CHATMACRO9: &CStr = c"Yes";
pub const HUSTR_CHATMACRO0: &CStr = c"No";

pub const HUSTR_TALKTOSELF1: &CStr = c"You mumble to yourself";
pub const HUSTR_TALKTOSELF2: &CStr = c"Who's there?";
pub const HUSTR_TALKTOSELF3: &CStr = c"You scare yourself";
pub const HUSTR_TALKTOSELF4: &CStr = c"You start to rave";
pub const HUSTR_TALKTOSELF5: &CStr = c"You've lost it...";

pub const HUSTR_MESSAGESENT: &CStr = c"[Message Sent]";

// The following should NOT be changed unless it seems
// just AWFULLY necessary

pub const HUSTR_PLRGREEN: &CStr = c"Green: ";
pub const HUSTR_PLRINDIGO: &CStr = c"Indigo: ";
pub const HUSTR_PLRBROWN: &CStr = c"Brown: ";
pub const HUSTR_PLRRED: &CStr = c"Red: ";

#define HUSTR_KEYGREEN	'g'
#define HUSTR_KEYINDIGO	'i'
#define HUSTR_KEYBROWN	'b'
#define HUSTR_KEYRED	'r'

//
//	AM_map.C
//

pub const AMSTR_FOLLOWON: &CStr = c"Follow Mode ON";
pub const AMSTR_FOLLOWOFF: &CStr = c"Follow Mode OFF";

pub const AMSTR_GRIDON: &CStr = c"Grid ON";
pub const AMSTR_GRIDOFF: &CStr = c"Grid OFF";

pub const AMSTR_MARKEDSPOT: &CStr = c"Marked Spot";
pub const AMSTR_MARKSCLEARED: &CStr = c"All Marks Cleared";

//
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



//
// Character cast strings F_FINALE.C
//
pub const CC_ZOMBIE: &CStr = c"ZOMBIEMAN";
pub const CC_SHOTGUN: &CStr = c"SHOTGUN GUY";
pub const CC_HEAVY: &CStr = c"HEAVY WEAPON DUDE";
pub const CC_IMP: &CStr = c"IMP";
pub const CC_DEMON: &CStr = c"DEMON";
pub const CC_LOST: &CStr = c"LOST SOUL";
pub const CC_CACO: &CStr = c"CACODEMON";
pub const CC_HELL: &CStr = c"HELL KNIGHT";
pub const CC_BARON: &CStr = c"BARON OF HELL";
pub const CC_ARACH: &CStr = c"ARACHNOTRON";
pub const CC_PAIN: &CStr = c"PAIN ELEMENTAL";
pub const CC_REVEN: &CStr = c"REVENANT";
pub const CC_MANCU: &CStr = c"MANCUBUS";
pub const CC_ARCH: &CStr = c"ARCH-VILE";
pub const CC_SPIDER: &CStr = c"THE SPIDER MASTERMIND";
pub const CC_CYBER: &CStr = c"THE CYBERDEMON";
pub const CC_HERO: &CStr = c"OUR HERO";
*/
