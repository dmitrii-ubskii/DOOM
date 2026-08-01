//	DOOM Network game communication and protocol,
//	all OS independend parts.
#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::{mem, num::Saturating, ptr::null_mut};

use libc::{c_char, fclose};

use crate::{
	d_event::*, d_main::*, d_ticcmd::ticcmd_t, doomdef::*, g_game::*, i_net::I_InitNetwork,
	i_system::*, i_video::*, m_menu::*,
};

type short = i16;
type unsigned = u32;
type byte = u8;

// Network play related stuff.
// There is a data struct that stores network
//  communication related stuff, and another
//  one that defines the actual packets to
//  be transmitted.

pub(crate) const DOOMCOM_ID: u32 = 0x12345678;

// Max computers/players in a game.
pub(crate) const MAXNETNODES: usize = 8;

// Networking and tick handling related.
pub(crate) const BACKUPTICS: usize = 12;

#[expect(unused, reason = "used in unimplemented functions")]
#[repr(i16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum command_t {
	CMD_SEND = 1,
	CMD_GET = 2,
}

impl From<command_t> for u8 {
	fn from(value: command_t) -> Self {
		match value {
			command_t::CMD_SEND => 1,
			command_t::CMD_GET => 2,
		}
	}
}

// Network packet data.
#[derive(Clone, Copy)]
pub(crate) struct doomdata_t {
	// High bit is retransmit request.
	checksum: unsigned,
	// Only valid if NCMD_RETRANSMIT.
	retransmitfrom: byte,

	starttic: byte,
	player: byte,
	numtics: byte,
	cmds: [ticcmd_t; BACKUPTICS],
}

#[expect(unused, reason = "used in unimplemented functions")]
pub(crate) struct doomcom_t {
	// Supposed to be DOOMCOM_ID?
	pub(crate) id: u32,

	// DOOM executes an int to execute commands.
	pub(crate) intnum: short,
	// Communication between DOOM and the driver.
	// Is CMD_SEND or CMD_GET.
	pub(crate) command: command_t,
	// Is dest for send, set by get (-1 = no packet).
	pub(crate) remotenode: u16,

	// Number of bytes in doomdata to be sent
	pub(crate) datalength: short,

	// Info common to all nodes.
	// Console is allways node 0.
	pub(crate) numnodes: u16,
	// Flag: 1 = no duplication, 2-5 = dup for slow nets.
	pub(crate) ticdup: u16,
	// Flag: 1 = send a backup tic in every packet.
	pub(crate) extratics: u16,
	// Flag: 1 = deathmatch.
	pub(crate) deathmatch: short,
	// Flag: -1 = new game, 0-5 = load savegame
	pub(crate) savegame: short,
	pub(crate) episode: short, // 1-3
	pub(crate) map: short,     // 1-9
	pub(crate) skill: short,   // 1-5

	// Info specific to this node.
	pub(crate) consoleplayer: u16,
	pub(crate) numplayers: u16,

	// These are related to the 3-display mode,
	//  in which two drones looking left and right
	//  were used to render two additional views
	//  on two additional computers.
	// Probably not operational anymore.
	// 1 = left, 0 = center, -1 = right
	pub(crate) angleoffset: short,
	// 1 = drone
	pub(crate) drone: short,

	// The packet data to be sent.
	pub(crate) data: doomdata_t,
}

const NCMD_EXIT: u32 = 0x80000000;
const NCMD_RETRANSMIT: u32 = 0x40000000;
const NCMD_SETUP: u32 = 0x20000000;
const NCMD_KILL: u32 = 0x10000000; // kill game
// const NCMD_CHECKSUM: u32 = 0x0fffffff;

pub(crate) static mut doomcom: *mut doomcom_t = null_mut();
pub(crate) static mut netbuffer: *mut doomdata_t = null_mut(); // points inside doomcom

