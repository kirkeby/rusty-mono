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
    let sdk_root = "../mono_framework/dist/mono".to_string();
    let mut gcc = gcc::Config::new();
    gcc.cpp(true);
    for i in INCLUDES {
        gcc.include(format!("{}/include/{}", sdk_root, i));
    }
    gcc.file("src/generated.cc").compile("libgenerated.a");
}
