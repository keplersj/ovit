extern crate nom;
extern crate ovit_util;

use super::{MFSINodeIter, MFSVolumes};
use log::warn;
use nom::{bytes::streaming::tag, error::ErrorKind, number::streaming::be_u32, Err, IResult};
use ovit_util::get_blocks_from_file;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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

#[derive(Debug, Clone)]
pub struct MFSZone {
    pub sector: u64,
    pub backup_sector: u64,
    pub zonemap_size: u32,
    pub next_zonemap_ptr: u64,
    pub backup_next_zonemap_ptr: u64,
    pub next_zonemap_size: u32,
    pub next_zonemap_partition_size: u32,
    pub next_zonemap_min_allocation: u32,
    pub logstamp: u32,
    pub r#type: MFSZoneType,
    pub checksum: u32,
    pub first_sector: u64,
    pub last_sector: u64,
    pub size: u32,
    pub min_allocations: u32,
    pub free_space: u32,
    pub bitmap_num: u32,
}

impl MFSZone {
    fn parse(input: &[u8]) -> IResult<&[u8], MFSZone> {
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
            MFSZone {
                sector: u64::from(sector),
                backup_sector: u64::from(backup_sector),
                zonemap_size,
                next_zonemap_ptr: u64::from(next_zonemap_ptr),
                backup_next_zonemap_ptr: u64::from(backup_next_zonemap_ptr),
                next_zonemap_size,
                next_zonemap_partition_size,
                next_zonemap_min_allocation,
                r#type,
                logstamp,
                checksum,
                first_sector: u64::from(first_sector),
                last_sector: u64::from(last_sector),
                size,
                min_allocations,
                free_space,
                bitmap_num,
            },
        ))
    }

    fn from_file_at_sector(
        path: &str,
        sector: u64,
        backup_sector: u64,
        size: usize,
        is_byte_swapped: bool,
    ) -> Result<MFSZone, String> {
        let zonemap_bytes = &match get_blocks_from_file(
            path,
            sector,
            // size,
            1,
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

        match MFSZone::parse(&zonemap_bytes) {
            Ok((_, zonemap)) => Ok(zonemap),
            Err(_) => {
                warn!("Couldn't load zonemap, trying backup");
                let backup_zonemap_bytes = &match get_blocks_from_file(
                    path,
                    backup_sector,
                    // size,
                    1,
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
                match MFSZone::parse(&backup_zonemap_bytes) {
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

#[derive(Debug, Clone)]
pub struct MFSZoneMap {
    source_file_path: String,
    is_source_byte_swapped: bool,

    next_zonemap_ptr: u64,
    backup_next_zonemap_ptr: u64,
    next_zonemap_size: u32,

    volumes: MFSVolumes,
}

impl MFSZoneMap {
    pub fn new(
        path: &str,
        volumes: &MFSVolumes,
        sector: u64,
        backup_sector: u64,
        size: usize,
        is_byte_swapped: bool,
    ) -> Result<MFSZoneMap, String> {
        Ok(MFSZoneMap {
            source_file_path: path.to_string(),
            is_source_byte_swapped: is_byte_swapped,

            next_zonemap_ptr: sector,
            backup_next_zonemap_ptr: backup_sector,
            next_zonemap_size: size as u32,

            volumes: volumes.clone(),
        })
    }

    pub fn inode_iter(&mut self) -> Result<MFSINodeIter, String> {
        let inode_zone = match self.find(|node| node.r#type == MFSZoneType::INode) {
            Some(node_zone) => node_zone,
            None => {
                return Err("Could not load inode zone".to_string());
            }
        };

        Ok(MFSINodeIter {
            source_file_path: String::from(&self.source_file_path),
            partition_starting_sector: self
                .volumes
                .find_sector_volume(inode_zone.first_sector)
                .disk_sector
                .into(),
            is_source_byte_swapped: self.is_source_byte_swapped,

            next_inode_sector: inode_zone.first_sector,
            last_inode_sector: inode_zone.last_sector,
        })
    }

    pub fn inode_count(&mut self) -> u32 {
        self.filter(|zone| zone.r#type == MFSZoneType::INode)
            .map(|zone| zone.size)
            .sum::<u32>()
            / 2
    }
}

impl Iterator for MFSZoneMap {
    type Item = MFSZone;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_zonemap_ptr != 0 {
            let zonemap = match MFSZone::from_file_at_sector(
                &self.source_file_path,
                self.volumes
                    .clone()
                    .sector_to_disk_location(self.next_zonemap_ptr),
                self.volumes
                    .clone()
                    .sector_to_disk_location(self.backup_next_zonemap_ptr),
                self.next_zonemap_size as usize,
                self.is_source_byte_swapped,
            ) {
                Ok(map) => map,
                Err(_) => {
                    return None;
                }
            };

            self.next_zonemap_ptr = zonemap.next_zonemap_ptr;
            self.next_zonemap_size = zonemap.next_zonemap_size;
            self.backup_next_zonemap_ptr = zonemap.backup_next_zonemap_ptr;

            Some(zonemap)
        } else {
            None
        }
    }
}
