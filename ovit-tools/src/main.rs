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
        .subcommand(SubCommand::with_name("zones").arg(Arg::with_name("INPUT")
            .help("The drive image to read from")
            .required(true)))
        .subcommand(SubCommand::with_name("header").arg(Arg::with_name("INPUT")
            .help("The drive image to read from")
            .required(true)))
        .subcommand(SubCommand::with_name("inodes")
            .arg(Arg::with_name("INPUT")
                .help("The drive image to read from")
                .required(true))
            .arg(Arg::with_name("count")
                .short("c")
                .long("count")
                .value_name("NUMBER")
                .help("Sets how many INodes to read")
                .takes_value(true)
                .required(false)
                .default_value("25")))
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
        ("zones", Some(sub_match)) => {
            // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
            // required we could have used an 'if let' to conditionally get the value)
            let input_path = sub_match.value_of("INPUT").unwrap();

            let tivo_drive =
                ovit::TivoDrive::from_disk_image(input_path).expect("Could not load TiVo drive");

            // Create the table
            let mut table = Table::new();

            table.add_row(row![
                "Sector",
                "Backup Sector",
                "Zonemap Size",
                "Next Zonemap Pointer",
                "Backup Next Zonemap Pointer",
                "Next Zonemap Size",
                "Next Zonemap Partition Size",
                "Next Zonemap Min. Allocation",
                "Logstamp",
                "Type",
                "Checksum",
                "First Sector",
                "Last Sector",
                "Size",
                "Min. Allocations",
                "Free Space",
                "Bitmap Number"
            ]);
            for zone in tivo_drive.zonemap {
                table.add_row(row![
                    zone.sector,
                    zone.backup_sector,
                    zone.zonemap_size,
                    zone.next_zonemap_ptr,
                    zone.backup_next_zonemap_ptr,
                    zone.next_zonemap_size,
                    zone.next_zonemap_partition_size,
                    zone.next_zonemap_min_allocation,
                    zone.logstamp,
                    format!("{:#?}", zone.r#type),
                    zone.checksum,
                    zone.first_sector,
                    zone.last_sector,
                    zone.size,
                    zone.min_allocations,
                    zone.free_space,
                    zone.bitmap_num
                ]);
            }
        }
        ("header", Some(sub_match)) => {
            // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
            // required we could have used an 'if let' to conditionally get the value)
            let input_path = sub_match.value_of("INPUT").unwrap();

            let tivo_drive =
                ovit::TivoDrive::from_disk_image(input_path).expect("Could not load TiVo drive");

            // Create the table
            let mut table = Table::new();

            let header = tivo_drive.volume_header;

            table.add_row(row!["Variable", "Value"]);
            table.add_row(row!["State", header.state]);
            table.add_row(row!["Checksum", header.checksum]);
            table.add_row(row!["Root FSID", header.root_fsid]);
            table.add_row(row!["First Partition Size", header.firstpartsize]);
            table.add_row(row!["Partition List", header.partitionlist]);
            table.add_row(row!["Total Sectors", header.total_sectors]);
            table.add_row(row!["Zonemap Sector", header.next_zonemap_sector]);
            table.add_row(row![
                "Zonemap Backup Sector",
                header.next_zonemap_backup_sector
            ]);
            table.add_row(row![
                "Zonemap Partition Size",
                header.next_zonemap_partition_size
            ]);
            table.add_row(row!["Next FSID", header.next_fsid]);

            // Print the table to stdout
            table.printstd();
        }

        ("inodes", Some(sub_match)) => {
            // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
            // required we could have used an 'if let' to conditionally get the value)
            let input_path = sub_match.value_of("INPUT").unwrap();
            let inode_count: usize = sub_match.value_of("count").unwrap().parse().unwrap();

            let mut tivo_drive =
                ovit::TivoDrive::from_disk_image(input_path).expect("Could not load TiVo drive");

            // Create the table
            let mut table = Table::new();

            table.add_row(row![
                "FSID",
                "Reference Count",
                "Boot Cycles",
                "Boot Seconds",
                "INode",
                "Size",
                "Block Size",
                "Blocks Used",
                "Last Modified",
                "Type",
                "Zone",
                "Checksum",
                "Flags",
                "Number of Blocks",
                "Data",
            ]);
            for inode in tivo_drive.zonemap.inode_iter().unwrap().take(inode_count) {
                let data = if (inode.numblocks == 0) {
                    format!("{:#X?}", inode.data)
                } else {
                    format!("{:#?}", inode.datablocks)
                };

                table.add_row(row![
                    inode.fsid,
                    inode.refcount,
                    inode.bootcycles,
                    inode.bootsecs,
                    inode.inode,
                    inode.size,
                    inode.blocksize,
                    inode.blockused,
                    inode.last_modified,
                    format!("{:#?}", inode.r#type),
                    inode.zone,
                    inode.checksum,
                    inode.flags,
                    inode.numblocks,
                    data
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
