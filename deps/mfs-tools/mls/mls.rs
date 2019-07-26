#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, extern_types)]
extern crate libc;
extern "C" {
    pub type log_hdr_s;
    pub type tivo_partition_file;
    #[no_mangle]
    fn mfs_resolve(mfshnd: *mut mfs_handle, pathin: *const libc::c_char)
     -> uint32_t;
    #[no_mangle]
    fn mfs_dir(mfshnd: *mut mfs_handle, fsid: libc::c_int,
               count: *mut uint32_t) -> *mut mfs_dirent;
    // Historically, the drive was accessed as big endian (MSB), however newer platforms (Roamio) are mipsel based, hence the numeric values are little endian (LSB).
    /* Drive is little endian */
    #[no_mangle]
    static mut mfsLSB: libc::c_int;
    #[no_mangle]
    fn mfs_read_inode_by_fsid(mfshnd: *mut mfs_handle, fsid: uint32_t)
     -> *mut mfs_inode;
    #[no_mangle]
    fn mfs_type_string(type_0: fsid_type) -> *mut libc::c_char;
    #[no_mangle]
    fn mfs_dir_free(dir: *mut mfs_dirent);
    #[no_mangle]
    fn mfs_perror(mfshnd: *mut mfs_handle, str: *mut libc::c_char);
    #[no_mangle]
    fn mfs_has_error(mfshnd: *mut mfs_handle) -> libc::c_int;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
#[inline]
unsafe extern "C" fn Endian32_Swap(mut var: uint32_t) -> uint32_t {
    var = var << 16i32 | var >> 16i32;
    var = (var & 0xff00ff00u32) >> 8i32 | var << 8i32 & 0xff00ff00u32;
    return var;
}
#[inline]
unsafe extern "C" fn intswap32(mut n: uint32_t) -> uint32_t {
    if mfsLSB == 0i32 { return n }
    return Endian32_Swap(n);
}
#[no_mangle]
pub static mut progname: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut mfs: *mut mfs_handle = 0 as *const mfs_handle as *mut mfs_handle;
static mut long_list: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn mls_usage() { }
unsafe extern "C" fn dir_list(mut fsid: libc::c_int,
                              mut recurse: libc::c_int) {
    let mut dir: *mut mfs_dirent = 0 as *mut mfs_dirent;
    let mut count: uint32_t = 0;
    let mut i: uint32_t = 0;
    dir = mfs_dir(mfs, fsid, &mut count);
    if fsid == 0i32 { exit(1i32); }
    if dir.is_null() { exit(1i32); }
    if 0 != long_list {
        printf(b"     FsId Type         Date  Time      Size Name\n\x00" as
                   *const u8 as *const libc::c_char);
        printf(b"     ---- ----         ----  ----      ---- ----\n\x00" as
                   *const u8 as *const libc::c_char);
    } else {
        printf(b"      FsId   Type     Name\n\x00" as *const u8 as
                   *const libc::c_char);
        printf(b"      ----   ----     ----\n\x00" as *const u8 as
                   *const libc::c_char);
    }
    i = 0i32 as uint32_t;
    while i < count {
        let mut date: [libc::c_char; 17] =
            *::std::mem::transmute::<&[u8; 17],
                                     &mut [libc::c_char; 17]>(b"xx/xx/xx xx:xx\x00\x00\x00");
        if 0 != long_list {
            let mut inode: *mut mfs_inode = 0 as *mut mfs_inode;
            let mut size: uint64_t = 0i32 as uint64_t;
            inode =
                mfs_read_inode_by_fsid(mfs, (*dir.offset(i as isize)).fsid);
            if !inode.is_null() {
                if intswap32((*inode).unk3) == 0x20000i32 as libc::c_uint {
                    size =
                        intswap32((*inode).size).wrapping_mul(intswap32((*inode).unk3))
                            as uint64_t
                } else { size = intswap32((*inode).size) as uint64_t }
            }
            printf(b"%9d %-8s %14s%10lld %s\n\x00" as *const u8 as
                       *const libc::c_char, (*dir.offset(i as isize)).fsid,
                   mfs_type_string((*dir.offset(i as isize)).type_0),
                   date.as_mut_ptr(), size, (*dir.offset(i as isize)).name);
            if !inode.is_null() { free(inode); }
        } else {
            printf(b"   %7d   %-8s %s\n\x00" as *const u8 as
                       *const libc::c_char, (*dir.offset(i as isize)).fsid,
                   mfs_type_string((*dir.offset(i as isize)).type_0),
                   (*dir.offset(i as isize)).name);
        }
        i = i.wrapping_add(1)
    }
    if 0 != recurse {
        i = 0i32 as uint32_t;
        while i < count {
            if (*dir.offset(i as isize)).type_0 as libc::c_int ==
                   tyDir as libc::c_int {
                printf(b"\n%s[%d]:\n\x00" as *const u8 as *const libc::c_char,
                       (*dir.offset(i as isize)).name,
                       (*dir.offset(i as isize)).fsid);
                dir_list((*dir.offset(i as isize)).fsid as libc::c_int, 1i32);
            }
            i = i.wrapping_add(1)
        }
    }
    if !dir.is_null() { mfs_dir_free(dir); };
}
#[no_mangle]
pub unsafe extern "C" fn mls_main(mut argc: libc::c_int,
                                  mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut opt: libc::c_int = 0i32;
    let mut fsid: libc::c_int = 0;
    let mut recurse: libc::c_int = 0i32;
    let mut arg: *mut libc::c_char = *argv.offset(1isize);
    let mut hda: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hdb: *mut libc::c_char = 0 as *mut libc::c_char;
    progname = *argv.offset(0isize);
    tivo_partition_direct();
    loop  {
        opt =
            getopt(argc, argv,
                   b"hRl\x00" as *const u8 as *const libc::c_char);
        if !(opt > 0i32) { break ; }
        match opt {
            82 => { recurse = 1i32 }
            108 => { long_list = 1i32 }
            _ => { mls_usage(); return 1i32 }
        }
    }
    if argc > 2i32 {
        hda = *argv.offset(1isize);
        arg = *argv.offset(2isize);
        if argc > 3i32 {
            hdb = *argv.offset(2isize);
            arg = *argv.offset(3isize);
            if argc > 4i32 { mls_usage(); return 1i32 }
        }
    } else {
        hda =
            getenv(b"MFS_HDA\x00" as *const u8 as *const libc::c_char) as
                *mut libc::c_char;
        hdb =
            getenv(b"MFS_HDB\x00" as *const u8 as *const libc::c_char) as
                *mut libc::c_char;
        if hda.is_null() || 0 == *hda {
            hda =
                b"/dev/hda\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char;
            hdb =
                b"/dev/hdb\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char
        }
    }
    if mfs.is_null() { return 1i32 }
    if 0 != mfs_has_error(mfs) {
        mfs_perror(mfs, *argv.offset(0isize));
        return 1i32
    }
    fsid = mfs_resolve(mfs, arg) as libc::c_int;
    dir_list(fsid, recurse);
    return 0i32;
}