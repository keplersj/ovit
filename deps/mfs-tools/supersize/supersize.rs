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
    // Historically, the drive was accessed as big endian (MSB), however newer platforms (Roamio) are mipsel based, hence the numeric values are little endian (LSB).
    /* Drive is little endian */
    #[no_mangle]
    static mut mfsLSB: libc::c_int;
    #[no_mangle]
    fn mfs_has_error(mfshnd: *mut mfs_handle) -> libc::c_int;
    #[no_mangle]
    fn mfs_perror(mfshnd: *mut mfs_handle, str: *mut libc::c_char);
    #[no_mangle]
    fn mfs_inode_count(mfshnd: *mut mfs_handle) -> uint32_t;
    #[no_mangle]
    fn mfs_read_inode(mfshnd: *mut mfs_handle, inode: uint32_t)
     -> *mut mfs_inode;
    #[no_mangle]
    fn mfs_read_inode_by_fsid(mfshnd: *mut mfs_handle, fsid: uint32_t)
     -> *mut mfs_inode;
    #[no_mangle]
    fn mfs_write_inode(mfshnd: *mut mfs_handle, inode: *mut mfs_inode)
     -> libc::c_int;
    #[no_mangle]
    fn mfs_read_inode_data(mfshnd: *mut mfs_handle, inode: *mut mfs_inode,
                           size: *mut libc::c_int) -> *mut libc::c_uchar;
    #[no_mangle]
    fn mfs_write_inode_data_part(mfshnd: *mut mfs_handle,
                                 inode: *mut mfs_inode,
                                 data: *mut libc::c_uchar, start: uint32_t,
                                 count: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn mfs_resolve(mfshnd: *mut mfs_handle, pathin: *const libc::c_char)
     -> uint32_t;
    #[no_mangle]
    fn parse_object(fsid: libc::c_int, buf: *mut libc::c_void,
                    fn_0: object_fn);
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
pub type uint16_t = libc::c_ushort;
pub type fsid_type_e = libc::c_uchar;
pub const tyDb: fsid_type_e = 8;
pub const tyDir: fsid_type_e = 4;
pub const tyStream: fsid_type_e = 2;
pub const tyFile: fsid_type_e = 1;
pub const tyNone: fsid_type_e = 0;
pub type fsid_type = fsid_type_e;
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mfs_obj_attr_s {
    pub fsid: uint32_t,
    pub subobj: libc::c_int,
}
pub type mfs_obj_attr = mfs_obj_attr_s;
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
static mut mfs: *mut mfs_handle = 0 as *const mfs_handle as *mut mfs_handle;
#[no_mangle]
pub static mut user: libc::c_int = -1i32;
#[no_mangle]
pub static mut clip: libc::c_int = -1i32;
#[no_mangle]
pub static mut max: libc::c_int = 0x7fffffffi32;
#[no_mangle]
pub unsafe extern "C" fn supersize_usage(mut progname: *mut libc::c_char) { }
/* *****************************************************************/
/* Read an inode data based on an fsid, scanning ahead as needed. */
/* Same as mfs_read_inode_by_fsid, but we don't look at refcount.  This allows us to find a inode that WinMFS set to refcount == 0 */
#[no_mangle]
pub unsafe extern "C" fn mfs_read_inode_by_fsid_ignore_refcount(mut mfshnd:
                                                                    *mut mfs_handle,
                                                                mut fsid:
                                                                    uint32_t)
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
/* right return.  */
    if cur.is_null() || intswap32((*cur).fsid) == fsid { return cur }
    /* This is not the inode you are looking for.  Move along. */
    free(cur);
    return 0 as *mut mfs_inode;
}
unsafe extern "C" fn unlock_callback(mut fsid: libc::c_int,
                                     mut obj: *mut mfs_subobj_header,
                                     mut attr: *mut mfs_attr_header,
                                     mut data: *mut libc::c_void) {
    let mut i: libc::c_int = 0;
    let mut p: *mut libc::c_char = data as *mut libc::c_char;
    let mut objattr: *mut mfs_obj_attr = 0 as *mut mfs_obj_attr;
    static mut lasttype: uint16_t = 0;
    static mut lastid: libc::c_int = 0;
    if attr.is_null() { lasttype = intswap16((*obj).obj_type); return }
    match intswap16((*attr).attreltype) as libc::c_int >> 8i32 >> 6i32 {
        0 | 3 => {
            if lasttype as libc::c_int == 111i32 &&
                   intswap16((*attr).attreltype) as libc::c_int & 0xffi32 ==
                       16i32 {
                lastid =
                    intswap32(*(p as *mut libc::c_int) as uint32_t) as
                        libc::c_int
            }
            //MaxDiskSize
            if lasttype as libc::c_int == 112i32 &&
                   intswap16((*attr).attreltype) as libc::c_int & 0xffi32 ==
                       20i32 {
                let mut oldsize: libc::c_int =
                    intswap32(*(p as *mut libc::c_int) as uint32_t) as
                        libc::c_int;
                if oldsize != max {
                    *(p as *mut libc::c_int) =
                        intswap32(max as uint32_t) as libc::c_int
                }
            }
            //User
            if lasttype as libc::c_int == 111i32 &&
                   intswap16((*attr).attreltype) as libc::c_int & 0xffi32 ==
                       17i32 && lastid == 10i32 {
                let mut oldsize_0: libc::c_int =
                    intswap32(*(p as *mut libc::c_int) as uint32_t) as
                        libc::c_int;
                if oldsize_0 != user {
                    *(p as *mut libc::c_int) =
                        intswap32(user as uint32_t) as libc::c_int
                }
            }
            //TivoClips
            if lasttype as libc::c_int == 111i32 &&
                   intswap16((*attr).attreltype) as libc::c_int & 0xffi32 ==
                       17i32 && lastid == 11i32 {
                let mut oldsize_1: libc::c_int =
                    intswap32(*(p as *mut libc::c_int) as uint32_t) as
                        libc::c_int;
                if oldsize_1 != clip {
                    *(p as *mut libc::c_int) =
                        intswap32(clip as uint32_t) as libc::c_int
                }
            }
        }
        1 | 2 | _ => { }
    };
}
#[no_mangle]
pub unsafe extern "C" fn supersize() -> libc::c_int {
    let mut fsid: uint32_t =
        mfs_resolve(mfs,
                    b"/Config/DiskConfigurations/Active\x00" as *const u8 as
                        *const libc::c_char);
    let mut inode: *mut mfs_inode = 0 as *mut mfs_inode;
    let mut buf: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut size: uint32_t = 0;
    if fsid == 0i32 as libc::c_uint { return 0i32 }
    inode = mfs_read_inode_by_fsid(mfs, fsid);
    if inode.is_null() {
        //We'll make an effort to undo a WinMFS supersize...
        inode = mfs_read_inode_by_fsid_ignore_refcount(mfs, fsid);
        if inode.is_null() { return 0i32 }
        // Let's reset the refcount to 3
        (*inode).refcount = intswap32(3i32 as uint32_t);
        mfs_write_inode(mfs, inode);
    }
    if (*inode).type_0 as libc::c_int != tyDb as libc::c_int { return 0i32 }
    if intswap32((*inode).unk3) == 0x20000i32 as libc::c_uint {
        size = intswap32((*inode).size).wrapping_mul(intswap32((*inode).unk3))
    } else { size = intswap32((*inode).size) }
    buf =
        mfs_read_inode_data(mfs, inode,
                            &mut size as *mut uint32_t as *mut libc::c_int) as
            *mut libc::c_void;
    parse_object(fsid as libc::c_int, buf,
                 Some(unlock_callback as
                          unsafe extern "C" fn(_: libc::c_int,
                                               _: *mut mfs_subobj_header,
                                               _: *mut mfs_attr_header,
                                               _: *mut libc::c_void) -> ()));
    mfs_write_inode_data_part(mfs, inode, buf as *mut libc::c_uchar,
                              0i32 as uint32_t, size);
    free(buf);
    panic!("Reached end of non-void function without returning");
}