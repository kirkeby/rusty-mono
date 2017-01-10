#![allow(non_snake_case)]
#![allow(improper_ctypes)]
pub mod mono {
    pub struct AppRunLoop {
    #[allow(dead_code)]
        raw: [u8; 32],
    }
    pub struct ApplicationContext {
    #[allow(dead_code)]
        raw: [u8; 1],
    }
    pub struct IApplication {
    #[allow(dead_code)]
        raw: [u8; 8],
    }
    pub struct IApplicationContext {
    #[allow(dead_code)]
        raw: [u8; 1],
    }
    pub struct IRunLoopTask {
    #[allow(dead_code)]
        raw: [u8; 32],
    }
    extern "C" {
        // mono::AppRunLoop::CheckUsbDtr()
        fn stub__ZN4mono10AppRunLoop11CheckUsbDtrEv(o: *mut u8) -> ();
        // mono::AppRunLoop::addDynamicTask(mono::IRunLoopTask *)
        fn stub__ZN4mono10AppRunLoop14addDynamicTaskEPNS_12IRunLoopTaskE(o: *mut u8, p0: *mut ::mono::IRunLoopTask) -> bool;
        // mono::AppRunLoop::removeDynamicTask(mono::IRunLoopTask *)
        fn stub__ZN4mono10AppRunLoop17removeDynamicTaskEPNS_12IRunLoopTaskE(o: *mut u8, p0: *mut ::mono::IRunLoopTask) -> bool;
        // mono::AppRunLoop::setResetOnUserButton(bool)
        fn stub__ZN4mono10AppRunLoop20setResetOnUserButtonEb(o: *mut u8, p0: bool) -> ();
        // mono::AppRunLoop::exec()
        fn stub__ZN4mono10AppRunLoop4execEv(o: *mut u8) -> ();
        // mono::AppRunLoop::quit()
        fn stub__ZN4mono10AppRunLoop4quitEv(o: *mut u8) -> ();
        // mono::AppRunLoop::AppRunLoop()
        fn stub__ZN4mono10AppRunLoopC1Ev(o: *mut u8) -> ();
        // mono::IApplication::enterRunLoop()
        fn stub__ZN4mono12IApplication12enterRunLoopEv(o: *mut u8) -> i32;
        // mono::IApplication::monoWakeFromReset()
        fn stub__ZN4mono12IApplication17monoWakeFromResetEv(o: *mut u8) -> ();
        // mono::IApplication::monoWakeFromSleep()
        fn stub__ZN4mono12IApplication17monoWakeFromSleepEv(o: *mut u8) -> ();
        // mono::IApplication::monoWillGotoSleep()
        fn stub__ZN4mono12IApplication17monoWillGotoSleepEv(o: *mut u8) -> ();
        // mono::ApplicationContext::setMonoApplication(mono::IApplication *)
        fn stub__ZN4mono18ApplicationContext18setMonoApplicationEPNS_12IApplicationE(o: *mut u8, p0: *mut ::mono::IApplication) -> ();
        // mono::ApplicationContext::exec()
        fn stub__ZN4mono18ApplicationContext4execEv(o: *mut u8) -> i32;
        // mono::IApplicationContext::SleepForMs(uint32_t)
        fn stub__ZN4mono19IApplicationContext10SleepForMsEj(o: *mut u8, p0: u32) -> ();
        // mono::IApplicationContext::SoftwareReset()
        fn stub__ZN4mono19IApplicationContext13SoftwareResetEv(o: *mut u8) -> ();
        // mono::IApplicationContext::EnterSleepMode()
        fn stub__ZN4mono19IApplicationContext14EnterSleepModeEv(o: *mut u8) -> ();
        // mono::IApplicationContext::ResetOnUserButton()
        fn stub__ZN4mono19IApplicationContext17ResetOnUserButtonEv(o: *mut u8) -> ();
        // mono::IApplicationContext::setMonoApplication(mono::IApplication *)
        fn stub__ZN4mono19IApplicationContext18setMonoApplicationEPNS_12IApplicationE(o: *mut u8, p0: *mut ::mono::IApplication) -> ();
        // mono::IApplicationContext::SoftwareResetToBootloader()
        fn stub__ZN4mono19IApplicationContext25SoftwareResetToBootloaderEv(o: *mut u8) -> ();
        // mono::IApplicationContext::SoftwareResetToApplication()
        fn stub__ZN4mono19IApplicationContext26SoftwareResetToApplicationEv(o: *mut u8) -> ();
        // mono::IApplicationContext::exec()
        fn stub__ZN4mono19IApplicationContext4execEv(o: *mut u8) -> i32;
    }
    impl AppRunLoop {
        pub fn CheckUsbDtr(&mut self) -> () {
            unsafe { stub__ZN4mono10AppRunLoop11CheckUsbDtrEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn addDynamicTask(&mut self, p0: *mut ::mono::IRunLoopTask) -> bool {
            unsafe { stub__ZN4mono10AppRunLoop14addDynamicTaskEPNS_12IRunLoopTaskE(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn exec(&mut self) -> () {
            unsafe { stub__ZN4mono10AppRunLoop4execEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn new() -> ::mono::AppRunLoop {
            let mut o = ::mono::AppRunLoop { raw: Default::default() };
            unsafe { stub__ZN4mono10AppRunLoopC1Ev(&mut o.raw[0] as *mut u8); }
            o
        }
        pub fn quit(&mut self) -> () {
            unsafe { stub__ZN4mono10AppRunLoop4quitEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn removeDynamicTask(&mut self, p0: *mut ::mono::IRunLoopTask) -> bool {
            unsafe { stub__ZN4mono10AppRunLoop17removeDynamicTaskEPNS_12IRunLoopTaskE(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn setResetOnUserButton(&mut self, p0: bool) -> () {
            unsafe { stub__ZN4mono10AppRunLoop20setResetOnUserButtonEb(&mut self.raw[0] as *mut u8, p0) }
        }
    }
    impl ApplicationContext {
        pub fn exec(&mut self) -> i32 {
            unsafe { stub__ZN4mono18ApplicationContext4execEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn setMonoApplication(&mut self, p0: *mut ::mono::IApplication) -> () {
            unsafe { stub__ZN4mono18ApplicationContext18setMonoApplicationEPNS_12IApplicationE(&mut self.raw[0] as *mut u8, p0) }
        }
    }
    impl IApplication {
        pub fn enterRunLoop(&mut self) -> i32 {
            unsafe { stub__ZN4mono12IApplication12enterRunLoopEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn monoWakeFromReset(&mut self) -> () {
            unsafe { stub__ZN4mono12IApplication17monoWakeFromResetEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn monoWakeFromSleep(&mut self) -> () {
            unsafe { stub__ZN4mono12IApplication17monoWakeFromSleepEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn monoWillGotoSleep(&mut self) -> () {
            unsafe { stub__ZN4mono12IApplication17monoWillGotoSleepEv(&mut self.raw[0] as *mut u8) }
        }
    }
    impl IApplicationContext {
        pub fn EnterSleepMode(&mut self) -> () {
            unsafe { stub__ZN4mono19IApplicationContext14EnterSleepModeEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn ResetOnUserButton(&mut self) -> () {
            unsafe { stub__ZN4mono19IApplicationContext17ResetOnUserButtonEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn SleepForMs(&mut self, p0: u32) -> () {
            unsafe { stub__ZN4mono19IApplicationContext10SleepForMsEj(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn SoftwareReset(&mut self) -> () {
            unsafe { stub__ZN4mono19IApplicationContext13SoftwareResetEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn SoftwareResetToApplication(&mut self) -> () {
            unsafe { stub__ZN4mono19IApplicationContext26SoftwareResetToApplicationEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn SoftwareResetToBootloader(&mut self) -> () {
            unsafe { stub__ZN4mono19IApplicationContext25SoftwareResetToBootloaderEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn exec(&mut self) -> i32 {
            unsafe { stub__ZN4mono19IApplicationContext4execEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn setMonoApplication(&mut self, p0: *mut ::mono::IApplication) -> () {
            unsafe { stub__ZN4mono19IApplicationContext18setMonoApplicationEPNS_12IApplicationE(&mut self.raw[0] as *mut u8, p0) }
        }
    }
}
extern "C" {
}
