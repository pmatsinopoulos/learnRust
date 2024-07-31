use cc;
use std::env;

fn main() {
    // Tell Cargo, if the given file changes to re-run the build script
    println!("cargo::rerun-if-changed=src/hello.c");
    // Use the "cc" crate to build a C file and statically linked to it
    cc::Build::new().file("src/hello.c").compile("hello");

    let out_dir = env::var("OUT_DIR");
    println!("Out dir = *{:?}*", out_dir); // note: These lines will be printed only if cargo build is invoked with "-vv".
}
