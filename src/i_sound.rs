#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::{
	ffi::{c_int, c_void},
	process::exit,
	ptr::null_mut,
	time::Duration,
};

use libc::{O_WRONLY, c_char, ioctl};

use crate::{
	const_conv::*,
	doomdef::TICRATE,
	g_game::gametic,
	i_system::I_Error,
	s_sound::snd_MusicVolume,
	sounds::{S_sfx, sfxenum_t, sfxinfo_t},
	w_wad::{W_CacheLumpNum, W_CheckNumForName, W_GetNumForName, W_LumpLength},
	z_zone::{PU_STATIC, Z_Free, Z_Malloc},
};

const SOUND_INTERVAL: i32 = 50;

// A quick hack to establish a protocol between
// synchronous mix buffer updates and asynchronous
// audio writes. Probably redundant with gametic.
static mut flag: usize = 0;

// The number of internal mixing channels,
//  the samples calculated for each mixing step,
//  the size of the 16bit, 2 hardware channel (stereo)
//  mixing buffer, and the samplerate of the raw data.

// Needed for calling the actual sound output.
const SAMPLECOUNT: usize = 512;
const NUM_CHANNELS: usize = 8;
// It is 2 for 16bit, and 2 for two channels.
const BUFMUL: usize = 4;
const MIXBUFFERSIZE: usize = SAMPLECOUNT * BUFMUL;

const SAMPLERATE: usize = 11025; // Hz
const SAMPLESIZE: usize = 2; // 16bit

// The actual lengths of all sound effects.
static mut lengths: [usize; sfxenum_t::NUMSFX.to_usize()] = [0; sfxenum_t::NUMSFX.to_usize()];

// The actual output device.
static mut audio_fd: i32 = 0;

// The global mixing buffer.
// Basically, samples from all active internal channels
//  are modifed and added, and stored in the buffer
//  that is submitted to the audio device.
static mut mixbuffer: [i16; MIXBUFFERSIZE] = [0; MIXBUFFERSIZE];

// The channel step amount...
static mut channelstep: [u32; NUM_CHANNELS] = [0; NUM_CHANNELS];
// ... and a 0.16 bit remainder of last step.
static mut channelstepremainder: [u32; NUM_CHANNELS] = [0; NUM_CHANNELS];

// The channel data pointers, start and end.
static mut channels: [*mut u8; NUM_CHANNELS] = [null_mut(); NUM_CHANNELS];
static mut channelsend: [*mut u8; NUM_CHANNELS] = [null_mut(); NUM_CHANNELS];

// Time/gametic that the channel started playing,
//  used to determine oldest, which automatically
//  has lowest priority.
// In case number of active sounds exceeds
//  available channels.
static mut channelstart: [usize; NUM_CHANNELS] = [0; NUM_CHANNELS];

// The sound in channel handles,
//  determined on registration,
//  might be used to unregister/stop/modify,
//  currently unused.
static mut channelhandles: [u16; NUM_CHANNELS] = [0; NUM_CHANNELS];

// SFX id of the playing sound effect.
// Used to catch duplicates (like chainsaw).
static mut channelids: [sfxenum_t; NUM_CHANNELS] = [sfxenum_t::sfx_None; NUM_CHANNELS];

// Pitch to stepping lookup, unused.
static mut steptable: [u32; 256] = [0; 256];

// Volume lookups.
static mut vol_lookup: [i32; 128 * 256] = [0; 128 * 256];

// Hardware left and right channel volume lookup.
static mut channelleftvol_lookup: [*mut i32; NUM_CHANNELS] = [null_mut(); NUM_CHANNELS];
static mut channelrightvol_lookup: [*mut i32; NUM_CHANNELS] = [null_mut(); NUM_CHANNELS];

// Safe ioctl, convenience.
fn myioctl(fd: i32, command: u32, arg: *mut i32) {
	unsafe {
		let rc = ioctl(fd, command, arg);
		if rc < 0 {
			eprintln!("ioctl(dsp,{:x},arg) failed", command);
			exit(-1)
		}
	}
}

