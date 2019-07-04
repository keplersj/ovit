mod ovit;

extern crate clap;
extern crate pbr;
extern crate positioned_io;

use clap::{App, Arg};

use std::fs::File;

use std::io::prelude::*;

use pbr::{ProgressBar, Units};

use positioned_io::ReadAt;

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

    for mfs_partition in tivo_image
        .partition_map
        .partitions
        .iter()
        .filter(|partition| partition.r#type == "MFS")
        .filter(|partition| partition.name.contains("application"))
    {
        println!("{:#?}", mfs_partition);

        let mut partition_export =
            File::create(format! {"/Volumes/External/{}.iso", mfs_partition.name})
                .expect("Could not create file");

        let mut pb = ProgressBar::new(u64::from(mfs_partition.sector_size) * 512u64);
        pb.set_units(Units::Bytes);

        for sector in 0..=mfs_partition.sector_size {

            let mut buffer = vec![0; 512];
            file.read_at(
                u64::from(mfs_partition.starting_sector + sector) * 512u64,
                &mut buffer,
            )
            .expect("Could not read partition from image");

            let corrected_chunk = ovit::correct_byte_order(&buffer, true);

            partition_export
                .write_all(&corrected_chunk)
                .expect("Could not export partition");

            pb.add(512);
        }

        println!(
            "Wrote {} partition to file at ./{}.iso",
            mfs_partition.name, mfs_partition.name
        )
    }
}
