extern crate clap;
extern crate positioned_io;

use clap::{App, Arg};

use std::convert::TryInto;

use std::fs::File;

use std::io::prelude::*;

use std::ops::RangeInclusive;

use std::vec::Vec;

use positioned_io::ReadAt;

const TIVO_BOOT_MAGIC: u16 = 0x1492;
const TIVO_BOOT_AMIGC: u16 = 0x9214;
const APM_BLOCK_SIZE: usize = 512;

#[derive(Debug)]
struct Partition {
    signature: String,
    partitions_total: u32,
    starting_sector: u32,
    sector_size: u32,
    name: String,
    r#type: String,
    starting_data_sector: u32,
    data_sectors: u32,
    status: String,
}

fn get_string_from_bytes_range(bytes: &[u8], range: RangeInclusive<usize>) -> String {
    String::from_utf8(
        bytes
            .get(range)
            .expect("Could not get signature bytes from partition entry")
            .to_vec(),
    )
    .expect("Could not get signature from partition entry")
    .to_string()
}

fn get_u32_from_bytes_range(bytes: &[u8], range: RangeInclusive<usize>) -> u32 {
    u32::from_be_bytes(
        bytes
            .get(range)
            .expect("Could not get partitions total from partition entry")
            .try_into()
            .unwrap(),
    )
}

impl Partition {
    fn new(bytes: Vec<u8>) -> Result<Partition, &'static str> {
        let signature = get_string_from_bytes_range(&bytes, 0..=1);

        if signature != "PM" {
            return Err("Invalid signature in sector");
        }

        let partitions_total = get_u32_from_bytes_range(&bytes, 4..=7);

        let starting_sector = get_u32_from_bytes_range(&bytes, 8..=11);

        let sector_size = get_u32_from_bytes_range(&bytes, 12..=15);

        let name = get_string_from_bytes_range(&bytes, 16..=47)
            .trim_matches(char::from(0))
            .to_string();

        let r#type = get_string_from_bytes_range(&bytes, 48..=79)
            .trim_matches(char::from(0))
            .to_string();

        let starting_data_sector = get_u32_from_bytes_range(&bytes, 80..=83);

        let data_sectors = get_u32_from_bytes_range(&bytes, 84..=87);

        let status = format!("{:#X}", get_u32_from_bytes_range(&bytes, 88..=91));

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
struct ApplePartitionMap {
    partitions: Vec<Partition>,
}

#[derive(Debug)]
struct TivoDrive {
    partition_map: ApplePartitionMap,
}

fn open_tivo_image(path: &str) -> Result<TivoDrive, &'static str> {
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
    let mut partition_map_buffer = [0; APM_BLOCK_SIZE];
    match file.read_exact_at(512, &mut partition_map_buffer) {
        Ok(_) => {}
        Err(_) => {
            return Err("Could not read block containing partition map");
        }
    }

    let partition_map_partition: Vec<u8> = partition_map_buffer
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
        .collect();

    let driver_descriptor_map = Partition::new(partition_map_partition)
        .expect("Could not reconstruct Driver Descriptor Map");

    let mut partitions = vec![driver_descriptor_map];

    for offset in 2..=partitions.get(0).unwrap().partitions_total {
        let mut raw_partition_buffer = [0; APM_BLOCK_SIZE];
        file.read_exact_at(512 * u64::from(offset), &mut raw_partition_buffer)
            .expect("Could not read block containing partition map");

        let partition_buffer: Vec<u8> = raw_partition_buffer
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
            .collect();

        match Partition::new(partition_buffer) {
            Ok(partition) => {
                partitions.push(partition);
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    Ok(TivoDrive {
        partition_map: ApplePartitionMap { partitions },
    })
}

fn main() {
    let matches = App::new("TiVo MFS Experiment")
        .version("0.0.0-dev")
        .author("Kepler Sticka-Jones <kepler@stickajones.org>")
        .about("An experimental binary to retrieve MPEG streams from a TiVo hard drive (image) and do other TiVo drive related things.")
        .arg(Arg::with_name("INPUT")
            .help("The drive image to read from")
            .required(true))
        .get_matches();

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    let input_path = matches.value_of("INPUT").unwrap();

    let tivo_image = open_tivo_image(input_path).expect("Could not open TiVo Drive Image");
    println!("{:#?}", tivo_image);
}
