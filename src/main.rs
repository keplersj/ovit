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

            let tivo_drive = ovit::TivoDrive::from_disk_image(input_path);

            println!("TiVo Drive: {:?}", tivo_drive);
        }
        ("schema", Some(_sub_matches)) => {
            // let schema_contents = include_str!("schema.txt");
            println!("Not interacting with the schema right now!");
        }
        _ => {}
    }
}
