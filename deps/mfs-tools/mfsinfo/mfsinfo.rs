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
    pub bootcycle: libc::c_int,
    pub bootsecs: libc::c_int,
    pub lastlogsync: libc::c_int,
    pub lastlogcommit: libc::c_int,
    pub err_msg: *mut libc::c_char,
    pub err_arg1: libc::c_int,
    pub err_arg2: libc::c_int,
    pub err_arg3: libc::c_int,
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
#[no_mangle]
pub unsafe extern "C" fn mfsinfo_usage(mut progname: *mut libc::c_char) { }
#[no_mangle]
pub unsafe extern "C" fn partition_info(mut mfs: *mut mfs_handle,
                                        mut drives: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut count: libc::c_int = 0i32;
    let mut loop_0: libc::c_int = 0;
    let mut names: [*mut libc::c_char; 12] = [0 as *mut libc::c_char; 12];
    let mut namelens: [libc::c_int; 12] = [0; 12];
    let mut list: *mut libc::c_char = mfs_partition_list(mfs);
    while 0 != *list as libc::c_int && count < 12i32 {
        while 0 != *list as libc::c_int && 0 != isspace(*list as libc::c_int)
              {
            list = list.offset(1isize)
        }
        if 0 == *list { break ; }
        names[count as usize] = list;
        count += 1
    }
    loop_0 = 0i32;
    while loop_0 < count {
        let mut d: libc::c_int = 0;
        let mut p: libc::c_int = 0;
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