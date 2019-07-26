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
}
pub type uint32_t = libc::c_uint;
#[inline]
unsafe extern "C" fn Endian32_Swap(mut var: uint32_t) -> uint32_t {
    var = var << 16i32 | var >> 16i32;
    var = (var & 0xff00ff00u32) >> 8i32 | var << 8i32 & 0xff00ff00u32;
    return var;
}
#[inline]
unsafe extern "C" fn intswap32(mut n: uint32_t) -> uint32_t {
    if mfsLSB == 0i32 { return n }
    return Endian32_Swap(n);
}
/* ANSI X3.66 CRC32 checksum */
/* CRC polynomial 0xedb88320 */
static mut crc32tab: [libc::c_ulong; 256] =
    [0i32 as libc::c_ulong, 0x77073096i32 as libc::c_ulong,
     0xee0e612cu32 as libc::c_ulong, 0x990951bau32 as libc::c_ulong,
     0x76dc419i32 as libc::c_ulong, 0x706af48fi32 as libc::c_ulong,
     0xe963a535u32 as libc::c_ulong, 0x9e6495a3u32 as libc::c_ulong,
     0xedb8832i32 as libc::c_ulong, 0x79dcb8a4i32 as libc::c_ulong,
     0xe0d5e91eu32 as libc::c_ulong, 0x97d2d988u32 as libc::c_ulong,
     0x9b64c2bi32 as libc::c_ulong, 0x7eb17cbdi32 as libc::c_ulong,
     0xe7b82d07u32 as libc::c_ulong, 0x90bf1d91u32 as libc::c_ulong,
     0x1db71064i32 as libc::c_ulong, 0x6ab020f2i32 as libc::c_ulong,
     0xf3b97148u32 as libc::c_ulong, 0x84be41deu32 as libc::c_ulong,
     0x1adad47di32 as libc::c_ulong, 0x6ddde4ebi32 as libc::c_ulong,
     0xf4d4b551u32 as libc::c_ulong, 0x83d385c7u32 as libc::c_ulong,
     0x136c9856i32 as libc::c_ulong, 0x646ba8c0i32 as libc::c_ulong,
     0xfd62f97au32 as libc::c_ulong, 0x8a65c9ecu32 as libc::c_ulong,
     0x14015c4fi32 as libc::c_ulong, 0x63066cd9i32 as libc::c_ulong,
     0xfa0f3d63u32 as libc::c_ulong, 0x8d080df5u32 as libc::c_ulong,
     0x3b6e20c8i32 as libc::c_ulong, 0x4c69105ei32 as libc::c_ulong,
     0xd56041e4u32 as libc::c_ulong, 0xa2677172u32 as libc::c_ulong,
     0x3c03e4d1i32 as libc::c_ulong, 0x4b04d447i32 as libc::c_ulong,
     0xd20d85fdu32 as libc::c_ulong, 0xa50ab56bu32 as libc::c_ulong,
     0x35b5a8fai32 as libc::c_ulong, 0x42b2986ci32 as libc::c_ulong,
     0xdbbbc9d6u32 as libc::c_ulong, 0xacbcf940u32 as libc::c_ulong,
     0x32d86ce3i32 as libc::c_ulong, 0x45df5c75i32 as libc::c_ulong,
     0xdcd60dcfu32 as libc::c_ulong, 0xabd13d59u32 as libc::c_ulong,
     0x26d930aci32 as libc::c_ulong, 0x51de003ai32 as libc::c_ulong,
     0xc8d75180u32 as libc::c_ulong, 0xbfd06116u32 as libc::c_ulong,
     0x21b4f4b5i32 as libc::c_ulong, 0x56b3c423i32 as libc::c_ulong,
     0xcfba9599u32 as libc::c_ulong, 0xb8bda50fu32 as libc::c_ulong,
     0x2802b89ei32 as libc::c_ulong, 0x5f058808i32 as libc::c_ulong,
     0xc60cd9b2u32 as libc::c_ulong, 0xb10be924u32 as libc::c_ulong,
     0x2f6f7c87i32 as libc::c_ulong, 0x58684c11i32 as libc::c_ulong,
     0xc1611dabu32 as libc::c_ulong, 0xb6662d3du32 as libc::c_ulong,
     0x76dc4190i32 as libc::c_ulong, 0x1db7106i32 as libc::c_ulong,
     0x98d220bcu32 as libc::c_ulong, 0xefd5102au32 as libc::c_ulong,
     0x71b18589i32 as libc::c_ulong, 0x6b6b51fi32 as libc::c_ulong,
     0x9fbfe4a5u32 as libc::c_ulong, 0xe8b8d433u32 as libc::c_ulong,
     0x7807c9a2i32 as libc::c_ulong, 0xf00f934i32 as libc::c_ulong,
     0x9609a88eu32 as libc::c_ulong, 0xe10e9818u32 as libc::c_ulong,
     0x7f6a0dbbi32 as libc::c_ulong, 0x86d3d2di32 as libc::c_ulong,
     0x91646c97u32 as libc::c_ulong, 0xe6635c01u32 as libc::c_ulong,
     0x6b6b51f4i32 as libc::c_ulong, 0x1c6c6162i32 as libc::c_ulong,
     0x856530d8u32 as libc::c_ulong, 0xf262004eu32 as libc::c_ulong,
     0x6c0695edi32 as libc::c_ulong, 0x1b01a57bi32 as libc::c_ulong,
     0x8208f4c1u32 as libc::c_ulong, 0xf50fc457u32 as libc::c_ulong,
     0x65b0d9c6i32 as libc::c_ulong, 0x12b7e950i32 as libc::c_ulong,
     0x8bbeb8eau32 as libc::c_ulong, 0xfcb9887cu32 as libc::c_ulong,
     0x62dd1ddfi32 as libc::c_ulong, 0x15da2d49i32 as libc::c_ulong,
     0x8cd37cf3u32 as libc::c_ulong, 0xfbd44c65u32 as libc::c_ulong,
     0x4db26158i32 as libc::c_ulong, 0x3ab551cei32 as libc::c_ulong,
     0xa3bc0074u32 as libc::c_ulong, 0xd4bb30e2u32 as libc::c_ulong,
     0x4adfa541i32 as libc::c_ulong, 0x3dd895d7i32 as libc::c_ulong,
     0xa4d1c46du32 as libc::c_ulong, 0xd3d6f4fbu32 as libc::c_ulong,
     0x4369e96ai32 as libc::c_ulong, 0x346ed9fci32 as libc::c_ulong,
     0xad678846u32 as libc::c_ulong, 0xda60b8d0u32 as libc::c_ulong,
     0x44042d73i32 as libc::c_ulong, 0x33031de5i32 as libc::c_ulong,
     0xaa0a4c5fu32 as libc::c_ulong, 0xdd0d7cc9u32 as libc::c_ulong,
     0x5005713ci32 as libc::c_ulong, 0x270241aai32 as libc::c_ulong,
     0xbe0b1010u32 as libc::c_ulong, 0xc90c2086u32 as libc::c_ulong,
     0x5768b525i32 as libc::c_ulong, 0x206f85b3i32 as libc::c_ulong,
     0xb966d409u32 as libc::c_ulong, 0xce61e49fu32 as libc::c_ulong,
     0x5edef90ei32 as libc::c_ulong, 0x29d9c998i32 as libc::c_ulong,
     0xb0d09822u32 as libc::c_ulong, 0xc7d7a8b4u32 as libc::c_ulong,
     0x59b33d17i32 as libc::c_ulong, 0x2eb40d81i32 as libc::c_ulong,
     0xb7bd5c3bu32 as libc::c_ulong, 0xc0ba6cadu32 as libc::c_ulong,
     0xedb88320u32 as libc::c_ulong, 0x9abfb3b6u32 as libc::c_ulong,
     0x3b6e20ci32 as libc::c_ulong, 0x74b1d29ai32 as libc::c_ulong,
     0xead54739u32 as libc::c_ulong, 0x9dd277afu32 as libc::c_ulong,
     0x4db2615i32 as libc::c_ulong, 0x73dc1683i32 as libc::c_ulong,
     0xe3630b12u32 as libc::c_ulong, 0x94643b84u32 as libc::c_ulong,
     0xd6d6a3ei32 as libc::c_ulong, 0x7a6a5aa8i32 as libc::c_ulong,
     0xe40ecf0bu32 as libc::c_ulong, 0x9309ff9du32 as libc::c_ulong,
     0xa00ae27i32 as libc::c_ulong, 0x7d079eb1i32 as libc::c_ulong,
     0xf00f9344u32 as libc::c_ulong, 0x8708a3d2u32 as libc::c_ulong,
     0x1e01f268i32 as libc::c_ulong, 0x6906c2fei32 as libc::c_ulong,
     0xf762575du32 as libc::c_ulong, 0x806567cbu32 as libc::c_ulong,
     0x196c3671i32 as libc::c_ulong, 0x6e6b06e7i32 as libc::c_ulong,
     0xfed41b76u32 as libc::c_ulong, 0x89d32be0u32 as libc::c_ulong,
     0x10da7a5ai32 as libc::c_ulong, 0x67dd4acci32 as libc::c_ulong,
     0xf9b9df6fu32 as libc::c_ulong, 0x8ebeeff9u32 as libc::c_ulong,
     0x17b7be43i32 as libc::c_ulong, 0x60b08ed5i32 as libc::c_ulong,
     0xd6d6a3e8u32 as libc::c_ulong, 0xa1d1937eu32 as libc::c_ulong,
     0x38d8c2c4i32 as libc::c_ulong, 0x4fdff252i32 as libc::c_ulong,
     0xd1bb67f1u32 as libc::c_ulong, 0xa6bc5767u32 as libc::c_ulong,
     0x3fb506ddi32 as libc::c_ulong, 0x48b2364bi32 as libc::c_ulong,
     0xd80d2bdau32 as libc::c_ulong, 0xaf0a1b4cu32 as libc::c_ulong,
     0x36034af6i32 as libc::c_ulong, 0x41047a60i32 as libc::c_ulong,
     0xdf60efc3u32 as libc::c_ulong, 0xa867df55u32 as libc::c_ulong,
     0x316e8eefi32 as libc::c_ulong, 0x4669be79i32 as libc::c_ulong,
     0xcb61b38cu32 as libc::c_ulong, 0xbc66831au32 as libc::c_ulong,
     0x256fd2a0i32 as libc::c_ulong, 0x5268e236i32 as libc::c_ulong,
     0xcc0c7795u32 as libc::c_ulong, 0xbb0b4703u32 as libc::c_ulong,
     0x220216b9i32 as libc::c_ulong, 0x5505262fi32 as libc::c_ulong,
     0xc5ba3bbeu32 as libc::c_ulong, 0xb2bd0b28u32 as libc::c_ulong,
     0x2bb45a92i32 as libc::c_ulong, 0x5cb36a04i32 as libc::c_ulong,
     0xc2d7ffa7u32 as libc::c_ulong, 0xb5d0cf31u32 as libc::c_ulong,
     0x2cd99e8bi32 as libc::c_ulong, 0x5bdeae1di32 as libc::c_ulong,
     0x9b64c2b0u32 as libc::c_ulong, 0xec63f226u32 as libc::c_ulong,
     0x756aa39ci32 as libc::c_ulong, 0x26d930ai32 as libc::c_ulong,
     0x9c0906a9u32 as libc::c_ulong, 0xeb0e363fu32 as libc::c_ulong,
     0x72076785i32 as libc::c_ulong, 0x5005713i32 as libc::c_ulong,
     0x95bf4a82u32 as libc::c_ulong, 0xe2b87a14u32 as libc::c_ulong,
     0x7bb12baei32 as libc::c_ulong, 0xcb61b38i32 as libc::c_ulong,
     0x92d28e9bu32 as libc::c_ulong, 0xe5d5be0du32 as libc::c_ulong,
     0x7cdcefb7i32 as libc::c_ulong, 0xbdbdf21i32 as libc::c_ulong,
     0x86d3d2d4u32 as libc::c_ulong, 0xf1d4e242u32 as libc::c_ulong,
     0x68ddb3f8i32 as libc::c_ulong, 0x1fda836ei32 as libc::c_ulong,
     0x81be16cdu32 as libc::c_ulong, 0xf6b9265bu32 as libc::c_ulong,
     0x6fb077e1i32 as libc::c_ulong, 0x18b74777i32 as libc::c_ulong,
     0x88085ae6u32 as libc::c_ulong, 0xff0f6a70u32 as libc::c_ulong,
     0x66063bcai32 as libc::c_ulong, 0x11010b5ci32 as libc::c_ulong,
     0x8f659effu32 as libc::c_ulong, 0xf862ae69u32 as libc::c_ulong,
     0x616bffd3i32 as libc::c_ulong, 0x166ccf45i32 as libc::c_ulong,
     0xa00ae278u32 as libc::c_ulong, 0xd70dd2eeu32 as libc::c_ulong,
     0x4e048354i32 as libc::c_ulong, 0x3903b3c2i32 as libc::c_ulong,
     0xa7672661u32 as libc::c_ulong, 0xd06016f7u32 as libc::c_ulong,
     0x4969474di32 as libc::c_ulong, 0x3e6e77dbi32 as libc::c_ulong,
     0xaed16a4au32 as libc::c_ulong, 0xd9d65adcu32 as libc::c_ulong,
     0x40df0b66i32 as libc::c_ulong, 0x37d83bf0i32 as libc::c_ulong,
     0xa9bcae53u32 as libc::c_ulong, 0xdebb9ec5u32 as libc::c_ulong,
     0x47b2cf7fi32 as libc::c_ulong, 0x30b5ffe9i32 as libc::c_ulong,
     0xbdbdf21cu32 as libc::c_ulong, 0xcabac28au32 as libc::c_ulong,
     0x53b39330i32 as libc::c_ulong, 0x24b4a3a6i32 as libc::c_ulong,
     0xbad03605u32 as libc::c_ulong, 0xcdd70693u32 as libc::c_ulong,
     0x54de5729i32 as libc::c_ulong, 0x23d967bfi32 as libc::c_ulong,
     0xb3667a2eu32 as libc::c_ulong, 0xc4614ab8u32 as libc::c_ulong,
     0x5d681b02i32 as libc::c_ulong, 0x2a6f2b94i32 as libc::c_ulong,
     0xb40bbe37u32 as libc::c_ulong, 0xc30c8ea1u32 as libc::c_ulong,
     0x5a05df1bi32 as libc::c_ulong, 0x2d02ef8di32 as libc::c_ulong];
