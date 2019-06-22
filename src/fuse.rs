extern crate env_logger;
extern crate fuse;

use fuse::Filesystem;
use std::env;

struct MediaFilesystem;

impl Filesystem for MediaFilesystem {}

fn main() {
    env_logger::init();
    let mountpoint = env::args_os().nth(1).unwrap();
    fuse::mount(MediaFilesystem, &mountpoint, &[]).unwrap();
}