// This function loads the sound data from the WAD lump,
//  for single sound.
fn getsfx(sfxname: *const c_char, len: *mut usize) -> *mut c_void {
	unsafe {
		// Get the sound data from the WAD, allocate lump
		//  in zone memory.
		let mut name = [0; 20];
		libc::sprintf(name.as_mut_ptr(), c"ds%s".as_ptr(), sfxname);

		// Now, there is a severe problem with the
		//  sound handling, in it is not (yet/anymore)
		//  gamemode aware. That means, sounds from
		//  DOOM II will be requested even with DOOM
		//  shareware.
		// The sound list is wired into sounds.c,
		//  which sets the external variable.
		// I do not do runtime patches to that
		//  variable. Instead, we will use a
		//  default sound for replacement.
		let sfxlump = if W_CheckNumForName(name.as_ptr()) == -1 {
			usize::try_from(W_GetNumForName(c"dspistol".as_ptr())).unwrap()
		} else {
			usize::try_from(W_GetNumForName(name.as_ptr())).unwrap()
		};

		let size = W_LumpLength(sfxlump);

		// Debug.
		// fprintf( stderr, "." );
		//fprintf( stderr, " -loading  %s (lump %d, %d bytes)\n",
		//		 sfxname, sfxlump, size );
		//fflush( stderr );

		let sfx = W_CacheLumpNum(sfxlump, PU_STATIC).cast();

		// Pads the sound effect out to the mixing buffer size.
		// The original realloc would interfere with zone memory.
		let paddedsize = (size - 8).next_multiple_of(SAMPLECOUNT);

		// Allocate from zone memory.
		let paddedsfx = Z_Malloc(paddedsize + 8, PU_STATIC, null_mut()).cast::<u8>();
		// ddt: (unsigned char *) realloc(sfx, paddedsize+8);
		// This should interfere with zone memory handling,
		//  which does not kick in in the soundserver.

		// Now copy and pad.
		libc::memcpy(paddedsfx.cast(), sfx, size);
		for i in size..paddedsize + 8 {
			*paddedsfx.wrapping_add(i) = 128;
		}

		// Remove the cached lump.
		Z_Free(sfx);

		// Preserve padded length.
		*len = paddedsize;

		// Return allocated padded data.
		paddedsfx.wrapping_add(8).cast()
	}
}