// NETWORKING
//
// gametic is the tic about to (or currently being) run
// maketic is the tick that hasn't had control made for it yet
// nettics[] has the maketics for all players
//
// a gametic cannot be run until nettics[] > gametic for all players
const RESENDCOUNT: usize = 10;
const PL_DRONE: u8 = 0x80; // bit flag in doomdata->player

pub(crate) static mut localcmds: [ticcmd_t; BACKUPTICS] = [unsafe { mem::zeroed() }; BACKUPTICS];

pub(crate) static mut netcmds: [[ticcmd_t; BACKUPTICS]; MAXPLAYERS] =
	[[unsafe { mem::zeroed() }; BACKUPTICS]; MAXPLAYERS];
static mut nettics: [usize; MAXNETNODES] = [0; MAXNETNODES];
static mut nodeingame: [bool; MAXNETNODES] = [false; MAXNETNODES]; // set false as nodes leave game
static mut remoteresend: [bool; MAXNETNODES] = [false; MAXNETNODES]; // set when local needs tics
static mut resendto: [usize; MAXNETNODES] = [0; MAXNETNODES]; // set when remote needs tics
static mut resendcount: [Saturating<usize>; MAXNETNODES] = [Saturating(0); MAXNETNODES];

static mut nodeforplayer: [usize; MAXNETNODES] = [0; MAXNETNODES];

pub(crate) static mut maketic: usize = 0;
pub(crate) static mut skiptics: usize = 0;
pub(crate) static mut ticdup: usize = 0;
pub(crate) static mut maxsend: usize = 0; // BACKUPTICS/(2*ticdup)-1

static mut reboundpacket: bool = false;
static mut reboundstore: doomdata_t = unsafe { mem::zeroed() };

/*
int NetbufferSize (void)
{
	return (int)&(((doomdata_t *)0)->cmds[(*netbuffer).numtics]);
}
*/

// Checksum
#[allow(clippy::needless_return, reason = "might need to reimpl this")]
fn NetbufferChecksum() -> u32 {
	// 	unsigned		c;
	// 	int		i,l;
	//
	// 	c = 0x1234567;
	//
	// 	// FIXME -endianess?
	// #ifdef NORMALUNIX
	return 0; // byte order problems
	// #endif
	//
	// 	l = (NetbufferSize () - (int)&(((doomdata_t *)0)->retransmitfrom))/4;
	// 	for (i=0 ; i<l ; i++)
	// 	c += ((unsigned *)&(*netbuffer).retransmitfrom)[i] * (i+1);
	//
	// 	return c & NCMD_CHECKSUM;
}

fn ExpandTics(low: usize) -> usize {
	unsafe {
		let delta = usize::checked_signed_diff(low, maketic & 0xff).unwrap();

		match delta {
			-64..=64 => (maketic & !0xff) + low,
			65.. => (maketic & !0xff) + low - 256,
			..-64 => (maketic & !0xff) + 256 + low,
		}
	}
}

// HSendPacket
fn HSendPacket(node: usize, flags: u32) {
	unsafe {
		(*netbuffer).checksum = NetbufferChecksum() | flags;

		if node == 0 {
			reboundstore = *netbuffer;
			reboundpacket = true;
			return;
		}

		if demoplayback {
			return;
		}

		if !netgame {
			I_Error("Tried to transmit to another node");
		}

		todo!()
		/*
		(*doomcom).command = CMD_SEND;
		(*doomcom).remotenode = node;
		(*doomcom).datalength = NetbufferSize ();

		if (debugfile)
		{
		int		i;
		int		realretrans;
		if ((*netbuffer).checksum & NCMD_RETRANSMIT)
		realretrans = ExpandTics ((*netbuffer).retransmitfrom);
		else
		realretrans = -1;

		fprintf (debugfile,"send (%i + %i, R %i) [%i] ",
		ExpandTics((*netbuffer).starttic),
		(*netbuffer).numtics, realretrans, (*doomcom).datalength);

		for (i=0 ; i<(*doomcom).datalength ; i++)
		fprintf (debugfile,"%i ",((byte *)netbuffer)[i]);

		fprintf (debugfile,"\n");
		}

		I_NetCmd ();
		*/
	}
}

