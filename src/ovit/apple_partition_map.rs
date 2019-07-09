extern crate nom;

use nom::{
    bytes::complete::{tag, take},
    error::ErrorKind,
    number::complete::be_u32,
    Err, IResult,
};

#[derive(Debug, PartialEq)]
pub struct Partition {
    pub partitions_total: u32,
    pub starting_sector: u32,
    pub sector_size: u32,
    pub name: String,
    pub r#type: String,
    pub starting_data_sector: u32,
    pub data_sectors: u32,
    pub status: u32,
}

fn string(input: &[u8]) -> IResult<&[u8], String> {
    let (input, str_bytes) = take(32 as usize)(input)?;
    match String::from_utf8(str_bytes.to_vec()) {
        Ok(string) => Ok((input, string.trim_matches(char::from(0)).to_string())),
        Err(_) => Err(Err::Error((input, ErrorKind::ParseTo))),
    }
}

impl Partition {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Partition> {
        let (input, _) = tag("PM")(input)?;
        let (input, _) = take(2 as usize)(input)?;
        let (input, partitions_total) = be_u32(input)?;
        let (input, starting_sector) = be_u32(input)?;
        let (input, sector_size) = be_u32(input)?;
        let (input, name) = string(input)?;
        let (input, r#type) = string(input)?;
        let (input, starting_data_sector) = be_u32(input)?;
        let (input, data_sectors) = be_u32(input)?;
        let (input, status) = be_u32(input)?;

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
            },
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct ApplePartitionMap {
    pub partitions: Vec<Partition>,
}