// This function adds a sound to the
//  list of currently active sounds,
//  which is maintained as a given number
//  (eight, usually) of internal channels.
// Returns a handle.
fn addsfx(sfxid: sfxenum_t, volume: u32, step: u32, mut seperation: i32) -> usize {
	unsafe {
		static mut handlenums: u16 = 0;

		// Chainsaw troubles.
		// Play these sound effects only one at a time.
		if sfxid == sfxenum_t::sfx_sawup
			|| sfxid == sfxenum_t::sfx_sawidl
			|| sfxid == sfxenum_t::sfx_sawful
			|| sfxid == sfxenum_t::sfx_sawhit
			|| sfxid == sfxenum_t::sfx_stnmov
			|| sfxid == sfxenum_t::sfx_pistol
		{
			// Loop all channels, check.
			for i in 0..NUM_CHANNELS {
				// Active, and using the same SFX?
				if !channels[i].is_null() && channelids[i] == sfxid {
					// Reset.
					channels[i] = null_mut();
					// We are sure that iff,
					//  there will only be one.
					break;
				}
			}
		}

		// Loop all channels to find oldest SFX.
		let mut i = 0;
		let mut oldest = gametic;
		let mut oldestnum = 0;
		while i < NUM_CHANNELS && !channels[i].is_null() {
			if channelstart[i] < oldest {
				oldestnum = i;
				oldest = channelstart[i];
			}
			i += 1;
		}

		// Tales from the cryptic.
		// If we found a channel, fine.
		// If not, we simply overwrite the first one, 0.
		// Probably only happens at startup.
		let slot = if i == NUM_CHANNELS { oldestnum } else { i };

		// Okay, in the less recent channel,
		//  we will handle the new SFX.
		// Set pointer to raw data.
		channels[slot] = S_sfx[usize::from(sfxid)].data.cast();
		// Set pointer to end of raw data.
		channelsend[slot] = channels[slot].wrapping_add(lengths[usize::from(sfxid)]);

		// Reset current handle number, limited to 0..100.
		if handlenums == 0 {
			handlenums = 100;
		}

		// Assign current handle number.
		// Preserved so sounds could be stopped (unused).
		channelhandles[slot] = handlenums;
		let rc = handlenums;
		handlenums += 1;

		// Set stepping???
		// Kinda getting the impression this is never used.
		channelstep[slot] = step;
		// ???
		channelstepremainder[slot] = 0;
		// Should be gametic, I presume.
		channelstart[slot] = gametic;

		// Separation, that is, orientation/stereo.
		//  range is: 1 - 256
		seperation += 1;

		// Per left/right channel.
		//  x^2 seperation,
		//  adjust volume properly.
		let leftvol = volume.saturating_sub((volume * seperation.unsigned_abs().pow(2)) >> 16); // /(256*256);
		seperation -= 257;
		let rightvol = volume.saturating_sub((volume * seperation.unsigned_abs().pow(2)) >> 16);

		// Sanity check, clamp volume.
		if rightvol > 127 {
			I_Error!(c"rightvol out of bounds".as_ptr());
		}

		if leftvol > 127 {
			I_Error!(c"leftvol out of bounds".as_ptr());
		}

		// Get the proper lookup table piece
		//  for this volume level???
		channelleftvol_lookup[slot] = &raw mut vol_lookup[usize::try_from(leftvol).unwrap() * 256];
		channelrightvol_lookup[slot] =
			&raw mut vol_lookup[usize::try_from(rightvol).unwrap() * 256];

		// Preserve sound SFX id,
		//  e.g. for avoiding duplicates of chainsaw.
		channelids[slot] = sfxid;

		// You tell me.
		rc.into()
	}
}

// SFX API
// Note: this was called by S_Init.
// However, whatever they did in the
// old DPMS based DOS version, this
// were simply dummies in the Linux
// version.
// See soundserver initdata().
pub(crate) fn I_SetChannels() {
	unsafe {
		// Init internal lookups (raw data, mixing buffer, channels).
		// This function sets up internal lookups used during
		//  the mixing process.

		let steptablemid = steptable[128..].as_mut_ptr();

		// This table provides step widths for pitch parameters.
		// I fail to see that this is currently used.
		#[allow(clippy::as_conversions)]
		for i in -128..128 {
			*steptablemid.offset(i) = (f32::powf(2.0, i as f32 / 64.0) * 65536.0) as u32;
		}

		// Generates volume lookup tables
		//  which also turn the unsigned samples
		//  into signed samples.
		for i in 0..128 {
			for j in 0..256 {
				vol_lookup[usize::try_from(i * 256 + j).unwrap()] = (i * (j - 128) * 256) / 127;
			}
		}
	}
}

// MUSIC API - dummy. Some code from DOS version.
pub(crate) fn I_SetMusicVolume(volume: u32) {
	// Internal state variable.
	unsafe { snd_MusicVolume = volume };
	// Now set volume on output device.
	// Whatever( snd_MusciVolume );
}

// Retrieve the raw data lump index
//  for a given SFX name.
pub(crate) unsafe fn I_GetSfxLumpNum(sfx: *mut sfxinfo_t) -> isize {
	unsafe {
		let mut namebuf = [0; 9];
		libc::sprintf(namebuf.as_mut_ptr(), c"ds%s".as_ptr(), (*sfx).name);
		W_GetNumForName(namebuf.as_ptr())
	}
}

