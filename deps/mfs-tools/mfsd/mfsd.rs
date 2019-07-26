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
    pub type tivo_partition_file;
    #[no_mangle]
    fn mfsvol_enable_memwrite(hnd: *mut volume_handle);
    /* Size of each bitmap is (nints + (nbits < 8? 1: 2)) * 4 */
/* Don't ask why, thats just the way it is. */
/* In bitmap, MSB is first, LSB last */
    #[no_mangle]
    fn mfs_next_zone(mfshdn: *mut mfs_handle, cur: *mut zone_header)
     -> *mut zone_header;
    #[no_mangle]
    fn mfs_perror(mfshnd: *mut mfs_handle, str: *mut libc::c_char);
    #[no_mangle]
    fn mfs_has_error(mfshnd: *mut mfs_handle) -> libc::c_int;
    #[no_mangle]
    fn mfs_log_read(mfshnd: *mut mfs_handle, buf: *mut libc::c_void,
                    logstamp: libc::c_uint) -> libc::c_int;
    #[no_mangle]
    fn mfs_log_fssync(mfshnd: *mut mfs_handle) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn isprint(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
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
    pub sector: libc::c_int,
    pub count: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub sector: libc::c_uint,
    pub count: libc::c_uint,
}
pub type log_inode_update = log_inode_update_s;
pub const Hex: C2RustUnnamed_5 = 1;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const Bin: C2RustUnnamed_5 = 2;
pub const None_0: C2RustUnnamed_5 = 0;
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
    if (*entry).type_0 as libc::c_int == tyStream as libc::c_int {
        0 != hexvals;
    } else { 0 != hexvals; }
    if 0 == (*entry).inodedata {
        let mut loop_0: libc::c_int = 0;
        0 != (*mfs).is_64;
    } else {
        printf(b"Data is in inode block.\n\x00" as *const u8 as
                   *const libc::c_char);
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
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn dump_zone_map(mut sector: libc::c_int,
                                       mut buf: *mut libc::c_uchar,
                                       mut bufsize: libc::c_uint)
 -> libc::c_int {
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn dump_bitmaps(mut base: *mut libc::c_uchar,
                                      mut bufsize: libc::c_uint,
                                      mut fsmem_ptrs: *mut libc::c_uint,
                                      mut sector: libc::c_int,
                                      mut size: libc::c_int,
                                      mut num: libc::c_int,
                                      mut blocksize: libc::c_int) {
    let mut nbits: libc::c_uint = 0;
    let mut intwidth: libc::c_uint = 0;
    let mut loop_0: libc::c_uint = 0;
    /* Find how wide the sector size is in the appropriate number base */
    0 != hexvals;
}