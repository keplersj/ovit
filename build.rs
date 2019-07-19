extern crate cc;
extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::path::Path;

fn main() {
    let mfs_tools_library_path = Path::new("deps/mfs-tools/include");

    // Build mfs-tools library files.
    cc::Build::new()
        .include(mfs_tools_library_path)
        .file("deps/mfs-tools/lib/crc.c")
        .file("deps/mfs-tools/lib/inode.c")
        .file("deps/mfs-tools/lib/log.c")
        // .file("deps/mfs-tools/lib/macpart.c")
        .file("deps/mfs-tools/lib/mfs.c")
        .file("deps/mfs-tools/lib/mfsdbschema.c")
        .file("deps/mfs-tools/lib/readwrite.c")
        .file("deps/mfs-tools/lib/volume.c")
        .file("deps/mfs-tools/lib/zonemap.c")
        .warnings(false)
        .compile("libmfstools.a");

    let mfs_tools_bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("deps/mfs-tools/include/wrapper.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate mfs-tools bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    mfs_tools_bindings
        .write_to_file(out_path.join("mfs_tools_bindings.rs"))
        .expect("Couldn't write bindings!");
}