// Starting a sound means adding it
//  to the current list of active sounds
//  in the internal channels.
// As the SFX info struct contains
//  e.g. a pointer to the raw data,
//  it is ignored.
// As our sound handling does not handle
//  priority, it is ignored.
// Pitching (that is, increased speed of playback)
//  is set, but currently not used by mixing.
pub(crate) fn I_StartSound(id: sfxenum_t, vol: u32, sep: i32, pitch: i32, _priority: i32) -> usize {
	unsafe { addsfx(id, vol, steptable[usize::try_from(pitch).unwrap()], sep) }
}

pub(crate) fn I_StopSound(_handle: usize) {
	// You need the handle returned by StartSound.
	// Would be looping all channels,
	//  tracking down the handle,
	//  an setting the channel to zero.
}

pub(crate) fn I_SoundIsPlaying(handle: usize) -> bool {
	// Ouch.
	unsafe { gametic < handle }
}

// This function loops all active (internal) sound
//  channels, retrieves a given number of samples
//  from the raw sound data, modifies it according
//  to the current (internal) channel parameters,
//  mixes the per channel samples into the global
//  mixbuffer, clamping it to the allowed range,
//  and sets up everything for transferring the
//  contents of the mixbuffer to the (two)
//  hardware channels (left and right, that is).
//
// This function currently supports only 16bit.
#[allow(static_mut_refs)]
pub(crate) fn I_UpdateSound() {
	unsafe {
		static mut misses: usize = 0;

		// Mix current sound data.
		// Left and right channel
		//  are in global mixbuffer, alternating.
		let mut leftout = mixbuffer.as_mut_ptr();
		let mut rightout = mixbuffer.as_mut_ptr().offset(1);
		let step = SAMPLESIZE;

		// Determine end, for left channel only
		//  (right channel is implicit).
		let leftend = mixbuffer.as_mut_ptr().wrapping_add(SAMPLECOUNT * step);

		// Mix sounds into the mixing buffer.
		// Loop over step*SAMPLECOUNT,
		//  that is 512 values for two channels.
		while leftout != leftend {
			// Reset left/right value.
			let mut dl = 0;
			let mut dr = 0;

			// Love thy L2 chache - made this a loop.
			// Now more channels could be set at compile time
			//  as well. Thus loop those  channels.
			for chan in 0..NUM_CHANNELS {
				// Check channel, if active.
				if !channels[chan].is_null() {
					// Get the raw data from the channel.
					let sample = usize::from(*channels[chan]);
					// Add left and right part
					//  for this channel (sound)
					//  to the current data.
					// Adjust volume accordingly.
					dl += *channelleftvol_lookup[chan].wrapping_add(sample);
					dr += *channelrightvol_lookup[chan].wrapping_add(sample);
					// Increment index ???
					channelstepremainder[chan] += channelstep[chan];
					// MSB is next sample???
					channels[chan] = channels[chan]
						.wrapping_add(usize::try_from(channelstepremainder[chan]).unwrap() >> 16);
					// Limit to LSB???
					channelstepremainder[chan] &= 65536 - 1;

					// Check whether we are done.
					if channels[chan] >= channelsend[chan] {
						channels[chan] = null_mut();
					}
				}
			}

			// Clamp to range. Left hardware channel.
			// Has been char instead of short.
			// if (dl > 127) *leftout = 127;
			// else if (dl < -128) *leftout = -128;
			// else *leftout = dl;

			if dl > 0x7fff {
				*leftout = 0x7fff;
			} else if dl < -0x8000 {
				*leftout = -0x8000;
			} else {
				*leftout = i16::try_from(dl).unwrap();
			}

			// Same for right hardware channel.
			if dr > 0x7fff {
				*rightout = 0x7fff;
			} else if dr < -0x8000 {
				*rightout = -0x8000;
			} else {
				*rightout = i16::try_from(dr).unwrap();
			}

			// Increment current pointers in mixbuffer.
			leftout = leftout.wrapping_add(step);
			rightout = rightout.wrapping_add(step);
		}

		// Debug check.
		if flag != 0 {
			misses += flag;
			flag = 0;
		}

		if misses > 10 {
			eprintln!("I_SoundUpdate: missed 10 buffer writes");
			misses = 0;
		}

		// Increment flag for update.
		flag += 1;
	}
}

