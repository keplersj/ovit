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
    fn mfs_check_crc(data: *mut libc::c_uchar, size: libc::c_uint,
                     off: libc::c_uint) -> libc::c_uint;
    #[no_mangle]
    fn mfs_update_crc(data: *mut libc::c_uchar, size: libc::c_uint,
                      off: libc::c_uint);
    #[no_mangle]
    fn mfs_perror(mfshnd: *mut mfs_handle, str: *mut libc::c_char);
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
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
    pub sector: libc::c_int,
    pub count: libc::c_int,
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
    pub fsid: libc::c_int,
    pub type_0: fsid_type,
    pub name: *mut libc::c_char,
}
pub type mfs_dirent = mfs_dirent_s;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mfs_obj_header_s {
    pub fill1: libc::c_int,
    pub size: libc::c_int,
}
pub type mfs_obj_header = mfs_obj_header_s;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mfs_subobj_header_s {
    pub len: libc::c_int,
    pub len1: libc::c_int,
    pub obj_type: libc::c_int,
    pub flags: libc::c_int,
    pub fill: [libc::c_int; 2],
    pub id: libc::c_int,
}
pub type mfs_subobj_header = mfs_subobj_header_s;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mfs_attr_header_s {
    pub attreltype: libc::c_int,
    pub len: libc::c_int,
}
pub type mfs_attr_header = mfs_attr_header_s;
/* Borrowed from mfs-utils */
/* return a string identifier for a tivo file type */
pub type object_fn
    =
    Option<unsafe extern "C" fn(_: libc::c_int, _: *mut mfs_subobj_header,
                                _: *mut mfs_attr_header, _: *mut libc::c_void)
               -> ()>;
/* ************************************/
/* Read an inode data and return it. */
#[no_mangle]
pub unsafe extern "C" fn mfs_read_inode(mut mfshnd: *mut mfs_handle,
                                        mut inode: libc::c_uint)
 -> *mut mfs_inode {
    let mut in_0: *mut mfs_inode =
        calloc(512i32 as libc::c_ulong, 1i32 as libc::c_ulong) as
            *mut mfs_inode;
    if mfs_read_inode_to_buf(mfshnd, inode, in_0) <= 0i32 { free(in_0); }
    return in_0;
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
    }
    data =
        malloc(((*size + 511i32) as libc::c_uint & !511i32 as libc::c_uint) as
                   libc::c_ulong) as *mut libc::c_uchar;
    if data.is_null() { *size = 0i32 }
    /* This function is just a wrapper for read_inode_data_part, with the last */
/* parameter being implicitly the whole data. */
    if result < 0i32 { *size = result; free(data); }
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
        i += 1
    }
    free(dir);
}
/* list a mfs directory - make sure you free with mfs_dir_free() */
#[no_mangle]
pub unsafe extern "C" fn mfs_dir(mut mfshnd: *mut mfs_handle,
                                 mut fsid: libc::c_int,
                                 mut count: *mut libc::c_int)
 -> *mut mfs_dirent {
    let mut n: libc::c_int = 0i32;
    let mut i: libc::c_int = 0;
    let mut dsize: libc::c_int = 0;
    let mut dflags: libc::c_int = 0;
    let mut ret: *mut mfs_dirent = 0 as *mut mfs_dirent;
    let mut inode: *mut mfs_inode = 0 as *mut mfs_inode;
    let mut size: libc::c_int = 0i32;
    if (*inode).type_0 as libc::c_int != tyDir as libc::c_int {
        (*mfshnd).err_msg =
            b"fsid %d is not a tyDir\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        mfs_perror(mfshnd,
                   b"mfs_dir\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char);
    }
    ret =
        malloc(((n + 1i32) as
                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<mfs_dirent>()
                                                    as libc::c_ulong)) as
            *mut mfs_dirent;
    i = 0i32;
    while i < n { i += 1 }
    /* handle meta-directories. These are just directories which are
	   lists of other directories. All we need to do is recursively read
	   the other directories and piece together the top level directory */
    if dflags == 0x200i32 {
        let mut meta_dir: *mut mfs_dirent = 0 as *mut mfs_dirent;
        let mut meta_size: libc::c_int = 0i32;
        i = 0i32;
        while i < n {
            let mut d2: *mut mfs_dirent = 0 as *mut mfs_dirent;
            let mut n2: libc::c_uint = 0;
            if (*ret.offset(i as isize)).type_0 as libc::c_int !=
                   tyDir as libc::c_int {
                (*mfshnd).err_msg =
                    b"ERROR: non dir %d/%s in meta-dir %d!\x00" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                mfs_perror(mfshnd,
                           b"mfs_dir\x00" as *const u8 as *const libc::c_char
                               as *mut libc::c_char);
            } else if !(d2.is_null() || n2 == 0i32 as libc::c_uint) {
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
            i += 1
        }
        mfs_dir_free(ret);
        return meta_dir
    }
    return ret;
}
/* this is the low-level interface to parsing an object. It will call fn() on
   all elements in all subobjects */
#[no_mangle]
pub unsafe extern "C" fn parse_object(mut fsid: libc::c_int,
                                      mut buf: *mut libc::c_void,
                                      mut fn_0: object_fn) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut obj: *mut mfs_obj_header = buf as *mut mfs_obj_header;
    let mut i: libc::c_int = 0i32;
    p = buf as *mut libc::c_char;
}