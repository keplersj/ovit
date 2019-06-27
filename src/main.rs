extern crate clap;
extern crate pbr;

use clap::{App, Arg, SubCommand};

use pbr::{ProgressBar, Units};

use std::fs::File;

use std::io::prelude::*;

const TIVO_BOOT_MAGIC: u16 = 0x1492;
const TIVO_BOOT_AMIGC: u16 = 0x9214;

fn bytes_to_short(first_byte: u8, second_byte: u8) -> u16 {
    (u16::from(first_byte) << 8) | u16::from(second_byte)
}

fn create_byte_swapped_image() {
    println!("Creating Byte Swapped Disk Image");

    let mut file = File::open("/Volumes/External/tivo_hdd.iso").expect("Couldn't open image");
    let mut swapped_file =
        File::create("/Volumes/External/tivo_hdd_swapped.iso").expect("Couldn't create image");

    let file_bytes_len = file.metadata().expect("Could not get image metadata").len();

    let mut pb = ProgressBar::new(file_bytes_len);
    pb.set_units(Units::Bytes);

    // Create a One Megabyte Buffer to hold swapped bytes in
    let mut buffer = [0u8; 1_048_576]; // 1024 * 1024 * 2 = 1,048,576

    while let Ok(bytes_read) = file.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        let swapped_buffer = buffer
            .chunks(2)
            .flat_map(|chunk| vec![chunk[1], chunk[0]])
            .collect::<Vec<u8>>();

        let bytes_written = swapped_file
            .write(&swapped_buffer)
            .expect("Could not write to new file");

        pb.add(bytes_written as u64);
    }

    pb.finish_print("Swapped file created!");
}


fn old_main() {
    println!("Reading Tivo Harddrive Disk Image");
    let mut file = File::open("/Volumes/External/tivo_hdd.iso").expect("Couldn't open image");

    let mut buffer = [0; 2];
    file.read_exact(&mut buffer)
        .expect("Could not read first two bytes from file");

    match bytes_to_short(buffer[0], buffer[1]) {
        TIVO_BOOT_MAGIC => println!("Disk Image is in Correct Order!"),
        TIVO_BOOT_AMIGC => {
            println!("Disk Image is Byte Swapped!");
            println!("Going to create byte swapped image now.");
            create_byte_swapped_image();
        }
        _ => println!("I don't think this is a Tivo disk image"),
    }
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
    println!("Using input file: {}", matches.value_of("INPUT").unwrap());
}
