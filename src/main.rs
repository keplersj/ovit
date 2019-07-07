mod ovit;

extern crate clap;

use clap::{App, Arg};

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

    let node_count = tivo_image
        .inodes
        .iter()
        .filter(|inode| inode.r#type == ovit::MFSINodeType::Node)
        .count();
    let file_count = tivo_image
        .inodes
        .iter()
        .filter(|inode| inode.r#type == ovit::MFSINodeType::File)
        .count();
    let stream_count = tivo_image
        .inodes
        .iter()
        .filter(|inode| inode.r#type == ovit::MFSINodeType::Stream)
        .count();
    let dir_count = tivo_image
        .inodes
        .iter()
        .filter(|inode| inode.r#type == ovit::MFSINodeType::Dir)
        .count();
    let db_count = tivo_image
        .inodes
        .iter()
        .filter(|inode| inode.r#type == ovit::MFSINodeType::Db)
        .count();

    println!("Node Count: {:#?}", node_count);
    println!("File Count: {:#?}", file_count);
    println!("Stream Count: {:#?}", stream_count);
    println!("Dir Count: {:#?}", dir_count);
    println!("Db Count: {:#?}", db_count);
}
