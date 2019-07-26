#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast)]
extern crate libc;
extern "C" {
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char)
     -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
               _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
}
pub type mainfunc
    =
    Option<unsafe extern "C" fn(_: libc::c_int, _: *mut *mut libc::c_char)
               -> libc::c_int>;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed {
    pub name: *mut libc::c_char,
    pub main: mainfunc,
    pub desc: *mut libc::c_char,
}
#[no_mangle]
pub static mut funcs: [C2RustUnnamed; 1] =
    [C2RustUnnamed{name: 0 as *const libc::c_char as *mut libc::c_char,
                   main: None,
                   desc: 0 as *const libc::c_char as *mut libc::c_char,}];
#[no_mangle]
pub unsafe extern "C" fn find_function(mut name: *mut libc::c_char)
 -> mainfunc {
    let mut loop_0: libc::c_int = 0;
    loop_0 = 0i32;
    while !funcs[loop_0 as usize].name.is_null() {
        if 0 == strcasecmp(funcs[loop_0 as usize].name, name) {
            return funcs[loop_0 as usize].main
        }
        loop_0 += 1
    }
    if 0 ==
           strncmp(name, b"mfs\x00" as *const u8 as *const libc::c_char,
                   3i32 as libc::c_ulong) {
        return find_function(name.offset(3isize))
    }
    return None;
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    let mut toolmain: mainfunc = None;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut loop_0: libc::c_int = 0;
    tmp = strrchr(*argv.offset(0isize), '/' as i32);
    tmp =
        if !tmp.is_null() {
            tmp.offset(1isize)
        } else { *argv.offset(0isize) };
    toolmain = find_function(tmp);
    if toolmain.is_some() {
        return toolmain.expect("non-null function pointer")(argc, argv)
    }
    if argc > 1i32 &&
           {
               toolmain = find_function(*argv.offset(1isize));
               toolmain.is_some()
           } {
        return toolmain.expect("non-null function pointer")(argc - 1i32,
                                                            argv.offset(1isize))
    }
    return 1i32;
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}