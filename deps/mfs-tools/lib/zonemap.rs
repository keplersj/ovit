#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
#![feature(ptr_wrapping_offset_from)]
extern crate libc;
extern "C" {
    #[no_mangle]
    fn mfsvol_read_data(hnd: *mut volume_handle, buf: *mut libc::c_void,
                        sector: uint64_t, count: uint32_t) -> libc::c_int;
    #[no_mangle]
    fn mfsvol_is_writable(hnd: *mut volume_handle, sector: uint64_t)
     -> libc::c_int;
    #[no_mangle]
    fn mfsvol_device_translate(hnd: *mut volume_handle,
                               dev: *mut libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn mfsvol_volume_size(hnd: *mut volume_handle, sector: uint64_t)
     -> uint64_t;
    #[no_mangle]
    fn mfsvol_volume_set_size(hnd: *mut volume_handle) -> uint64_t;
    // Historically, the drive was accessed as big endian (MSB), however newer platforms (Roamio) are mipsel based, hence the numeric values are little endian (LSB).
    /* Drive is little endian */
    #[no_mangle]
    static mut mfsLSB: libc::c_int;
    #[no_mangle]
    fn mfs_check_crc(data: *mut libc::c_uchar, size: libc::c_uint,
                     off: libc::c_uint) -> libc::c_uint;
    #[no_mangle]
    fn mfs_update_crc(data: *mut libc::c_uchar, size: libc::c_uint,
                      off: libc::c_uint);
    #[no_mangle]
    fn mfsvol_write_data(hnd: *mut volume_handle, buf: *mut libc::c_void,
                         sector: uint64_t, count: uint32_t) -> libc::c_int;
    #[no_mangle]
    fn mfs_write_volume_header(mfshnd: *mut mfs_handle) -> libc::c_int;
    #[no_mangle]
    fn tivo_partition_close(file: *mut tpFILE);
    #[no_mangle]
    fn mfs_log_last_sync(mfshnd: *mut mfs_handle) -> libc::c_uint;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    /* Fill in everything with lots and lots of aaaaaaaa for a vegetarian MFS */
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
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
    pub bootcycle: uint32_t,
    pub bootsecs: uint32_t,
    pub lastlogsync: uint32_t,
    pub lastlogcommit: uint32_t,
    pub err_msg: *mut libc::c_char,
    pub err_arg1: int64_t,
    pub err_arg2: int64_t,
    pub err_arg3: int64_t,
}
pub type int64_t = libc::c_longlong;
pub type uint32_t = libc::c_uint;
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
    pub nbits: uint32_t,
    pub freeblocks: uint32_t,
    pub last: uint32_t,
    pub nints: uint32_t,
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
    pub sector: uint64_t,
    pub sbackup: uint64_t,
    pub next_sector: uint64_t,
    pub next_sbackup: uint64_t,
    pub next_size: uint64_t,
    pub first: uint64_t,
    pub last: uint64_t,
    pub size: uint64_t,
    pub free: uint64_t,
    pub next_length: uint32_t,
    pub length: uint32_t,
    pub min: uint32_t,
    pub next_min: uint32_t,
    pub logstamp: uint32_t,
    pub type_0: zone_type,
    pub checksum: uint32_t,
    pub zero: uint32_t,
    pub num: uint32_t,
}
pub type zone_type = zone_type_e;
pub type zone_type_e = libc::c_uint;
pub const ztPad: zone_type_e = 4294967295;
pub const ztMax: zone_type_e = 3;
pub const ztMedia: zone_type_e = 2;
pub const ztApplication: zone_type_e = 1;
pub const ztInode: zone_type_e = 0;
pub type uint64_t = libc::c_ulonglong;
/* addresses, pointing to mmapped */
	/* memory from /tmp/fsmem for bitmaps */
