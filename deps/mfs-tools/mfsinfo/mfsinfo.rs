#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
#![feature(extern_types)]
extern crate libc;
extern "C" {
    pub type log_hdr_s;
    pub type tivo_partition_file;
    #[no_mangle]
    fn mfs_has_error(mfshnd: *mut mfs_handle) -> libc::c_int;
    #[no_mangle]
    fn mfs_perror(mfshnd: *mut mfs_handle, str: *mut libc::c_char);
    #[no_mangle]
    fn mfs_partition_list(mfshnd: *mut mfs_handle) -> *mut libc::c_char;
    #[no_mangle]
    fn mfsvol_volume_size(hnd: *mut volume_handle, sector: uint64_t)
     -> uint64_t;
    #[no_mangle]
    fn isspace(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn isdigit(_: libc::c_int) -> libc::c_int;
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
#[no_mangle]
pub unsafe extern "C" fn mfsinfo_usage(mut progname: *mut libc::c_char) { }
#[no_mangle]
pub unsafe extern "C" fn partition_info(mut mfs: *mut mfs_handle,
                                        mut drives: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut count: libc::c_int = 0i32;
    let mut loop_0: libc::c_int = 0;
    let mut offset: uint64_t = 0;
    let mut names: [*mut libc::c_char; 12] = [0 as *mut libc::c_char; 12];
    let mut namelens: [libc::c_int; 12] = [0; 12];
    let mut list: *mut libc::c_char = mfs_partition_list(mfs);
    while 0 != *list as libc::c_int && count < 12i32 {
        offset = 0i32 as uint64_t;
        while 0 != *list as libc::c_int && 0 != isspace(*list as libc::c_int)
              {
            list = list.offset(1isize)
        }
        if 0 == *list { break ; }
        while 0 != *list.offset(offset as isize) as libc::c_int &&
                  0 == isspace(*list.offset(offset as isize) as libc::c_int) {
            offset = offset.wrapping_add(1)
        }
        names[count as usize] = list;
        namelens[count as usize] = offset as libc::c_int;
        count += 1;
        list = list.offset(offset as isize)
    }
    offset = 0i32 as uint64_t;
    loop_0 = 0i32;
    while loop_0 < count {
        let mut d: libc::c_int = 0;
        let mut p: libc::c_int = 0;
        let mut size: uint64_t = 0;
        p = namelens[loop_0 as usize] - 1i32;
        while 0 !=
                  isdigit(*names[loop_0 as usize].offset(p as isize) as
                              libc::c_int) && p > 0i32 {
            p -= 1
        }
        p += 1;
        if 0 != *names[loop_0 as usize].offset(p as isize) {
            d =
                *names[loop_0 as usize].offset((p - 1i32) as isize) as
                    libc::c_int - 'a' as i32;
            p = atoi(names[loop_0 as usize].offset(p as isize))
        }
        size = mfsvol_volume_size((*mfs).vols, offset);
        offset =
            (offset as libc::c_ulonglong).wrapping_add(size) as uint64_t as
                uint64_t;
        loop_0 += 1
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn mfsinfo_main(mut argc: libc::c_int,
                                      mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut opt: libc::c_int = 0;
    let mut ndrives: libc::c_int = 0i32;
    let mut drives: [*mut libc::c_char; 3] = [0 as *mut libc::c_char; 3];
    let mut mfs: *mut mfs_handle = 0 as *mut mfs_handle;
    let mut nparts: libc::c_int = 0i32;
    tivo_partition_direct();
    opt = getopt(argc, argv, b"h\x00" as *const u8 as *const libc::c_char);
    if opt > 0i32 {
        match opt { _ => { } }
        mfsinfo_usage(*argv.offset(0isize));
        return 1i32
    }
    while ndrives + 1i32 < argc && ndrives < 3i32 {
        drives[ndrives as usize] = *argv.offset((ndrives + 1i32) as isize);
        ndrives += 1
    }
    if ndrives == 0i32 || ndrives > 2i32 {
        mfsinfo_usage(*argv.offset(0isize));
        return 1i32
    }
    if mfs.is_null() { return 1i32 }
    if 0 != mfs_has_error(mfs) {
        mfs_perror(mfs, *argv.offset(0isize));
        return 1i32
    }
    0 != (*mfs).is_64;
    nparts = partition_info(mfs, drives.as_mut_ptr());
    let mut loop_0: libc::c_int = 0i32;
    let mut cur: *mut zone_map = 0 as *mut zone_map;
    cur = (*mfs).loaded_zones;
    while !cur.is_null() {
        0 != (*mfs).is_64;
        loop_0 += 1;
        cur = (*cur).next_loaded
    }
    return 0i32;
}