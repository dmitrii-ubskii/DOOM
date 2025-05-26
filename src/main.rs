#![allow(non_upper_case_globals)]

pub mod doomstat;
pub mod m_argv;

use std::{
    env,
    ffi::{CString, c_char},
    ptr::null,
};

#[unsafe(no_mangle)]
static mut myargc: i32 = 0;
#[unsafe(no_mangle)]
static mut myargv: *const *const c_char = null();

unsafe extern "C" {
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
