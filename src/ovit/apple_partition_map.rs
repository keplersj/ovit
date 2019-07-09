extern crate nom;

use nom::{
    bytes::streaming::{tag, take},
    error::ErrorKind,
    multi::fold_many_m_n,
    number::streaming::be_u32,
    Err, IResult,
};

fn string(size: usize, input: &[u8]) -> IResult<&[u8], String> {
    let (input, str_bytes) = take(size)(input)?;
    match String::from_utf8(str_bytes.to_vec()) {
        Ok(string) => Ok((input, string.trim_matches(char::from(0)).to_string())),
        Err(_) => Err(Err::Error((input, ErrorKind::ParseTo))),
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Partition {
    pub partitions_total: u32,
    pub starting_sector: u32,
    pub sector_size: u32,
    pub name: String,
    pub r#type: String,
    pub starting_data_sector: u32,
    pub data_sectors: u32,
    pub status: u32,
    pub boot_code_starting_sector: u32,
    pub boot_code_size: u32,
    pub bootloader_address: u32,
    pub boot_code_entry_point: u32,
    pub boot_code_checksum: u32,
    pub processor_type: String,
}

impl Partition {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Partition> {
        let (input, _signature) = tag("PM")(input)?;
        let (input, _reserved) = take(2 as usize)(input)?;
        let (input, partitions_total) = be_u32(input)?;
        let (input, starting_sector) = be_u32(input)?;
        let (input, sector_size) = be_u32(input)?;
        let (input, name) = string(32 as usize, input)?;
        let (input, r#type) = string(32 as usize, input)?;
        let (input, starting_data_sector) = be_u32(input)?;
        let (input, data_sectors) = be_u32(input)?;
        let (input, status) = be_u32(input)?;
        let (input, boot_code_starting_sector) = be_u32(input)?;
        let (input, boot_code_size) = be_u32(input)?;
        let (input, bootloader_address) = be_u32(input)?;
        let (input, _reserved) = be_u32(input)?;
        let (input, boot_code_entry_point) = be_u32(input)?;
        let (input, _reserved) = be_u32(input)?;
        let (input, boot_code_checksum) = be_u32(input)?;
        let (input, processor_type) = string(16 as usize, input)?;
        let (input, _reserved) = take(376 as usize)(input)?;

        Ok((
            input,
            Partition {
                partitions_total,
                starting_sector,
                sector_size,
                name,
                r#type,
                starting_data_sector,
                data_sectors,
                status,
                boot_code_starting_sector,
                boot_code_size,
                bootloader_address,
                boot_code_entry_point,
                boot_code_checksum,
                processor_type,
            },
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct ApplePartitionMap {
    pub partitions: Vec<Partition>,
}

impl ApplePartitionMap {
    pub fn parse_from_driver_descriptor_map(input: &[u8]) -> IResult<&[u8], ApplePartitionMap> {
        let (input, partitions) = fold_many_m_n(
            1,
            64,
            Partition::parse,
            Vec::new(),
            |mut acc: Vec<Partition>, item| {
                acc.push(item);
                acc
            },
        )(input)?;

        Ok((input, ApplePartitionMap { partitions }))
    }
}
