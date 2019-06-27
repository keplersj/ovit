extern crate clap;

use clap::{App, Arg};

use std::fs::File;

use std::io::prelude::*;

const TIVO_BOOT_MAGIC: u16 = 0x1492;
const TIVO_BOOT_AMIGC: u16 = 0x9214;

fn bytes_to_short(first_byte: u8, second_byte: u8) -> u16 {
    (u16::from(first_byte) << 8) | u16::from(second_byte)
}


fn open_tivo_image(path: &str) {
    println!("Reading Tivo Harddrive Disk Image");
    let mut file = File::open(path).expect("Couldn't open image");

    let mut buffer = [0; 2];
    file.read_exact(&mut buffer)
        .expect("Could not read first two bytes from file");

    match bytes_to_short(buffer[0], buffer[1]) {
        TIVO_BOOT_MAGIC => println!("Disk Image is in Correct Order!"),
        TIVO_BOOT_AMIGC => println!("Disk Image is Byte Swapped!"),
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
    let input_path = matches.value_of("INPUT").unwrap();
    println!("Using input file: {}", input_path);

    open_tivo_image(input_path);
}
