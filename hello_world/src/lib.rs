#![no_std]

extern crate mono_framework_rs;

use mono_framework_rs::debug_print;
use mono_framework_rs::helpers::{run_mono, MonoCallbacks};
use mono_framework_rs::mono;

struct App {
    label: mono::ui::TextLabelView,
}

impl App {
    fn new() -> App {
        let rect = mono::geo::Rect::new_iiii(0, 100, 176, 20);
        let mut label = mono::ui::TextLabelView::new_PKc(
            "Hello, I am Rusty!\0".as_ptr() as *mut i8);
        label.setRect(rect);
        //helloLabel->setAlignment(mono::ui::TextLabelView::ALIGN_CENTER);
        //helloLabel->setTextColor(mono::display::TurquoiseColor);
        App { label: label }
    }

    fn wake_from_reset(&mut self) {
        self.label.show();
    }

    fn wake_from_sleep(&mut self) {
        self.label.scheduleRepaint();
    }
}


// Glue into Mono framework
static mut APP : Option<App> = None;

extern fn initalize() -> () {
    unsafe {
        debug_print("initialize");
        APP = Some(App::new())
    }
}

extern fn wake_from_reset() -> () {
    unsafe {
        debug_print("wake_from_reset");
        match APP {
            Some(ref mut app) => app.wake_from_reset(),
            None => debug_print("No App here. Where the fuck am I?!"),
        };
    }
}

extern fn will_goto_sleep() -> () {
    debug_print("will_goto_sleep");
}

extern fn wake_from_sleep() -> () {
    unsafe {
        debug_print("wake_from_sleep");
        match APP {
            Some(ref mut app) => app.wake_from_sleep(),
            None => debug_print("No App here. Where the fuck am I?!"),
        };
    }
}

#[no_mangle]
pub fn main() {
    unsafe {
        run_mono(MonoCallbacks {
            initialize: initalize,
            wake_from_reset: wake_from_reset,
            will_goto_sleep: will_goto_sleep,
            wake_from_sleep: wake_from_sleep,
        });
    }
}
