mod ovit;

extern crate clap;
extern crate ovit_util;
#[macro_use]
extern crate prettytable;
extern crate tivo_media_file_system;

use clap::{App, Arg, SubCommand};
use ovit_util::get_blocks_from_file;
use prettytable::Table;
use std::convert::TryInto;

fn main() {
    let matches = App::new("oViT")
        .version("0.0.0-dev")
        .author("Kepler Sticka-Jones <kepler@stickajones.org>")
        .about("An experimental binary to retrieve MPEG streams from a TiVo hard drive (image) and do other TiVo drive related things.")
        .subcommand(SubCommand::with_name("info").arg(Arg::with_name("INPUT")
            .help("The drive image to read from")
            .required(true)))
        .subcommand(SubCommand::with_name("partitions").arg(Arg::with_name("INPUT")
            .help("The drive image to read from")
            .required(true)))
        .get_matches();

    match matches.subcommand() {
        ("info", Some(sub_match)) => {
            // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
            // required we could have used an 'if let' to conditionally get the value)
            let input_path = sub_match.value_of("INPUT").unwrap();

            println!("Loading TiVo Drive");

            let mut tivo_drive =
                ovit::TivoDrive::from_disk_image(input_path).expect("Could not load TiVo drive");

            println!("TiVo Drive Loaded!");

            println!();

            println!("Source: {}", input_path);
            println!(
                "Partitions Count: {}",
                tivo_drive.partition_map.partitions.len()
            );
            println!(
                "INode Count: {}",
                tivo_drive.zonemap.inode_iter().unwrap().len()
            );
        }
        ("partitions", Some(sub_match)) => {
            // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
            // required we could have used an 'if let' to conditionally get the value)
            let input_path = sub_match.value_of("INPUT").unwrap();

            let tivo_drive =
                ovit::TivoDrive::from_disk_image(input_path).expect("Could not load TiVo drive");

            // Create the table
            let mut table = Table::new();

            table.add_row(row![
                "Partition Total",
                "Starting Sector",
                "Sector Size",
                "Name",
                "Type",
                "Starting Data Sector",
                "Data Sectors",
                "Status"
            ]);
            for partition in tivo_drive.partition_map.partitions {
                table.add_row(row![
                    partition.partitions_total,
                    partition.starting_sector,
                    partition.sector_size,
                    partition.name,
                    partition.r#type,
                    partition.starting_data_sector,
                    partition.data_sectors,
                    format!("{:#08X}", partition.status)
                ]);
            }

            // Print the table to stdout
            table.printstd();
        }
        ("experiment", Some(sub_match)) => {
            // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
            // required we could have used an 'if let' to conditionally get the value)
            let input_path = sub_match.value_of("INPUT").unwrap();

            println!("Loading TiVo Drive");

            let mut tivo_drive =
                ovit::TivoDrive::from_disk_image(input_path).expect("Could not load TiVo drive");

            println!("TiVo Drive Loaded!");

            println!();

            let inode_sample: Vec<tivo_media_file_system::MFSINode> = tivo_drive
                .zonemap
                .inode_iter()
                .unwrap()
                .filter(|inode| inode.r#type == tivo_media_file_system::MFSINodeType::Dir)
                .filter(|inode| !inode.datablocks.is_empty())
                .take(5)
                .collect();

            println!("{:#?}", inode_sample);

            let block = get_blocks_from_file(
                &input_path,
                u64::from(
                    inode_sample[0].partition_starting_sector
                        + inode_sample[0].datablocks[0].sector,
                ),
                inode_sample[0].datablocks[0].count as usize,
                true,
            )
            .unwrap();

            let block32: Vec<u32> = block
                .chunks(4)
                .map(|chunk| u32::from_be_bytes(chunk.try_into().unwrap()))
                .collect();

            println!("{:X?}", block32);
            println!("{}", String::from_utf8_lossy(&block));
        }
        _ => {}
    }
}
