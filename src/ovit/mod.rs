pub mod apple_partition_map;
pub mod media_file_system;
pub mod util;

use apple_partition_map::ApplePartitionMap;
use media_file_system::{MFSVolumeHeader, MFSZoneMap};
use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;

pub const TIVO_BOOT_MAGIC: u16 = 0x1492;
pub const TIVO_BOOT_AMIGC: u16 = 0x9214;

#[derive(Debug)]
pub struct TivoDrive {
    pub source_file: File,
    pub partition_map: ApplePartitionMap,
    pub volume_header: MFSVolumeHeader,
    pub zonemap: MFSZoneMap,
}

impl TivoDrive {
    fn check_byte_order(file: &mut File) -> Result<bool, String> {
        let mut buffer = [0; 2];
        match file.read_exact(&mut buffer) {
            Ok(_) => {}
            Err(_) => {
                return Err("Could not read first two bytes from file".to_string());
            }
        };

        match u16::from_be_bytes(buffer[0..2].try_into().unwrap()) {
            TIVO_BOOT_MAGIC => Ok(false),
            TIVO_BOOT_AMIGC => Ok(true),
            _ => Err("Not a TiVo Drive".to_string()),
        }
    }

    pub fn from_disk_image(path: &str) -> Result<TivoDrive, String> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(_) => {
                return Err("Couldn't open drive".to_string());
            }
        };

        let is_byte_swapped = TivoDrive::check_byte_order(&mut file)?;

        let partition_map = ApplePartitionMap::read_from_file(&mut file, is_byte_swapped)?;

        let app_region = partition_map
            .partitions
            .iter()
            .find(|partition| partition.r#type == "MFS")
            .unwrap();

        let volume_header =
            MFSVolumeHeader::from_partition(app_region, &mut file, is_byte_swapped)?;

        let zonemap = MFSZoneMap::new(
            path,
            app_region.starting_sector,
            volume_header.next_zonemap_sector,
            volume_header.next_zonemap_backup_sector,
            volume_header.next_zonemap_partition_size as usize,
            is_byte_swapped,
        )?;

        Ok(TivoDrive {
            source_file: file,
            partition_map,
            volume_header,
            zonemap,
        })
    }
}
