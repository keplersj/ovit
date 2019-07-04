mod ovit;

extern crate clap;

use clap::{App, Arg};

use std::fs::File;

#[derive(Debug)]
struct MFSVolumeHeader {
    state: u32,
    magic: String,
    checksum: u32,
    root_fsid: u32,
    firstpartsize: u32,
    partitionlist: String,
    total_sectors: u32,
    zonemap_ptr: u32,
    backup_zonemap_ptr: u32,
    zonemap_size: u32,
    next_fsid: u32
}

fn main() {
    let matches = App::new("oViT")
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

    let tivo_image =
        ovit::TivoDrive::from_disk_image(input_path).expect("Could not open TiVo Drive Image");

    let file = File::open(input_path).expect("Couldn't open image");

    let app_region = tivo_image
        .partition_map
        .partitions
        .iter()
        .find(|partition| partition.r#type == "MFS")
        .unwrap();

    println!("{:#?}", app_region);

    let app_region_block = ovit::correct_byte_order(
        &ovit::get_block_from_drive(&file, u64::from(app_region.starting_sector)).unwrap(),
        true,
    );

    println!("{:X?}", app_region_block);

    let header = MFSVolumeHeader {
        state: ovit::get_u32_from_bytes_range(&app_region_block, 0..=3),
        magic: format!(
            "{:X}",
            ovit::get_u32_from_bytes_range(&app_region_block, 4..=7)
        ),
        checksum: ovit::get_u32_from_bytes_range(&app_region_block, 8..=11),
        root_fsid: ovit::get_u32_from_bytes_range(&app_region_block, 16..=19),
        firstpartsize: ovit::get_u32_from_bytes_range(&app_region_block, 20..=23),
        partitionlist: ovit::get_string_from_bytes_range(&app_region_block, 36..=163)
            .expect("Could not get device list from bytes")
            .trim_matches(char::from(0))
            .to_string(),
        total_sectors: ovit::get_u32_from_bytes_range(&app_region_block, 164..=167),
        zonemap_ptr: ovit::get_u32_from_bytes_range(&app_region_block, 196..=199),
        backup_zonemap_ptr: ovit::get_u32_from_bytes_range(&app_region_block, 200..=203),
        zonemap_size: ovit::get_u32_from_bytes_range(&app_region_block, 204..=207),
        next_fsid: ovit::get_u32_from_bytes_range(&app_region_block, 216..=219),
    };

    println!("{:#?}", header);
}
