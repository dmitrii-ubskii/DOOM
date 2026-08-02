#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::{
	ffi::{CStr, c_char},
	mem,
};

use libc::{
	AF_INET, EWOULDBLOCK, FIONBIO, INADDR_ANY, IPPROTO_UDP, PF_INET, SOCK_DGRAM, bind, hostent,
	in_addr, in_addr_t, ioctl, recvfrom, sendto, sockaddr_in, socket, strerror,
};

use crate::{
	d_net::{
		BACKUPTICS, DOOMCOM_ID, MAXNETNODES, command_t, doomcom, doomcom_t, doomdata_t, netbuffer,
	},
	d_ticcmd::ticcmd_t,
	g_game::netgame,
	i_system::I_Error,
	m_argv::M_CheckParm,
	myargc, myargv,
};

// For some odd reason...
fn htonl(x: u32) -> u32 {
	x.swap_bytes()
}

fn ntohl(x: u32) -> u32 {
	x.swap_bytes()
}

fn htonus(x: u16) -> u16 {
	x.swap_bytes()
}

fn htons(x: i16) -> i16 {
	x.swap_bytes()
}

fn ntohs(x: i16) -> i16 {
	x.swap_bytes()
}

// NETWORKING
const IPPORT_USERRESERVED: u16 = 5000;
static mut DOOMPORT: u16 = IPPORT_USERRESERVED + 0x1d;

static mut sendsocket: i32 = 0;
static mut insocket: i32 = 0;

static mut sendaddress: [sockaddr_in; MAXNETNODES] = unsafe { mem::zeroed() };

static mut netget: fn() = PacketGet;
static mut netsend: fn() = PacketSend;

unsafe extern "C" {
	static errno: i32;
}

// UDPsocket
fn UDPsocket() -> i32 {
	unsafe {
		let s = socket(PF_INET, SOCK_DGRAM, IPPROTO_UDP);
		if s < 0 {
			I_Error(format_args!(
				"can't create socket: {}",
				CStr::from_ptr(strerror(errno)).to_str().unwrap()
			));
		}
		s
	}
}

// BindToLocalPort
fn BindToLocalPort(s: i32, port: u16) {
	unsafe {
		let address = sockaddr_in {
			sin_family: u16::try_from(AF_INET).unwrap(),
			sin_port: port,
			sin_addr: in_addr { s_addr: INADDR_ANY },
			sin_zero: [0; 8],
		};

		let v =
			bind(s, (&raw const address).cast(), u32::try_from(size_of::<sockaddr_in>()).unwrap());
		if v == -1 {
			I_Error(format_args!(
				"BindToPort: bind: {}",
				CStr::from_ptr(strerror(errno)).to_str().unwrap()
			));
		}
	}
}

// PacketSend
fn PacketSend() {
	unsafe {
		let mut sw = doomdata_t {
			checksum: htonl((*netbuffer).checksum),
			retransmitfrom: (*netbuffer).retransmitfrom,
			starttic: (*netbuffer).starttic,
			player: (*netbuffer).player,
			numtics: (*netbuffer).numtics,
			cmds: [ticcmd_t::default(); BACKUPTICS],
		};

		for c in 0..usize::from((*netbuffer).numtics) {
			sw.cmds[c].forwardmove = (*netbuffer).cmds[c].forwardmove;
			sw.cmds[c].sidemove = (*netbuffer).cmds[c].sidemove;
			sw.cmds[c].angleturn = htons((*netbuffer).cmds[c].angleturn);
			sw.cmds[c].consistancy = htons((*netbuffer).cmds[c].consistancy);
			sw.cmds[c].chatchar = (*netbuffer).cmds[c].chatchar;
			sw.cmds[c].buttons = (*netbuffer).cmds[c].buttons;
		}

		//printf ("sending %i\n",gametic);
		sendto(
			sendsocket,
			(&raw const sw).cast(),
			usize::from((*doomcom).datalength),
			0,
			(&raw const sendaddress[usize::try_from((*doomcom).remotenode).unwrap()]).cast(),
			2,
		);
	}
}

