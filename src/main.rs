extern crate clap;
extern crate positioned_io;

use clap::{App, Arg};

use std::fs::File;

use std::io::prelude::*;

use positioned_io::ReadAt;

const TIVO_BOOT_MAGIC: u16 = 0x1492;
const TIVO_BOOT_AMIGC: u16 = 0x9214;
const APM_SIGNATURE: u16 = 0x504d;
const APM_BLOCK_SIZE: u16 = 512;

struct Parition {
    name: &'static str,
    r#type: &'static str,
}

struct ApplePartitionMap {
    partitions: Vec<Parition>,
}

struct TivoDrive {
    partition_map: ApplePartitionMap,
}

fn bytes_to_short(first_byte: u8, second_byte: u8) -> u16 {
    (u16::from(first_byte) << 8) | u16::from(second_byte)
}


fn open_tivo_image(path: &str) -> Result<(), &'static str> {
    println!("Reading Tivo Harddrive Disk Image");
    let mut file = File::open(path).expect("Couldn't open image");

    let mut buffer = [0; 2];
    file.read_exact(&mut buffer)
        .expect("Could not read first two bytes from file");

    let is_byte_swapped: bool;

    match bytes_to_short(buffer[0], buffer[1]) {
        TIVO_BOOT_MAGIC => {
            println!("Disk Image is in Correct Order! (Drive is LittleEndian)");
            is_byte_swapped = false;
        }
        TIVO_BOOT_AMIGC => {
            println!("Disk Image is Byte Swapped! (Drive is BigEndian)");
            is_byte_swapped = true;
        }
        _ => {
            println!("I don't think this is a TiVo disk image");
            return Err("Not a TiVo Drive");
        }
    }

    // The first block on a TiVo drive contain special TiVo magic,
    //  we're not worried about this for reconstructing the partition map.
    //  The partition entry describing the partition map should be in the second block (offet: 512)
    let mut partition_map_buffer = [0; APM_BLOCK_SIZE as usize];
    file.read_at(512, &mut partition_map_buffer)
        .expect("Could not read block containing partition map");

    match bytes_to_short(buffer[0], buffer[1]) {
        APM_SIGNATURE => {
            println!("Valid APM Partition entry in the second block!");
        },
        _ => {
            println!("Second block does not contain valid APM Partition entry");
            return Err("Invalid Block at Offset 512")
        }
    }

    return Ok(());
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
    println!("Using input file: {}", input_path);

    open_tivo_image(input_path);
}
