#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::ffi::c_void;

// Experimental stuff.
// To compile this as "ANSI C with classes"
//  we will need to handle the various
//  action functions cleanly.
pub type actionf_v = unsafe extern "C" fn();
pub type actionf_p1 = unsafe extern "C" fn(*mut c_void);
pub type actionf_p2 = unsafe extern "C" fn(*mut c_void, *mut c_void);

#[repr(C)]
pub union actionf_t {
	pub acp1: Option<actionf_p1>,
	pub acv: Option<actionf_v>,
	pub acp2: Option<actionf_p2>,
}

// Historically, "think_t" is yet another
//  function pointer to a routine to handle
//  an actor.
type think_t = actionf_t;

// Doubly linked list of actors.
#[repr(C)]
pub struct thinker_t {
	pub prev: *mut thinker_t,
	pub next: *mut thinker_t,
	pub function: think_t,
}
