#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

use std::{ffi::c_char, ptr::null_mut};

use crate::{
	d_player::wbstartstruct_t,
	doomdata::{
		ML_BLOCKMAP, ML_LINEDEFS, ML_NODES, ML_REJECT, ML_SECTORS, ML_SEGS, ML_SIDEDEFS,
		ML_SSECTORS, ML_THINGS, ML_TWOSIDED, ML_VERTEXES, maplinedef_t, mapnode_t, mapsector_t,
		mapseg_t, mapsidedef_t, mapsubsector_t, mapthing_t, mapvertex_t,
	},
	doomdef::{GameMode_t, MAXPLAYERS, skill_t},
	doomstat::gamemode,
	i_system::I_Error,
	info::sprnames,
	m_bbox::{BOXBOTTOM, BOXLEFT, BOXRIGHT, BOXTOP, M_AddToBox, M_ClearBox},
	m_fixed::{FRACBITS, FixedDiv, fixed_t},
	p_local::{MAPBLOCKSHIFT, MAXRADIUS},
	p_mobj::mobj_t,
	p_tick::{P_InitThinkers, leveltime, playeringame, players},
	r_defs::{line_t, node_t, sector_t, seg_t, side_t, slopetype_t, subsector_t, vertex_t},
	tables::angle_t,
	w_wad::{W_CacheLumpNum, W_GetNumForName, W_LumpLength, W_Reload},
	z_zone::{PU_LEVEL, PU_PURGELEVEL, PU_STATIC, Z_Free, Z_FreeTags, Z_Malloc},
};

// MAP related Lookup tables.
// Store VERTEXES, LINEDEFS, SIDEDEFS, etc.
#[unsafe(no_mangle)]
pub static mut numvertexes: usize = 0;
#[unsafe(no_mangle)]
pub static mut vertexes: *mut vertex_t = null_mut();

#[unsafe(no_mangle)]
pub static mut numsegs: usize = 0;
#[unsafe(no_mangle)]
pub static mut segs: *mut seg_t = null_mut();

#[unsafe(no_mangle)]
pub static mut numsectors: usize = 0;
#[unsafe(no_mangle)]
pub static mut sectors: *mut sector_t = null_mut();

#[unsafe(no_mangle)]
pub static mut numsubsectors: usize = 0;
#[unsafe(no_mangle)]
pub static mut subsectors: *mut subsector_t = null_mut();

#[unsafe(no_mangle)]
pub static mut numnodes: usize = 0;
#[unsafe(no_mangle)]
pub static mut nodes: *mut node_t = null_mut();

#[unsafe(no_mangle)]
pub static mut numlines: usize = 0;
#[unsafe(no_mangle)]
pub static mut lines: *mut line_t = null_mut();

#[unsafe(no_mangle)]
pub static mut numsides: usize = 0;
#[unsafe(no_mangle)]
pub static mut sides: *mut side_t = null_mut();

// BLOCKMAP
// Created from axis aligned bounding box
// of the map, a rectangular array of
// blocks of size ...
// Used to speed up collision detection
// by spatial subdivision in 2D.

// Blockmap size.
#[unsafe(no_mangle)]
pub static mut bmapwidth: usize = 0;
#[unsafe(no_mangle)]
pub static mut bmapheight: usize = 0; // size in mapblocks
#[unsafe(no_mangle)]
pub static mut blockmap: *mut i16 = null_mut(); // usize for larger maps
// offsets in blockmap are from here
#[unsafe(no_mangle)]
pub static mut blockmaplump: *mut i16 = null_mut();
// origin of block map
#[unsafe(no_mangle)]
pub static mut bmaporgx: fixed_t = 0;
#[unsafe(no_mangle)]
pub static mut bmaporgy: fixed_t = 0;
// for thing chains
#[unsafe(no_mangle)]
pub static mut blocklinks: *mut *mut mobj_t = null_mut();

// REJECT
// For fast sight rejection.
// Speeds up enemy AI by skipping detailed
//  LineOf Sight calculation.
// Without special effect, this could be
//  used as a PVS lookup as well.
#[unsafe(no_mangle)]
pub static mut rejectmatrix: *mut u8 = null_mut();

