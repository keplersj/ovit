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
    /* Compression */
    pub type z_stream_s;
    #[no_mangle]
    fn backup_set_resource_check(info: *mut backup_info) -> libc::c_int;
    #[no_mangle]
    fn backup_set_thresh(info: *mut backup_info, thresh: libc::c_uint);
    #[no_mangle]
    fn backup_set_skipdb(info: *mut backup_info, skipdb: libc::c_uint);
    #[no_mangle]
    fn backup_check_truncated_volume(info: *mut backup_info);
    #[no_mangle]
    fn backup_start(info: *mut backup_info) -> libc::c_int;
    #[no_mangle]
    fn backup_read(info: *mut backup_info, buf: *mut libc::c_uchar,
                   size: libc::c_uint) -> libc::c_uint;
    #[no_mangle]
    fn backup_finish(info: *mut backup_info) -> libc::c_int;
    #[no_mangle]
    fn backup_perror(info: *mut backup_info, str: *mut libc::c_char);
    #[no_mangle]
    fn backup_has_error(info: *mut backup_info) -> libc::c_int;
    #[no_mangle]
    fn init_restore(flags: libc::c_uint) -> *mut backup_info;
    #[no_mangle]
    fn restore_set_varsize(info: *mut backup_info, size: libc::c_int);
    #[no_mangle]
    fn restore_set_dbsize(info: *mut backup_info, size: libc::c_int);
    #[no_mangle]
    fn restore_set_swapsize(info: *mut backup_info, size: libc::c_int);
    #[no_mangle]
    fn restore_set_mfs_type(info: *mut backup_info, bits: libc::c_int);
    #[no_mangle]
    fn restore_set_minalloc(info: *mut backup_info, minalloc: libc::c_uint);
    #[no_mangle]
    fn restore_set_bswap(info: *mut backup_info, bswap: libc::c_int);
    #[no_mangle]
    fn restore_write(info: *mut backup_info, buf: *mut libc::c_uchar,
                     size: libc::c_uint) -> libc::c_uint;
    #[no_mangle]
    fn restore_start(info: *mut backup_info) -> libc::c_int;
    #[no_mangle]
    fn restore_finish(info: *mut backup_info) -> libc::c_int;
    #[no_mangle]
    fn restore_perror(info: *mut backup_info, str: *mut libc::c_char);
    #[no_mangle]
    fn restore_has_error(info: *mut backup_info) -> libc::c_int;
    #[no_mangle]
    fn tivo_partition_direct();
    #[no_mangle]
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_ulong;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct backup_block {
    pub firstsector: libc::c_uint,
    pub sectors: libc::c_uint,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct backup_partition {
    pub sectors: libc::c_uint,
    pub partno: libc::c_char,
    pub devno: libc::c_char,
    pub reserved: [libc::c_char; 2],
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct zone_map_info {
    pub map_length: libc::c_uint,
    pub zone_type: libc::c_uint,
    pub fsmem_base: libc::c_uint,
    pub min_au: libc::c_uint,
    pub size: libc::c_int,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct device_info {
    pub files: *mut *mut tivo_partition_file,
    pub nparts: libc::c_int,
    pub devname: *mut libc::c_char,
}
pub type backup_format = libc::c_uint;
pub const bfWinMFS: backup_format = 2;
pub const bfV3: backup_format = 1;
pub const bfV1: backup_format = 0;
pub type backup_state_ret = libc::c_int;
pub const bsNextState: backup_state_ret = 1;
pub const bsMoreData: backup_state_ret = 0;
pub const bsError: backup_state_ret = -1;
/* States of the state machine for backup and restore */
/* In addition to the state, there are 2 state specific values state_val1 */
/* and state_val2 which are set to 0 at every state change. */
/* The pointer in state_ptr1 is also set to NULL at every state change. */
/* The value shared_val1 is shared between states. */
pub type backup_state = libc::c_uint;
// --- no state val usage
			// 512 bytes with CRC at the end
			// Restore should check for CRC32_RESIDUAL as crc value at end
pub const bsMax: backup_state = 18;
// state_val1 as current inode number.
		// state_val2 as offset within current inode.
		// state_ptr1 as pointer to current inode structure.
			// For each inode, inode meta-data followed by inode data in the
			// next 512 byte aligned block
pub const bsComplete: backup_state = 17;
// --- no state val usage
			// Create zone maps and initialize MFS
pub const bsInodes: backup_state = 16;
// --- no state val usage
			// Region referenced by volume header
pub const bsMfsReinit: backup_state = 15;
// --- no state val usage
			// Region referenced by volume header
pub const bsUnkRegion: backup_state = 14;
// --- no state val usage
			// Offset 0 of MFS volume
pub const bsTransactionLog: backup_state = 13;
// state_val1 as current MFS block.
		// state_val2 as offset within current MFS blocks.
			// Blocks read from MFS - all of MFS backed up
/* v3 backup only after this point */
pub const bsVolumeHeader: backup_state = 12;
// --- no state val used
			// Loads the MFS volumes (Restore only)
/* v1 backup only */
pub const bsBlocks: backup_state = 11;
// state_val1 as current partition number.
		// state_val2 as offset within current partition.
			// Raw partitions to backup, one after another
pub const bsMFSInit: backup_state = 10;
// --- no state val used
			// Sector 0 of the A drive
pub const bsPartitions: backup_state = 9;
// If shared_val1 is not 0 or 512, consume remainder of block.
			// Consume partial block left by MFS partition list
pub const bsBootBlock: backup_state = 8;
// state_val1 as current extra info offset.
		// state_val2 as current extra info index.
		// shared_val1 as offset within current block
			// List follows immediately after  zone map info
pub const bsInfoEnd: backup_state = 7;
// state_val1 as current zone map offset.
		// shared_val1 as offset within current block
			// List follows immediately after partition list
pub const bsInfoExtra: backup_state = 6;
// state_val1 as current MFS partition offset.
		// shared_val1 as offset within current block of last MFS partition,
			// List follows immediately after inode or block list
/* v3 backup only */
pub const bsInfoZoneMaps: backup_state = 5;
// state_val1 as current MFS block offset.
		// shared_val1 as offset within current block of last MFS block,
			// List follows immediately after partition list
pub const bsInfoMFSPartitions: backup_state = 4;
// state_val1 as current partition offset.
		// shared_val1 as offset within current block of last partition,
			// List follows immediately after backup header
/* v1 backup only */
pub const bsInfoBlocks: backup_state = 3;
// shared_val1 initialized to sizeof backup header padded to 8 bytes.
			// Write backup header
/* Backup description collection */
pub const bsInfoPartitions: backup_state = 2;
// --- no state val used
			// No data consumed, just scans MFS for what should be backed up
pub const bsBegin: backup_state = 1;
pub const bsScanMFS: backup_state = 0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct backup_info {
    pub cursector: libc::c_int,
    pub nsectors: libc::c_int,
    pub state_val1: libc::c_int,
    pub state_val2: libc::c_int,
    pub shared_val1: libc::c_int,
    pub state_ptr1: *mut libc::c_void,
    pub state: backup_state,
    pub format: backup_format,
    pub state_machine: *mut backup_state_handler,
    pub ndevs: libc::c_int,
    pub devs: *mut device_info,
    pub nparts: libc::c_int,
    pub parts: *mut backup_partition,
    pub nblocks: libc::c_int,
    pub blocks: *mut backup_block,
    pub ninodes: libc::c_int,
    pub inodes: *mut libc::c_uint,
    pub nextrainfo: libc::c_int,
    pub extrainfosize: libc::c_int,
    pub extrainfo: *mut *mut extrainfo,
    pub nmfs: libc::c_int,
    pub mfsparts: *mut backup_partition,
    pub nzones: libc::c_int,
    pub zonemaps: *mut zone_map_info,
    pub ilogtype: libc::c_uint,
    pub appsectors: libc::c_int,
    pub mediasectors: libc::c_int,
    pub appinodes: libc::c_int,
    pub mediainodes: libc::c_int,
    pub back_flags: libc::c_int,
    pub rest_flags: libc::c_int,
    pub crc: libc::c_int,
    pub comp: *mut z_stream_s,
    pub comp_buf: *mut libc::c_uchar,
    pub mfs: *mut mfs_handle,
    pub err_msg: *mut libc::c_char,
    pub err_arg1: libc::c_int,
    pub err_arg2: libc::c_int,
    pub err_arg3: libc::c_int,
    pub thresh: libc::c_uint,
    pub skipdb: libc::c_uint,
    pub hda: *mut libc::c_char,
    pub shrink_to: libc::c_uint,
}
/* Used to track information such as release and model number */
/* Also potentially for hints that restore can use on new backups without */
/* changing the format to add the hints. */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct extrainfo {
    pub typelength: libc::c_uchar,
    pub datatype: libc::c_uchar,
    pub datalength: libc::c_ushort,
    pub data: [libc::c_char; 0],
}
pub type backup_state_handler
    =
    [Option<unsafe extern "C" fn(_: *mut backup_info, _: *mut libc::c_void,
                                 _: libc::c_uint, _: *mut libc::c_uint)
                -> backup_state_ret>; 18];
#[no_mangle]
pub unsafe extern "C" fn copy_usage(mut progname: *mut libc::c_char) { }
#[no_mangle]
pub unsafe extern "C" fn get_drives(mut drives: *mut libc::c_char,
                                    mut adrive: *mut libc::c_char,
                                    mut bdrive: *mut libc::c_char) {
    let mut devlen: libc::c_int =
        strcspn(drives, b":\x00" as *const u8 as *const libc::c_char) as
            libc::c_int;
    strncpy(adrive, drives, devlen as libc::c_ulong);
    *adrive.offset(devlen as isize) = 0i32 as libc::c_char;
    if *drives.offset(devlen as isize) as libc::c_int != 0i32 {
        strcpy(bdrive, drives.offset(devlen as isize).offset(1isize));
    };
}