mod legacy;

pub mod apple_partition_map;

pub mod media_file_system;

pub mod util;

use apple_partition_map::ApplePartitionMap;

use media_file_system::{MFSINode, MFSVolumeHeader, MFSZoneMap, MFSZoneType};

use std::convert::TryInto;

use std::fs::File;

use std::io::prelude::*;

use std::io::SeekFrom;

use std::vec::Vec;

use util::correct_byte_order;

pub const TIVO_BOOT_MAGIC: u16 = 0x1492;
pub const TIVO_BOOT_AMIGC: u16 = 0x9214;
pub const APM_BLOCK_SIZE: usize = 512;

pub fn get_block_from_drive(file: &mut File, location: u64) -> Result<Vec<u8>, String> {
    get_blocks_from_drive(file, location, 1)
}

pub fn get_blocks_from_drive(
    file: &mut File,
    location: u64,
    count: usize,
) -> Result<Vec<u8>, String> {
    let mut buffer = vec![0; APM_BLOCK_SIZE * count];

    match file.seek(SeekFrom::Start(location * APM_BLOCK_SIZE as u64)) {
        Ok(_) => {}
        Err(_) => {
            return Err(format!(
                "Could not set file cursor to location {}",
                location
            ));
        }
    };

    match file.read(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(_) => Err(format!(
            "Could not read block from file at location {}",
            location
        )),
    }
}

#[derive(Debug)]
pub struct TivoDrive {
    pub source_file: File,
    pub partition_map: ApplePartitionMap,
    pub volume_header: MFSVolumeHeader,
    pub zones: Vec<MFSZoneMap>,
    pub inodes: Vec<MFSINode>,
}

impl TivoDrive {
    pub fn from_disk_image(path: &str) -> Result<TivoDrive, String> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(_) => {
                return Err("Couldn't open image".to_string());
            }
        };

        let mut buffer = [0; 2];
        match file.read_exact(&mut buffer) {
            Ok(_) => {}
            Err(_) => {
                return Err("Could not read first two bytes from file".to_string());
            }
        };

        let is_byte_swapped = match u16::from_be_bytes(buffer[0..2].try_into().unwrap()) {
            TIVO_BOOT_MAGIC => false,
            TIVO_BOOT_AMIGC => true,
            _ => {
                return Err("Not a TiVo Drive".to_string());
            }
        };

        // The first block on a TiVo drive contain special TiVo magic,
        //  we're not worried about this for reconstructing the partition map.
        let partition_map_buffer = match get_blocks_from_drive(&mut file, 1, 64) {
            Ok(buffer) => correct_byte_order(&buffer, is_byte_swapped),
            Err(_) => {
                return Err("Could not read block containing partition map".to_string());
            }
        };

        let partition_map =
            match ApplePartitionMap::parse_from_driver_descriptor_map(&partition_map_buffer) {
                Ok((_, partition_map)) => partition_map,
                Err(err) => {
                    return Err(format!("Could not parse partition map: {:?}", err));
                }
            };

        let app_region = partition_map
            .partitions
            .iter()
            .find(|partition| partition.r#type == "MFS")
            .unwrap();

        let app_region_block = correct_byte_order(
            &get_block_from_drive(&mut file, u64::from(app_region.starting_sector)).unwrap(),
            is_byte_swapped,
        );

        let volume_header = match MFSVolumeHeader::parse(&app_region_block) {
            Ok((_, header)) => header,
            Err(err) => {
                return Err(format!("Could not parse volume header: {:X?}", err));
            }
        };

        let first_zonemap_block = correct_byte_order(
            &get_block_from_drive(
                &mut file,
                u64::from(app_region.starting_sector + volume_header.next_zonemap_sector),
            )
            .unwrap(),
            is_byte_swapped,
        );

        let first_zonemap = match MFSZoneMap::parse(&first_zonemap_block) {
            Ok((_, zonemap)) => zonemap,
            Err(err) => {
                return Err(format!("Could not parse zonemap: {:?}", err));
            }
        };

        let mut zones = vec![first_zonemap];

        let mut next_zone_ptr = zones[0].next_zonemap_ptr;
        let mut next_zone_size = zones[0].next_zonemap_size;

        while next_zone_ptr != 0 {
            let zonemap_bytes = correct_byte_order(
                &match get_blocks_from_drive(
                    &mut file,
                    u64::from(app_region.starting_sector + next_zone_ptr),
                    next_zone_size as usize,
                ) {
                    Ok(blocks) => blocks,
                    Err(_) => {
                        println!(
                            "Couldn't load block at sector {} and size {}:",
                            next_zone_ptr, next_zone_size
                        );
                        break;
                    }
                },
                is_byte_swapped,
            );

            let zonemap = match MFSZoneMap::parse(&zonemap_bytes) {
                Ok((_, zonemap)) => zonemap,
                Err(_err) => {
                    println!(
                        "Couldn't parse zonemap blocks at sector {} and size {}:,",
                        next_zone_ptr, next_zone_size
                    );
                    // println!("Parser error: {:?}", err);
                    println!("Lazilly continuing");
                    break;
                }
            };

            next_zone_ptr = zonemap.next_zonemap_ptr;
            next_zone_size = zonemap.next_zonemap_size;

            zones.push(zonemap);
        }

        let inode_zone = zones
            .iter()
            .find(|zone| zone.r#type == MFSZoneType::INode)
            .unwrap();

        let mut inodes: Vec<MFSINode> = vec![];

        for sector in 0..inode_zone.size {
            // Every inode is stored twice, only load every other
            if sector == 0 || sector % 2 == 0 {
                let disk_sector =
                    u64::from(app_region.starting_sector + inode_zone.first_sector + sector);
                let block = correct_byte_order(
                    &get_block_from_drive(&mut file, disk_sector)
                        .expect("Could not get block from drive"),
                    is_byte_swapped,
                );
                let inode = match MFSINode::parse(&block) {
                    Ok((_, inode)) => inode,
                    Err(err) => {
                        return Err(format!("Could not parse inode: {:?}", err));
                    }
                };

                if inode.fsid != 0 {
                    inodes.push(inode);
                }
            }
        }

        Ok(TivoDrive {
            source_file: file,
            partition_map,
            volume_header,
            zones,
            inodes,
        })
    }
}