// Maintain single and multi player starting spots.
const MAX_DEATHMATCH_STARTS: usize = 10;

#[unsafe(no_mangle)]
pub static mut deathmatchstarts: [mapthing_t; MAX_DEATHMATCH_STARTS] =
	[mapthing_t { x: 0, y: 0, angle: 0, ty: 0, options: 0 }; MAX_DEATHMATCH_STARTS];
#[unsafe(no_mangle)]
pub static mut deathmatch_p: *mut mapthing_t = null_mut();
#[unsafe(no_mangle)]
pub static mut playerstarts: [mapthing_t; MAXPLAYERS] =
	[mapthing_t { x: 0, y: 0, angle: 0, ty: 0, options: 0 }; MAXPLAYERS];

// P_LoadVertexes
fn P_LoadVertexes(lump: usize) {
	unsafe {
		// Determine number of lumps:
		//  total lump length / vertex record length.
		numvertexes = W_LumpLength(lump) / size_of::<mapvertex_t>();

		// Allocate zone memory for buffer.
		vertexes = Z_Malloc(numvertexes * size_of::<vertex_t>(), PU_LEVEL, null_mut()).cast();

		// Load data into cache.
		let data = W_CacheLumpNum(lump, PU_STATIC);

		let mut ml = data as *mut mapvertex_t;
		let mut li = vertexes;

		// Copy and convert vertex coordinates,
		// internal representation as fixed.
		for _ in 0..numvertexes {
			(*li).x = ((*ml).x as i32) << FRACBITS;
			(*li).y = ((*ml).y as i32) << FRACBITS;
			li = li.wrapping_add(1);
			ml = ml.wrapping_add(1);
		}

		// Free buffer memory.
		Z_Free(data);
	}
}

// P_LoadSegs
fn P_LoadSegs(lump: usize) {
	unsafe {
		numsegs = W_LumpLength(lump) / size_of::<mapseg_t>();
		segs = Z_Malloc(numsegs * size_of::<seg_t>(), PU_LEVEL, null_mut()).cast();
		libc::memset(segs.cast(), 0, numsegs * size_of::<seg_t>());
		let data = W_CacheLumpNum(lump, PU_STATIC);

		let mut ml = data as *mut mapseg_t;
		let mut li = segs;
		for _ in 0..numsegs {
			(*li).v1 = vertexes.wrapping_add((*ml).v1 as usize);
			(*li).v2 = vertexes.wrapping_add((*ml).v2 as usize);

			(*li).angle = ((*ml).angle as angle_t) << 16;
			(*li).offset = ((*ml).offset as i32) << 16;
			let linedef = (*ml).linedef as usize;
			let ldef = lines.wrapping_add(linedef);
			(*li).linedef = ldef;
			let side = (*ml).side;
			(*li).sidedef = sides.wrapping_add((*ldef).sidenum[side as usize] as usize);
			(*li).frontsector =
				(*sides.wrapping_add((*ldef).sidenum[side as usize] as usize)).sector;
			if (*ldef).flags & ML_TWOSIDED != 0 {
				(*li).backsector =
					(*sides.wrapping_add((*ldef).sidenum[side as usize ^ 1] as usize)).sector;
			} else {
				(*li).backsector = null_mut();
			}
			li = li.wrapping_add(1);
			ml = ml.wrapping_add(1);
		}

		Z_Free(data);
	}
}

// P_LoadSubsectors
fn P_LoadSubsectors(lump: usize) {
	unsafe {
		numsubsectors = W_LumpLength(lump) / size_of::<mapsubsector_t>();
		subsectors =
			Z_Malloc(numsubsectors * size_of::<subsector_t>(), PU_LEVEL, null_mut()).cast();
		let data = W_CacheLumpNum(lump, PU_STATIC);

		let mut ms = data as *mut mapsubsector_t;
		libc::memset(subsectors.cast(), 0, numsubsectors * size_of::<subsector_t>());
		let mut ss = subsectors;

		for _ in 0..numsubsectors {
			(*ss).numlines = (*ms).numsegs;
			(*ss).firstline = (*ms).firstseg;
			ss = ss.wrapping_add(1);
			ms = ms.wrapping_add(1);
		}

		Z_Free(data);
	}
}

