pub mod doomstat;

use std::{
    env,
    ffi::{CString, c_char},
};

unsafe extern "C" {
    static mut myargc: i32;
    static mut myargv: *const *const c_char;

    fn D_DoomMain();
}

fn main() {
    let args: Vec<_> = env::args().map(|arg| CString::new(arg).unwrap()).collect();
    let argv: Vec<_> = args.iter().map(|cstring| cstring.as_ptr()).collect();
    unsafe {
        myargc = args.len() as i32;
        myargv = argv.as_ptr();
        D_DoomMain();
    }
}
