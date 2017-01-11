#![allow(non_snake_case)]
#![allow(improper_ctypes)]
pub mod mono {
    pub mod display {
        pub struct Color {
        #[allow(dead_code)]
            raw: [u8; 2],
        }
        #[allow(dead_code)]
        extern "C" {
            // mono::display::Color::BytePointer()
            fn stub__ZN4mono7display5Color11BytePointerEv(o: *mut u8) -> *mut u8;
            // mono::display::Color::Color(const mono::display::Color &)
            fn stub__ZN4mono7display5ColorC1ERKS1_(o: *mut u8, p0: &::generated::mono::display::Color) -> ();
            // mono::display::Color::Color(uint8_t, uint8_t, uint8_t)
            fn stub__ZN4mono7display5ColorC1Ehhh(o: *mut u8, p0: u8, p1: u8, p2: u8) -> ();
            // mono::display::Color::Color(const int)
            fn stub__ZN4mono7display5ColorC1Ei(o: *mut u8, p0: i32) -> ();
            // mono::display::Color::Color()
            fn stub__ZN4mono7display5ColorC1Ev(o: *mut u8) -> ();
            // mono::display::Color::operator=(mono::display::Color)
            fn stub__ZN4mono7display5ColoraSES1_(o: *mut u8, p0: ::generated::mono::display::Color) -> u16;
            // mono::display::Color::operator==(const mono::display::Color &)
            fn stub__ZN4mono7display5ColoreqERKS1_(o: *mut u8, p0: &::generated::mono::display::Color) -> bool;
            // mono::display::Color::operator*(const mono::display::Color &)
            fn stub__ZN4mono7display5ColormlERKS1_(o: *mut u8, p0: &::generated::mono::display::Color) -> ::generated::mono::display::Color;
            // mono::display::Color::operator!=(const mono::display::Color &)
            fn stub__ZN4mono7display5ColorneERKS1_(o: *mut u8, p0: &::generated::mono::display::Color) -> bool;
            // mono::display::Color::operator+(const mono::display::Color &)
            fn stub__ZN4mono7display5ColorplERKS1_(o: *mut u8, p0: &::generated::mono::display::Color) -> ::generated::mono::display::Color;
            // mono::display::Color::alphaBlend(uint8_t, const mono::display::Color &)
            fn stub__ZNK4mono7display5Color10alphaBlendEhRKS1_(o: *mut u8, p0: u8, p1: &::generated::mono::display::Color) -> ::generated::mono::display::Color;
            // mono::display::Color::blendAdditive(mono::display::Color)
            fn stub__ZNK4mono7display5Color13blendAdditiveES1_(o: *mut u8, p0: ::generated::mono::display::Color) -> ::generated::mono::display::Color;
            // mono::display::Color::blendMultiply(mono::display::Color)
            fn stub__ZNK4mono7display5Color13blendMultiplyES1_(o: *mut u8, p0: ::generated::mono::display::Color) -> ::generated::mono::display::Color;
            // mono::display::Color::Red()
            fn stub__ZNK4mono7display5Color3RedEv(o: *mut u8) -> u8;
            // mono::display::Color::Blue()
            fn stub__ZNK4mono7display5Color4BlueEv(o: *mut u8) -> u8;
            // mono::display::Color::Green()
            fn stub__ZNK4mono7display5Color5GreenEv(o: *mut u8) -> u8;
            // mono::display::Color::scale(uint8_t)
            fn stub__ZNK4mono7display5Color5scaleEh(o: *mut u8, p0: u8) -> ::generated::mono::display::Color;
            // mono::display::Color::invert()
            fn stub__ZNK4mono7display5Color6invertEv(o: *mut u8) -> ::generated::mono::display::Color;
            // mono::display::Color::toString()
            fn stub__ZNK4mono7display5Color8toStringEv(o: *mut u8) -> ::generated::mono::String;
        }
        impl Color {
            pub fn Blue(&mut self) -> u8 {
                unsafe { stub__ZNK4mono7display5Color4BlueEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn BytePointer(&mut self) -> *mut u8 {
                unsafe { stub__ZN4mono7display5Color11BytePointerEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Green(&mut self) -> u8 {
                unsafe { stub__ZNK4mono7display5Color5GreenEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Red(&mut self) -> u8 {
                unsafe { stub__ZNK4mono7display5Color3RedEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn alphaBlend(&mut self, p0: u8, p1: &::generated::mono::display::Color) -> ::generated::mono::display::Color {
                unsafe { stub__ZNK4mono7display5Color10alphaBlendEhRKS1_(&mut self.raw[0] as *mut u8, p0, p1) }
            }
            pub fn blendAdditive(&mut self, p0: ::generated::mono::display::Color) -> ::generated::mono::display::Color {
                unsafe { stub__ZNK4mono7display5Color13blendAdditiveES1_(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn blendMultiply(&mut self, p0: ::generated::mono::display::Color) -> ::generated::mono::display::Color {
                unsafe { stub__ZNK4mono7display5Color13blendMultiplyES1_(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn invert(&mut self) -> ::generated::mono::display::Color {
                unsafe { stub__ZNK4mono7display5Color6invertEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn new() -> ::generated::mono::display::Color {
                let mut o = Color { raw: [0; 2] };
                unsafe { stub__ZN4mono7display5ColorC1Ev(&mut o.raw[0] as *mut u8); }
                o
            }
            pub fn new_RKS1_(p0: &::generated::mono::display::Color) -> ::generated::mono::display::Color {
                let mut o = Color { raw: [0; 2] };
                unsafe { stub__ZN4mono7display5ColorC1ERKS1_(&mut o.raw[0] as *mut u8, p0); }
                o
            }
            pub fn new_hhh(p0: u8, p1: u8, p2: u8) -> ::generated::mono::display::Color {
                let mut o = Color { raw: [0; 2] };
                unsafe { stub__ZN4mono7display5ColorC1Ehhh(&mut o.raw[0] as *mut u8, p0, p1, p2); }
                o
            }
            pub fn new_i(p0: i32) -> ::generated::mono::display::Color {
                let mut o = Color { raw: [0; 2] };
                unsafe { stub__ZN4mono7display5ColorC1Ei(&mut o.raw[0] as *mut u8, p0); }
                o
            }
            pub fn scale(&mut self, p0: u8) -> ::generated::mono::display::Color {
                unsafe { stub__ZNK4mono7display5Color5scaleEh(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn toString(&mut self) -> ::generated::mono::String {
                unsafe { stub__ZNK4mono7display5Color8toStringEv(&mut self.raw[0] as *mut u8) }
            }
        }
    }
    pub mod geo {
        pub struct Circle {
        #[allow(dead_code)]
            raw: [u8; 12],
        }
        pub struct Point {
        #[allow(dead_code)]
            raw: [u8; 8],
        }
        pub struct Rect {
        #[allow(dead_code)]
            raw: [u8; 16],
        }
        pub struct Size {
        #[allow(dead_code)]
            raw: [u8; 8],
        }
        #[allow(dead_code)]
        extern "C" {
            // mono::geo::Rect::Area()
            fn stub__ZN4mono3geo4Rect4AreaEv(o: *mut u8) -> i32;
            // mono::geo::Rect::Size()
            fn stub__ZN4mono3geo4Rect4SizeEv(o: *mut u8) -> &::generated::mono::geo::Size;
            // mono::geo::Rect::setSize(class Size)
            fn stub__ZN4mono3geo4Rect7setSizeENS0_4SizeE(o: *mut u8, p0: ::generated::mono::geo::Size) -> ();
            // mono::geo::Rect::setPoint(class Point)
            fn stub__ZN4mono3geo4Rect8setPointENS0_5PointE(o: *mut u8, p0: ::generated::mono::geo::Point) -> ();
            // mono::geo::Rect::Rect(const mono::geo::Rect &)
            fn stub__ZN4mono3geo4RectC1ERKS1_(o: *mut u8, p0: &::generated::mono::geo::Rect) -> ();
            // mono::geo::Rect::Rect(mono::geo::Point &, mono::geo::Size &)
            fn stub__ZN4mono3geo4RectC1ERNS0_5PointERNS0_4SizeE(o: *mut u8, p0: &::generated::mono::geo::Point, p1: &::generated::mono::geo::Size) -> ();
            // mono::geo::Rect::Rect(int, int, int, int)
            fn stub__ZN4mono3geo4RectC1Eiiii(o: *mut u8, p0: i32, p1: i32, p2: i32, p3: i32) -> ();
            // mono::geo::Rect::Rect()
            fn stub__ZN4mono3geo4RectC1Ev(o: *mut u8) -> ();
            // mono::geo::Rect::operator=(mono::geo::Rect)
            fn stub__ZN4mono3geo4RectaSES1_(o: *mut u8, p0: ::generated::mono::geo::Rect) -> &::generated::mono::geo::Rect;
            // mono::geo::Size::setWidth(int)
            fn stub__ZN4mono3geo4Size8setWidthEi(o: *mut u8, p0: i32) -> ();
            // mono::geo::Size::setHeight(int)
            fn stub__ZN4mono3geo4Size9setHeightEi(o: *mut u8, p0: i32) -> ();
            // mono::geo::Size::Size(const mono::geo::Size &)
            fn stub__ZN4mono3geo4SizeC1ERKS1_(o: *mut u8, p0: &::generated::mono::geo::Size) -> ();
            // mono::geo::Size::Size(int, int)
            fn stub__ZN4mono3geo4SizeC1Eii(o: *mut u8, p0: i32, p1: i32) -> ();
            // mono::geo::Size::Size()
            fn stub__ZN4mono3geo4SizeC1Ev(o: *mut u8) -> ();
            // mono::geo::Size::operator=(const mono::geo::Size &)
            fn stub__ZN4mono3geo4SizeaSERKS1_(o: *mut u8, p0: &::generated::mono::geo::Size) -> &::generated::mono::geo::Size;
            // mono::geo::Point::setX(int)
            fn stub__ZN4mono3geo5Point4setXEi(o: *mut u8, p0: i32) -> ();
            // mono::geo::Point::setY(int)
            fn stub__ZN4mono3geo5Point4setYEi(o: *mut u8, p0: i32) -> ();
            // mono::geo::Point::appendX(int)
            fn stub__ZN4mono3geo5Point7appendXEi(o: *mut u8, p0: i32) -> ();
            // mono::geo::Point::appendY(int)
            fn stub__ZN4mono3geo5Point7appendYEi(o: *mut u8, p0: i32) -> ();
            // mono::geo::Point::Point(const mono::geo::Point &)
            fn stub__ZN4mono3geo5PointC1ERKS1_(o: *mut u8, p0: &::generated::mono::geo::Point) -> ();
            // mono::geo::Point::Point(int, int)
            fn stub__ZN4mono3geo5PointC1Eii(o: *mut u8, p0: i32, p1: i32) -> ();
            // mono::geo::Point::Point()
            fn stub__ZN4mono3geo5PointC1Ev(o: *mut u8) -> ();
            // mono::geo::Point::operator=(const mono::geo::Point &)
            fn stub__ZN4mono3geo5PointaSERKS1_(o: *mut u8, p0: &::generated::mono::geo::Point) -> &::generated::mono::geo::Point;
            // mono::geo::Circle::Circle(mono::geo::Point, uint32_t)
            fn stub__ZN4mono3geo6CircleC1ENS0_5PointEj(o: *mut u8, p0: ::generated::mono::geo::Point, p1: u32) -> ();
            // mono::geo::Circle::Circle(int, int, uint32_t)
            fn stub__ZN4mono3geo6CircleC1Eiij(o: *mut u8, p0: i32, p1: i32, p2: u32) -> ();
            // mono::geo::Circle::Circle()
            fn stub__ZN4mono3geo6CircleC1Ev(o: *mut u8) -> ();
            // mono::geo::Rect::LowerRight()
            fn stub__ZNK4mono3geo4Rect10LowerRightEv(o: *mut u8) -> ::generated::mono::geo::Point;
            // mono::geo::Rect::UpperRight()
            fn stub__ZNK4mono3geo4Rect10UpperRightEv(o: *mut u8) -> ::generated::mono::geo::Point;
            // mono::geo::Rect::X2()
            fn stub__ZNK4mono3geo4Rect2X2Ev(o: *mut u8) -> i32;
            // mono::geo::Rect::Y2()
            fn stub__ZNK4mono3geo4Rect2Y2Ev(o: *mut u8) -> i32;
            // mono::geo::Rect::crop(const mono::geo::Rect &)
            fn stub__ZNK4mono3geo4Rect4cropERKS1_(o: *mut u8, p0: &::generated::mono::geo::Rect) -> ::generated::mono::geo::Rect;
            // mono::geo::Rect::Point()
            fn stub__ZNK4mono3geo4Rect5PointEv(o: *mut u8) -> ::generated::mono::geo::Point;
            // mono::geo::Rect::Center()
            fn stub__ZNK4mono3geo4Rect6CenterEv(o: *mut u8) -> ::generated::mono::geo::Point;
            // mono::geo::Rect::ToString()
            fn stub__ZNK4mono3geo4Rect8ToStringEv(o: *mut u8) -> ::generated::mono::String;
            // mono::geo::Rect::contains(const mono::geo::Rect &)
            fn stub__ZNK4mono3geo4Rect8containsERKS1_(o: *mut u8, p0: &::generated::mono::geo::Rect) -> bool;
            // mono::geo::Rect::contains(class Point &)
            fn stub__ZNK4mono3geo4Rect8containsERNS0_5PointE(o: *mut u8, p0: &::generated::mono::geo::Point) -> bool;
            // mono::geo::Rect::LowerLeft()
            fn stub__ZNK4mono3geo4Rect9LowerLeftEv(o: *mut u8) -> ::generated::mono::geo::Point;
            // mono::geo::Rect::UpperLeft()
            fn stub__ZNK4mono3geo4Rect9UpperLeftEv(o: *mut u8) -> ::generated::mono::geo::Point;
            // mono::geo::Size::Width()
            fn stub__ZNK4mono3geo4Size5WidthEv(o: *mut u8) -> i32;
            // mono::geo::Size::Height()
            fn stub__ZNK4mono3geo4Size6HeightEv(o: *mut u8) -> i32;
            // mono::geo::Point::X()
            fn stub__ZNK4mono3geo5Point1XEv(o: *mut u8) -> i32;
            // mono::geo::Point::Y()
            fn stub__ZNK4mono3geo5Point1YEv(o: *mut u8) -> i32;
            // mono::geo::Point::Abs()
            fn stub__ZNK4mono3geo5Point3AbsEv(o: *mut u8) -> u32;
            // mono::geo::Point::toString()
            fn stub__ZNK4mono3geo5Point8toStringEv(o: *mut u8) -> ::generated::mono::String;
            // mono::geo::Circle::Radius()
            fn stub__ZNK4mono3geo6Circle6RadiusEv(o: *mut u8) -> u32;
        }
        impl Circle {
            pub fn Radius(&mut self) -> u32 {
                unsafe { stub__ZNK4mono3geo6Circle6RadiusEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn new() -> ::generated::mono::geo::Circle {
                let mut o = Circle { raw: [0; 12] };
                unsafe { stub__ZN4mono3geo6CircleC1Ev(&mut o.raw[0] as *mut u8); }
                o
            }
            pub fn new_NS0_5PointEj(p0: ::generated::mono::geo::Point, p1: u32) -> ::generated::mono::geo::Circle {
                let mut o = Circle { raw: [0; 12] };
                unsafe { stub__ZN4mono3geo6CircleC1ENS0_5PointEj(&mut o.raw[0] as *mut u8, p0, p1); }
                o
            }
            pub fn new_iij(p0: i32, p1: i32, p2: u32) -> ::generated::mono::geo::Circle {
                let mut o = Circle { raw: [0; 12] };
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
            pub fn new() -> ::generated::mono::geo::Point {
                let mut o = Point { raw: [0; 8] };
                unsafe { stub__ZN4mono3geo5PointC1Ev(&mut o.raw[0] as *mut u8); }
                o
            }
            pub fn new_RKS1_(p0: &::generated::mono::geo::Point) -> ::generated::mono::geo::Point {
                let mut o = Point { raw: [0; 8] };
                unsafe { stub__ZN4mono3geo5PointC1ERKS1_(&mut o.raw[0] as *mut u8, p0); }
                o
            }
            pub fn new_ii(p0: i32, p1: i32) -> ::generated::mono::geo::Point {
                let mut o = Point { raw: [0; 8] };
                unsafe { stub__ZN4mono3geo5PointC1Eii(&mut o.raw[0] as *mut u8, p0, p1); }
                o
            }
            pub fn setX(&mut self, p0: i32) -> () {
                unsafe { stub__ZN4mono3geo5Point4setXEi(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setY(&mut self, p0: i32) -> () {
                unsafe { stub__ZN4mono3geo5Point4setYEi(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn toString(&mut self) -> ::generated::mono::String {
                unsafe { stub__ZNK4mono3geo5Point8toStringEv(&mut self.raw[0] as *mut u8) }
            }
        }
        impl Rect {
            pub fn Area(&mut self) -> i32 {
                unsafe { stub__ZN4mono3geo4Rect4AreaEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Center(&mut self) -> ::generated::mono::geo::Point {
                unsafe { stub__ZNK4mono3geo4Rect6CenterEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn LowerLeft(&mut self) -> ::generated::mono::geo::Point {
                unsafe { stub__ZNK4mono3geo4Rect9LowerLeftEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn LowerRight(&mut self) -> ::generated::mono::geo::Point {
                unsafe { stub__ZNK4mono3geo4Rect10LowerRightEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Point(&mut self) -> ::generated::mono::geo::Point {
                unsafe { stub__ZNK4mono3geo4Rect5PointEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Size(&mut self) -> &::generated::mono::geo::Size {
                unsafe { stub__ZN4mono3geo4Rect4SizeEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn ToString(&mut self) -> ::generated::mono::String {
                unsafe { stub__ZNK4mono3geo4Rect8ToStringEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn UpperLeft(&mut self) -> ::generated::mono::geo::Point {
                unsafe { stub__ZNK4mono3geo4Rect9UpperLeftEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn UpperRight(&mut self) -> ::generated::mono::geo::Point {
                unsafe { stub__ZNK4mono3geo4Rect10UpperRightEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn X2(&mut self) -> i32 {
                unsafe { stub__ZNK4mono3geo4Rect2X2Ev(&mut self.raw[0] as *mut u8) }
            }
            pub fn Y2(&mut self) -> i32 {
                unsafe { stub__ZNK4mono3geo4Rect2Y2Ev(&mut self.raw[0] as *mut u8) }
            }
            pub fn contains_KS1_(&mut self, p0: &::generated::mono::geo::Rect) -> bool {
                unsafe { stub__ZNK4mono3geo4Rect8containsERKS1_(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn contains_NS0_5PointE(&mut self, p0: &::generated::mono::geo::Point) -> bool {
                unsafe { stub__ZNK4mono3geo4Rect8containsERNS0_5PointE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn crop(&mut self, p0: &::generated::mono::geo::Rect) -> ::generated::mono::geo::Rect {
                unsafe { stub__ZNK4mono3geo4Rect4cropERKS1_(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn new() -> ::generated::mono::geo::Rect {
                let mut o = Rect { raw: [0; 16] };
                unsafe { stub__ZN4mono3geo4RectC1Ev(&mut o.raw[0] as *mut u8); }
                o
            }
            pub fn new_RKS1_(p0: &::generated::mono::geo::Rect) -> ::generated::mono::geo::Rect {
                let mut o = Rect { raw: [0; 16] };
                unsafe { stub__ZN4mono3geo4RectC1ERKS1_(&mut o.raw[0] as *mut u8, p0); }
                o
            }
            pub fn new_RNS0_5PointERNS0_4SizeE(p0: &::generated::mono::geo::Point, p1: &::generated::mono::geo::Size) -> ::generated::mono::geo::Rect {
                let mut o = Rect { raw: [0; 16] };
                unsafe { stub__ZN4mono3geo4RectC1ERNS0_5PointERNS0_4SizeE(&mut o.raw[0] as *mut u8, p0, p1); }
                o
            }
            pub fn new_iiii(p0: i32, p1: i32, p2: i32, p3: i32) -> ::generated::mono::geo::Rect {
                let mut o = Rect { raw: [0; 16] };
                unsafe { stub__ZN4mono3geo4RectC1Eiiii(&mut o.raw[0] as *mut u8, p0, p1, p2, p3); }
                o
            }
            pub fn setPoint(&mut self, p0: ::generated::mono::geo::Point) -> () {
                unsafe { stub__ZN4mono3geo4Rect8setPointENS0_5PointE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setSize(&mut self, p0: ::generated::mono::geo::Size) -> () {
                unsafe { stub__ZN4mono3geo4Rect7setSizeENS0_4SizeE(&mut self.raw[0] as *mut u8, p0) }
            }
        }
        impl Size {
            pub fn Height(&mut self) -> i32 {
                unsafe { stub__ZNK4mono3geo4Size6HeightEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Width(&mut self) -> i32 {
                unsafe { stub__ZNK4mono3geo4Size5WidthEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn new() -> ::generated::mono::geo::Size {
                let mut o = Size { raw: [0; 8] };
                unsafe { stub__ZN4mono3geo4SizeC1Ev(&mut o.raw[0] as *mut u8); }
                o
            }
            pub fn new_RKS1_(p0: &::generated::mono::geo::Size) -> ::generated::mono::geo::Size {
                let mut o = Size { raw: [0; 8] };
                unsafe { stub__ZN4mono3geo4SizeC1ERKS1_(&mut o.raw[0] as *mut u8, p0); }
                o
            }
            pub fn new_ii(p0: i32, p1: i32) -> ::generated::mono::geo::Size {
                let mut o = Size { raw: [0; 8] };
                unsafe { stub__ZN4mono3geo4SizeC1Eii(&mut o.raw[0] as *mut u8, p0, p1); }
                o
            }
            pub fn setHeight(&mut self, p0: i32) -> () {
                unsafe { stub__ZN4mono3geo4Size9setHeightEi(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setWidth(&mut self, p0: i32) -> () {
                unsafe { stub__ZN4mono3geo4Size8setWidthEi(&mut self.raw[0] as *mut u8, p0) }
            }
        }
    }
    pub mod ui {
        pub struct TextLabelView {
        #[allow(dead_code)]
            raw: [u8; 112],
        }
        pub struct View {
        #[allow(dead_code)]
            raw: [u8; 40],
        }
        #[allow(dead_code)]
        extern "C" {
            // mono::ui::TextLabelView::setTextSize(uint8_t)
            fn stub__ZN4mono2ui13TextLabelView11setTextSizeEh(o: *mut u8, p0: u8) -> ();
            // mono::ui::TextLabelView::setAlignment(mono::ui::TextLabelView::TextAlignment)
            fn stub__ZN4mono2ui13TextLabelView12setAlignmentENS1_13TextAlignmentE(o: *mut u8, p0: i32) -> ();
            // mono::ui::TextLabelView::setTextColor(display::Color)
            fn stub__ZN4mono2ui13TextLabelView12setTextColorENS_7display5ColorE(o: *mut u8, p0: ::generated::mono::display::Color) -> ();
            // mono::ui::TextLabelView::setBackground(display::Color)
            fn stub__ZN4mono2ui13TextLabelView13setBackgroundENS_7display5ColorE(o: *mut u8, p0: ::generated::mono::display::Color) -> ();
            // mono::ui::TextLabelView::scheduleRepaint()
            fn stub__ZN4mono2ui13TextLabelView15scheduleRepaintEv(o: *mut u8) -> ();
            // mono::ui::TextLabelView::setBackgroundColor(display::Color)
            fn stub__ZN4mono2ui13TextLabelView18setBackgroundColorENS_7display5ColorE(o: *mut u8, p0: ::generated::mono::display::Color) -> ();
            // mono::ui::TextLabelView::repaint()
            fn stub__ZN4mono2ui13TextLabelView7repaintEv(o: *mut u8) -> ();
            // mono::ui::TextLabelView::setFont(const MonoFont &)
            fn stub__ZN4mono2ui13TextLabelView7setFontERK8MonoFont(o: *mut u8, p0: &::generated::MonoFont) -> ();
            // mono::ui::TextLabelView::setText(mono::String, bool)
            fn stub__ZN4mono2ui13TextLabelView7setTextENS_6StringEb(o: *mut u8, p0: ::generated::mono::String, p1: bool) -> ();
            // mono::ui::TextLabelView::setText(display::Color)
            fn stub__ZN4mono2ui13TextLabelView7setTextENS_7display5ColorE(o: *mut u8, p0: ::generated::mono::display::Color) -> ();
            // mono::ui::TextLabelView::setText(const char *, bool)
            fn stub__ZN4mono2ui13TextLabelView7setTextEPKcb(o: *mut u8, p0: *mut i8, p1: bool) -> ();
            // mono::ui::TextLabelView::setText(char *, bool)
            fn stub__ZN4mono2ui13TextLabelView7setTextEPcb(o: *mut u8, p0: *mut i8, p1: bool) -> ();
            // mono::ui::View::DisplaySize()
            fn stub__ZN4mono2ui4View11DisplaySizeEv(o: *mut u8) -> ::generated::mono::geo::Size;
            // mono::ui::View::setPosition(geo::Point)
            fn stub__ZN4mono2ui4View11setPositionENS_3geo5PointE(o: *mut u8, p0: ::generated::mono::geo::Point) -> ();
            // mono::ui::View::DisplayWidth()
            fn stub__ZN4mono2ui4View12DisplayWidthEv(o: *mut u8) -> u16;
            // mono::ui::View::DisplayHeight()
            fn stub__ZN4mono2ui4View13DisplayHeightEv(o: *mut u8) -> u16;
            // mono::ui::View::scheduleRepaint()
            fn stub__ZN4mono2ui4View15scheduleRepaintEv(o: *mut u8) -> ();
            // mono::ui::View::DisplayOrientation()
            fn stub__ZN4mono2ui4View18DisplayOrientationEv(o: *mut u8) -> i32;
            // mono::ui::View::Size()
            fn stub__ZN4mono2ui4View4SizeEv(o: *mut u8) -> &::generated::mono::geo::Size;
            // mono::ui::View::hide()
            fn stub__ZN4mono2ui4View4hideEv(o: *mut u8) -> ();
            // mono::ui::View::show()
            fn stub__ZN4mono2ui4View4showEv(o: *mut u8) -> ();
            // mono::ui::View::setRect(geo::Rect)
            fn stub__ZN4mono2ui4View7setRectENS_3geo4RectE(o: *mut u8, p0: ::generated::mono::geo::Rect) -> ();
            // mono::ui::View::setSize(geo::Size)
            fn stub__ZN4mono2ui4View7setSizeENS_3geo4SizeE(o: *mut u8, p0: ::generated::mono::geo::Size) -> ();
            // mono::ui::View::Position()
            fn stub__ZN4mono2ui4View8PositionEv(o: *mut u8) -> &::generated::mono::geo::Point;
            // mono::ui::View::~View()
            fn stub__ZN4mono2ui4ViewD0Ev(o: *mut u8) -> ();
            // mono::ui::TextLabelView::TextPixelWidth()
            fn stub__ZNK4mono2ui13TextLabelView14TextPixelWidthEv(o: *mut u8) -> u16;
            // mono::ui::TextLabelView::TextPixelHeight()
            fn stub__ZNK4mono2ui13TextLabelView15TextPixelHeightEv(o: *mut u8) -> u16;
            // mono::ui::TextLabelView::Font()
            fn stub__ZNK4mono2ui13TextLabelView4FontEv(o: *mut u8) -> &::generated::MonoFont;
            // mono::ui::TextLabelView::TextSize()
            fn stub__ZNK4mono2ui13TextLabelView8TextSizeEv(o: *mut u8) -> u8;
            // mono::ui::TextLabelView::Alignment()
            fn stub__ZNK4mono2ui13TextLabelView9AlignmentEv(o: *mut u8) -> i32;
            // mono::ui::TextLabelView::TextColor()
            fn stub__ZNK4mono2ui13TextLabelView9TextColorEv(o: *mut u8) -> ::generated::mono::display::Color;
            // mono::ui::View::Visible()
            fn stub__ZNK4mono2ui4View7VisibleEv(o: *mut u8) -> bool;
            // mono::ui::View::ViewRect()
            fn stub__ZNK4mono2ui4View8ViewRectEv(o: *mut u8) -> &::generated::mono::geo::Rect;
        }
        impl Drop for View {
             fn drop(&mut self) -> () {
                unsafe { stub__ZN4mono2ui4ViewD0Ev(&mut self.raw[0] as *mut u8) }
            }
        }
        impl TextLabelView {
            pub fn Alignment(&mut self) -> i32 {
                unsafe { stub__ZNK4mono2ui13TextLabelView9AlignmentEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Font(&mut self) -> &::generated::MonoFont {
                unsafe { stub__ZNK4mono2ui13TextLabelView4FontEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn TextColor(&mut self) -> ::generated::mono::display::Color {
                unsafe { stub__ZNK4mono2ui13TextLabelView9TextColorEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn TextPixelHeight(&mut self) -> u16 {
                unsafe { stub__ZNK4mono2ui13TextLabelView15TextPixelHeightEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn TextPixelWidth(&mut self) -> u16 {
                unsafe { stub__ZNK4mono2ui13TextLabelView14TextPixelWidthEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn TextSize(&mut self) -> u8 {
                unsafe { stub__ZNK4mono2ui13TextLabelView8TextSizeEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn repaint(&mut self) -> () {
                unsafe { stub__ZN4mono2ui13TextLabelView7repaintEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn scheduleRepaint(&mut self) -> () {
                unsafe { stub__ZN4mono2ui13TextLabelView15scheduleRepaintEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn setAlignment(&mut self, p0: i32) -> () {
                unsafe { stub__ZN4mono2ui13TextLabelView12setAlignmentENS1_13TextAlignmentE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setBackground(&mut self, p0: ::generated::mono::display::Color) -> () {
                unsafe { stub__ZN4mono2ui13TextLabelView13setBackgroundENS_7display5ColorE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setBackgroundColor(&mut self, p0: ::generated::mono::display::Color) -> () {
                unsafe { stub__ZN4mono2ui13TextLabelView18setBackgroundColorENS_7display5ColorE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setFont(&mut self, p0: &::generated::MonoFont) -> () {
                unsafe { stub__ZN4mono2ui13TextLabelView7setFontERK8MonoFont(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setTextColor(&mut self, p0: ::generated::mono::display::Color) -> () {
                unsafe { stub__ZN4mono2ui13TextLabelView12setTextColorENS_7display5ColorE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setTextSize(&mut self, p0: u8) -> () {
                unsafe { stub__ZN4mono2ui13TextLabelView11setTextSizeEh(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setText_NS_6StringEb(&mut self, p0: ::generated::mono::String, p1: bool) -> () {
                unsafe { stub__ZN4mono2ui13TextLabelView7setTextENS_6StringEb(&mut self.raw[0] as *mut u8, p0, p1) }
            }
            pub fn setText_NS_7display5ColorE(&mut self, p0: ::generated::mono::display::Color) -> () {
                unsafe { stub__ZN4mono2ui13TextLabelView7setTextENS_7display5ColorE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setText_PKcb(&mut self, p0: *mut i8, p1: bool) -> () {
                unsafe { stub__ZN4mono2ui13TextLabelView7setTextEPKcb(&mut self.raw[0] as *mut u8, p0, p1) }
            }
            pub fn setText_Pcb(&mut self, p0: *mut i8, p1: bool) -> () {
                unsafe { stub__ZN4mono2ui13TextLabelView7setTextEPcb(&mut self.raw[0] as *mut u8, p0, p1) }
            }
        }
        impl View {
            pub fn DisplayHeight(&mut self) -> u16 {
                unsafe { stub__ZN4mono2ui4View13DisplayHeightEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn DisplayOrientation(&mut self) -> i32 {
                unsafe { stub__ZN4mono2ui4View18DisplayOrientationEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn DisplaySize(&mut self) -> ::generated::mono::geo::Size {
                unsafe { stub__ZN4mono2ui4View11DisplaySizeEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn DisplayWidth(&mut self) -> u16 {
                unsafe { stub__ZN4mono2ui4View12DisplayWidthEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Position(&mut self) -> &::generated::mono::geo::Point {
                unsafe { stub__ZN4mono2ui4View8PositionEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Size(&mut self) -> &::generated::mono::geo::Size {
                unsafe { stub__ZN4mono2ui4View4SizeEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn ViewRect(&mut self) -> &::generated::mono::geo::Rect {
                unsafe { stub__ZNK4mono2ui4View8ViewRectEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Visible(&mut self) -> bool {
                unsafe { stub__ZNK4mono2ui4View7VisibleEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn hide(&mut self) -> () {
                unsafe { stub__ZN4mono2ui4View4hideEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn scheduleRepaint(&mut self) -> () {
                unsafe { stub__ZN4mono2ui4View15scheduleRepaintEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn setPosition(&mut self, p0: ::generated::mono::geo::Point) -> () {
                unsafe { stub__ZN4mono2ui4View11setPositionENS_3geo5PointE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setRect(&mut self, p0: ::generated::mono::geo::Rect) -> () {
                unsafe { stub__ZN4mono2ui4View7setRectENS_3geo4RectE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setSize(&mut self, p0: ::generated::mono::geo::Size) -> () {
                unsafe { stub__ZN4mono2ui4View7setSizeENS_3geo4SizeE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn show(&mut self) -> () {
                unsafe { stub__ZN4mono2ui4View4showEv(&mut self.raw[0] as *mut u8) }
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
        fn stub__ZN4mono19IApplicationContext18setMonoApplicationEPNS_12IApplicationE(o: *mut u8, p0: *mut ::generated::mono::IApplication) -> ();
        // mono::IApplicationContext::SoftwareResetToBootloader()
        fn stub__ZN4mono19IApplicationContext25SoftwareResetToBootloaderEv(o: *mut u8) -> ();
        // mono::IApplicationContext::SoftwareResetToApplication()
        fn stub__ZN4mono19IApplicationContext26SoftwareResetToApplicationEv(o: *mut u8) -> ();
        // mono::IApplicationContext::exec()
        fn stub__ZN4mono19IApplicationContext4execEv(o: *mut u8) -> i32;
        // mono::String::preAllocbytes(uint32_t)
        fn stub__ZN4mono6String13preAllocbytesEj(o: *mut u8, p0: u32) -> ();
        // mono::String::Format(const char *, ...)
        fn stub__ZN4mono6String6FormatEPKcz(o: *mut u8, p0: *mut i8) -> ::generated::mono::String;
        // mono::String::String(const char *)
        fn stub__ZN4mono6StringC1EPKc(o: *mut u8, p0: *mut i8) -> ();
        // mono::String::String(char *)
        fn stub__ZN4mono6StringC1EPc(o: *mut u8, p0: *mut i8) -> ();
        // mono::String::String(char *, uint32_t)
        fn stub__ZN4mono6StringC1EPcj(o: *mut u8, p0: *mut i8, p1: u32) -> ();
        // mono::String::String(const mono::String &)
        fn stub__ZN4mono6StringC1ERKS0_(o: *mut u8, p0: &::generated::mono::String) -> ();
        // mono::String::String(uint32_t)
        fn stub__ZN4mono6StringC1Ej(o: *mut u8, p0: u32) -> ();
        // mono::String::String()
        fn stub__ZN4mono6StringC1Ev(o: *mut u8) -> ();
        // mono::String::~String()
        fn stub__ZN4mono6StringD0Ev(o: *mut u8) -> ();
        // mono::String::operator=(const char *)
        fn stub__ZN4mono6StringaSEPKc(o: *mut u8, p0: *mut i8) -> &::generated::mono::String;
        // mono::String::operator=(const mono::String &)
        fn stub__ZN4mono6StringaSERKS0_(o: *mut u8, p0: &::generated::mono::String) -> &::generated::mono::String;
        // mono::String::operator==(const char *)
        fn stub__ZN4mono6StringeqEPKc(o: *mut u8, p0: *mut i8) -> bool;
        // mono::String::operator==(const mono::String &)
        fn stub__ZN4mono6StringeqERKS0_(o: *mut u8, p0: &::generated::mono::String) -> bool;
        // mono::String::operator!=(const char *)
        fn stub__ZN4mono6StringneEPKc(o: *mut u8, p0: *mut i8) -> bool;
        // mono::String::operator!=(const mono::String &)
        fn stub__ZN4mono6StringneERKS0_(o: *mut u8, p0: &::generated::mono::String) -> bool;
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
        pub fn setMonoApplication(&mut self, p0: *mut ::generated::mono::IApplication) -> () {
            unsafe { stub__ZN4mono19IApplicationContext18setMonoApplicationEPNS_12IApplicationE(&mut self.raw[0] as *mut u8, p0) }
        }
    }
    impl String {
        pub fn Format(&mut self, p0: *mut i8) -> ::generated::mono::String {
            unsafe { stub__ZN4mono6String6FormatEPKcz(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn Length(&mut self) -> u32 {
            unsafe { stub__ZNK4mono6String6LengthEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn new() -> ::generated::mono::String {
            let mut o = String { raw: [0; 24] };
            unsafe { stub__ZN4mono6StringC1Ev(&mut o.raw[0] as *mut u8); }
            o
        }
        pub fn new_PKc(p0: *mut i8) -> ::generated::mono::String {
            let mut o = String { raw: [0; 24] };
            unsafe { stub__ZN4mono6StringC1EPKc(&mut o.raw[0] as *mut u8, p0); }
            o
        }
        pub fn new_Pc(p0: *mut i8) -> ::generated::mono::String {
            let mut o = String { raw: [0; 24] };
            unsafe { stub__ZN4mono6StringC1EPc(&mut o.raw[0] as *mut u8, p0); }
            o
        }
        pub fn new_Pcj(p0: *mut i8, p1: u32) -> ::generated::mono::String {
            let mut o = String { raw: [0; 24] };
            unsafe { stub__ZN4mono6StringC1EPcj(&mut o.raw[0] as *mut u8, p0, p1); }
            o
        }
        pub fn new_RKS0_(p0: &::generated::mono::String) -> ::generated::mono::String {
            let mut o = String { raw: [0; 24] };
            unsafe { stub__ZN4mono6StringC1ERKS0_(&mut o.raw[0] as *mut u8, p0); }
            o
        }
        pub fn new_j(p0: u32) -> ::generated::mono::String {
            let mut o = String { raw: [0; 24] };
            unsafe { stub__ZN4mono6StringC1Ej(&mut o.raw[0] as *mut u8, p0); }
            o
        }
        pub fn preAllocbytes(&mut self, p0: u32) -> () {
            unsafe { stub__ZN4mono6String13preAllocbytesEj(&mut self.raw[0] as *mut u8, p0) }
        }
    }
}
pub struct MonoFont {
#[allow(dead_code)]
    raw: [u8; 32],
}
#[allow(dead_code)]
extern "C" {
}