unsafe extern "C" {
	fn R_FlatNumForName(name: *const c_char) -> i32;
}

// P_LoadSectors
fn P_LoadSectors(lump: usize) {
	unsafe {
		numsectors = W_LumpLength(lump) / size_of::<mapsector_t>();
		sectors = Z_Malloc(numsectors * size_of::<sector_t>(), PU_LEVEL, null_mut()).cast();
		libc::memset(sectors.cast(), 0, numsectors * size_of::<sector_t>());
		let data = W_CacheLumpNum(lump, PU_STATIC);

		let mut ms = data as *mut mapsector_t;
		let mut ss = sectors;
		for _ in 0..numsectors {
			(*ss).floorheight = ((*ms).floorheight as i32) << FRACBITS;
			(*ss).ceilingheight = ((*ms).ceilingheight as i32) << FRACBITS;
			(*ss).floorpic = R_FlatNumForName(&raw const (*ms).floorpic[0]) as i16;
			(*ss).ceilingpic = R_FlatNumForName(&raw const (*ms).ceilingpic[0]) as i16;
			(*ss).lightlevel = (*ms).lightlevel;
			(*ss).special = (*ms).special;
			(*ss).tag = (*ms).tag;
			(*ss).thinglist = null_mut();
			ss = ss.wrapping_add(1);
			ms = ms.wrapping_add(1);
		}

		Z_Free(data);
	}
}

// P_LoadNodes
fn P_LoadNodes(lump: usize) {
	unsafe {
		numnodes = W_LumpLength(lump) / size_of::<mapnode_t>();
		nodes = Z_Malloc(numnodes * size_of::<node_t>(), PU_LEVEL, null_mut()).cast();
		let data = W_CacheLumpNum(lump, PU_STATIC);

		let mut mn = data as *mut mapnode_t;
		let mut no = nodes;

		for _ in 0..numnodes {
			(*no).x = ((*mn).x as i32) << FRACBITS;
			(*no).y = ((*mn).y as i32) << FRACBITS;
			(*no).dx = ((*mn).dx as i32) << FRACBITS;
			(*no).dy = ((*mn).dy as i32) << FRACBITS;
			(*no).children = (*mn).children;
			for j in 0..2 {
				for k in 0..4 {
					(*no).bbox[j][k] = ((*mn).bbox[j][k] as i32) << FRACBITS;
				}
			}
			no = no.wrapping_add(1);
			mn = mn.wrapping_add(1);
		}

		Z_Free(data);
	}
}

unsafe extern "C" {
	fn P_SpawnMapThing(mthing: *mut mapthing_t);
}

// P_LoadThings
fn P_LoadThings(lump: usize) {
	unsafe {
		let data = W_CacheLumpNum(lump, PU_STATIC);
		let numthings = W_LumpLength(lump) / size_of::<mapthing_t>();

		let mut mt = data as *mut mapthing_t;
		for _ in 0..numthings {
			let mut spawn = true;

			// Do not spawn cool, new monsters if !commercial
			if gamemode != GameMode_t::commercial {
				match (*mt).ty {
					| 68	// Arachnotron
					| 64	// Archvile
					| 88	// Boss Brain
					| 89	// Boss Shooter
					| 69	// Hell Knight
					| 67	// Mancubus
					| 71	// Pain Elemental
					| 65	// Former Human Commando
					| 66	// Revenant
					| 84	// Wolf SS
						=> spawn = false,
					_ => (),
				}
			}
			if !spawn {
				break;
			}

			// Do spawn all other stuff.
			P_SpawnMapThing(mt);

			mt = mt.wrapping_add(1);
		}

		Z_Free(data);
	}
}

