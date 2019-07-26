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
    fn tivo_partition_size(file: *mut tpFILE) -> uint64_t;
    /* From readwrite.c */
    #[no_mangle]
    fn tivo_partition_read(file: *mut tpFILE, buf: *mut libc::c_void,
                           sector: uint64_t, count: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn tivo_partition_write(file: *mut tpFILE, buf: *mut libc::c_void,
                            sector: uint64_t, count: libc::c_int)
     -> libc::c_int;
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
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    /* Zero out the data that is about to be overwritten */
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn isprint(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
}
pub type int64_t = libc::c_longlong;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
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
    (*hnd).err_arg1 = dev_len as size_t as int64_t;
    (*hnd).err_arg2 = dev as size_t as int64_t;
    return 0 as *mut libc::c_char;
}
/* ******************************************************/
/* Return the volume info for the volume sector is in. */
#[no_mangle]
pub unsafe extern "C" fn mfsvol_get_volume(mut hnd: *mut volume_handle,
                                           mut sector: uint64_t)
 -> *mut volume_info {
    let mut vol: *mut volume_info = 0 as *mut volume_info;
    /* Find the volume this sector is from in the table of open volumes. */
    vol = (*hnd).volumes;
    while !vol.is_null() {
        if (*vol).start <= sector &&
               (*vol).start.wrapping_add((*vol).sectors) > sector {
            break ;
        }
        vol = (*vol).next
    }
    return vol;
}
/* ************************************************/
/* Return the size of volume starting at sector. */
#[no_mangle]
pub unsafe extern "C" fn mfsvol_volume_size(mut hnd: *mut volume_handle,
                                            mut sector: uint64_t)
 -> uint64_t {
    let mut vol: *mut volume_info = 0 as *mut volume_info;
    /* Find the volume this sector is from in the table of open volumes. */
    vol = (*hnd).volumes;
    while !vol.is_null() {
        if (*vol).start == sector { break ; }
        vol = (*vol).next
    }
    if !vol.is_null() { return (*vol).sectors }
    return 0i32 as uint64_t;
}
/* *********************************************/
/* Return the size of all loaded volume sets. */
#[no_mangle]
pub unsafe extern "C" fn mfsvol_volume_set_size(mut hnd: *mut volume_handle)
 -> uint64_t {
    let mut vol: *mut volume_info = 0 as *mut volume_info;
    let mut total: uint64_t = 0i32 as uint64_t;
    vol = (*hnd).volumes;
    while !vol.is_null() {
        total =
            (total as libc::c_ulonglong).wrapping_add((*vol).sectors) as
                uint64_t as uint64_t;
        vol = (*vol).next
    }
    return total;
}
/* ***************************************************************************/
/* Verify that a sector is writable.  This should be done for all groups of */
/* sectors to be written, since individual volumes can be opened RDONLY. */
#[no_mangle]
pub unsafe extern "C" fn mfsvol_is_writable(mut hnd: *mut volume_handle,
                                            mut sector: uint64_t)
 -> libc::c_int {
    let mut vol: *mut volume_info = 0 as *mut volume_info;
    vol = mfsvol_get_volume(hnd, sector);
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
/* ****************************************************************************/
/* Locate a block in memory for reading. */
/* This returns the first sector with data for the read, so the reader will */
/* need to walk the list to get the rest. */
#[no_mangle]
pub unsafe extern "C" fn mfsvol_locate_mem_data_for_read(mut volume:
                                                             *mut volume_info,
                                                         mut sector: uint64_t,
                                                         mut count:
                                                             libc::c_int)
 -> *mut volume_mem_data {
    let mut block: *mut volume_mem_data = 0 as *mut volume_mem_data;
    block = (*volume).mem_blocks;
    while !block.is_null() {
        if (*block).start.wrapping_add((*block).sectors) > sector {
            if (*block).start <
                   sector.wrapping_add(count as libc::c_ulonglong) {
                return block
            }
            return 0 as *mut volume_mem_data
        }
        block = (*block).next
    }
    return 0 as *mut volume_mem_data;
}
/* ****************************************************************************/
/* Locate a block in memory for writing. */
/* This allocates a new block if needed, and will coalesce neighboring blocks. */
#[no_mangle]
pub unsafe extern "C" fn mfsvol_locate_mem_data_for_write(mut volume:
                                                              *mut volume_info,
                                                          mut sector:
                                                              uint64_t,
                                                          mut count:
                                                              libc::c_int)
 -> *mut volume_mem_data {
    let mut block: *mut *mut volume_mem_data = 0 as *mut *mut volume_mem_data;
    let mut ret: *mut volume_mem_data = 0 as *mut volume_mem_data;
    let mut tmp: *mut volume_mem_data = 0 as *mut volume_mem_data;
    let mut last_sector: uint64_t =
        sector.wrapping_add(count as libc::c_ulonglong);
    /* Find the first block that overlaps or butts up against the block to write */
    block = &mut (*volume).mem_blocks;
    while !(*block).is_null() {
        if (**block).start.wrapping_add((**block).sectors) >= sector {
            break ;
        }
        block = &mut (**block).next
    }
    /* Find the last sector in blocks that overlap or butt up against the block to write */
    tmp = *block;
    while !tmp.is_null() {
        if (*tmp).start.wrapping_add((*tmp).sectors) >=
               sector.wrapping_add(count as libc::c_ulonglong) {
            if (*tmp).start <= last_sector {
                last_sector = (*tmp).start.wrapping_add((*tmp).sectors)
            }
            break ;
        } else { tmp = (*tmp).next }
    }
    if !(*block).is_null() && (**block).start <= sector {
        /* Simple case, the desired write is entirely within an existing block. */
        if (**block).start.wrapping_add((**block).sectors) >=
               sector.wrapping_add(count as libc::c_ulonglong) {
            return *block
        }
        /* Re-use the existing block, reallocating it to be big enough. */
        *block =
            realloc(*block as *mut libc::c_void,
                    (::std::mem::size_of::<volume_mem_data>() as libc::c_ulong
                         as
                         libc::c_ulonglong).wrapping_add(last_sector.wrapping_sub((**block).start).wrapping_mul(512i32
                                                                                                                    as
                                                                                                                    libc::c_ulonglong))
                        as libc::c_ulong) as *mut volume_mem_data;
        ret = *block;
        /* Out of memory */
        if ret.is_null() { return 0 as *mut volume_mem_data }
        (*ret).sectors = last_sector.wrapping_sub((*ret).start)
    } else {
        /* No blocks overlap with the beginning of the area to write, so create a new entry */
        ret =
            malloc((::std::mem::size_of::<volume_mem_data>() as libc::c_ulong
                        as
                        libc::c_ulonglong).wrapping_add(last_sector.wrapping_sub(sector).wrapping_mul(512i32
                                                                                                          as
                                                                                                          libc::c_ulonglong))
                       as libc::c_ulong) as *mut volume_mem_data;
        (*ret).start = sector;
        (*ret).sectors = last_sector.wrapping_sub(sector);
        (*ret).next = *block;
        *block = ret
    }
    tmp = (*ret).next;
    while !tmp.is_null() && (*tmp).start < last_sector {
        /* Only copy the tail end of the overlap, the rest is about to be overwritten */
        if (*tmp).start.wrapping_add((*tmp).sectors) >
               sector.wrapping_add(count as libc::c_ulonglong) {
            memcpy(&mut *(*ret).data.as_mut_ptr().offset(sector.wrapping_add(count
                                                                                 as
                                                                                 libc::c_ulonglong).wrapping_sub((*ret).start).wrapping_mul(512i32
                                                                                                                                                as
                                                                                                                                                libc::c_ulonglong)
                                                             as isize) as
                       *mut libc::c_uchar as *mut libc::c_void,
                   &mut *(*tmp).data.as_mut_ptr().offset(sector.wrapping_add(count
                                                                                 as
                                                                                 libc::c_ulonglong).wrapping_sub((*tmp).start).wrapping_mul(512i32
                                                                                                                                                as
                                                                                                                                                libc::c_ulonglong)
                                                             as isize) as
                       *mut libc::c_uchar as *const libc::c_void,
                   (*tmp).start.wrapping_add((*tmp).sectors).wrapping_sub(sector.wrapping_add(count
                                                                                                  as
                                                                                                  libc::c_ulonglong)).wrapping_mul(512i32
                                                                                                                                       as
                                                                                                                                       libc::c_ulonglong)
                       as libc::c_ulong);
        }
        (*ret).next = (*tmp).next;
        free(tmp);
        tmp = (*ret).next
    }
    memset(&mut *(*ret).data.as_mut_ptr().offset(sector.wrapping_sub((*ret).start).wrapping_mul(512i32
                                                                                                    as
                                                                                                    libc::c_ulonglong)
                                                     as isize) as
               *mut libc::c_uchar as *mut libc::c_void, 0i32,
           (count * 512i32) as libc::c_ulong);
    return ret;
}
/* ****************************************************************************/
/* Read data from the MFS volume set.  It must be in whole sectors, and must */
/* not cross a volume boundry. */
#[no_mangle]
pub unsafe extern "C" fn mfsvol_read_data(mut hnd: *mut volume_handle,
                                          mut buf: *mut libc::c_void,
                                          mut sector: uint64_t,
                                          mut count: uint32_t)
 -> libc::c_int {
    let mut vol: *mut volume_info = 0 as *mut volume_info;
    let mut block: *mut volume_mem_data = 0 as *mut volume_mem_data;
    let mut nread: libc::c_int = 0i32;
    vol = mfsvol_get_volume(hnd, sector);
    /* If no volumes claim this sector, it's an IO error. */
    if vol.is_null() { return -1i32 }
    /* Make the sector number relative to this volume. */
    sector =
        (sector as libc::c_ulonglong).wrapping_sub((*vol).start) as uint64_t
            as uint64_t;
    if sector.wrapping_add(count as libc::c_ulonglong) > (*vol).sectors {
        return -1i32
    }
    /* Search for any mem data blocks within the read region. */
    block =
        mfsvol_locate_mem_data_for_read(vol, sector, count as libc::c_int);
    while (nread as libc::c_uint) < count.wrapping_mul(512i32 as libc::c_uint)
          {
        if !block.is_null() &&
               (*block).start <=
                   sector.wrapping_add((nread / 512i32) as libc::c_ulonglong)
           {
            /* Copy the data from a memory block if available */
            let mut tocopy: libc::c_int =
                (*block).sectors.wrapping_sub(sector.wrapping_add((nread /
                                                                       512i32)
                                                                      as
                                                                      libc::c_ulonglong).wrapping_sub((*block).start))
                    as libc::c_int;
            if (*block).start.wrapping_add((*block).sectors) >
                   sector.wrapping_add(count as libc::c_ulonglong) {
                tocopy =
                    (tocopy as
                         libc::c_ulonglong).wrapping_sub((*block).start.wrapping_add((*block).sectors).wrapping_sub(sector.wrapping_add(count
                                                                                                                                            as
                                                                                                                                            libc::c_ulonglong)))
                        as libc::c_int as libc::c_int
            }
            memcpy(buf.offset((nread & !511i32) as isize),
                   &mut *(*block).data.as_mut_ptr().offset(sector.wrapping_add((nread
                                                                                    /
                                                                                    512i32)
                                                                                   as
                                                                                   libc::c_ulonglong).wrapping_sub((*block).start).wrapping_mul(512i32
                                                                                                                                                    as
                                                                                                                                                    libc::c_ulonglong)
                                                               as isize) as
                       *mut libc::c_uchar as *const libc::c_void,
                   (tocopy * 512i32) as libc::c_ulong);
            nread += tocopy * 512i32;
            block = (*block).next;
            /* Make sure the new block is still within the read */
            if !block.is_null() &&
                   (*block).start >=
                       sector.wrapping_add(count as libc::c_ulonglong) {
                block = 0 as *mut volume_mem_data
            }
        } else {
            /* Only read to the beginning of a memory block, is one is present. */
            let mut newread: libc::c_int = 0;
            let mut toread: libc::c_int =
                count.wrapping_sub((nread / 512i32) as libc::c_uint) as
                    libc::c_int;
            if !block.is_null() {
                toread =
                    (*block).start.wrapping_sub(sector).wrapping_sub((nread /
                                                                          512i32)
                                                                         as
                                                                         libc::c_ulonglong)
                        as libc::c_int
            }
            newread =
                tivo_partition_read((*vol).file,
                                    buf.offset((nread & !511i32) as isize),
                                    sector.wrapping_add((nread / 512i32) as
                                                            libc::c_ulonglong),
                                    toread);
            /* Propogate errors from any read up */
            if newread < 512i32 {
                if newread < 0i32 { return newread }
                return -1i32
            }
            nread += newread & !511i32
        }
    }
    /* Read the data. */
    return nread;
}
/* ***************************************************************************/
/* Doesn't really belong here, but useful for debugging with MFS_FAKE_WRITE */
/* set, this gets called instead of writing. */
unsafe extern "C" fn hexdump(mut buf: *mut libc::c_uchar,
                             mut sector: libc::c_uint) {
    let mut ofs: libc::c_int = 0;
    ofs = 0i32;
    while ofs < 512i32 {
        let mut line: [libc::c_uchar; 20] = [0; 20];
        let mut myo: libc::c_int = 0;
        myo = 0i32;
        while myo < 16i32 {
            line[myo as usize] =
                (if 0 !=
                        isprint(*buf.offset((myo + ofs) as isize) as
                                    libc::c_int) {
                     *buf.offset((myo + ofs) as isize) as libc::c_int
                 } else { '.' as i32 }) as libc::c_uchar;
            myo += 1
        }
        line[16usize] = ':' as i32 as libc::c_uchar;
        line[17usize] = '\n' as i32 as libc::c_uchar;
        line[18usize] = 0i32 as libc::c_uchar;
        ofs += 16i32
    };
}
/* ***************************************************************************/
/* Write data to the MFS volume set.  It must be in whole sectors, and must */
/* not cross a volume boundry. */
#[no_mangle]
pub unsafe extern "C" fn mfsvol_write_data(mut hnd: *mut volume_handle,
                                           mut buf: *mut libc::c_void,
                                           mut sector: uint64_t,
                                           mut count: uint32_t)
 -> libc::c_int {
    let mut vol: *mut volume_info = 0 as *mut volume_info;
    vol = mfsvol_get_volume(hnd, sector);
    /* If no volumes claim this sector, it's an IO error. */
    if vol.is_null() { return -1i32 }
    if 0 !=
           (*hnd).write_mode as libc::c_uint &
               vwFake as libc::c_int as libc::c_uint {
        let mut loop_0: libc::c_int = 0;
        loop_0 = 0i32;
        while (loop_0 as libc::c_uint) < count {
            hexdump((buf as
                         *mut libc::c_uchar).offset((loop_0 * 512i32) as
                                                        isize),
                    sector.wrapping_add(loop_0 as libc::c_ulonglong) as
                        libc::c_uint);
            loop_0 += 1
        }
        /* Allow mem writes to continue, but not regular writes. */
        if (*hnd).write_mode as libc::c_uint ==
               vwFake as libc::c_int as libc::c_uint {
            return count.wrapping_mul(512i32 as libc::c_uint) as libc::c_int
        }
    }
    /* Make the sector number relative to this volume. */
    sector =
        (sector as libc::c_ulonglong).wrapping_sub((*vol).start) as uint64_t
            as uint64_t;
    if 0 !=
           (*hnd).write_mode as libc::c_uint &
               vwLocal as libc::c_int as libc::c_uint {
        let mut block: *mut volume_mem_data =
            mfsvol_locate_mem_data_for_write(vol, sector,
                                             count as libc::c_int);
        if block.is_null() { return -1i32 }
        memcpy(&mut *(*block).data.as_mut_ptr().offset(sector.wrapping_sub((*block).start).wrapping_mul(512i32
                                                                                                            as
                                                                                                            libc::c_ulonglong)
                                                           as isize) as
                   *mut libc::c_uchar as *mut libc::c_void, buf,
               count.wrapping_mul(512i32 as libc::c_uint) as libc::c_ulong);
        return count.wrapping_mul(512i32 as libc::c_uint) as libc::c_int
    }
    /* If the volume this sector is in was opened read-only, it's an error. */
    if 0 != (*vol).vol_flags & 2i32 { return -1i32 }
    if sector.wrapping_add(count as libc::c_ulonglong) > (*vol).sectors {
        return -1i32
    }
    /* Write the data. */
    return tivo_partition_write((*vol).file, buf, sector,
                                count as libc::c_int);
}
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
        (*volume).mem_blocks = 0 as *mut volume_mem_data;
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
        sprintf(str, (*hnd).err_msg, (*hnd).err_arg1, (*hnd).err_arg2,
                (*hnd).err_arg3);
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
    (*hnd).err_arg1 = 0i32 as int64_t;
    (*hnd).err_arg2 = 0i32 as int64_t;
    (*hnd).err_arg3 = 0i32 as int64_t;
}