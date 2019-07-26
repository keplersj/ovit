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
    fn mfs_partition_list(mfshnd: *mut mfs_handle) -> *mut libc::c_char;
    #[no_mangle]
    fn mfs_has_error(mfshnd: *mut mfs_handle) -> libc::c_int;
    #[no_mangle]
    fn mfs_perror(mfshnd: *mut mfs_handle, str: *mut libc::c_char);
    #[no_mangle]
    fn mfs_volume_pair_app_size(mfshnd: *mut mfs_handle, blocks: uint64_t,
                                minalloc: libc::c_uint) -> uint64_t;
    #[no_mangle]
    fn mfs_sa_hours_estimate(mfshnd: *mut mfs_handle) -> libc::c_uint;
    #[no_mangle]
    fn tivo_partition_count(device: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn tivo_partition_total_free(device: *const libc::c_char) -> uint64_t;
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
    fn tivo_partition_add(device: *const libc::c_char, size: uint64_t,
                          before: libc::c_int, name: *const libc::c_char,
                          type_0: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn tivo_partition_table_write(device: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn tivo_partition_largest_free(device: *const libc::c_char) -> uint64_t;
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
pub type int64_t = libc::c_longlong;
pub type uint64_t = libc::c_ulonglong;
pub type uint32_t = libc::c_uint;
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
pub unsafe extern "C" fn mfsadd_add_extends(mut mfs: *mut mfs_handle,
                                            mut drives:
                                                *mut *mut libc::c_char,
                                            mut xdevs: *mut *mut libc::c_char,
                                            mut pairs: *mut *mut libc::c_char,
                                            mut pairnums: *mut libc::c_char,
                                            mut npairs: *mut libc::c_int,
                                            mut minalloc: libc::c_int,
                                            mut maxdisk: int64_t,
                                            mut maxmedia: int64_t,
                                            mut fill: libc::c_int)
 -> libc::c_int {
    let mut loop_0: libc::c_int = 0i32;
    let mut loop2: libc::c_int = 0i32;
    let mut tmp: libc::c_char = 0;
    let mut appname: [libc::c_char; 32] = [0; 32];
    let mut medianame: [libc::c_char; 32] = [0; 32];
    let mut nparts: libc::c_int = 0i32;
    let mut mfs_partitions: *mut libc::c_char = 0 as *mut libc::c_char;
    // Get the current mfs partition count, so we can name the adds and make sure we don't exceed the max allowed
    mfs_partitions = mfs_partition_list(mfs);
    loop_0 = 0i32;
    while 0 != *mfs_partitions.offset(loop_0 as isize) {
        nparts += 1;
        while 0 != *mfs_partitions.offset(loop_0 as isize) as libc::c_int &&
                  0 ==
                      isspace(*mfs_partitions.offset(loop_0 as isize) as
                                  libc::c_int) {
            loop_0 += 1
        }
        while 0 != *mfs_partitions.offset(loop_0 as isize) as libc::c_int &&
                  0 !=
                      isspace(*mfs_partitions.offset(loop_0 as isize) as
                                  libc::c_int) {
            loop_0 += 1
        }
    }
    loop_0 = 0i32;
    while loop_0 < 2i32 && !(*xdevs.offset(loop_0 as isize)).is_null() {
        loop  {
            let mut maxfree: uint64_t =
                tivo_partition_largest_free(*xdevs.offset(loop_0 as isize));
            let mut totalfree: uint64_t =
                tivo_partition_total_free(*xdevs.offset(loop_0 as isize));
            let mut totalused: uint64_t =
                tivo_partition_total_used(*xdevs.offset(loop_0 as isize)) as
                    uint64_t;
            let mut mediasize: uint64_t = 0i32 as uint64_t;
            let mut appsize: uint64_t = 0i32 as uint64_t;
            let mut part1: libc::c_uint = 0;
            let mut part2: libc::c_uint = 0;
            let mut devn: libc::c_int =
                if *xdevs.offset(loop_0 as isize) == *drives.offset(0isize) {
                    0i32
                } else { 1i32 };
            if maxfree < (1024i32 * 1024i32 * 2i32) as libc::c_ulonglong {
                break ;
            }
            // Limit the total disk size if set
            if 0 != maxdisk &&
                   (maxdisk as libc::c_ulonglong) <
                       totalfree.wrapping_add(totalused) {
                if maxdisk as libc::c_ulonglong > totalused {
                    totalfree =
                        (maxdisk as libc::c_ulonglong).wrapping_sub(totalused)
                } else { totalfree = 0i32 as uint64_t }
                if maxfree > totalfree { maxfree = totalfree }
            }
            // Limit the parttion size if needed
            if 0 != maxmedia && (maxmedia as libc::c_ulonglong) < maxfree {
                maxfree = maxmedia as uint64_t
            }
            // TODO: Change mediasize to this in order to round to the nearest chunk size (TiVo doesn't bother, so neither do we for now)
			//mediasize = maxfree & ~(minalloc - 1); /* only works when minalloc is base-2, which it might not be now, better to use the next one */
			//mediasize = maxfree / minalloc * minalloc; /* this math works better for rounding down to the nearest minalloc, but doesn't take into account rounding to 4K boundary for modern disks, which tivo_partition_add will do, move along */
			//mediasize = (maxfree / minalloc * minalloc) / 8 * 8; /* That should do it for rounding to the nearest chunk size (minalloc), if we must */
            /* Round down to the nearest 4K boundary because tivo_partition_add does */
            mediasize =
                maxfree.wrapping_div(8i32 as
                                         libc::c_ulonglong).wrapping_mul(8i32
                                                                             as
                                                                             libc::c_ulonglong);
            appsize =
                mfs_volume_pair_app_size(mfs, mediasize,
                                         minalloc as libc::c_uint);
            if totalfree.wrapping_sub(maxfree) < appsize &&
                   maxfree.wrapping_sub(mediasize) < appsize {
                //TODO: Change mediasize to this in order to round to the nearest chunk size (TiVo doesn't bother, so neither do we for now)
				//mediasize = (maxfree - required) & ~(minalloc - 1);; /* only works when minalloc is base-2, which it might not be now, better to use the next one */
				//mediasize = (maxfree - required) / minalloc * minalloc; /* this math works better for rounding down to the nearest minalloc, but doesn't take into account rounding to 4K boundary for modern disks, which tivo_partition_add will do, move along */
				//mediasize = ((maxfree - required) / minalloc * minalloc) / 8 * 8; /* That should do it for rounding to the nearest chunk size (minalloc), if we must */
                /* Round down to the nearest 4K boundary because tivo_partition_add does */
                mediasize =
                    maxfree.wrapping_sub(appsize).wrapping_div(8i32 as
                                                                   libc::c_ulonglong).wrapping_mul(8i32
                                                                                                       as
                                                                                                       libc::c_ulonglong);
                appsize =
                    mfs_volume_pair_app_size(mfs, mediasize,
                                             minalloc as libc::c_uint)
            }
            sprintf(appname.as_mut_ptr(),
                    b"MFS application region  %d\x00" as *const u8 as
                        *const libc::c_char,
                    *npairs / 2i32 + nparts / 2i32 + 1i32);
            sprintf(medianame.as_mut_ptr(),
                    b"MFS media region %d\x00" as *const u8 as
                        *const libc::c_char,
                    *npairs / 2i32 + nparts / 2i32 + 1i32);
            if totalfree.wrapping_sub(maxfree) >= appsize &&
                   maxfree.wrapping_sub(mediasize) < appsize {
                part2 =
                    tivo_partition_add(*xdevs.offset(loop_0 as isize),
                                       mediasize, 0i32,
                                       medianame.as_mut_ptr(),
                                       b"MFS\x00" as *const u8 as
                                           *const libc::c_char) as
                        libc::c_uint;
                part1 =
                    tivo_partition_add(*xdevs.offset(loop_0 as isize),
                                       appsize, part2 as libc::c_int,
                                       appname.as_mut_ptr(),
                                       b"MFS\x00" as *const u8 as
                                           *const libc::c_char) as
                        libc::c_uint;
                part2 = part2.wrapping_add(1)
            } else {
                part1 =
                    tivo_partition_add(*xdevs.offset(loop_0 as isize),
                                       appsize, 0i32, appname.as_mut_ptr(),
                                       b"MFS\x00" as *const u8 as
                                           *const libc::c_char) as
                        libc::c_uint;
                part2 =
                    tivo_partition_add(*xdevs.offset(loop_0 as isize),
                                       mediasize, 0i32,
                                       medianame.as_mut_ptr(),
                                       b"MFS\x00" as *const u8 as
                                           *const libc::c_char) as
                        libc::c_uint
            }
            if part1 < 2i32 as libc::c_uint || part2 < 2i32 as libc::c_uint ||
                   part1 > 16i32 as libc::c_uint ||
                   part2 > 16i32 as libc::c_uint {
                if 0 != *npairs {
                    // We were able to expand, just not in this iteration
                    break ;
                } else { return -1i32 }
            } else {
                let fresh2 = *npairs;
                *npairs = *npairs + 1;
                *pairnums.offset(fresh2 as isize) =
                    ((devn << 6i32) as libc::c_uint | part1) as libc::c_char;
                let fresh3 = *npairs;
                *npairs = *npairs + 1;
                *pairnums.offset(fresh3 as isize) =
                    ((devn << 6i32) as libc::c_uint | part2) as libc::c_char;
                if (*pairs.offset((*npairs - 2i32) as isize)).is_null() ||
                       (*pairs.offset((*npairs - 1i32) as isize)).is_null() {
                    if 0 != *npairs {
                        // We were able to expand, just not in this iteration
                        break ;
                    } else { return -1i32 }
                } else if fill == 0i32 {
                    // Not asked to create multiple mfs partitions on a drive
                    break ;
                } else if totalfree.wrapping_sub(mediasize).wrapping_sub(appsize)
                              < (minalloc + 4i32) as libc::c_ulonglong {
                    // Out of space
                    break ;
                } else if part2.wrapping_add(2i32 as libc::c_uint) >
                              16i32 as libc::c_uint {
                    // Reached the max partitions for this drive
                    break ;
                } else {
                    if !(nparts + *npairs + 2i32 > 12i32) { continue ; }
                    // Reached the max total partitions
                    break ;
                }
            }
        }
        loop_0 += 1
    }
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