// P_LoadLineDefs
// Also counts secret lines for intermissions.
fn P_LoadLineDefs(lump: usize) {
	unsafe {
		numlines = W_LumpLength(lump) / size_of::<maplinedef_t>();
		lines = Z_Malloc(numlines * size_of::<line_t>(), PU_LEVEL, null_mut()).cast();
		libc::memset(lines.cast(), 0, numlines * size_of::<line_t>());
		let data = W_CacheLumpNum(lump, PU_STATIC);

		let mut mld = data as *mut maplinedef_t;
		let mut ld = lines;
		for _ in 0..numlines {
			(*ld).flags = (*mld).flags;
			(*ld).special = (*mld).special;
			(*ld).tag = (*mld).tag;
			(*ld).v1 = vertexes.wrapping_add((*mld).v1 as usize);
			(*ld).v2 = vertexes.wrapping_add((*mld).v2 as usize);
			let v1 = (*ld).v1;
			let v2 = (*ld).v2;
			(*ld).dx = (*v2).x - (*v1).x;
			(*ld).dy = (*v2).y - (*v1).y;

			if (*ld).dx == 0 {
				(*ld).slopetype = slopetype_t::ST_VERTICAL;
			} else if (*ld).dy == 0 {
				(*ld).slopetype = slopetype_t::ST_HORIZONTAL;
			} else if FixedDiv((*ld).dy, (*ld).dx) > 0 {
				(*ld).slopetype = slopetype_t::ST_POSITIVE;
			} else {
				(*ld).slopetype = slopetype_t::ST_NEGATIVE;
			}

			if (*v1).x < (*v2).x {
				(*ld).bbox[BOXLEFT] = (*v1).x;
				(*ld).bbox[BOXRIGHT] = (*v2).x;
			} else {
				(*ld).bbox[BOXLEFT] = (*v2).x;
				(*ld).bbox[BOXRIGHT] = (*v1).x;
			}

			if (*v1).y < (*v2).y {
				(*ld).bbox[BOXBOTTOM] = (*v1).y;
				(*ld).bbox[BOXTOP] = (*v2).y;
			} else {
				(*ld).bbox[BOXBOTTOM] = (*v2).y;
				(*ld).bbox[BOXTOP] = (*v1).y;
			}

			(*ld).sidenum[0] = (*mld).sidenum[0];
			(*ld).sidenum[1] = (*mld).sidenum[1];

			if (*ld).sidenum[0] != -1 {
				(*ld).frontsector = (*sides.wrapping_add((*ld).sidenum[0] as usize)).sector;
			} else {
				(*ld).frontsector = null_mut();
			}

			if (*ld).sidenum[1] != -1 {
				(*ld).backsector = (*sides.wrapping_add((*ld).sidenum[1] as usize)).sector;
			} else {
				(*ld).backsector = null_mut();
			}
			mld = mld.wrapping_add(1);
			ld = ld.wrapping_add(1);
		}

		Z_Free(data);
	}
}

unsafe extern "C" {
	fn R_TextureNumForName(name: *const c_char) -> i32;
}

// P_LoadSideDefs
fn P_LoadSideDefs(lump: usize) {
	unsafe {
		numsides = W_LumpLength(lump) / size_of::<mapsidedef_t>();
		sides = Z_Malloc(numsides * size_of::<side_t>(), PU_LEVEL, null_mut()).cast();
		libc::memset(sides.cast(), 0, numsides * size_of::<side_t>());
		let data = W_CacheLumpNum(lump, PU_STATIC);

		let mut msd = data as *mut mapsidedef_t;
		let mut sd = sides;
		for _ in 0..numsides {
			(*sd).textureoffset = ((*msd).textureoffset as i32) << FRACBITS;
			(*sd).rowoffset = ((*msd).rowoffset as i32) << FRACBITS;
			(*sd).toptexture = R_TextureNumForName(&raw const (*msd).toptexture[0]) as i16;
			(*sd).bottomtexture = R_TextureNumForName(&raw const (*msd).bottomtexture[0]) as i16;
			(*sd).midtexture = R_TextureNumForName(&raw const (*msd).midtexture[0]) as i16;
			(*sd).sector = sectors.wrapping_add((*msd).sector as usize);
			msd = msd.wrapping_add(1);
			sd = sd.wrapping_add(1);
		}

		Z_Free(data);
	}
}

