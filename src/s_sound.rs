#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::{ffi::c_void, ptr::null_mut};

use crate::{
	doomdef::GameMode_t,
	doomstat::gamemode,
	g_game::{consoleplayer, gameepisode, gamemap, players},
	i_system::I_Error,
	m_fixed::{FRACBITS, FixedMul, fixed_t},
	m_random::M_Random,
	p_mobj::mobj_t,
	sounds::{S_music, S_sfx, musicenum_t, musicinfo_t, sfxenum_t, sfxinfo_t},
	tables::{ANGLETOFINESHIFT, finesine},
	w_wad::{W_CacheLumpNum, W_GetNumForName},
	z_zone::{PU_CACHE, PU_MUSIC, PU_STATIC, Z_ChangeTag, Z_Malloc},
};

type boolean = i32;

// when to clip out sounds
// Does not fit the large outdoor areas.
const S_CLIPPING_DIST: i32 = 1200 * 0x10000;

// Distance tp origin when sounds should be maxed out.
// This should relate to movement clipping resolution
// (see BLOCKMAP handling).
// Originally: (200*0x10000).
const S_CLOSE_DIST: i32 = 160 * 0x10000;

const S_ATTENUATOR: i32 = (S_CLIPPING_DIST - S_CLOSE_DIST) >> FRACBITS;

// Adjustable by menu.

const NORM_PITCH: i32 = 128;
const NORM_PRIORITY: i32 = 64;
const NORM_SEP: i32 = 128;

const S_STEREO_SWING: fixed_t = 96 * 0x10000;

struct channel_t {
	// sound information (if null, channel avail.)
	pub sfxinfo: *mut sfxinfo_t,

	// origin of sound
	pub origin: *mut c_void,

	// handle of the sound being played
	pub handle: i32,
}

// the set of channels available
static mut channels: *mut channel_t = null_mut();

// These are not used, but should be (menu).
// Maximum volume of a sound effect.
// Internal default is max out of 0-15.
#[unsafe(no_mangle)]
pub static mut snd_SfxVolume: i32 = 15;

// Maximum volume of music. Useless so far.
#[unsafe(no_mangle)]
pub static mut snd_MusicVolume: i32 = 15;

// whether songs are mus_paused
static mut mus_paused: boolean = 0;

// music currently being played
static mut mus_playing: *mut musicinfo_t = null_mut();

// following is set
//  by the defaults code in M_misc:
// number of channels available
#[unsafe(no_mangle)]
pub static mut numChannels: usize = 0;

static mut nextcleanup: i32 = 0;

unsafe extern "C" {
	fn I_SetChannels();
}

// Initializes sound stuff, including volume
// Sets channels, SFX and music volume,
//  allocates channel buffer, sets S_sfx lookup.
pub(crate) fn S_Init(sfxVolume: i32, musicVolume: i32) {
	unsafe {
		eprintln!("S_Init: default sfx volume {sfxVolume}");

		// Whatever these did with DMX, these are rather dummies now.
		I_SetChannels();

		S_SetSfxVolume(sfxVolume);
		// No music with Linux - another dummy.
		S_SetMusicVolume(musicVolume);

		// Allocating the internal channels for mixing
		// (the maximum numer of sounds rendered
		// simultaneously) within zone memory.
		channels = Z_Malloc(numChannels * size_of::<channel_t>(), PU_STATIC, null_mut()).cast();

		// Free all channels for use
		for i in 0..numChannels {
			(*channels.wrapping_add(i)).sfxinfo = null_mut();
		}

		// no sounds are playing, and they are not mus_paused
		mus_paused = 0;

		// Note that sounds have not been cached (yet).
		#[allow(clippy::needless_range_loop)]
		for i in 1..sfxenum_t::NUMSFX as usize {
			S_sfx[i].lumpnum = -1;
			S_sfx[i].usefulness = -1;
		}
	}
}

// Per level startup code.
// Kills playing sounds at start of level,
//  determines music if any, changes music.
pub(crate) fn S_Start() {
	unsafe {
		// kill all playing sounds at start of level
		//  (trust me - a good idea)
		for cnum in 0..numChannels {
			if !(*channels.wrapping_add(cnum)).sfxinfo.is_null() {
				S_StopChannel(cnum);
			}
		}

		// start new music for the level
		mus_paused = 0;

		let mnum;
		if gamemode == GameMode_t::commercial {
			mnum = (musicenum_t::mus_runnin as usize + gamemap - 1).into();
		} else {
			const spmus: [musicenum_t; 9] = [
				// Song - Who? - Where?
				musicenum_t::mus_e3m4, // American	e4m1
				musicenum_t::mus_e3m2, // Romero	e4m2
				musicenum_t::mus_e3m3, // Shawn	e4m3
				musicenum_t::mus_e1m5, // American	e4m4
				musicenum_t::mus_e2m7, // Tim 	e4m5
				musicenum_t::mus_e2m4, // Romero	e4m6
				musicenum_t::mus_e2m6, // J.Anderson	e4m7 CHIRON.WAD
				musicenum_t::mus_e2m5, // Shawn	e4m8
				musicenum_t::mus_e1m9, // Tim		e4m9
			];

			if gameepisode < 4 {
				mnum =
					(musicenum_t::mus_e1m1 as usize + (gameepisode - 1) * 9 + gamemap - 1).into();
			} else {
				mnum = spmus[gamemap - 1];
			}
		}

		// HACK FOR COMMERCIAL
		//  if (commercial && mnum > mus_e3m9)
		//      mnum -= mus_e3m9;

		S_ChangeMusic(mnum, 1);

		nextcleanup = 15;
	}
}

