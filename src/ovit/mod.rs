pub mod apple_partition_map;
pub mod media_file_system;
pub mod util;

use apple_partition_map::ApplePartitionMap;
use media_file_system::{MFSINode, MFSVolumeHeader, MFSZoneMap, MFSZoneType};
use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;
use util::get_block_from_drive_and_correct_order;

pub const TIVO_BOOT_MAGIC: u16 = 0x1492;
pub const TIVO_BOOT_AMIGC: u16 = 0x9214;

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
                return Err("Couldn't open drive".to_string());
            }
        };

        let is_byte_swapped = check_byte_order(&mut file)?;

        let partition_map = ApplePartitionMap::read_from_file(&mut file, is_byte_swapped)?;

        let app_region = partition_map
            .partitions
            .iter()
            .find(|partition| partition.r#type == "MFS")
            .unwrap();

        let volume_header =
            MFSVolumeHeader::from_partition(app_region, &mut file, is_byte_swapped)?;

        let first_zonemap = MFSZoneMap::from_file_at_sector(
            &mut file,
            u64::from(app_region.starting_sector + volume_header.next_zonemap_sector),
            u64::from(app_region.starting_sector + volume_header.next_zonemap_backup_sector),
            volume_header.next_zonemap_partition_size as usize,
            is_byte_swapped,
        )?;

        let mut zones = vec![first_zonemap];

        let mut next_zone_ptr = zones[0].next_zonemap_ptr;
        let mut next_zone_backup = zones[0].backup_next_zonemap_ptr;
        let mut next_zone_size = zones[0].next_zonemap_size;

        while next_zone_ptr != 0 {
            let zonemap = match MFSZoneMap::from_file_at_sector(
                &mut file,
                u64::from(app_region.starting_sector + next_zone_ptr),
                u64::from(app_region.starting_sector + next_zone_backup),
                next_zone_size as usize,
                is_byte_swapped,
            ) {
                Ok(map) => map,
                Err(err) => {
                    println!("{:?}", err);
                    println!("Continuing anyway");
                    break;
                }
            };

            next_zone_ptr = zonemap.next_zonemap_ptr;
            next_zone_backup = zonemap.backup_next_zonemap_ptr;
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
                let block =
                    get_block_from_drive_and_correct_order(&mut file, disk_sector, is_byte_swapped)
                        .expect("Could not get block from drive");
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
