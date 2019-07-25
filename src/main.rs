mod ovit;
extern crate clap;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("oViT")
        .version("0.0.0-dev")
        .author("Kepler Sticka-Jones <kepler@stickajones.org>")
        .about("An experimental binary to retrieve MPEG streams from a TiVo hard drive (image) and do other TiVo drive related things.")
        .subcommand(SubCommand::with_name("info").arg(Arg::with_name("INPUT")
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

            let inode_zone = tivo_drive
                .zonemap
                .find(|node| node.r#type == ovit::media_file_system::MFSZoneType::INode)
                .expect("Could not load INode zone");

            println!("{:#?}", inode_zone);

            let app_region = tivo_drive
                .partition_map
                .partitions
                .iter()
                .find(|partition| partition.r#type == "MFS")
                .unwrap();

            let first_inode = ovit::media_file_system::MFSINode::from_path_at_sector(
                &input_path,
                app_region.starting_sector,
                inode_zone.last_sector,
                true,
            )
            .unwrap();

            println!("{:#?}", first_inode);
        }
        ("schema", Some(_sub_matches)) => {
            // let schema_contents = include_str!("schema.txt");
            println!("Not interacting with the schema right now!");
        }
        _ => {}
    }
}
