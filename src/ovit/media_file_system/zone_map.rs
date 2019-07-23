extern crate nom;

use crate::ovit::util::get_blocks_from_drive_and_correct_order;
use nom::{bytes::streaming::tag, error::ErrorKind, number::streaming::be_u32, Err, IResult};
use std::fs::File;

#[derive(Debug, PartialEq, Eq)]
pub enum MFSZoneType {
    INode = 0,
    Application = 1,
    Media = 2,
    Max = 3,
}

impl MFSZoneType {
    pub fn parse(input: &[u8]) -> IResult<&[u8], MFSZoneType> {
        let (input, n) = be_u32(input)?;
        match n {
            0 => Ok((input, MFSZoneType::INode)),
            1 => Ok((input, MFSZoneType::Application)),
            2 => Ok((input, MFSZoneType::Media)),
            3 => Ok((input, MFSZoneType::Max)),
            _ => Err(Err::Error((input, ErrorKind::NoneOf))),
        }
    }
}

#[derive(Debug)]
pub struct MFSZoneMap {
    // source_file: File,
    // starting_sector: u32,
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
    fn parse(input: &[u8]) -> IResult<&[u8], MFSZoneMap> {
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
        let (input, _) = tag([0, 0, 0, 0])(input)?;
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

    pub fn from_file_at_sector(
        file: &mut File,
        sector: u64,
        backup_sector: u64,
        size: usize,
        is_byte_swapped: bool,
    ) -> Result<MFSZoneMap, String> {
        let zonemap_bytes =
            &match get_blocks_from_drive_and_correct_order(file, sector, size, is_byte_swapped) {
                Ok(blocks) => blocks.to_vec(),
                Err(err) => {
                    return Err(format!(
                        "Couldn't load block at sector {} and size {} with error {:?}:",
                        sector, size, err
                    ));
                }
            };

        match MFSZoneMap::parse(&zonemap_bytes) {
            Ok((_, zonemap)) => Ok(zonemap),
            Err(_) => {
                println!("Couldn't load zonemap, trying backup");
                let backup_zonemap_bytes = &match get_blocks_from_drive_and_correct_order(
                    file,
                    backup_sector,
                    size,
                    is_byte_swapped,
                ) {
                    Ok(blocks) => blocks.to_vec(),
                    Err(err) => {
                        return Err(format!(
                            "Couldn't load block at sector {} and size {} with error {:?}:",
                            sector, size, err
                        ));
                    }
                };
                match MFSZoneMap::parse(&backup_zonemap_bytes) {
                    Ok((_, backup_zonemap)) => Ok(backup_zonemap),
                    Err(backup_err) => Err(format!(
                        "Couldn't parse zonemap blocks at sector {} and size {} with err {:?}:,",
                        sector, size, backup_err
                    )),
                }
            }
        }
    }
}
