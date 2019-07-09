extern crate nom;
extern crate positioned_io;

mod apple_partition_map;

pub mod util;

use apple_partition_map::{ApplePartitionMap, Partition};

use nom::{
    bytes::complete::tag, bytes::complete::take, error::ErrorKind, number::complete::be_u16,
    number::complete::be_u32, number::complete::be_u8, Err, IResult,
};

use positioned_io::ReadAt;

use std::convert::TryInto;

use std::fs::File;

use std::io::prelude::*;

use std::vec::Vec;

use util::correct_byte_order;

pub const TIVO_BOOT_MAGIC: u16 = 0x1492;
pub const TIVO_BOOT_AMIGC: u16 = 0x9214;
pub const APM_BLOCK_SIZE: usize = 512;

pub fn get_block_from_drive(file: &File, location: u64) -> Result<Vec<u8>, String> {
    get_blocks_from_drive(file, location, 1)
}

pub fn get_blocks_from_drive(file: &File, location: u64, count: usize) -> Result<Vec<u8>, String> {
    let mut buffer = vec![0; APM_BLOCK_SIZE * count];

    match file.read_at(location * APM_BLOCK_SIZE as u64, &mut buffer) {
        Ok(_) => Ok(buffer),
        Err(_) => Err(format!(
            "Could not read block from file at location {}",
            location
        )),
    }
}

fn string(input: &[u8]) -> IResult<&[u8], String> {
    let (input, str_bytes) = take(128 as usize)(input)?;
    match String::from_utf8(str_bytes.to_vec()) {
        Ok(string) => Ok((input, string.trim_matches(char::from(0)).to_string())),
        Err(_) => Err(Err::Error((input, ErrorKind::ParseTo))),
    }
}

#[derive(Debug, PartialEq)]
pub struct MFSVolumeHeader {
    pub state: u32,
    pub checksum: u32,
    pub root_fsid: u32,
    pub firstpartsize: u32,
    pub partitionlist: String,
    pub total_sectors: u32,
    pub next_zonemap_sector: u32,
    pub next_zonemap_backup_sector: u32,
    pub next_zonemap_partition_size: u32,
    pub next_fsid: u32,
}

impl MFSVolumeHeader {
    pub fn parse(input: &[u8]) -> IResult<&[u8], MFSVolumeHeader> {
        let (input, state) = be_u32(input)?;
        let (input, _) = tag([0xAB, 0xBA, 0xFE, 0xED])(input)?;
        let (input, checksum) = be_u32(input)?;
        let (input, _) = take(4 as usize)(input)?;
        let (input, root_fsid) = be_u32(input)?;
        let (input, _) = take(4 as usize)(input)?;
        let (input, firstpartsize) = be_u32(input)?;
        let (input, _) = take(4 as usize)(input)?;
        let (input, _) = take(4 as usize)(input)?;
        let (input, partitionlist) = string(input)?;
        let (input, total_sectors) = be_u32(input)?;
        let (input, _) = take(4 as usize)(input)?;
        let (input, _logstart) = be_u32(input)?;
        let (input, _lognsectors) = be_u32(input)?;
        let (input, _volhdrlogstamp) = be_u32(input)?;
        let (input, _unkstart) = be_u32(input)?;
        let (input, _unksectors) = be_u32(input)?;
        let (input, _unkstamp) = be_u32(input)?;
        let (input, next_zonemap_sector) = be_u32(input)?;
        let (input, next_zonemap_backup_sector) = be_u32(input)?;
        let (input, _next_zonemap_sector_length) = be_u32(input)?;
        let (input, next_zonemap_partition_size) = be_u32(input)?;
        let (input, _next_zonemap_min_allocation) = be_u32(input)?;
        let (input, next_fsid) = be_u32(input)?;
        let (input, _bootcycles) = be_u32(input)?;
        let (input, _bootsecs) = be_u32(input)?;
        let (input, _) = take(4 as usize)(input)?;

        Ok((
            input,
            MFSVolumeHeader {
                state,
                checksum,
                root_fsid,
                firstpartsize,
                partitionlist,
                total_sectors,
                next_zonemap_sector,
                next_zonemap_backup_sector,
                next_zonemap_partition_size,
                next_fsid,
            },
        ))
    }
}

