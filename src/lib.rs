#![no_std]

mod generated;
pub mod helpers;

pub use generated::mono;

impl From<&'static str> for mono::String {
    fn from(s: &str) -> mono::String {
        mono::String::new_Pcj(
            s.as_ptr() as *mut i8,
            s.len() as u32)
    }
}