// PacketGet
fn PacketGet() {
	unsafe {
		let mut fromlen = size_of::<sockaddr_in>();
		let mut fromaddress = mem::zeroed::<sockaddr_in>();
		let mut sw = doomdata_t::default();

		let c = recvfrom(
			insocket,
			(&raw mut sw).cast(),
			size_of_val(&sw),
			0,
			(&raw mut fromaddress).cast(),
			(&raw mut fromlen).cast(),
		);

		if c == -1 {
			if errno != EWOULDBLOCK {
				I_Error(format_args!(
					"GetPacket: {}",
					CStr::from_ptr(strerror(errno)).to_str().unwrap()
				));
			}
			(*doomcom).remotenode = -1; // no packet
			return;
		}

		{
			static mut first: bool = true;
			if first {
				println!(
					"len={}:p=[0x{:08x} 0x{:08x}] ",
					c,
					sw.checksum,
					u32::from_ne_bytes([sw.retransmitfrom, sw.starttic, sw.player, sw.numtics]),
				);
			}
			first = false;
		}

		// find remote node number
		let mut i = 0;
		while i < (*doomcom).numnodes {
			if fromaddress.sin_addr.s_addr == sendaddress[usize::from(i)].sin_addr.s_addr {
				break;
			}
			i += 1;
		}

		if i == (*doomcom).numnodes {
			// packet is not from one of the players (new game broadcast)
			(*doomcom).remotenode = -1; // no packet
			return;
		}

		(*doomcom).remotenode = i16::try_from(i).unwrap(); // good packet from a game player
		(*doomcom).datalength = u16::try_from(c).unwrap();

		// byte swap
		(*netbuffer).checksum = ntohl(sw.checksum);
		(*netbuffer).player = sw.player;
		(*netbuffer).retransmitfrom = sw.retransmitfrom;
		(*netbuffer).starttic = sw.starttic;
		(*netbuffer).numtics = sw.numtics;

		for c in 0..usize::from((*netbuffer).numtics) {
			(*netbuffer).cmds[c].forwardmove = sw.cmds[c].forwardmove;
			(*netbuffer).cmds[c].sidemove = sw.cmds[c].sidemove;
			(*netbuffer).cmds[c].angleturn = ntohs(sw.cmds[c].angleturn);
			(*netbuffer).cmds[c].consistancy = ntohs(sw.cmds[c].consistancy);
			(*netbuffer).cmds[c].chatchar = sw.cmds[c].chatchar;
			(*netbuffer).cmds[c].buttons = sw.cmds[c].buttons;
		}
	}
}

unsafe extern "C" {
	fn gethostbyname(name: *const c_char) -> *const hostent;
	fn inet_addr(cp: *const c_char) -> in_addr_t;
}

// I_InitNetwork
#[allow(static_mut_refs)]
pub(crate) fn I_InitNetwork() {
	unsafe {
		doomcom = libc::malloc(size_of::<doomcom_t>()).cast();
		libc::memset(doomcom.cast(), 0, size_of::<doomcom_t>());

		// set up for network
		let i = M_CheckParm(c"-dup".as_ptr());
		if i != 0 && i < myargc - 1 {
			(*doomcom).ticdup =
				u16::try_from(**myargv.wrapping_add(i + 1)).unwrap() - u16::from(b'0');
			(*doomcom).ticdup = (*doomcom).ticdup.clamp(1, 9);
		} else {
			(*doomcom).ticdup = 1;
		}

		if M_CheckParm(c"-extratic".as_ptr()) != 0 {
			(*doomcom).extratics = 1;
		} else {
			(*doomcom).extratics = 0;
		}

		let p = M_CheckParm(c"-port".as_ptr());
		if p != 0 && p < myargc - 1 {
			DOOMPORT = u16::try_from(libc::atoi(*myargv.wrapping_add(p + 1))).unwrap();
			println!("using alternate port {}", DOOMPORT);
		}

		// parse network game options,
		//  -net <consoleplayer> <host> <host> ...
		let mut i = M_CheckParm(c"-net".as_ptr());
		if i == 0 {
			// single player game
			netgame = false;
			(*doomcom).id = DOOMCOM_ID;
			(*doomcom).numplayers = 1;
			(*doomcom).numnodes = 1;
			(*doomcom).deathmatch = 0;
			(*doomcom).consoleplayer = 0;
			return;
		}

		netsend = PacketSend;
		netget = PacketGet;
		netgame = true;

		// parse player number and host list
		(*doomcom).consoleplayer =
			u16::from(u8::try_from(*(*myargv.wrapping_add(i + 1))).unwrap() - b'1');

		(*doomcom).numnodes = 1; // this node for sure

		i += 1;
		while {
			i += 1;
			i
		} < myargc && **myargv.wrapping_add(i) != i8::try_from(b'-').unwrap()
		{
			sendaddress[usize::from((*doomcom).numnodes)].sin_family =
				u16::try_from(AF_INET).unwrap();
			sendaddress[usize::from((*doomcom).numnodes)].sin_port = htonus(DOOMPORT);
			if **myargv.wrapping_add(i) == i8::try_from(b'.').unwrap() {
				sendaddress[usize::from((*doomcom).numnodes)].sin_addr.s_addr =
					inet_addr((*myargv.wrapping_add(i)).wrapping_add(1));
			} else {
				let hostentry = gethostbyname(*myargv.wrapping_add(i));
				if hostentry.is_null() {
					I_Error(format_args!(
						"gethostbyname: couldn't find {}",
						CStr::from_ptr(*myargv.wrapping_add(i)).to_str().unwrap()
					));
				}
				sendaddress[usize::from((*doomcom).numnodes)].sin_addr.s_addr =
					u32::try_from(**(*hostentry).h_addr_list).unwrap();
			}
			(*doomcom).numnodes += 1;
		}

		(*doomcom).id = DOOMCOM_ID;
		(*doomcom).numplayers = (*doomcom).numnodes;

		// build message to receive
		insocket = UDPsocket();
		BindToLocalPort(insocket, htonus(DOOMPORT));
		ioctl(insocket, FIONBIO, &true);

		sendsocket = UDPsocket();
	}
}

pub(crate) fn I_NetCmd() {
	unsafe {
		if (*doomcom).command == command_t::CMD_SEND {
			netsend();
		} else if (*doomcom).command == command_t::CMD_GET {
			netget();
		} else {
			I_Error(format_args!("Bad net cmd: {}\n", u8::from((*doomcom).command)));
		}
	}
}
