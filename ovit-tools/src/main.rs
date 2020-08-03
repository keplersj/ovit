extern crate clap;
#[macro_use]
extern crate prettytable;
extern crate ovit;
extern crate tivo_media_file_system;

use clap::{App, Arg, SubCommand};
use prettytable::Table;
use tivo_media_file_system::{MFSINode, MFSINodeType};

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
                .default_value("25"))
            .arg(Arg::with_name("data")
                .short("d")
                .long("display-data")
                .required(false)))
        .subcommand(SubCommand::with_name("fsid")
            .arg(Arg::with_name("INPUT")
                .help("The drive image to read from")
                .required(true))
            .arg(Arg::with_name("id")
                .value_name("NUMBER")
                .help("Sets the FSID to lookup")
                .required(true)))
        .subcommand(SubCommand::with_name("inode")
            .arg(Arg::with_name("INPUT")
                .help("The drive image to read from")
                .required(true))
            .arg(Arg::with_name("id")
                .value_name("NUMBER")
                .help("Sets the INode to lookup")
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
                tivo_drive.raw_zonemap.inode_iter().unwrap().len()
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

            // Print the table to stdout
            table.printstd();
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
            let show_data = sub_match.is_present("data");

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
                if show_data { "Data" } else { "" },
            ]);
            for inode in tivo_drive
                .raw_zonemap
                .inode_iter()
                .unwrap()
                .take(inode_count)
            {
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
                    if show_data {
                        format!("{:?}", inode.get_data(input_path.to_string()))
                    } else {
                        "".to_string()
                    }
                ]);
            }

            // Print the table to stdout
            table.printstd();
        }
        ("fsid", Some(sub_match)) => {
            // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
            // required we could have used an 'if let' to conditionally get the value)
            let input_path = sub_match.value_of("INPUT").unwrap();
            let fsid: u32 = sub_match.value_of("id").unwrap().parse().unwrap();

            println!("Loading TiVo Drive");

            let mut tivo_drive =
                ovit::TivoDrive::from_disk_image(input_path).expect("Could not load TiVo drive");

            println!("TiVo Drive Loaded!");

            println!();

            println!("Looking for FSID: {}", fsid);

            let found_inode = tivo_drive.get_inode_from_fsid(fsid).unwrap();

            println!("Found INode: {:#?}", found_inode);

            if found_inode.r#type == MFSINodeType::Dir {
                println!("INode is a Directory, getting directory entries.");

                let entries = found_inode
                    .get_entries_from_directory(input_path.to_string())
                    .unwrap();

                println!("Entries: {:#?}", entries);
            }
        }
        ("inode", Some(sub_match)) => {
            // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
            // required we could have used an 'if let' to conditionally get the value)
            let input_path = sub_match.value_of("INPUT").unwrap();
            let inode: u64 = sub_match.value_of("id").unwrap().parse().unwrap();

            println!("Loading TiVo Drive");

            let mut tivo_drive =
                ovit::TivoDrive::from_disk_image(input_path).expect("Could not load TiVo drive");

            println!("TiVo Drive Loaded!");

            println!();

            println!("Looking for INode: {}", inode);

            fn sector_for_inode(inode: u64) -> u64 {
                (2 * inode) + 1122
            }

            let sector = sector_for_inode(inode);

            let found_inode = MFSINode::from_file_at_sector(
                &mut tivo_drive.source_file,
                tivo_drive
                    .volumes
                    .find_sector_volume(sector)
                    .disk_sector
                    .into(),
                sector,
                tivo_drive.is_byte_swapped,
            )
            .unwrap();

            println!("Found INode: {:#?}", found_inode);

            if found_inode.r#type == MFSINodeType::Dir {
                println!("INode is a Directory, getting directory entries.");

                let entries = found_inode
                    .get_entries_from_directory(input_path.to_string())
                    .unwrap();

                println!("Entries: {:#?}", entries);
            }
        }
        _ => {
            println!("{}", matches.usage());
        }
    }
}