// HGetPacket
// Returns false if no packet is waiting
fn HGetPacket() -> bool {
	unsafe {
		if reboundpacket {
			*netbuffer = reboundstore;
			(*doomcom).remotenode = 0;
			reboundpacket = false;
			return true;
		}

		if !netgame {
			return false;
		}

		todo!();
		/*
		if (demoplayback)
		return false;

		(*doomcom).command = CMD_GET;
		I_NetCmd ();

		if ((*doomcom).remotenode == -1)
		return false;

		if ((*doomcom).datalength != NetbufferSize ())
		{
		if (debugfile)
		fprintf (debugfile,"bad packet length %i\n",(*doomcom).datalength);
		return false;
		}

		if (NetbufferChecksum () != ((*netbuffer).checksum&NCMD_CHECKSUM) )
		{
		if (debugfile)
		fprintf (debugfile,"bad packet checksum\n");
		return false;
		}

		if (debugfile)
		{
		int		realretrans;
		int	i;

		if ((*netbuffer).checksum & NCMD_SETUP)
		fprintf (debugfile,"setup packet\n");
		else
		{
		if ((*netbuffer).checksum & NCMD_RETRANSMIT)
		realretrans = ExpandTics ((*netbuffer).retransmitfrom);
		else
		realretrans = -1;

		fprintf (debugfile,"get %i = (%i + %i, R %i)[%i] ",
		(*doomcom).remotenode,
		ExpandTics((*netbuffer).starttic),
		(*netbuffer).numtics, realretrans, (*doomcom).datalength);

		for (i=0 ; i<(*doomcom).datalength ; i++)
		fprintf (debugfile,"%i ",((byte *)netbuffer)[i]);
		fprintf (debugfile,"\n");
		}
		}
		return true;
		*/
	}
}

// GetPackets
static mut exitmsg: [c_char; 80] = [0; 80];

#[allow(static_mut_refs)]
fn GetPackets() {
	unsafe {
		/*
		   ticcmd_t	*src, *dest;
		*/

		while HGetPacket() {
			if (*netbuffer).checksum & NCMD_SETUP != 0 {
				continue; // extra setup packet
			}

			let netconsole = usize::from((*netbuffer).player & !PL_DRONE);
			let netnode = usize::from((*doomcom).remotenode);

			// to save bytes, only the low byte of tic numbers are sent
			// Figure out what the rest of the bytes are
			let realstart = ExpandTics(usize::from((*netbuffer).starttic));
			let realend = realstart + usize::from((*netbuffer).numtics);

			// check for exiting the game
			if (*netbuffer).checksum & NCMD_EXIT != 0 {
				if !nodeingame[netnode] {
					continue;
				}
				nodeingame[netnode] = false;
				playeringame[netconsole] = false;
				libc::strcpy(exitmsg.as_mut_ptr(), c"Player 1 left the game".as_ptr());
				exitmsg[7] += i8::try_from(netconsole).unwrap();
				players[consoleplayer].message = exitmsg.as_ptr();
				if demorecording {
					G_CheckDemoStatus();
				}
				continue;
			}

			// check for a remote game kill
			if (*netbuffer).checksum & NCMD_KILL != 0 {
				I_Error("Killed by network driver");
			}

			nodeforplayer[netconsole] = netnode;

			// check for retransmit request
			if resendcount[netnode].0 == 0 && ((*netbuffer).checksum & NCMD_RETRANSMIT != 0) {
				resendto[netnode] = ExpandTics((*netbuffer).retransmitfrom.into());
				// if (debugfile) {
				// 	fprintf (debugfile,"retransmit from %i\n", resendto[netnode]);
				// }
				resendcount[netnode].0 = RESENDCOUNT;
			} else {
				resendcount[netnode] -= 1;
			}

			// check for out of order / duplicated packet
			if realend == nettics[netnode] {
				continue;
			}

			if realend < nettics[netnode] {
				// if (debugfile)
				// fprintf (debugfile, "out of order packet (%i + %i)\n" ,
				// realstart,(*netbuffer).numtics);
				continue;
			}

			// check for a missed packet
			if realstart > nettics[netnode] {
				// stop processing until the other system resends the missed tics
				// if (debugfile)
				// fprintf (debugfile,
				// 	"missed tics from %i (%i - %i)\n",
				// 	netnode, realstart, nettics[netnode]);
				remoteresend[netnode] = true;
				continue;
			}

			// update command store from the packet
			{
				remoteresend[netnode] = false;

				let start = nettics[netnode] - realstart;
				let mut src = &raw const (*netbuffer).cmds[start];

				while nettics[netnode] < realend {
					let dest = &raw mut netcmds[netconsole][nettics[netnode] % BACKUPTICS];
					nettics[netnode] += 1;
					*dest = *src;
					src = src.wrapping_add(1);
				}
			}
		}
	}
}

