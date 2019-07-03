extern crate clap;
extern crate positioned_io;

use clap::{App, Arg};

use std::convert::TryInto;

use std::fs::File;

use std::io::prelude::*;

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

impl Partition {
    fn new(bytes: Vec<u8>) -> Result<Partition, &'static str> {
        let signature = String::from_utf8(
            bytes
                .get(0..2)
                .expect("Could not get signature bytes from partition entry")
                .to_vec(),
        )
        .expect("Could not get signature from partition entry")
        .to_string();

        if signature != "PM" {
            return Err("Invalid signature in sector");
        }

        let partitions_total = u32::from_be_bytes(
            bytes
                .get(4..8)
                .expect("Could not get partitions total from partition entry")
                .try_into()
                .unwrap(),
        );

        let starting_sector = u32::from_be_bytes(
            bytes
                .get(8..12)
                .expect("Could not get starting sector from partition entry")
                .try_into()
                .unwrap(),
        );

        let sector_size = u32::from_be_bytes(
            bytes
                .get(12..16)
                .expect("Could not get sector size from partition entry")
                .try_into()
                .unwrap(),
        );

        let name = String::from_utf8(
            bytes
                .get(16..48)
                .expect("Could not get name bytes from partition entry")
                .to_vec(),
        )
        .expect("Could not get name from partition entry")
        .trim_matches(char::from(0))
        .to_string();

        let r#type = String::from_utf8(
            bytes
                .get(48..80)
                .expect("Could not get type bytes from partition entry")
                .to_vec(),
        )
        .expect("Could not get type from partition entry")
        .trim_matches(char::from(0))
        .to_string();

        let starting_data_sector = u32::from_be_bytes(
            bytes
                .get(80..84)
                .expect("Could not get the sector where data begins from partition entry")
                .try_into()
                .unwrap(),
        );

        let data_sectors = u32::from_be_bytes(
            bytes
                .get(84..88)
                .expect("Could not get sector size of data from partition entry")
                .try_into()
                .unwrap(),
        );

        let status = format!(
            "{:#X}",
            u32::from_be_bytes(
                bytes
                    .get(88..92)
                    .expect("Could not get status from partition entry")
                    .try_into()
                    .unwrap(),
            )
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
struct ApplePartitionMap {
    partitions: Vec<Partition>,
}

#[derive(Debug)]
struct TivoDrive {
    partition_map: ApplePartitionMap,
}

fn open_tivo_image(path: &str) -> Result<TivoDrive, &'static str> {
    let mut file = File::open(path).expect("Couldn't open image");

    let mut buffer = [0; 2];
    file.read_exact(&mut buffer)
        .expect("Could not read first two bytes from file");

    let is_byte_swapped: bool;

    match u16::from_be_bytes(buffer[0..2].try_into().unwrap()) {
        TIVO_BOOT_MAGIC => {
            // println!("Disk Image is in Correct Order! (Drive is LittleEndian)");
            is_byte_swapped = false;
        }
        TIVO_BOOT_AMIGC => {
            // println!("Disk Image is Byte Swapped! (Drive is BigEndian)");
            is_byte_swapped = true;
        }
        _ => {
            // println!("I don't think this is a TiVo disk image");
            return Err("Not a TiVo Drive");
        }
    }

    // The first block on a TiVo drive contain special TiVo magic,
    //  we're not worried about this for reconstructing the partition map.
    //  The partition entry describing the partition map should be in the second block (offet: 512)
    let mut partition_map_buffer = [0; APM_BLOCK_SIZE];
    file.read_exact_at(512, &mut partition_map_buffer)
        .expect("Could not read block containing partition map");

    let partition_map_partition: Vec<u8> = partition_map_buffer
        .chunks_exact(2)
        .map(|chunk| u16::from_be_bytes(chunk[0..2].try_into().unwrap()))
        .map(|byte| match is_byte_swapped {
            true => byte,
            false => byte.swap_bytes(),
        })
        .flat_map(|byte| -> Vec<u8> { byte.to_ne_bytes().to_vec() })
        .collect();

    let driver_descriptor_map = Partition::new(partition_map_partition)
        .expect("Could not reconstruct Driver Descriptor Map");

    let mut partitions = vec![driver_descriptor_map];

    for offset in 2..=partitions.get(0).unwrap().partitions_total {
        let mut raw_partition_buffer = [0; APM_BLOCK_SIZE];
        file.read_exact_at(512 * offset as u64, &mut raw_partition_buffer)
            .expect("Could not read block containing partition map");

        let partition_buffer: Vec<u8> = raw_partition_buffer
            .chunks_exact(2)
            .map(|chunk| u16::from_be_bytes(chunk[0..2].try_into().unwrap()))
            .map(|byte| match is_byte_swapped {
                true => byte,
                false => byte.swap_bytes(),
            })
            .flat_map(|byte| -> Vec<u8> { byte.to_ne_bytes().to_vec() })
            .collect();

        let partition = Partition::new(partition_buffer).expect(&format!(
            "Could not reconstruct partition at offset {}",
            512 * offset
        ));

        partitions.push(partition);
    }

    return Ok(TivoDrive {
        partition_map: ApplePartitionMap { partitions },
    });
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
