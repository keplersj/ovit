extern crate positioned_io;

use std::convert::TryInto;

use std::fs::File;

use std::io::prelude::*;

use std::ops::RangeInclusive;

use std::vec::Vec;

use positioned_io::ReadAt;

pub const TIVO_BOOT_MAGIC: u16 = 0x1492;
pub const TIVO_BOOT_AMIGC: u16 = 0x9214;
pub const APM_BLOCK_SIZE: usize = 512;
pub const MFS32_HEADER_MAGIC: u32 = 0xABBA_FEED;
pub const MFS64_HEADER_MAGIC: u32 = 0xEBBA_FEED;

pub fn get_string_from_bytes_range(
    bytes: &[u8],
    range: RangeInclusive<usize>,
) -> Result<String, String> {
    match String::from_utf8(match bytes.get(range) {
        Some(vec) => vec.to_vec(),
        _ => {
            return Err("Could not get bytes".to_string());
        }
    }) {
        Ok(string) => Ok(string.to_string()),
        Err(err) => Err(format!("Could not convert bytes to string: {:#X?}", err)),
    }
}

pub fn get_u32_from_bytes_range(bytes: &[u8], range: RangeInclusive<usize>) -> Result<u32, String> {
    Ok(u32::from_be_bytes(
        match bytes.get(range) {
            Some(bytes) => bytes,
            _ => return Err("Could not get bytes from range".to_string()),
        }
        .try_into()
        .unwrap(),
    ))
}

pub fn correct_byte_order(raw_buffer: &[u8], is_byte_swapped: bool) -> Vec<u8> {
    raw_buffer
        .chunks_exact(2)
        .map(|chunk| u16::from_be_bytes(chunk[0..2].try_into().unwrap()))
        .map(|byte| {
            if is_byte_swapped {
                byte
            } else {
                byte.swap_bytes()
            }
        })
        .flat_map(|byte| -> Vec<u8> { byte.to_ne_bytes().to_vec() })
        .collect()
}

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

#[derive(Debug)]
pub struct Partition {
    pub signature: String,
    pub partitions_total: u32,
    pub starting_sector: u32,
    pub sector_size: u32,
    pub name: String,
    pub r#type: String,
    pub starting_data_sector: u32,
    pub data_sectors: u32,
    pub status: String,
}

impl Partition {
    pub fn new(bytes: Vec<u8>) -> Result<Partition, &'static str> {
        let signature =
            get_string_from_bytes_range(&bytes, 0..=1).expect("Could not get signature from bytes");

        if signature != "PM" {
            return Err("Invalid signature in sector");
        }

        let partitions_total =
            get_u32_from_bytes_range(&bytes, 4..=7).expect("Could not get partitions total");

        let starting_sector =
            get_u32_from_bytes_range(&bytes, 8..=11).expect("Could not get starting sector");

        let sector_size =
            get_u32_from_bytes_range(&bytes, 12..=15).expect("Could not get sector size");

        let name = get_string_from_bytes_range(&bytes, 16..=47)
            .expect("Could not get name from bytes")
            .trim_matches(char::from(0))
            .to_string();

        let r#type = get_string_from_bytes_range(&bytes, 48..=79)
            .expect("Could not get type from bytes")
            .trim_matches(char::from(0))
            .to_string();

        let starting_data_sector =
            get_u32_from_bytes_range(&bytes, 80..=83).expect("Could not get starting data sector");

        let data_sectors =
            get_u32_from_bytes_range(&bytes, 84..=87).expect("Could not get data sectors");

        let status = format!(
            "{:#X}",
            get_u32_from_bytes_range(&bytes, 88..=91).expect("Could not get status")
        );

        Ok(Partition {
            signature,
            partitions_total,
            starting_sector,
            sector_size,
            name,
            r#type,
            starting_data_sector,
            data_sectors,
            status,
        })
    }
}

#[derive(Debug)]
pub struct ApplePartitionMap {
    pub partitions: Vec<Partition>,
}

#[derive(Debug)]
pub struct MFSVolumeHeader {
    pub state: u32,
    pub magic: String,
    pub checksum: u32,
    pub root_fsid: u32,
    pub firstpartsize: u32,
    pub partitionlist: String,
    pub total_sectors: u32,
    pub zonemap_ptr: u32,
    pub backup_zonemap_ptr: u32,
    pub zonemap_size: u32,
    pub next_fsid: u32,
}

impl MFSVolumeHeader {
    pub fn from_bytes(block: &[u8]) -> MFSVolumeHeader {
        MFSVolumeHeader {
            state: get_u32_from_bytes_range(block, 0..=3).expect("Could not get state"),
            magic: format!(
                "{:X}",
                get_u32_from_bytes_range(block, 4..=7).expect("Could not get magic")
            ),
            checksum: get_u32_from_bytes_range(block, 8..=11).expect("Could not get checksum"),
            root_fsid: get_u32_from_bytes_range(block, 16..=19).expect("Could not get root fsid"),
            firstpartsize: get_u32_from_bytes_range(block, 20..=23)
                .expect("Could not get first partition size"),
            partitionlist: get_string_from_bytes_range(block, 36..=163)
                .expect("Could not get device list from bytes")
                .trim_matches(char::from(0))
                .to_string(),
            total_sectors: get_u32_from_bytes_range(block, 164..=167)
                .expect("Could not get total sectors"),
            zonemap_ptr: get_u32_from_bytes_range(block, 196..=199)
                .expect("Could not get zonemap pointer"),
            backup_zonemap_ptr: get_u32_from_bytes_range(block, 200..=203)
                .expect("Could not get backup zonemap pointer"),
            zonemap_size: get_u32_from_bytes_range(block, 204..=207)
                .expect("Could not get zonemap size"),
            next_fsid: get_u32_from_bytes_range(block, 216..=219).expect("Could not get next fsid"),
        }
    }
}

