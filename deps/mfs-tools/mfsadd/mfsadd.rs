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
    #[no_mangle]
    fn mfs_has_error(mfshnd: *mut mfs_handle) -> libc::c_int;
    #[no_mangle]
    fn mfs_perror(mfshnd: *mut mfs_handle, str: *mut libc::c_char);
    #[no_mangle]
    fn mfs_partition_list(mfshnd: *mut mfs_handle) -> *mut libc::c_char;
    #[no_mangle]
    fn mfs_sa_hours_estimate(mfshnd: *mut mfs_handle) -> libc::c_uint;
    #[no_mangle]
    fn tivo_partition_count(device: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn tivo_partition_type(device: *const libc::c_char, partnum: libc::c_int)
     -> *mut libc::c_char;
    #[no_mangle]
    fn tivo_partition_direct();
    #[no_mangle]
    fn tivo_partition_swabbed(device: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn tivo_partition_devswabbed(device: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn tivo_partition_table_init(device: *const libc::c_char,
                                 swab: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn tivo_partition_table_write(device: *const libc::c_char) -> libc::c_int;
    /* *
 * Tivo device names.
 * Defaults to /dev/hd{a,b}.  For a Premier backup, we'll replace these
 * with /dev/sd{a,b}.
 */
    #[no_mangle]
    static mut tivo_devnames: [*mut libc::c_char; 0];
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn isspace(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn strtoul(_: *const libc::c_char, _: *mut *mut libc::c_char,
               _: libc::c_int) -> libc::c_ulong;
    //TODO: Change mediasize to this in order to round to the nearest chunk size (TiVo doesn't bother, so neither do we for now)
				//mediasize = (maxfree - required) & ~(minalloc - 1);; /* only works when minalloc is base-2, which it might not be now, better to use the next one */
				//mediasize = (maxfree - required) / minalloc * minalloc; /* this math works better for rounding down to the nearest minalloc, but doesn't take into account rounding to 4K boundary for modern disks, which tivo_partition_add will do, move along */
				//mediasize = ((maxfree - required) / minalloc * minalloc) / 8 * 8; /* That should do it for rounding to the nearest chunk size (minalloc), if we must */
    /* Round down to the nearest 4K boundary because tivo_partition_add does */
    // Friendly partition names
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn isdigit(_: libc::c_int) -> libc::c_int;
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
#[no_mangle]
pub unsafe extern "C" fn mfsadd_usage(mut progname: *mut libc::c_char) { }
#[no_mangle]
pub unsafe extern "C" fn mfsadd_scan_partitions(mut mfs: *mut mfs_handle,
                                                mut used: *mut libc::c_int,
                                                mut seconddrive:
                                                    *mut libc::c_char)
 -> libc::c_int {
    let mut partitions: [libc::c_char; 256] = [0; 256];
    let mut loop_0: *mut libc::c_char = partitions.as_mut_ptr();
    let mut havebdrive: libc::c_int = 0i32;
    let mut havecdrive: libc::c_int = 0i32;
    strncpy(partitions.as_mut_ptr(), mfs_partition_list(mfs),
            255i32 as libc::c_ulong);
    partitions[255usize] = 0i32 as libc::c_char;
    if 0 ==
           strncmp(partitions.as_mut_ptr(),
                   b"/dev/sd\x00" as *const u8 as *const libc::c_char,
                   7i32 as libc::c_ulong) {
        let ref mut fresh0 = *tivo_devnames.as_mut_ptr().offset(0isize);
        *fresh0 =
            b"/dev/sda\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char;
        let ref mut fresh1 = *tivo_devnames.as_mut_ptr().offset(1isize);
        *fresh1 =
            b"/dev/sdb\x00" as *const u8 as *const libc::c_char as
                *mut libc::c_char
    }
    *seconddrive = 0i32 as libc::c_char;
    while 0 != *loop_0 {
        let mut drive: libc::c_int = 0;
        let mut partition: libc::c_int = 0;
        while 0 != *loop_0 as libc::c_int &&
                  0 != isspace(*loop_0 as libc::c_int) {
            loop_0 = loop_0.offset(1isize)
        }
        // Premiere and later use /dev/sd*
        if !(strncmp(loop_0,
                     b"/dev/hd\x00" as *const u8 as *const libc::c_char,
                     7i32 as libc::c_ulong) == 0i32 ||
                 strncmp(loop_0,
                         b"/dev/sd\x00" as *const u8 as *const libc::c_char,
                         7i32 as libc::c_ulong) == 0i32) {
            return -1i32
        }
        loop_0 = loop_0.offset(7isize);
        match *loop_0 as libc::c_int {
            97 => { drive = 0i32 }
            98 => { drive = 1i32; havebdrive = 1i32 }
            99 => { drive = 1i32; havecdrive = 1i32 }
            _ => { return -1i32 }
        }
        loop_0 = loop_0.offset(1isize);
        partition = strtoul(loop_0, &mut loop_0, 10i32) as libc::c_int;
        if 0 != *loop_0 as libc::c_int &&
               (0 == isspace(*loop_0 as libc::c_int) || partition < 2i32 ||
                    partition > 16i32) {
            return -1i32
        }
        if 0 != *used.offset(drive as isize) & 1i32 << partition {
            return -1i32
        }
        *used.offset(drive as isize) |= 1i32 << partition
    }
    if 0 != havebdrive && 0 != havecdrive { return -1i32 }
    if 0 != havebdrive {
        *seconddrive = 'b' as i32 as libc::c_char
    } else if 0 != havecdrive { *seconddrive = 'c' as i32 as libc::c_char }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn check_partition_count(mut mfs: *mut mfs_handle,
                                               mut pairnums:
                                                   *mut libc::c_char,
                                               mut npairs: libc::c_int)
 -> libc::c_int {
    let mut loop_0: libc::c_int = 0;
    let mut total: libc::c_int = 0;
    total = strlen(mfs_partition_list(mfs)) as libc::c_int;
    loop_0 = 0i32;
    while loop_0 < npairs {
        if *pairnums.offset(loop_0 as isize) as libc::c_int & 31i32 >= 10i32 {
            total += 11i32
        } else { total += 10i32 }
        loop_0 += 1
    }
    if total >= 128i32 { return -1i32 }
    return 0i32;
}