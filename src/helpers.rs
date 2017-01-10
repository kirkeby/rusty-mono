#[repr(C)]
pub struct MonoCallbacks {
    pub initialize: extern fn() -> (),
    pub wake_from_reset: extern fn() -> (),
    pub will_goto_sleep: extern fn() -> (),
    pub wake_from_sleep: extern fn() -> (),
}

extern "C" {
    pub fn run_mono(cb: MonoCallbacks) -> !;
}