unsafe extern "C" {
	fn I_StartSound(id: sfxenum_t, vol: i32, sep: i32, pitch: i32, priority: i32) -> i32;
	fn I_GetSfxLumpNum(sfx: *mut sfxinfo_t) -> i32;
}

fn S_StartSoundAtVolume(origin_p: *mut c_void, sfx_id: sfxenum_t, mut volume: i32) {
	unsafe {
		let origin = origin_p as *mut mobj_t;

		// Debug.
		/*fprintf( stderr,
		"S_StartSoundAtVolume: playing sound %d (%s)\n",
		sfx_id, S_sfx[sfx_id].name );*/

		// check for bogus sound #
		if (sfx_id as usize) < 1 || sfx_id as usize > sfxenum_t::NUMSFX as usize {
			I_Error(c"Bad sfx #: %d".as_ptr(), sfx_id);
		}

		let sfx = &mut S_sfx[sfx_id as usize];

		// Initialize sound parameters
		let mut pitch;
		let priority;
		if !sfx.link.is_null() {
			pitch = sfx.pitch;
			priority = sfx.priority;
			volume += sfx.volume;

			if volume < 1 {
				return;
			}
			if volume > snd_SfxVolume {
				volume = snd_SfxVolume;
			}
		} else {
			pitch = NORM_PITCH;
			priority = NORM_PRIORITY;
		}

		// Check to see if it is audible,
		//  and if not, modify the params
		let mut sep = 0;
		if !origin.is_null() && !std::ptr::eq(origin, players[consoleplayer].mo) {
			let rc = S_AdjustSoundParams(
				&mut *players[consoleplayer].mo,
				&mut *origin,
				&mut volume,
				&mut sep,
				&mut pitch,
			);

			if (*origin).x == (*players[consoleplayer].mo).x
				&& (*origin).y == (*players[consoleplayer].mo).y
			{
				sep = NORM_SEP;
			}

			if rc == 0 {
				return;
			}
		} else {
			sep = NORM_SEP;
		}

		// hacks to vary the sfx pitches
		if sfx_id as usize >= sfxenum_t::sfx_sawup as usize
			&& sfx_id as usize <= sfxenum_t::sfx_sawhit as usize
		{
			pitch += 8 - (M_Random() & 15);
			pitch = pitch.clamp(0, 255);
		} else if sfx_id as usize != sfxenum_t::sfx_itemup as usize
			&& sfx_id as usize != sfxenum_t::sfx_tink as usize
		{
			pitch += 16 - (M_Random() & 31);
			pitch = pitch.clamp(0, 255);
		}

		// kill old sound
		S_StopSound(origin.cast());

		// try to find a channel
		let cnum = S_getChannel(origin.cast(), sfx);

		if cnum < 0 {
			return;
		}

		// This is supposed to handle the loading/caching.
		// For some odd reason, the caching is done nearly
		//  each time the sound is needed?

		// get lumpnum if necessary
		if sfx.lumpnum < 0 {
			sfx.lumpnum = I_GetSfxLumpNum(sfx);
		}

		// increase the usefulness
		if sfx.usefulness < 0 {
			sfx.usefulness = 1;
		} else {
			sfx.usefulness += 1;
		}

		// Assigns the handle to one of the channels in the
		//  mix/output buffer.
		(*channels.wrapping_offset(cnum)).handle =
			I_StartSound(sfx_id, /*sfx.data,*/ volume, sep, pitch, priority);
	}
}

// Start sound for thing at <origin>
//  using <sound_id> from sounds.h
#[unsafe(no_mangle)]
pub extern "C" fn S_StartSound(origin: *mut c_void, sfx_id: sfxenum_t) {
	unsafe {
		S_StartSoundAtVolume(origin, sfx_id, snd_SfxVolume);
	}
}

