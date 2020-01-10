mod ovit_fuse;

extern crate clap;

use clap::{App, Arg};
use ovit_fuse::TiVoFS;
use std::ffi::OsStr;

fn main() {
    let matches = App::new("ovit-fuse")
        .version("0.0.0-dev")
        .author("Kepler Sticka-Jones <kepler@stickajones.org>")
        .about("FUSE driver for interacting with TiVo Media File Systems")
        .arg(
            Arg::with_name("TARGET")
                .help("The TiVo drive containing the Media File System to be mounted")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("MOUNT POINT")
                .help("Location to mount Media File System")
                .required(true)
                .index(2),
        )
        .get_matches();

    let tivo_drive_location = matches.value_of("TARGET").expect("No TiVo drive provided!");
    let mount_point = matches
        .value_of("MOUNT POINT")
        .expect("No mount point provided!");

    let filesystem = TiVoFS {
        drive_location: tivo_drive_location.to_string(),
    };

    let fuse_args: Vec<&OsStr> = vec![&OsStr::new("-o"), &OsStr::new("auto_unmount")];

    println!("Mounting TiVoFS with FUSE");

    fuse_mt::mount(
        fuse_mt::FuseMT::new(filesystem, 1),
        &mount_point,
        &fuse_args,
    )
    .expect("Could not mount!");
}