// NetUpdate
// Builds ticcmds for console player,
// sends out a packet
static mut gametime: usize = 0;

pub(crate) fn NetUpdate() {
	unsafe {
		// check time
		let nowtime = I_GetTime() / ticdup;
		let mut newtics = nowtime.saturating_sub(gametime);
		gametime = nowtime;

		if newtics == 0 {
			// nothing new to update
			// listen for other packets
			GetPackets();
			return;
		}

		if skiptics <= newtics {
			newtics -= skiptics;
			skiptics = 0;
		} else {
			skiptics -= newtics;
			newtics = 0;
		}

		(*netbuffer).player = consoleplayer.try_into().unwrap();

		// build new ticcmds for console player
		let gameticdiv = gametic / ticdup;
		for _ in 0..newtics {
			I_StartTic();
			D_ProcessEvents();
			if maketic - gameticdiv >= BACKUPTICS / 2 - 1 {
				break; // can't hold any more
			}

			G_BuildTiccmd(&raw mut localcmds[maketic % BACKUPTICS]);
			maketic += 1;
		}

		if singletics {
			return; // singletic update is syncronous
		}

		// send the packet to the other nodes
		for i in 0..usize::from((*doomcom).numnodes) {
			if nodeingame[i] {
				let realstart = resendto[i];
				(*netbuffer).starttic = u8::try_from(realstart & 0xff).unwrap();
				(*netbuffer).numtics = u8::try_from(maketic - realstart).unwrap();
				if usize::from((*netbuffer).numtics) > BACKUPTICS {
					I_Error("NetUpdate: (*netbuffer).numtics > BACKUPTICS");
				}

				resendto[i] = maketic - usize::from((*doomcom).extratics);

				for j in 0..usize::from((*netbuffer).numtics) {
					(*netbuffer).cmds[j] = localcmds[(realstart + j) % BACKUPTICS];
				}

				if remoteresend[i] {
					(*netbuffer).retransmitfrom = u8::try_from(nettics[i] & 0xff).unwrap();
					HSendPacket(i, NCMD_RETRANSMIT);
				} else {
					(*netbuffer).retransmitfrom = 0;
					HSendPacket(i, 0);
				}
			}
		}
	}
}

/*
// CheckAbort
//
void CheckAbort (void)
{
	event_t *ev;
	int		stoptic;

	stoptic = I_GetTime () + 2;
	while (I_GetTime() < stoptic)
	I_StartTic ();

	I_StartTic ();
	for ( ; eventtail != eventhead
		  ; eventtail = (++eventtail)&(MAXEVENTS-1) )
	{
	ev = &events[eventtail];
	if (ev->type == ev_keydown && ev->data1 == KEY_ESCAPE)
		I_Error ("Network game synchronization aborted.");
	}
}
*/

