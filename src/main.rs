extern crate pbr;

use pbr::{ProgressBar, Units};

use std::fs::File;

use std::io::prelude::*;

const TIVO_BOOT_MAGIC: u16 = 0x1492;
const TIVO_BOOT_AMIGC: u16 = 0x9214;

fn bytes_to_short(first_byte: u8, second_byte: u8) -> u16 {
    ((first_byte as u16) << 8) | second_byte as u16
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

        swapped_file
            .write(&swapped_buffer)
            .expect("Could not write to new file");

        pb.add(swapped_buffer.len() as u64);
    }

    pb.finish_print("Swapped file created!");
}


fn main() {
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
