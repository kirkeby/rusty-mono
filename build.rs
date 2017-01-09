extern crate gcc;

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
    let sdk_root = "../mono_framework/dist/mono/include/".to_string();
    let mut gcc = gcc::Config::new();
    gcc.cpp(true);
    for i in INCLUDES {
        gcc.include(format!("{}{}", sdk_root, i));
    }
    gcc.file("src/stubs.cc").compile("libstubs.a");
    println!("cargo:rustc-link-lib=static=empty");
}