// D_ArbitrateNetStart
fn D_ArbitrateNetStart() {
	todo!()
	/*
	int		i;
	boolean	gotinfo[MAXNETNODES];

	autostart = true;
	memset (gotinfo,0,sizeof(gotinfo));

	if ((*doomcom).consoleplayer)
	{
	// listen for setup info from key player
	printf ("listening for network start info...\n");
	while (1)
	{
		CheckAbort ();
		if (!HGetPacket ())
		continue;
		if ((*netbuffer).checksum & NCMD_SETUP)
		{
		if ((*netbuffer).player != VERSION)
			I_Error ("Different DOOM versions cannot play a net game!");
		startskill = (*netbuffer).retransmitfrom & 15;
		deathmatch = ((*netbuffer).retransmitfrom & 0xc0) >> 6;
		nomonsters = ((*netbuffer).retransmitfrom & 0x20) > 0;
		respawnparm = ((*netbuffer).retransmitfrom & 0x10) > 0;
		startmap = (*netbuffer).starttic & 0x3f;
		startepisode = (*netbuffer).starttic >> 6;
		return;
		}
	}
	}
	else
	{
	// key player, send the setup info
	printf ("sending network start info...\n");
	do
	{
		CheckAbort ();
		for (i=0 ; i<(*doomcom).numnodes ; i++)
		{
		(*netbuffer).retransmitfrom = startskill;
		if (deathmatch)
			(*netbuffer).retransmitfrom |= (deathmatch<<6);
		if (nomonsters)
			(*netbuffer).retransmitfrom |= 0x20;
		if (respawnparm)
			(*netbuffer).retransmitfrom |= 0x10;
		(*netbuffer).starttic = startepisode * 64 + startmap;
		(*netbuffer).player = VERSION;
		(*netbuffer).numtics = 0;
		HSendPacket (i, NCMD_SETUP);
		}

	#if 1
		for(i = 10 ; i  &&  HGetPacket(); --i)
		{
		if(((*netbuffer).player&0x7f) < MAXNETNODES)
			gotinfo[(*netbuffer).player&0x7f] = true;
		}
	#else
		while (HGetPacket ())
		{
		gotinfo[(*netbuffer).player&0x7f] = true;
		}
	#endif

		for (i=1 ; i<(*doomcom).numnodes ; i++)
		if (!gotinfo[i])
			break;
	} while (i < (*doomcom).numnodes);
	}
	*/
}

// D_CheckNetGame
// Works out player numbers among the net participants
#[allow(static_mut_refs)]
pub(crate) fn D_CheckNetGame() {
	unsafe {
		for i in 0..MAXNETNODES {
			nodeingame[i] = false;
			nettics[i] = 0;
			remoteresend[i] = false; // set when local needs tics
			resendto[i] = 0; // which tic to start sending
		}

		// I_InitNetwork sets doomcom and netgame
		I_InitNetwork();
		if (*doomcom).id != DOOMCOM_ID {
			I_Error("Doomcom buffer invalid!");
		}

		netbuffer = &raw mut (*doomcom).data;
		displayplayer = (*doomcom).consoleplayer.into();
		consoleplayer = displayplayer;

		if netgame {
			D_ArbitrateNetStart();
		}

		println!(
			"startskill {}  deathmatch: {}  startmap: {}  startepisode: {}",
			startskill, deathmatch, startmap, startepisode
		);

		// read values out of doomcom
		ticdup = (*doomcom).ticdup.into();
		maxsend = BACKUPTICS / (2 * ticdup) - 1;
		if maxsend < 1 {
			maxsend = 1;
		}

		#[allow(clippy::needless_range_loop)]
		for i in 0..usize::from((*doomcom).numplayers) {
			playeringame[i] = true;
		}
		#[allow(clippy::needless_range_loop)]
		for i in 0..usize::from((*doomcom).numnodes) {
			nodeingame[i] = true;
		}

		println!(
			"player {} of {} ({} nodes)",
			consoleplayer + 1,
			(*doomcom).numplayers,
			(*doomcom).numnodes
		);
	}
}