// P_LoadBlockMap
fn P_LoadBlockMap(lump: usize) {
	unsafe {
		blockmaplump = W_CacheLumpNum(lump, PU_LEVEL).cast();
		blockmap = blockmaplump.wrapping_add(4);

		bmaporgx = (*blockmaplump.wrapping_add(0) as i32) << FRACBITS;
		bmaporgy = (*blockmaplump.wrapping_add(1) as i32) << FRACBITS;
		bmapwidth = *blockmaplump.wrapping_add(2) as usize;
		bmapheight = *blockmaplump.wrapping_add(3) as usize;

		// clear out mobj chains
		let count = size_of::<*mut mobj_t>() * bmapwidth * bmapheight;
		blocklinks = Z_Malloc(count, PU_LEVEL, null_mut()).cast();
		libc::memset(blocklinks.cast(), 0, count);
	}
}

// P_GroupLines
// Builds sector line lists and subsector sector numbers.
// Finds block bounding boxes for sectors.
fn P_GroupLines() {
	unsafe {
		// look up sector number for each subsector
		let mut ss = subsectors;
		for _ in 0..numsubsectors {
			let seg = segs.wrapping_add((*ss).firstline as usize);
			(*ss).sector = (*(*seg).sidedef).sector;
			ss = ss.wrapping_add(1);
		}

		// count number of lines in each sector
		let mut li = lines;
		let mut total = 0;
		for _ in 0..numlines {
			total += 1;
			(*(*li).frontsector).linecount += 1;

			if !(*li).backsector.is_null() && (*li).backsector != (*li).frontsector {
				(*(*li).backsector).linecount += 1;
				total += 1;
			}
			li = li.wrapping_add(1);
		}

		// build line tables for each sector
		let mut linebuffer =
			Z_Malloc(total * size_of::<*mut line_t>(), PU_LEVEL, null_mut()).cast();
		let mut sector = sectors;
		let mut bbox = [0; 4];
		for _ in 0..numsectors {
			M_ClearBox(&mut bbox);
			(*sector).lines = linebuffer;
			li = lines;
			for _ in 0..numlines {
				if (*li).frontsector == sector || (*li).backsector == sector {
					*linebuffer = li;
					linebuffer = linebuffer.wrapping_add(1);
					M_AddToBox(&mut bbox, (*(*li).v1).x, (*(*li).v1).y);
					M_AddToBox(&mut bbox, (*(*li).v2).x, (*(*li).v2).y);
				}
				li = li.wrapping_add(1);
			}
			if linebuffer.offset_from((*sector).lines) != (*sector).linecount as isize {
				I_Error(c"P_GroupLines: miscounted".as_ptr());
			}

			// set the degenmobj_t to the middle of the bounding box
			(*sector).soundorg.x = (bbox[BOXRIGHT] + bbox[BOXLEFT]) / 2;
			(*sector).soundorg.y = (bbox[BOXTOP] + bbox[BOXBOTTOM]) / 2;

			// adjust bounding box to map blocks
			let mut block = (bbox[BOXTOP] - bmaporgy + MAXRADIUS) >> MAPBLOCKSHIFT;
			block = if block >= bmapheight as i32 { bmapheight as i32 - 1 } else { block };
			(*sector).blockbox[BOXTOP] = block;

			block = (bbox[BOXBOTTOM] - bmaporgy - MAXRADIUS) >> MAPBLOCKSHIFT;
			block = if block < 0 { 0 } else { block };
			(*sector).blockbox[BOXBOTTOM] = block;

			block = (bbox[BOXRIGHT] - bmaporgx + MAXRADIUS) >> MAPBLOCKSHIFT;
			block = if block >= bmapwidth as i32 { bmapwidth as i32 - 1 } else { block };
			(*sector).blockbox[BOXRIGHT] = block;

			block = (bbox[BOXLEFT] - bmaporgx - MAXRADIUS) >> MAPBLOCKSHIFT;
			block = if block < 0 { 0 } else { block };
			(*sector).blockbox[BOXLEFT] = block;
			sector = sector.wrapping_add(1);
		}
	}
}

type boolean = i32;

