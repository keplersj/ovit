extern crate chrono;
extern crate nom;

use chrono::{DateTime, TimeZone, Utc};
use nom::{
    bytes::streaming::{tag, take},
    error::ErrorKind,
    number::streaming::{be_u16, be_u32, be_u8},
    Err, IResult,
};

#[derive(Debug, PartialEq, Eq)]
pub enum MFSINodeType {
    Node = 0,
    File = 1,
    Stream = 2,
    Dir = 4,
    Db = 8,
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
            _ => Err(Err::Error((input, ErrorKind::NoneOf))),
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
    pub size: u32,
    pub blocksize: u32,
    pub blockused: u32,
    pub last_modified: DateTime<Utc>,
    pub r#type: MFSINodeType,
    pub zone: u8,
    pub checksum: u32,
    pub flags: u32,
    pub data: Vec<u8>,
    pub numblocks: u32,
    pub data_block_sector: u32,
    pub data_block_count: u32,
}

const INODE_DATA_IN_HEADER: u32 = 0x4000_0000;

impl MFSINode {
    pub fn parse(input: &[u8]) -> IResult<&[u8], MFSINode> {
        let (input, fsid) = be_u32(input)?;
        let (input, refcount) = be_u32(input)?;
        let (input, bootcycles) = be_u32(input)?;
        let (input, bootsecs) = be_u32(input)?;
        let (input, inode) = be_u32(input)?; // Should be (sectornum - 1122) / 2
        let (input, _) = take(4 as usize)(input)?;
        let (input, size) = be_u32(input)?;
        let (input, blocksize) = be_u32(input)?;
        let (input, blockused) = be_u32(input)?;
        let (input, last_modified) = be_u32(input)?;
        let (input, r#type) = MFSINodeType::parse(input)?;
        let (input, zone) = be_u8(input)?;
        let (input, _pad) = be_u16(input)?;
        let (input, _sig) = tag([0x91, 0x23, 0x1e, 0xbc])(input)?;
        let (input, checksum) = be_u32(input)?;
        let (input, flags) = be_u32(input)?;
        let (input, data) = if flags == INODE_DATA_IN_HEADER {
            let data = input.to_vec();
            let input: &[u8] = &[];
            (input, data)
        } else {
            (input, vec![])
        };
        let (input, numblocks) = if flags == INODE_DATA_IN_HEADER {
            (input, 0)
        } else {
            be_u32(input)?
        };
        let (input, data_block_sector) = if flags == INODE_DATA_IN_HEADER {
            (input, 0)
        } else {
            be_u32(input)?
        };
        let (input, data_block_count) = if flags == INODE_DATA_IN_HEADER {
            (input, 0)
        } else {
            be_u32(input)?
        };

        Ok((
            input,
            MFSINode {
                fsid,
                refcount,
                bootcycles,
                bootsecs,
                inode,
                size,
                blocksize,
                blockused,
                last_modified: Utc.timestamp(i64::from(last_modified), 0),
                r#type,
                zone,
                checksum,
                flags,
                data,
                numblocks,
                data_block_sector,
                data_block_count,
            },
        ))
    }
}