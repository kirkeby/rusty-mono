extern crate gcc;

fn main() {
    gcc::Config::new()
        .cpp(true)
        .include("../mono_framework/src")
        .file("src/stubs.cc")
        .compile("libstubs.a");
    println!("cargo:rustc-link-lib=static=empty");
}