// D_QuitNetGame
// Called before quitting to leave a net game
// without hanging the other players
pub(crate) fn D_QuitNetGame() {
	unsafe {
		if !debugfile.is_null() {
			fclose(debugfile);
		}

		if !netgame || !usergame || consoleplayer == usize::MAX || demoplayback {
			return;
		}

		// send a bunch of packets for security
		(*netbuffer).player = u8::try_from(consoleplayer).unwrap();
		(*netbuffer).numtics = 0;
		for _ in 0..4 {
			for (j, &node) in nodeingame[1..].iter().enumerate() {
				if node {
					HSendPacket(j, NCMD_EXIT);
				}
			}
			I_WaitVBL(1);
		}
	}
}

// TryRunTics
static mut frameon: usize = 0;
static mut frameskip: [bool; 4] = [false; 4];
static mut oldnettics: usize = 0;

pub(crate) fn TryRunTics() {
	static mut oldentertics: usize = 0;

	unsafe {
		// get real tics
		let entertic = I_GetTime() / ticdup;
		let realtics = entertic - oldentertics;
		oldentertics = entertic;

		// get available tics
		NetUpdate();

		let mut lowtic = usize::MAX;

		for i in 0..usize::from((*doomcom).numnodes) {
			if nodeingame[i] && nettics[i] < lowtic {
				lowtic = nettics[i];
			}
		}
		let availabletics = lowtic - gametic / ticdup;

		// decide how many tics to run
		let mut counts = if realtics + 1 < availabletics {
			realtics + 1
		} else if realtics < availabletics {
			realtics
		} else {
			availabletics
		};

		if counts < 1 {
			counts = 1;
		}

		frameon += 1;

		/*
		if (debugfile)
		fprintf (debugfile,
			 "=======real: %i  avail: %i  game: %i\n",
			 realtics, availabletics,counts);
		*/

		if !demoplayback {
			// ideally nettics[0] should be 1 - 3 tics above lowtic
			// if we are consistantly slower, speed up time
			let mut i = 0;
			for i_ in 0..MAXPLAYERS {
				i = i_;
				if playeringame[i] {
					break;
				}
			}
			if consoleplayer == i {
				// the key player does not adapt
			} else {
				if nettics[0] <= nettics[nodeforplayer[i]] {
					gametime -= 1;
				}
				frameskip[frameon & 3] = oldnettics > nettics[nodeforplayer[i]];
				oldnettics = nettics[0];
				if frameskip[0] && frameskip[1] && frameskip[2] && frameskip[3] {
					skiptics = 1;
				}
			}
		} // demoplayback

		// wait for new tics if needed
		while lowtic < gametic / ticdup + counts {
			NetUpdate();
			lowtic = usize::MAX;

			for i in 0..usize::from((*doomcom).numnodes) {
				if nodeingame[i] && nettics[i] < lowtic {
					lowtic = nettics[i];
				}
			}

			if lowtic < gametic / ticdup {
				I_Error("TryRunTics: lowtic < gametic");
			}

			// don't stay in here forever -- give the menu a chance to work
			if I_GetTime() / ticdup - entertic >= 20 {
				M_Ticker();
				return;
			}
		}

		// run the count * ticdup dics
		for _ in 0..counts {
			for i in 0..ticdup {
				if gametic / ticdup > lowtic {
					I_Error("gametic>lowtic");
				}
				if advancedemo {
					D_DoAdvanceDemo();
				}
				M_Ticker();
				G_Ticker();
				gametic += 1;

				// modify command for duplicated tics
				if i != ticdup - 1 {
					let buf = (gametic / ticdup) % BACKUPTICS;
					#[allow(clippy::needless_range_loop)]
					for j in 0..MAXPLAYERS {
						let cmd = &mut netcmds[j][buf];
						cmd.chatchar = 0;
						if cmd.buttons & BT_SPECIAL != 0 {
							cmd.buttons = 0;
						}
					}
				}
			}
			NetUpdate(); // check for new console commands
		}
	}
}