// Stop sound for thing at <origin>
pub(crate) fn S_StopSound(origin: *mut c_void) {
	unsafe {
		for cnum in 0..numChannels {
			if !(*channels.wrapping_add(cnum)).sfxinfo.is_null()
				&& (*channels.wrapping_add(cnum)).origin == origin
			{
				S_StopChannel(cnum);
				break;
			}
		}
	}
}

unsafe extern "C" {
	fn I_PauseSong(handle: i32);
	fn I_ResumeSong(handle: i32);
}

// Stop and resume music, during game PAUSE.
pub(crate) fn S_PauseSound() {
	unsafe {
		if !mus_playing.is_null() && mus_paused == 0 {
			I_PauseSong((*mus_playing).handle);
			mus_paused = 1;
		}
	}
}

pub(crate) fn S_ResumeSound() {
	unsafe {
		if !mus_playing.is_null() && mus_paused != 0 {
			I_ResumeSong((*mus_playing).handle);
			mus_paused = 0;
		}
	}
}

unsafe extern "C" {
	fn I_SoundIsPlaying(handle: i32) -> boolean;
	fn I_UpdateSoundParams(handle: i32, vol: i32, sep: i32, pitch: i32);
}

// Updates music & sounds
pub(crate) fn S_UpdateSounds(listener_p: *mut c_void) {
	unsafe {
		for cnum in 0..numChannels {
			let c = &mut *channels.wrapping_add(cnum);

			if let Some(sfx) = c.sfxinfo.as_ref() {
				if I_SoundIsPlaying(c.handle) != 0 {
					// initialize parameters
					let mut volume = snd_SfxVolume;
					let mut pitch = NORM_PITCH;
					let mut sep = NORM_SEP;

					if !sfx.link.is_null() {
						pitch = sfx.pitch;
						volume += sfx.volume;
						if volume < 1 {
							S_StopChannel(cnum);
							continue;
						} else if volume > snd_SfxVolume {
							volume = snd_SfxVolume;
						}
					}

					// check non-local sounds for distance clipping
					//  or modify their params
					if !c.origin.is_null() && !std::ptr::eq(listener_p, c.origin) {
						let audible = S_AdjustSoundParams(
							&mut *listener_p.cast(),
							&mut *c.origin.cast(),
							&mut volume,
							&mut sep,
							&mut pitch,
						);

						if audible == 0 {
							S_StopChannel(cnum);
						} else {
							I_UpdateSoundParams(c.handle, volume, sep, pitch);
						}
					}
				} else {
					// if channel is allocated but sound has stopped,
					//  free it
					S_StopChannel(cnum);
				}
			}
		}
	}
}

unsafe extern "C" {
	fn I_SetMusicVolume(volume: i32);
}

#[unsafe(no_mangle)]
pub extern "C" fn S_SetMusicVolume(volume: i32) {
	unsafe {
		if !(0..=127).contains(&volume) {
			I_Error(c"Attempt to set music volume at %d".as_ptr(), volume);
		}

		I_SetMusicVolume(127);
		I_SetMusicVolume(volume);
		snd_MusicVolume = volume;
	}
}

#[unsafe(no_mangle)]
pub extern "C" fn S_SetSfxVolume(volume: i32) {
	unsafe {
		if !(0..=127).contains(&volume) {
			I_Error(c"Attempt to set sfx volume at %d".as_ptr(), volume);
		}

		snd_SfxVolume = volume;
	}
}

// Starts some music with the music id found in sounds.h.
#[unsafe(no_mangle)]
pub extern "C" fn S_StartMusic(m_id: musicenum_t) {
	S_ChangeMusic(m_id, 0);
}

unsafe extern "C" {
	fn I_RegisterSong(data: *mut c_void) -> i32;
	fn I_PlaySong(handle: i32, looping: boolean);
}

// Start music using <music_id> from sounds.h,
//  and set whether looping
#[unsafe(no_mangle)]
pub extern "C" fn S_ChangeMusic(musicnum: musicenum_t, looping: boolean) {
	unsafe {
		let music = if (musicnum as usize) <= musicenum_t::mus_None as usize
			|| musicnum as usize >= musicenum_t::NUMMUSIC as usize
		{
			I_Error(c"Bad music number %d".as_ptr(), musicnum);
		} else {
			&mut S_music[musicnum as usize]
		};

		if mus_playing == music {
			return;
		}

		// shutdown old music
		S_StopMusic();

		// get lumpnum if neccessary
		if music.lumpnum != 0 {
			let mut namebuf = [0; 9];
			libc::sprintf(namebuf.as_mut_ptr(), c"d_%s".as_ptr(), music.name);
			music.lumpnum = W_GetNumForName(namebuf.as_ptr()) as usize;
		}

		// load & register it
		music.data = W_CacheLumpNum(music.lumpnum, PU_MUSIC).cast();
		music.handle = I_RegisterSong(music.data);

		// play it
		I_PlaySong(music.handle, looping);

		mus_playing = music;
	}
}

