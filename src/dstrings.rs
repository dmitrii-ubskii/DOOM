use std::ffi::{CStr, c_char};

pub const SAVEGAMENAME: *const c_char = c"doomsav".as_ptr();

pub(crate) const NUM_QUITMESSAGES: usize = 22;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Smuggle<T>(pub *const T);

unsafe impl<T> Sync for Smuggle<T> {}

impl<T> Smuggle<T> {
	pub fn u(self) -> *const T {
		self.0
	}
}

// from d_englsh.h
const QUITMSG: &CStr = c"are you sure you want to\nquit this great game?";

#[unsafe(no_mangle)]
pub static endmsg: [Smuggle<c_char>; NUM_QUITMESSAGES + 1] = [
	// DOOM1
	Smuggle(QUITMSG.as_ptr()),
	Smuggle(c"please don't leave, there's more\ndemons to toast!".as_ptr()),
	Smuggle(c"let's beat it -- this is turning\ninto a bloodbath!".as_ptr()),
	Smuggle(c"i wouldn't leave if i were you.\ndos is much worse.".as_ptr()),
	Smuggle(c"you're trying to say you like dos\nbetter than me, right?".as_ptr()),
	Smuggle(c"don't leave yet -- there's a\ndemon around that corner!".as_ptr()),
	Smuggle(c"ya know, next time you come in here\ni'm gonna toast ya.".as_ptr()),
	Smuggle(c"go ahead and leave. see if i care.".as_ptr()),
	// QuitDOOM II messages
	Smuggle(c"you want to quit?\nthen, thou hast lost an eighth!".as_ptr()),
	Smuggle(c"don't go now, there's a \ndimensional shambler waiting\nat the dos prompt!".as_ptr()),
	Smuggle(c"get outta here and go back\nto your boring programs.".as_ptr()),
	Smuggle(c"if i were your boss, i'd \n deathmatch ya in a minute!".as_ptr()),
	Smuggle(c"look, bud. you leave now\nand you forfeit your body count!".as_ptr()),
	Smuggle(c"just leave. when you come\nback, i'll be waiting with a bat.".as_ptr()),
	Smuggle(c"you're lucky i don't smack\nyou for thinking about leaving.".as_ptr()),
	// FinalDOOM?
	Smuggle(c"fuck you, pussy!\nget the fuck out!".as_ptr()),
	Smuggle(c"you quit and i'll jizz\nin your cystholes!".as_ptr()),
	Smuggle(c"if you leave, i'll make\nthe lord drink my jizz.".as_ptr()),
	Smuggle(c"hey, ron! can we say\n'fuck' in the game?".as_ptr()),
	Smuggle(c"i'd leave: this is just\nmore monsters and levels.\nwhat a load.".as_ptr()),
	Smuggle(c"suck it down, asshole!\nyou're a fucking wimp!".as_ptr()),
	Smuggle(c"don't quit now! we're \nstill spending your money!".as_ptr()),
	// Internal debug. Different style, too.
	Smuggle(c"THIS IS NO MESSAGE!\nPage intentionally left blank.".as_ptr()),
];
