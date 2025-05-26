use std::ffi::c_char;

unsafe extern "C" {
	pub fn I_Error(error: *const c_char, ...);
}
