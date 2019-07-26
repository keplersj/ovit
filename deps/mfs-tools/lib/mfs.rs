#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast,
           extern_types,
           ptr_wrapping_offset_from)]
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
    fn mfs_load_zone_maps(hnd: *mut mfs_handle) -> libc::c_int;
    #[no_mangle]
    fn mfsvol_volume_size(hnd: *mut volume_handle, sector: uint64_t)
     -> uint64_t;
    #[no_mangle]
    fn mfsvol_volume_set_size(hnd: *mut volume_handle) -> uint64_t;
    #[no_mangle]
    fn mfsvol_read_data(hnd: *mut volume_handle, buf: *mut libc::c_void,
                        sector: uint64_t, count: uint32_t) -> libc::c_int;
    #[no_mangle]
    fn mfsvol_write_data(hnd: *mut volume_handle, buf: *mut libc::c_void,
                         sector: uint64_t, count: uint32_t) -> libc::c_int;
    #[no_mangle]
    fn mfsvol_cleanup(hnd: *mut volume_handle);
    #[no_mangle]
    fn mfsvol_init(hda: *const libc::c_char, hdb: *const libc::c_char)
     -> *mut volume_handle;
    #[no_mangle]
    fn mfsvol_perror(hnd: *mut volume_handle, str: *mut libc::c_char);
    #[no_mangle]
    fn mfsvol_strerror(hnd: *mut volume_handle, str: *mut libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn mfsvol_has_error(hnd: *mut volume_handle) -> libc::c_int;
    #[no_mangle]
    fn mfsvol_clearerror(hnd: *mut volume_handle);
    #[no_mangle]
    fn mfs_cleanup_zone_maps(mfshnd: *mut mfs_handle);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_ulong;
    #[no_mangle]
    fn strspn(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_ulong;
    #[no_mangle]
    fn bzero(_: *mut libc::c_void, _: libc::c_ulong);
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
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
// #ifdef HAVE_ERRNO_H
// #endif
#[no_mangle]
pub static mut tivo_devnames: [*mut libc::c_char; 2] =
    [b"/dev/hda\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"/dev/hdb\x00" as *const u8 as *const libc::c_char as
         *mut libc::c_char];
#[no_mangle]
pub static mut mfsLSB: libc::c_int = 0i32;
#[no_mangle]
pub static mut partLSB: libc::c_int = 0i32;
/* ************************************/
/* Write the volume header back out. */
#[no_mangle]
pub unsafe extern "C" fn mfs_write_volume_header(mut mfshnd: *mut mfs_handle)
 -> libc::c_int {
    let mut buf: [libc::c_uchar; 512] = [0; 512];
    memset(buf.as_mut_ptr() as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<[libc::c_uchar; 512]>() as libc::c_ulong);
    if 0 != (*mfshnd).is_64 {
        mfs_update_crc(&mut (*mfshnd).vol_hdr.v64 as *mut volume_header_64 as
                           *mut libc::c_uchar,
                       ::std::mem::size_of::<volume_header_64>() as
                           libc::c_ulong as libc::c_uint,
                       (&mut (*mfshnd).vol_hdr.v64.checksum as *mut uint32_t
                            as
                            *mut libc::c_uint).wrapping_offset_from(&mut (*mfshnd).vol_hdr.v64
                                                                        as
                                                                        *mut volume_header_64
                                                                        as
                                                                        *mut libc::c_uint)
                           as libc::c_long as libc::c_uint);
    } else {
        mfs_update_crc(&mut (*mfshnd).vol_hdr.v32 as *mut volume_header_32 as
                           *mut libc::c_uchar,
                       ::std::mem::size_of::<volume_header_32>() as
                           libc::c_ulong as libc::c_uint,
                       (&mut (*mfshnd).vol_hdr.v32.checksum as *mut uint32_t
                            as
                            *mut libc::c_uint).wrapping_offset_from(&mut (*mfshnd).vol_hdr.v32
                                                                        as
                                                                        *mut volume_header_32
                                                                        as
                                                                        *mut libc::c_uint)
                           as libc::c_long as libc::c_uint);
    }
    memcpy(buf.as_mut_ptr() as *mut libc::c_void,
           &mut (*mfshnd).vol_hdr as *mut volume_header as
               *const libc::c_void,
           ::std::mem::size_of::<volume_header>() as libc::c_ulong);
    if mfsvol_write_data((*mfshnd).vols,
                         buf.as_mut_ptr() as *mut libc::c_void,
                         0i32 as uint64_t, 1i32 as uint32_t) != 512i32 {
        (*mfshnd).err_msg =
            b"%s writing volume header\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char;
        return -1i32
    }
    if mfsvol_write_data((*mfshnd).vols,
                         buf.as_mut_ptr() as *mut libc::c_void,
                         mfsvol_volume_size((*mfshnd).vols,
                                            0i32 as
                                                uint64_t).wrapping_sub(1i32 as
                                                                           libc::c_ulonglong),
                         1i32 as uint32_t) != 512i32 {
        (*mfshnd).err_msg =
            b"%s writing volume header\x00" as *const u8 as
                *const libc::c_char as *mut libc::c_char;
        return -1i32
    }
    return 512i32;
}
/* **********************************************************************/
/* Return the list of partitions from the volume header.  That is all. */
#[no_mangle]
pub unsafe extern "C" fn mfs_partition_list(mut mfshnd: *mut mfs_handle)
 -> *mut libc::c_char {
    if 0 != (*mfshnd).is_64 {
        return (*mfshnd).vol_hdr.v64.partitionlist.as_mut_ptr()
    } else { return (*mfshnd).vol_hdr.v32.partitionlist.as_mut_ptr() };
}
/* *******************************/
/* Wrapper for first init case. */
/* The caller is responsible for checking for error cases. */
#[no_mangle]
pub unsafe extern "C" fn mfs_init(mut hda: *mut libc::c_char,
                                  mut hdb: *mut libc::c_char,
                                  mut flags: libc::c_int) -> *mut mfs_handle {
    let mut mfshnd: *mut mfs_handle =
        malloc(::std::mem::size_of::<mfs_handle>() as libc::c_ulong) as
            *mut mfs_handle;
    if mfshnd.is_null() { return 0 as *mut mfs_handle }
    mfs_init_internal(mfshnd, hda, hdb, flags);
    return mfshnd;
}
/* ************************/
/* Display the MFS error */
#[no_mangle]
pub unsafe extern "C" fn mfs_perror(mut mfshnd: *mut mfs_handle,
                                    mut str: *mut libc::c_char) {
    let mut err: libc::c_int = 0i32;
    if !(*mfshnd).err_msg.is_null() { err = 1i32 }
    if !(*(*mfshnd).vols).err_msg.is_null() {
        mfsvol_perror((*mfshnd).vols, str);
        err = 2i32
    }
    err == 0i32;
}
/* ************************************/
/* Return the MFS error in a string. */
#[no_mangle]
pub unsafe extern "C" fn mfs_strerror(mut mfshnd: *mut mfs_handle,
                                      mut str: *mut libc::c_char)
 -> libc::c_int {
    if !(*mfshnd).err_msg.is_null() {
        sprintf(str, (*mfshnd).err_msg, (*mfshnd).err_arg1,
                (*mfshnd).err_arg2, (*mfshnd).err_arg3);
    } else { return mfsvol_strerror((*mfshnd).vols, str) }
    return 1i32;
}
/* ******************************/
/* Check if there is an error. */
#[no_mangle]
pub unsafe extern "C" fn mfs_has_error(mut mfshnd: *mut mfs_handle)
 -> libc::c_int {
    if !(*mfshnd).err_msg.is_null() { return 1i32 }
    return mfsvol_has_error((*mfshnd).vols);
}
/* *******************/
/* Clear any errors */
#[no_mangle]
pub unsafe extern "C" fn mfs_clearerror(mut mfshnd: *mut mfs_handle) {
    (*mfshnd).err_msg = 0 as *mut libc::c_char;
    (*mfshnd).err_arg1 = 0i32 as int64_t;
    (*mfshnd).err_arg2 = 0i32 as int64_t;
    (*mfshnd).err_arg3 = 0i32 as int64_t;
    if !(*mfshnd).vols.is_null() { mfsvol_clearerror((*mfshnd).vols); };
}
/* ***********************************************/
/* Free all used memory and close opened files. */
#[no_mangle]
pub unsafe extern "C" fn mfs_cleanup(mut mfshnd: *mut mfs_handle) {
    mfs_cleanup_zone_maps(mfshnd);
    if !(*mfshnd).vols.is_null() { mfsvol_cleanup((*mfshnd).vols); }
    if !(*mfshnd).current_log.is_null() { free((*mfshnd).current_log); }
    free(mfshnd);
}
/* *******************************/
/* Do a cleanup and init fresh. */
/* The caller is responsible for checking for error cases. */
#[no_mangle]
pub unsafe extern "C" fn mfs_reinit(mut mfshnd: *mut mfs_handle,
                                    mut flags: libc::c_int) -> libc::c_int {
    let mut vols: *mut volume_handle = (*mfshnd).vols;
    mfs_cleanup_zone_maps(mfshnd);
    mfs_init_internal(mfshnd, (*vols).hda, (*vols).hdb, flags);
    mfsvol_cleanup(vols);
    return 0i32;
}