pub type zone_header_32 = zone_header_32_s;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct zone_header_32_s {
    pub sector: uint32_t,
    pub sbackup: uint32_t,
    pub length: uint32_t,
    pub next: zone_map_ptr_32,
    pub type_0: zone_type,
    pub logstamp: uint32_t,
    pub checksum: uint32_t,
    pub first: uint32_t,
    pub last: uint32_t,
    pub size: uint32_t,
    pub min: uint32_t,
    pub free: uint32_t,
    pub zero: uint32_t,
    pub num: uint32_t,
}
pub type zone_map_ptr_32 = zone_map_ptr_32_s;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct zone_map_ptr_32_s {
    pub sector: uint32_t,
    pub sbackup: uint32_t,
    pub length: uint32_t,
    pub size: uint32_t,
    pub min: uint32_t,
}
/* Head of zone maps linked list, contains totals as well */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct zone_map_head {
    pub size: uint64_t,
    pub free: uint64_t,
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
    pub magicLSB: uint32_t,
    pub magicMSB: uint32_t,
    pub checksum: uint32_t,
    pub off0c: uint32_t,
    pub root_fsid: uint32_t,
    pub off14: uint32_t,
    pub firstpartsize: uint32_t,
    pub off1c: uint32_t,
    pub off20: uint32_t,
    pub partitionlist: [libc::c_char; 132],
    pub total_sectors: uint64_t,
    pub logstart: uint64_t,
    pub volhdrlogstamp: uint64_t,
    pub unkstart: uint64_t,
    pub offc8: uint32_t,
    pub unkstamp: uint32_t,
    pub zonemap: zone_map_ptr_64,
    pub unknsectors: uint32_t,
    pub lognsectors: uint32_t,
    pub off100: uint32_t,
    pub next_fsid: uint32_t,
    pub bootcycles: uint32_t,
    pub bootsecs: uint32_t,
    pub off110: uint32_t,
    pub off114: uint32_t,
}
pub type zone_map_ptr_64 = zone_map_ptr_64_s;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct zone_map_ptr_64_s {
    pub sector: uint64_t,
    pub sbackup: uint64_t,
    pub length: uint64_t,
    pub size: uint64_t,
    pub min: uint64_t,
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
    pub magicLSB: uint32_t,
    pub magicMSB: uint32_t,
    pub checksum: uint32_t,
    pub off0c: uint32_t,
    pub root_fsid: uint32_t,
    pub off14: uint32_t,
    pub firstpartsize: uint32_t,
    pub off1c: uint32_t,
    pub off20: uint32_t,
    pub partitionlist: [libc::c_char; 128],
    pub total_sectors: uint32_t,
    pub offa8: uint32_t,
    pub logstart: uint32_t,
    pub lognsectors: uint32_t,
    pub volhdrlogstamp: uint32_t,
    pub unkstart: uint32_t,
    pub unksectors: uint32_t,
    pub unkstamp: uint32_t,
    pub zonemap: zone_map_ptr_32,
    pub next_fsid: uint32_t,
    pub bootcycles: uint32_t,
    pub bootsecs: uint32_t,
    pub offe4: uint32_t,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct volume_handle {
    pub volumes: *mut volume_info,
    pub write_mode: volume_write_mode_e,
    pub hda: *mut libc::c_char,
    pub hdb: *mut libc::c_char,
    pub err_msg: *mut libc::c_char,
    pub err_arg1: int64_t,
    pub err_arg2: int64_t,
    pub err_arg3: int64_t,
}
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
    pub start: uint64_t,
    pub sectors: uint64_t,
    pub offset: uint64_t,
    pub mem_blocks: *mut volume_mem_data,
    pub next: *mut volume_info,
}
/* Block written to memory */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct volume_mem_data {
    pub start: uint64_t,
    pub sectors: uint64_t,
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
    pub sectors: uint64_t,
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
    pub sectors: uint64_t,
    pub start: uint64_t,
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
    pub devsize: uint64_t,
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
pub type size_t = libc::c_ulong;
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
    pub sector: uint64_t,
    pub count: uint32_t,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub sector: libc::c_uint,
    pub count: libc::c_uint,
}
pub type mfs_inode = mfs_inode_s;
pub type tpFILE = tivo_partition_file;
#[inline]
unsafe extern "C" fn Endian32_Swap(mut var: uint32_t) -> uint32_t {
    var = var << 16i32 | var >> 16i32;
    var = (var & 0xff00ff00u32) >> 8i32 | var << 8i32 & 0xff00ff00u32;
    return var;
}
#[inline]
unsafe extern "C" fn Endian64_Swap(mut var: uint64_t) -> uint64_t {
    var = var >> 32i32 | var << 32i32;
    var =
        var >> 16i32 & 0xffff0000ffffi64 as libc::c_ulonglong |
            (var & 0xffff0000ffffi64 as libc::c_ulonglong) << 16i32;
    var =
        var >> 8i32 & 0xff00ff00ff00ffi64 as libc::c_ulonglong |
            (var & 0xff00ff00ff00ffi64 as libc::c_ulonglong) << 8i32;
    return var;
}
#[inline]
unsafe extern "C" fn intswap32(mut n: uint32_t) -> uint32_t {
    if mfsLSB == 0i32 { return n }
    return Endian32_Swap(n);
}
#[inline]
unsafe extern "C" fn intswap64(mut n: uint64_t) -> uint64_t {
    if mfsLSB == 0i32 { return n }
    return Endian64_Swap(n);
}
#[inline]
unsafe extern "C" fn sectorswap64(mut n: uint64_t) -> uint64_t {
    let mut ret: uint64_t = 0;
    // *NOTE*  Little endian drives (Roamio) have reversed hi an lo 32 bits
    if mfsLSB == 0i32 { ret = n } else { ret = Endian64_Swap(n) }
    if mfsLSB == 1i32 { ret = ret >> 32i32 | ret << 32i32 }
    return ret;
}
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
    let mut sectors: uint64_t =
        (*mfshnd).zones[ztMedia as libc::c_int as usize].size;
    if sectors > (72i32 * 1024i32 * 1024i32 * 2i32) as libc::c_ulonglong {
        sectors =
            (sectors as
                 libc::c_ulonglong).wrapping_sub((12i32 * 1024i32 * 1024i32 *
                                                      2i32) as
                                                     libc::c_ulonglong) as
                uint64_t as uint64_t
    } else if sectors >
                  (14i32 * 1024i32 * 1024i32 * 2i32) as libc::c_ulonglong {
        sectors =
            (sectors as
                 libc::c_ulonglong).wrapping_sub(sectors.wrapping_sub((14i32 *
                                                                           1024i32
                                                                           *
                                                                           1024i32
                                                                           *
                                                                           2i32)
                                                                          as
                                                                          libc::c_ulonglong).wrapping_div(4i32
                                                                                                              as
                                                                                                              libc::c_ulonglong))
                as uint64_t as uint64_t
    }
    return sectors.wrapping_div(1630000i32 as libc::c_ulonglong) as
               libc::c_uint;
}
/* ****************************************************************************/
/* Return the count of inodes.  Each inode is 2 sectors, so the count is the */
/* size of the inode zone maps divided by 2. */
#[no_mangle]
pub unsafe extern "C" fn mfs_inode_count(mut mfshnd: *mut mfs_handle)
 -> uint32_t {
    return (*mfshnd).zones[ztInode as libc::c_int as
                               usize].size.wrapping_div(2i32 as
                                                            libc::c_ulonglong)
               as uint32_t;
}
/* ***************************************/
/* Find the sector number for an inode. */
#[no_mangle]
pub unsafe extern "C" fn mfs_inode_to_sector(mut mfshnd: *mut mfs_handle,
                                             mut inode: libc::c_uint)
 -> uint64_t {
    let mut cur: *mut zone_map = 0 as *mut zone_map;
    let mut sector: uint64_t =
        inode.wrapping_mul(2i32 as libc::c_uint) as uint64_t;
    /* Don't bother if it's not a valid inode. */
    if inode >= mfs_inode_count(mfshnd) { return 0i32 as uint64_t }
    if 0 != (*mfshnd).is_64 {
        /* Loop through each inode map, seeing if the current inode is within it. */
        cur = (*mfshnd).zones[ztInode as libc::c_int as usize].next;
        while !cur.is_null() {
            if sector < intswap64((*(*cur).map).z64.size) {
                return sector.wrapping_add(intswap64((*(*cur).map).z64.first))
            }
            /* If not, subtract the size so the inode sector offset is now relative to */
/* the next inode zone. */
            sector =
                (sector as
                     libc::c_ulonglong).wrapping_sub(intswap64((*(*cur).map).z64.size))
                    as uint64_t as uint64_t;
            cur = (*cur).next
        }
    } else {
        /* Loop through each inode map, seeing if the current inode is within it. */
        cur = (*mfshnd).zones[ztInode as libc::c_int as usize].next;
        while !cur.is_null() {
            if sector < intswap32((*(*cur).map).z32.size) as libc::c_ulonglong
               {
                return sector.wrapping_add(intswap32((*(*cur).map).z32.first)
                                               as libc::c_ulonglong)
            }
            /* If not, subtract the size so the inode sector offset is now relative to */
/* the next inode zone. */
            sector =
                (sector as
                     libc::c_ulonglong).wrapping_sub(intswap32((*(*cur).map).z32.size)
                                                         as libc::c_ulonglong)
                    as uint64_t as uint64_t;
            cur = (*cur).next
        }
    }
    /* This should never happen. */
    (*mfshnd).err_msg =
        b"Inode %d out of bounds\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    (*mfshnd).err_arg1 = inode as int64_t;
    return 0i32 as uint64_t;
}
#[inline]
unsafe extern "C" fn mfs_zone_for_block(mut mfshnd: *mut mfs_handle,
                                        mut sector: uint64_t,
                                        mut size: uint64_t) -> *mut zone_map {
    let mut zone: *mut zone_map = 0 as *mut zone_map;
    if 0 != (*mfshnd).is_64 {
        /* Find the zone to update based on the start sector */
        zone = (*mfshnd).loaded_zones;
        while !zone.is_null() {
            if sector >= intswap64((*(*zone).map).z64.first) &&
                   sector <= intswap64((*(*zone).map).z64.last) {
                break ;
            }
            zone = (*zone).next_loaded
        }
    } else {
        /* Find the zone to update based on the start sector */
        zone = (*mfshnd).loaded_zones;
        while !zone.is_null() {
            if sector >=
                   intswap32((*(*zone).map).z32.first) as libc::c_ulonglong &&
                   sector <=
                       intswap32((*(*zone).map).z32.last) as libc::c_ulonglong
               {
                break ;
            }
            zone = (*zone).next_loaded
        }
    }
    if zone.is_null() {
        (*mfshnd).err_msg =
            b"Sector %u out of bounds for zone map\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char;
        (*mfshnd).err_arg1 = sector as int64_t;
        return 0 as *mut zone_map
    }
    if 0 != (*mfshnd).is_64 &&
           sector.wrapping_add(size).wrapping_sub(1i32 as libc::c_ulonglong) >
               intswap64((*(*zone).map).z64.last) ||
           0 == (*mfshnd).is_64 &&
               sector.wrapping_add(size).wrapping_sub(1i32 as
                                                          libc::c_ulonglong) >
                   intswap32((*(*zone).map).z32.last) as libc::c_ulonglong {
        (*mfshnd).err_msg =
            b"Sector %u size %d crosses zone map boundry\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char;
        (*mfshnd).err_arg1 = sector as int64_t;
        (*mfshnd).err_arg2 = size as int64_t;
        return 0 as *mut zone_map
    }
    if 0 != (*mfshnd).is_64 &&
           0 !=
               sector.wrapping_sub(intswap64((*(*zone).map).z64.first)).wrapping_rem(size)
           ||
           0 == (*mfshnd).is_64 &&
               0 !=
                   sector.wrapping_sub(intswap32((*(*zone).map).z32.first) as
                                           libc::c_ulonglong).wrapping_rem(size)
       {
        (*mfshnd).err_msg =
            b"Sector %u size %d not aligned with zone map\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char;
        (*mfshnd).err_arg1 = sector as int64_t;
        (*mfshnd).err_arg2 = size as int64_t;
        return 0 as *mut zone_map
    }
    return zone;
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
    bit = intswap32((1i32 << bit) as uint32_t);
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
    bit = intswap32((1i32 << bit) as uint32_t);
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
    bit = intswap32((1i32 << bit) as uint32_t);
    *mapints &= !bit;
}
/* ***********************************************************************/
/* Get the current state of a specifc block in the zone map */
/* This checks only for the explicit size, not that the block could be part */
/* of a larger free block, for example. */
#[no_mangle]
pub unsafe extern "C" fn mfs_zone_map_block_state(mut mfshnd: *mut mfs_handle,
                                                  mut sector: uint64_t,
                                                  mut size: uint64_t)
 -> libc::c_int {
    let mut order: libc::c_int = 0;
    let mut minalloc: libc::c_uint = 0;
    let mut numbitmaps: libc::c_uint = 0;
    let mut first: uint64_t = 0;
    let mut zone: *mut zone_map = mfs_zone_for_block(mfshnd, sector, size);
    if zone.is_null() { return -1i32 }
    if 0 != (*mfshnd).is_64 {
        minalloc = intswap32((*(*zone).map).z64.min);
        numbitmaps = intswap32((*(*zone).map).z64.num);
        first = intswap64((*(*zone).map).z64.first)
    } else {
        minalloc = intswap32((*(*zone).map).z32.min);
        numbitmaps = intswap32((*(*zone).map).z32.num);
        first = intswap32((*(*zone).map).z32.first) as uint64_t
    }
    /* Find which level of bitmaps this block is on */
    order = 0i32;
    while (order as libc::c_uint) < numbitmaps {
        if (minalloc << order) as libc::c_ulonglong >= size { break ; }
        order += 1
    }
    /* One last set of sanity checks on the size */
    if order as libc::c_uint >= numbitmaps {
        /* Should be caught by above check that it crosses the zone map boundry */
        (*mfshnd).err_msg =
            b"Sector %u size %d too large for zone map\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char;
        (*mfshnd).err_arg1 = sector as int64_t;
        (*mfshnd).err_arg2 = size as int64_t;
        return -1i32
    }
    if (minalloc as uint64_t) << order != size {
        (*mfshnd).err_msg =
            b"Sector %u size %d not multiple of zone map allocation\x00" as
                *const u8 as *const libc::c_char as *mut libc::c_char;
        (*mfshnd).err_arg1 = sector as int64_t;
        (*mfshnd).err_arg2 = size as int64_t;
        return -1i32
    }
    /* Return the current state as 1 or 0 */
    return if 0 !=
                  mfs_zone_map_bit_state_get(*(*zone).bitmaps.offset(order as
                                                                         isize),
                                             (sector.wrapping_sub(first) >>
                                                  order).wrapping_div(minalloc
                                                                          as
                                                                          libc::c_ulonglong)
                                                 as libc::c_uint) {
               1i32
           } else { 0i32 };
}
/* ***********************************************************************/
/* Allocate or free a block out of the bitmap */
#[no_mangle]
pub unsafe extern "C" fn mfs_zone_map_update(mut mfshnd: *mut mfs_handle,
                                             mut sector: uint64_t,
                                             mut size: uint64_t,
                                             mut state: libc::c_uint,
                                             mut logstamp: libc::c_uint)
 -> libc::c_int {
    let mut zone: *mut zone_map = 0 as *mut zone_map;
    let mut order: libc::c_int = 0;
    let mut orderfree: libc::c_int = 0;
    let mut mapbit: libc::c_uint = 0;
    let mut minalloc: libc::c_uint = 0;
    let mut numbitmaps: libc::c_uint = 0;
    zone = mfs_zone_for_block(mfshnd, sector, size);
    if zone.is_null() { return 0i32 }
    /* Check the logstamp to see if this has already been updated...  */
	/* Sure, there could be some integer wrap... */
	/* After a hundred or so years */
    if 0 != (*mfshnd).is_64 &&
           logstamp <= intswap32((*(*zone).map).z64.logstamp) ||
           0 == (*mfshnd).is_64 &&
               logstamp <= intswap32((*(*zone).map).z32.logstamp) {
        return 1i32
    }
    /* From this point on, it is assumed that the request makes sense */
	/* For example, no request to free a block that is partly free, or to */
	/* allocate a block that is partly allocated */
	/* Allocating a block that is fully allocated or freeing a block that */
	/* is fully free is fine, however. */
    if 0 != (*mfshnd).is_64 {
        minalloc = intswap32((*(*zone).map).z64.min);
        numbitmaps = intswap32((*(*zone).map).z64.num)
    } else {
        minalloc = intswap32((*(*zone).map).z32.min);
        numbitmaps = intswap32((*(*zone).map).z32.num)
    }
    /* Find which level of bitmaps this block is on */
    order = 0i32;
    while (order as libc::c_uint) < numbitmaps {
        if (minalloc as uint64_t) << order >= size { break ; }
        order += 1
    }
    /* One last set of sanity checks on the size */
    if order as libc::c_uint >= numbitmaps {
        /* Should be caught by above check that it crosses the zone map boundry */
        (*mfshnd).err_msg =
            b"Sector %u size %d too large for zone map\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char;
        (*mfshnd).err_arg1 = sector as int64_t;
        (*mfshnd).err_arg2 = size as int64_t;
        return 0i32
    }
    if (minalloc as uint64_t) << order != size {
        (*mfshnd).err_msg =
            b"Sector %u size %d not multiple of zone map allocation\x00" as
                *const u8 as *const libc::c_char as *mut libc::c_char;
        (*mfshnd).err_arg1 = sector as int64_t;
        (*mfshnd).err_arg2 = size as int64_t;
        return 0i32
    }
    if 0 != (*mfshnd).is_64 {
        mapbit =
            sector.wrapping_sub(intswap64((*(*zone).map).z64.first)).wrapping_div((minalloc
                                                                                       as
                                                                                       uint64_t)
                                                                                      <<
                                                                                      order)
                as libc::c_uint
    } else {
        mapbit =
            sector.wrapping_sub(intswap32((*(*zone).map).z32.first) as
                                    libc::c_ulonglong).wrapping_div((minalloc
                                                                         as
                                                                         uint64_t)
                                                                        <<
                                                                        order)
                as libc::c_uint
    }
    /* Find the first free bit */
    orderfree = order;
    while (orderfree as libc::c_uint) < numbitmaps {
        if 0 !=
               mfs_zone_map_bit_state_get(*(*zone).bitmaps.offset(orderfree as
                                                                      isize),
                                          mapbit >> orderfree - order) {
            break ;
        }
        orderfree += 1
    }
    /* Free bit not found */
    if orderfree as libc::c_uint >= numbitmaps { orderfree = -1i32 }
    if 0 != state {
        /* Free a block */
        if orderfree >= 0i32 {
            /* Already free */
            return 1i32
        }
        /* Set the bit to mark it free */
        if 0 != (*mfshnd).is_64 {
            (*(*zone).map).z64.free =
                intswap64(intswap64((*(*zone).map).z64.free).wrapping_add(size))
        } else {
            (*(*zone).map).z32.free =
                intswap32((intswap32((*(*zone).map).z32.free) as
                               libc::c_ulonglong).wrapping_add(size) as
                              uint32_t)
        }
        mfs_zone_map_bit_state_set(*(*zone).bitmaps.offset(order as isize),
                                   mapbit);
        (**(*zone).bitmaps.offset(order as isize)).freeblocks =
            intswap32(intswap32((**(*zone).bitmaps.offset(order as
                                                              isize)).freeblocks).wrapping_add(1i32
                                                                                                   as
                                                                                                   libc::c_uint));
        /* Coalesce neighboring free bits into larger blocks */
        while ((order + 1i32) as libc::c_uint) < numbitmaps &&
                  0 !=
                      mfs_zone_map_bit_state_get(*(*zone).bitmaps.offset(order
                                                                             as
                                                                             isize),
                                                 mapbit ^
                                                     1i32 as libc::c_uint) {
            /* Clear the bit and it's neighbor in the bitmap */
            mfs_zone_map_bit_state_clear(*(*zone).bitmaps.offset(order as
                                                                     isize),
                                         mapbit);
            mfs_zone_map_bit_state_clear(*(*zone).bitmaps.offset(order as
                                                                     isize),
                                         mapbit ^ 1i32 as libc::c_uint);
            (**(*zone).bitmaps.offset(order as isize)).freeblocks =
                intswap32(intswap32((**(*zone).bitmaps.offset(order as
                                                                  isize)).freeblocks).wrapping_sub(2i32
                                                                                                       as
                                                                                                       libc::c_uint));
            /* Move on to the next bitmap */
            order += 1;
            mapbit >>= 1i32;
            /* Set the single bit in the next bitmap that represents both bits cleared */
            mfs_zone_map_bit_state_set(*(*zone).bitmaps.offset(order as
                                                                   isize),
                                       mapbit);
            (**(*zone).bitmaps.offset(order as isize)).freeblocks =
                intswap32(intswap32((**(*zone).bitmaps.offset(order as
                                                                  isize)).freeblocks).wrapping_add(1i32
                                                                                                       as
                                                                                                       libc::c_uint))
        }
        /* Mark it dirty */
        (*zone).dirty = 1i32;
        /* Done! */
        return 1i32
    } else {
        /* Allocate a block */
        if orderfree < 0i32 {
            /* Already allocated */
            return 1i32
        }
        /* Set all the bit as free that are left over from borrowing from larger chunks */
        while order < orderfree {
            mfs_zone_map_bit_state_set(*(*zone).bitmaps.offset(order as
                                                                   isize),
                                       mapbit ^ 1i32 as libc::c_uint);
            (**(*zone).bitmaps.offset(order as isize)).freeblocks =
                intswap32(intswap32((**(*zone).bitmaps.offset(order as
                                                                  isize)).freeblocks).wrapping_add(1i32
                                                                                                       as
                                                                                                       libc::c_uint));
            /* Move on to the next bitmap */
            order += 1;
            mapbit >>= 1i32
        }
        /* Clear the bit to mark it allocated */
        if 0 != (*mfshnd).is_64 {
            (*(*zone).map).z64.free =
                intswap64(intswap64((*(*zone).map).z64.free).wrapping_sub(size))
        } else {
            (*(*zone).map).z32.free =
                intswap32((intswap32((*(*zone).map).z32.free) as
                               libc::c_ulonglong).wrapping_sub(size) as
                              uint32_t)
        }
        mfs_zone_map_bit_state_clear(*(*zone).bitmaps.offset(order as isize),
                                     mapbit);
        (**(*zone).bitmaps.offset(order as isize)).freeblocks =
            intswap32(intswap32((**(*zone).bitmaps.offset(order as
                                                              isize)).freeblocks).wrapping_sub(1i32
                                                                                                   as
                                                                                                   libc::c_uint));
        /* Set the last bit allocated - bit numbering is 1 based here (Or maybe it's next bit after last allocated) */
		/* Hypothesis: This is used as a base for the search for next free bit */
        (**(*zone).bitmaps.offset(order as isize)).last =
            intswap32(mapbit.wrapping_add(1i32 as libc::c_uint));
        /* Mark it dirty */
        (*zone).dirty = 1i32;
        /* Done! */
        return 1i32
    };
}
/* ***********************************************************************/
/* Clean up storage used by tracking changes */
unsafe extern "C" fn mfs_zone_map_clear_changes(mut mfshnd: *mut mfs_handle,
                                                mut zone: *mut zone_map) {
    let mut numbitmaps: libc::c_int = 0;
    let mut loop_0: libc::c_int = 0;
    if 0 != (*mfshnd).is_64 {
        numbitmaps = intswap32((*(*zone).map).z64.num) as libc::c_int
    } else { numbitmaps = intswap32((*(*zone).map).z32.num) as libc::c_int }
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
            if 0 != (*mfshnd).is_64 {
                (*(*zone).map).z64.logstamp = intswap32(logstamp)
            } else { (*(*zone).map).z32.logstamp = intswap32(logstamp) }
            mfs_zone_map_clear_changes(mfshnd, zone);
        }
        zone = (*zone).next_loaded
    };
}
/* ***********************************************************************/
/* Write changed zone maps back to disk */
#[no_mangle]
pub unsafe extern "C" fn mfs_zone_map_sync(mut mfshnd: *mut mfs_handle,
                                           mut logstamp: libc::c_uint)
 -> libc::c_int {
    let mut zone: *mut zone_map = 0 as *mut zone_map;
    zone = (*mfshnd).loaded_zones;
    while !zone.is_null() {
        if 0 != (*zone).dirty {
            let mut towrite: libc::c_int = 0;
            let mut sector: uint64_t = 0;
            let mut sbackup: uint64_t = 0;
            if 0 != (*mfshnd).is_64 {
                (*(*zone).map).z64.logstamp = intswap32(logstamp);
                towrite = intswap32((*(*zone).map).z64.length) as libc::c_int;
                sector = intswap64((*(*zone).map).z64.sector);
                sbackup = intswap64((*(*zone).map).z64.sbackup);
                mfs_update_crc((*zone).map as *mut libc::c_uchar,
                               (towrite * 512i32) as libc::c_uint,
                               (&mut (*(*zone).map).z64.checksum as
                                    *mut uint32_t as
                                    *mut libc::c_uint).wrapping_offset_from((*zone).map
                                                                                as
                                                                                *mut libc::c_uint)
                                   as libc::c_long as libc::c_uint);
            } else {
                (*(*zone).map).z32.logstamp = intswap32(logstamp);
                towrite = intswap32((*(*zone).map).z32.length) as libc::c_int;
                sector = intswap32((*(*zone).map).z32.sector) as uint64_t;
                sbackup = intswap32((*(*zone).map).z32.sbackup) as uint64_t;
                mfs_update_crc((*zone).map as *mut libc::c_uchar,
                               (towrite * 512i32) as libc::c_uint,
                               (&mut (*(*zone).map).z32.checksum as
                                    *mut uint32_t as
                                    *mut libc::c_uint).wrapping_offset_from((*zone).map
                                                                                as
                                                                                *mut libc::c_uint)
                                   as libc::c_long as libc::c_uint);
            }
            if mfsvol_write_data((*mfshnd).vols,
                                 (*zone).map as *mut libc::c_void, sector,
                                 towrite as uint32_t) < 0i32 {
                return -1i32
            }
            if mfsvol_write_data((*mfshnd).vols,
                                 (*zone).map as *mut libc::c_void, sbackup,
                                 towrite as uint32_t) < 0i32 {
                return -1i32
            }
            mfs_zone_map_clear_changes(mfshnd, zone);
        }
        zone = (*zone).next_loaded
    }
    return 1i32;
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
/* ***************************************************************************/
/* Create a new zone map at the requested sector, pointing to the requested */
/* sector, and link it in. */
#[no_mangle]
pub unsafe extern "C" fn mfs_new_zone_map(mut mfshnd: *mut mfs_handle,
                                          mut sector: uint64_t,
                                          mut backup: uint64_t,
                                          mut first: uint64_t,
                                          mut size: uint64_t,
                                          mut minalloc: libc::c_uint,
                                          mut type_0: zone_type,
                                          mut fsmem_base: libc::c_uint)
 -> libc::c_int {
    let mut blocks: uint64_t =
        size.wrapping_div(minalloc as libc::c_ulonglong);
    let mut zonesize: libc::c_int =
        mfs_new_zone_map_size(mfshnd, blocks as libc::c_uint) + 511i32 &
            !511i32;
    let mut buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut zone: *mut zone_header = 0 as *mut zone_header;
    let mut last: *mut zone_header = 0 as *mut zone_header;
    let mut cur: *mut zone_map = 0 as *mut zone_map;
    let mut loop_0: libc::c_int = 0;
    let mut order: libc::c_int = 0i32;
    let mut fsmem_pointers: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut curofs: *mut libc::c_uint = 0 as *mut libc::c_uint;
    /* Truncate the size to the nearest allocation sized block. */
    size =
        (size as
             libc::c_ulonglong).wrapping_sub(size.wrapping_rem(minalloc as
                                                                   libc::c_ulonglong))
            as uint64_t as uint64_t;
    /* Find the last loaded zone. */
    cur = (*mfshnd).loaded_zones;
    while !cur.is_null() && !(*cur).next_loaded.is_null() {
        cur = (*cur).next_loaded
    }
    if !cur.is_null() {
        last = (*cur).map
    } else { last = 0 as *mut zone_header }
    /* To get the pointer into fsmem, start with the first pointer from the */
/* previous zone map.  Subtract the header and all the fsmem pointers from */
/* it, and thats the base for that map.  Now add in all the sectors from that */
/* map, plus 1 extra and 8 bytes. */
/* It looks like it's more complicated than that now...  But it also looks */
/* like the numbers don't matter and TiVo will correct them itself because */
/* this still works. */
    if 0 == fsmem_base {
        if last.is_null() {
            (*mfshnd).err_msg =
                b"Attempt to create first zone without fsmem base\x00" as
                    *const u8 as *const libc::c_char as *mut libc::c_char;
            return -1i32
        }
        if 0 != (*mfshnd).is_64 {
            fsmem_base =
                (intswap32(*((&mut (*last).z64 as
                                  *mut zone_header_64).offset(1isize) as
                                 *mut libc::c_uint)) as
                     libc::c_ulong).wrapping_sub((::std::mem::size_of::<zone_header_64>()
                                                      as
                                                      libc::c_ulong).wrapping_add(intswap32((*last).z64.num).wrapping_mul(4i32
                                                                                                                              as
                                                                                                                              libc::c_uint)
                                                                                      as
                                                                                      libc::c_ulong)).wrapping_add(intswap32((*last).z64.length).wrapping_mul(512i32
                                                                                                                                                                  as
                                                                                                                                                                  libc::c_uint)
                                                                                                                       as
                                                                                                                       libc::c_ulong).wrapping_add(512i32
                                                                                                                                                       as
                                                                                                                                                       libc::c_ulong).wrapping_add(8i32
                                                                                                                                                                                       as
                                                                                                                                                                                       libc::c_ulong)
                    as libc::c_uint
        } else {
            fsmem_base =
                (intswap32(*((&mut (*last).z32 as
                                  *mut zone_header_32).offset(1isize) as
                                 *mut libc::c_uint)) as
                     libc::c_ulong).wrapping_sub((::std::mem::size_of::<zone_header_32>()
                                                      as
                                                      libc::c_ulong).wrapping_add(intswap32((*last).z32.num).wrapping_mul(4i32
                                                                                                                              as
                                                                                                                              libc::c_uint)
                                                                                      as
                                                                                      libc::c_ulong)).wrapping_add(intswap32((*last).z32.length).wrapping_mul(512i32
                                                                                                                                                                  as
                                                                                                                                                                  libc::c_uint)
                                                                                                                       as
                                                                                                                       libc::c_ulong).wrapping_add(512i32
                                                                                                                                                       as
                                                                                                                                                       libc::c_ulong).wrapping_add(8i32
                                                                                                                                                                                       as
                                                                                                                                                                                       libc::c_ulong)
                    as libc::c_uint
        }
    }
    buf = malloc(zonesize as libc::c_ulong) as *mut libc::c_uchar;
    if buf.is_null() { return -1i32 }
    memset(buf as *mut libc::c_void, 0xaai32, zonesize as libc::c_ulong);
    /* Figure out the order of the blocks count. */
    while ((1i32 << order) as libc::c_ulonglong) < blocks { order += 1 }
    order += 1;
    zone = buf as *mut zone_header;
    /* Fill in the header values. */
    if 0 != (*mfshnd).is_64 {
        (*zone).z64.sector = intswap64(sector);
        (*zone).z64.sbackup = intswap64(backup);
        (*zone).z64.length = intswap32((zonesize / 512i32) as uint32_t);
        (*zone).z64.next_sector = 0i32 as uint64_t;
        (*zone).z64.next_length = 0i32 as uint32_t;
        (*zone).z64.next_size = 0i32 as uint64_t;
        (*zone).z64.next_min = 0i32 as uint32_t;
        (*zone).z64.type_0 = intswap32(type_0 as uint32_t) as zone_type;
        (*zone).z64.logstamp = intswap32(mfs_log_last_sync(mfshnd));
        (*zone).z64.checksum = intswap32(0xdeadf00du32);
        (*zone).z64.first = intswap64(first);
        (*zone).z64.last =
            intswap64(first.wrapping_add(size).wrapping_sub(1i32 as
                                                                libc::c_ulonglong));
        (*zone).z64.size = intswap64(size);
        (*zone).z64.min = intswap32(minalloc);
        (*zone).z64.free = intswap64(size);
        (*zone).z64.zero = 0i32 as uint32_t;
        (*zone).z64.num = intswap32(order as uint32_t);
        /* Grab a pointer to the array where fsmem pointers will go. */
        fsmem_pointers =
            (&mut (*zone).z64 as *mut zone_header_64).offset(1isize) as
                *mut libc::c_uint;
        curofs =
            ((&mut (*zone).z64 as *mut zone_header_64).offset(1isize) as
                 *mut libc::c_uint).offset(order as isize)
    } else {
        (*zone).z32.sector = intswap32(sector as uint32_t);
        (*zone).z32.sbackup = intswap32(backup as uint32_t);
        (*zone).z32.length = intswap32((zonesize / 512i32) as uint32_t);
        (*zone).z32.next.sector = 0i32 as uint32_t;
        (*zone).z32.next.length = 0i32 as uint32_t;
        (*zone).z32.next.size = 0i32 as uint32_t;
        (*zone).z32.next.min = 0i32 as uint32_t;
        (*zone).z32.type_0 = intswap32(type_0 as uint32_t) as zone_type;
        (*zone).z32.logstamp = intswap32(mfs_log_last_sync(mfshnd));
        (*zone).z32.checksum = intswap32(0xdeadf00du32);
        (*zone).z32.first = intswap32(first as uint32_t);
        (*zone).z32.last =
            intswap32(first.wrapping_add(size).wrapping_sub(1i32 as
                                                                libc::c_ulonglong)
                          as uint32_t);
        (*zone).z32.size = intswap32(size as uint32_t);
        (*zone).z32.min = intswap32(minalloc);
        (*zone).z32.free = intswap32(size as uint32_t);
        (*zone).z32.zero = 0i32 as uint32_t;
        (*zone).z32.num = intswap32(order as uint32_t);
        /* Grab a pointer to the array where fsmem pointers will go. */
        fsmem_pointers =
            (&mut (*zone).z32 as *mut zone_header_32).offset(1isize) as
                *mut libc::c_uint;
        curofs =
            ((&mut (*zone).z32 as *mut zone_header_32).offset(1isize) as
                 *mut libc::c_uint).offset(order as isize)
    }
    /* Fill in the allocation bitmaps.  This is simpler than it sounds.  The */
/* bitmaps are regressing from the full 1 bit = min allocation block up to */
/* 1 bit = entire drive.  A bit means the block is free.  Free blocks are */
/* represented by the largest bit possible.  In a perfect power of 2, a */
/* completely free table is represented by 1 bit in the last table.  This */
/* may sound complex, but it's really easy to fill in an empty table. */
/* While filling in the size values for the headers for each bitmap, any */
/* time you have an odd number of active bits, set the last one, because */
/* it is not represented by any larger bits. */
    loop_0 = 0i32;
    loop  {
        let fresh2 = order;
        order = order - 1;
        if !(fresh2 > 0i32) { break ; }
        let mut nbits: libc::c_int = 0;
        let mut nints: libc::c_int = 0;
        let mut bitmap: *mut bitmap_header = curofs as *mut bitmap_header;
        *fsmem_pointers.offset(loop_0 as isize) =
            intswap32((curofs as
                           *mut libc::c_char).offset(fsmem_base as
                                                         isize).wrapping_offset_from(zone
                                                                                         as
                                                                                         *mut libc::c_char)
                          as libc::c_long as uint32_t);
        /* Set in the basic, constant header values.  The nbits is how many bits */
/* there are in the table, including extra inactive bits padding to the */
/* next power of 2.  The nints represents how many ints those bits take up. */
        nbits = 1i32 << order;
        (*bitmap).nbits = intswap32(nbits as uint32_t);
        nints = (nbits + 31i32) / 32i32;
        (*bitmap).nints = intswap32(nints as uint32_t);
        /* Clear all the bits by default. */
        memset(curofs.offset((::std::mem::size_of::<bitmap_header>() as
                                  libc::c_ulong).wrapping_div(4i32 as
                                                                  libc::c_ulong)
                                 as isize) as *mut libc::c_void, 0i32,
               (nints * 4i32) as libc::c_ulong);
        /* Set the rest of the header.  The */
/* reason to set the last bit is that this is the last table that block */
/* will be represented in, so it needs to be marked free here.  The next */
/* table's bit is too big it overflows into the inactive area, so is itself */
/* inactive. */
        if 0 != blocks & 1i32 as libc::c_ulonglong {
            (*bitmap).freeblocks = intswap32(1i32 as uint32_t);
            *curofs.offset((4i32 as
                                libc::c_ulonglong).wrapping_add(blocks.wrapping_sub(1i32
                                                                                        as
                                                                                        libc::c_ulonglong).wrapping_div(32i32
                                                                                                                            as
                                                                                                                            libc::c_ulonglong))
                               as isize) =
                intswap32((1i32 <<
                               (31i32 as
                                    libc::c_ulonglong).wrapping_sub(blocks.wrapping_sub(1i32
                                                                                            as
                                                                                            libc::c_ulonglong).wrapping_rem(32i32
                                                                                                                                as
                                                                                                                                libc::c_ulonglong)))
                              as uint32_t)
        } else { (*bitmap).freeblocks = 0i32 as uint32_t }
        (*bitmap).last = 0i32 as uint32_t;
        /* Step past this table. */
        curofs =
            curofs.offset((::std::mem::size_of::<bitmap_header>() as
                               libc::c_ulong).wrapping_div(4i32 as
                                                               libc::c_ulong).wrapping_add(((nbits
                                                                                                 +
                                                                                                 57i32)
                                                                                                /
                                                                                                32i32)
                                                                                               as
                                                                                               libc::c_ulong)
                              as isize);
        loop_0 += 1;
        blocks =
            (blocks as
                 libc::c_ulonglong).wrapping_div(2i32 as libc::c_ulonglong) as
                uint64_t as uint64_t
    }
    if 0 != (*mfshnd).is_64 {
        /* Copy the pointer into the current end of the zone list. */
        if !last.is_null() {
            (*last).z64.next_sector = (*zone).z64.sector;
            (*last).z64.next_sbackup = (*zone).z64.sbackup;
            (*last).z64.next_length = (*zone).z64.length;
            (*last).z64.next_size = (*zone).z64.size;
            (*last).z64.next_min = (*zone).z64.min;
            /* Update the CRC in the new zone, as well as the previous tail, since it's */
/* next pointer was updated. */
            mfs_update_crc(last as *mut libc::c_uchar,
                           intswap32((*last).z64.length).wrapping_mul(512i32
                                                                          as
                                                                          libc::c_uint),
                           (&mut (*last).z64.checksum as *mut uint32_t as
                                *mut libc::c_uint).wrapping_offset_from(last
                                                                            as
                                                                            *mut libc::c_uint)
                               as libc::c_long as libc::c_uint);
        } else {
            (*mfshnd).vol_hdr.v64.zonemap.sector = (*zone).z64.sector;
            (*mfshnd).vol_hdr.v64.zonemap.sbackup = (*zone).z64.sbackup;
            (*mfshnd).vol_hdr.v64.zonemap.length =
                intswap64(intswap32((*zone).z64.length) as uint64_t);
            (*mfshnd).vol_hdr.v64.zonemap.size = (*zone).z64.size;
            (*mfshnd).vol_hdr.v64.zonemap.min =
                intswap64(intswap32((*zone).z64.min) as uint64_t)
        }
        mfs_update_crc(zone as *mut libc::c_uchar,
                       intswap32((*zone).z64.length).wrapping_mul(512i32 as
                                                                      libc::c_uint),
                       (&mut (*zone).z64.checksum as *mut uint32_t as
                            *mut libc::c_uint).wrapping_offset_from(zone as
                                                                        *mut libc::c_uint)
                           as libc::c_long as libc::c_uint);
    } else {
        /* Copy the pointer into the current end of the zone list. */
        if !last.is_null() {
            (*last).z32.next.sector = (*zone).z32.sector;
            (*last).z32.next.sbackup = (*zone).z32.sbackup;
            (*last).z32.next.length = (*zone).z32.length;
            (*last).z32.next.size = (*zone).z32.size;
            (*last).z32.next.min = (*zone).z32.min;
            /* Update the CRC in the new zone, as well as the previous tail, since it's */
/* next pointer was updated. */
            mfs_update_crc(last as *mut libc::c_uchar,
                           intswap32((*last).z32.length).wrapping_mul(512i32
                                                                          as
                                                                          libc::c_uint),
                           (&mut (*last).z32.checksum as *mut uint32_t as
                                *mut libc::c_uint).wrapping_offset_from(last
                                                                            as
                                                                            *mut libc::c_uint)
                               as libc::c_long as libc::c_uint);
        } else {
            (*mfshnd).vol_hdr.v32.zonemap.sector = (*zone).z32.sector;
            (*mfshnd).vol_hdr.v32.zonemap.sbackup = (*zone).z32.sbackup;
            (*mfshnd).vol_hdr.v32.zonemap.length = (*zone).z32.length;
            (*mfshnd).vol_hdr.v32.zonemap.size = (*zone).z32.size;
            (*mfshnd).vol_hdr.v32.zonemap.min = (*zone).z32.min
        }
        mfs_update_crc(zone as *mut libc::c_uchar,
                       intswap32((*zone).z32.length).wrapping_mul(512i32 as
                                                                      libc::c_uint),
                       (&mut (*zone).z32.checksum as *mut uint32_t as
                            *mut libc::c_uint).wrapping_offset_from(zone as
                                                                        *mut libc::c_uint)
                           as libc::c_long as libc::c_uint);
    }
    /* Write the changes, with the changes to live MFS last.  This should use */
/* the journaling facilities, but I don't know how. */
    mfsvol_write_data((*mfshnd).vols, zone as *mut libc::c_void, sector,
                      (zonesize / 512i32) as uint32_t);
    mfsvol_write_data((*mfshnd).vols, zone as *mut libc::c_void, backup,
                      (zonesize / 512i32) as uint32_t);
    if !last.is_null() {
        if 0 != (*mfshnd).is_64 {
            mfsvol_write_data((*mfshnd).vols, last as *mut libc::c_void,
                              intswap64((*last).z64.sector),
                              intswap32((*last).z64.length));
            mfsvol_write_data((*mfshnd).vols, last as *mut libc::c_void,
                              intswap64((*last).z64.sbackup),
                              intswap32((*last).z64.length));
        } else {
            mfsvol_write_data((*mfshnd).vols, last as *mut libc::c_void,
                              intswap32((*last).z32.sector) as uint64_t,
                              intswap32((*last).z32.length));
            mfsvol_write_data((*mfshnd).vols, last as *mut libc::c_void,
                              intswap32((*last).z32.sbackup) as uint64_t,
                              intswap32((*last).z32.length));
        }
    } else { mfs_write_volume_header(mfshnd); }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_volume_pair_app_size(mut mfshnd: *mut mfs_handle,
                                                  mut blocks: uint64_t,
                                                  mut minalloc: libc::c_uint)
 -> uint64_t {
    if minalloc == 0i32 as libc::c_uint {
        minalloc = 0x800i32 as libc::c_uint
    }
    // Make it twice as big as needed for some spare room
    return (2i32 +
                4i32 *
                    ((mfs_new_zone_map_size(mfshnd,
                                            blocks.wrapping_div(minalloc as
                                                                    libc::c_ulonglong)
                                                as libc::c_uint) + 511i32) /
                         512i32) + 1024i32 - 1i32 & !(1024i32 - 1i32)) as
               uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn mfs_can_add_volume_pair(mut mfshnd: *mut mfs_handle,
                                                 mut app: *mut libc::c_char,
                                                 mut media: *mut libc::c_char,
                                                 mut minalloc: libc::c_uint)
 -> libc::c_int {
    let mut cur: *mut zone_map = 0 as *mut zone_map;
    /* If no minalloc, make it default. */
    if minalloc == 0i32 as libc::c_uint {
        minalloc = 0x800i32 as libc::c_uint
    }
    /* Make sure the volumes being added don't overflow the 128 bytes. */
    if 0 != (*mfshnd).is_64 &&
           strlen((*mfshnd).vol_hdr.v64.partitionlist.as_mut_ptr()).wrapping_add(strlen(app)).wrapping_add(strlen(media)).wrapping_add(3i32
                                                                                                                                           as
                                                                                                                                           libc::c_ulong)
               >=
               ::std::mem::size_of::<[libc::c_char; 132]>() as libc::c_ulong
           ||
           0 == (*mfshnd).is_64 &&
               strlen((*mfshnd).vol_hdr.v32.partitionlist.as_mut_ptr()).wrapping_add(strlen(app)).wrapping_add(strlen(media)).wrapping_add(3i32
                                                                                                                                               as
                                                                                                                                               libc::c_ulong)
                   >=
                   ::std::mem::size_of::<[libc::c_char; 128]>() as
                       libc::c_ulong {
        (*mfshnd).err_msg =
            b"No space in volume list for new volumes\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char;
        return -1i32
    }
    /* Make sure block 0 is writable.  It wouldn't do to get all the way to */
/* the end and not be able to update the volume header. */
    if 0 == mfsvol_is_writable((*mfshnd).vols, 0i32 as uint64_t) {
        (*mfshnd).err_msg =
            b"Readonly volume set\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        return -1i32
    }
    /* Walk the list of zone maps to find the last loaded zone map. */
    cur = (*mfshnd).loaded_zones;
    while !cur.is_null() && !(*cur).next_loaded.is_null() {
        cur = (*cur).next_loaded
    }
    /* For cur to be null, it must have never been set. */
    if cur.is_null() {
        (*mfshnd).err_msg =
            b"Zone maps not loaded\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        return -1i32
    }
    /* Check that the last zone map is writable.  This is needed for adding the */
/* new pointer. */
    if 0 != (*mfshnd).is_64 &&
           0 ==
               mfsvol_is_writable((*mfshnd).vols,
                                  intswap64((*(*cur).map).z64.sector)) ||
           0 == (*mfshnd).is_64 &&
               0 ==
                   mfsvol_is_writable((*mfshnd).vols,
                                      intswap32((*(*cur).map).z32.sector) as
                                          uint64_t) {
        (*mfshnd).err_msg =
            b"Readonly volume set\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        return -1i32
    }
    return 0i32;
}
/* **********************************************************************/
/* Add a new set of partitions to the MFS volume set.  In other words, */
/* mfsadd. */
#[no_mangle]
pub unsafe extern "C" fn mfs_add_volume_pair(mut mfshnd: *mut mfs_handle,
                                             mut app: *mut libc::c_char,
                                             mut media: *mut libc::c_char,
                                             mut minalloc: libc::c_uint)
 -> libc::c_int {
    let mut cur: *mut zone_map = 0 as *mut zone_map;
    let mut tpApp: *mut tpFILE = 0 as *mut tpFILE;
    let mut tpMedia: *mut tpFILE = 0 as *mut tpFILE;
    let mut appstart: uint64_t = 0;
    let mut mediastart: uint64_t = 0;
    let mut appsize: uint64_t = 0;
    let mut mediasize: uint64_t = 0;
    let mut mapsize: uint64_t = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut foo: [libc::c_char; 512] = [0; 512];
    /* If no minalloc, make it default. */
    if minalloc == 0i32 as libc::c_uint {
        minalloc = 0x800i32 as libc::c_uint
    }
    /* Make sure the volumes being added don't overflow the 128 bytes. */
    if 0 != (*mfshnd).is_64 &&
           strlen((*mfshnd).vol_hdr.v64.partitionlist.as_mut_ptr()).wrapping_add(strlen(app)).wrapping_add(strlen(media)).wrapping_add(3i32
                                                                                                                                           as
                                                                                                                                           libc::c_ulong)
               >=
               ::std::mem::size_of::<[libc::c_char; 132]>() as libc::c_ulong
           ||
           0 == (*mfshnd).is_64 &&
               strlen((*mfshnd).vol_hdr.v32.partitionlist.as_mut_ptr()).wrapping_add(strlen(app)).wrapping_add(strlen(media)).wrapping_add(3i32
                                                                                                                                               as
                                                                                                                                               libc::c_ulong)
                   >=
                   ::std::mem::size_of::<[libc::c_char; 128]>() as
                       libc::c_ulong {
        (*mfshnd).err_msg =
            b"No space in volume list for new volumes\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char;
        return -1i32
    }
    /* Make sure block 0 is writable.  It wouldn't do to get all the way to */
/* the end and not be able to update the volume header. */
    if 0 == mfsvol_is_writable((*mfshnd).vols, 0i32 as uint64_t) {
        (*mfshnd).err_msg =
            b"Readonly volume set\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        return -1i32
    }
    /* Walk the list of zone maps to find the last loaded zone map. */
    cur = (*mfshnd).loaded_zones;
    while !cur.is_null() && !(*cur).next_loaded.is_null() {
        cur = (*cur).next_loaded
    }
    /* For cur to be null, it must have never been set. */
    if cur.is_null() {
        (*mfshnd).err_msg =
            b"Zone maps not loaded\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        return -1i32
    }
    /* Check that the last zone map is writable.  This is needed for adding the */
/* new pointer. */
    if 0 != (*mfshnd).is_64 &&
           0 ==
               mfsvol_is_writable((*mfshnd).vols,
                                  intswap64((*(*cur).map).z64.sector)) ||
           0 == (*mfshnd).is_64 &&
               0 ==
                   mfsvol_is_writable((*mfshnd).vols,
                                      intswap32((*(*cur).map).z32.sector) as
                                          uint64_t) {
        (*mfshnd).err_msg =
            b"Readonly volume set\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        return -1i32
    }
    tmp = mfsvol_device_translate((*mfshnd).vols, app);
    if tmp.is_null() { return -1i32 }
    if tpApp.is_null() {
        (*mfshnd).err_msg =
            b"%s: %s\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        (*mfshnd).err_arg1 = tmp as size_t as int64_t;
        return -1i32
    }
    tmp = mfsvol_device_translate((*mfshnd).vols, media);
    if tpMedia.is_null() {
        (*mfshnd).err_msg =
            b"%s: %s\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        (*mfshnd).err_arg1 = tmp as size_t as int64_t;
        return -1i32
    }
    tivo_partition_close(tpApp);
    tivo_partition_close(tpMedia);
    if appstart < 0i32 as libc::c_ulonglong ||
           mediastart < 0i32 as libc::c_ulonglong {
        (*mfshnd).err_msg =
            b"Error adding new volumes to set\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char;
        return -1i32
    }
    if 0 == mfsvol_is_writable((*mfshnd).vols, appstart) ||
           0 == mfsvol_is_writable((*mfshnd).vols, mediastart) {
        (*mfshnd).err_msg =
            b"Could not add new volumes writable\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char;
        return -1i32
    }
    appsize = mfsvol_volume_size((*mfshnd).vols, appstart);
    mediasize = mfsvol_volume_size((*mfshnd).vols, mediastart);
    mapsize =
        ((mfs_new_zone_map_size(mfshnd,
                                mediasize.wrapping_div(minalloc as
                                                           libc::c_ulonglong)
                                    as libc::c_uint) + 511i32) / 512i32) as
            uint64_t;
    if mapsize.wrapping_mul(2i32 as
                                libc::c_ulonglong).wrapping_add(2i32 as
                                                                    libc::c_ulonglong)
           > appsize {
        (*mfshnd).err_msg =
            b"New app size too small, need %d more bytes\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char;
        (*mfshnd).err_arg1 =
            mapsize.wrapping_mul(2i32 as
                                     libc::c_ulonglong).wrapping_add(2i32 as
                                                                         libc::c_ulonglong).wrapping_sub(appsize).wrapping_mul(512i32
                                                                                                                                   as
                                                                                                                                   libc::c_ulonglong)
                as int64_t;
        return -1i32
    }
    if mfs_new_zone_map(mfshnd,
                        appstart.wrapping_add(1i32 as libc::c_ulonglong),
                        appstart.wrapping_add(appsize).wrapping_sub(mapsize).wrapping_sub(1i32
                                                                                              as
                                                                                              libc::c_ulonglong),
                        mediastart, mediasize, minalloc, ztMedia,
                        0i32 as libc::c_uint) < 0i32 {
        (*mfshnd).err_msg =
            b"Failed initializing new zone map\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char;
        return -1i32
    }
    if 0 != (*mfshnd).is_64 {
        snprintf(foo.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 132]>() as
                     libc::c_ulong,
                 b"%s %s %s\x00" as *const u8 as *const libc::c_char,
                 (*mfshnd).vol_hdr.v64.partitionlist.as_mut_ptr(), app,
                 media);
        foo[127usize] = 0i32 as libc::c_char;
        strcpy((*mfshnd).vol_hdr.v64.partitionlist.as_mut_ptr(),
               foo.as_mut_ptr());
        (*mfshnd).vol_hdr.v64.total_sectors =
            intswap64(mfsvol_volume_set_size((*mfshnd).vols))
    } else {
        snprintf(foo.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 128]>() as
                     libc::c_ulong,
                 b"%s %s %s\x00" as *const u8 as *const libc::c_char,
                 (*mfshnd).vol_hdr.v32.partitionlist.as_mut_ptr(), app,
                 media);
        foo[127usize] = 0i32 as libc::c_char;
        strcpy((*mfshnd).vol_hdr.v32.partitionlist.as_mut_ptr(),
               foo.as_mut_ptr());
        (*mfshnd).vol_hdr.v32.total_sectors =
            intswap32(mfsvol_volume_set_size((*mfshnd).vols) as uint32_t)
    }
    mfs_write_volume_header(mfshnd);
    mfs_cleanup_zone_maps(mfshnd);
    return mfs_load_zone_maps(mfshnd);
}
/* *****************************************/
/* Free the memory used by the zone maps. */
#[no_mangle]
pub unsafe extern "C" fn mfs_cleanup_zone_maps(mut mfshnd: *mut mfs_handle) {
    let mut loop_0: libc::c_int = 0;
    loop_0 = 0i32;
    while loop_0 < ztMax as libc::c_int {
        while !(*mfshnd).zones[loop_0 as usize].next.is_null() {
            let mut map: *mut zone_map =
                (*mfshnd).zones[loop_0 as usize].next;
            mfs_zone_map_clear_changes(mfshnd, map);
            (*mfshnd).zones[loop_0 as usize].next = (*map).next;
            free((*map).map);
            if !(*map).bitmaps.is_null() { free((*map).bitmaps); }
            if !(*map).changed_runs.is_null() { free((*map).changed_runs); }
            if !(*map).changes.is_null() { free((*map).changes); }
            free(map);
        }
        loop_0 += 1
    }
    (*mfshnd).loaded_zones = 0 as *mut zone_map;
}
/* ************************************************************/
/* Load a zone map from the drive and verify it's integrity. */
unsafe extern "C" fn mfs_load_zone_map(mut mfshnd: *mut mfs_handle,
                                       mut sector: uint64_t,
                                       mut sbackup: uint64_t,
                                       mut length: uint32_t)
 -> *mut zone_header {
    let mut hdr: *mut zone_header =
        calloc(length as libc::c_ulong, 512i32 as libc::c_ulong) as
            *mut zone_header;
    if hdr.is_null() { return 0 as *mut zone_header }
    /* Read the map. */
    mfsvol_read_data((*mfshnd).vols,
                     hdr as *mut libc::c_uchar as *mut libc::c_void, sector,
                     length);
    /* Verify the CRC matches. */
    if 0 != (*mfshnd).is_64 &&
           0 ==
               mfs_check_crc(hdr as *mut libc::c_uchar,
                             length.wrapping_mul(512i32 as libc::c_uint),
                             (&mut (*hdr).z64.checksum as *mut uint32_t as
                                  *mut libc::c_uint).wrapping_offset_from(hdr
                                                                              as
                                                                              *mut libc::c_uchar
                                                                              as
                                                                              *mut libc::c_uint)
                                 as libc::c_long as libc::c_uint) ||
           0 == (*mfshnd).is_64 &&
               0 ==
                   mfs_check_crc(hdr as *mut libc::c_uchar,
                                 length.wrapping_mul(512i32 as libc::c_uint),
                                 (&mut (*hdr).z32.checksum as *mut uint32_t as
                                      *mut libc::c_uint).wrapping_offset_from(hdr
                                                                                  as
                                                                                  *mut libc::c_uchar
                                                                                  as
                                                                                  *mut libc::c_uint)
                                     as libc::c_long as libc::c_uint) {
        /* If the CRC doesn't match, try the backup map. */
        mfsvol_read_data((*mfshnd).vols,
                         hdr as *mut libc::c_uchar as *mut libc::c_void,
                         sbackup, length);
        if 0 != (*mfshnd).is_64 &&
               0 ==
                   mfs_check_crc(hdr as *mut libc::c_uchar,
                                 length.wrapping_mul(512i32 as libc::c_uint),
                                 (&mut (*hdr).z64.checksum as *mut uint32_t as
                                      *mut libc::c_uint).wrapping_offset_from(hdr
                                                                                  as
                                                                                  *mut libc::c_uchar
                                                                                  as
                                                                                  *mut libc::c_uint)
                                     as libc::c_long as libc::c_uint) ||
               0 == (*mfshnd).is_64 &&
                   0 ==
                       mfs_check_crc(hdr as *mut libc::c_uchar,
                                     length.wrapping_mul(512i32 as
                                                             libc::c_uint),
                                     (&mut (*hdr).z32.checksum as
                                          *mut uint32_t as
                                          *mut libc::c_uint).wrapping_offset_from(hdr
                                                                                      as
                                                                                      *mut libc::c_uchar
                                                                                      as
                                                                                      *mut libc::c_uint)
                                         as libc::c_long as libc::c_uint) {
            (*mfshnd).err_msg =
                b"Zone map checksum error\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char;
            free(hdr);
            return 0 as *mut zone_header
        }
    }
    return hdr;
}
/* **************************/
/* Load the zone map list. */
#[no_mangle]
pub unsafe extern "C" fn mfs_load_zone_maps(mut mfshnd: *mut mfs_handle)
 -> libc::c_int {
    let mut ptrsector: uint64_t = 0;
    let mut ptrsbackup: uint64_t = 0;
    let mut ptrlength: uint32_t = 0;
    let mut cur: *mut zone_header = 0 as *mut zone_header;
    let mut loaded_head: *mut *mut zone_map = &mut (*mfshnd).loaded_zones;
    let mut cur_heads: [*mut *mut zone_map; 3] = [0 as *mut *mut zone_map; 3];
    let mut loop_0: libc::c_int = 0;
    if 0 != (*mfshnd).is_64 {
        ptrsector = intswap64((*mfshnd).vol_hdr.v64.zonemap.sector);
        ptrsbackup = intswap64((*mfshnd).vol_hdr.v64.zonemap.sbackup);
        ptrlength =
            intswap64((*mfshnd).vol_hdr.v64.zonemap.length) as uint32_t
    } else {
        ptrsector =
            intswap32((*mfshnd).vol_hdr.v32.zonemap.sector) as uint64_t;
        ptrsbackup =
            intswap32((*mfshnd).vol_hdr.v32.zonemap.sbackup) as uint64_t;
        ptrlength = intswap32((*mfshnd).vol_hdr.v32.zonemap.length)
    }
    /* Start clean. */
    mfs_cleanup_zone_maps(mfshnd);
    memset((*mfshnd).zones.as_mut_ptr() as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<[zone_map_head; 3]>() as libc::c_ulong);
    loop_0 = 0i32;
    while loop_0 < ztMax as libc::c_int {
        cur_heads[loop_0 as usize] =
            &mut (*(*mfshnd).zones.as_mut_ptr().offset(loop_0 as isize)).next;
        loop_0 += 1
    }
    loop_0 = 0i32;
    while 0 != ptrsector && ptrsbackup != 0xdeadbeefu32 as libc::c_ulonglong
              && 0 != ptrlength {
        let mut newmap: *mut zone_map = 0 as *mut zone_map;
        let mut bitmap_ptrs: *mut uint32_t = 0 as *mut uint32_t;
        let mut loop2: libc::c_int = 0;
        let mut type_0: libc::c_int = 0;
        let mut numbitmaps: libc::c_int = 0;
        /* Read the map, verify it's checksum. */
        cur = mfs_load_zone_map(mfshnd, ptrsector, ptrsbackup, ptrlength);
        if cur.is_null() { return -1i32 }
        if 0 != (*mfshnd).is_64 {
            type_0 = intswap32((*cur).z64.type_0 as uint32_t) as libc::c_int;
            numbitmaps = intswap32((*cur).z64.num) as libc::c_int
        } else {
            type_0 = intswap32((*cur).z32.type_0 as uint32_t) as libc::c_int;
            numbitmaps = intswap32((*cur).z32.num) as libc::c_int
        }
        if type_0 < 0i32 || type_0 >= ztMax as libc::c_int {
            (*mfshnd).err_msg =
                b"Bad map type %d\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char;
            (*mfshnd).err_arg1 = type_0 as int64_t;
            free(cur);
            return -1i32
        }
        newmap =
            calloc(::std::mem::size_of::<zone_map>() as libc::c_ulong,
                   1i32 as libc::c_ulong) as *mut zone_map;
        if newmap.is_null() {
            (*mfshnd).err_msg =
                b"Out of memory\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char;
            free(cur);
            return -1i32
        }
        if 0 != numbitmaps {
            (*newmap).bitmaps =
                calloc(::std::mem::size_of::<*mut bitmap_header>() as
                           libc::c_ulong, numbitmaps as libc::c_ulong) as
                    *mut *mut bitmap_header;
            if (*newmap).bitmaps.is_null() {
                (*mfshnd).err_msg =
                    b"Out of memory\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                free(newmap);
                free(cur);
                return -1i32
            }
        } else { (*newmap).bitmaps = 0 as *mut *mut bitmap_header }
        /* Link it into the proper map type pool. */
        (*newmap).map = cur;
        *cur_heads[type_0 as usize] = newmap;
        cur_heads[type_0 as usize] = &mut (*newmap).next;
        /* Get pointers to the bitmaps for easy access */
        if numbitmaps != 0i32 {
            if 0 != (*mfshnd).is_64 {
                bitmap_ptrs =
                    (&mut (*cur).z64 as *mut zone_header_64).offset(1isize) as
                        *mut uint32_t
            } else {
                bitmap_ptrs =
                    (&mut (*cur).z32 as *mut zone_header_32).offset(1isize) as
                        *mut uint32_t
            }
            let ref mut fresh3 = *(*newmap).bitmaps.offset(0isize);
            *fresh3 =
                &mut *bitmap_ptrs.offset(numbitmaps as isize) as *mut uint32_t
                    as *mut bitmap_header;
            loop2 = 1i32;
            while loop2 < numbitmaps {
                let ref mut fresh4 =
                    *(*newmap).bitmaps.offset(loop2 as isize);
                *fresh4 =
                    (*(*newmap).bitmaps.offset(0isize) as
                         size_t).wrapping_add(intswap32(*bitmap_ptrs.offset(loop2
                                                                                as
                                                                                isize)).wrapping_sub(intswap32(*bitmap_ptrs.offset(0isize)))
                                                  as libc::c_ulong) as
                        *mut bitmap_header;
                loop2 += 1
            }
            /* Allocate head pointers for changes for each level of the map */
            (*newmap).changed_runs =
                calloc(::std::mem::size_of::<*mut zone_changed_run>() as
                           libc::c_ulong, numbitmaps as libc::c_ulong) as
                    *mut *mut zone_changed_run;
            (*newmap).changes =
                calloc(::std::mem::size_of::<zone_changes>() as libc::c_ulong,
                       numbitmaps as libc::c_ulong) as *mut zone_changes
        }
        /* Also link it into the loaded order. */
        *loaded_head = newmap;
        loaded_head = &mut (*newmap).next_loaded;
        /* And add it to the totals. */
        if 0 != (*mfshnd).is_64 {
            (*mfshnd).zones[type_0 as usize].size =
                ((*mfshnd).zones[type_0 as usize].size as
                     libc::c_ulonglong).wrapping_add(intswap64((*cur).z64.size))
                    as uint64_t as uint64_t;
            (*mfshnd).zones[type_0 as usize].free =
                ((*mfshnd).zones[type_0 as usize].free as
                     libc::c_ulonglong).wrapping_add(intswap64((*cur).z64.free))
                    as uint64_t as uint64_t;
            ptrsector = intswap64((*cur).z64.next_sector);
            ptrsbackup = intswap64((*cur).z64.next_sbackup);
            ptrlength = intswap32((*cur).z64.next_length)
        } else {
            (*mfshnd).zones[type_0 as usize].size =
                ((*mfshnd).zones[type_0 as usize].size as
                     libc::c_ulonglong).wrapping_add(intswap32((*cur).z32.size)
                                                         as libc::c_ulonglong)
                    as uint64_t as uint64_t;
            (*mfshnd).zones[type_0 as usize].free =
                ((*mfshnd).zones[type_0 as usize].free as
                     libc::c_ulonglong).wrapping_add(intswap32((*cur).z32.free)
                                                         as libc::c_ulonglong)
                    as uint64_t as uint64_t;
            ptrsector = intswap32((*cur).z32.next.sector) as uint64_t;
            ptrsbackup = intswap32((*cur).z32.next.sbackup) as uint64_t;
            ptrlength = intswap32((*cur).z32.next.length)
        }
        loop_0 += 1
    }
    return loop_0;
}
/* ************************************************************/
/* Find a free run of a certain size within a specific szone */
unsafe extern "C" fn mfs_zone_find_run(mut mfshnd: *mut mfs_handle,
                                       mut zone: *mut zone_map,
                                       mut order: libc::c_int)
 -> libc::c_int {
    let mut curorder: libc::c_int = 0;
    let mut numbitmaps: libc::c_int = 0;
    let mut freebit: libc::c_int = -1i32;
    let mut changed_runs: *mut *mut zone_changed_run =
        0 as *mut *mut zone_changed_run;
    if 0 != (*mfshnd).is_64 {
        numbitmaps = intswap32((*(*zone).map).z64.num) as libc::c_int
    } else { numbitmaps = intswap32((*(*zone).map).z32.num) as libc::c_int }
    curorder = order;
    while curorder < numbitmaps {
        let mut numfree: libc::c_int =
            intswap32((**(*zone).bitmaps.offset(curorder as
                                                    isize)).freeblocks).wrapping_add((*(*zone).changes.offset(curorder
                                                                                                                  as
                                                                                                                  isize)).freed
                                                                                         as
                                                                                         libc::c_uint).wrapping_sub((*(*zone).changes.offset(curorder
                                                                                                                                                 as
                                                                                                                                                 isize)).allocated
                                                                                                                        as
                                                                                                                        libc::c_uint)
                as libc::c_int;
        if numfree > 0i32 { break ; }
        curorder += 1
    }
    if curorder >= numbitmaps { return -1i32 }
    /* Find the free bit in the bitmap */
    changed_runs =
        &mut *(*zone).changed_runs.offset(curorder as isize) as
            *mut *mut zone_changed_run;
    while !(*changed_runs).is_null() {
        if 0 != (**changed_runs).newstate {
            let mut tmp: *mut zone_changed_run = *changed_runs;
            *changed_runs = (*tmp).next;
            freebit = (*tmp).bitno;
            free(tmp);
            break ;
        } else { changed_runs = &mut (**changed_runs).next }
    }
    /* Didn't find something in the list, find it in the bitmap */
    if freebit < 0i32 {
        let mut nints: libc::c_int =
            intswap32((**(*zone).bitmaps.offset(curorder as isize)).nints) as
                libc::c_int;
        let mut startint: libc::c_int =
            intswap32((**(*zone).bitmaps.offset(curorder as
                                                    isize)).last).wrapping_div(32i32
                                                                                   as
                                                                                   libc::c_uint).wrapping_rem(nints
                                                                                                                  as
                                                                                                                  libc::c_uint)
                as libc::c_int;
        let mut loop_0: libc::c_int = 0;
        let mut bits: *mut libc::c_uint =
            (*(*zone).bitmaps.offset(curorder as isize)).offset(1isize) as
                *mut libc::c_uint;
        loop_0 = 0i32;
        while loop_0 < nints {
            let mut curint: libc::c_uint =
                *bits.offset(((loop_0 + startint) % nints) as isize);
            if 0 != curint {
                curint = intswap32(curint);
                let mut loop2: libc::c_int = 0;
                let mut thisbit: libc::c_int = -1i32;
                loop2 = 0i32;
                while 0 != curint && loop2 < 32i32 {
                    if 0 != curint & (1i32 << 31i32 - loop2) as libc::c_uint {
                        thisbit = (loop_0 + startint) % nints * 32i32 + loop2;
                        /* Make sure the bit wasn't already allocated */
                        let mut crloop: *mut zone_changed_run =
                            0 as *mut zone_changed_run;
                        crloop =
                            *(*zone).changed_runs.offset(curorder as isize);
                        while !crloop.is_null() {
                            if (*crloop).bitno == thisbit { break ; }
                            crloop = (*crloop).next
                        }
                        if crloop.is_null() { break ; }
                        thisbit = -1i32
                    }
                    loop2 += 1
                }
                if thisbit >= 0i32 { freebit = thisbit }
            }
            loop_0 += 1
        }
        if freebit < 0i32 {
            /* Something is wrong */
            return -1i32
        }
        /* Add the allocation to the list */
		/* Due to the loop earlier, this points to the tail of the list */
        *changed_runs =
            calloc(::std::mem::size_of::<*mut zone_changed_run>() as
                       libc::c_ulong, 1i32 as libc::c_ulong) as
                *mut zone_changed_run;
        (**changed_runs).bitno = freebit
    }
    let ref mut fresh5 =
        (*(*zone).changes.offset(curorder as isize)).allocated;
    *fresh5 += 1;
    while curorder > order {
        let mut newchange: *mut zone_changed_run = 0 as *mut zone_changed_run;
        freebit <<= 1i32;
        curorder -= 1;
        /* Create a notation that there is now a free block for the */
		/* "other half" of the allocation */
        newchange =
            calloc(::std::mem::size_of::<zone_changed_run>() as libc::c_ulong,
                   1i32 as libc::c_ulong) as *mut zone_changed_run;
        (*newchange).next = *(*zone).changed_runs.offset(curorder as isize);
        let ref mut fresh6 = *(*zone).changed_runs.offset(curorder as isize);
        *fresh6 = newchange;
        (*newchange).newstate = 1i32;
        (*newchange).bitno = freebit + 1i32;
        let ref mut fresh7 =
            (*(*zone).changes.offset(curorder as isize)).freed;
        *fresh7 += 1
    }
    return freebit;
}
/* Simplified "greedy" allocation scheme */
/* Works well on a fresh MFS, not so well on a well used volume */
/* ******************************/
/* Perform a greedy allocation */
/* This is not an ideal solution, but the ideal solution is not NP complete */
/* (See knapsack problem) */
/* This allocates blocks for the file starting at the largest and going */
/* down to the smallest.  This works great for a fresh volume, but it */
/* can break down when there has been some churn, leaving lots of */
/* small runs unallocated until late. */
/* This can be a problem if the free space is so fragmented that it needs */
/* more runs than can be allocated to describe a file */
#[no_mangle]
pub unsafe extern "C" fn mfs_alloc_greedy(mut mfshnd: *mut mfs_handle,
                                          mut inode: *mut mfs_inode,
                                          mut highest: uint64_t)
 -> libc::c_int {
    let mut alloctype: zone_type = ztApplication;
    let mut size: uint64_t = intswap32((*inode).size) as uint64_t;
    let mut runsizes: *mut uint64_t = 0 as *mut uint64_t;
    let mut freeblocks: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut curorders: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nbitmaps: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut zones: *mut *mut zone_map = 0 as *mut *mut zone_map;
    let mut nzones: libc::c_int = 0;
    let mut zone: *mut zone_map = 0 as *mut zone_map;
    let mut maxruns: libc::c_int = 0;
    let mut currun: libc::c_int = 0i32;
    let mut lastrunsize: uint64_t = 0xffffffffu32 as uint64_t;
    if 0 != (*mfshnd).is_64 {
        maxruns =
            (512i32 as
                 libc::c_ulong).wrapping_sub(60u64).wrapping_div(::std::mem::size_of::<C2RustUnnamed_4>()
                                                                     as
                                                                     libc::c_ulong)
                as libc::c_int
    } else {
        maxruns =
            (512i32 as
                 libc::c_ulong).wrapping_sub(60u64).wrapping_div(::std::mem::size_of::<C2RustUnnamed_5>()
                                                                     as
                                                                     libc::c_ulong)
                as libc::c_int
    }
    if (*inode).type_0 as libc::c_int == tyStream as libc::c_int {
        alloctype = ztMedia;
        size =
            (size as
                 libc::c_ulonglong).wrapping_mul(intswap32((*inode).blocksize)
                                                     as libc::c_ulonglong) as
                uint64_t as uint64_t
    }
    /* Convert bytes to blocks */
    size =
        size.wrapping_add(511i32 as
                              libc::c_ulonglong).wrapping_div(512i32 as
                                                                  libc::c_ulonglong);
    (*inode).numblocks = 0i32 as libc::c_uint;
    /* Make it really high if it wasn't specified */
    if 0 == highest { highest = !0i64 as uint64_t }
    /* Count the number of loaded maps */
    nzones = 0i32;
    zone = (*mfshnd).zones[alloctype as usize].next;
    while !zone.is_null() {
        if 0 != (*mfshnd).is_64 {
            if intswap64((*(*zone).map).z64.last) < highest { nzones += 1 }
        } else if (intswap32((*(*zone).map).z32.last) as libc::c_ulonglong) <
                      highest {
            nzones += 1
        }
        zone = (*zone).next
    }
    /* Quick sanity check */
    if 0 == nzones { return 0i32 }
    /* Allocate temp arrays (Off the stack) */
    freeblocks =
        alloca((nzones as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong)) as
            *mut libc::c_int;
    curorders =
        alloca((nzones as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong)) as
            *mut libc::c_int;
    runsizes =
        alloca((nzones as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<uint64_t>()
                                                    as libc::c_ulong)) as
            *mut uint64_t;
    zones =
        alloca((nzones as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut zone_map>()
                                                    as libc::c_ulong)) as
            *mut *mut zone_map;
    nbitmaps =
        alloca((nzones as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                    as libc::c_ulong)) as
            *mut libc::c_int;
    /* Fill in the data for the zones */
    nzones = 0i32;
    zone = (*mfshnd).zones[alloctype as usize].next;
    while !zone.is_null() {
        if 0 != (*mfshnd).is_64 {
            if intswap64((*(*zone).map).z64.last) < highest {
                let mut curorder: libc::c_int =
                    intswap32((*(*zone).map).z64.num).wrapping_sub(1i32 as
                                                                       libc::c_uint)
                        as libc::c_int;
                let ref mut fresh8 = *zones.offset(nzones as isize);
                *fresh8 = zone;
                *runsizes.offset(nzones as isize) =
                    intswap32((*(*zone).map).z64.min) as uint64_t;
                *curorders.offset(nzones as isize) = curorder;
                *freeblocks.offset(nzones as isize) =
                    intswap32((**(*zone).bitmaps.offset(curorder as
                                                            isize)).freeblocks).wrapping_add((*(*zone).changes.offset(curorder
                                                                                                                          as
                                                                                                                          isize)).freed
                                                                                                 as
                                                                                                 libc::c_uint).wrapping_sub((*(*zone).changes.offset(curorder
                                                                                                                                                         as
                                                                                                                                                         isize)).allocated
                                                                                                                                as
                                                                                                                                libc::c_uint)
                        as libc::c_int;
                *nbitmaps.offset(nzones as isize) = curorder + 1i32;
                nzones += 1
            }
        } else if (intswap32((*(*zone).map).z32.last) as libc::c_ulonglong) <
                      highest {
            let mut curorder_0: libc::c_int =
                intswap32((*(*zone).map).z32.num).wrapping_sub(1i32 as
                                                                   libc::c_uint)
                    as libc::c_int;
            let ref mut fresh9 = *zones.offset(nzones as isize);
            *fresh9 = zone;
            *runsizes.offset(nzones as isize) =
                intswap32((*(*zone).map).z32.min) as uint64_t;
            *curorders.offset(nzones as isize) = curorder_0;
            *freeblocks.offset(nzones as isize) =
                intswap32((**(*zone).bitmaps.offset(curorder_0 as
                                                        isize)).freeblocks).wrapping_add((*(*zone).changes.offset(curorder_0
                                                                                                                      as
                                                                                                                      isize)).freed
                                                                                             as
                                                                                             libc::c_uint).wrapping_sub((*(*zone).changes.offset(curorder_0
                                                                                                                                                     as
                                                                                                                                                     isize)).allocated
                                                                                                                            as
                                                                                                                            libc::c_uint)
                    as libc::c_int;
            *nbitmaps.offset(nzones as isize) = curorder_0 + 1i32;
            nzones += 1
        }
        zone = (*zone).next
    }
    /* Keep going until the entire size is taken up */
    while size > 0i32 as libc::c_ulonglong {
        let mut loop_0: libc::c_int = 0;
        let mut largestfit: uint64_t = 0i32 as uint64_t;
        let mut largestfitno: libc::c_int = 0i32;
        /* To use as a tie-breaker */
        let mut largestfitborrow: libc::c_int = 0i32;
        /* To use as a second tiebreaker */
        let mut largestfitfree: libc::c_uint = 0i32 as libc::c_uint;
        if currun >= maxruns {
            /* Out of space within the requested limits */
            return 0i32
        }
        let mut current_block_80: u64;
        /* Find the first zone with available space */
        loop_0 = 0i32;
        while loop_0 < nzones {
            let mut unitsleft: libc::c_uint =
                size.wrapping_add(*runsizes.offset(loop_0 as
                                                       isize)).wrapping_sub(1i32
                                                                                as
                                                                                libc::c_ulonglong).wrapping_div(*runsizes.offset(loop_0
                                                                                                                                     as
                                                                                                                                     isize))
                    as libc::c_uint;
            let mut nborrow: libc::c_int = 0i32;
            let mut loop2: libc::c_int = 0;
            let mut runstoalloc: libc::c_int = 0;
            /* Don't waste space unless it's the last level */
			/* Also make sure it never goes bigger */
            while *curorders.offset(loop_0 as isize) > 0i32 &&
                      (*runsizes.offset(loop_0 as isize) <<
                           *curorders.offset(loop_0 as isize) > lastrunsize ||
                           0 ==
                               unitsleft >> *curorders.offset(loop_0 as isize)
                           || 0 == *freeblocks.offset(loop_0 as isize)) {
                let ref mut fresh10 = *curorders.offset(loop_0 as isize);
                *fresh10 -= 1;
                *freeblocks.offset(loop_0 as isize) <<= 1i32;
                let ref mut fresh11 = *freeblocks.offset(loop_0 as isize);
                *fresh11 =
                    (*fresh11 as
                         libc::c_uint).wrapping_add(intswap32((**(**zones.offset(loop_0
                                                                                     as
                                                                                     isize)).bitmaps.offset(*curorders.offset(loop_0
                                                                                                                                  as
                                                                                                                                  isize)
                                                                                                                as
                                                                                                                isize)).freeblocks).wrapping_add((*(**zones.offset(loop_0
                                                                                                                                                                       as
                                                                                                                                                                       isize)).changes.offset(*curorders.offset(loop_0
                                                                                                                                                                                                                    as
                                                                                                                                                                                                                    isize)
                                                                                                                                                                                                  as
                                                                                                                                                                                                  isize)).freed
                                                                                                                                                     as
                                                                                                                                                     libc::c_uint).wrapping_sub((*(**zones.offset(loop_0
                                                                                                                                                                                                      as
                                                                                                                                                                                                      isize)).changes.offset(*curorders.offset(loop_0
                                                                                                                                                                                                                                                   as
                                                                                                                                                                                                                                                   isize)
                                                                                                                                                                                                                                 as
                                                                                                                                                                                                                                 isize)).allocated
                                                                                                                                                                                    as
                                                                                                                                                                                    libc::c_uint))
                        as libc::c_int as libc::c_int
            }
            /* If the current largest is already bigger, keep going */
			/* As an exception, if the largest is bigger than the size */
			/* remaining, keep this block in consideration */
            if !(largestfit >
                     *runsizes.offset(loop_0 as isize) <<
                         *curorders.offset(loop_0 as isize) &&
                     largestfit < size) {
                /* If this is the last level of this zone map, switch from */
			/* biggest fit to smallest */
                if 0 == *curorders.offset(loop_0 as isize) {
                    if 0 != largestfit &&
                           size < *runsizes.offset(loop_0 as isize) &&
                           *runsizes.offset(loop_0 as isize) > largestfit {
                        current_block_80 = 5372832139739605200;
                    } else { current_block_80 = 10778260831612459202; }
                } else { current_block_80 = 10778260831612459202; }
                match current_block_80 {
                    5372832139739605200 => { }
                    _ => {
                        runstoalloc =
                            (unitsleft >> *curorders.offset(loop_0 as isize))
                                as libc::c_int;
                        if runstoalloc > *freeblocks.offset(loop_0 as isize) {
                            runstoalloc = *freeblocks.offset(loop_0 as isize)
                        }
                        /* ??? Shouldn't ever happen */
                        if !(0 == runstoalloc) {
                            /* Count how many blocks would need to be borrowed */
                            loop2 = *curorders.offset(loop_0 as isize);
                            while 0 != runstoalloc &&
                                      loop2 <
                                          *nbitmaps.offset(loop_0 as isize) {
                                let mut thisfree: libc::c_uint =
                                    intswap32((**(**zones.offset(loop_0 as
                                                                     isize)).bitmaps.offset(loop2
                                                                                                as
                                                                                                isize)).freeblocks).wrapping_add((*(**zones.offset(loop_0
                                                                                                                                                       as
                                                                                                                                                       isize)).changes.offset(loop2
                                                                                                                                                                                  as
                                                                                                                                                                                  isize)).freed
                                                                                                                                     as
                                                                                                                                     libc::c_uint).wrapping_sub((*(**zones.offset(loop_0
                                                                                                                                                                                      as
                                                                                                                                                                                      isize)).changes.offset(loop2
                                                                                                                                                                                                                 as
                                                                                                                                                                                                                 isize)).allocated
                                                                                                                                                                    as
                                                                                                                                                                    libc::c_uint);
                                thisfree =
                                    thisfree <<
                                        loop2 -
                                            *curorders.offset(loop_0 as
                                                                  isize);
                                if thisfree > runstoalloc as libc::c_uint {
                                    thisfree = runstoalloc as libc::c_uint
                                }
                                /* Theoretically the algorithm will never borrow more */
				/* than one from anything but the current level */
                                nborrow =
                                    (nborrow as
                                         libc::c_uint).wrapping_add(thisfree.wrapping_mul((loop2
                                                                                               -
                                                                                               *curorders.offset(loop_0
                                                                                                                     as
                                                                                                                     isize))
                                                                                              as
                                                                                              libc::c_uint))
                                        as libc::c_int as libc::c_int;
                                runstoalloc =
                                    (runstoalloc as
                                         libc::c_uint).wrapping_sub(thisfree)
                                        as libc::c_int as libc::c_int;
                                loop2 += 1
                            }
                            /* If they are equal, go to the tiebreaker */
                            if largestfit ==
                                   *runsizes.offset(loop_0 as isize) <<
                                       *curorders.offset(loop_0 as isize) {
                                /* First tiebreaker, whoever is borrowing the least */
                                if nborrow > largestfitborrow {
                                    current_block_80 = 5372832139739605200;
                                } else if (*freeblocks.offset(loop_0 as isize)
                                               as libc::c_uint) <
                                              largestfitfree {
                                    current_block_80 = 5372832139739605200;
                                } else {
                                    current_block_80 = 18137396335907573669;
                                }
                            } else {
                                current_block_80 = 18137396335907573669;
                            }
                            match current_block_80 {
                                5372832139739605200 => { }
                                _ => {
                                    largestfit =
                                        *runsizes.offset(loop_0 as isize) <<
                                            *curorders.offset(loop_0 as
                                                                  isize);
                                    largestfitfree =
                                        *freeblocks.offset(loop_0 as isize) as
                                            libc::c_uint;
                                    largestfitborrow = nborrow;
                                    largestfitno = loop_0
                                }
                            }
                        }
                    }
                }
            }
            loop_0 += 1
        }
        if 0 == largestfit {
            /* Out of space within the requested limits */
            return 0i32
        }
        let mut bitno: libc::c_int =
            mfs_zone_find_run(mfshnd, *zones.offset(largestfitno as isize),
                              *curorders.offset(largestfitno as isize));
        if bitno < 0i32 {
            /* Shouldn't happen, but just in case */
            return 0i32
        }
        if 0 != (*mfshnd).is_64 {
            let mut sector: uint64_t =
                intswap64((*(**zones.offset(largestfitno as
                                                isize)).map).z64.first);
            sector =
                (sector as
                     libc::c_ulonglong).wrapping_add((bitno as
                                                          libc::c_ulonglong).wrapping_mul(*runsizes.offset(largestfitno
                                                                                                               as
                                                                                                               isize))
                                                         <<
                                                         *curorders.offset(largestfitno
                                                                               as
                                                                               isize))
                    as uint64_t as uint64_t;
            (*inode).datablocks.d64[currun as usize].sector =
                sectorswap64(sector);
            (*inode).datablocks.d64[currun as usize].count =
                intswap32(largestfit as uint32_t)
        } else {
            let mut sector_0: libc::c_uint =
                intswap32((*(**zones.offset(largestfitno as
                                                isize)).map).z32.first);
            sector_0 =
                (sector_0 as
                     libc::c_ulonglong).wrapping_add((bitno as
                                                          libc::c_ulonglong).wrapping_mul(*runsizes.offset(largestfitno
                                                                                                               as
                                                                                                               isize))
                                                         <<
                                                         *curorders.offset(largestfitno
                                                                               as
                                                                               isize))
                    as libc::c_uint as libc::c_uint;
            (*inode).datablocks.d32[currun as usize].sector =
                intswap32(sector_0);
            (*inode).datablocks.d32[currun as usize].count =
                intswap32(largestfit as uint32_t)
        }
        currun += 1;
        let ref mut fresh12 = *freeblocks.offset(largestfitno as isize);
        *fresh12 -= 1;
        if size > largestfit {
            size =
                (size as libc::c_ulonglong).wrapping_sub(largestfit) as
                    uint64_t as uint64_t
        } else { size = 0i32 as uint64_t }
        lastrunsize = largestfit
    }
    (*inode).numblocks = intswap32(currun as uint32_t);
    return currun;
}