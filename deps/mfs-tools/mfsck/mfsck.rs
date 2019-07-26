#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
extern crate libc;
extern "C" {
    // Historically, the drive was accessed as big endian (MSB), however newer platforms (Roamio) are mipsel based, hence the numeric values are little endian (LSB).
    /* Drive is little endian */
    #[no_mangle]
    static mut mfsLSB: libc::c_int;
    #[no_mangle]
    fn mfsvol_volume_set_size(hnd: *mut volume_handle) -> uint64_t;
    #[no_mangle]
    fn mfs_inode_count(mfshnd: *mut mfs_handle) -> uint32_t;
    #[no_mangle]
    fn mfs_read_inode(mfshnd: *mut mfs_handle, inode: uint32_t)
     -> *mut mfs_inode;
    #[no_mangle]
    fn mfs_perror(mfshnd: *mut mfs_handle, str: *mut libc::c_char);
    #[no_mangle]
    fn mfs_strerror(mfshnd: *mut mfs_handle, str: *mut libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn mfs_has_error(mfshnd: *mut mfs_handle) -> libc::c_int;
    #[no_mangle]
    fn mfs_clearerror(mfshnd: *mut mfs_handle);
    #[no_mangle]
    fn mfsvol_enable_memwrite(hnd: *mut volume_handle);
    /* Size of each bitmap is (nints + (nbits < 8? 1: 2)) * 4 */
/* Don't ask why, thats just the way it is. */
/* In bitmap, MSB is first, LSB last */
    #[no_mangle]
    fn mfs_next_zone(mfshdn: *mut mfs_handle, cur: *mut zone_header)
     -> *mut zone_header;
    #[no_mangle]
    fn tivo_partition_direct();
    #[no_mangle]
    fn mfs_log_fssync(mfshnd: *mut mfs_handle) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
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
pub type size_t = libc::c_ulong;
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
/* Represents the lowest level bitmap for a zone */
/* This is the opposite order from TiVo bitmaps - bit 0 is LSB */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct zone_bitmap_s {
    pub first: uint64_t,
    pub last: uint64_t,
    pub blocksize: uint32_t,
    pub bits: *mut uint32_t,
    pub fsids: *mut uint32_t,
    pub type_0: libc::c_int,
    pub next: *mut zone_bitmap_s,
}
pub type zone_bitmap = zone_bitmap_s;
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
#[no_mangle]
pub unsafe extern "C" fn mfsck_usage(mut progname: *mut libc::c_char) { }
#[no_mangle]
pub unsafe extern "C" fn scan_bit_range(mut map: *mut zone_bitmap,
                                        mut startbit: libc::c_int,
                                        mut endbit: libc::c_int,
                                        mut desiredval: libc::c_int)
 -> libc::c_int {
    let mut desiredbits: libc::c_uint = 0i32 as libc::c_uint;
    let mut startint: libc::c_int = 0;
    let mut endint: libc::c_int = 0;
    let mut startbits: libc::c_uint = 0;
    let mut endbits: libc::c_uint = 0;
    if 0 != desiredval { desiredbits = !0i32 as libc::c_uint }
    startint = startbit / 32i32;
    endint = endbit / 32i32;
    startbit = startbit & 31i32;
    endbit = endbit & 31i32;
    startbits = !((1i32 << startbit) - 1i32) as libc::c_uint;
    if endbit == 31i32 {
        endbits = !0i32 as libc::c_uint
    } else { endbits = ((1i32 << endbit + 1i32) - 1i32) as libc::c_uint }
    /* Easy case, they are the same int, so check the range between */
    if startint == endint {
        return (*(*map).bits.offset(startint as isize) & startbits & endbits
                    == desiredbits & startbits & endbits) as libc::c_int
    }
    /* Check the bits in the first int */
    if *(*map).bits.offset(startint as isize) & startbits !=
           desiredbits & startbits {
        return 0i32
    }
    /* Check all the ints inbetween */
    loop  {
        startint += 1;
        if !(startint < endint) { break ; }
        if *(*map).bits.offset(startint as isize) != desiredbits {
            return 0i32
        }
    }
    /* Check the bits in the last int */
    if *(*map).bits.offset(endint as isize) & endbits != desiredbits & endbits
       {
        return 0i32
    }
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn set_bit_range(mut map: *mut zone_bitmap,
                                       mut startbit: libc::c_int,
                                       mut endbit: libc::c_int) {
    let mut startint: libc::c_int = 0;
    let mut endint: libc::c_int = 0;
    let mut startbits: libc::c_uint = 0;
    let mut endbits: libc::c_uint = 0;
    startint = startbit / 32i32;
    endint = endbit / 32i32;
    startbit = startbit & 31i32;
    endbit = endbit & 31i32;
    startbits = !((1i32 << startbit) - 1i32) as libc::c_uint;
    if endbit == 31i32 {
        endbits = !0i32 as libc::c_uint
    } else { endbits = ((1i32 << endbit + 1i32) - 1i32) as libc::c_uint }
    /* Easy case, they are the same int, so set the range between */
    if startint == endint {
        let ref mut fresh0 = *(*map).bits.offset(startint as isize);
        *fresh0 |= startbits & endbits;
        return
    }
    /* Set the bits in the first int */
    let ref mut fresh1 = *(*map).bits.offset(startint as isize);
    *fresh1 |= startbits;
    /* Set all the ints inbetween */
    loop  {
        startint += 1;
        if !(startint < endint) { break ; }
        *(*map).bits.offset(startint as isize) = !0i32 as uint32_t
    }
    /* Set the bits in the last int */
    let ref mut fresh2 = *(*map).bits.offset(endint as isize);
    *fresh2 |= endbits;
}
#[no_mangle]
pub unsafe extern "C" fn clear_bit_range(mut map: *mut zone_bitmap,
                                         mut startbit: libc::c_int,
                                         mut endbit: libc::c_int) {
    let mut startint: libc::c_int = 0;
    let mut endint: libc::c_int = 0;
    let mut startbits: libc::c_uint = 0;
    let mut endbits: libc::c_uint = 0;
    startint = startbit / 32i32;
    endint = endbit / 32i32;
    startbit = startbit & 31i32;
    endbit = endbit & 31i32;
    startbits = !((1i32 << startbit) - 1i32) as libc::c_uint;
    if endbit == 31i32 {
        endbits = !0i32 as libc::c_uint
    } else { endbits = ((1i32 << endbit + 1i32) - 1i32) as libc::c_uint }
    /* Easy case, they are the same int, so set the range between */
    if startint == endint {
        let ref mut fresh3 = *(*map).bits.offset(startint as isize);
        *fresh3 &= !(startbits & endbits);
        return
    }
    /* Set the bits in the first int */
    let ref mut fresh4 = *(*map).bits.offset(startint as isize);
    *fresh4 &= !startbits;
    /* Set all the ints inbetween */
    loop  {
        startint += 1;
        if !(startint < endint) { break ; }
        *(*map).bits.offset(startint as isize) = 0i32 as uint32_t
    }
    /* Set the bits in the last int */
    let ref mut fresh5 = *(*map).bits.offset(endint as isize);
    *fresh5 &= !endbits;
}
#[no_mangle]
pub unsafe extern "C" fn scan_zone_maps(mut mfs: *mut mfs_handle,
                                        mut ckbitmaps:
                                            *mut *mut zone_bitmap) {
    let mut nextsector: uint64_t = 0;
    let mut sector: uint64_t = 0;
    let mut nextsbackup: uint64_t = 0;
    let mut sbackup: uint64_t = 0;
    let mut nextlength: uint32_t = 0;
    let mut length: uint32_t = 0;
    let mut nextsize: uint64_t = 0;
    let mut size: uint64_t = 0;
    let mut nextblocksize: uint32_t = 0;
    let mut blocksize: uint32_t = 0;
    let mut first: uint64_t = 0;
    let mut last: uint64_t = 0;
    let mut free_0: uint32_t = 0;
    let mut foundfree: uint32_t = 0;
    let mut vol_set_size: uint64_t = mfsvol_volume_set_size((*mfs).vols);
    let mut fsmem_ptrs: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut numbitmaps: libc::c_int = 0;
    let mut curzone: *mut zone_header = 0 as *mut zone_header;
    let mut zoneno: libc::c_int = -1i32;
    let mut loop_0: libc::c_int = 0;
    let mut bitmaploop: *mut *mut zone_bitmap = 0 as *mut *mut zone_bitmap;
    *ckbitmaps = 0 as *mut zone_bitmap;
    let mut totalfree: uint64_t = 0i32 as uint64_t;
    let mut totalbits: libc::c_int = 0i32;
    /* Find the first zone pointer */
    if 0 != (*mfs).is_64 {
        let mut vol_hdr: *mut volume_header_64 = &mut (*mfs).vol_hdr.v64;
        nextsector = intswap64((*vol_hdr).zonemap.sector);
        nextsbackup = intswap64((*vol_hdr).zonemap.sbackup);
        nextlength = intswap64((*vol_hdr).zonemap.length) as uint32_t;
        nextsize = intswap64((*vol_hdr).zonemap.size);
        nextblocksize = intswap64((*vol_hdr).zonemap.min) as uint32_t
    } else {
        let mut vol_hdr_0: *mut volume_header_32 = &mut (*mfs).vol_hdr.v32;
        nextsector = intswap32((*vol_hdr_0).zonemap.sector) as uint64_t;
        nextsbackup = intswap32((*vol_hdr_0).zonemap.sbackup) as uint64_t;
        nextlength = intswap32((*vol_hdr_0).zonemap.length);
        nextsize = intswap32((*vol_hdr_0).zonemap.size) as uint64_t;
        nextblocksize = intswap32((*vol_hdr_0).zonemap.min)
    }
    loop  {
        curzone = mfs_next_zone(mfs, curzone);
        if curzone.is_null() { break ; }
        let mut type_0: libc::c_int = 0;
        zoneno += 1;
        if 0 != (*mfs).is_64 {
            sector = intswap64((*curzone).z64.sector);
            sbackup = intswap64((*curzone).z64.sbackup);
            length = intswap32((*curzone).z64.length);
            size = intswap64((*curzone).z64.size);
            blocksize = intswap32((*curzone).z64.min);
            first = intswap64((*curzone).z64.first);
            last = intswap64((*curzone).z64.last);
            free_0 = intswap64((*curzone).z64.free) as uint32_t;
            numbitmaps = intswap32((*curzone).z64.num) as libc::c_int;
            type_0 =
                intswap32((*curzone).z64.type_0 as uint32_t) as libc::c_int;
            fsmem_ptrs =
                (&mut (*curzone).z64 as *mut zone_header_64).offset(1isize) as
                    *mut libc::c_void as *mut libc::c_uint
        } else {
            sector = intswap32((*curzone).z32.sector) as uint64_t;
            sbackup = intswap32((*curzone).z32.sbackup) as uint64_t;
            length = intswap32((*curzone).z32.length);
            size = intswap32((*curzone).z32.size) as uint64_t;
            blocksize = intswap32((*curzone).z32.min);
            first = intswap32((*curzone).z32.first) as uint64_t;
            last = intswap32((*curzone).z32.last) as uint64_t;
            free_0 = intswap32((*curzone).z32.free);
            numbitmaps = intswap32((*curzone).z32.num) as libc::c_int;
            type_0 =
                intswap32((*curzone).z32.type_0 as uint32_t) as libc::c_int;
            fsmem_ptrs =
                (&mut (*curzone).z32 as *mut zone_header_32).offset(1isize) as
                    *mut libc::c_void as *mut libc::c_uint
        }
        /* Check the current zone against the previous zone's pointer */
        sector != nextsector;
        sbackup != nextsbackup;
        if length != nextlength {
            printf(b"Zone %d length (%d) mismatch to zone %d next length (%d)\n\x00"
                       as *const u8 as *const libc::c_char, zoneno, length,
                   zoneno - 1i32, nextlength);
        }
        size != nextsize;
        if blocksize != nextblocksize {
            printf(b"Zone %d block size (%d) mismatch to zone %d next block size (%d)\n\x00"
                       as *const u8 as *const libc::c_char, zoneno, blocksize,
                   zoneno - 1i32, nextblocksize);
        }
        if 0 != (*mfs).is_64 {
            nextsector = intswap64((*curzone).z64.next_sector);
            nextsbackup = intswap64((*curzone).z64.next_sbackup);
            nextlength = intswap32((*curzone).z64.next_length);
            nextsize = intswap64((*curzone).z64.next_size);
            nextblocksize = intswap32((*curzone).z64.next_min)
        } else {
            nextsector = intswap32((*curzone).z32.next.sector) as uint64_t;
            nextsbackup = intswap32((*curzone).z32.next.sbackup) as uint64_t;
            nextlength = intswap32((*curzone).z32.next.length);
            nextsize = intswap32((*curzone).z32.next.size) as uint64_t;
            nextblocksize = intswap32((*curzone).z32.next.min)
        }
        /* Check a few values for sanity */
        first > last;
        size !=
            last.wrapping_sub(first).wrapping_add(1i32 as libc::c_ulonglong);
        last >= vol_set_size;
        0 != size.wrapping_rem(blocksize as libc::c_ulonglong);
        /* Make sure this zone doesn't overlap with any others */
        loop_0 = 0i32;
        bitmaploop = ckbitmaps;
        while !(*bitmaploop).is_null() {
            first <= (**bitmaploop).last && last >= (**bitmaploop).first;
            bitmaploop = &mut (**bitmaploop).next;
            loop_0 += 1
        }
        size =
            (size as
                 libc::c_ulonglong).wrapping_div(blocksize as
                                                     libc::c_ulonglong) as
                uint64_t as uint64_t;
        *bitmaploop =
            calloc(::std::mem::size_of::<zone_bitmap>() as libc::c_ulong,
                   1i32 as libc::c_ulong) as *mut zone_bitmap;
        (**bitmaploop).bits =
            calloc(size.wrapping_add(32i32 as
                                         libc::c_ulonglong).wrapping_div(32i32
                                                                             as
                                                                             libc::c_ulonglong)
                       as libc::c_ulong, 4i32 as libc::c_ulong) as
                *mut uint32_t;
        (**bitmaploop).type_0 = type_0;
        (**bitmaploop).first = first;
        (**bitmaploop).last = last;
        (**bitmaploop).blocksize = blocksize;
        foundfree = 0i32 as uint32_t;
        loop_0 = 0i32;
        while loop_0 < numbitmaps {
            let mut nbits: libc::c_uint = 0;
            let mut nints: libc::c_uint = 0;
            let mut setbits: libc::c_uint = 0;
            let mut foundbits: libc::c_uint = 0;
            let mut curint: libc::c_uint = 0;
            let mut bitmaphdr: *mut bitmap_header =
                (&mut *fsmem_ptrs.offset(numbitmaps as isize) as
                     *mut libc::c_uint as
                     size_t).wrapping_add(intswap32(*fsmem_ptrs.offset(loop_0
                                                                           as
                                                                           isize))
                                              as
                                              libc::c_ulong).wrapping_sub(intswap32(*fsmem_ptrs.offset(0isize))
                                                                              as
                                                                              libc::c_ulong)
                    as *mut bitmap_header;
            let mut ints: *mut libc::c_uint =
                bitmaphdr.offset(1isize) as *mut libc::c_uint;
            /* Check to make sure it's not pointing off into random memory */
            if ints as size_t >=
                   (curzone as
                        size_t).wrapping_add(length.wrapping_mul(512i32 as
                                                                     libc::c_uint)
                                                 as libc::c_ulong) ||
                   (ints as
                        size_t).wrapping_add(intswap32((*bitmaphdr).nints).wrapping_mul(4i32
                                                                                            as
                                                                                            libc::c_uint)
                                                 as libc::c_ulong) >=
                       (curzone as
                            size_t).wrapping_add(length.wrapping_mul(512i32 as
                                                                         libc::c_uint)
                                                     as libc::c_ulong) {
                printf(b"Zone %d bitmap %d is beyond end of the zone map\n\x00"
                           as *const u8 as *const libc::c_char, zoneno,
                       loop_0);
            } else {
                nbits = intswap32((*bitmaphdr).nbits);
                nints = intswap32((*bitmaphdr).nints);
                setbits = intswap32((*bitmaphdr).freeblocks);
                foundbits = 0i32 as libc::c_uint;
                /* Sanity check the values in the bitmap header */
                if nbits.wrapping_add(31i32 as
                                          libc::c_uint).wrapping_div(32i32 as
                                                                         libc::c_uint)
                       != nints {
                    printf(b"Zone %d bitmap %d number of ints (%d) does not match number of bits (%d bits / %d ints)\n\x00"
                               as *const u8 as *const libc::c_char, zoneno,
                           loop_0, nints, nbits,
                           nbits.wrapping_add(31i32 as
                                                  libc::c_uint).wrapping_div(32i32
                                                                                 as
                                                                                 libc::c_uint));
                }
                (nbits as libc::c_ulonglong) < size;
                /* Scan for set bits on a coarse level */
                curint = 0i32 as libc::c_uint;
                while curint < nints {
                    let mut bits: libc::c_uint = 0;
                    let mut curbit: libc::c_uint = 0;
                    /* Just track the last bit in this int for reporting what can */
				/* be combined in the bitmap */
                    let mut lastbit: libc::c_uint = -1i32 as libc::c_uint;
                    /* If no bits found here, skip to the next int */
                    if !(0 == *ints.offset(curint as isize)) {
                        /* Scan this int on a fine level */
                        bits = intswap32(*ints.offset(curint as isize));
                        curbit = 0i32 as libc::c_uint;
                        while curbit < 32i32 as libc::c_uint && 0 != bits {
                            if 0 !=
                                   bits &
                                       (1i32 <<
                                            (31i32 as
                                                 libc::c_uint).wrapping_sub(curbit))
                                           as libc::c_uint {
                                let mut bitno: libc::c_int =
                                    curint.wrapping_mul(32i32 as
                                                            libc::c_uint).wrapping_add(curbit)
                                        as libc::c_int;
                                0 != curbit & 1i32 as libc::c_uint &&
                                    lastbit ==
                                        curbit.wrapping_sub(1i32 as
                                                                libc::c_uint);
                                lastbit = curbit;
                                /* Clear it so the loop can break out early */
                                bits &=
                                    !(1i32 <<
                                          (31i32 as
                                               libc::c_uint).wrapping_sub(curbit))
                                        as libc::c_uint;
                                /* Make sure it is within the bitmap */
                                if !(bitno as libc::c_ulonglong >= size) {
                                    /* Make sure the bit wasn't already set */
                                    0 ==
                                        scan_bit_range(*bitmaploop,
                                                       bitno << loop_0,
                                                       (bitno + 1i32 <<
                                                            loop_0) - 1i32,
                                                       0i32);
                                    /* Track this bitmap's bit */
                                    foundbits = foundbits.wrapping_add(1);
                                    set_bit_range(*bitmaploop,
                                                  bitno << loop_0,
                                                  (bitno + 1i32 << loop_0) -
                                                      1i32);
                                }
                            }
                            curbit = curbit.wrapping_add(1)
                        }
                    }
                    curint = curint.wrapping_add(1)
                }
                if foundbits != setbits {
                    printf(b"Zone %d bitmap %d bits marked available (%d) mismatch against bitmap header (%d)\n\x00"
                               as *const u8 as *const libc::c_char, zoneno,
                           loop_0, foundbits, setbits);
                }
                foundfree =
                    (foundfree as
                         libc::c_uint).wrapping_add(foundbits.wrapping_mul(blocksize))
                        as uint32_t as uint32_t;
                totalbits =
                    (totalbits as libc::c_uint).wrapping_add(foundbits) as
                        libc::c_int as libc::c_int
            }
            loop_0 += 1;
            size =
                (size as
                     libc::c_ulonglong).wrapping_div(2i32 as
                                                         libc::c_ulonglong) as
                    uint64_t as uint64_t;
            blocksize =
                (blocksize as libc::c_uint).wrapping_mul(2i32 as libc::c_uint)
                    as uint32_t as uint32_t
        }
        if free_0 != foundfree {
            printf(b"Zone %d free space (%d) does not match header (%d)\n\x00"
                       as *const u8 as *const libc::c_char, zoneno, foundfree,
                   free_0);
        }
        totalfree =
            (totalfree as
                 libc::c_ulonglong).wrapping_add(foundfree as
                                                     libc::c_ulonglong) as
                uint64_t as uint64_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn set_fsid_range(mut bitmap: *mut zone_bitmap,
                                        mut startbit: libc::c_int,
                                        mut endbit: libc::c_int,
                                        mut fsid: libc::c_uint) {
    if (*bitmap).fsids.is_null() {
        (*bitmap).fsids =
            calloc(4i32 as libc::c_ulong,
                   (*bitmap).last.wrapping_add(1i32 as
                                                   libc::c_ulonglong).wrapping_sub((*bitmap).first).wrapping_div((*bitmap).blocksize
                                                                                                                     as
                                                                                                                     libc::c_ulonglong)
                       as libc::c_ulong) as *mut uint32_t
    }
    while startbit <= endbit {
        *(*bitmap).fsids.offset(startbit as isize) = fsid;
        startbit += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn scan_inode_overlap(mut bitmap: *mut zone_bitmap,
                                            mut curinode: libc::c_uint,
                                            mut inode: *mut mfs_inode,
                                            mut bitno: libc::c_int,
                                            mut bitcount: libc::c_int) {
    let mut rangestart: libc::c_int = -1i32;
    let mut rangefsid: libc::c_int = 0i32;
    while bitcount > 0i32 {
        let mut newfsid: libc::c_int = 0i32;
        let mut isclear: libc::c_int =
            scan_bit_range(bitmap, bitno, bitno, 0i32);
        if !(*bitmap).fsids.is_null() {
            newfsid = *(*bitmap).fsids.offset(bitno as isize) as libc::c_int
        }
        if newfsid != rangefsid && (rangefsid != 0i32 || rangestart >= 0i32)
               || rangestart >= 0i32 && 0 != isclear {
            rangefsid > 0i32;
            if 0 != isclear {
                rangestart = -1i32;
                rangefsid = 0i32
            } else { rangestart = bitno; rangefsid = newfsid }
        } else if newfsid != rangefsid {
            if 0 != isclear {
                rangestart = -1i32;
                rangefsid = -1i32
            } else { rangestart = bitno; rangefsid = newfsid }
        }
        bitcount -= 1;
        bitno += 1
    }
    rangefsid > 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn scan_inodes(mut mfs: *mut mfs_handle,
                                     mut bitmaps: *mut zone_bitmap) {
    let mut curinode: libc::c_int = 0i32;
    let mut maxinode: libc::c_int = mfs_inode_count(mfs) as libc::c_int;
    let mut loop_0: libc::c_int = 0;
    let mut nchained: libc::c_int = 0i32;
    let mut chainlength: libc::c_int = 0i32;
    let mut maxchainlength: libc::c_int = 0i32;
    let mut extrachained: libc::c_int = 0i32;
    let mut needchained: libc::c_int = 0i32;
    let mut allocinode: libc::c_int = 0i32;
    let mut maxblocks: libc::c_int = 0i32;
    let mut inode: *mut mfs_inode = 0 as *mut mfs_inode;
    // Bit 1 = chain needed, bit 2 = chain set
    let mut chained_inodes: *mut libc::c_uchar =
        calloc(1i32 as libc::c_ulong, maxinode as libc::c_ulong) as
            *mut libc::c_uchar;
    if 0 != (*mfs).is_64 {
        maxblocks =
            (512i32 as
                 libc::c_ulong).wrapping_sub(::std::mem::size_of::<mfs_inode>()
                                                 as
                                                 libc::c_ulong).wrapping_div(::std::mem::size_of::<C2RustUnnamed_4>()
                                                                                 as
                                                                                 libc::c_ulong)
                as libc::c_int
    } else {
        maxblocks =
            (512i32 as
                 libc::c_ulong).wrapping_sub(::std::mem::size_of::<mfs_inode>()
                                                 as
                                                 libc::c_ulong).wrapping_div(::std::mem::size_of::<C2RustUnnamed_5>()
                                                                                 as
                                                                                 libc::c_ulong)
                as libc::c_int
    }
    curinode = 0i32;
    while curinode < maxinode {
        inode = mfs_read_inode(mfs, curinode as uint32_t);
        if inode.is_null() {
            if 0 != mfs_has_error(mfs) {
                let mut msg: [libc::c_char; 1024] = [0; 1024];
                mfs_strerror(mfs, msg.as_mut_ptr());
                printf(b"Error reading inode %d: %s\n\x00" as *const u8 as
                           *const libc::c_char, curinode, msg.as_mut_ptr());
                mfs_clearerror(mfs);
            } else {
                printf(b"Error reading inode %d: Unknown\n\x00" as *const u8
                           as *const libc::c_char, curinode);
            }
        } else {
            match intswap32((*inode).sig) {
                2434997948 => {
                    if 0 != (*mfs).is_64 {
                        printf(b"Inode %d claims to be 32 bit in 64 bit volume\n\x00"
                                   as *const u8 as *const libc::c_char,
                               curinode);
                    }
                }
                3508739772 => {
                    if 0 == (*mfs).is_64 {
                        printf(b"Inode %d claims to be 64 bit in 32 bit volume\n\x00"
                                   as *const u8 as *const libc::c_char,
                               curinode);
                    }
                }
                _ => {
                    printf(b"Inode %d unknown signature %08x\n\x00" as
                               *const u8 as *const libc::c_char, curinode,
                           intswap32((*inode).sig));
                }
            }
            /* Mark if this inode is chained */
            if 0 != (*inode).inode_flags & intswap32(0x80000000u32) {
                let ref mut fresh6 =
                    *chained_inodes.offset(curinode as isize);
                *fresh6 = (*fresh6 as libc::c_int | 2i32) as libc::c_uchar
            }
            if 0 != (*inode).fsid {
                let mut curchainlength: libc::c_int = 0i32;
                let mut expectedzonetype: libc::c_int = 0;
                allocinode += 1;
                /* Mark if this fsid needs any inodes before it chained */
                loop_0 =
                    intswap32((*inode).fsid).wrapping_mul(0x106d9i32 as
                                                              libc::c_uint).wrapping_rem(maxinode
                                                                                             as
                                                                                             libc::c_uint)
                        as libc::c_int;
                while loop_0 != curinode {
                    let ref mut fresh7 =
                        *chained_inodes.offset(loop_0 as isize);
                    *fresh7 =
                        (*fresh7 as libc::c_int | 1i32) as libc::c_uchar;
                    curchainlength += 1;
                    loop_0 = (loop_0 + 1i32) % maxinode
                }
                /* Track statistics on chained inodes */
                if curchainlength > 0i32 {
                    nchained += 1;
                    chainlength += curchainlength;
                    if curchainlength > maxchainlength {
                        maxchainlength = curchainlength
                    }
                }
                if intswap32((*inode).inode) != curinode as libc::c_uint {
                    printf(b"Inode %d fsid %d inode number mismatch with data %d\n\x00"
                               as *const u8 as *const libc::c_char, curinode,
                           intswap32((*inode).fsid),
                           intswap32((*inode).inode));
                }
                if 0 == (*inode).refcount {
                    printf(b"Inode %d fsid %d has zero reference count\n\x00"
                               as *const u8 as *const libc::c_char, curinode,
                           intswap32((*inode).fsid));
                }
                let mut current_block_61: u64;
                match (*inode).type_0 as libc::c_int {
                    2 => {
                        if (*inode).blocksize != (*inode).unk3 {
                            printf(b"Inode %d fsid %d stream total block blocksize %d mismatch used block blocksize %d\n\x00"
                                       as *const u8 as *const libc::c_char,
                                   curinode, intswap32((*inode).fsid),
                                   intswap32((*inode).unk3),
                                   intswap32((*inode).blocksize));
                        }
                        if intswap32((*inode).size) <
                               intswap32((*inode).blockused) {
                            printf(b"Inode %d fsid %d stream total block count %d less than used block count %d\n\x00"
                                       as *const u8 as *const libc::c_char,
                                   curinode, intswap32((*inode).fsid),
                                   intswap32((*inode).size),
                                   intswap32((*inode).blockused));
                        }
                        expectedzonetype = ztMedia as libc::c_int;
                        if (*inode).zone as libc::c_int != 1i32 {
                            printf(b"Inode %d fsid %d marked for data type %d (Expect 1)\n\x00"
                                       as *const u8 as *const libc::c_char,
                                   curinode, intswap32((*inode).fsid),
                                   (*inode).zone as libc::c_int);
                        }
                        current_block_61 = 11796148217846552555;
                    }
                    1 | 4 | 8 => { current_block_61 = 12127865715653409884; }
                    _ => {
                        printf(b"Inode %d fsid %d unknown type %d\n\x00" as
                                   *const u8 as *const libc::c_char, curinode,
                               intswap32((*inode).fsid),
                               (*inode).type_0 as libc::c_int);
                        /* Intentionally fall through */
                        current_block_61 = 12127865715653409884;
                    }
                }
                match current_block_61 {
                    12127865715653409884 => {
                        if 0 != (*inode).blocksize || 0 != (*inode).blockused
                               || 0 != (*inode).unk3 {
                            printf(b"Inode %d fsid %d non-stream inode defines stream block sizes\n\x00"
                                       as *const u8 as *const libc::c_char,
                                   curinode, intswap32((*inode).fsid));
                        }
                        expectedzonetype = ztApplication as libc::c_int;
                        if (*inode).zone as libc::c_int != 2i32 {
                            printf(b"Inode %d fsid %d marked for data type %d (Expect 2)\n\x00"
                                       as *const u8 as *const libc::c_char,
                                   curinode, intswap32((*inode).fsid),
                                   (*inode).zone as libc::c_int);
                        }
                    }
                    _ => { }
                }
                if 0 !=
                       (*inode).inode_flags &
                           intswap32(0x40000000i32 as uint32_t) ||
                       0 !=
                           (*inode).inode_flags &
                               intswap32(0x2i32 as uint32_t) {
                    if 0 != (*inode).numblocks {
                        printf(b"Inode %d fsid %d has data in inode block and non-zero extent count %d\n\x00"
                                   as *const u8 as *const libc::c_char,
                               curinode, intswap32((*inode).fsid),
                               intswap32((*inode).numblocks));
                    }
                    if (intswap32((*inode).size) as
                            libc::c_ulong).wrapping_add(::std::mem::size_of::<mfs_inode>()
                                                            as libc::c_ulong)
                           > 512i32 as libc::c_ulong {
                        printf(b"Inode %d fsid %d has data in inode block but size %d greather than max allowed %d\n\x00"
                                   as *const u8 as *const libc::c_char,
                               curinode, intswap32((*inode).fsid),
                               intswap32((*inode).size),
                               (512i32 as
                                    libc::c_ulong).wrapping_sub(::std::mem::size_of::<mfs_inode>()
                                                                    as
                                                                    libc::c_ulong)
                                   as libc::c_int);
                    }
                    if (*inode).type_0 as libc::c_int ==
                           tyStream as libc::c_int {
                        printf(b"Inode %d fsid %d has data in inode block with tyStream data type\n\x00"
                                   as *const u8 as *const libc::c_char,
                               curinode, intswap32((*inode).fsid));
                    }
                } else if intswap32((*inode).numblocks) >
                              maxblocks as libc::c_uint {
                    printf(b"Inode %d fsid %d has more extents (%d) than max (%d)\n\x00"
                               as *const u8 as *const libc::c_char, curinode,
                           intswap32((*inode).fsid),
                           intswap32((*inode).numblocks), maxblocks);
                } else {
                    let mut totalsize: uint64_t = 0i32 as uint64_t;
                    loop_0 = 0i32;
                    while (loop_0 as libc::c_uint) <
                              intswap32((*inode).numblocks) {
                        let mut sector: uint64_t = 0;
                        let mut count: uint32_t = 0;
                        let mut bitno: libc::c_int = 0;
                        let mut bitcount: libc::c_int = 0;
                        if 0 != (*mfs).is_64 {
                            sector =
                                sectorswap64((*inode).datablocks.d64[loop_0 as
                                                                         usize].sector);
                            count =
                                intswap32((*inode).datablocks.d64[loop_0 as
                                                                      usize].count)
                        } else {
                            sector =
                                intswap32((*inode).datablocks.d32[loop_0 as
                                                                      usize].sector)
                                    as uint64_t;
                            count =
                                intswap32((*inode).datablocks.d32[loop_0 as
                                                                      usize].count)
                        }
                        totalsize =
                            (totalsize as
                                 libc::c_ulonglong).wrapping_add(count as
                                                                     libc::c_ulonglong)
                                as uint64_t as uint64_t;
                        let mut bitmapforblock: *mut zone_bitmap = bitmaps;
                        while (*bitmapforblock).first > sector ||
                                  (*bitmapforblock).last < sector {
                            bitmapforblock = (*bitmapforblock).next
                        }
                        if !bitmapforblock.is_null() {
                            if expectedzonetype != (*bitmapforblock).type_0 {
                                printf(b"Inode %d fsid %d expected zone type %d but extent %d is in type %d\n\x00"
                                           as *const u8 as
                                           *const libc::c_char, curinode,
                                       intswap32((*inode).fsid),
                                       expectedzonetype, loop_0,
                                       (*bitmapforblock).type_0);
                            }
                            if !(0 !=
                                     sector.wrapping_sub((*bitmapforblock).first).wrapping_rem((*bitmapforblock).blocksize
                                                                                                   as
                                                                                                   libc::c_ulonglong))
                               {
                                if !(0 !=
                                         count.wrapping_rem((*bitmapforblock).blocksize))
                                   {
                                    /* Make sure the range isn't marked already */
                                    bitno =
                                        sector.wrapping_sub((*bitmapforblock).first).wrapping_div((*bitmapforblock).blocksize
                                                                                                      as
                                                                                                      libc::c_ulonglong)
                                            as libc::c_int;
                                    bitcount =
                                        count.wrapping_div((*bitmapforblock).blocksize)
                                            as libc::c_int;
                                    if 0 ==
                                           scan_bit_range(bitmapforblock,
                                                          bitno,
                                                          bitno + bitcount -
                                                              1i32, 0i32) {
                                        scan_inode_overlap(bitmapforblock,
                                                           curinode as
                                                               libc::c_uint,
                                                           inode, bitno,
                                                           bitcount);
                                    }
                                    set_bit_range(bitmapforblock, bitno,
                                                  bitno + bitcount - 1i32);
                                    set_fsid_range(bitmapforblock, bitno,
                                                   bitno + bitcount - 1i32,
                                                   intswap32((*inode).fsid));
                                }
                            }
                        }
                        loop_0 += 1
                    }
                    if (*inode).type_0 as libc::c_int ==
                           tyStream as libc::c_int {
                        totalsize.wrapping_mul(512i32 as libc::c_ulonglong) <
                            intswap32((*inode).size).wrapping_mul(intswap32((*inode).unk3))
                                as libc::c_ulonglong;
                    } else {
                        totalsize.wrapping_mul(512i32 as libc::c_ulonglong) <
                            intswap32((*inode).size) as libc::c_ulonglong;
                    }
                }
            } else {
                if 0 != (*inode).refcount {
                    printf(b"Inode %d has %d references and no fsid\n\x00" as
                               *const u8 as *const libc::c_char, curinode,
                           intswap32((*inode).refcount));
                }
                if 0 != (*inode).numblocks {
                    printf(b"Inode %d free but has datablocks allocated to it\n\x00"
                               as *const u8 as *const libc::c_char, curinode);
                }
            }
            free(inode);
        }
        curinode += 1
    }
    curinode = 0i32;
    while curinode < maxinode {
        /* Bit 1 = chain needed, bit 2 = chain set */
        match *chained_inodes.offset(curinode as isize) as libc::c_int {
            1 => {
                printf(b"Inode %d requires chained flag, but not set\n\x00" as
                           *const u8 as *const libc::c_char, curinode);
                needchained += 1
            }
            2 => { extrachained += 1 }
            _ => { }
        }
        curinode += 1
    }
    printf(b"%d/%d inodes used\n\x00" as *const u8 as *const libc::c_char,
           allocinode, maxinode);
    if 0 != nchained {
        printf(b"%d fsids in chained inodes, %d max inode chain length, %d average length\n\x00"
                   as *const u8 as *const libc::c_char, nchained,
               maxchainlength, (chainlength + nchained / 2i32) / nchained);
    }
    if 0 != extrachained || 0 != needchained {
        printf(b"%d inodes unnecessarily chained, %d not chained need to be\n\x00"
                   as *const u8 as *const libc::c_char, extrachained,
               needchained);
    }
    free(chained_inodes);
}
#[no_mangle]
pub unsafe extern "C" fn scan_unclaimed_blocks(mut mfs: *mut mfs_handle,
                                               mut bitmap: *mut zone_bitmap) {
    while !bitmap.is_null() {
        let mut nbits: libc::c_int =
            (*bitmap).last.wrapping_add(1i32 as
                                            libc::c_ulonglong).wrapping_sub((*bitmap).first).wrapping_div((*bitmap).blocksize
                                                                                                              as
                                                                                                              libc::c_ulonglong)
                as libc::c_int;
        let mut nints: libc::c_int = (nbits + 31i32) / 32i32;
        let mut loop_0: libc::c_int = 0;
        let mut startunclaimed: libc::c_int = -1i32;
        loop_0 = 0i32;
        while loop_0 < nints {
            if *(*bitmap).bits.offset(loop_0 as isize) !=
                   !0i32 as libc::c_uint {
                let mut loop2: libc::c_int = 0;
                loop2 = 0i32;
                while loop2 < 32i32 && loop2 + loop_0 * 32i32 < nbits {
                    if 0 ==
                           *(*bitmap).bits.offset(loop_0 as isize) &
                               (1i32 << loop2) as libc::c_uint {
                        if startunclaimed < 0i32 {
                            startunclaimed = loop2 + loop_0 * 32i32
                        }
                    } else if startunclaimed >= 0i32 {
                        startunclaimed = -1i32
                    }
                    loop2 += 1
                }
            } else if startunclaimed >= 0i32 { startunclaimed = -1i32 }
            loop_0 += 1
        }
        if startunclaimed >= 0i32 { startunclaimed = -1i32 }
        bitmap = (*bitmap).next
    };
}