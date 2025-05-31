use crate::doomdef::{SCREENHEIGHT, SCREENWIDTH, SCREEN_MUL};

pub const ST_HEIGHT: usize = 32 * SCREEN_MUL;
pub const ST_WIDTH: usize = SCREENWIDTH;
pub const ST_Y: usize = SCREENHEIGHT - ST_HEIGHT;
