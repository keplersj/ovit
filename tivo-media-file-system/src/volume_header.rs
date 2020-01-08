extern crate apple_partition_map;
extern crate nom;
extern crate ovit_util;

use apple_partition_map::Partition;
use nom::{
    bytes::streaming::{tag, take},
    error::ErrorKind,
    number::streaming::be_u32,
    Err, IResult,
};
use ovit_util::get_block_from_drive_and_correct_order;
use std::fs::File;

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
    fn parse(input: &[u8]) -> IResult<&[u8], MFSVolumeHeader> {
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

    pub fn from_partition(
        partition: &Partition,
        source: &mut File,
        is_byte_swapped: bool,
    ) -> Result<MFSVolumeHeader, String> {
        let block = get_block_from_drive_and_correct_order(
            source,
            u64::from(partition.starting_sector),
            is_byte_swapped,
        )?;

        match MFSVolumeHeader::parse(&block) {
            Ok((_, header)) => Ok(header),
            Err(err) => Err(format!("Could not parse volume header: {:X?}", err)),
        }
    }
}
