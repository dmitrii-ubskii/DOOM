use std::ffi::{CStr, c_char};

pub(crate) const SAVEGAMENAME: *const c_char = c"doomsav".as_ptr();

pub(crate) const NUM_QUITMESSAGES: usize = 22;

// from d_englsh.h
const QUITMSG: &CStr = c"are you sure you want to\nquit this great game?";

pub(crate) const endmsg: [&CStr; NUM_QUITMESSAGES + 1] = [
	// DOOM1
	QUITMSG,
	c"please don't leave, there's more\ndemons to toast!",
	c"let's beat it -- this is turning\ninto a bloodbath!",
	c"i wouldn't leave if i were you.\ndos is much worse.",
	c"you're trying to say you like dos\nbetter than me, right?",
	c"don't leave yet -- there's a\ndemon around that corner!",
	c"ya know, next time you come in here\ni'm gonna toast ya.",
	c"go ahead and leave. see if i care.",
	// QuitDOOM II messages
	c"you want to quit?\nthen, thou hast lost an eighth!",
	c"don't go now, there's a \ndimensional shambler waiting\nat the dos prompt!",
	c"get outta here and go back\nto your boring programs.",
	c"if i were your boss, i'd \n deathmatch ya in a minute!",
	c"look, bud. you leave now\nand you forfeit your body count!",
	c"just leave. when you come\nback, i'll be waiting with a bat.",
	c"you're lucky i don't smack\nyou for thinking about leaving.",
	// FinalDOOM?
	c"fuck you, pussy!\nget the fuck out!",
	c"you quit and i'll jizz\nin your cystholes!",
	c"if you leave, i'll make\nthe lord drink my jizz.",
	c"hey, ron! can we say\n'fuck' in the game?",
	c"i'd leave: this is just\nmore monsters and levels.\nwhat a load.",
	c"suck it down, asshole!\nyou're a fucking wimp!",
	c"don't quit now! we're \nstill spending your money!",
	// Internal debug. Different style, too.
	c"THIS IS NO MESSAGE!\nPage intentionally left blank.",
];
