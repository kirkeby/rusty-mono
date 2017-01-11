#![allow(non_snake_case)]
#![allow(improper_ctypes)]
pub mod mono {
    pub mod geo {
        pub struct Circle {
        #[allow(dead_code)]
            raw: [u8; 12],
        }
        pub struct Point {
        #[allow(dead_code)]
            raw: [u8; 8],
        }
        #[allow(dead_code)]
        extern "C" {
            // mono::geo::Point::setX(int)
            fn stub__ZN4mono3geo5Point4setXEi(o: *mut u8, p0: i32) -> ();
            // mono::geo::Point::setY(int)
            fn stub__ZN4mono3geo5Point4setYEi(o: *mut u8, p0: i32) -> ();
            // mono::geo::Point::appendX(int)
            fn stub__ZN4mono3geo5Point7appendXEi(o: *mut u8, p0: i32) -> ();
            // mono::geo::Point::appendY(int)
            fn stub__ZN4mono3geo5Point7appendYEi(o: *mut u8, p0: i32) -> ();
            // mono::geo::Point::Point(const mono::geo::Point &)
            fn stub__ZN4mono3geo5PointC1ERKS1_(o: *mut u8, p0: &::mono::geo::Point) -> ();
            // mono::geo::Point::Point(int, int)
            fn stub__ZN4mono3geo5PointC1Eii(o: *mut u8, p0: i32, p1: i32) -> ();
            // mono::geo::Point::Point()
            fn stub__ZN4mono3geo5PointC1Ev(o: *mut u8) -> ();
            // mono::geo::Point::operator=(const mono::geo::Point &)
            fn stub__ZN4mono3geo5PointaSERKS1_(o: *mut u8, p0: &::mono::geo::Point) -> &::mono::geo::Point;
            // mono::geo::Circle::Circle(mono::geo::Point, uint32_t)
            fn stub__ZN4mono3geo6CircleC1ENS0_5PointEj(o: *mut u8, p0: ::mono::geo::Point, p1: u32) -> ();
            // mono::geo::Circle::Circle(int, int, uint32_t)
            fn stub__ZN4mono3geo6CircleC1Eiij(o: *mut u8, p0: i32, p1: i32, p2: u32) -> ();
            // mono::geo::Circle::Circle()
            fn stub__ZN4mono3geo6CircleC1Ev(o: *mut u8) -> ();
            // mono::geo::Point::X()
            fn stub__ZNK4mono3geo5Point1XEv(o: *mut u8) -> i32;
            // mono::geo::Point::Y()
            fn stub__ZNK4mono3geo5Point1YEv(o: *mut u8) -> i32;
            // mono::geo::Point::Abs()
            fn stub__ZNK4mono3geo5Point3AbsEv(o: *mut u8) -> u32;
            // mono::geo::Point::toString()
            fn stub__ZNK4mono3geo5Point8toStringEv(o: *mut u8) -> ::mono::String;
            // mono::geo::Circle::Radius()
            fn stub__ZNK4mono3geo6Circle6RadiusEv(o: *mut u8) -> u32;
        }
        impl Circle {
            pub fn Radius(&mut self) -> u32 {
                unsafe { stub__ZNK4mono3geo6Circle6RadiusEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn new() -> ::mono::geo::Circle {
                let mut o = ::mono::geo::Circle { raw: Default::default() };
                unsafe { stub__ZN4mono3geo6CircleC1Ev(&mut o.raw[0] as *mut u8); }
                o
            }
            pub fn new_NS0_5PointEj(p0: ::mono::geo::Point, p1: u32) -> ::mono::geo::Circle {
                let mut o = ::mono::geo::Circle { raw: Default::default() };
                unsafe { stub__ZN4mono3geo6CircleC1ENS0_5PointEj(&mut o.raw[0] as *mut u8, p0, p1); }
                o
            }
            pub fn new_iij(p0: i32, p1: i32, p2: u32) -> ::mono::geo::Circle {
                let mut o = ::mono::geo::Circle { raw: Default::default() };
                unsafe { stub__ZN4mono3geo6CircleC1Eiij(&mut o.raw[0] as *mut u8, p0, p1, p2); }
                o
            }
        }
        impl Point {
            pub fn Abs(&mut self) -> u32 {
                unsafe { stub__ZNK4mono3geo5Point3AbsEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn X(&mut self) -> i32 {
                unsafe { stub__ZNK4mono3geo5Point1XEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Y(&mut self) -> i32 {
                unsafe { stub__ZNK4mono3geo5Point1YEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn appendX(&mut self, p0: i32) -> () {
                unsafe { stub__ZN4mono3geo5Point7appendXEi(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn appendY(&mut self, p0: i32) -> () {
                unsafe { stub__ZN4mono3geo5Point7appendYEi(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn new() -> ::mono::geo::Point {
                let mut o = ::mono::geo::Point { raw: Default::default() };
                unsafe { stub__ZN4mono3geo5PointC1Ev(&mut o.raw[0] as *mut u8); }
                o
            }
            pub fn new_RKS1_(p0: &::mono::geo::Point) -> ::mono::geo::Point {
                let mut o = ::mono::geo::Point { raw: Default::default() };
                unsafe { stub__ZN4mono3geo5PointC1ERKS1_(&mut o.raw[0] as *mut u8, p0); }
                o
            }
            pub fn new_ii(p0: i32, p1: i32) -> ::mono::geo::Point {
                let mut o = ::mono::geo::Point { raw: Default::default() };
                unsafe { stub__ZN4mono3geo5PointC1Eii(&mut o.raw[0] as *mut u8, p0, p1); }
                o
            }
            pub fn setX(&mut self, p0: i32) -> () {
                unsafe { stub__ZN4mono3geo5Point4setXEi(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setY(&mut self, p0: i32) -> () {
                unsafe { stub__ZN4mono3geo5Point4setYEi(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn toString(&mut self) -> ::mono::String {
                unsafe { stub__ZNK4mono3geo5Point8toStringEv(&mut self.raw[0] as *mut u8) }
            }
        }
    }
    pub struct IApplication {
    #[allow(dead_code)]
        raw: [u8; 8],
    }
    pub struct IApplicationContext {
    #[allow(dead_code)]
        raw: [u8; 80],
    }
    pub struct IRunLoopTask {
    #[allow(dead_code)]
        raw: [u8; 32],
    }
    pub struct String {
    #[allow(dead_code)]
        raw: [u8; 24],
    }
    #[allow(dead_code)]
    extern "C" {
        // mono::IApplication::enterRunLoop()
        fn stub__ZN4mono12IApplication12enterRunLoopEv(o: *mut u8) -> i32;
        // mono::IApplication::monoWakeFromReset()
        fn stub__ZN4mono12IApplication17monoWakeFromResetEv(o: *mut u8) -> ();
        // mono::IApplication::monoWakeFromSleep()
        fn stub__ZN4mono12IApplication17monoWakeFromSleepEv(o: *mut u8) -> ();
        // mono::IApplication::monoWillGotoSleep()
        fn stub__ZN4mono12IApplication17monoWillGotoSleepEv(o: *mut u8) -> ();
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
        // mono::String::preAllocbytes(uint32_t)
        fn stub__ZN4mono6String13preAllocbytesEj(o: *mut u8, p0: u32) -> ();
        // mono::String::Format(const char *, ...)
        fn stub__ZN4mono6String6FormatEPKcz(o: *mut u8, p0: *mut i8) -> ::mono::String;
        // mono::String::String(const char *)
        fn stub__ZN4mono6StringC1EPKc(o: *mut u8, p0: *mut i8) -> ();
        // mono::String::String(char *)
        fn stub__ZN4mono6StringC1EPc(o: *mut u8, p0: *mut i8) -> ();
        // mono::String::String(char *, uint32_t)
        fn stub__ZN4mono6StringC1EPcj(o: *mut u8, p0: *mut i8, p1: u32) -> ();
        // mono::String::String(const mono::String &)
        fn stub__ZN4mono6StringC1ERKS0_(o: *mut u8, p0: &::mono::String) -> ();
        // mono::String::String(uint32_t)
        fn stub__ZN4mono6StringC1Ej(o: *mut u8, p0: u32) -> ();
        // mono::String::String()
        fn stub__ZN4mono6StringC1Ev(o: *mut u8) -> ();
        // mono::String::~String()
        fn stub__ZN4mono6StringD0Ev(o: *mut u8) -> ();
        // mono::String::operator=(const char *)
        fn stub__ZN4mono6StringaSEPKc(o: *mut u8, p0: *mut i8) -> &::mono::String;
        // mono::String::operator=(const mono::String &)
        fn stub__ZN4mono6StringaSERKS0_(o: *mut u8, p0: &::mono::String) -> &::mono::String;
        // mono::String::operator==(const char *)
        fn stub__ZN4mono6StringeqEPKc(o: *mut u8, p0: *mut i8) -> bool;
        // mono::String::operator==(const mono::String &)
        fn stub__ZN4mono6StringeqERKS0_(o: *mut u8, p0: &::mono::String) -> bool;
        // mono::String::operator!=(const char *)
        fn stub__ZN4mono6StringneEPKc(o: *mut u8, p0: *mut i8) -> bool;
        // mono::String::operator!=(const mono::String &)
        fn stub__ZN4mono6StringneERKS0_(o: *mut u8, p0: &::mono::String) -> bool;
        // mono::String::Length()
        fn stub__ZNK4mono6String6LengthEv(o: *mut u8) -> u32;
        // mono::String::operator()()
        fn stub__ZNK4mono6StringclEv(o: *mut u8) -> *mut i8;
        // mono::String::operator[](uint32_t)
        fn stub__ZNK4mono6StringixEj(o: *mut u8, p0: u32) -> i8;
    }
    impl Drop for String {
         fn drop(&mut self) -> () {
            unsafe { stub__ZN4mono6StringD0Ev(&mut self.raw[0] as *mut u8) }
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
    impl String {
        pub fn Format(&mut self, p0: *mut i8) -> ::mono::String {
            unsafe { stub__ZN4mono6String6FormatEPKcz(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn Length(&mut self) -> u32 {
            unsafe { stub__ZNK4mono6String6LengthEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn new() -> ::mono::String {
            let mut o = ::mono::String { raw: Default::default() };
            unsafe { stub__ZN4mono6StringC1Ev(&mut o.raw[0] as *mut u8); }
            o
        }
        pub fn new_PKc(p0: *mut i8) -> ::mono::String {
            let mut o = ::mono::String { raw: Default::default() };
            unsafe { stub__ZN4mono6StringC1EPKc(&mut o.raw[0] as *mut u8, p0); }
            o
        }
        pub fn new_Pc(p0: *mut i8) -> ::mono::String {
            let mut o = ::mono::String { raw: Default::default() };
            unsafe { stub__ZN4mono6StringC1EPc(&mut o.raw[0] as *mut u8, p0); }
            o
        }
        pub fn new_Pcj(p0: *mut i8, p1: u32) -> ::mono::String {
            let mut o = ::mono::String { raw: Default::default() };
            unsafe { stub__ZN4mono6StringC1EPcj(&mut o.raw[0] as *mut u8, p0, p1); }
            o
        }
        pub fn new_RKS0_(p0: &::mono::String) -> ::mono::String {
            let mut o = ::mono::String { raw: Default::default() };
            unsafe { stub__ZN4mono6StringC1ERKS0_(&mut o.raw[0] as *mut u8, p0); }
            o
        }
        pub fn new_j(p0: u32) -> ::mono::String {
            let mut o = ::mono::String { raw: Default::default() };
            unsafe { stub__ZN4mono6StringC1Ej(&mut o.raw[0] as *mut u8, p0); }
            o
        }
        pub fn preAllocbytes(&mut self, p0: u32) -> () {
            unsafe { stub__ZN4mono6String13preAllocbytesEj(&mut self.raw[0] as *mut u8, p0) }
        }
    }
}
#[allow(dead_code)]
extern "C" {
}