// This would be used to write out the mixbuffer
//  during each game loop update.
// Updates sound buffer and audio device at runtime.
// It is called during Timer interrupt with SNDINTR.
// Mixing now done synchronous, and
//  only output be done asynchronous?
#[expect(unused, reason = "used in unimplemented functions")]
#[allow(static_mut_refs)]
pub(crate) fn I_SubmitSound() {
	// Write it to DSP device.
	unsafe { libc::write(audio_fd, mixbuffer.as_ptr().cast(), SAMPLECOUNT * BUFMUL) };
}

pub(crate) fn I_UpdateSoundParams(_handle: usize, _vol: u32, _sep: i32, _pitch: i32) {
	// I fail too see that this is used.
	// Would be using the handle to identify
	//  on which channel the sound might be active,
	//  and resetting the channel parameters.
}

pub(crate) fn I_ShutdownSound() {
	// Wait till all pending sounds are finished.
	let mut done = false;

	// FIXME (below).
	eprintln!("I_ShutdownSound: NOT finishing pending sounds");

	while !done {
		// for( i=0 ; i<8 && !channels[i] ; i++);

		// FIXME. No proper channel output.
		//if (i==8)
		done = true;
	}

	I_SoundDelTimer();

	// Cleaning up -releasing the DSP device.
	unsafe { libc::close(audio_fd) };
}

#[allow(static_mut_refs)]
pub(crate) fn I_InitSound() {
	unsafe {
		const SIOC_VOID: u32 = 0x0000_0000;
		const SIOC_OUT: u32 = 0x8000_0000;
		const SIOC_IN: u32 = 0x4000_0000;
		const SIOC_INOUT: u32 = SIOC_IN | SIOC_OUT;
		macro_rules! _SIO {
			($x:literal, $y:literal) => {
				SIOC_VOID | u32_from_u8($x) << 8 | u32_from_u8($y)
			};
		}
		macro_rules! _SIOR {
			($x:literal, $y:literal, $t:ty) => {
				SIOC_OUT
					| u32_from_usize(size_of::<$t>()) << 16
					| u32_from_u8($x) << 8
					| u32_from_u8($y)
			};
		}
		macro_rules! _SIOWR {
			($x:literal, $y:literal, $t:ty) => {
				SIOC_INOUT
					| u32_from_usize(size_of::<$t>()) << 16
					| u32_from_u8($x) << 8
					| u32_from_u8($y)
			};
		}
		const SNDCTL_DSP_SETFRAGMENT: u32 = _SIOWR!(b'P', 10, c_int);
		const SNDCTL_DSP_RESET: u32 = _SIO!(b'P', 0);
		const SNDCTL_DSP_SPEED: u32 = _SIOWR!(b'P', 2, c_int);
		const SNDCTL_DSP_STEREO: u32 = _SIOWR!(b'P', 3, c_int);
		const SNDCTL_DSP_GETFMTS: u32 = _SIOR!(b'P', 11, c_int);
		const SNDCTL_DSP_SETFMT: u32 = _SIOWR!(b'P', 5, c_int);

		const AFMT_S16_LE: i32 = 0x10;

		eprintln!("I_SoundSetTimer: {} microsecs", SOUND_INTERVAL);
		I_SoundSetTimer(SOUND_INTERVAL);

		// Secure and configure sound device first.
		eprint!("I_InitSound: ");

		audio_fd = libc::open(c"/dev/dsp".as_ptr(), O_WRONLY);
		if audio_fd < 0 {
			eprintln!("Could not open /dev/dsp");
			return;
		}

		let mut i = 11 | (2 << 16);
		myioctl(audio_fd, SNDCTL_DSP_SETFRAGMENT, &raw mut i);
		myioctl(audio_fd, SNDCTL_DSP_RESET, null_mut());

		let mut i = i32::try_from(SAMPLERATE).unwrap();

		myioctl(audio_fd, SNDCTL_DSP_SPEED, &raw mut i);

		let mut i = 1;
		myioctl(audio_fd, SNDCTL_DSP_STEREO, &raw mut i);

		myioctl(audio_fd, SNDCTL_DSP_GETFMTS, &raw mut i);
		i &= AFMT_S16_LE;

		if i != 0 {
			myioctl(audio_fd, SNDCTL_DSP_SETFMT, &raw mut i);
		} else {
			eprintln!("Could not play signed 16 data");
		}

		eprintln!(" configured audio device");

		// Initialize external data (all sounds) at start, keep static.
		eprint!("I_InitSound: ");

		for i in 1..usize::from(sfxenum_t::NUMSFX) {
			// Alias? Example is the chaingun sound linked to pistol.
			if S_sfx[i].link.is_null() {
				// Load data from WAD file.
				S_sfx[i].data = getsfx(S_sfx[i].name, &raw mut lengths[i]);
			} else {
				// Previously loaded already?
				S_sfx[i].data = (*S_sfx[i].link).data;
				lengths[i] = lengths[usize::try_from(S_sfx[i].link.offset_from(S_sfx.as_ptr()))
					.unwrap() / size_of::<sfxinfo_t>()];
			}
		}

		eprintln!(" pre-cached all sound data");

		// Now initialize mixbuffer with zero.

		mixbuffer = [0; MIXBUFFERSIZE];

		// Finished initialization.
		eprintln!("I_InitSound: sound module ready");
	}
}

