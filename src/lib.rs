#![no_std]
#![feature(lang_items, collections)]

extern crate alloc_malloc;
extern crate collections;

use collections::string::String;

pub use generated::mono;

mod generated;
pub mod helpers;

impl From<&'static str> for mono::String {
    fn from(s: &str) -> mono::String {
        mono::String::new_Pcj(
            s.as_ptr() as *mut i8,
            s.len() as u32)
    }
}

// Low-level stuffs
extern "C" {
    #[link_name="printf"]
    fn _printf(s: *const u8) -> ();
}

pub fn debug_print(s: &str) -> () {
    let mut s : String = s.into();
    s = s.replace("%", "%%");
    s.push_str("\r\n\0");
    unsafe { _printf(s.as_ptr()); }
}

#[lang="panic_fmt"]
fn panic_fmt() -> ! {
    debug_print("PANIC!");
    debug_print("Going into infinte loop. Better save me.!");
    loop {}
}
