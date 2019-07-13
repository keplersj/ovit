mod ovit;
extern crate clap;
use clap::{App, Arg, SubCommand};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1, line_ending, space1},
    multi::many0,
    IResult,
};

#[derive(Debug)]
struct SchemaAttribute {
    root_type_id: u8,
    root_type_name: String,
    attribute_id: u8,
    attribute_name: String,
    r#type: String,
    required: String,
}

fn parse_schema_row(input: &str) -> IResult<&str, SchemaAttribute> {
    let (input, root_type_id) = digit1(input)?;
    let (input, _) = space1(input)?;
    let (input, root_type_name) = alphanumeric1(input)?;
    let (input, _) = space1(input)?;
    let (input, attribute_id) = digit1(input)?;
    let (input, _) = space1(input)?;
    let (input, attribute_name) = alt((tag("WasUpgradedFrom1_3"), alphanumeric1))(input)?;
    let (input, _) = space1(input)?;
    let (input, r#type) = alphanumeric1(input)?;
    let (input, _) = space1(input)?;
    let (input, required) = alphanumeric1(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = alt((tag("{}"), alphanumeric1))(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = alphanumeric1(input)?;
    let (input, _) = line_ending(input)?;
    Ok((
        input,
        SchemaAttribute {
            root_type_id: u8::from_str_radix(root_type_id, 10)
                .expect("Couldn't convert root type id"),
            root_type_name: root_type_name.to_string(),
            attribute_id: u8::from_str_radix(attribute_id, 10)
                .expect("Couldn't convert attribute id"),
            attribute_name: attribute_name.to_string(),
            r#type: r#type.to_string(),
            required: required.to_string(),
        },
    ))
}

fn main() {
    let matches = App::new("oViT")
        .version("0.0.0-dev")
        .author("Kepler Sticka-Jones <kepler@stickajones.org>")
        .about("An experimental binary to retrieve MPEG streams from a TiVo hard drive (image) and do other TiVo drive related things.")
        .subcommand(SubCommand::with_name("info").arg(Arg::with_name("INPUT")
            .help("The drive image to read from")
            .required(true)))
        .subcommand(SubCommand::with_name("schema"))
        .get_matches();

    match matches.subcommand() {
        ("info", Some(sub_match)) => {
            // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
            // required we could have used an 'if let' to conditionally get the value)
            let input_path = sub_match.value_of("INPUT").unwrap();

            println!("Parsing TiVo Drive. Please wait");

            let tivo_image = ovit::TivoDrive::from_disk_image(input_path)
                .expect("Could not open TiVo Drive Image");
            println!("TiVo Drive Parsed!");

            let data_in_header_dir_inodes: Vec<&ovit::media_file_system::MFSINode> = tivo_image
                .inodes
                .iter()
                .filter(|inode| inode.flags == 0x4000_0000)
                .filter(|inode| inode.r#type == ovit::media_file_system::MFSINodeType::Dir)
                .collect();
            println!("{:#X?}", data_in_header_dir_inodes);
        }
        ("schema", Some(_sub_matches)) => {
            let schema_contents = include_str!("schema.txt");
            let (_, parsed_schema) =
                many0(parse_schema_row)(schema_contents).expect("Could not parse schema");
            println!("{:#?}", parsed_schema);
        }
        _ => {}
    }
}