// Still no music done.
// Remains. Dummies.
pub(crate) fn I_ShutdownMusic() {}

static mut looping: bool = false;
static mut musicdies: usize = usize::MAX;

pub(crate) fn I_PlaySong(_handle: i32, _loops: bool) {
	unsafe { musicdies = gametic + TICRATE * 30 }
}

pub(crate) fn I_PauseSong(_handle: i32) {}

pub(crate) fn I_ResumeSong(_handle: i32) {}

pub(crate) fn I_StopSong(_handle: i32) {
	unsafe {
		looping = false;
		musicdies = 0;
	}
}

pub(crate) fn I_UnRegisterSong(_handle: i32) {}

pub(crate) fn I_RegisterSong(_data: *mut c_void) -> i32 {
	1
}

/*
// Is the song playing?
int I_QrySongPlaying(int handle)
{
  // UNUSED.
  handle = 0;
  return looping || musicdies > gametic;
}
*/

// Interrupt handler.
#[allow(static_mut_refs)]
fn I_HandleSoundTimer(_: i32) {
	unsafe {
		// Debug.
		//fprintf( stderr, "%c", '+' ); fflush( stderr );

		// Feed sound device if necesary.
		if flag != 0 {
			// See I_SubmitSound().
			// Write it to DSP device.
			// TODO replace this with something that doesn't take 50ms to flush
			libc::write(audio_fd, mixbuffer.as_ptr().cast(), SAMPLECOUNT * BUFMUL);

			// Reset flag counter.
			flag = 0;
		}
	}
}

// Get the interrupt. Set duration in millisecs.
fn I_SoundSetTimer(duration_of_tick: i32) -> i32 {
	std::thread::spawn(move || {
		loop {
			std::thread::sleep(Duration::from_micros(u64::try_from(duration_of_tick).unwrap()));
			I_HandleSoundTimer(0);
		}
	});
	0
}

// Remove the interrupt. Set duration to zero.
fn I_SoundDelTimer() {
	// Debug.
	if I_SoundSetTimer(0) == -1 {
		eprintln!("I_SoundDelTimer: failed to remove interrupt. Doh!");
	}
}
