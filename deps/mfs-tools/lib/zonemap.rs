#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
extern crate libc;
extern "C" {
    #[no_mangle]
    fn mfsvol_device_translate(hnd: *mut volume_handle,
                               dev: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn mfs_write_volume_header(mfshnd: *mut mfs_handle) -> libc::c_int;
    #[no_mangle]
    fn tivo_partition_close(file: *mut tpFILE);
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    /* Fill in everything with lots and lots of aaaaaaaa for a vegetarian MFS */
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn alloca(_: libc::c_ulong) -> *mut libc::c_void;
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mfs_handle {
    pub vols: *mut volume_handle,
    pub vol_hdr: volume_header,
    pub zones: [zone_map_head; 3],
    pub loaded_zones: *mut zone_map,
    pub current_log: *mut log_hdr_s,
    pub inode_log_type: libc::c_int,
    pub is_64: libc::c_int,
    pub bootcycle: libc::c_int,
    pub bootsecs: libc::c_int,
    pub lastlogsync: libc::c_int,
    pub lastlogcommit: libc::c_int,
    pub err_msg: *mut libc::c_char,
    pub err_arg1: libc::c_int,
    pub err_arg2: libc::c_int,
    pub err_arg3: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct log_hdr_s {
    pub logstamp: libc::c_uint,
    pub crc: libc::c_uint,
    pub first: libc::c_uint,
    pub size: libc::c_uint,
}
/* Linked lists of zone maps for a certain type of map */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct zone_map {
    pub map: *mut zone_header,
    pub bitmaps: *mut *mut bitmap_header,
    pub changed_runs: *mut *mut zone_changed_run,
    pub changes: *mut zone_changes,
    pub dirty: libc::c_int,
    pub next: *mut zone_map,
    pub next_loaded: *mut zone_map,
}
/* Summary of changes to a zone bitmap since last commit */
/* As above, this only includes frees from split runs. */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct zone_changes {
    pub allocated: libc::c_int,
    pub freed: libc::c_int,
}
/* Linked list of runs allocated or freed since the last commit */
/* This only includes frees created by splitting an existing */
/* run.  Including frees created by actually freeing a run wuld not */
/* be transactionally safe to do, since it would result in allocating */
/* (and overwriting) a run with currently live data in it. */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct zone_changed_run {
    pub bitno: libc::c_int,
    pub newstate: libc::c_int,
    pub next: *mut zone_changed_run,
}
pub type bitmap_header = bitmap_header_s;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct bitmap_header_s {
    pub nbits: libc::c_int,
    pub freeblocks: libc::c_int,
    pub last: libc::c_int,
    pub nints: libc::c_int,
}
pub type zone_header = zone_header_u;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union zone_header_u {
    pub z32: zone_header_32,
    pub z64: zone_header_64,
}
/* addresses, pointing to mmapped */
	/* memory from /tmp/fsmem for bitmaps */
pub type zone_header_64 = zone_header_64_s;
#[derive ( Copy , Clone )]
#[repr(C, packed)]
pub struct zone_header_64_s {
    pub sector: libc::c_int,
    pub sbackup: libc::c_int,
    pub next_sector: libc::c_int,
    pub next_sbackup: libc::c_int,
    pub next_size: libc::c_int,
    pub first: libc::c_int,
    pub last: libc::c_int,
    pub size: libc::c_int,
    pub free: libc::c_int,
    pub next_length: libc::c_int,
    pub length: libc::c_int,
    pub min: libc::c_int,
    pub next_min: libc::c_int,
    pub logstamp: libc::c_int,
    pub type_0: zone_type,
    pub checksum: libc::c_int,
    pub zero: libc::c_int,
    pub num: libc::c_int,
}
pub type zone_type = zone_type_e;
pub type zone_type_e = libc::c_uint;
pub const ztPad: zone_type_e = 4294967295;
pub const ztMax: zone_type_e = 3;
pub const ztMedia: zone_type_e = 2;
pub const ztApplication: zone_type_e = 1;
pub const ztInode: zone_type_e = 0;
/* addresses, pointing to mmapped */
	/* memory from /tmp/fsmem for bitmaps */
pub type zone_header_32 = zone_header_32_s;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct zone_header_32_s {
    pub sector: libc::c_int,
    pub sbackup: libc::c_int,
    pub length: libc::c_int,
    pub next: zone_map_ptr_32,
    pub type_0: zone_type,
    pub logstamp: libc::c_int,
    pub checksum: libc::c_int,
    pub first: libc::c_int,
    pub last: libc::c_int,
    pub size: libc::c_int,
    pub min: libc::c_int,
    pub free: libc::c_int,
    pub zero: libc::c_int,
    pub num: libc::c_int,
}
pub type zone_map_ptr_32 = zone_map_ptr_32_s;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct zone_map_ptr_32_s {
    pub sector: libc::c_int,
    pub sbackup: libc::c_int,
    pub length: libc::c_int,
    pub size: libc::c_int,
    pub min: libc::c_int,
}
/* Head of zone maps linked list, contains totals as well */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct zone_map_head {
    pub size: libc::c_int,
    pub free: libc::c_int,
    pub next: *mut zone_map,
}
pub type volume_header = volume_header_u;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union volume_header_u {
    pub v32: volume_header_32,
    pub v64: volume_header_64,
}
pub type volume_header_64 = volume_header_64_s;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct volume_header_64_s {
    pub magicLSB: libc::c_int,
    pub magicMSB: libc::c_int,
    pub checksum: libc::c_int,
    pub off0c: libc::c_int,
    pub root_fsid: libc::c_int,
    pub off14: libc::c_int,
    pub firstpartsize: libc::c_int,
    pub off1c: libc::c_int,
    pub off20: libc::c_int,
    pub partitionlist: [libc::c_char; 132],
    pub total_sectors: libc::c_int,
    pub logstart: libc::c_int,
    pub volhdrlogstamp: libc::c_int,
    pub unkstart: libc::c_int,
    pub offc8: libc::c_int,
    pub unkstamp: libc::c_int,
    pub zonemap: zone_map_ptr_64,
    pub unknsectors: libc::c_int,
    pub lognsectors: libc::c_int,
    pub off100: libc::c_int,
    pub next_fsid: libc::c_int,
    pub bootcycles: libc::c_int,
    pub bootsecs: libc::c_int,
    pub off110: libc::c_int,
    pub off114: libc::c_int,
}
pub type zone_map_ptr_64 = zone_map_ptr_64_s;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct zone_map_ptr_64_s {
    pub sector: libc::c_int,
    pub sbackup: libc::c_int,
    pub length: libc::c_int,
    pub size: libc::c_int,
    pub min: libc::c_int,
}
pub type volume_header_32 = volume_header_32_s;
// mfs filesystem database consistent 
//(GSOD) Filesystem is inconsistent - cannot mount!  -  Filesystem is inconsistent, will attempt repair!          - Triggered by kickstart 5 7, and others
//(GSOD) Filesystem is inconsistent - cannot mount!  -  Filesystem logs are bad - log roll-forward inhibited!     - Triggered by ???
//(GSOD) Database is inconsistent - cannot mount!    -  fsfix:  mounted MFS volume, starting consistency checks.  - Triggered when a THD beackup with eSata restored to a single drive, without fixing off0c/off14 and trying to remove eSata from UI
// Clean up objects with missing tystreams                                                                        - Triggered after a GSOD encounters bad refcounts or missing media tystreams (eg, truncated restore)
// bit is set when mfs is 64-bit
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct volume_header_32_s {
    pub magicLSB: libc::c_int,
    pub magicMSB: libc::c_int,
    pub checksum: libc::c_int,
    pub off0c: libc::c_int,
    pub root_fsid: libc::c_int,
    pub off14: libc::c_int,
    pub firstpartsize: libc::c_int,
    pub off1c: libc::c_int,
    pub off20: libc::c_int,
    pub partitionlist: [libc::c_char; 128],
    pub total_sectors: libc::c_int,
    pub offa8: libc::c_int,
    pub logstart: libc::c_int,
    pub lognsectors: libc::c_int,
    pub volhdrlogstamp: libc::c_int,
    pub unkstart: libc::c_int,
    pub unksectors: libc::c_int,
    pub unkstamp: libc::c_int,
    pub zonemap: zone_map_ptr_32,
    pub next_fsid: libc::c_int,
    pub bootcycles: libc::c_int,
    pub bootsecs: libc::c_int,
    pub offe4: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct volume_handle {
    pub volumes: *mut volume_info,
    pub write_mode: volume_write_mode_e,
    pub hda: *mut libc::c_char,
    pub hdb: *mut libc::c_char,
    pub err_msg: *mut libc::c_char,
    pub err_arg1: libc::c_int,
    pub err_arg2: libc::c_int,
    pub err_arg3: libc::c_int,
}
/* Size that TiVo rounds the partitions down to whole increments of. */
/* Flags for vol_flags below */
/* #define VOL_FILE        1        This volume is really a file */
/* This volume is read-only */
/* #define VOL_SWAB        4        This volume is byte-swapped */
pub type volume_write_mode_e = libc::c_uint;
// Writes are cached in memory and returned on subsequent reads, but not written to the volume
pub const vwLocal: volume_write_mode_e = 2;
// Writes pretend to go to the volume, but are hex dumped instead
pub const vwFake: volume_write_mode_e = 1;
// Writes go to the volume (If RW mode)
pub const vwNormal: volume_write_mode_e = 0;
/* Information about the list of volumes needed for reads */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct volume_info {
    pub file: *mut tivo_partition_file,
    pub vol_flags: libc::c_int,
    pub start: libc::c_int,
    pub sectors: libc::c_int,
    pub offset: libc::c_int,
    pub mem_blocks: *mut volume_mem_data,
    pub next: *mut volume_info,
}
/* Block written to memory */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct volume_mem_data {
    pub start: libc::c_int,
    pub sectors: libc::c_int,
    pub next: *mut volume_mem_data,
    pub data: [libc::c_uchar; 0],
}
/* there is more stuff after this that we don't need */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct tivo_partition_file {
    pub tptype: C2RustUnnamed_2,
    pub fd: libc::c_int,
    pub extra: C2RustUnnamed,
}
/* Only for pDIRECT and friend. */
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed {
    pub direct: C2RustUnnamed_1,
    pub kernel: C2RustUnnamed_0,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub sectors: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub pt: *mut tivo_partition_table,
    pub part: *mut tivo_partition,
}
/* TiVo partition map partition */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct tivo_partition {
    pub sectors: libc::c_int,
    pub start: libc::c_int,
    pub refs: libc::c_uint,
    pub name: *mut libc::c_char,
    pub type_0: *mut libc::c_char,
    pub table: *mut tivo_partition_table,
}
/* TiVo partition map information */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct tivo_partition_table {
    pub device: *mut libc::c_char,
    pub ro_fd: libc::c_int,
    pub rw_fd: libc::c_int,
    pub vol_flags: libc::c_int,
    pub count: libc::c_int,
    pub refs: libc::c_int,
    pub devsize: libc::c_int,
    pub allocated: libc::c_int,
    pub partitions: *mut tivo_partition,
    pub next: *mut tivo_partition_table,
    pub parent: *mut tivo_partition_table,
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const pDIRECT: C2RustUnnamed_2 = 4;
pub const pDIRECTFILE: C2RustUnnamed_2 = 3;
pub const pDEVICE: C2RustUnnamed_2 = 2;
pub const pFILE: C2RustUnnamed_2 = 1;
pub const pUNKNOWN: C2RustUnnamed_2 = 0;
/* Prime number used in hash for finding base inode of fsid. */
pub type fsid_type_e = libc::c_uchar;
pub const tyDb: fsid_type_e = 8;
pub const tyDir: fsid_type_e = 4;
pub const tyStream: fsid_type_e = 2;
pub const tyFile: fsid_type_e = 1;
pub const tyNone: fsid_type_e = 0;
pub type fsid_type = fsid_type_e;
/* For inode_flags below. */
/* More than one fsid that hash to this inode follow */
/* Data for this inode is in the inode header */
/* Data for this inode is in the inode header (observed in Roamio)*/
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mfs_inode_s {
    pub fsid: libc::c_uint,
    pub refcount: libc::c_uint,
    pub bootcycles: libc::c_uint,
    pub bootsecs: libc::c_uint,
    pub inode: libc::c_uint,
    pub unk3: libc::c_uint,
    pub size: libc::c_uint,
    pub blocksize: libc::c_uint,
    pub blockused: libc::c_uint,
    pub lastmodified: libc::c_uint,
    pub type_0: fsid_type,
    pub zone: libc::c_uchar,
    pub pad: libc::c_ushort,
    pub sig: libc::c_uint,
    pub checksum: libc::c_uint,
    pub inode_flags: libc::c_uint,
    pub numblocks: libc::c_uint,
    pub datablocks: C2RustUnnamed_3,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_3 {
    pub d32: [C2RustUnnamed_5; 0],
    pub d64: [C2RustUnnamed_4; 0],
}
#[derive ( Copy , Clone )]
#[repr(C, packed)]
pub struct C2RustUnnamed_4 {
    pub sector: libc::c_int,
    pub count: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub sector: libc::c_uint,
    pub count: libc::c_uint,
}
pub type mfs_inode = mfs_inode_s;
pub type tpFILE = tivo_partition_file;
/* Size of each bitmap is (nints + (nbits < 8? 1: 2)) * 4 */
/* Don't ask why, thats just the way it is. */
/* In bitmap, MSB is first, LSB last */
#[no_mangle]
pub unsafe extern "C" fn mfs_next_zone(mut mfshnd: *mut mfs_handle,
                                       mut cur: *mut zone_header)
 -> *mut zone_header {
    let mut loop_0: *mut zone_map = (*mfshnd).loaded_zones;
    if cur.is_null() && !loop_0.is_null() { return (*loop_0).map }
    while !loop_0.is_null() && (*loop_0).map != cur {
        loop_0 = (*loop_0).next_loaded
    }
    loop_0 = (*loop_0).next_loaded;
    if !loop_0.is_null() { return (*loop_0).map }
    return 0 as *mut zone_header;
}
/* *********************************/
/* Estimate size of MFS in hours. */
#[no_mangle]
pub unsafe extern "C" fn mfs_sa_hours_estimate(mut mfshnd: *mut mfs_handle)
 -> libc::c_uint {
    panic!("Reached end of non-void function without returning");
}
/* ****************************************************************************/
/* Return the count of inodes.  Each inode is 2 sectors, so the count is the */
/* size of the inode zone maps divided by 2. */
#[no_mangle]
pub unsafe extern "C" fn mfs_inode_count(mut mfshnd: *mut mfs_handle)
 -> libc::c_uint {
    panic!("Reached end of non-void function without returning");
}
/* ***********************************************************************/
/* Return the state of a bit in a bitmap */
unsafe extern "C" fn mfs_zone_map_bit_state_get(mut bitmap:
                                                    *mut bitmap_header,
                                                mut bit: libc::c_uint)
 -> libc::c_int {
    let mut mapints: *mut libc::c_uint =
        bitmap.offset(1isize) as *mut libc::c_uint;
    /* Find the int that contains this bit */
    mapints =
        mapints.offset(bit.wrapping_div(32i32 as libc::c_uint) as isize);
    /* Adjust the bit to be within this int */
	/* MSB is bit 0, LSB is bit 31, etc */
    bit = 31i32 as libc::c_uint & !bit;
    /* Make it the actual bit */
    /* return it as 1 or 0 */
    return if 0 != bit & *mapints { 1i32 } else { 0i32 };
}
/* ***********************************************************************/
/* Set the state of a bit in a bitmap */
unsafe extern "C" fn mfs_zone_map_bit_state_set(mut bitmap:
                                                    *mut bitmap_header,
                                                mut bit: libc::c_uint) {
    let mut mapints: *mut libc::c_uint =
        bitmap.offset(1isize) as *mut libc::c_uint;
    /* Find the int that contains this bit */
    mapints =
        mapints.offset(bit.wrapping_div(32i32 as libc::c_uint) as isize);
    /* Adjust the bit to be within this int */
	/* MSB is bit 0, LSB is bit 31, etc */
    bit = 31i32 as libc::c_uint & !bit;
    /* Make it the actual bit */
    *mapints |= bit;
}
/* ***********************************************************************/
/* Clear the state of a bit in a bitmap */
unsafe extern "C" fn mfs_zone_map_bit_state_clear(mut bitmap:
                                                      *mut bitmap_header,
                                                  mut bit: libc::c_uint) {
    let mut mapints: *mut libc::c_uint =
        bitmap.offset(1isize) as *mut libc::c_uint;
    /* Find the int that contains this bit */
    mapints =
        mapints.offset(bit.wrapping_div(32i32 as libc::c_uint) as isize);
    /* Adjust the bit to be within this int */
	/* MSB is bit 0, LSB is bit 31, etc */
    bit = 31i32 as libc::c_uint & !bit;
    /* Make it the actual bit */
    *mapints &= !bit;
}
/* ***********************************************************************/
/* Clean up storage used by tracking changes */
unsafe extern "C" fn mfs_zone_map_clear_changes(mut mfshnd: *mut mfs_handle,
                                                mut zone: *mut zone_map) {
    let mut numbitmaps: libc::c_int = 0;
    let mut loop_0: libc::c_int = 0;
    0 != (*mfshnd).is_64;
    loop_0 = 0i32;
    while loop_0 < numbitmaps {
        while !(*(*zone).changed_runs.offset(loop_0 as isize)).is_null() {
            let mut cur: *mut zone_changed_run =
                *(*zone).changed_runs.offset(loop_0 as isize);
            let ref mut fresh0 =
                *(*zone).changed_runs.offset(loop_0 as isize);
            *fresh0 = (*cur).next;
            free(cur);
        }
        (*(*zone).changes.offset(loop_0 as isize)).allocated = 0i32;
        (*(*zone).changes.offset(loop_0 as isize)).freed = 0i32;
        loop_0 += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn mfs_zone_map_commit(mut mfshnd: *mut mfs_handle,
                                             mut logstamp: libc::c_uint) {
    let mut zone: *mut zone_map = 0 as *mut zone_map;
    zone = (*mfshnd).loaded_zones;
    while !zone.is_null() {
        if 0 != (*zone).dirty {
            0 != (*mfshnd).is_64;
            mfs_zone_map_clear_changes(mfshnd, zone);
        }
        zone = (*zone).next_loaded
    };
}
/* ***********************************************************************/
/* Return how big a new zone map would need to be for a given number of */
/* allocation blocks. */
#[no_mangle]
pub unsafe extern "C" fn mfs_new_zone_map_size(mut mfshnd: *mut mfs_handle,
                                               mut blocks: libc::c_uint)
 -> libc::c_int {
    /* I don't remember what the original +4 was for, but the +24 is because */
/* apparently TiVo likes a little breathing room at the end, and throws */
/* a tantrum if it doesn't have it.  This happens when the zone map has */
/* 18 levels of bitmaps. */
    let mut size: libc::c_int = 4i32 + 28i32;
    let mut order: libc::c_int = 0i32;
    if 0 != (*mfshnd).is_64 {
        size =
            (size as
                 libc::c_ulong).wrapping_add(::std::mem::size_of::<zone_header_64>()
                                                 as libc::c_ulong) as
                libc::c_int as libc::c_int
    } else {
        size =
            (size as
                 libc::c_ulong).wrapping_add(::std::mem::size_of::<zone_header_32>()
                                                 as libc::c_ulong) as
                libc::c_int as libc::c_int
    }
    /* Figure out the first order of 2 that is needed to have at least 1 bit for */
/* every block. */
    while ((1i32 << order) as libc::c_uint) < blocks { order += 1 }
    /* Increment it by one for loops and math. */
    order += 1;
    /* Start by adding in the sizes for all the bitmap headers. */
    size =
        (size as
             libc::c_ulong).wrapping_add((::std::mem::size_of::<bitmap_header>()
                                              as
                                              libc::c_ulong).wrapping_add(::std::mem::size_of::<*mut bitmap_header>()
                                                                              as
                                                                              libc::c_ulong).wrapping_mul(order
                                                                                                              as
                                                                                                              libc::c_ulong))
            as libc::c_int as libc::c_int;
    /* Estimate the size of the bitmap table for each order of 2. */
    loop  {
        let fresh1 = order;
        order = order - 1;
        if !(0 != fresh1) { break ; }
        let mut bits: libc::c_int = 1i32 << order;
        /* This produces the right results, oddly enough.  Every bitmap with 8 or */
/* more bits takes 1 int more than needed, and this produces that. */
        let mut tblints: libc::c_int = (bits + 57i32) / 32i32;
        size += tblints * 4i32
    }
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_volume_pair_app_size(mut mfshnd: *mut mfs_handle,
                                                  mut blocks: libc::c_int,
                                                  mut minalloc: libc::c_uint)
 -> libc::c_int {
    if minalloc == 0i32 as libc::c_uint {
        minalloc = 0x800i32 as libc::c_uint
    }
    panic!("Reached end of non-void function without returning");
}
/* *****************************************/
/* Free the memory used by the zone maps. */
#[no_mangle]
pub unsafe extern "C" fn mfs_cleanup_zone_maps(mut mfshnd: *mut mfs_handle) {
    let mut loop_0: libc::c_int = 0;
    loop_0 = 0i32;
    while loop_0 < ztMax as libc::c_int { loop_0 += 1 };
}
/* **************************/
/* Load the zone map list. */
#[no_mangle]
pub unsafe extern "C" fn mfs_load_zone_maps(mut mfshnd: *mut mfs_handle)
 -> libc::c_int {
    let mut cur: *mut zone_header = 0 as *mut zone_header;
    let mut loaded_head: *mut *mut zone_map = &mut (*mfshnd).loaded_zones;
    let mut cur_heads: [*mut *mut zone_map; 3] = [0 as *mut *mut zone_map; 3];
    let mut loop_0: libc::c_int = 0;
    0 != (*mfshnd).is_64;
    /* Start clean. */
    mfs_cleanup_zone_maps(mfshnd);
    loop_0 = 0i32;
    while loop_0 < ztMax as libc::c_int { loop_0 += 1 }
    loop_0 = 0i32;
    /* Read the map, verify it's checksum. */
    /* Link it into the proper map type pool. */
    /* Get pointers to the bitmaps for easy access */
    /* Allocate head pointers for changes for each level of the map */
    /* Also link it into the loaded order. */
    /* And add it to the totals. */
    return loop_0;
}