/* ************************************************/
/* Compute the running CRC for a block of memory */
#[no_mangle]
pub unsafe extern "C" fn compute_crc(mut data: *mut libc::c_uchar,
                                     mut size: libc::c_uint,
                                     mut CRC: libc::c_uint) -> libc::c_uint {
    while 0 != size {
        CRC =
            (crc32tab[((CRC as libc::c_int ^ *data as libc::c_int) & 0xffi32)
                          as usize] ^
                 (CRC >> 8i32 & 0xffffffi32 as libc::c_uint) as libc::c_ulong)
                as libc::c_uint;
        data = data.offset(1isize);
        size = size.wrapping_sub(1)
    }
    return CRC;
}
/* *********************************************************************/
/* Compute the checksum, replacing the integer at off with 0xdeadf00d */
#[no_mangle]
pub unsafe extern "C" fn mfs_compute_crc(mut data: *mut libc::c_uchar,
                                         mut size: libc::c_uint,
                                         mut off: libc::c_uint)
 -> libc::c_uint {
    let mut CRC: libc::c_uint = 0i32 as libc::c_uint;
    static mut deadfood: [libc::c_uchar; 4] =
        [0xdei32 as libc::c_uchar, 0xadi32 as libc::c_uchar,
         0xf0i32 as libc::c_uchar, 0xdi32 as libc::c_uchar];
    static mut odfoadde: [libc::c_uchar; 4] =
        [0xdi32 as libc::c_uchar, 0xf0i32 as libc::c_uchar,
         0xadi32 as libc::c_uchar, 0xdei32 as libc::c_uchar];
    off =
        off.wrapping_mul(4i32 as
                             libc::c_uint).wrapping_add(3i32 as libc::c_uint);
    while 0 != size {
        if off < 4i32 as libc::c_uint {
            /* This replaces the checksum offset without actually modifying the data. */
            if mfsLSB == 0i32 {
                CRC =
                    (crc32tab[((CRC as libc::c_int ^
                                    deadfood[(3i32 as
                                                  libc::c_uint).wrapping_sub(off)
                                                 as usize] as libc::c_int) &
                                   0xffi32) as usize] ^
                         (CRC >> 8i32 & 0xffffffi32 as libc::c_uint) as
                             libc::c_ulong) as libc::c_uint
            } else {
                CRC =
                    (crc32tab[((CRC as libc::c_int ^
                                    odfoadde[(3i32 as
                                                  libc::c_uint).wrapping_sub(off)
                                                 as usize] as libc::c_int) &
                                   0xffi32) as usize] ^
                         (CRC >> 8i32 & 0xffffffi32 as libc::c_uint) as
                             libc::c_ulong) as libc::c_uint
            }
        } else {
            CRC =
                (crc32tab[((CRC as libc::c_int ^ *data as libc::c_int) &
                               0xffi32) as usize] ^
                     (CRC >> 8i32 & 0xffffffi32 as libc::c_uint) as
                         libc::c_ulong) as libc::c_uint
        }
        data = data.offset(1isize);
        size = size.wrapping_sub(1);
        off = off.wrapping_sub(1)
    }
    return intswap32(CRC);
}
/* *****************************/
/* Verify the CRC is correct. */
#[no_mangle]
pub unsafe extern "C" fn mfs_check_crc(mut data: *mut libc::c_uchar,
                                       mut size: libc::c_uint,
                                       mut off: libc::c_uint)
 -> libc::c_uint {
    let mut target: libc::c_uint =
        *(data as *mut libc::c_uint).offset(off as isize);
    return (target == mfs_compute_crc(data, size, off)) as libc::c_int as
               libc::c_uint;
}
/* ************************/
/* Make the CRC correct. */
#[no_mangle]
pub unsafe extern "C" fn mfs_update_crc(mut data: *mut libc::c_uchar,
                                        mut size: libc::c_uint,
                                        mut off: libc::c_uint) {
    *(data as *mut libc::c_uint).offset(off as isize) =
        mfs_compute_crc(data, size, off);
}