#[derive(Debug, PartialEq)]
pub enum MFSZoneType {
    INode,
    Application,
    Media,
    Max,
    Unknown(u32),
}

impl MFSZoneType {
    pub fn parse(input: &[u8]) -> IResult<&[u8], MFSZoneType> {
        let (input, n) = be_u32(input)?;
        match n {
            0 => Ok((input, MFSZoneType::INode)),
            1 => Ok((input, MFSZoneType::Application)),
            2 => Ok((input, MFSZoneType::Media)),
            3 => Ok((input, MFSZoneType::Max)),
            _ => Ok((input, MFSZoneType::Unknown(n))),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct MFSZoneMap {
    pub sector: u32,
    pub backup_sector: u32,
    pub zonemap_size: u32,
    pub next_zonemap_ptr: u32,
    pub backup_next_zonemap_ptr: u32,
    pub next_zonemap_size: u32,
    pub next_zonemap_partition_size: u32,
    pub next_zonemap_min_allocation: u32,
    pub logstamp: u32,
    pub r#type: MFSZoneType,
    pub checksum: u32,
    pub first_sector: u32,
    pub last_sector: u32,
    pub size: u32,
    pub min_allocations: u32,
    pub free_space: u32,
    pub bitmap_num: u32,
}

impl MFSZoneMap {
    pub fn parse(input: &[u8]) -> IResult<&[u8], MFSZoneMap> {
        let (input, sector) = be_u32(input)?;
        let (input, backup_sector) = be_u32(input)?;
        let (input, zonemap_size) = be_u32(input)?;
        let (input, next_zonemap_ptr) = be_u32(input)?;
        let (input, backup_next_zonemap_ptr) = be_u32(input)?;
        let (input, next_zonemap_size) = be_u32(input)?;
        let (input, next_zonemap_partition_size) = be_u32(input)?;
        let (input, next_zonemap_min_allocation) = be_u32(input)?;
        let (input, r#type) = MFSZoneType::parse(input)?;
        let (input, logstamp) = be_u32(input)?;
        let (input, checksum) = be_u32(input)?;
        let (input, first_sector) = be_u32(input)?;
        let (input, last_sector) = be_u32(input)?;
        let (input, size) = be_u32(input)?;
        let (input, min_allocations) = be_u32(input)?;
        let (input, free_space) = be_u32(input)?;
        let (input, _) = take(4 as usize)(input)?;
        let (input, bitmap_num) = be_u32(input)?;

        Ok((
            input,
            MFSZoneMap {
                sector,
                backup_sector,
                zonemap_size,
                next_zonemap_ptr,
                backup_next_zonemap_ptr,
                next_zonemap_size,
                next_zonemap_partition_size,
                next_zonemap_min_allocation,
                r#type,
                logstamp,
                checksum,
                first_sector,
                last_sector,
                size,
                min_allocations,
                free_space,
                bitmap_num,
            },
        ))
    }
}

#[derive(Debug, PartialEq)]
pub enum MFSINodeType {
    Node,
    File,
    Stream,
    Dir,
    Db,
    Unknown(u8),
}

impl MFSINodeType {
    pub fn parse(input: &[u8]) -> IResult<&[u8], MFSINodeType> {
        let (input, n) = be_u8(input)?;
        match n {
            0 => Ok((input, MFSINodeType::Node)),
            1 => Ok((input, MFSINodeType::File)),
            2 => Ok((input, MFSINodeType::Stream)),
            4 => Ok((input, MFSINodeType::Dir)),
            8 => Ok((input, MFSINodeType::Db)),
            _ => Ok((input, MFSINodeType::Unknown(n))),
        }
    }
}

#[derive(Debug)]
pub struct MFSINode {
    pub fsid: u32,
    pub refcount: u32,
    pub bootcycles: u32,
    pub bootsecs: u32,
    pub inode: u32,
    pub unk3: u32,
    pub size: u32,
    pub blocksize: u32,
    pub blockused: u32,
    pub last_modified: u32,
    pub r#type: MFSINodeType,
    pub zone: u8,
    pub pad: u16,
    pub checksum: u32,
    pub flags: u32,
    pub numblocks: u32,
    pub data_block_sector: u32,
    pub data_block_count: u32,
}

impl MFSINode {
    pub fn parse(input: &[u8]) -> IResult<&[u8], MFSINode> {
        let (input, fsid) = be_u32(input)?;
        let (input, refcount) = be_u32(input)?;
        let (input, bootcycles) = be_u32(input)?;
        let (input, bootsecs) = be_u32(input)?;
        let (input, inode) = be_u32(input)?;
        let (input, unk3) = be_u32(input)?;
        let (input, size) = be_u32(input)?;
        let (input, blocksize) = be_u32(input)?;
        let (input, blockused) = be_u32(input)?;
        let (input, last_modified) = be_u32(input)?;
        let (input, r#type) = MFSINodeType::parse(input)?;
        let (input, zone) = be_u8(input)?;
        let (input, pad) = be_u16(input)?;
        let (input, _sig) = be_u32(input)?;
        let (input, checksum) = be_u32(input)?;
        let (input, flags) = be_u32(input)?;
        let (input, numblocks) = be_u32(input)?;
        let (input, data_block_sector) = be_u32(input)?;
        let (input, data_block_count) = be_u32(input)?;

        Ok((
            input,
            MFSINode {
                fsid,
                refcount,
                bootcycles,
                bootsecs,
                inode,
                unk3,
                size,
                blocksize,
                blockused,
                last_modified,
                r#type,
                zone,
                pad,
                checksum,
                flags,
                numblocks,
                data_block_sector,
                data_block_count,
            },
        ))
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
        //  The partition entry describing the partition map should be in the second block (offet: 512)
        let driver_descriptor_buffer = match get_block_from_drive(&file, 1) {
            Ok(buffer) => correct_byte_order(&buffer, is_byte_swapped),
            Err(_) => {
                return Err("Could not read block containing partition map".to_string());
            }
        };

        let (_, driver_descriptor_map) = Partition::parse(&driver_descriptor_buffer)
            .expect("Could not reconstruct Driver Descriptor Map");

        let mut partitions = vec![driver_descriptor_map];

        for offset in 2..=partitions.get(0).unwrap().partitions_total {
            let partition_buffer = match get_block_from_drive(&file, u64::from(offset)) {
                Ok(buffer) => correct_byte_order(&buffer, is_byte_swapped),
                Err(_) => {
                    return Err("Could not read block containing partition map".to_string());
                }
            };

            match Partition::parse(&partition_buffer) {
                Ok((_, partition)) => {
                    partitions.push(partition);
                }
                Err(err) => {
                    return Err(format!("Error parsing partition: {:?}", err));
                }
            }
        }

        let partition_map = ApplePartitionMap { partitions };

        let app_region = partition_map
            .partitions
            .iter()
            .find(|partition| partition.r#type == "MFS")
            .unwrap();

        let app_region_block = correct_byte_order(
            &get_block_from_drive(&file, u64::from(app_region.starting_sector)).unwrap(),
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
                &file,
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
                    &file,
                    u64::from(app_region.starting_sector + next_zone_ptr),
                    next_zone_size as usize,
                ) {
                    Ok(blocks) => blocks,
                    Err(_) => {
                        println!(
                            "Couldn't load zonemap blocks at ptr and size: {}, {}",
                            next_zone_ptr, next_zone_size
                        );
                        println!("Lazilly continuing");
                        break;
                    }
                },
                is_byte_swapped,
            );

            let zonemap = match MFSZoneMap::parse(&zonemap_bytes) {
                Ok((_, zonemap)) => zonemap,
                Err(err) => {
                    return Err(format!("Could not parse zonemap: {:?}", err));
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
                    &get_block_from_drive(&file, disk_sector)
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
