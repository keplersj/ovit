#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
extern crate libc;
extern "C" {
    /* From macpart.c */
    #[no_mangle]
    fn tivo_partition_open(device: *mut libc::c_char, flags: libc::c_int)
     -> *mut tpFILE;
    #[no_mangle]
    fn tivo_partition_close(file: *mut tpFILE);
    #[no_mangle]
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_ulong;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...)
     -> libc::c_int;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
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
pub type tpFILE = tivo_partition_file;
// #ifdef HAVE_ERRNO_H
// #endif
/* **********************************************************************/
/* Translate a device name from the TiVo view of the world to reality, */
/* allowing relocating of MFS volumes by setting MFS_... variables. */
/* For example, MFS_HDA=/dev/hdb would make /dev/hda* into /dev/hdb* */
/* Note that it goes most specific first, so MFS_HDA10 would match before */
/* MFS_HDA on /dev/hda10.  Also note that MFS_HDA1 would match /dev/hda10, */
/* so be careful.  In addition to relocating, if a relocated device starts */
/* with RO: the device or file will be opened O_RDONLY no matter what the */
/* requested mode was. */
#[no_mangle]
pub unsafe extern "C" fn mfsvol_device_translate(mut hnd: *mut volume_handle,
                                                 mut dev: *mut libc::c_char)
 -> *mut libc::c_char {
    static mut devname: [libc::c_char; 1024] = [0; 1024];
    let mut dev_len: libc::c_int =
        strcspn(dev, b" \x00" as *const u8 as *const libc::c_char) as
            libc::c_int;
    /* See if it is in /dev, to be relocated. */
    if 0 ==
           strncmp(dev, b"/dev/hd\x00" as *const u8 as *const libc::c_char,
                   7i32 as libc::c_ulong) ||
           0 ==
               strncmp(dev,
                       b"/dev/sd\x00" as *const u8 as *const libc::c_char,
                       7i32 as libc::c_ulong) {
        let mut devbase: *mut libc::c_char = 0 as *mut libc::c_char;
        match *dev.offset(7isize) as libc::c_int {
            97 => { devbase = (*hnd).hda }
            98 => { devbase = (*hnd).hdb }
            99 => { devbase = (*hnd).hdb }
            _ => { }
        }
        if !devbase.is_null() {
            sprintf(devname.as_mut_ptr(),
                    b"%s%.*s\x00" as *const u8 as *const libc::c_char,
                    devbase, dev_len - 8i32, dev.offset(8isize));
            return devname.as_mut_ptr()
        }
    }
    // Only hda and hdb allowed as device names.
    (*hnd).err_msg =
        b"Unknown MFS partition device %.*s\x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char;
    return 0 as *mut libc::c_char;
}
/* *********************************************/
/* Return the size of all loaded volume sets. */
#[no_mangle]
pub unsafe extern "C" fn mfsvol_volume_set_size(mut hnd: *mut volume_handle)
 -> libc::c_int {
    let mut vol: *mut volume_info = 0 as *mut volume_info;
    vol = (*hnd).volumes;
    while !vol.is_null() { vol = (*vol).next }
    panic!("Reached end of non-void function without returning");
}
/* ***************************************************************************/
/* Verify that a sector is writable.  This should be done for all groups of */
/* sectors to be written, since individual volumes can be opened RDONLY. */
#[no_mangle]
pub unsafe extern "C" fn mfsvol_is_writable(mut hnd: *mut volume_handle,
                                            mut sector: libc::c_int)
 -> libc::c_int {
    let mut vol: *mut volume_info = 0 as *mut volume_info;
    if vol.is_null() { return 0i32 }
    if 0 != (*vol).vol_flags & 2i32 {
        return ((*hnd).write_mode as libc::c_uint !=
                    vwNormal as libc::c_int as libc::c_uint) as libc::c_int
    }
    return 1i32;
}
/* **********************************************/
/* Free space used by the volumes linked list. */
#[no_mangle]
pub unsafe extern "C" fn mfsvol_cleanup(mut hnd: *mut volume_handle) {
    while !(*hnd).volumes.is_null() {
        let mut cur: *mut volume_info = 0 as *mut volume_info;
        cur = (*hnd).volumes;
        (*hnd).volumes = (*(*hnd).volumes).next;
        tivo_partition_close((*cur).file);
        free(cur);
    }
    if !(*hnd).hda.is_null() { free((*hnd).hda); }
    if !(*hnd).hdb.is_null() { free((*hnd).hdb); }
    free(hnd);
}
/* Write the data. */
/* *****************************************************************************/
/* Set local mem write mode for making temp changes in memory. */
#[no_mangle]
pub unsafe extern "C" fn mfsvol_enable_memwrite(mut hnd: *mut volume_handle) {
    (*hnd).write_mode =
        ::std::mem::transmute::<libc::c_uint,
                                volume_write_mode_e>((*hnd).write_mode as
                                                         libc::c_uint |
                                                         vwLocal as
                                                             libc::c_int as
                                                             libc::c_uint);
}
/* *****************************************************************************/
/* Discard changes written to memory and disable mem write mode. */
#[no_mangle]
pub unsafe extern "C" fn mfsvol_discard_memwrite(mut hnd:
                                                     *mut volume_handle) {
    let mut volume: *mut volume_info = 0 as *mut volume_info;
    volume = (*hnd).volumes;
    while !volume.is_null() {
        let mut cur: *mut volume_mem_data = 0 as *mut volume_mem_data;
        let mut next: *mut volume_mem_data = 0 as *mut volume_mem_data;
        cur = (*volume).mem_blocks;
        while !cur.is_null() { free(cur); cur = next; next = (*cur).next }
        volume = (*volume).next
    }
    (*hnd).write_mode =
        ::std::mem::transmute::<libc::c_uint,
                                volume_write_mode_e>((*hnd).write_mode as
                                                         libc::c_uint &
                                                         !(vwLocal as
                                                               libc::c_int) as
                                                             libc::c_uint);
}
/* *****************************************************************************/
/* Just a quick init.  All it really does is scan for the env MFS_FAKE_WRITE. */
/* Also get the real device names of hda and hdb. */
#[no_mangle]
pub unsafe extern "C" fn mfsvol_init(mut hda: *const libc::c_char,
                                     mut hdb: *const libc::c_char)
 -> *mut volume_handle {
    let mut fake: *mut libc::c_char =
        getenv(b"MFS_FAKE_WRITE\x00" as *const u8 as *const libc::c_char) as
            *mut libc::c_char;
    let mut hnd: *mut volume_handle = 0 as *mut volume_handle;
    hnd =
        calloc(::std::mem::size_of::<volume_handle>() as libc::c_ulong,
               1i32 as libc::c_ulong) as *mut volume_handle;
    if hnd.is_null() { return hnd }
    if !fake.is_null() && 0 != *fake as libc::c_int {
        (*hnd).write_mode =
            ::std::mem::transmute::<libc::c_uint,
                                    volume_write_mode_e>((*hnd).write_mode as
                                                             libc::c_uint |
                                                             vwFake as
                                                                 libc::c_int
                                                                 as
                                                                 libc::c_uint)
    }
    if !hda.is_null() && 0 != *hda as libc::c_int { (*hnd).hda = strdup(hda) }
    if !hdb.is_null() && 0 != *hdb as libc::c_int { (*hnd).hdb = strdup(hdb) }
    return hnd;
}
/* ************************/
/* Display the MFS volume error */
#[no_mangle]
pub unsafe extern "C" fn mfsvol_perror(mut hnd: *mut volume_handle,
                                       mut str: *mut libc::c_char) {
    !(*hnd).err_msg.is_null();
}
/* ************************************/
/* Return the MFS error in a string. */
#[no_mangle]
pub unsafe extern "C" fn mfsvol_strerror(mut hnd: *mut volume_handle,
                                         mut str: *mut libc::c_char)
 -> libc::c_int {
    if !(*hnd).err_msg.is_null() {
    } else {
        sprintf(str, b"No error\x00" as *const u8 as *const libc::c_char);
        return 0i32
    }
    return 1i32;
}
/* ******************************/
/* Check if there is an error. */
#[no_mangle]
pub unsafe extern "C" fn mfsvol_has_error(mut hnd: *mut volume_handle)
 -> libc::c_int {
    if !(*hnd).err_msg.is_null() { return 1i32 }
    return 0i32;
}
/* *******************/
/* Clear any errors */
#[no_mangle]
pub unsafe extern "C" fn mfsvol_clearerror(mut hnd: *mut volume_handle) {
    (*hnd).err_msg = 0 as *mut libc::c_char;
}