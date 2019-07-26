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
    pub type tivo_partition_file;
    #[no_mangle]
    fn mfs_read_inode_data_part(mfshnd: *mut mfs_handle,
                                inode: *mut mfs_inode,
                                data: *mut libc::c_uchar, start: uint64_t,
                                count: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn mfs_has_error(mfshnd: *mut mfs_handle) -> libc::c_int;
    #[no_mangle]
    fn mfs_read_inode_by_fsid(mfshnd: *mut mfs_handle, fsid: uint32_t)
     -> *mut mfs_inode;
    #[no_mangle]
    fn mfs_read_inode(mfshnd: *mut mfs_handle, inode: uint32_t)
     -> *mut mfs_inode;
    // Historically, the drive was accessed as big endian (MSB), however newer platforms (Roamio) are mipsel based, hence the numeric values are little endian (LSB).
    /* Drive is little endian */
    #[no_mangle]
    static mut mfsLSB: libc::c_int;
    #[no_mangle]
    fn mfs_check_crc(data: *mut libc::c_uchar, size: libc::c_uint,
                     off: libc::c_uint) -> libc::c_uint;
    #[no_mangle]
    fn mfs_perror(mfshnd: *mut mfs_handle, str: *mut libc::c_char);
    #[no_mangle]
    fn mfsvol_read_data(hnd: *mut volume_handle, buf: *mut libc::c_void,
                        sector: uint64_t, count: uint32_t) -> libc::c_int;
    #[no_mangle]
    fn mfsvol_enable_memwrite(hnd: *mut volume_handle);
    #[no_mangle]
    fn mfs_inode_count(mfshnd: *mut mfs_handle) -> uint32_t;
    /* Size of each bitmap is (nints + (nbits < 8? 1: 2)) * 4 */
/* Don't ask why, thats just the way it is. */
/* In bitmap, MSB is first, LSB last */
    #[no_mangle]
    fn mfs_next_zone(mfshdn: *mut mfs_handle, cur: *mut zone_header)
     -> *mut zone_header;
    #[no_mangle]
    fn mfs_log_read(mfshnd: *mut mfs_handle, buf: *mut libc::c_void,
                    logstamp: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn mfs_log_fssync(mfshnd: *mut mfs_handle) -> libc::c_int;
    #[no_mangle]
    fn mfs_log_stamp_to_sector(mfshnd: *mut mfs_handle,
                               logstamp: libc::c_uint) -> uint64_t;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn isprint(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
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
/* Prime number used in hash for finding base inode of fsid. */
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
pub type log_hdr = log_hdr_s;
#[derive ( Copy , Clone )]
#[repr(C, packed)]
pub struct log_entry_s {
    pub length: libc::c_ushort,
    pub unk1: libc::c_uint,
    pub bootcycles: libc::c_uint,
    pub bootsecs: libc::c_uint,
    pub fsid: libc::c_uint,
    pub transtype: libc::c_uint,
    pub unk2: libc::c_uint,
}
pub type log_entry = log_entry_s;
#[derive ( Copy , Clone )]
#[repr(C, packed)]
pub struct log_map_update_32_s {
    pub log: log_entry,
    pub remove: libc::c_uint,
    pub sector: libc::c_uint,
    pub size: libc::c_uint,
    pub unk: libc::c_uint,
}
pub type log_map_update_32 = log_map_update_32_s;
#[derive ( Copy , Clone )]
#[repr(C, packed)]
pub struct log_map_update_64_s {
    pub log: log_entry,
    pub remove: libc::c_uint,
    pub pad: libc::c_uint,
    pub sector: uint64_t,
    pub size: uint64_t,
    pub flag: libc::c_uchar,
    pub pad2: libc::c_uchar,
    pub pad3: libc::c_ushort,
    pub pad4: libc::c_uint,
}
pub type log_map_update_64 = log_map_update_64_s;
#[derive ( Copy , Clone )]
#[repr(C, packed)]
pub struct log_inode_update_s {
    pub log: log_entry,
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
    pub inodedata: libc::c_uint,
    pub datasize: libc::c_uint,
    pub datablocks: C2RustUnnamed_2,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed_2 {
    pub d32: [C2RustUnnamed_4; 0],
    pub d64: [C2RustUnnamed_3; 0],
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub sector: uint64_t,
    pub count: uint32_t,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub sector: libc::c_uint,
    pub count: libc::c_uint,
}
pub type log_inode_update = log_inode_update_s;
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union log_entry_all_u {
    pub log: log_entry,
    pub zonemap_32: log_map_update_32,
    pub zonemap_64: log_map_update_64,
    pub inode: log_inode_update,
}
pub type log_entry_all = log_entry_all_u;
pub type log_trans_types_e = libc::c_uint;
pub const ltInodeUpdate2: log_trans_types_e = 8;
pub const ltMapUpdate64: log_trans_types_e = 7;
/* ? = 6 */
pub const ltUnknownType6: log_trans_types_e = 6;
pub const ltLogReplay: log_trans_types_e = 5;
/* Rollback = 3? */
pub const ltFsSync: log_trans_types_e = 4;
pub const ltCommit: log_trans_types_e = 2;
pub const ltInodeUpdate: log_trans_types_e = 1;
pub const ltMapUpdate: log_trans_types_e = 0;
pub const Hex: C2RustUnnamed_5 = 1;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const Bin: C2RustUnnamed_5 = 2;
pub const None_0: C2RustUnnamed_5 = 0;
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
unsafe extern "C" fn intswap64(mut n: uint64_t) -> uint64_t {
    if mfsLSB == 0i32 { return n }
    return Endian64_Swap(n);
}
static mut progname: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut mfs: *mut mfs_handle = 0 as *const mfs_handle as *mut mfs_handle;
static mut hexvals: libc::c_int = 0i32;
unsafe extern "C" fn usage() { }
unsafe extern "C" fn hexdump(mut buf: *mut libc::c_uchar,
                             mut sector: libc::c_uint,
                             mut size: libc::c_uint) {
    let mut ofs: libc::c_int = 0;
    ofs = 0i32;
    while ofs < 512i32 && size > 0i32 as libc::c_uint {
        let mut line: [libc::c_uchar; 20] = [0; 20];
        let mut myo: libc::c_int = 0;
        if sector == 0xdeadbeefu32 {
            printf(b"%03x \x00" as *const u8 as *const libc::c_char, ofs);
        } else if sector == 0xffffffffu32 {
            printf(b"\t\x00" as *const u8 as *const libc::c_char);
        } else {
            printf(b"%08x:%03x \x00" as *const u8 as *const libc::c_char,
                   sector, ofs);
        }
        myo = 0i32;
        while myo < 16i32 {
            let fresh0 = size;
            size = size.wrapping_sub(1);
            if 0 != fresh0 {
                printf(b"%02x%c\x00" as *const u8 as *const libc::c_char,
                       *buf.offset((myo + ofs) as isize) as libc::c_int,
                       if myo < 15i32 && myo & 3i32 == 3i32 {
                           '-' as i32
                       } else { ' ' as i32 });
                line[myo as usize] =
                    (if 0 !=
                            isprint(*buf.offset((myo + ofs) as isize) as
                                        libc::c_int) {
                         *buf.offset((myo + ofs) as isize) as libc::c_int
                     } else { '.' as i32 }) as libc::c_uchar
            } else {
                line[myo as usize] = '|' as i32 as libc::c_uchar;
                line[(myo + 1i32) as usize] = '\n' as i32 as libc::c_uchar;
                line[(myo + 2i32) as usize] = 0i32 as libc::c_uchar;
                loop  {
                    printf(b"  %c\x00" as *const u8 as *const libc::c_char,
                           if myo < 15i32 && myo & 3i32 == 3i32 {
                               '-' as i32
                           } else { ' ' as i32 });
                    myo += 1;
                    if !(myo < 16i32) { break ; }
                }
                printf(b"|%s\x00" as *const u8 as *const libc::c_char,
                       line.as_mut_ptr());
                return
            }
            myo += 1
        }
        printf(b"|\x00" as *const u8 as *const libc::c_char);
        line[16usize] = '|' as i32 as libc::c_uchar;
        line[17usize] = '\n' as i32 as libc::c_uchar;
        line[18usize] = 0i32 as libc::c_uchar;
        printf(b"%s\x00" as *const u8 as *const libc::c_char,
               line.as_mut_ptr());
        ofs += 16i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn dump_inode_log(mut entry: *mut log_inode_update)
 -> libc::c_int {
    let mut date: [libc::c_char; 17] =
        *::std::mem::transmute::<&[u8; 17],
                                 &mut [libc::c_char; 17]>(b"xx/xx/xx xx:xx\x00\x00\x00");
    printf(b"Inode: %-13xFSid: %x\n\x00" as *const u8 as *const libc::c_char,
           intswap32((*entry).inode), intswap32((*entry).fsid));
    printf(b"Refcount: %-10xType: \x00" as *const u8 as *const libc::c_char,
           intswap32((*entry).refcount));
    match (*entry).type_0 as libc::c_int {
        4 => { printf(b"tyDir\n\x00" as *const u8 as *const libc::c_char); }
        8 => { printf(b"tyDb\n\x00" as *const u8 as *const libc::c_char); }
        2 => {
            printf(b"tyStream\n\x00" as *const u8 as *const libc::c_char);
        }
        1 => { printf(b"tyFile\n\x00" as *const u8 as *const libc::c_char); }
        _ => {
            printf(b"??? (%d)\n\x00" as *const u8 as *const libc::c_char,
                   (*entry).type_0 as libc::c_int);
        }
    }
    printf(b"Last modified: %s\n\x00" as *const u8 as *const libc::c_char,
           date.as_mut_ptr());
    printf(b"Last update boot: %-15uSecs: %u\n\x00" as *const u8 as
               *const libc::c_char, intswap32((*entry).bootcycles),
           intswap32((*entry).bootsecs));
    if (*entry).type_0 as libc::c_int == tyStream as libc::c_int {
        0 != hexvals;
    } else if 0 != hexvals {
        printf(b"Size: %x bytes\n\x00" as *const u8 as *const libc::c_char,
               intswap32((*entry).size));
    } else {
        printf(b"Size: %u bytes\n\x00" as *const u8 as *const libc::c_char,
               intswap32((*entry).size));
    }
    if 0 != (*entry).inodedata &&
           (*entry).inodedata != intswap32(1i32 as uint32_t) {
        printf(b"Data in inode: %u\n\x00" as *const u8 as *const libc::c_char,
               intswap32((*entry).inodedata));
    }
    if 0 == (*entry).inodedata {
        let mut loop_0: libc::c_int = 0;
        if 0 != (*mfs).is_64 {
            loop_0 = 0i32;
            while (loop_0 as libc::c_ulong) <
                      (intswap32((*entry).datasize) as
                           libc::c_ulong).wrapping_div(::std::mem::size_of::<C2RustUnnamed_3>()
                                                           as libc::c_ulong) {
                0 != hexvals;
                loop_0 += 1
            }
        } else {
            printf(b"Data is in %u blocks:\n\x00" as *const u8 as
                       *const libc::c_char,
                   (intswap32((*entry).datasize) as
                        libc::c_ulong).wrapping_div(::std::mem::size_of::<C2RustUnnamed_4>()
                                                        as libc::c_ulong) as
                       libc::c_uint);
            loop_0 = 0i32;
            while (loop_0 as libc::c_ulong) <
                      (intswap32((*entry).datasize) as
                           libc::c_ulong).wrapping_div(::std::mem::size_of::<C2RustUnnamed_4>()
                                                           as libc::c_ulong) {
                if 0 != hexvals {
                    printf(b"At %x %x sectors\n\x00" as *const u8 as
                               *const libc::c_char,
                           intswap32((*entry).datablocks.d32[loop_0 as
                                                                 usize].sector),
                           intswap32((*entry).datablocks.d32[loop_0 as
                                                                 usize].count));
                } else {
                    printf(b"At %u %u sectors\n\x00" as *const u8 as
                               *const libc::c_char,
                           intswap32((*entry).datablocks.d32[loop_0 as
                                                                 usize].sector),
                           intswap32((*entry).datablocks.d32[loop_0 as
                                                                 usize].count));
                }
                loop_0 += 1
            }
        }
    } else {
        printf(b"Data is in inode block.\n\x00" as *const u8 as
                   *const libc::c_char);
        hexdump(&mut *(*entry).datablocks.d32.as_mut_ptr().offset(0isize) as
                    *mut C2RustUnnamed_4 as *mut libc::c_void as
                    *mut libc::c_uchar, 0i32 as libc::c_uint,
                intswap32((*entry).datasize));
    }
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn dump_inode(mut inode_buf: *mut mfs_inode,
                                    mut buf: *mut libc::c_uchar,
                                    mut bufsize: libc::c_uint)
 -> libc::c_int {
    let mut date: [libc::c_char; 17] =
        *::std::mem::transmute::<&[u8; 17],
                                 &mut [libc::c_char; 17]>(b"xx/xx/xx xx:xx\x00\x00\x00");
    if inode_buf.is_null() && bufsize >= 512i32 as libc::c_uint {
        // If it wasn't read as an inode, check if it looks like one
        inode_buf = buf as *mut mfs_inode;
        if (*inode_buf).sig != intswap32(0x91231ebcu32) ||
               0 ==
                   mfs_check_crc(inode_buf as *mut libc::c_uchar,
                                 512i32 as libc::c_uint,
                                 (&mut (*inode_buf).checksum as
                                      *mut libc::c_uint).wrapping_offset_from(inode_buf
                                                                                  as
                                                                                  *mut libc::c_uint)
                                     as libc::c_long as libc::c_uint) {
            return 0i32
        }
    }
    loop  {
        printf(b"\n    Inode block\n\x00" as *const u8 as
                   *const libc::c_char);
        printf(b"Inode: %-13uFSid: %u\n\x00" as *const u8 as
                   *const libc::c_char, intswap32((*inode_buf).inode),
               intswap32((*inode_buf).fsid));
        printf(b"Refcount: %-10uType: \x00" as *const u8 as
                   *const libc::c_char, intswap32((*inode_buf).refcount));
        match (*inode_buf).type_0 as libc::c_int {
            4 => {
                printf(b"tyDir\n\x00" as *const u8 as *const libc::c_char);
            }
            8 => {
                printf(b"tyDb\n\x00" as *const u8 as *const libc::c_char);
            }
            2 => {
                printf(b"tyStream\n\x00" as *const u8 as *const libc::c_char);
            }
            1 => {
                printf(b"tyFile\n\x00" as *const u8 as *const libc::c_char);
            }
            _ => {
                printf(b"??? (%d)\n\x00" as *const u8 as *const libc::c_char,
                       (*inode_buf).type_0 as libc::c_int);
            }
        }
        printf(b"Last modified: %s\n\x00" as *const u8 as *const libc::c_char,
               date.as_mut_ptr());
        printf(b"Last update boot: %-15uSecs: %u\n\x00" as *const u8 as
                   *const libc::c_char, intswap32((*inode_buf).bootcycles),
               intswap32((*inode_buf).bootsecs));
        if (*inode_buf).type_0 as libc::c_int == tyStream as libc::c_int {
            0 != hexvals;
        } else if 0 != hexvals {
            printf(b"Size: %x bytes\n\x00" as *const u8 as
                       *const libc::c_char, intswap32((*inode_buf).size));
        } else {
            printf(b"Size: %u bytes\n\x00" as *const u8 as
                       *const libc::c_char, intswap32((*inode_buf).size));
        }
        printf(b"Checksum: %08x  Flags:\x00" as *const u8 as
                   *const libc::c_char, (*inode_buf).checksum);
        if 0 != (*inode_buf).inode_flags & intswap32(0x80000000u32) {
            printf(b" CHAINED\x00" as *const u8 as *const libc::c_char);
        }
        if 0 !=
               (*inode_buf).inode_flags & intswap32(0x40000000i32 as uint32_t)
           {
            printf(b" DATA\x00" as *const u8 as *const libc::c_char);
        }
        if 0 != (*inode_buf).inode_flags & intswap32(0x2i32 as uint32_t) {
            printf(b" DATA2\x00" as *const u8 as *const libc::c_char);
        }
        if 0 !=
               (*inode_buf).inode_flags &
                   intswap32(!((0x40000000i32 | 0x2i32) as libc::c_uint |
                                   0x80000000u32)) {
            printf(b" ? (%08x)\n\x00" as *const u8 as *const libc::c_char,
                   intswap32((*inode_buf).inode_flags));
        } else { printf(b"\n\x00" as *const u8 as *const libc::c_char); }
        printf(b"Sig: %08x (%d bit)\n\x00" as *const u8 as
                   *const libc::c_char, intswap32((*inode_buf).sig),
               if 0 != (*inode_buf).sig & intswap32(0x40000000i32 as uint32_t)
                  {
                   64i32
               } else { 32i32 });
        if 0 != intswap32((*inode_buf).numblocks) {
            let mut loop_0: libc::c_int = 0;
            if 0 != (*mfs).is_64 {
                printf(b"Data is in %u blocks:\n\x00" as *const u8 as
                           *const libc::c_char,
                       intswap32((*inode_buf).numblocks));
                loop_0 = 0i32;
                while (loop_0 as libc::c_uint) <
                          intswap32((*inode_buf).numblocks) {
                    0 != hexvals;
                    loop_0 += 1
                }
            } else {
                printf(b"Data is in %u blocks:\n\x00" as *const u8 as
                           *const libc::c_char,
                       intswap32((*inode_buf).numblocks));
                loop_0 = 0i32;
                while (loop_0 as libc::c_uint) <
                          intswap32((*inode_buf).numblocks) {
                    if 0 != hexvals {
                        printf(b"At %x %x sectors\n\x00" as *const u8 as
                                   *const libc::c_char,
                               intswap32((*inode_buf).datablocks.d32[loop_0 as
                                                                         usize].sector),
                               intswap32((*inode_buf).datablocks.d32[loop_0 as
                                                                         usize].count));
                    } else {
                        printf(b"At %u %u sectors\n\x00" as *const u8 as
                                   *const libc::c_char,
                               intswap32((*inode_buf).datablocks.d32[loop_0 as
                                                                         usize].sector),
                               intswap32((*inode_buf).datablocks.d32[loop_0 as
                                                                         usize].count));
                    }
                    loop_0 += 1
                }
            }
        } else {
            printf(b"Data is in inode block.\n\x00" as *const u8 as
                       *const libc::c_char);
        }
        buf = buf.offset(1024isize);
        bufsize = bufsize.wrapping_sub(1024i32 as libc::c_uint);
        inode_buf = buf as *mut mfs_inode;
        if !(bufsize > 512i32 as libc::c_uint &&
                 (*inode_buf).sig == intswap32(0x91231ebcu32) &&
                 0 !=
                     mfs_check_crc(inode_buf as *mut libc::c_uchar,
                                   512i32 as libc::c_uint,
                                   (&mut (*inode_buf).checksum as
                                        *mut libc::c_uint).wrapping_offset_from(inode_buf
                                                                                    as
                                                                                    *mut libc::c_uint)
                                       as libc::c_long as libc::c_uint)) {
            break ;
        }
    }
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn dump_mfs_header_32(mut hdr: *mut volume_header_32,
                                            mut buf: *mut libc::c_uchar,
                                            mut bufsize: libc::c_uint)
 -> libc::c_int {
    if (bufsize as libc::c_ulong) <
           ::std::mem::size_of::<volume_header_32>() as libc::c_ulong {
        return 0i32
    }
    if 0 ==
           mfs_check_crc(hdr as *mut libc::c_uchar,
                         ::std::mem::size_of::<volume_header_32>() as
                             libc::c_ulong as libc::c_uint,
                         (&mut (*hdr).checksum as *mut uint32_t as
                              *mut libc::c_uint).wrapping_offset_from(hdr as
                                                                          *mut libc::c_uint)
                             as libc::c_long as libc::c_uint) {
        return 0i32
    }
    printf(b"\n    MFS Volume Header (32-bit)\n\x00" as *const u8 as
               *const libc::c_char);
    //State is the magic entry that does not match the mfs endainness
    printf(b"State: %08x   First partition size: %ux1024 sectors (%u MiB)\n\x00"
               as *const u8 as *const libc::c_char,
           if 0 != mfsLSB {
               intswap32((*hdr).magicMSB)
           } else { intswap32((*hdr).magicLSB) },
           intswap32((*hdr).firstpartsize),
           intswap32((*hdr).firstpartsize).wrapping_div(2i32 as
                                                            libc::c_uint));
    //magic is the magic entry that matches the mfs endainness
    printf(b"Sig: %08x   CRC: %08x   Size: %u\n\x00" as *const u8 as
               *const libc::c_char,
           if 0 != mfsLSB {
               intswap32((*hdr).magicLSB)
           } else { intswap32((*hdr).magicMSB) }, intswap32((*hdr).checksum),
           intswap32((*hdr).total_sectors));
    printf(b"MFS Partitions: %s\n\x00" as *const u8 as *const libc::c_char,
           (*hdr).partitionlist.as_mut_ptr());
    printf(b"Root FSID: %-13uNext FSID: %u\n\x00" as *const u8 as
               *const libc::c_char, intswap32((*hdr).root_fsid),
           intswap32((*hdr).next_fsid));
    if 0 != hexvals {
        printf(b"Redo log start: %08x     Size: %08x\n\x00" as *const u8 as
                   *const libc::c_char, intswap32((*hdr).logstart),
               intswap32((*hdr).lognsectors));
        printf(b"?        start: %08x     Size: %08x\n\x00" as *const u8 as
                   *const libc::c_char, intswap32((*hdr).unkstart),
               intswap32((*hdr).unksectors));
        printf(b"Zone map start: %08x     Size: %08x\n\x00" as *const u8 as
                   *const libc::c_char, intswap32((*hdr).zonemap.sector),
               intswap32((*hdr).zonemap.length));
        printf(b"        backup: %08x     Zone size: %08x      Allocation size: %08x\n\x00"
                   as *const u8 as *const libc::c_char,
               intswap32((*hdr).zonemap.sbackup),
               intswap32((*hdr).zonemap.size), intswap32((*hdr).zonemap.min));
    } else {
        printf(b"Redo log start: %-13uSize: %u\n\x00" as *const u8 as
                   *const libc::c_char, intswap32((*hdr).logstart),
               intswap32((*hdr).lognsectors));
        printf(b"?        start: %-13uSize: %u\n\x00" as *const u8 as
                   *const libc::c_char, intswap32((*hdr).unkstart),
               intswap32((*hdr).unksectors));
        printf(b"Zone map start: %-13uSize: %u\n\x00" as *const u8 as
                   *const libc::c_char, intswap32((*hdr).zonemap.sector),
               intswap32((*hdr).zonemap.length));
        printf(b"        backup: %-13uZone size: %-13uAllocation size: %u\n\x00"
                   as *const u8 as *const libc::c_char,
               intswap32((*hdr).zonemap.sbackup),
               intswap32((*hdr).zonemap.size), intswap32((*hdr).zonemap.min));
    }
    printf(b"Last sync boot: %-13uTimestamp: %-13uLast Commit: %u\n\x00" as
               *const u8 as *const libc::c_char, intswap32((*hdr).bootcycles),
           intswap32((*hdr).bootsecs), intswap32((*hdr).volhdrlogstamp));
    if 0 != (*hdr).off0c || 0 != (*hdr).off14 || 0 != (*hdr).off1c ||
           0 != (*hdr).off20 || 0 != (*hdr).offa8 || 0 != (*hdr).offe4 {
        printf(b"Unknown data\n\x00" as *const u8 as *const libc::c_char);
        if 0 != (*hdr).off0c {
            printf(b"00000000:00c %02x %02x %02x %02x\n\x00" as *const u8 as
                       *const libc::c_char,
                   *buf.offset(12isize) as libc::c_int,
                   *buf.offset(13isize) as libc::c_int,
                   *buf.offset(14isize) as libc::c_int,
                   *buf.offset(15isize) as libc::c_int);
        }
        if 0 != (*hdr).off14 {
            printf(b"00000000:014 %02x %02x %02x %02x\n\x00" as *const u8 as
                       *const libc::c_char,
                   *buf.offset(20isize) as libc::c_int,
                   *buf.offset(21isize) as libc::c_int,
                   *buf.offset(22isize) as libc::c_int,
                   *buf.offset(23isize) as libc::c_int);
        }
        if 0 != (*hdr).off1c || 0 != (*hdr).off20 {
            printf(b"00000000:01c %02x %02x %02x %02x %02x %02x %02x %02x\n\x00"
                       as *const u8 as *const libc::c_char,
                   *buf.offset(28isize) as libc::c_int,
                   *buf.offset(29isize) as libc::c_int,
                   *buf.offset(30isize) as libc::c_int,
                   *buf.offset(31isize) as libc::c_int,
                   *buf.offset(32isize) as libc::c_int,
                   *buf.offset(33isize) as libc::c_int,
                   *buf.offset(34isize) as libc::c_int,
                   *buf.offset(35isize) as libc::c_int);
        }
        if 0 != (*hdr).offa8 {
            printf(b"00000000:0a8 %02x %02x %02x %02x\n\x00" as *const u8 as
                       *const libc::c_char,
                   *buf.offset(168isize) as libc::c_int,
                   *buf.offset(169isize) as libc::c_int,
                   *buf.offset(170isize) as libc::c_int,
                   *buf.offset(171isize) as libc::c_int);
        }
        if 0 != (*hdr).offe4 {
            printf(b"00000000:0e4 %02x %02x %02x %02x\n\x00" as *const u8 as
                       *const libc::c_char,
                   *buf.offset(228isize) as libc::c_int,
                   *buf.offset(229isize) as libc::c_int,
                   *buf.offset(230isize) as libc::c_int,
                   *buf.offset(231isize) as libc::c_int);
        }
    }
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn dump_mfs_header_64(mut hdr: *mut volume_header_64,
                                            mut buf: *mut libc::c_uchar,
                                            mut bufsize: libc::c_uint)
 -> libc::c_int {
    if (bufsize as libc::c_ulong) <
           ::std::mem::size_of::<volume_header_64>() as libc::c_ulong {
        return 0i32
    }
    if 0 ==
           mfs_check_crc(hdr as *mut libc::c_uchar,
                         ::std::mem::size_of::<volume_header_64>() as
                             libc::c_ulong as libc::c_uint,
                         (&mut (*hdr).checksum as *mut uint32_t as
                              *mut libc::c_uint).wrapping_offset_from(hdr as
                                                                          *mut libc::c_uint)
                             as libc::c_long as libc::c_uint) {
        return 0i32
    }
    printf(b"\n    MFS Volume Header (64-bit)\n\x00" as *const u8 as
               *const libc::c_char);
    //State is the magic entry that does not match the mfs endainness
    printf(b"State: %08x   First partition size: %ux1024 sectors (%u MiB)\n\x00"
               as *const u8 as *const libc::c_char,
           if 0 != mfsLSB {
               intswap32((*hdr).magicMSB)
           } else { intswap32((*hdr).magicLSB) },
           intswap32((*hdr).firstpartsize),
           intswap32((*hdr).firstpartsize).wrapping_div(2i32 as
                                                            libc::c_uint));
    //Magic is the magic entry that matches the mfs endainness
    printf(b"MFS Partitions: %s\n\x00" as *const u8 as *const libc::c_char,
           (*hdr).partitionlist.as_mut_ptr());
    printf(b"Root FSID: %-13uNext FSID: %u\n\x00" as *const u8 as
               *const libc::c_char, intswap32((*hdr).root_fsid),
           intswap32((*hdr).next_fsid));
    0 != hexvals;
    if 0 != (*hdr).off0c || 0 != (*hdr).off14 || 0 != (*hdr).off1c ||
           0 != (*hdr).off20 || 0 != (*hdr).offc8 || 0 != (*hdr).off100 ||
           0 != (*hdr).off110 || 0 != (*hdr).off114 {
        printf(b"Unknown data\n\x00" as *const u8 as *const libc::c_char);
        if 0 != (*hdr).off0c {
            printf(b"00000000:00c %02x %02x %02x %02x\n\x00" as *const u8 as
                       *const libc::c_char,
                   *buf.offset(12isize) as libc::c_int,
                   *buf.offset(13isize) as libc::c_int,
                   *buf.offset(14isize) as libc::c_int,
                   *buf.offset(15isize) as libc::c_int);
        }
        if 0 != (*hdr).off14 {
            printf(b"00000000:014 %02x %02x %02x %02x\n\x00" as *const u8 as
                       *const libc::c_char,
                   *buf.offset(20isize) as libc::c_int,
                   *buf.offset(21isize) as libc::c_int,
                   *buf.offset(22isize) as libc::c_int,
                   *buf.offset(23isize) as libc::c_int);
        }
        if 0 != (*hdr).off1c || 0 != (*hdr).off20 {
            printf(b"00000000:01c %02x %02x %02x %02x %02x %02x %02x %02x\n\x00"
                       as *const u8 as *const libc::c_char,
                   *buf.offset(28isize) as libc::c_int,
                   *buf.offset(29isize) as libc::c_int,
                   *buf.offset(30isize) as libc::c_int,
                   *buf.offset(31isize) as libc::c_int,
                   *buf.offset(32isize) as libc::c_int,
                   *buf.offset(33isize) as libc::c_int,
                   *buf.offset(34isize) as libc::c_int,
                   *buf.offset(35isize) as libc::c_int);
        }
        if 0 != (*hdr).offc8 {
            printf(b"00000000:0c0 %02x %02x %02x %02x\n\x00" as *const u8 as
                       *const libc::c_char,
                   *buf.offset(200isize) as libc::c_int,
                   *buf.offset(201isize) as libc::c_int,
                   *buf.offset(202isize) as libc::c_int,
                   *buf.offset(203isize) as libc::c_int);
        }
        if 0 != (*hdr).off100 {
            printf(b"00000000:100 %02x %02x %02x %02x\n\x00" as *const u8 as
                       *const libc::c_char,
                   *buf.offset(256isize) as libc::c_int,
                   *buf.offset(257isize) as libc::c_int,
                   *buf.offset(258isize) as libc::c_int,
                   *buf.offset(259isize) as libc::c_int);
        }
        if 0 != (*hdr).off110 || 0 != (*hdr).off114 {
            printf(b"00000000:110 %02x %02x %02x %02x %02x %02x %02x %02x\n\x00"
                       as *const u8 as *const libc::c_char,
                   *buf.offset(272isize) as libc::c_int,
                   *buf.offset(273isize) as libc::c_int,
                   *buf.offset(274isize) as libc::c_int,
                   *buf.offset(275isize) as libc::c_int,
                   *buf.offset(276isize) as libc::c_int,
                   *buf.offset(277isize) as libc::c_int,
                   *buf.offset(278isize) as libc::c_int,
                   *buf.offset(279isize) as libc::c_int);
        }
    }
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn dump_mfs_header(mut buf: *mut libc::c_uchar,
                                         mut bufsize: libc::c_uint)
 -> libc::c_int {
    let mut hdr: *mut volume_header = 0 as *mut volume_header;
    if (bufsize as libc::c_ulong) <
           ::std::mem::size_of::<volume_header_32>() as libc::c_ulong {
        return 0i32
    }
    hdr = buf as *mut volume_header;
    if 0 !=
           mfs_check_crc(&mut (*hdr).v64 as *mut volume_header_64 as
                             *mut libc::c_uchar,
                         ::std::mem::size_of::<volume_header_64>() as
                             libc::c_ulong as libc::c_uint,
                         (&mut (*hdr).v64.checksum as *mut uint32_t as
                              *mut libc::c_uint).wrapping_offset_from(&mut (*hdr).v64
                                                                          as
                                                                          *mut volume_header_64
                                                                          as
                                                                          *mut libc::c_uint)
                             as libc::c_long as libc::c_uint) {
        return dump_mfs_header_64(&mut (*hdr).v64, buf, bufsize)
    } else {
        if 0 !=
               mfs_check_crc(&mut (*hdr).v32 as *mut volume_header_32 as
                                 *mut libc::c_uchar,
                             ::std::mem::size_of::<volume_header_32>() as
                                 libc::c_ulong as libc::c_uint,
                             (&mut (*hdr).v32.checksum as *mut uint32_t as
                                  *mut libc::c_uint).wrapping_offset_from(&mut (*hdr).v32
                                                                              as
                                                                              *mut volume_header_32
                                                                              as
                                                                              *mut libc::c_uint)
                                 as libc::c_long as libc::c_uint) {
            return dump_mfs_header_32(&mut (*hdr).v32, buf, bufsize)
        }
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn dump_zone_map_32(mut sector: uint64_t,
                                          mut buf: *mut libc::c_uchar,
                                          mut bufsize: libc::c_uint)
 -> libc::c_int {
    let mut fsmem_base: libc::c_uint = 0;
    let mut blocksize: libc::c_uint = 0;
    let mut fsmem_ptrs: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut zone: *mut zone_header_32 = 0 as *mut zone_header_32;
    let mut bitmaps: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if (bufsize as libc::c_ulong) <
           ::std::mem::size_of::<zone_header_32>() as libc::c_ulong {
        return 0i32
    }
    zone = buf as *mut zone_header_32;
    if sector != intswap32((*zone).sector) as libc::c_ulonglong &&
           sector != intswap32((*zone).sbackup) as libc::c_ulonglong ||
           (intswap32((*zone).length).wrapping_mul(512i32 as libc::c_uint) >
                bufsize ||
                0 ==
                    mfs_check_crc(zone as *mut libc::c_uchar,
                                  intswap32((*zone).length).wrapping_mul(512i32
                                                                             as
                                                                             libc::c_uint),
                                  (&mut (*zone).checksum as *mut uint32_t as
                                       *mut libc::c_uint).wrapping_offset_from(zone
                                                                                   as
                                                                                   *mut libc::c_uint)
                                      as libc::c_long as libc::c_uint)) {
        return 0i32
    }
    printf(b"\n    Zone map \x00" as *const u8 as *const libc::c_char);
    match intswap32((*zone).type_0 as uint32_t) {
        0 => { printf(b"(Inode)\n\x00" as *const u8 as *const libc::c_char); }
        1 => {
            printf(b"(Application)\n\x00" as *const u8 as
                       *const libc::c_char);
        }
        2 => { printf(b"(Media)\n\x00" as *const u8 as *const libc::c_char); }
        _ => {
            printf(b"(Unknown type %d)\n\x00" as *const u8 as
                       *const libc::c_char,
                   intswap32((*zone).type_0 as uint32_t));
        }
    }
    fsmem_ptrs = zone.offset(1isize) as *mut libc::c_uint;
    blocksize = intswap32((*zone).min);
    fsmem_base =
        (intswap32(*fsmem_ptrs.offset(0isize)) as
             libc::c_ulong).wrapping_sub((::std::mem::size_of::<zone_header_32>()
                                              as
                                              libc::c_ulong).wrapping_add(intswap32((*zone).num).wrapping_mul(4i32
                                                                                                                  as
                                                                                                                  libc::c_uint)
                                                                              as
                                                                              libc::c_ulong))
            as libc::c_uint;
    if 0 != hexvals {
        printf(b"This zone:                Sector: %08x         Backup: %08x\n\x00"
                   as *const u8 as *const libc::c_char,
               intswap32((*zone).sector), intswap32((*zone).sbackup));
        printf(b"   Length: %08x         Size: %08x     Block size: %08x\n\x00"
                   as *const u8 as *const libc::c_char,
               intswap32((*zone).length), intswap32((*zone).size),
               intswap32((*zone).min));
        printf(b"Next zone:                Sector: %08x         Backup: %08x\n\x00"
                   as *const u8 as *const libc::c_char,
               intswap32((*zone).next.sector),
               intswap32((*zone).next.sbackup));
        printf(b"   Length: %08x         Size: %08x     Block size: %08x\n\x00"
                   as *const u8 as *const libc::c_char,
               intswap32((*zone).next.length), intswap32((*zone).next.size),
               intswap32((*zone).next.min));
        printf(b"First    : %08x         Last: %08x\n\x00" as *const u8 as
                   *const libc::c_char, intswap32((*zone).first),
               intswap32((*zone).last));
        printf(b" Size    : %08x         Free: %08x\n\x00" as *const u8 as
                   *const libc::c_char, intswap32((*zone).size),
               intswap32((*zone).free));
    } else {
        printf(b"This zone:                Sector: %-13u    Backup: %u\n\x00"
                   as *const u8 as *const libc::c_char,
               intswap32((*zone).sector), intswap32((*zone).sbackup));
        printf(b"   Length: %-13u    Size: %-13uBlock size: %u\n\x00" as
                   *const u8 as *const libc::c_char,
               intswap32((*zone).length), intswap32((*zone).size),
               intswap32((*zone).min));
        printf(b"Next zone:                Sector: %-13u    Backup: %u\n\x00"
                   as *const u8 as *const libc::c_char,
               intswap32((*zone).next.sector),
               if (*zone).next.sbackup == 0xaaaaaaaau32 {
                   0i32 as libc::c_uint
               } else { intswap32((*zone).next.sbackup) });
        printf(b"   Length: %-13u    Size: %-13uBlock size: %u\n\x00" as
                   *const u8 as *const libc::c_char,
               intswap32((*zone).next.length), intswap32((*zone).next.size),
               intswap32((*zone).next.min));
        printf(b"First    : %-13u    Last: %u\n\x00" as *const u8 as
                   *const libc::c_char, intswap32((*zone).first),
               intswap32((*zone).last));
        printf(b" Size    : %-13u    Free: %u\n\x00" as *const u8 as
                   *const libc::c_char, intswap32((*zone).size),
               intswap32((*zone).free));
    }
    printf(b"Logstamp : %-13uChecksum: %08x           Zero: %u\n\x00" as
               *const u8 as *const libc::c_char, intswap32((*zone).logstamp),
           intswap32((*zone).checksum), intswap32((*zone).zero));
    printf(b"Bitmaps: %-13ufsmem base: %08x\n\x00" as *const u8 as
               *const libc::c_char, intswap32((*zone).num), fsmem_base);
    bitmaps =
        buf.offset(intswap32(*fsmem_ptrs.offset(0isize)) as
                       isize).offset(-(fsmem_base as isize));
    dump_bitmaps(bitmaps,
                 (bufsize as libc::c_long -
                      bitmaps.wrapping_offset_from(buf) as libc::c_long) as
                     libc::c_uint, fsmem_ptrs,
                 intswap32((*zone).first) as uint64_t,
                 intswap32((*zone).size) as uint64_t, intswap32((*zone).num),
                 intswap32((*zone).min) as uint64_t);
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn dump_zone_map_64(mut sector: uint64_t,
                                          mut buf: *mut libc::c_uchar,
                                          mut bufsize: libc::c_uint)
 -> libc::c_int {
    let mut fsmem_base: libc::c_uint = 0;
    let mut blocksize: libc::c_uint = 0;
    let mut fsmem_ptrs: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut zone: *mut zone_header_64 = 0 as *mut zone_header_64;
    let mut bitmaps: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if (bufsize as libc::c_ulong) <
           ::std::mem::size_of::<zone_header_64>() as libc::c_ulong {
        return 0i32
    }
    zone = buf as *mut zone_header_64;
    if sector != intswap64((*zone).sector) &&
           sector != intswap64((*zone).sbackup) ||
           intswap32((*zone).length).wrapping_mul(512i32 as libc::c_uint) >
               bufsize ||
           0 ==
               mfs_check_crc(zone as *mut libc::c_uchar,
                             intswap32((*zone).length).wrapping_mul(512i32 as
                                                                        libc::c_uint),
                             (&mut (*zone).checksum as *mut uint32_t as
                                  *mut libc::c_uint).wrapping_offset_from(zone
                                                                              as
                                                                              *mut libc::c_uint)
                                 as libc::c_long as libc::c_uint) {
        return 0i32
    }
    printf(b"\n    Zone map \x00" as *const u8 as *const libc::c_char);
    match intswap32((*zone).type_0 as uint32_t) {
        0 => { printf(b"(Inode)\n\x00" as *const u8 as *const libc::c_char); }
        1 => {
            printf(b"(Application)\n\x00" as *const u8 as
                       *const libc::c_char);
        }
        2 => { printf(b"(Media)\n\x00" as *const u8 as *const libc::c_char); }
        _ => {
            printf(b"(Unknown type %d)\n\x00" as *const u8 as
                       *const libc::c_char,
                   intswap32((*zone).type_0 as uint32_t));
        }
    }
    fsmem_ptrs = zone.offset(1isize) as *mut libc::c_uint;
    blocksize = intswap32((*zone).min);
    fsmem_base =
        (intswap32(*fsmem_ptrs.offset(0isize)) as
             libc::c_ulong).wrapping_sub((::std::mem::size_of::<zone_header_64>()
                                              as
                                              libc::c_ulong).wrapping_add(intswap32((*zone).num).wrapping_mul(4i32
                                                                                                                  as
                                                                                                                  libc::c_uint)
                                                                              as
                                                                              libc::c_ulong))
            as libc::c_uint;
    if 0 != hexvals {
        printf(b"   Length: %08x         Size: %08x    Block size: %08x\n\x00"
                   as *const u8 as *const libc::c_char,
               intswap32((*zone).length), intswap32((*zone).size as uint32_t),
               intswap32((*zone).min));
        printf(b"   Length: %08x         Size: %08x    Block size: %08x\n\x00"
                   as *const u8 as *const libc::c_char,
               intswap32((*zone).next_length),
               intswap32((*zone).next_size as uint32_t),
               intswap32((*zone).next_min));
    } else {
        printf(b"   Length: %-13u    Size: %-13uBlock size: %u\n\x00" as
                   *const u8 as *const libc::c_char,
               intswap32((*zone).length), intswap32((*zone).size as uint32_t),
               intswap32((*zone).min));
        printf(b"   Length: %-13u    Size: %-13uBlock size: %u\n\x00" as
                   *const u8 as *const libc::c_char,
               intswap32((*zone).next_length),
               intswap32((*zone).next_size as uint32_t),
               intswap32((*zone).next_min));
    }
    printf(b"Logstamp : %-13uChecksum: %08x           Zero: %u\n\x00" as
               *const u8 as *const libc::c_char, intswap32((*zone).logstamp),
           intswap32((*zone).checksum), intswap32((*zone).zero));
    printf(b"Bitmaps: %-13ufsmem base: %08x\n\x00" as *const u8 as
               *const libc::c_char, intswap32((*zone).num), fsmem_base);
    bitmaps =
        buf.offset(intswap32(*fsmem_ptrs.offset(0isize)) as
                       isize).offset(-(fsmem_base as isize));
    dump_bitmaps(bitmaps,
                 (bufsize as libc::c_long -
                      bitmaps.wrapping_offset_from(buf) as libc::c_long) as
                     libc::c_uint, fsmem_ptrs, intswap64((*zone).first),
                 intswap32((*zone).size as uint32_t) as uint64_t,
                 intswap32((*zone).num), intswap32((*zone).min) as uint64_t);
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn dump_zone_map(mut sector: uint64_t,
                                       mut buf: *mut libc::c_uchar,
                                       mut bufsize: libc::c_uint)
 -> libc::c_int {
    if 0 != (*mfs).is_64 {
        return dump_zone_map_64(sector, buf, bufsize)
    } else { return dump_zone_map_32(sector, buf, bufsize) };
}
#[no_mangle]
pub unsafe extern "C" fn dump_bitmaps(mut base: *mut libc::c_uchar,
                                      mut bufsize: libc::c_uint,
                                      mut fsmem_ptrs: *mut libc::c_uint,
                                      mut sector: uint64_t,
                                      mut size: uint64_t, mut num: uint32_t,
                                      mut blocksize: uint64_t) {
    let mut nbits: libc::c_uint =
        size.wrapping_div(blocksize) as libc::c_uint;
    let mut intwidth: libc::c_uint = 0;
    let mut loop_0: libc::c_uint = 0;
    let mut bigloop: uint64_t = 0;
    /* Find how wide the sector size is in the appropriate number base */
    if 0 != hexvals {
        bigloop = 1i32 as uint64_t;
        intwidth = 0i32 as libc::c_uint;
        while sector.wrapping_add(size) > bigloop {
            intwidth = intwidth.wrapping_add(1);
            bigloop =
                (bigloop as
                     libc::c_ulonglong).wrapping_mul(16i32 as
                                                         libc::c_ulonglong) as
                    uint64_t as uint64_t
        }
    } else {
        bigloop = 1i32 as uint64_t;
        intwidth = 0i32 as libc::c_uint;
        while sector.wrapping_add(size) > bigloop {
            intwidth = intwidth.wrapping_add(1);
            bigloop =
                (bigloop as
                     libc::c_ulonglong).wrapping_mul(10i32 as
                                                         libc::c_ulonglong) as
                    uint64_t as uint64_t
        }
    }
    loop_0 = 0i32 as libc::c_uint;
    while loop_0 < num {
        let mut loop2: libc::c_int = 0;
        let mut bits: *mut libc::c_uint = 0 as *mut libc::c_uint;
        let mut found: libc::c_int = 0i32;
        let mut linelength: libc::c_int = 0i32;
        let mut bitmap: *mut bitmap_header =
            (intswap32(*fsmem_ptrs.offset(loop_0 as isize)) as
                 size_t).wrapping_sub(intswap32(*fsmem_ptrs.offset(0isize)) as
                                          size_t).wrapping_add(base as size_t)
                as *mut libc::c_void as *mut bitmap_header;
        if !((bitmap as size_t) < base as size_t ||
                 bitmap as size_t >
                     (base as size_t).wrapping_add(bufsize as libc::c_ulong))
           {
            0 != hexvals;
            printf(b" Words: %-12uBits: %-14uActual: %u\n\x00" as *const u8 as
                       *const libc::c_char, intswap32((*bitmap).nints),
                   intswap32((*bitmap).nbits), nbits);
            0 != hexvals;
            bits = bitmap.offset(1isize) as *mut libc::c_uint;
            loop2 = 0i32;
            while (loop2 as libc::c_uint) < intswap32((*bitmap).nints) {
                if 0 != *bits.offset(loop2 as isize) {
                    let mut bitloop: libc::c_int = 0;
                    bitloop = 0i32;
                    while bitloop < 32i32 {
                        if 0 !=
                               intswap32(*bits.offset(loop2 as isize)) &
                                   (1i32 << 31i32 - bitloop) as libc::c_uint {
                            let mut bitaddr: libc::c_uint =
                                ((loop2 * 32i32 + bitloop) as
                                     libc::c_ulonglong).wrapping_mul(blocksize).wrapping_add(sector)
                                    as libc::c_uint;
                            if 0 == found {
                                linelength =
                                    (8i32 as
                                         libc::c_uint).wrapping_add(intwidth.wrapping_mul(2i32
                                                                                              as
                                                                                              libc::c_uint)).wrapping_add(1i32
                                                                                                                              as
                                                                                                                              libc::c_uint)
                                        as libc::c_int;
                                linelength =
                                    (linelength as
                                         libc::c_uint).wrapping_sub((linelength
                                                                         as
                                                                         libc::c_uint).wrapping_rem(intwidth.wrapping_mul(2i32
                                                                                                                              as
                                                                                                                              libc::c_uint).wrapping_add(2i32
                                                                                                                                                             as
                                                                                                                                                             libc::c_uint))).wrapping_add(8i32
                                                                                                                                                                                              as
                                                                                                                                                                                              libc::c_uint)
                                        as libc::c_int;
                                printf(b"%-*s \x00" as *const u8 as
                                           *const libc::c_char,
                                       linelength - 1i32,
                                       b"    Free blocks:\x00" as *const u8 as
                                           *const libc::c_char);
                                found += 1
                            } else if linelength == 0i32 {
                                printf(b"        \x00" as *const u8 as
                                           *const libc::c_char);
                                linelength = 8i32
                            } else {
                                printf(b" \x00" as *const u8 as
                                           *const libc::c_char);
                                linelength += 1
                            }
                            0 != hexvals;
                            found += 1;
                            linelength =
                                (linelength as
                                     libc::c_uint).wrapping_add(intwidth.wrapping_mul(2i32
                                                                                          as
                                                                                          libc::c_uint).wrapping_add(1i32
                                                                                                                         as
                                                                                                                         libc::c_uint))
                                    as libc::c_int as libc::c_int;
                            if (linelength as
                                    libc::c_uint).wrapping_add(intwidth.wrapping_mul(2i32
                                                                                         as
                                                                                         libc::c_uint)).wrapping_add(1i32
                                                                                                                         as
                                                                                                                         libc::c_uint)
                                   >= 80i32 as libc::c_uint {
                                printf(b"\n\x00" as *const u8 as
                                           *const libc::c_char);
                                linelength = 0i32
                            }
                        }
                        bitloop += 1
                    }
                }
                loop2 += 1
            }
            if 0 != linelength {
                printf(b"\n\x00" as *const u8 as *const libc::c_char);
            }
        }
        loop_0 = loop_0.wrapping_add(1);
        nbits = nbits.wrapping_div(2i32 as libc::c_uint);
        blocksize =
            (blocksize as
                 libc::c_ulonglong).wrapping_mul(2i32 as libc::c_ulonglong) as
                uint64_t as uint64_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn dump_log_entry(mut sector: libc::c_uint,
                                        mut buf: *mut libc::c_uchar,
                                        mut bufsize: libc::c_uint)
 -> libc::c_int {
    let mut hdr: *mut log_hdr = 0 as *mut log_hdr;
    let mut off: libc::c_uint = 0;
    let mut hdroff: libc::c_uint = 0;
    if bufsize < 512i32 as libc::c_uint { return 0i32 }
    hdr = buf as *mut log_hdr;
    if sector != 0xdeadbeefu32 &&
           sector as libc::c_ulonglong !=
               mfs_log_stamp_to_sector(mfs, intswap32((*hdr).logstamp)) ||
           0 ==
               mfs_check_crc(buf, 512i32 as libc::c_uint,
                             (&mut (*hdr).crc as
                                  *mut libc::c_uint).wrapping_offset_from(buf
                                                                              as
                                                                              *mut libc::c_uint)
                                 as libc::c_long as libc::c_uint) {
        return 0i32
    }
    printf(b"\n    Log entry stamp %u\n\x00" as *const u8 as
               *const libc::c_char, intswap32((*hdr).logstamp));
    printf(b"Size: %-13uFirst: %-13uCRC: %08x\n\x00" as *const u8 as
               *const libc::c_char, intswap32((*hdr).size),
           intswap32((*hdr).first), intswap32((*hdr).crc));
    off = intswap32((*hdr).first);
    hdroff = 0i32 as libc::c_uint;
    while off < bufsize && off < intswap32((*hdr).size) ||
              hdroff.wrapping_add(512i32 as libc::c_uint) <= bufsize {
        let mut allocated: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut allocwritten: libc::c_uint = 0i32 as libc::c_uint;
        let mut entry: *mut log_entry_all = 0 as *mut log_entry_all;
        if off >= intswap32((*hdr).size) {
            let mut oldlogstamp: libc::c_uint = intswap32((*hdr).logstamp);
            hdroff = hdroff.wrapping_add(512i32 as libc::c_uint);
            off = 0i32 as libc::c_uint;
            hdr = buf.offset(hdroff as isize) as *mut log_hdr;
            if hdroff >= bufsize ||
                   oldlogstamp.wrapping_add(1i32 as libc::c_uint) !=
                       intswap32((*hdr).logstamp) ||
                   0 ==
                       mfs_check_crc(buf.offset(hdroff as isize),
                                     512i32 as libc::c_uint,
                                     (&mut (*hdr).crc as
                                          *mut libc::c_uint).wrapping_offset_from(buf.offset(hdroff
                                                                                                 as
                                                                                                 isize)
                                                                                      as
                                                                                      *mut libc::c_uint)
                                         as libc::c_long as libc::c_uint) {
                return 1i32
            }
            printf(b"\n    Log entry stamp %u\n\x00" as *const u8 as
                       *const libc::c_char, intswap32((*hdr).logstamp));
            printf(b"Size: %-13uFirst: %-13uCRC: %08x\n\x00" as *const u8 as
                       *const libc::c_char, intswap32((*hdr).size),
                   intswap32((*hdr).first), intswap32((*hdr).crc));
        } else {
            entry =
                buf.offset(off as
                               isize).offset(hdroff as
                                                 isize).offset(::std::mem::size_of::<log_hdr>()
                                                                   as
                                                                   libc::c_ulong
                                                                   as isize)
                    as *mut log_entry_all;
            if (*entry).log.length as libc::c_int == 0i32 {
                off = off.wrapping_add(2i32 as libc::c_uint)
            } else {
                // Entry extends into the next log sector
                while off.wrapping_add(intswap16((*entry).log.length) as
                                           libc::c_uint).wrapping_add(2i32 as
                                                                          libc::c_uint).wrapping_sub(allocwritten)
                          > intswap32((*hdr).size) {
                    let mut oldlogstamp_0: libc::c_uint =
                        intswap32((*hdr).logstamp);
                    if allocated.is_null() {
                        allocated =
                            malloc((intswap16((*entry).log.length) as
                                        libc::c_int + 2i32) as libc::c_ulong)
                                as *mut libc::c_uchar;
                        allocwritten = 0i32 as libc::c_uint;
                        entry = allocated as *mut log_entry_all
                    }
                    memcpy(allocated.offset(allocwritten as isize) as
                               *mut libc::c_void,
                           buf.offset(hdroff as
                                          isize).offset(off as
                                                            isize).offset(::std::mem::size_of::<log_hdr>()
                                                                              as
                                                                              libc::c_ulong
                                                                              as
                                                                              isize)
                               as *const libc::c_void,
                           intswap32((*hdr).size).wrapping_sub(off) as
                               libc::c_ulong);
                    allocwritten =
                        allocwritten.wrapping_add(intswap32((*hdr).size).wrapping_sub(off));
                    hdroff = hdroff.wrapping_add(512i32 as libc::c_uint);
                    off = 0i32 as libc::c_uint;
                    hdr = buf.offset(hdroff as isize) as *mut log_hdr;
                    if hdroff >= bufsize ||
                           oldlogstamp_0.wrapping_add(1i32 as libc::c_uint) !=
                               intswap32((*hdr).logstamp) ||
                           0 ==
                               mfs_check_crc(buf.offset(hdroff as isize),
                                             512i32 as libc::c_uint,
                                             (&mut (*hdr).crc as
                                                  *mut libc::c_uint).wrapping_offset_from(buf.offset(hdroff
                                                                                                         as
                                                                                                         isize)
                                                                                              as
                                                                                              *mut libc::c_uint)
                                                 as libc::c_long as
                                                 libc::c_uint) {
                        printf(b"... Continued in next log entry\n\x00" as
                                   *const u8 as *const libc::c_char);
                        free(allocated);
                        return 1i32
                    }
                    printf(b"\n    Continued in log entry stamp %u\n\x00" as
                               *const u8 as *const libc::c_char,
                           intswap32((*hdr).logstamp));
                    printf(b"Size: %-13uFirst: %-13uCRC: %08x\n\x00" as
                               *const u8 as *const libc::c_char,
                           intswap32((*hdr).size), intswap32((*hdr).first),
                           intswap32((*hdr).crc));
                }
                if !allocated.is_null() {
                    memcpy(allocated.offset(allocwritten as isize) as
                               *mut libc::c_void,
                           buf.offset(hdroff as
                                          isize).offset(off as
                                                            isize).offset(::std::mem::size_of::<log_hdr>()
                                                                              as
                                                                              libc::c_ulong
                                                                              as
                                                                              isize)
                               as *const libc::c_void,
                           ((intswap16((*entry).log.length) as libc::c_int +
                                 2i32) as
                                libc::c_uint).wrapping_sub(allocwritten) as
                               libc::c_ulong);
                    off =
                        off.wrapping_add(((intswap16((*entry).log.length) as
                                               libc::c_int + 2i32) as
                                              libc::c_uint).wrapping_sub(allocwritten))
                } else {
                    off =
                        off.wrapping_add((intswap16((*entry).log.length) as
                                              libc::c_int + 2i32) as
                                             libc::c_uint)
                }
                printf(b"\nLog entry length: %-13uType: \x00" as *const u8 as
                           *const libc::c_char,
                       intswap16((*entry).log.length) as libc::c_int);
                match intswap32((*entry).log.transtype) {
                    6 => {
                        // TODO: Unknown transtype 6 first observed on Romio drives.  They should be investigated further.
                        printf(b"WARNING: Unknown log type 6 encountered\n\x00"
                                   as *const u8 as *const libc::c_char);
                    }
                    0 => {
                        printf(b"Zone Map Update\n\x00" as *const u8 as
                                   *const libc::c_char);
                    }
                    7 => {
                        printf(b"Zone Map Update 64 bit\n\x00" as *const u8 as
                                   *const libc::c_char);
                    }
                    1 => {
                        printf(b"Inode Update\n\x00" as *const u8 as
                                   *const libc::c_char);
                    }
                    8 => {
                        printf(b"Inode Update 2\n\x00" as *const u8 as
                                   *const libc::c_char);
                    }
                    2 => {
                        printf(b"Log Commit\n\x00" as *const u8 as
                                   *const libc::c_char);
                    }
                    4 => {
                        printf(b"FS Sync Complete\n\x00" as *const u8 as
                                   *const libc::c_char);
                    }
                    5 => {
                        printf(b"Replay Transaction Log\n\x00" as *const u8 as
                                   *const libc::c_char);
                    }
                    _ => {
                        printf(b"Unknown (%d)\n\x00" as *const u8 as
                                   *const libc::c_char,
                               intswap32((*entry).log.transtype));
                    }
                }
                printf(b"Boot: %-13uTimestamp: %u\n\x00" as *const u8 as
                           *const libc::c_char,
                       intswap32((*entry).log.bootcycles),
                       intswap32((*entry).log.bootsecs));
                printf(b"FSId: %-13u???: %-13u???: %u\n\x00" as *const u8 as
                           *const libc::c_char, intswap32((*entry).log.fsid),
                       intswap32((*entry).log.unk1),
                       intswap32((*entry).log.unk2));
                match intswap32((*entry).log.transtype) {
                    6 => {
                        // TODO: Unknown transtype 6 first observed on Romio drives.  They should be investigated further.
                        printf(b"WARNING: Unknown log type 6 encountered:\n\x00"
                                   as *const u8 as *const libc::c_char);
                    }
                    0 => {
                        printf(b"Zone map update:\n\x00" as *const u8 as
                                   *const libc::c_char);
                        if 0 == (*entry).zonemap_32.remove {
                            printf(b"Change: Allocate     \x00" as *const u8
                                       as *const libc::c_char);
                        } else if (*entry).zonemap_32.remove ==
                                      intswap32(1i32 as uint32_t) {
                            printf(b"Change: Free         \x00" as *const u8
                                       as *const libc::c_char);
                        } else {
                            printf(b"Change: ?%-12u\x00" as *const u8 as
                                       *const libc::c_char,
                                   intswap32((*entry).zonemap_32.remove));
                        }
                        if 0 != (*entry).zonemap_32.unk {
                            printf(b"???: %u\n\x00" as *const u8 as
                                       *const libc::c_char,
                                   intswap32((*entry).zonemap_32.unk));
                        }
                        if 0 != hexvals {
                            printf(b"Sector: %08x     Size: %08x\n\x00" as
                                       *const u8 as *const libc::c_char,
                                   intswap32((*entry).zonemap_32.sector),
                                   intswap32((*entry).zonemap_32.size));
                        } else {
                            printf(b"Sector: %-13uSize: %u\n\x00" as *const u8
                                       as *const libc::c_char,
                                   intswap32((*entry).zonemap_32.sector),
                                   intswap32((*entry).zonemap_32.size));
                        }
                    }
                    7 => {
                        printf(b"Zone map update:\n\x00" as *const u8 as
                                   *const libc::c_char);
                        if 0 == (*entry).zonemap_64.remove {
                            printf(b"Change: Allocate     \x00" as *const u8
                                       as *const libc::c_char);
                        } else if (*entry).zonemap_64.remove ==
                                      intswap32(1i32 as uint32_t) {
                            printf(b"Change: Free         \x00" as *const u8
                                       as *const libc::c_char);
                        } else {
                            printf(b"Change: ?%-12u\x00" as *const u8 as
                                       *const libc::c_char,
                                   intswap32((*entry).zonemap_64.remove));
                        }
                        0 != hexvals;
                        printf(b"Unknown Flag: %u\n\x00" as *const u8 as
                                   *const libc::c_char,
                               (*entry).zonemap_64.flag as libc::c_int);
                    }
                    1 | 8 => {
                        printf(b"Inode update:\n\x00" as *const u8 as
                                   *const libc::c_char);
                        dump_inode_log(&mut (*entry).inode);
                    }
                    _ => { }
                }
                if !allocated.is_null() { free(allocated); }
            }
        }
    }
    return 1i32;
}