unsafe extern "C" {
	static mut bodyqueslot: i32;
	static mut consoleplayer: i32;
	static mut deathmatch: boolean;
	static mut iquehead: i32;
	static mut iquetail: i32;
	static mut precache: boolean;
	static mut totalitems: i32;
	static mut totalkills: i32;
	static mut totalsecret: i32;
	static mut wminfo: wbstartstruct_t;

	fn S_Start();
	fn P_SpawnSpecials();
	fn R_PrecacheLevel();
	fn G_DeathMatchSpawnPlayer(player: usize);
	fn P_InitSwitchList();
	fn P_InitPicAnims();
	fn R_InitSprites(sprnames: *const *const c_char);
}

// NOT called by W_Ticker. Fixme.
// P_SetupLevel
#[unsafe(no_mangle)]
pub extern "C" fn P_SetupLevel(episode: i32, map: i32, _playermask: i32, _skill: skill_t) {
	unsafe {
		totalkills = 0;
		totalitems = 0;
		totalsecret = 0;
		wminfo.maxfrags = 0;
		wminfo.partime = 180;
		for p in &mut players[0..MAXPLAYERS] {
			p.killcount = 0;
			p.secretcount = 0;
			p.itemcount = 0;
		}

		// Initial height of PointOfView
		// will be set by player think.
		players[consoleplayer as usize].viewz = 1;

		// Make sure all sounds are stopped before Z_FreeTags.
		S_Start();

		Z_FreeTags(PU_LEVEL, PU_PURGELEVEL - 1);

		// UNUSED W_Profile ();
		P_InitThinkers();

		// if working with a devlopment map, reload it
		W_Reload();

		let mut lumpname = [0; 9];

		// find map name
		if gamemode == GameMode_t::commercial {
			if map < 10 {
				libc::sprintf(&raw mut lumpname[0], c"map0%i".as_ptr(), map);
			} else {
				libc::sprintf(&raw mut lumpname[0], c"map%i".as_ptr(), map);
			}
		} else {
			lumpname[0] = b'E' as c_char;
			lumpname[1] = b'0' as c_char + episode as c_char;
			lumpname[2] = b'M' as c_char;
			lumpname[3] = b'0' as c_char + map as c_char;
			lumpname[4] = 0;
		}

		let lumpnum = W_GetNumForName(&raw const lumpname[0]) as usize;

		leveltime = 0;

		// note: most of this ordering is important
		P_LoadBlockMap(lumpnum + ML_BLOCKMAP);
		P_LoadVertexes(lumpnum + ML_VERTEXES);
		P_LoadSectors(lumpnum + ML_SECTORS);
		P_LoadSideDefs(lumpnum + ML_SIDEDEFS);

		P_LoadLineDefs(lumpnum + ML_LINEDEFS);
		P_LoadSubsectors(lumpnum + ML_SSECTORS);
		P_LoadNodes(lumpnum + ML_NODES);
		P_LoadSegs(lumpnum + ML_SEGS);

		rejectmatrix = W_CacheLumpNum(lumpnum + ML_REJECT, PU_LEVEL).cast();
		P_GroupLines();

		bodyqueslot = 0;
		deathmatch_p = &raw mut deathmatchstarts[0];
		P_LoadThings(lumpnum + ML_THINGS);

		// if deathmatch, randomly spawn the active players
		if deathmatch != 0 {
			for i in 0..MAXPLAYERS {
				if playeringame[i] != 0 {
					players[i].mo = null_mut();
					G_DeathMatchSpawnPlayer(i);
				}
			}
		}

		// clear special respawning que
		iquehead = 0;
		iquetail = 0;

		// set up world state
		P_SpawnSpecials();

		// build subsector connect matrix
		//	UNUSED P_ConnectSubsectors ();

		// preload graphics
		if precache != 0 {
			R_PrecacheLevel();
		}

		//printf ("free memory: 0x%x\n", Z_FreeMemory());
	}
}

// Called by startup code.
// P_Init
pub(crate) fn P_Init() {
	unsafe {
		P_InitSwitchList();
		P_InitPicAnims();
		R_InitSprites(&raw const sprnames[0]);
	}
}
