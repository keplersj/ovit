mod ovit;

extern crate clap;

use clap::{App, Arg, SubCommand};

use std::collections::HashMap;

use std::fs;

fn main() {
    let matches = App::new("oViT")
        .version("0.0.0-dev")
        .author("Kepler Sticka-Jones <kepler@stickajones.org>")
        .about("An experimental binary to retrieve MPEG streams from a TiVo hard drive (image) and do other TiVo drive related things.")
        .subcommand(SubCommand::with_name("info").arg(Arg::with_name("INPUT")
            .help("The drive image to read from")
            .required(true)))
        .subcommand(SubCommand::with_name("schema").arg(Arg::with_name("INPUT")
            .help("Schema file")
            .required(true)))
        .get_matches();

    match matches.subcommand() {
        ("info", Some(sub_match)) => {
            // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
            // required we could have used an 'if let' to conditionally get the value)
            let input_path = sub_match.value_of("INPUT").unwrap();

            let tivo_image = ovit::TivoDrive::from_disk_image(input_path)
                .expect("Could not open TiVo Drive Image");

            let node_count = tivo_image
                .inodes
                .iter()
                .filter(|inode| inode.r#type == ovit::media_file_system::MFSINodeType::Node)
                .count();
            let file_count = tivo_image
                .inodes
                .iter()
                .filter(|inode| inode.r#type == ovit::media_file_system::MFSINodeType::File)
                .count();
            let stream_count = tivo_image
                .inodes
                .iter()
                .filter(|inode| inode.r#type == ovit::media_file_system::MFSINodeType::Stream)
                .count();
            let dir_count = tivo_image
                .inodes
                .iter()
                .filter(|inode| inode.r#type == ovit::media_file_system::MFSINodeType::Dir)
                .count();
            let db_count = tivo_image
                .inodes
                .iter()
                .filter(|inode| inode.r#type == ovit::media_file_system::MFSINodeType::Db)
                .count();

            println!("Node Count: {:#?}", node_count);
            println!("File Count: {:#?}", file_count);
            println!("Stream Count: {:#?}", stream_count);
            println!("Dir Count: {:#?}", dir_count);
            println!("Db Count: {:#?}", db_count);

            // let db_test_object = tivo_image
            //     .inodes
            //     .iter()
            //     .find(|inode| inode.r#type == ovit::MFSINodeType::Db)
            //     .unwrap();
            // println!("{:#?}", db_test_object);

            // let object_data = ovit::util::correct_byte_order(
            //     &ovit::get_blocks_from_drive(
            //         &tivo_image.source_file,
            //         u64::from(
            //             tivo_image
            //                 .zones
            //                 .get(db_test_object.zone as usize - 1)
            //                 .unwrap()
            //                 .first_sector
            //                 + db_test_object.data_block_sector,
            //         ),
            //         // db_test_object.data_block_sector as usize,
            //         1,
            //     )
            //     .unwrap(),
            //     true,
            // );

            // println!("{:?}", object_data);
        }
        ("schema", Some(sub_match)) => {
            // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
            // required we could have used an 'if let' to conditionally get the value)
            let input_path = sub_match.value_of("INPUT").unwrap();

            let schema_contents = String::from_utf8(fs::read(input_path).unwrap()).unwrap();
            let mut schema = HashMap::new();

            for line in schema_contents.split('\n') {
                let columns: Vec<&str> = line.split(' ').collect();
                if columns.len() == 8 {
                    let root = columns[1];
                    let attr = columns[3];

                    if !schema.contains_key(root) {
                        schema.insert(root, vec![(attr, columns[4..=7].to_vec())]);
                    } else {
                        let mut array = schema.get(root).unwrap().to_vec();
                        array.push((attr, columns[4..=7].to_vec()));
                        schema.insert(root, array);
                    }
                }
            }

            println!("{:#?}", schema);
        }
        _ => {}
    }
}
