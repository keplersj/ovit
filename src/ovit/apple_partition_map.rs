#[path = "util.rs"]
mod util;

use util::{get_string_from_bytes_range, get_u32_from_bytes_range};

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
