extern crate gcc;

use std::process::Command;

const INCLUDES : &'static [&'static str] = &[
    "",
    "mbed",
    "mbed/api",
    "mbed/hal",
    "mbed/target_cypress",
    "mbed/libraries/fs/sd",
    "mbed/libraries/fs/fat",
    "mbed/libraries/fs/fat/ChaN",
    "display",
    "display/ili9225g",
    "display/ui",
    "media",
    "wireless",
    "mbed/target_cypress",
];

fn main() {
    let sdk_root = "../mono_framework/dist/mono".to_string();
    let mut gcc = gcc::Config::new();
    gcc.cpp(true);
    for i in INCLUDES {
        gcc.include(format!("{}/include/{}", sdk_root, i));
    }
    gcc.file("src/generated.cc");
    gcc.file("src/helpers.cc");
    gcc.compile("libgenerated.a");

    // The Mono framework comes with a bunch of very broken ar(1) archives,
    // first problem is they're not named properly, so cc(1) can not find them
    // and link against them, second problem is that some of them contain
    // non-object-files, which really make Rust unhappy.
    Command::new("./build.sh").status().expect("build.sh failed");
    println!("cargo:rustc-link-lib=static=all-of-mono");
}
