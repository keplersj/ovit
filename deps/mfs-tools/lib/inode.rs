#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
#![feature(extern_types, ptr_wrapping_offset_from)]
extern crate libc;
extern "C" {
    pub type log_hdr_s;
    pub type tivo_partition_file;
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
    fn mfs_perror(mfshnd: *mut mfs_handle, str: *mut libc::c_char);
    #[no_mangle]
    fn mfs_inode_count(mfshnd: *mut mfs_handle) -> uint32_t;
    #[no_mangle]
    fn mfs_inode_to_sector(mfshnd: *mut mfs_handle, inode: uint32_t)
     -> uint64_t;
    #[no_mangle]
    fn mfsvol_read_data(hnd: *mut volume_handle, buf: *mut libc::c_void,
                        sector: uint64_t, count: uint32_t) -> libc::c_int;
    #[no_mangle]
    fn mfsvol_write_data(hnd: *mut volume_handle, buf: *mut libc::c_void,
                         sector: uint64_t, count: uint32_t) -> libc::c_int;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
pub type size_t = libc::c_ulong;
pub type uint16_t = libc::c_ushort;
pub type uint8_t = libc::c_uchar;
pub type fsid_type_e = libc::c_uchar;
pub const tyDb: fsid_type_e = 8;
pub const tyDir: fsid_type_e = 4;
pub const tyStream: fsid_type_e = 2;
pub const tyFile: fsid_type_e = 1;
pub const tyNone: fsid_type_e = 0;
pub type fsid_type = fsid_type_e;
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
    pub datablocks: C2RustUnnamed,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed {
    pub d32: [C2RustUnnamed_1; 0],
    pub d64: [C2RustUnnamed_0; 0],
}
#[derive ( Copy , Clone )]
#[repr(C, packed)]
pub struct C2RustUnnamed_0 {
    pub sector: uint64_t,
    pub count: uint32_t,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub sector: libc::c_uint,
    pub count: libc::c_uint,
}
pub type mfs_inode = mfs_inode_s;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mfs_dirent_s {
    pub fsid: uint32_t,
    pub type_0: fsid_type,
    pub name: *mut libc::c_char,
}
pub type mfs_dirent = mfs_dirent_s;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mfs_obj_header_s {
    pub fill1: uint32_t,
    pub size: uint32_t,
}
pub type mfs_obj_header = mfs_obj_header_s;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mfs_subobj_header_s {
    pub len: uint16_t,
    pub len1: uint16_t,
    pub obj_type: uint16_t,
    pub flags: uint16_t,
    pub fill: [uint16_t; 2],
    pub id: uint32_t,
}
pub type mfs_subobj_header = mfs_subobj_header_s;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mfs_attr_header_s {
    pub attreltype: uint16_t,
    pub len: uint16_t,
}
pub type mfs_attr_header = mfs_attr_header_s;
/* Borrowed from mfs-utils */
/* return a string identifier for a tivo file type */
pub type object_fn
    =
    Option<unsafe extern "C" fn(_: libc::c_int, _: *mut mfs_subobj_header,
                                _: *mut mfs_attr_header, _: *mut libc::c_void)
               -> ()>;
#[inline]
unsafe extern "C" fn Endian16_Swap(mut var: uint16_t) -> uint16_t {
    var =
        ((var as libc::c_int) << 8i32 | var as libc::c_int >> 8i32) as
            uint16_t;
    return var;
}
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
/* If byte order is not set, assume whatever platform it is doesn't have byteorder.h, and is probably x86 based */
// Fix endianness in the MFS
#[inline]
unsafe extern "C" fn intswap16(mut n: uint16_t) -> uint16_t {
    if mfsLSB == 0i32 { return n }
    return Endian16_Swap(n);
}
#[inline]
unsafe extern "C" fn intswap32(mut n: uint32_t) -> uint32_t {
    if mfsLSB == 0i32 { return n }
    return Endian32_Swap(n);
}
#[inline]
unsafe extern "C" fn sectorswap64(mut n: uint64_t) -> uint64_t {
    let mut ret: uint64_t = 0;
    // *NOTE*  Little endian drives (Roamio) have reversed hi an lo 32 bits
    if mfsLSB == 0i32 { ret = n } else { ret = Endian64_Swap(n) }
    if mfsLSB == 1i32 { ret = ret >> 32i32 | ret << 32i32 }
    return ret;
}
/* ********************************************/
/* Read an inode into a pre-allocated buffer */
#[no_mangle]
pub unsafe extern "C" fn mfs_read_inode_to_buf(mut mfshnd: *mut mfs_handle,
                                               mut inode: libc::c_uint,
                                               mut inode_buf: *mut mfs_inode)
 -> libc::c_int {
    let mut sector: uint64_t = 0;
    if inode_buf.is_null() { return -1i32 }
    /* Find the sector number for this inode. */
    sector = mfs_inode_to_sector(mfshnd, inode);
    if sector == 0i32 as libc::c_ulonglong { return -1i32 }
    if mfsvol_read_data((*mfshnd).vols, inode_buf as *mut libc::c_void,
                        sector, 1i32 as uint32_t) != 512i32 {
        return -1i32
    }
    /* If the CRC is good, don't bother reading the next inode. */
    if 0 !=
           mfs_check_crc(inode_buf as *mut libc::c_uchar,
                         512i32 as libc::c_uint,
                         (&mut (*inode_buf).checksum as
                              *mut libc::c_uint).wrapping_offset_from(inode_buf
                                                                          as
                                                                          *mut libc::c_uint)
                             as libc::c_long as libc::c_uint) {
        return 1i32
    }
    /* CRC is bad, try reading the backup on the next sector. */
    if mfsvol_read_data((*mfshnd).vols, inode_buf as *mut libc::c_void,
                        sector.wrapping_add(1i32 as libc::c_ulonglong),
                        1i32 as uint32_t) != 512i32 {
        return -1i32
    }
    if 0 !=
           mfs_check_crc(inode_buf as *mut libc::c_uchar,
                         512i32 as libc::c_uint,
                         (&mut (*inode_buf).checksum as
                              *mut libc::c_uint).wrapping_offset_from(inode_buf
                                                                          as
                                                                          *mut libc::c_uint)
                             as libc::c_long as libc::c_uint) {
        return 1i32
    }
    (*mfshnd).err_msg =
        b"Inode %d corrupt\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    (*mfshnd).err_arg1 = inode as int64_t;
    return -1i32;
}
/* ************************************/
/* Read an inode data and return it. */
#[no_mangle]
pub unsafe extern "C" fn mfs_read_inode(mut mfshnd: *mut mfs_handle,
                                        mut inode: libc::c_uint)
 -> *mut mfs_inode {
    let mut in_0: *mut mfs_inode =
        calloc(512i32 as libc::c_ulong, 1i32 as libc::c_ulong) as
            *mut mfs_inode;
    if mfs_read_inode_to_buf(mfshnd, inode, in_0) <= 0i32 {
        free(in_0);
        return 0 as *mut mfs_inode
    }
    return in_0;
}
/* ******************/
/* Write an inode. */
#[no_mangle]
pub unsafe extern "C" fn mfs_write_inode(mut mfshnd: *mut mfs_handle,
                                         mut inode: *mut mfs_inode)
 -> libc::c_int {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut sector: uint64_t = 0;
    /* Find the sector number for this inode. */
    sector = mfs_inode_to_sector(mfshnd, intswap32((*inode).inode));
    if sector == 0i32 as libc::c_ulonglong { return -1i32 }
    memcpy(buf.as_mut_ptr() as *mut libc::c_void,
           inode as *const libc::c_void, 512i32 as libc::c_ulong);
    /* Do it after to avoid writing to source */
    mfs_update_crc(buf.as_mut_ptr() as *mut libc::c_uchar,
                   512i32 as libc::c_uint,
                   (&mut (*(buf.as_mut_ptr() as *mut mfs_inode)).checksum as
                        *mut libc::c_uint).wrapping_offset_from(buf.as_mut_ptr()
                                                                    as
                                                                    *mut libc::c_uint)
                       as libc::c_long as libc::c_uint);
    memcpy(buf.as_mut_ptr().offset(512isize) as *mut libc::c_void,
           buf.as_mut_ptr() as *const libc::c_void, 512i32 as libc::c_ulong);
    if mfsvol_write_data((*mfshnd).vols,
                         buf.as_mut_ptr() as *mut libc::c_void, sector,
                         2i32 as uint32_t) != 1024i32 {
        return -1i32
    }
    return 0i32;
}
/* *****************************************************************/
/* Read an inode data based on an fsid, scanning ahead as needed. */
#[no_mangle]
pub unsafe extern "C" fn mfs_read_inode_by_fsid(mut mfshnd: *mut mfs_handle,
                                                mut fsid: uint32_t)
 -> *mut mfs_inode {
    let mut inode: libc::c_uint =
        fsid.wrapping_mul(0x106d9i32 as libc::c_uint) &
            mfs_inode_count(mfshnd).wrapping_sub(1i32 as libc::c_uint);
    let mut cur: *mut mfs_inode = 0 as *mut mfs_inode;
    let mut inode_base: libc::c_uint = inode;
    loop  {
        if !cur.is_null() { free(cur); }
        cur = mfs_read_inode(mfshnd, inode);
        if !(!cur.is_null() && intswap32((*cur).fsid) != fsid &&
                 0 != intswap32((*cur).inode_flags) & 0x80000000u32 &&
                 {
                     inode =
                         inode.wrapping_add(1i32 as
                                                libc::c_uint).wrapping_rem(mfs_inode_count(mfshnd));
                     inode != inode_base
                 }) {
            break ;
        }
    }
    /* Repeat until either the fsid matches, the CHAINED flag is unset, or */
/* every inode has been checked, which I hope I will not have to do. */
    /* If cur is NULL or the fsid is correct and in use, then cur contains the */
/* right return. */
    if cur.is_null() ||
           intswap32((*cur).fsid) == fsid &&
               (*cur).refcount != 0i32 as libc::c_uint {
        return cur
    }
    /* This is not the inode you are looking for.  Move along. */
    free(cur);
    return 0 as *mut mfs_inode;
}
/* *****************************************************************/
/* Given a fsid, find an inode for it if one doesn't already exist. */
#[no_mangle]
pub unsafe extern "C" fn mfs_find_inode_for_fsid(mut mfshnd: *mut mfs_handle,
                                                 mut fsid: libc::c_uint)
 -> *mut mfs_inode {
    let mut inode: libc::c_uint =
        fsid.wrapping_mul(0x106d9i32 as libc::c_uint) &
            mfs_inode_count(mfshnd).wrapping_sub(1i32 as libc::c_uint);
    let mut cur: *mut mfs_inode = 0 as *mut mfs_inode;
    let mut inode_base: libc::c_uint = inode;
    let mut first: *mut mfs_inode = 0 as *mut mfs_inode;
    loop  {
        if !cur.is_null() && cur != first { free(cur); }
        cur = mfs_read_inode(mfshnd, inode);
        if !cur.is_null() && first.is_null() && 0 == (*cur).fsid &&
               0 == (*cur).refcount {
            first = cur
        }
        if !(!cur.is_null() && intswap32((*cur).fsid) != fsid &&
                 0 != intswap32((*cur).inode_flags) & 0x80000000u32 &&
                 {
                     inode =
                         inode.wrapping_add(1i32 as
                                                libc::c_uint).wrapping_rem(mfs_inode_count(mfshnd));
                     inode != inode_base
                 }) {
            break ;
        }
    }
    /* Repeat until either the fsid matches, the CHAINED flag is unset, or */
/* every inode has been checked, which I hope I will not have to do. */
    /* If nothing was read, something is wrong */
    if cur.is_null() {
        if !first.is_null() { free(first); }
        return 0 as *mut mfs_inode
    }
    /* If the fsid was found, return the inode */
    if !cur.is_null() && intswap32((*cur).fsid) == fsid {
        if !first.is_null() && first != cur { free(first); }
        return cur
    }
    /* If the fsid wasn't located, but an empty inode was, return that. */
    if !first.is_null() {
        if !cur.is_null() && cur != first { free(cur); }
        /* Make sure the inode number is set */
        (*first).inode = intswap32(inode);
        return first
    }
    /* Keep looking */
    loop  {
        if !cur.is_null() {
            /* Mark this inode chained */
            if 0 == (*cur).inode_flags & intswap32(0x80000000u32) {
                (*cur).inode_flags |= intswap32(0x80000000u32);
                if mfs_write_inode(mfshnd, cur) < 0i32 {
                    free(cur);
                    return 0 as *mut mfs_inode
                }
            }
            free(cur);
        }
        cur = mfs_read_inode(mfshnd, inode);
        if !(!cur.is_null() && (0 != (*cur).fsid || 0 != (*cur).refcount) &&
                 {
                     inode =
                         inode.wrapping_add(1i32 as
                                                libc::c_uint).wrapping_rem(mfs_inode_count(mfshnd));
                     inode != inode_base
                 }) {
            break ;
        }
    }
    /* Repeat until a free inode is found, or */
/* every inode has been checked, which I hope I will not have to do. */
    if cur.is_null() { return 0 as *mut mfs_inode }
    if 0 != (*cur).fsid || 0 != (*cur).refcount {
        free(cur);
        return 0 as *mut mfs_inode
    }
    (*cur).inode = intswap32(inode);
    return cur;
}
/* *************************************/
/* Write a portion of an inodes data. */
#[no_mangle]
pub unsafe extern "C" fn mfs_write_inode_data_part(mut mfshnd:
                                                       *mut mfs_handle,
                                                   mut inode: *mut mfs_inode,
                                                   mut data:
                                                       *mut libc::c_uchar,
                                                   mut start: uint32_t,
                                                   mut count: libc::c_uint)
 -> libc::c_int {
    let mut totwrit: libc::c_int = 0i32;
    /* Parameter sanity check. */
    if data.is_null() || 0 == count || inode.is_null() { return 0i32 }
    /* If it all fits in the inode block... */
    if 0 != (*inode).inode_flags & intswap32(0x40000000i32 as uint32_t) ||
           0 != (*inode).inode_flags & intswap32(0x2i32 as uint32_t) {
        let mut result: libc::c_int = 0;
        if 0 != start { return 0i32 }
        memcpy((inode as *mut libc::c_uchar).offset(0x3ci32 as isize) as
                   *mut libc::c_void, data as *const libc::c_void,
               (512i32 - 0x3ci32) as libc::c_ulong);
        result = mfs_write_inode(mfshnd, inode);
        return if result < 0i32 { result } else { 512i32 }
    } else {
        if 0 != (*inode).numblocks {
            /* If it doesn't fit in the sector find out where it is. */
            let mut loop_0: libc::c_int = 0;
            let mut current_block_35: u64;
            /* Loop through each block in the inode. */
            loop_0 = 0i32;
            while 0 != count &&
                      (loop_0 as libc::c_uint) < intswap32((*inode).numblocks)
                  {
                /* For sanity sake (Mine, not the code's), make these variables. */
                let mut blkstart: uint64_t = 0;
                let mut blkcount: uint32_t = 0;
                let mut result_0: libc::c_int = 0;
                if 0 != (*mfshnd).is_64 {
                    blkstart =
                        sectorswap64((*inode).datablocks.d64[loop_0 as
                                                                 usize].sector);
                    blkcount =
                        intswap32((*inode).datablocks.d64[loop_0 as
                                                              usize].count)
                } else {
                    blkstart =
                        intswap32((*inode).datablocks.d32[loop_0 as
                                                              usize].sector)
                            as uint64_t;
                    blkcount =
                        intswap32((*inode).datablocks.d32[loop_0 as
                                                              usize].count)
                }
                /* If the start offset has not been reached, skip to it. */
                if 0 != start {
                    if blkcount <= start {
                        /* If the start offset is not within this block, decrement the start and keep */
/* going. */
                        start =
                            (start as libc::c_uint).wrapping_sub(blkcount) as
                                uint32_t as uint32_t;
                        current_block_35 = 2968425633554183086;
                    } else {
                        /* The start offset is within this block.  Adjust the block parameters a */
/* little, since this is just local variables. */
                        blkstart =
                            (blkstart as
                                 libc::c_ulonglong).wrapping_add(start as
                                                                     libc::c_ulonglong)
                                as uint64_t as uint64_t;
                        blkcount =
                            (blkcount as libc::c_uint).wrapping_sub(start) as
                                uint32_t as uint32_t;
                        start = 0i32 as uint32_t;
                        current_block_35 = 14648156034262866959;
                    }
                } else { current_block_35 = 14648156034262866959; }
                match current_block_35 {
                    14648156034262866959 => {
                        /* If the entire data is within this block, make this block look like it */
/* is no bigger than the data. */
                        if blkcount > count { blkcount = count }
                        result_0 =
                            mfsvol_write_data((*mfshnd).vols,
                                              data as *mut libc::c_void,
                                              blkstart, blkcount);
                        count = count.wrapping_sub(blkcount);
                        /* Error - propogate it up. */
                        if result_0 < 0i32 { return result_0 }
                        /* Add to the total. */
                        totwrit += result_0;
                        data = data.offset(result_0 as isize);
                        /* If this is it, or if the amount written was truncated, return it. */
                        if result_0 as libc::c_uint !=
                               blkcount.wrapping_mul(512i32 as libc::c_uint)
                               || count == 0i32 as libc::c_uint {
                            return totwrit
                        }
                    }
                    _ => { }
                }
                loop_0 += 1
            }
        }
    }
    /* They must have asked for more data than there was.  Return the total written. */
    return totwrit;
}
/* ************************************/
/* Read a portion of an inodes data. */
#[no_mangle]
pub unsafe extern "C" fn mfs_read_inode_data_part(mut mfshnd: *mut mfs_handle,
                                                  mut inode: *mut mfs_inode,
                                                  mut data:
                                                      *mut libc::c_uchar,
                                                  mut start: uint64_t,
                                                  mut count: libc::c_uint)
 -> libc::c_int {
    let mut totread: libc::c_int = 0i32;
    /* Parameter sanity check. */
    if data.is_null() || 0 == count || inode.is_null() { return 0i32 }
    /* All the data fits in the inode */
    if 0 != (*inode).inode_flags & intswap32(0x40000000i32 as uint32_t) ||
           0 != (*inode).inode_flags & intswap32(0x2i32 as uint32_t) {
        let mut size: uint32_t = intswap32((*inode).size);
        if 0 != start { return 0i32 }
        /* Corrupted inode, but fake it at least */
        if size > (512i32 - 0x3ci32) as libc::c_uint {
            size = (512i32 - 0xc3i32) as uint32_t
        }
        memset(data.offset(size as isize) as *mut libc::c_void, 0i32,
               (512i32 as libc::c_uint).wrapping_sub(size) as libc::c_ulong);
        memcpy(data as *mut libc::c_void,
               (inode as *mut libc::c_uchar).offset(0x3ci32 as isize) as
                   *const libc::c_void, size as libc::c_ulong);
        return 512i32
    } else {
        /* If it doesn't fit in the sector find out where it is. */
        if 0 != (*inode).numblocks {
            let mut loop_0: libc::c_int = 0;
            let mut current_block_37: u64;
            /* Loop through each block in the inode. */
            loop_0 = 0i32;
            while 0 != count &&
                      (loop_0 as libc::c_uint) < intswap32((*inode).numblocks)
                  {
                /* For sanity sake, make these variables. */
                let mut blkstart: uint64_t = 0;
                let mut blkcount: uint64_t = 0;
                let mut result: libc::c_int = 0;
                if 0 != (*mfshnd).is_64 {
                    blkstart =
                        sectorswap64((*inode).datablocks.d64[loop_0 as
                                                                 usize].sector);
                    blkcount =
                        intswap32((*inode).datablocks.d64[loop_0 as
                                                              usize].count) as
                            uint64_t
                } else {
                    blkstart =
                        intswap32((*inode).datablocks.d32[loop_0 as
                                                              usize].sector)
                            as uint64_t;
                    blkcount =
                        intswap32((*inode).datablocks.d32[loop_0 as
                                                              usize].count) as
                            uint64_t
                }
                /* If the start offset has not been reached, skip to it. */
                if 0 != start {
                    if blkcount <= start {
                        /* If the start offset is not within this block, decrement the start and keep */
/* going. */
                        start =
                            (start as
                                 libc::c_ulonglong).wrapping_sub(blkcount) as
                                uint64_t as uint64_t;
                        current_block_37 = 12800627514080957624;
                    } else {
                        /* The start offset is within this block.  Adjust the block parameters a */
/* little, since this is just local variables. */
                        blkstart =
                            (blkstart as
                                 libc::c_ulonglong).wrapping_add(start) as
                                uint64_t as uint64_t;
                        blkcount =
                            (blkcount as
                                 libc::c_ulonglong).wrapping_sub(start) as
                                uint64_t as uint64_t;
                        start = 0i32 as uint64_t;
                        current_block_37 = 7205609094909031804;
                    }
                } else { current_block_37 = 7205609094909031804; }
                match current_block_37 {
                    7205609094909031804 => {
                        /* If the entire data is within this block, make this block look like it */
/* is no bigger than the data. */
                        if blkcount > count as libc::c_ulonglong {
                            blkcount = count as uint64_t
                        }
                        result =
                            mfsvol_read_data((*mfshnd).vols,
                                             data as *mut libc::c_void,
                                             blkstart, blkcount as uint32_t);
                        count = count.wrapping_sub(blkcount as uint32_t);
                        /* Error - propogate it up. */
                        if result < 0i32 { return result }
                        /* Add to the total. */
                        totread += result;
                        data = data.offset(result as isize);
                        /* If this is it, or if the amount read was truncated, return it. */
                        if result as libc::c_ulonglong !=
                               blkcount.wrapping_mul(512i32 as
                                                         libc::c_ulonglong) ||
                               count == 0i32 as libc::c_uint {
                            return totread
                        }
                    }
                    _ => { }
                }
                loop_0 += 1
            }
        }
    }
    /* They must have asked for more data than there was.  Return the total read. */
    return totread;
}
/* *****************************************************************************/
/* Read all the data from an inode, set size to how much was read.  This does */
/* not allow streams, since they are be so big. */
#[no_mangle]
pub unsafe extern "C" fn mfs_read_inode_data(mut mfshnd: *mut mfs_handle,
                                             mut inode: *mut mfs_inode,
                                             mut size: *mut libc::c_int)
 -> *mut libc::c_uchar {
    let mut data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut result: libc::c_int = 0;
    /* If it doesn't make sense to read the data, don't do it.  Since streams are */
/* so large, it doesn't make sense to read the whole thing. */
    if (*inode).type_0 as libc::c_int == tyStream as libc::c_int ||
           inode.is_null() || size.is_null() || 0 == (*inode).size {
        if !size.is_null() { *size = 0i32 }
        return 0 as *mut libc::c_uchar
    }
    *size = intswap32((*inode).size) as libc::c_int;
    data =
        malloc(((*size + 511i32) as libc::c_uint & !511i32 as libc::c_uint) as
                   libc::c_ulong) as *mut libc::c_uchar;
    if data.is_null() { *size = 0i32; return 0 as *mut libc::c_uchar }
    /* This function is just a wrapper for read_inode_data_part, with the last */
/* parameter being implicitly the whole data. */
    result =
        mfs_read_inode_data_part(mfshnd, inode, data, 0i32 as uint64_t,
                                 ((*size + 511i32) / 512i32) as libc::c_uint);
    if result < 0i32 {
        *size = result;
        free(data);
        return 0 as *mut libc::c_uchar
    }
    return data;
}
/* Borrowed from mfs-utils */
/* return a string identifier for a tivo file type */
#[no_mangle]
pub unsafe extern "C" fn mfs_type_string(mut type_0: fsid_type)
 -> *mut libc::c_char {
    match type_0 as libc::c_int {
        1 => {
            return b"tyFile\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        2 => {
            return b"tyStream\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        4 => {
            return b"tyDir\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        8 => {
            return b"tyDb\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        _ => {
            return b"ty???\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
    };
}
/* free a dir from mfs_dir */
#[no_mangle]
pub unsafe extern "C" fn mfs_dir_free(mut dir: *mut mfs_dirent) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while !(*dir.offset(i as isize)).name.is_null() {
        free((*dir.offset(i as isize)).name);
        let ref mut fresh0 = (*dir.offset(i as isize)).name;
        *fresh0 = 0 as *mut libc::c_char;
        i += 1
    }
    free(dir);
}
/* list a mfs directory - make sure you free with mfs_dir_free() */
#[no_mangle]
pub unsafe extern "C" fn mfs_dir(mut mfshnd: *mut mfs_handle,
                                 mut fsid: libc::c_int,
                                 mut count: *mut uint32_t)
 -> *mut mfs_dirent {
    let mut buf: *mut uint32_t = 0 as *mut uint32_t;
    let mut p: *mut uint32_t = 0 as *mut uint32_t;
    let mut n: libc::c_int = 0i32;
    let mut i: libc::c_int = 0;
    let mut dsize: libc::c_int = 0;
    let mut dflags: libc::c_int = 0;
    let mut ret: *mut mfs_dirent = 0 as *mut mfs_dirent;
    let mut u16buf: *mut uint16_t = 0 as *mut uint16_t;
    let mut inode: *mut mfs_inode = 0 as *mut mfs_inode;
    let mut size: libc::c_int = 0i32;
    *count = 0i32 as uint32_t;
    inode = mfs_read_inode_by_fsid(mfshnd, fsid as uint32_t);
    if !inode.is_null() {
        buf = mfs_read_inode_data(mfshnd, inode, &mut size) as *mut uint32_t
    }
    if size < 4i32 { return 0 as *mut mfs_dirent }
    if (*inode).type_0 as libc::c_int != tyDir as libc::c_int {
        (*mfshnd).err_msg =
            b"fsid %d is not a tyDir\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        (*mfshnd).err_arg1 = fsid as int64_t;
        mfs_perror(mfshnd,
                   b"mfs_dir\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char);
        return 0 as *mut mfs_dirent
    }
    u16buf = buf as *mut uint16_t;
    dsize = intswap16(*u16buf.offset(0isize)) as libc::c_int;
    dflags = intswap16(*u16buf.offset(1isize)) as libc::c_int;
    p = buf.offset(1isize);
    while (p.wrapping_offset_from(buf) as libc::c_long as libc::c_int) <
              dsize / 4i32 {
        let mut s: *mut uint8_t = (p as *mut libc::c_uchar).offset(4isize);
        p = p.offset((*s.offset(0isize) as libc::c_int / 4i32) as isize);
        n += 1
    }
    ret =
        malloc(((n + 1i32) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<mfs_dirent>()
                                                    as libc::c_ulong)) as
            *mut mfs_dirent;
    p = buf.offset(1isize);
    i = 0i32;
    while i < n {
        let mut s_0: *mut uint8_t = (p as *mut libc::c_uchar).offset(4isize);
        let ref mut fresh1 = (*ret.offset(i as isize)).name;
        *fresh1 = strdup((s_0 as *mut libc::c_char).offset(2isize));
        (*ret.offset(i as isize)).type_0 = *s_0.offset(1isize) as fsid_type;
        (*ret.offset(i as isize)).fsid = intswap32(*p.offset(0isize));
        p = p.offset((*s_0.offset(0isize) as libc::c_int / 4i32) as isize);
        i += 1
    }
    let ref mut fresh2 = (*ret.offset(n as isize)).name;
    *fresh2 = 0 as *mut libc::c_char;
    free(buf);
    *count = n as uint32_t;
    /* handle meta-directories. These are just directories which are
	   lists of other directories. All we need to do is recursively read
	   the other directories and piece together the top level directory */
    if dflags == 0x200i32 {
        let mut meta_dir: *mut mfs_dirent = 0 as *mut mfs_dirent;
        let mut meta_size: libc::c_int = 0i32;
        *count = 0i32 as uint32_t;
        i = 0i32;
        while i < n {
            let mut d2: *mut mfs_dirent = 0 as *mut mfs_dirent;
            let mut n2: libc::c_uint = 0;
            if (*ret.offset(i as isize)).type_0 as libc::c_int !=
                   tyDir as libc::c_int {
                (*mfshnd).err_msg =
                    b"ERROR: non dir %d/%s in meta-dir %d!\x00" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                (*mfshnd).err_arg1 =
                    (*ret.offset(i as isize)).type_0 as uint32_t as int64_t;
                (*mfshnd).err_arg2 =
                    (*ret.offset(i as isize)).name as size_t as int64_t;
                (*mfshnd).err_arg3 = fsid as int64_t;
                mfs_perror(mfshnd,
                           b"mfs_dir\x00" as *const u8 as *const libc::c_char
                               as *mut libc::c_char);
            } else {
                d2 =
                    mfs_dir(mfshnd,
                            (*ret.offset(i as isize)).fsid as libc::c_int,
                            &mut n2);
                if !(d2.is_null() || n2 == 0i32 as libc::c_uint) {
                    meta_dir =
                        realloc(meta_dir as *mut libc::c_void,
                                (::std::mem::size_of::<mfs_dirent>() as
                                     libc::c_ulong).wrapping_mul((meta_size as
                                                                      libc::c_uint).wrapping_add(n2).wrapping_add(1i32
                                                                                                                      as
                                                                                                                      libc::c_uint)
                                                                     as
                                                                     libc::c_ulong))
                            as *mut mfs_dirent;
                    memcpy(meta_dir.offset(meta_size as isize) as
                               *mut libc::c_void, d2 as *const libc::c_void,
                           (n2 as
                                libc::c_ulong).wrapping_mul(::std::mem::size_of::<mfs_dirent>()
                                                                as
                                                                libc::c_ulong));
                    meta_size =
                        (meta_size as libc::c_uint).wrapping_add(n2) as
                            libc::c_int as libc::c_int;
                    free(d2);
                }
            }
            i += 1
        }
        mfs_dir_free(ret);
        if !meta_dir.is_null() {
            let ref mut fresh3 = (*meta_dir.offset(meta_size as isize)).name;
            *fresh3 = 0 as *mut libc::c_char
        }
        *count = meta_size as uint32_t;
        return meta_dir
    }
    return ret;
}
/* resolve a path to a fsid */
#[no_mangle]
pub unsafe extern "C" fn mfs_resolve(mut mfshnd: *mut mfs_handle,
                                     mut pathin: *const libc::c_char)
 -> uint32_t {
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tok: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fsid: uint32_t = 0;
    let mut dir: *mut mfs_dirent = 0 as *mut mfs_dirent;
    if *pathin.offset(0isize) as libc::c_int != '/' as i32 {
        return atoi(pathin) as uint32_t
    }
    fsid = 1i32 as uint32_t;
    path = strdup(pathin);
    tok =
        strtok_r(path, b"/\x00" as *const u8 as *const libc::c_char, &mut r)
            as *mut libc::c_char;
    while !tok.is_null() {
        let mut count: uint32_t = 0;
        let mut i: libc::c_int = 0;
        dir = mfs_dir(mfshnd, fsid as libc::c_int, &mut count);
        if dir.is_null() {
            (*mfshnd).err_msg =
                b"resolve failed for fsid=%d\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char;
            (*mfshnd).err_arg1 = fsid as libc::c_int as int64_t;
            mfs_perror(mfshnd,
                       b"mfs_resolve\x00" as *const u8 as *const libc::c_char
                           as *mut libc::c_char);
            return 0i32 as uint32_t
        }
        i = 0i32;
        while (i as libc::c_uint) < count {
            if strcmp(tok, (*dir.offset(i as isize)).name) == 0i32 { break ; }
            i += 1
        }
        if i as libc::c_uint == count {
            fsid = 0i32 as uint32_t;
            break ;
        } else {
            fsid = (*dir.offset(i as isize)).fsid;
            if (*dir.offset(i as isize)).type_0 as libc::c_int !=
                   tyDir as libc::c_int {
                if !(0 !=
                         strtok_r(0 as *mut libc::c_void,
                                  b"/\x00" as *const u8 as
                                      *const libc::c_char, &mut r)) {
                    break ;
                }
                (*mfshnd).err_msg =
                    b"not a directory %s\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char;
                (*mfshnd).err_arg1 = tok as size_t as int64_t;
                mfs_perror(mfshnd,
                           b"mfs_resolve\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char);
                fsid = 0i32 as uint32_t;
                break ;
            } else {
                mfs_dir_free(dir);
                dir = 0 as *mut mfs_dirent;
                tok =
                    strtok_r(0 as *mut libc::c_void,
                             b"/\x00" as *const u8 as *const libc::c_char,
                             &mut r) as *mut libc::c_char
            }
        }
    }
    if !dir.is_null() { mfs_dir_free(dir); }
    if !path.is_null() { free(path); }
    return fsid;
}
unsafe extern "C" fn parse_attr(mut p: *mut libc::c_char,
                                mut obj_type: libc::c_int,
                                mut fsid: libc::c_int,
                                mut obj: *mut mfs_subobj_header,
                                mut fn_0: object_fn) -> libc::c_int {
    let mut attr: *mut mfs_attr_header = 0 as *mut mfs_attr_header;
    let mut ret: libc::c_int = 0;
    attr = p as *mut mfs_attr_header;
    p =
        p.offset(::std::mem::size_of::<mfs_attr_header>() as libc::c_ulong as
                     isize);
    fn_0.expect("non-null function pointer")(fsid, obj, attr,
                                             p as *mut libc::c_void);
    ret = intswap16((*attr).len) as libc::c_int + 3i32 & !3i32;
    return ret;
}
unsafe extern "C" fn parse_subobj(mut p: *mut libc::c_void,
                                  mut type_0: uint16_t, mut len: libc::c_int,
                                  mut fsid: libc::c_int,
                                  mut obj: *mut mfs_subobj_header,
                                  mut fn_0: object_fn) {
    let mut ofs: libc::c_int = 0i32;
    while ofs < len {
        ofs +=
            parse_attr(p.offset(ofs as isize) as *mut libc::c_char,
                       type_0 as libc::c_int, fsid, obj, fn_0)
    };
}
/* this is the low-level interface to parsing an object. It will call fn() on
   all elements in all subobjects */
#[no_mangle]
pub unsafe extern "C" fn parse_object(mut fsid: libc::c_int,
                                      mut buf: *mut libc::c_void,
                                      mut fn_0: object_fn) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ofs: uint32_t = 0;
    let mut obj: *mut mfs_obj_header = buf as *mut mfs_obj_header;
    let mut i: libc::c_int = 0i32;
    p = buf as *mut libc::c_char;
    ofs =
        ::std::mem::size_of::<mfs_obj_header>() as libc::c_ulong as uint32_t;
    /* now the subobjects */
    while ofs < intswap32((*obj).size) {
        let mut subobj: *mut mfs_subobj_header =
            buf.offset(ofs as isize) as *mut mfs_subobj_header;
        fn_0.expect("non-null function pointer")(fsid, subobj,
                                                 0 as *mut mfs_attr_header,
                                                 0 as *mut libc::c_void);
        parse_subobj(buf.offset(ofs as
                                    isize).offset(::std::mem::size_of::<mfs_subobj_header>()
                                                      as libc::c_ulong as
                                                      isize),
                     intswap16((*subobj).obj_type),
                     (intswap16((*subobj).len) as
                          libc::c_ulong).wrapping_sub(::std::mem::size_of::<mfs_subobj_header>()
                                                          as libc::c_ulong) as
                         libc::c_int, fsid, subobj, fn_0);
        ofs =
            (ofs as
                 libc::c_uint).wrapping_add(intswap16((*subobj).len) as
                                                libc::c_uint) as uint32_t as
                uint32_t;
        i += 1
    };
}