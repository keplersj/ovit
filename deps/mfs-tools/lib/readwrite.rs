#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
extern crate libc;
extern "C" {
    #[no_mangle]
    fn tivo_partition_size(file: *mut tpFILE) -> uint64_t;
    #[no_mangle]
    fn tivo_partition_offset(file: *mut tpFILE) -> uint64_t;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
}
pub type uint64_t = libc::c_ulonglong;
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
pub type tpFILE = tivo_partition_file;
/* Some quick routines, mainly intended for internal macpart use. */
#[inline]
unsafe extern "C" fn _tivo_partition_fd(mut file: *mut tpFILE)
 -> libc::c_int {
    return (*file).fd;
}
#[inline]
unsafe extern "C" fn _tivo_partition_swab(mut file: *mut tpFILE)
 -> libc::c_int {
    return (((*file).tptype as libc::c_uint ==
                 pDIRECT as libc::c_int as libc::c_uint ||
                 (*file).tptype as libc::c_uint ==
                     pDIRECTFILE as libc::c_int as libc::c_uint) &&
                0 != (*(*file).extra.direct.pt).vol_flags & 0x4i32) as
               libc::c_int;
}
// #ifdef HAVE_ERRNO_H
// #endif
/* #include "mfs.h" */
/* ********************************************/
/* Preform byte-swapping in a block of data. */
#[no_mangle]
pub unsafe extern "C" fn data_swab(mut data: *mut libc::c_void,
                                   mut size: libc::c_int) {
    let mut idata: *mut libc::c_uint = data as *mut libc::c_uint;
    /* Do it 32 bits at a time if possible. */
    while size > 3i32 {
        *idata =
            *idata << 8i32 & 0xff00ff00u32 | (*idata & 0xff00ff00u32) >> 8i32;
        size -= 4i32;
        idata = idata.offset(1isize)
    }
    /* Swap the odd out bytes.  If theres a final odd out, just ignore it. */
	/* Probably not the best solution for data integrity, but thats okay, */
	/* this should never happen. */
    if size > 1i32 {
        let mut cdata: *mut libc::c_uchar = idata as *mut libc::c_uchar;
        let mut tmp: libc::c_uchar = 0;
        tmp = *cdata.offset(0isize);
        *cdata.offset(0isize) = *cdata.offset(1isize);
        *cdata.offset(1isize) = tmp
    };
}