#[derive(Debug)]
pub enum MFSZoneType {
    INode,
    Application,
    Media,
    Max,
    Unknown(u32),
}

impl MFSZoneType {
    pub fn from_u32(n: u32) -> MFSZoneType {
        match n {
            0 => MFSZoneType::INode,
            1 => MFSZoneType::Application,
            2 => MFSZoneType::Media,
            3 => MFSZoneType::Max,
            _ => MFSZoneType::Unknown(n),
        }
    }
}

#[derive(Debug)]
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
    pub fn from_bytes(block: &[u8]) -> MFSZoneMap {
        MFSZoneMap {
            sector: get_u32_from_bytes_range(block, 0..=3).expect("Could not get sector"),
            backup_sector: get_u32_from_bytes_range(block, 4..=7)
                .expect("Could not get backup sector"),
            zonemap_size: get_u32_from_bytes_range(block, 8..=11)
                .expect("Could not get zonemap size"),
            next_zonemap_ptr: get_u32_from_bytes_range(block, 12..=15)
                .expect("Could not get next zonemap pointer"),
            backup_next_zonemap_ptr: get_u32_from_bytes_range(block, 16..=19)
                .expect("Could not get backup new zonemap pointer"),
            next_zonemap_size: get_u32_from_bytes_range(block, 20..=23)
                .expect("Could not get next zonemap size"),
            next_zonemap_partition_size: get_u32_from_bytes_range(block, 24..=27)
                .expect("Could not get next zonemap partition size"),
            next_zonemap_min_allocation: get_u32_from_bytes_range(block, 28..=31)
                .expect("Could not get next zonemap minimum allocation"),
            r#type: MFSZoneType::from_u32(
                get_u32_from_bytes_range(block, 32..=35).expect("Could not get type"),
            ),
            logstamp: get_u32_from_bytes_range(block, 36..=39).expect("Could not get logstamp"),
            checksum: get_u32_from_bytes_range(block, 40..=43).expect("Could not get checksum"),
            first_sector: get_u32_from_bytes_range(block, 44..=47)
                .expect("Could not get first sector"),
            last_sector: get_u32_from_bytes_range(block, 48..=51)
                .expect("Could not get last sector"),
            size: get_u32_from_bytes_range(block, 52..=55).expect("Could not get size"),
            min_allocations: get_u32_from_bytes_range(block, 56..=59)
                .expect("Could not get minimum allocations"),
            free_space: get_u32_from_bytes_range(block, 60..=63).expect("Could not get free space"),
            bitmap_num: get_u32_from_bytes_range(block, 68..=71)
                .expect("Could not get bitmap number"),
        }
    }
}

#[derive(Debug)]
pub struct TivoDrive {
    pub partition_map: ApplePartitionMap,
    pub volume_header: MFSVolumeHeader,
    pub zones: Vec<MFSZoneMap>,
}

impl TivoDrive {
    pub fn from_disk_image(path: &str) -> Result<TivoDrive, &'static str> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(_) => {
                return Err("Couldn't open image");
            }
        };

        let mut buffer = [0; 2];
        match file.read_exact(&mut buffer) {
            Ok(_) => {}
            Err(_) => {
                return Err("Could not read first two bytes from file");
            }
        };

        let is_byte_swapped = match u16::from_be_bytes(buffer[0..2].try_into().unwrap()) {
            TIVO_BOOT_MAGIC => false,
            TIVO_BOOT_AMIGC => true,
            _ => {
                return Err("Not a TiVo Drive");
            }
        };

        // The first block on a TiVo drive contain special TiVo magic,
        //  we're not worried about this for reconstructing the partition map.
        //  The partition entry describing the partition map should be in the second block (offet: 512)
        let driver_descriptor_buffer = match get_block_from_drive(&file, 1) {
            Ok(buffer) => correct_byte_order(&buffer, is_byte_swapped),
            Err(_) => {
                return Err("Could not read block containing partition map");
            }
        };

        let driver_descriptor_map = Partition::new(driver_descriptor_buffer)
            .expect("Could not reconstruct Driver Descriptor Map");

        let mut partitions = vec![driver_descriptor_map];

        for offset in 2..=partitions.get(0).unwrap().partitions_total {
            let partition_buffer = match get_block_from_drive(&file, u64::from(offset)) {
                Ok(buffer) => correct_byte_order(&buffer, is_byte_swapped),
                Err(_) => {
                    return Err("Could not read block containing partition map");
                }
            };

            match Partition::new(partition_buffer) {
                Ok(partition) => {
                    partitions.push(partition);
                }
                Err(err) => {
                    return Err(err);
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

        let volume_header = MFSVolumeHeader::from_bytes(&app_region_block);

        let first_zonemap_block = correct_byte_order(
            &get_block_from_drive(
                &file,
                u64::from(app_region.starting_sector + volume_header.zonemap_ptr),
            )
            .unwrap(),
            is_byte_swapped,
        );

        let first_zonemap = MFSZoneMap::from_bytes(&first_zonemap_block);

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

            let zonemap = MFSZoneMap::from_bytes(&zonemap_bytes);

            next_zone_ptr = zonemap.next_zonemap_ptr;
            next_zone_size = zonemap.next_zonemap_size;

            zones.push(zonemap);
        }

        Ok(TivoDrive {
            partition_map,
            volume_header,
            zones,
        })
    }
}