unsafe extern "C" {
	fn I_UnRegisterSong(handle: i32);
	fn I_StopSong(handle: i32);
}

fn S_StopMusic() {
	unsafe {
		if !mus_playing.is_null() {
			if mus_paused != 0 {
				I_ResumeSong((*mus_playing).handle);
			}

			I_StopSong((*mus_playing).handle);
			I_UnRegisterSong((*mus_playing).handle);
			Z_ChangeTag!((*mus_playing).data, PU_CACHE);

			(*mus_playing).data = null_mut();
			mus_playing = null_mut();
		}
	}
}

unsafe extern "C" {
	fn I_StopSound(handle: i32);
}

fn S_StopChannel(cnum: usize) {
	unsafe {
		let c = channels.wrapping_add(cnum);

		if !(*c).sfxinfo.is_null() {
			// stop the sound playing
			if I_SoundIsPlaying((*c).handle) != 0 {
				I_StopSound((*c).handle);
			}

			// check to see
			//  if other channels are playing the sound
			for i in 0..numChannels {
				if cnum != i && std::ptr::eq((*c).sfxinfo, (*channels.wrapping_add(i)).sfxinfo) {
					break;
				}
			}

			// degrade usefulness of sound data
			(*(*c).sfxinfo).usefulness -= 1;

			(*c).sfxinfo = null_mut();
		}
	}
}

unsafe extern "C" {
	fn R_PointToAngle2(x_1: i32, y_1: i32, x_2: i32, y_2: i32) -> u32;
}

// Changes volume, stereo-separation, and pitch variables
//  from the norm of a sound effect to be played.
// If the sound is not audible, returns a 0.
// Otherwise, modifies parameters and returns 1.
fn S_AdjustSoundParams(
	listener: &mut mobj_t,
	source: &mut mobj_t,
	vol: *mut i32,
	sep: *mut i32,
	_pitch: *mut i32,
) -> i32 {
	unsafe {
		// calculate the distance to sound origin
		//  and clip it if necessary
		let adx = i32::abs(listener.x - source.x);
		let ady = i32::abs(listener.y - source.y);

		// From _GG1_ p.428. Appox. eucledian distance fast.
		let mut approx_dist = adx + ady - ((adx.min(ady)) >> 1);

		if gamemap != 8 && approx_dist > S_CLIPPING_DIST {
			return 0;
		}

		// angle of source to listener
		let mut angle = R_PointToAngle2(listener.x, listener.y, source.x, source.y);

		if angle > listener.angle {
			angle -= listener.angle;
		} else {
			angle += 0xffffffff - listener.angle;
		}

		angle >>= ANGLETOFINESHIFT;

		// stereo separation
		*sep = 128 - (FixedMul(S_STEREO_SWING, finesine[angle as usize]) >> FRACBITS);

		// volume calculation
		if approx_dist < S_CLOSE_DIST {
			*vol = snd_SfxVolume;
		} else if gamemap == 8 {
			if approx_dist > S_CLIPPING_DIST {
				approx_dist = S_CLIPPING_DIST;
			}

			*vol = 15
				+ ((snd_SfxVolume - 15) * ((S_CLIPPING_DIST - approx_dist) >> FRACBITS))
					/ S_ATTENUATOR;
		} else {
			// distance effect
			*vol = (snd_SfxVolume * ((S_CLIPPING_DIST - approx_dist) >> FRACBITS)) / S_ATTENUATOR;
		}

		(*vol > 0) as i32
	}
}

// S_getChannel :
//   If none available, return -1.  Otherwise channel #.
fn S_getChannel(origin: *mut c_void, sfxinfo: *mut sfxinfo_t) -> isize {
	unsafe {
		// channel number to use
		// Find an open channel
		let mut cnum = 0;
		for i in 0..numChannels {
			cnum = i;
			if (*channels.wrapping_add(cnum)).sfxinfo.is_null() {
				break;
			} else if !origin.is_null() && (*channels.wrapping_add(cnum)).origin == origin {
				S_StopChannel(cnum);
				break;
			}
		}
		// None available
		if cnum == numChannels {
			// Look for lower priority
			for cnum in 0..numChannels {
				if (*(*channels.wrapping_add(cnum)).sfxinfo).priority >= (*sfxinfo).priority {
					break;
				}
			}

			if cnum == numChannels {
				// FUCK!  No lower priority.  Sorry, Charlie.
				return -1;
			} else {
				// Otherwise, kick out lower priority.
				S_StopChannel(cnum);
			}
		}

		let c = &mut *channels.wrapping_add(cnum);

		// channel is decided to be cnum.
		c.sfxinfo = sfxinfo;
		c.origin = origin;

		cnum as isize
	}
}
