extern crate nom;

use crate::MFSINodeType;
use nom::{
    bytes::streaming::take,
    error::ErrorKind,
    number::streaming::{be_u32, be_u8},
    Err, IResult,
};
use std::convert::TryInto;

fn string(size: usize, input: &[u8]) -> IResult<&[u8], String> {
    let (input, str_bytes) = take(size)(input)?;

    let raw_string = String::from_utf8_lossy(str_bytes).to_string();
    let split_by_null: Vec<&str> = raw_string.split('\u{0}').collect();
    let sanitized = split_by_null[0].to_string();

    Ok((input, sanitized))
}

#[derive(Debug, Clone)]
pub struct MFSEntry {
    fsid: u32,
    length: u8,
    r#type: MFSINodeType,
    name: String,
}

impl MFSEntry {
    pub fn parse(input: &[u8]) -> IResult<&[u8], MFSEntry> {
        let (input, fsid) = be_u32(input)?;

        if fsid == 0 {
            return Err(Err::Error((input, ErrorKind::ParseTo)));
        }

        let (input, length) = be_u8(input)?;

        if length == 0 {
            return Err(Err::Error((input, ErrorKind::ParseTo)));
        }

        let (input, r#type) = MFSINodeType::parse(input)?;
        let (input, name) = string((length - 6).try_into().unwrap(), input)?;

        Ok((
            input,
            MFSEntry {
                fsid,
                length,
                r#type,
                name,
            },
        ))
    }
}
