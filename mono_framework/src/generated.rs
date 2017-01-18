#![allow(non_snake_case)]
#![allow(improper_ctypes)]
pub mod mbed {
    #[allow(dead_code)]
    pub struct DigitalOut {
        raw: [u8],
    }
    #[allow(dead_code)]
    pub struct Serial {
        raw: [u8],
    }
    #[allow(dead_code)]
    extern "C" {
    }
}
pub mod mono {
    pub mod display {
        #[allow(dead_code)]
        pub struct Color {
            raw: [u8; 2],
        }
        #[allow(dead_code)]
        pub struct DisplayPainter {
            raw: [u8; 64],
        }
        #[allow(dead_code)]
        pub struct IDisplayController {
            raw: [u8],
        }
        #[allow(dead_code)]
        extern "C" {
            // mono::display::DisplayPainter::drawAALine(const geo::Point &, const geo::Point &, bool)
            fn stub__ZN4mono7display14DisplayPainter10drawAALineERKNS_3geo5PointES5_b(o: *mut u8, p0: &::generated::mono::geo::Point, p1: &::generated::mono::geo::Point, p2: bool) -> ();
            // mono::display::DisplayPainter::drawAALine(uint16_t, uint16_t, uint16_t, uint16_t, bool)
            fn stub__ZN4mono7display14DisplayPainter10drawAALineEttttb(o: *mut u8, p0: u16, p1: u16, p2: u16, p3: u16, p4: bool) -> ();
            // mono::display::DisplayPainter::drawCircle(const geo::Circle &, bool)
            fn stub__ZN4mono7display14DisplayPainter10drawCircleERKNS_3geo6CircleEb(o: *mut u8, p0: &::generated::mono::geo::Circle, p1: bool) -> ();
            // mono::display::DisplayPainter::drawCircle(uint16_t, uint16_t, uint16_t, bool)
            fn stub__ZN4mono7display14DisplayPainter10drawCircleEtttb(o: *mut u8, p0: u16, p1: u16, p2: u16, p3: bool) -> ();
            // mono::display::DisplayPainter::setTextSize(uint8_t)
            fn stub__ZN4mono7display14DisplayPainter11setTextSizeEh(o: *mut u8, p0: u8) -> ();
            // mono::display::DisplayPainter::drawFillRect(const geo::Rect &, bool)
            fn stub__ZN4mono7display14DisplayPainter12drawFillRectERKNS_3geo4RectEb(o: *mut u8, p0: &::generated::mono::geo::Rect, p1: bool) -> ();
            // mono::display::DisplayPainter::drawFillRect(uint16_t, uint16_t, uint16_t, uint16_t, bool)
            fn stub__ZN4mono7display14DisplayPainter12drawFillRectEttttb(o: *mut u8, p0: u16, p1: u16, p2: u16, p3: u16, p4: bool) -> ();
            // mono::display::DisplayPainter::setLineWidth(uint8_t)
            fn stub__ZN4mono7display14DisplayPainter12setLineWidthEh(o: *mut u8, p0: u8) -> ();
            // mono::display::DisplayPainter::drawFillCircle(const geo::Circle &, bool)
            fn stub__ZN4mono7display14DisplayPainter14drawFillCircleERKNS_3geo6CircleEb(o: *mut u8, p0: &::generated::mono::geo::Circle, p1: bool) -> ();
            // mono::display::DisplayPainter::drawFillCircle(uint16_t, uint16_t, uint16_t, bool)
            fn stub__ZN4mono7display14DisplayPainter14drawFillCircleEtttb(o: *mut u8, p0: u16, p1: u16, p2: u16, p3: bool) -> ();
            // mono::display::DisplayPainter::fillCircleHelper(int16_t, int16_t, int16_t, uint8_t, int16_t, bool)
            fn stub__ZN4mono7display14DisplayPainter16fillCircleHelperEssshsb(o: *mut u8, p0: i16, p1: i16, p2: i16, p3: u8, p4: i16, p5: bool) -> ();
            // mono::display::DisplayPainter::setBackgroundColor(mono::display::Color)
            fn stub__ZN4mono7display14DisplayPainter18setBackgroundColorENS0_5ColorE(o: *mut u8, p0: ::generated::mono::display::Color) -> ();
            // mono::display::DisplayPainter::setForegroundColor(mono::display::Color)
            fn stub__ZN4mono7display14DisplayPainter18setForegroundColorENS0_5ColorE(o: *mut u8, p0: ::generated::mono::display::Color) -> ();
            // mono::display::DisplayPainter::setRefreshCallback(void (*)())
            fn stub__ZN4mono7display14DisplayPainter18setRefreshCallbackEPFvvE(o: *mut u8, p0: *mut fn () -> ()) -> ();
            // mono::display::DisplayPainter::IsAntialiasedDrawing()
            fn stub__ZN4mono7display14DisplayPainter20IsAntialiasedDrawingEv(o: *mut u8) -> bool;
            // mono::display::DisplayPainter::useAntialiasedDrawing(bool)
            fn stub__ZN4mono7display14DisplayPainter21useAntialiasedDrawingEb(o: *mut u8, p0: bool) -> ();
            // mono::display::DisplayPainter::drawChar(uint16_t, uint16_t, char)
            fn stub__ZN4mono7display14DisplayPainter8drawCharEttc(o: *mut u8, p0: u16, p1: u16, p2: i8) -> ();
            // mono::display::DisplayPainter::drawLine(const geo::Point &, const geo::Point &, bool)
            fn stub__ZN4mono7display14DisplayPainter8drawLineERKNS_3geo5PointES5_b(o: *mut u8, p0: &::generated::mono::geo::Point, p1: &::generated::mono::geo::Point, p2: bool) -> ();
            // mono::display::DisplayPainter::drawLine(uint16_t, uint16_t, uint16_t, uint16_t, bool)
            fn stub__ZN4mono7display14DisplayPainter8drawLineEttttb(o: *mut u8, p0: u16, p1: u16, p2: u16, p3: u16, p4: bool) -> ();
            // mono::display::DisplayPainter::drawRect(const geo::Rect &, bool)
            fn stub__ZN4mono7display14DisplayPainter8drawRectERKNS_3geo4RectEb(o: *mut u8, p0: &::generated::mono::geo::Rect, p1: bool) -> ();
            // mono::display::DisplayPainter::drawRect(uint16_t, uint16_t, uint16_t, uint16_t, bool)
            fn stub__ZN4mono7display14DisplayPainter8drawRectEttttb(o: *mut u8, p0: u16, p1: u16, p2: u16, p3: u16, p4: bool) -> ();
            // mono::display::DisplayPainter::drawHLine(uint16_t, uint16_t, uint16_t, bool)
            fn stub__ZN4mono7display14DisplayPainter9drawHLineEtttb(o: *mut u8, p0: u16, p1: u16, p2: u16, p3: bool) -> ();
            // mono::display::DisplayPainter::drawPixel(const geo::Point &, bool)
            fn stub__ZN4mono7display14DisplayPainter9drawPixelERKNS_3geo5PointEb(o: *mut u8, p0: &::generated::mono::geo::Point, p1: bool) -> ();
            // mono::display::DisplayPainter::drawPixel(uint16_t, uint16_t, bool)
            fn stub__ZN4mono7display14DisplayPainter9drawPixelEttb(o: *mut u8, p0: u16, p1: u16, p2: bool) -> ();
            // mono::display::DisplayPainter::drawPixel(uint16_t, uint16_t, uint8_t, bool)
            fn stub__ZN4mono7display14DisplayPainter9drawPixelEtthb(o: *mut u8, p0: u16, p1: u16, p2: u8, p3: bool) -> ();
            // mono::display::DisplayPainter::drawVLine(uint16_t, uint16_t, uint16_t, bool)
            fn stub__ZN4mono7display14DisplayPainter9drawVLineEtttb(o: *mut u8, p0: u16, p1: u16, p2: u16, p3: bool) -> ();
            // mono::display::DisplayPainter::DisplayPainter(mono::display::IDisplayController *, bool)
            fn stub__ZN4mono7display14DisplayPainterC1EPNS0_18IDisplayControllerEb(o: *mut u8, p0: *mut ::generated::mono::display::IDisplayController, p1: bool) -> ();
            // mono::display::DisplayPainter::~DisplayPainter()
            fn stub__ZN4mono7display14DisplayPainterD0Ev(o: *mut u8) -> ();
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
            // mono::display::DisplayPainter::CanvasWidth()
            fn stub__ZNK4mono7display14DisplayPainter11CanvasWidthEv(o: *mut u8) -> u16;
            // mono::display::DisplayPainter::CanvasHeight()
            fn stub__ZNK4mono7display14DisplayPainter12CanvasHeightEv(o: *mut u8) -> u16;
            // mono::display::DisplayPainter::BackgroundColor()
            fn stub__ZNK4mono7display14DisplayPainter15BackgroundColorEv(o: *mut u8) -> ::generated::mono::display::Color;
            // mono::display::DisplayPainter::ForegroundColor()
            fn stub__ZNK4mono7display14DisplayPainter15ForegroundColorEv(o: *mut u8) -> ::generated::mono::display::Color;
            // mono::display::DisplayPainter::DisplayController()
            fn stub__ZNK4mono7display14DisplayPainter17DisplayControllerEv(o: *mut u8) -> *mut ::generated::mono::display::IDisplayController;
            // mono::display::DisplayPainter::TextSize()
            fn stub__ZNK4mono7display14DisplayPainter8TextSizeEv(o: *mut u8) -> u8;
            // mono::display::DisplayPainter::LineWidth()
            fn stub__ZNK4mono7display14DisplayPainter9LineWidthEv(o: *mut u8) -> u8;
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
        impl DisplayPainter {
            pub fn BackgroundColor(&mut self) -> ::generated::mono::display::Color {
                unsafe { stub__ZNK4mono7display14DisplayPainter15BackgroundColorEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn CanvasHeight(&mut self) -> u16 {
                unsafe { stub__ZNK4mono7display14DisplayPainter12CanvasHeightEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn CanvasWidth(&mut self) -> u16 {
                unsafe { stub__ZNK4mono7display14DisplayPainter11CanvasWidthEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn DisplayController(&mut self) -> *mut ::generated::mono::display::IDisplayController {
                unsafe { stub__ZNK4mono7display14DisplayPainter17DisplayControllerEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn ForegroundColor(&mut self) -> ::generated::mono::display::Color {
                unsafe { stub__ZNK4mono7display14DisplayPainter15ForegroundColorEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn IsAntialiasedDrawing(&mut self) -> bool {
                unsafe { stub__ZN4mono7display14DisplayPainter20IsAntialiasedDrawingEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn LineWidth(&mut self) -> u8 {
                unsafe { stub__ZNK4mono7display14DisplayPainter9LineWidthEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn TextSize(&mut self) -> u8 {
                unsafe { stub__ZNK4mono7display14DisplayPainter8TextSizeEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn drawAALine_RKNS_3geo5PointES5_b(&mut self, p0: &::generated::mono::geo::Point, p1: &::generated::mono::geo::Point, p2: bool) -> () {
                unsafe { stub__ZN4mono7display14DisplayPainter10drawAALineERKNS_3geo5PointES5_b(&mut self.raw[0] as *mut u8, p0, p1, p2) }
            }
            pub fn drawAALine_ttttb(&mut self, p0: u16, p1: u16, p2: u16, p3: u16, p4: bool) -> () {
                unsafe { stub__ZN4mono7display14DisplayPainter10drawAALineEttttb(&mut self.raw[0] as *mut u8, p0, p1, p2, p3, p4) }
            }
            pub fn drawChar(&mut self, p0: u16, p1: u16, p2: i8) -> () {
                unsafe { stub__ZN4mono7display14DisplayPainter8drawCharEttc(&mut self.raw[0] as *mut u8, p0, p1, p2) }
            }
            pub fn drawCircle_RKNS_3geo6CircleEb(&mut self, p0: &::generated::mono::geo::Circle, p1: bool) -> () {
                unsafe { stub__ZN4mono7display14DisplayPainter10drawCircleERKNS_3geo6CircleEb(&mut self.raw[0] as *mut u8, p0, p1) }
            }
            pub fn drawCircle_tttb(&mut self, p0: u16, p1: u16, p2: u16, p3: bool) -> () {
                unsafe { stub__ZN4mono7display14DisplayPainter10drawCircleEtttb(&mut self.raw[0] as *mut u8, p0, p1, p2, p3) }
            }
            pub fn drawFillCircle_RKNS_3geo6CircleEb(&mut self, p0: &::generated::mono::geo::Circle, p1: bool) -> () {
                unsafe { stub__ZN4mono7display14DisplayPainter14drawFillCircleERKNS_3geo6CircleEb(&mut self.raw[0] as *mut u8, p0, p1) }
            }
            pub fn drawFillCircle_tttb(&mut self, p0: u16, p1: u16, p2: u16, p3: bool) -> () {
                unsafe { stub__ZN4mono7display14DisplayPainter14drawFillCircleEtttb(&mut self.raw[0] as *mut u8, p0, p1, p2, p3) }
            }
            pub fn drawFillRect_RKNS_3geo4RectEb(&mut self, p0: &::generated::mono::geo::Rect, p1: bool) -> () {
                unsafe { stub__ZN4mono7display14DisplayPainter12drawFillRectERKNS_3geo4RectEb(&mut self.raw[0] as *mut u8, p0, p1) }
            }
            pub fn drawFillRect_ttttb(&mut self, p0: u16, p1: u16, p2: u16, p3: u16, p4: bool) -> () {
                unsafe { stub__ZN4mono7display14DisplayPainter12drawFillRectEttttb(&mut self.raw[0] as *mut u8, p0, p1, p2, p3, p4) }
            }
            pub fn drawHLine(&mut self, p0: u16, p1: u16, p2: u16, p3: bool) -> () {
                unsafe { stub__ZN4mono7display14DisplayPainter9drawHLineEtttb(&mut self.raw[0] as *mut u8, p0, p1, p2, p3) }
            }
            pub fn drawLine_RKNS_3geo5PointES5_b(&mut self, p0: &::generated::mono::geo::Point, p1: &::generated::mono::geo::Point, p2: bool) -> () {
                unsafe { stub__ZN4mono7display14DisplayPainter8drawLineERKNS_3geo5PointES5_b(&mut self.raw[0] as *mut u8, p0, p1, p2) }
            }
            pub fn drawLine_ttttb(&mut self, p0: u16, p1: u16, p2: u16, p3: u16, p4: bool) -> () {
                unsafe { stub__ZN4mono7display14DisplayPainter8drawLineEttttb(&mut self.raw[0] as *mut u8, p0, p1, p2, p3, p4) }
            }
            pub fn drawPixel_RKNS_3geo5PointEb(&mut self, p0: &::generated::mono::geo::Point, p1: bool) -> () {
                unsafe { stub__ZN4mono7display14DisplayPainter9drawPixelERKNS_3geo5PointEb(&mut self.raw[0] as *mut u8, p0, p1) }
            }
            pub fn drawPixel_ttb(&mut self, p0: u16, p1: u16, p2: bool) -> () {
                unsafe { stub__ZN4mono7display14DisplayPainter9drawPixelEttb(&mut self.raw[0] as *mut u8, p0, p1, p2) }
            }
            pub fn drawPixel_tthb(&mut self, p0: u16, p1: u16, p2: u8, p3: bool) -> () {
                unsafe { stub__ZN4mono7display14DisplayPainter9drawPixelEtthb(&mut self.raw[0] as *mut u8, p0, p1, p2, p3) }
            }
            pub fn drawRect_RKNS_3geo4RectEb(&mut self, p0: &::generated::mono::geo::Rect, p1: bool) -> () {
                unsafe { stub__ZN4mono7display14DisplayPainter8drawRectERKNS_3geo4RectEb(&mut self.raw[0] as *mut u8, p0, p1) }
            }
            pub fn drawRect_ttttb(&mut self, p0: u16, p1: u16, p2: u16, p3: u16, p4: bool) -> () {
                unsafe { stub__ZN4mono7display14DisplayPainter8drawRectEttttb(&mut self.raw[0] as *mut u8, p0, p1, p2, p3, p4) }
            }
            pub fn drawVLine(&mut self, p0: u16, p1: u16, p2: u16, p3: bool) -> () {
                unsafe { stub__ZN4mono7display14DisplayPainter9drawVLineEtttb(&mut self.raw[0] as *mut u8, p0, p1, p2, p3) }
            }
            pub fn fillCircleHelper(&mut self, p0: i16, p1: i16, p2: i16, p3: u8, p4: i16, p5: bool) -> () {
                unsafe { stub__ZN4mono7display14DisplayPainter16fillCircleHelperEssshsb(&mut self.raw[0] as *mut u8, p0, p1, p2, p3, p4, p5) }
            }
            pub fn new(p0: *mut ::generated::mono::display::IDisplayController, p1: bool) -> ::generated::mono::display::DisplayPainter {
                let mut o = DisplayPainter { raw: [0; 64] };
                unsafe { stub__ZN4mono7display14DisplayPainterC1EPNS0_18IDisplayControllerEb(&mut o.raw[0] as *mut u8, p0, p1); }
                o
            }
            pub fn setBackgroundColor(&mut self, p0: ::generated::mono::display::Color) -> () {
                unsafe { stub__ZN4mono7display14DisplayPainter18setBackgroundColorENS0_5ColorE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setForegroundColor(&mut self, p0: ::generated::mono::display::Color) -> () {
                unsafe { stub__ZN4mono7display14DisplayPainter18setForegroundColorENS0_5ColorE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setLineWidth(&mut self, p0: u8) -> () {
                unsafe { stub__ZN4mono7display14DisplayPainter12setLineWidthEh(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setRefreshCallback(&mut self, p0: *mut fn () -> ()) -> () {
                unsafe { stub__ZN4mono7display14DisplayPainter18setRefreshCallbackEPFvvE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setTextSize(&mut self, p0: u8) -> () {
                unsafe { stub__ZN4mono7display14DisplayPainter11setTextSizeEh(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn useAntialiasedDrawing(&mut self, p0: bool) -> () {
                unsafe { stub__ZN4mono7display14DisplayPainter21useAntialiasedDrawingEb(&mut self.raw[0] as *mut u8, p0) }
            }
        }
        impl Drop for DisplayPainter {
             fn drop(&mut self) -> () {
                unsafe { stub__ZN4mono7display14DisplayPainterD0Ev(&mut self.raw[0] as *mut u8) }
            }
        }
    }
    pub mod geo {
        #[allow(dead_code)]
        pub struct Circle {
            raw: [u8; 12],
        }
        #[allow(dead_code)]
        pub struct Point {
            raw: [u8; 8],
        }
        #[allow(dead_code)]
        pub struct Rect {
            raw: [u8; 16],
        }
        #[allow(dead_code)]
        pub struct Size {
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
            pub fn Abs(&mut self) -> u32 {
                unsafe { stub__ZNK4mono3geo5Point3AbsEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Radius(&mut self) -> u32 {
                unsafe { stub__ZNK4mono3geo6Circle6RadiusEv(&mut self.raw[0] as *mut u8) }
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
            pub fn Abs(&mut self) -> u32 {
                unsafe { stub__ZNK4mono3geo5Point3AbsEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Area(&mut self) -> i32 {
                unsafe { stub__ZN4mono3geo4Rect4AreaEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Center(&mut self) -> ::generated::mono::geo::Point {
                unsafe { stub__ZNK4mono3geo4Rect6CenterEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Height(&mut self) -> i32 {
                unsafe { stub__ZNK4mono3geo4Size6HeightEv(&mut self.raw[0] as *mut u8) }
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
            pub fn Width(&mut self) -> i32 {
                unsafe { stub__ZNK4mono3geo4Size5WidthEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn X(&mut self) -> i32 {
                unsafe { stub__ZNK4mono3geo5Point1XEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn X2(&mut self) -> i32 {
                unsafe { stub__ZNK4mono3geo4Rect2X2Ev(&mut self.raw[0] as *mut u8) }
            }
            pub fn Y(&mut self) -> i32 {
                unsafe { stub__ZNK4mono3geo5Point1YEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Y2(&mut self) -> i32 {
                unsafe { stub__ZNK4mono3geo4Rect2Y2Ev(&mut self.raw[0] as *mut u8) }
            }
            pub fn appendX(&mut self, p0: i32) -> () {
                unsafe { stub__ZN4mono3geo5Point7appendXEi(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn appendY(&mut self, p0: i32) -> () {
                unsafe { stub__ZN4mono3geo5Point7appendYEi(&mut self.raw[0] as *mut u8, p0) }
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
            pub fn setHeight(&mut self, p0: i32) -> () {
                unsafe { stub__ZN4mono3geo4Size9setHeightEi(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setPoint(&mut self, p0: ::generated::mono::geo::Point) -> () {
                unsafe { stub__ZN4mono3geo4Rect8setPointENS0_5PointE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setSize(&mut self, p0: ::generated::mono::geo::Size) -> () {
                unsafe { stub__ZN4mono3geo4Rect7setSizeENS0_4SizeE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setWidth(&mut self, p0: i32) -> () {
                unsafe { stub__ZN4mono3geo4Size8setWidthEi(&mut self.raw[0] as *mut u8, p0) }
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
    pub mod io {
        #[allow(dead_code)]
        pub struct DigitalOut {
            raw: [u8; 8],
        }
        #[allow(dead_code)]
        extern "C" {
            // mbed::DigitalOut::is_connected()
            fn stub__ZN4mbed10DigitalOut12is_connectedEv(o: *mut u8) -> i32;
            // mbed::DigitalOut::read()
            fn stub__ZN4mbed10DigitalOut4readEv(o: *mut u8) -> i32;
            // mbed::DigitalOut::write(int)
            fn stub__ZN4mbed10DigitalOut5writeEi(o: *mut u8, p0: i32) -> ();
            // mbed::DigitalOut::operator=(mbed::DigitalOut &)
            fn stub__ZN4mbed10DigitalOutaSERS0_(o: *mut u8, p0: &::generated::mbed::DigitalOut) -> &::generated::mbed::DigitalOut;
            // mono::io::DigitalOut::setMode(PinMode)
            fn stub__ZN4mono2io10DigitalOut7setModeE7PinMode(o: *mut u8, p0: i32) -> ();
            // mono::io::DigitalOut::DigitalOut(PinName)
            fn stub__ZN4mono2io10DigitalOutC1E7PinName(o: *mut u8, p0: i32) -> ();
            // mono::io::DigitalOut::DigitalOut(PinName, int)
            fn stub__ZN4mono2io10DigitalOutC1E7PinNamei(o: *mut u8, p0: i32, p1: i32) -> ();
            // mono::io::DigitalOut::DigitalOut(PinName, int, PinMode)
            fn stub__ZN4mono2io10DigitalOutC1E7PinNamei7PinMode(o: *mut u8, p0: i32, p1: i32, p2: i32) -> ();
            // mono::io::DigitalOut::operator=(mono::io::DigitalOut &)
            fn stub__ZN4mono2io10DigitalOutaSERS1_(o: *mut u8, p0: &::generated::mono::io::DigitalOut) -> &::generated::mono::io::DigitalOut;
            // mono::io::DigitalOut::operator=(int)
            fn stub__ZN4mono2io10DigitalOutaSEi(o: *mut u8, p0: i32) -> &::generated::mono::io::DigitalOut;
        }
        impl DigitalOut {
            pub fn is_connected(&mut self) -> i32 {
                unsafe { stub__ZN4mbed10DigitalOut12is_connectedEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn new_(p0: i32) -> ::generated::mono::io::DigitalOut {
                let mut o = DigitalOut { raw: [0; 8] };
                unsafe { stub__ZN4mono2io10DigitalOutC1E7PinName(&mut o.raw[0] as *mut u8, p0); }
                o
            }
            pub fn new_i(p0: i32, p1: i32) -> ::generated::mono::io::DigitalOut {
                let mut o = DigitalOut { raw: [0; 8] };
                unsafe { stub__ZN4mono2io10DigitalOutC1E7PinNamei(&mut o.raw[0] as *mut u8, p0, p1); }
                o
            }
            pub fn new_i7PinMode(p0: i32, p1: i32, p2: i32) -> ::generated::mono::io::DigitalOut {
                let mut o = DigitalOut { raw: [0; 8] };
                unsafe { stub__ZN4mono2io10DigitalOutC1E7PinNamei7PinMode(&mut o.raw[0] as *mut u8, p0, p1, p2); }
                o
            }
            pub fn read(&mut self) -> i32 {
                unsafe { stub__ZN4mbed10DigitalOut4readEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn setMode(&mut self, p0: i32) -> () {
                unsafe { stub__ZN4mono2io10DigitalOut7setModeE7PinMode(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn write(&mut self, p0: i32) -> () {
                unsafe { stub__ZN4mbed10DigitalOut5writeEi(&mut self.raw[0] as *mut u8, p0) }
            }
        }
    }
    pub mod media {
        #[allow(dead_code)]
        pub struct BMPImage {
            raw: [u8; 104],
        }
        #[allow(dead_code)]
        pub struct Image {
            raw: [u8; 8],
        }
        #[allow(dead_code)]
        extern "C" {
            // mono::media::Image::SeekToHLine(int)
            fn stub__ZN4mono5media5Image11SeekToHLineEi(o: *mut u8, p0: i32) -> ();
            // mono::media::Image::PixelByteSize()
            fn stub__ZN4mono5media5Image13PixelByteSizeEv(o: *mut u8) -> i32;
            // mono::media::Image::ReadPixelData(void *, int)
            fn stub__ZN4mono5media5Image13ReadPixelDataEPvi(o: *mut u8, p0: *mut (), p1: i32) -> i32;
            // mono::media::Image::SkipPixelData(int)
            fn stub__ZN4mono5media5Image13SkipPixelDataEi(o: *mut u8, p0: i32) -> i32;
            // mono::media::Image::Width()
            fn stub__ZN4mono5media5Image5WidthEv(o: *mut u8) -> i32;
            // mono::media::Image::Height()
            fn stub__ZN4mono5media5Image6HeightEv(o: *mut u8) -> i32;
            // mono::media::Image::IsValid()
            fn stub__ZN4mono5media5Image7IsValidEv(o: *mut u8) -> bool;
            // mono::media::Image::~Image()
            fn stub__ZN4mono5media5ImageD0Ev(o: *mut u8) -> ();
            // mono::media::BMPImage::SeekToHLine(int)
            fn stub__ZN4mono5media8BMPImage11SeekToHLineEi(o: *mut u8, p0: i32) -> ();
            // mono::media::BMPImage::PixelByteSize()
            fn stub__ZN4mono5media8BMPImage13PixelByteSizeEv(o: *mut u8) -> i32;
            // mono::media::BMPImage::ReadPixelData(void *, int)
            fn stub__ZN4mono5media8BMPImage13ReadPixelDataEPvi(o: *mut u8, p0: *mut (), p1: i32) -> i32;
            // mono::media::BMPImage::SkipPixelData(int)
            fn stub__ZN4mono5media8BMPImage13SkipPixelDataEi(o: *mut u8, p0: i32) -> i32;
            // mono::media::BMPImage::Width()
            fn stub__ZN4mono5media8BMPImage5WidthEv(o: *mut u8) -> i32;
            // mono::media::BMPImage::Height()
            fn stub__ZN4mono5media8BMPImage6HeightEv(o: *mut u8) -> i32;
            // mono::media::BMPImage::IsValid()
            fn stub__ZN4mono5media8BMPImage7IsValidEv(o: *mut u8) -> bool;
            // mono::media::BMPImage::BMPImage(mono::String)
            fn stub__ZN4mono5media8BMPImageC1ENS_6StringE(o: *mut u8, p0: ::generated::mono::String) -> ();
            // mono::media::BMPImage::BMPImage(const mono::media::BMPImage &)
            fn stub__ZN4mono5media8BMPImageC1ERKS1_(o: *mut u8, p0: &::generated::mono::media::BMPImage) -> ();
            // mono::media::BMPImage::BMPImage()
            fn stub__ZN4mono5media8BMPImageC1Ev(o: *mut u8) -> ();
            // mono::media::BMPImage::~BMPImage()
            fn stub__ZN4mono5media8BMPImageD0Ev(o: *mut u8) -> ();
            // mono::media::BMPImage::operator=(const mono::media::BMPImage &)
            fn stub__ZN4mono5media8BMPImageaSERKS1_(o: *mut u8, p0: &::generated::mono::media::BMPImage) -> &::generated::mono::media::BMPImage;
        }
        impl BMPImage {
            pub fn Height(&mut self) -> i32 {
                unsafe { stub__ZN4mono5media8BMPImage6HeightEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn IsValid(&mut self) -> bool {
                unsafe { stub__ZN4mono5media8BMPImage7IsValidEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn PixelByteSize(&mut self) -> i32 {
                unsafe { stub__ZN4mono5media8BMPImage13PixelByteSizeEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn ReadPixelData(&mut self, p0: *mut (), p1: i32) -> i32 {
                unsafe { stub__ZN4mono5media8BMPImage13ReadPixelDataEPvi(&mut self.raw[0] as *mut u8, p0, p1) }
            }
            pub fn SeekToHLine(&mut self, p0: i32) -> () {
                unsafe { stub__ZN4mono5media8BMPImage11SeekToHLineEi(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn SkipPixelData(&mut self, p0: i32) -> i32 {
                unsafe { stub__ZN4mono5media8BMPImage13SkipPixelDataEi(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn Width(&mut self) -> i32 {
                unsafe { stub__ZN4mono5media8BMPImage5WidthEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn new() -> ::generated::mono::media::BMPImage {
                let mut o = BMPImage { raw: [0; 104] };
                unsafe { stub__ZN4mono5media8BMPImageC1Ev(&mut o.raw[0] as *mut u8); }
                o
            }
            pub fn new_NS_6StringE(p0: ::generated::mono::String) -> ::generated::mono::media::BMPImage {
                let mut o = BMPImage { raw: [0; 104] };
                unsafe { stub__ZN4mono5media8BMPImageC1ENS_6StringE(&mut o.raw[0] as *mut u8, p0); }
                o
            }
            pub fn new_RKS1_(p0: &::generated::mono::media::BMPImage) -> ::generated::mono::media::BMPImage {
                let mut o = BMPImage { raw: [0; 104] };
                unsafe { stub__ZN4mono5media8BMPImageC1ERKS1_(&mut o.raw[0] as *mut u8, p0); }
                o
            }
        }
        impl Drop for BMPImage {
             fn drop(&mut self) -> () {
                unsafe { stub__ZN4mono5media8BMPImageD0Ev(&mut self.raw[0] as *mut u8) }
            }
        }
        impl Drop for Image {
             fn drop(&mut self) -> () {
                unsafe { stub__ZN4mono5media5ImageD0Ev(&mut self.raw[0] as *mut u8) }
            }
        }
        impl Image {
            pub fn Height(&mut self) -> i32 {
                unsafe { stub__ZN4mono5media5Image6HeightEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn IsValid(&mut self) -> bool {
                unsafe { stub__ZN4mono5media5Image7IsValidEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn PixelByteSize(&mut self) -> i32 {
                unsafe { stub__ZN4mono5media5Image13PixelByteSizeEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn ReadPixelData(&mut self, p0: *mut (), p1: i32) -> i32 {
                unsafe { stub__ZN4mono5media5Image13ReadPixelDataEPvi(&mut self.raw[0] as *mut u8, p0, p1) }
            }
            pub fn SeekToHLine(&mut self, p0: i32) -> () {
                unsafe { stub__ZN4mono5media5Image11SeekToHLineEi(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn SkipPixelData(&mut self, p0: i32) -> i32 {
                unsafe { stub__ZN4mono5media5Image13SkipPixelDataEi(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn Width(&mut self) -> i32 {
                unsafe { stub__ZN4mono5media5Image5WidthEv(&mut self.raw[0] as *mut u8) }
            }
        }
    }
    pub mod ui {
        #[allow(dead_code)]
        pub struct AbstractButtonView {
            raw: [u8; 120],
        }
        #[allow(dead_code)]
        pub struct Animator {
            raw: [u8; 72],
        }
        #[allow(dead_code)]
        pub struct BackgroundView {
            raw: [u8; 40],
        }
        #[allow(dead_code)]
        pub struct ButtonView {
            raw: [u8; 232],
        }
        #[allow(dead_code)]
        pub struct GraphView {
            raw: [u8; 64],
        }
        #[allow(dead_code)]
        pub struct IGraphViewDataSource {
            raw: [u8; 8],
        }
        #[allow(dead_code)]
        pub struct ImageView {
            raw: [u8; 64],
        }
        #[allow(dead_code)]
        pub struct OnOffButtonView {
            raw: [u8; 360],
        }
        #[allow(dead_code)]
        pub struct ProgressBarView {
            raw: [u8; 112],
        }
        #[allow(dead_code)]
        pub struct ResponderView {
            raw: [u8; 64],
        }
        #[allow(dead_code)]
        pub struct StatusIndicatorView {
            raw: [u8; 48],
        }
        #[allow(dead_code)]
        pub struct TextLabelView {
            raw: [u8; 112],
        }
        #[allow(dead_code)]
        pub struct TouchCalibrateView {
            raw: [u8; 408],
        }
        #[allow(dead_code)]
        pub struct View {
            raw: [u8; 40],
        }
        #[allow(dead_code)]
        extern "C" {
            // mono::TouchResponder::Deactivate()
            fn stub__ZN4mono14TouchResponder10DeactivateEv(o: *mut u8) -> ();
            // mono::TouchResponder::RespondTouchEnd(mono::TouchEvent &)
            fn stub__ZN4mono14TouchResponder15RespondTouchEndERNS_10TouchEventE(o: *mut u8, p0: &::generated::mono::TouchEvent) -> ();
            // mono::TouchResponder::RespondTouchMove(mono::TouchEvent &)
            fn stub__ZN4mono14TouchResponder16RespondTouchMoveERNS_10TouchEventE(o: *mut u8, p0: &::generated::mono::TouchEvent) -> ();
            // mono::TouchResponder::RespondTouchBegin(mono::TouchEvent &)
            fn stub__ZN4mono14TouchResponder17RespondTouchBeginERNS_10TouchEventE(o: *mut u8, p0: &::generated::mono::TouchEvent) -> ();
            // mono::TouchResponder::RunResponderChainTouchEnd(mono::TouchEvent &)
            fn stub__ZN4mono14TouchResponder25RunResponderChainTouchEndERNS_10TouchEventE(o: *mut u8, p0: &::generated::mono::TouchEvent) -> ();
            // mono::TouchResponder::RunResponderChainTouchMove(mono::TouchEvent &)
            fn stub__ZN4mono14TouchResponder26RunResponderChainTouchMoveERNS_10TouchEventE(o: *mut u8, p0: &::generated::mono::TouchEvent) -> ();
            // mono::TouchResponder::RunResponderChainTouchBegin(mono::TouchEvent &)
            fn stub__ZN4mono14TouchResponder27RunResponderChainTouchBeginERNS_10TouchEventE(o: *mut u8, p0: &::generated::mono::TouchEvent) -> ();
            // mono::TouchResponder::Activate()
            fn stub__ZN4mono14TouchResponder8ActivateEv(o: *mut u8) -> ();
            // mono::ui::ButtonView::setHighlight(mono::display::Color)
            fn stub__ZN4mono2ui10ButtonView12setHighlightENS_7display5ColorE(o: *mut u8, p0: ::generated::mono::display::Color) -> ();
            // mono::ui::ButtonView::setBackground(mono::display::Color)
            fn stub__ZN4mono2ui10ButtonView13setBackgroundENS_7display5ColorE(o: *mut u8, p0: ::generated::mono::display::Color) -> ();
            // mono::ui::ButtonView::setClickCallback(void (*)())
            fn stub__ZN4mono2ui10ButtonView16setClickCallbackEPFvvE(o: *mut u8, p0: *mut fn () -> ()) -> ();
            // mono::ui::ButtonView::repaint()
            fn stub__ZN4mono2ui10ButtonView7repaintEv(o: *mut u8) -> ();
            // mono::ui::ButtonView::setFont(const MonoFont &)
            fn stub__ZN4mono2ui10ButtonView7setFontERK8MonoFont(o: *mut u8, p0: &::generated::MonoFont) -> ();
            // mono::ui::ButtonView::setText(mono::String)
            fn stub__ZN4mono2ui10ButtonView7setTextENS_6StringE(o: *mut u8, p0: ::generated::mono::String) -> ();
            // mono::ui::ButtonView::setBorder(mono::display::Color)
            fn stub__ZN4mono2ui10ButtonView9setBorderENS_7display5ColorE(o: *mut u8, p0: ::generated::mono::display::Color) -> ();
            // mono::ui::ButtonView::ButtonView(geo::Rect, mono::String)
            fn stub__ZN4mono2ui10ButtonViewC1ENS_3geo4RectENS_6StringE(o: *mut u8, p0: ::generated::mono::geo::Rect, p1: ::generated::mono::String) -> ();
            // mono::ui::ButtonView::ButtonView(geo::Rect, const char *)
            fn stub__ZN4mono2ui10ButtonViewC1ENS_3geo4RectEPKc(o: *mut u8, p0: ::generated::mono::geo::Rect, p1: *mut i8) -> ();
            // mono::ui::ButtonView::ButtonView()
            fn stub__ZN4mono2ui10ButtonViewC1Ev(o: *mut u8) -> ();
            // mono::ui::ResponderView::hide()
            fn stub__ZN4mono2ui13ResponderView4hideEv(o: *mut u8) -> ();
            // mono::ui::ResponderView::show()
            fn stub__ZN4mono2ui13ResponderView4showEv(o: *mut u8) -> ();
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
            // mono::ui::TextLabelView::TextLabelView(geo::Rect, mono::String)
            fn stub__ZN4mono2ui13TextLabelViewC1ENS_3geo4RectENS_6StringE(o: *mut u8, p0: ::generated::mono::geo::Rect, p1: ::generated::mono::String) -> ();
            // mono::ui::TextLabelView::TextLabelView(geo::Rect, const char *)
            fn stub__ZN4mono2ui13TextLabelViewC1ENS_3geo4RectEPKc(o: *mut u8, p0: ::generated::mono::geo::Rect, p1: *mut i8) -> ();
            // mono::ui::TextLabelView::TextLabelView(mono::String)
            fn stub__ZN4mono2ui13TextLabelViewC1ENS_6StringE(o: *mut u8, p0: ::generated::mono::String) -> ();
            // mono::ui::TextLabelView::TextLabelView(const char *)
            fn stub__ZN4mono2ui13TextLabelViewC1EPKc(o: *mut u8, p0: *mut i8) -> ();
            // mono::ui::BackgroundView::setBackgroundColor(display::Color)
            fn stub__ZN4mono2ui14BackgroundView18setBackgroundColorENS_7display5ColorE(o: *mut u8, p0: ::generated::mono::display::Color) -> ();
            // mono::ui::BackgroundView::repaint()
            fn stub__ZN4mono2ui14BackgroundView7repaintEv(o: *mut u8) -> ();
            // mono::ui::BackgroundView::BackgroundView(display::Color)
            fn stub__ZN4mono2ui14BackgroundViewC1ENS_7display5ColorE(o: *mut u8, p0: ::generated::mono::display::Color) -> ();
            // mono::ui::OnOffButtonView::setHighlight(display::Color)
            fn stub__ZN4mono2ui15OnOffButtonView12setHighlightENS_7display5ColorE(o: *mut u8, p0: ::generated::mono::display::Color) -> ();
            // mono::ui::OnOffButtonView::setBackground(display::Color)
            fn stub__ZN4mono2ui15OnOffButtonView13setBackgroundENS_7display5ColorE(o: *mut u8, p0: ::generated::mono::display::Color) -> ();
            // mono::ui::OnOffButtonView::repaint()
            fn stub__ZN4mono2ui15OnOffButtonView7repaintEv(o: *mut u8) -> ();
            // mono::ui::OnOffButtonView::setBorder(display::Color)
            fn stub__ZN4mono2ui15OnOffButtonView9setBorderENS_7display5ColorE(o: *mut u8, p0: ::generated::mono::display::Color) -> ();
            // mono::ui::OnOffButtonView::OnOffButtonView(geo::Rect)
            fn stub__ZN4mono2ui15OnOffButtonViewC1ENS_3geo4RectE(o: *mut u8, p0: ::generated::mono::geo::Rect) -> ();
            // mono::ui::OnOffButtonView::OnOffButtonView()
            fn stub__ZN4mono2ui15OnOffButtonViewC1Ev(o: *mut u8) -> ();
            // mono::ui::ProgressBarView::setMaximum(int)
            fn stub__ZN4mono2ui15ProgressBarView10setMaximumEi(o: *mut u8, p0: i32) -> ();
            // mono::ui::ProgressBarView::setMinimum(int)
            fn stub__ZN4mono2ui15ProgressBarView10setMinimumEi(o: *mut u8, p0: i32) -> ();
            // mono::ui::ProgressBarView::setValue(int)
            fn stub__ZN4mono2ui15ProgressBarView8setValueEi(o: *mut u8, p0: i32) -> ();
            // mono::ui::ProgressBarView::ProgressBarView(geo::Rect)
            fn stub__ZN4mono2ui15ProgressBarViewC1ENS_3geo4RectE(o: *mut u8, p0: ::generated::mono::geo::Rect) -> ();
            // mono::ui::ProgressBarView::ProgressBarView()
            fn stub__ZN4mono2ui15ProgressBarViewC1Ev(o: *mut u8) -> ();
            // mono::ui::AbstractButtonView::setClickCallback(void (*)())
            fn stub__ZN4mono2ui18AbstractButtonView16setClickCallbackEPFvvE(o: *mut u8, p0: *mut fn () -> ()) -> ();
            // mono::ui::TouchCalibrateView::RespondTouchBegin(mono::TouchEvent &)
            fn stub__ZN4mono2ui18TouchCalibrateView17RespondTouchBeginERNS_10TouchEventE(o: *mut u8, p0: &::generated::mono::TouchEvent) -> ();
            // mono::ui::TouchCalibrateView::StartNewCalibration()
            fn stub__ZN4mono2ui18TouchCalibrateView19StartNewCalibrationEv(o: *mut u8) -> ();
            // mono::ui::TouchCalibrateView::show()
            fn stub__ZN4mono2ui18TouchCalibrateView4showEv(o: *mut u8) -> ();
            // mono::ui::TouchCalibrateView::TouchCalibrateView(geo::Rect)
            fn stub__ZN4mono2ui18TouchCalibrateViewC1ENS_3geo4RectE(o: *mut u8, p0: ::generated::mono::geo::Rect) -> ();
            // mono::ui::TouchCalibrateView::TouchCalibrateView()
            fn stub__ZN4mono2ui18TouchCalibrateViewC1Ev(o: *mut u8) -> ();
            // mono::ui::StatusIndicatorView::setOnStateColor(display::Color)
            fn stub__ZN4mono2ui19StatusIndicatorView15setOnStateColorENS_7display5ColorE(o: *mut u8, p0: ::generated::mono::display::Color) -> ();
            // mono::ui::StatusIndicatorView::setOffStateColor(display::Color)
            fn stub__ZN4mono2ui19StatusIndicatorView16setOffStateColorENS_7display5ColorE(o: *mut u8, p0: ::generated::mono::display::Color) -> ();
            // mono::ui::StatusIndicatorView::setState(bool)
            fn stub__ZN4mono2ui19StatusIndicatorView8setStateEb(o: *mut u8, p0: bool) -> ();
            // mono::ui::StatusIndicatorView::StatusIndicatorView(geo::Rect)
            fn stub__ZN4mono2ui19StatusIndicatorViewC1ENS_3geo4RectE(o: *mut u8, p0: ::generated::mono::geo::Rect) -> ();
            // mono::ui::StatusIndicatorView::StatusIndicatorView(geo::Rect, bool)
            fn stub__ZN4mono2ui19StatusIndicatorViewC1ENS_3geo4RectEb(o: *mut u8, p0: ::generated::mono::geo::Rect, p1: bool) -> ();
            // mono::ui::StatusIndicatorView::StatusIndicatorView()
            fn stub__ZN4mono2ui19StatusIndicatorViewC1Ev(o: *mut u8) -> ();
            // mono::ui::IGraphViewDataSource::BufferLenght()
            fn stub__ZN4mono2ui20IGraphViewDataSource12BufferLenghtEv(o: *mut u8) -> i32;
            // mono::ui::IGraphViewDataSource::NewestSampleIndex()
            fn stub__ZN4mono2ui20IGraphViewDataSource17NewestSampleIndexEv(o: *mut u8) -> i32;
            // mono::ui::IGraphViewDataSource::MaxSampleValueSpan()
            fn stub__ZN4mono2ui20IGraphViewDataSource18MaxSampleValueSpanEv(o: *mut u8) -> i32;
            // mono::ui::IGraphViewDataSource::DataPoint(int)
            fn stub__ZN4mono2ui20IGraphViewDataSource9DataPointEi(o: *mut u8, p0: i32) -> i32;
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
            // mono::ui::View::View(geo::Rect)
            fn stub__ZN4mono2ui4ViewC1ENS_3geo4RectE(o: *mut u8, p0: ::generated::mono::geo::Rect) -> ();
            // mono::ui::View::View()
            fn stub__ZN4mono2ui4ViewC1Ev(o: *mut u8) -> ();
            // mono::ui::View::~View()
            fn stub__ZN4mono2ui4ViewD0Ev(o: *mut u8) -> ();
            // mono::ui::Animator::setMoveVector(geo::Point)
            fn stub__ZN4mono2ui8Animator13setMoveVectorENS_3geo5PointE(o: *mut u8, p0: ::generated::mono::geo::Point) -> ();
            // mono::ui::Animator::setDestination(geo::Point)
            fn stub__ZN4mono2ui8Animator14setDestinationENS_3geo5PointE(o: *mut u8, p0: ::generated::mono::geo::Point) -> ();
            // mono::ui::Animator::Pause()
            fn stub__ZN4mono2ui8Animator5PauseEv(o: *mut u8) -> ();
            // mono::ui::Animator::Reset()
            fn stub__ZN4mono2ui8Animator5ResetEv(o: *mut u8) -> ();
            // mono::ui::Animator::Start()
            fn stub__ZN4mono2ui8Animator5StartEv(o: *mut u8) -> ();
            // mono::ui::Animator::Animator(mono::ui::View *)
            fn stub__ZN4mono2ui8AnimatorC1EPNS0_4ViewE(o: *mut u8, p0: *mut ::generated::mono::ui::View) -> ();
            // mono::ui::GraphView::setDataSource(const mono::ui::IGraphViewDataSource &)
            fn stub__ZN4mono2ui9GraphView13setDataSourceERKNS0_20IGraphViewDataSourceE(o: *mut u8, p0: &::generated::mono::ui::IGraphViewDataSource) -> ();
            // mono::ui::GraphView::setCursorActive(bool)
            fn stub__ZN4mono2ui9GraphView15setCursorActiveEb(o: *mut u8, p0: bool) -> ();
            // mono::ui::GraphView::GraphView(geo::Rect)
            fn stub__ZN4mono2ui9GraphViewC1ENS_3geo4RectE(o: *mut u8, p0: ::generated::mono::geo::Rect) -> ();
            // mono::ui::GraphView::GraphView(geo::Rect, const mono::ui::IGraphViewDataSource &)
            fn stub__ZN4mono2ui9GraphViewC1ENS_3geo4RectERKNS0_20IGraphViewDataSourceE(o: *mut u8, p0: ::generated::mono::geo::Rect, p1: &::generated::mono::ui::IGraphViewDataSource) -> ();
            // mono::ui::GraphView::GraphView()
            fn stub__ZN4mono2ui9GraphViewC1Ev(o: *mut u8) -> ();
            // mono::ui::ImageView::repaint()
            fn stub__ZN4mono2ui9ImageView7repaintEv(o: *mut u8) -> ();
            // mono::ui::ImageView::setCrop(geo::Rect)
            fn stub__ZN4mono2ui9ImageView7setCropENS_3geo4RectE(o: *mut u8, p0: ::generated::mono::geo::Rect) -> ();
            // mono::ui::ImageView::setCrop(geo::Size)
            fn stub__ZN4mono2ui9ImageView7setCropENS_3geo4SizeE(o: *mut u8, p0: ::generated::mono::geo::Size) -> ();
            // mono::ui::ImageView::setCrop(geo::Point)
            fn stub__ZN4mono2ui9ImageView7setCropENS_3geo5PointE(o: *mut u8, p0: ::generated::mono::geo::Point) -> ();
            // mono::ui::ImageView::setImage(media::Image *)
            fn stub__ZN4mono2ui9ImageView8setImageEPNS_5media5ImageE(o: *mut u8, p0: *mut ::generated::mono::media::Image) -> ();
            // mono::ui::ImageView::ImageView(media::Image *)
            fn stub__ZN4mono2ui9ImageViewC1EPNS_5media5ImageE(o: *mut u8, p0: *mut ::generated::mono::media::Image) -> ();
            // mono::ui::ImageView::ImageView()
            fn stub__ZN4mono2ui9ImageViewC1Ev(o: *mut u8) -> ();
            // mono::ui::ButtonView::TextLabel()
            fn stub__ZNK4mono2ui10ButtonView9TextLabelEv(o: *mut u8) -> &::generated::mono::ui::TextLabelView;
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
            // mono::ui::BackgroundView::Color()
            fn stub__ZNK4mono2ui14BackgroundView5ColorEv(o: *mut u8) -> ::generated::mono::display::Color;
            // mono::ui::OnOffButtonView::BorderColor()
            fn stub__ZNK4mono2ui15OnOffButtonView11BorderColorEv(o: *mut u8) -> ::generated::mono::display::Color;
            // mono::ui::OnOffButtonView::HighlightColor()
            fn stub__ZNK4mono2ui15OnOffButtonView14HighlightColorEv(o: *mut u8) -> ::generated::mono::display::Color;
            // mono::ui::OnOffButtonView::BackgroundColor()
            fn stub__ZNK4mono2ui15OnOffButtonView15BackgroundColorEv(o: *mut u8) -> ::generated::mono::display::Color;
            // mono::ui::OnOffButtonView::isActive()
            fn stub__ZNK4mono2ui15OnOffButtonView8isActiveEv(o: *mut u8) -> bool;
            // mono::ui::ProgressBarView::CurrentValue()
            fn stub__ZNK4mono2ui15ProgressBarView12CurrentValueEv(o: *mut u8) -> i32;
            // mono::ui::ProgressBarView::Maximum()
            fn stub__ZNK4mono2ui15ProgressBarView7MaximumEv(o: *mut u8) -> i32;
            // mono::ui::ProgressBarView::Minimum()
            fn stub__ZNK4mono2ui15ProgressBarView7MinimumEv(o: *mut u8) -> i32;
            // mono::ui::StatusIndicatorView::State()
            fn stub__ZNK4mono2ui19StatusIndicatorView5StateEv(o: *mut u8) -> bool;
            // mono::ui::View::Visible()
            fn stub__ZNK4mono2ui4View7VisibleEv(o: *mut u8) -> bool;
            // mono::ui::View::ViewRect()
            fn stub__ZNK4mono2ui4View8ViewRectEv(o: *mut u8) -> &::generated::mono::geo::Rect;
            // mono::ui::GraphView::DataSource()
            fn stub__ZNK4mono2ui9GraphView10DataSourceEv(o: *mut u8) -> *mut ::generated::mono::ui::IGraphViewDataSource;
            // mono::ui::GraphView::CursorActive()
            fn stub__ZNK4mono2ui9GraphView12CursorActiveEv(o: *mut u8) -> bool;
            // mono::ui::ImageView::Crop()
            fn stub__ZNK4mono2ui9ImageView4CropEv(o: *mut u8) -> &::generated::mono::geo::Rect;
        }
        impl AbstractButtonView {
            pub fn Activate(&mut self) -> () {
                unsafe { stub__ZN4mono14TouchResponder8ActivateEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Deactivate(&mut self) -> () {
                unsafe { stub__ZN4mono14TouchResponder10DeactivateEv(&mut self.raw[0] as *mut u8) }
            }
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
            pub fn RespondTouchBegin(&mut self, p0: &::generated::mono::TouchEvent) -> () {
                unsafe { stub__ZN4mono14TouchResponder17RespondTouchBeginERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn RespondTouchEnd(&mut self, p0: &::generated::mono::TouchEvent) -> () {
                unsafe { stub__ZN4mono14TouchResponder15RespondTouchEndERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn RespondTouchMove(&mut self, p0: &::generated::mono::TouchEvent) -> () {
                unsafe { stub__ZN4mono14TouchResponder16RespondTouchMoveERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn RunResponderChainTouchBegin(&mut self, p0: &::generated::mono::TouchEvent) -> () {
                unsafe { stub__ZN4mono14TouchResponder27RunResponderChainTouchBeginERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn RunResponderChainTouchEnd(&mut self, p0: &::generated::mono::TouchEvent) -> () {
                unsafe { stub__ZN4mono14TouchResponder25RunResponderChainTouchEndERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn RunResponderChainTouchMove(&mut self, p0: &::generated::mono::TouchEvent) -> () {
                unsafe { stub__ZN4mono14TouchResponder26RunResponderChainTouchMoveERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
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
                unsafe { stub__ZN4mono2ui13ResponderView4hideEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn scheduleRepaint(&mut self) -> () {
                unsafe { stub__ZN4mono2ui4View15scheduleRepaintEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn setClickCallback(&mut self, p0: *mut fn () -> ()) -> () {
                unsafe { stub__ZN4mono2ui18AbstractButtonView16setClickCallbackEPFvvE(&mut self.raw[0] as *mut u8, p0) }
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
                unsafe { stub__ZN4mono2ui13ResponderView4showEv(&mut self.raw[0] as *mut u8) }
            }
        }
        impl Animator {
            pub fn Pause(&mut self) -> () {
                unsafe { stub__ZN4mono2ui8Animator5PauseEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Reset(&mut self) -> () {
                unsafe { stub__ZN4mono2ui8Animator5ResetEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Start(&mut self) -> () {
                unsafe { stub__ZN4mono2ui8Animator5StartEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn new(p0: *mut ::generated::mono::ui::View) -> ::generated::mono::ui::Animator {
                let mut o = Animator { raw: [0; 72] };
                unsafe { stub__ZN4mono2ui8AnimatorC1EPNS0_4ViewE(&mut o.raw[0] as *mut u8, p0); }
                o
            }
            pub fn setDestination(&mut self, p0: ::generated::mono::geo::Point) -> () {
                unsafe { stub__ZN4mono2ui8Animator14setDestinationENS_3geo5PointE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setMoveVector(&mut self, p0: ::generated::mono::geo::Point) -> () {
                unsafe { stub__ZN4mono2ui8Animator13setMoveVectorENS_3geo5PointE(&mut self.raw[0] as *mut u8, p0) }
            }
        }
        impl BackgroundView {
            pub fn Color(&mut self) -> ::generated::mono::display::Color {
                unsafe { stub__ZNK4mono2ui14BackgroundView5ColorEv(&mut self.raw[0] as *mut u8) }
            }
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
            pub fn new(p0: ::generated::mono::display::Color) -> ::generated::mono::ui::BackgroundView {
                let mut o = BackgroundView { raw: [0; 40] };
                unsafe { stub__ZN4mono2ui14BackgroundViewC1ENS_7display5ColorE(&mut o.raw[0] as *mut u8, p0); }
                o
            }
            pub fn repaint(&mut self) -> () {
                unsafe { stub__ZN4mono2ui14BackgroundView7repaintEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn scheduleRepaint(&mut self) -> () {
                unsafe { stub__ZN4mono2ui4View15scheduleRepaintEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn setBackgroundColor(&mut self, p0: ::generated::mono::display::Color) -> () {
                unsafe { stub__ZN4mono2ui14BackgroundView18setBackgroundColorENS_7display5ColorE(&mut self.raw[0] as *mut u8, p0) }
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
        impl ButtonView {
            pub fn Activate(&mut self) -> () {
                unsafe { stub__ZN4mono14TouchResponder8ActivateEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Deactivate(&mut self) -> () {
                unsafe { stub__ZN4mono14TouchResponder10DeactivateEv(&mut self.raw[0] as *mut u8) }
            }
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
            pub fn RespondTouchBegin(&mut self, p0: &::generated::mono::TouchEvent) -> () {
                unsafe { stub__ZN4mono14TouchResponder17RespondTouchBeginERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn RespondTouchEnd(&mut self, p0: &::generated::mono::TouchEvent) -> () {
                unsafe { stub__ZN4mono14TouchResponder15RespondTouchEndERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn RespondTouchMove(&mut self, p0: &::generated::mono::TouchEvent) -> () {
                unsafe { stub__ZN4mono14TouchResponder16RespondTouchMoveERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn RunResponderChainTouchBegin(&mut self, p0: &::generated::mono::TouchEvent) -> () {
                unsafe { stub__ZN4mono14TouchResponder27RunResponderChainTouchBeginERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn RunResponderChainTouchEnd(&mut self, p0: &::generated::mono::TouchEvent) -> () {
                unsafe { stub__ZN4mono14TouchResponder25RunResponderChainTouchEndERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn RunResponderChainTouchMove(&mut self, p0: &::generated::mono::TouchEvent) -> () {
                unsafe { stub__ZN4mono14TouchResponder26RunResponderChainTouchMoveERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn Size(&mut self) -> &::generated::mono::geo::Size {
                unsafe { stub__ZN4mono2ui4View4SizeEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn TextLabel(&mut self) -> &::generated::mono::ui::TextLabelView {
                unsafe { stub__ZNK4mono2ui10ButtonView9TextLabelEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn ViewRect(&mut self) -> &::generated::mono::geo::Rect {
                unsafe { stub__ZNK4mono2ui4View8ViewRectEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Visible(&mut self) -> bool {
                unsafe { stub__ZNK4mono2ui4View7VisibleEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn hide(&mut self) -> () {
                unsafe { stub__ZN4mono2ui13ResponderView4hideEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn new() -> ::generated::mono::ui::ButtonView {
                let mut o = ButtonView { raw: [0; 232] };
                unsafe { stub__ZN4mono2ui10ButtonViewC1Ev(&mut o.raw[0] as *mut u8); }
                o
            }
            pub fn new_NS_3geo4RectENS_6StringE(p0: ::generated::mono::geo::Rect, p1: ::generated::mono::String) -> ::generated::mono::ui::ButtonView {
                let mut o = ButtonView { raw: [0; 232] };
                unsafe { stub__ZN4mono2ui10ButtonViewC1ENS_3geo4RectENS_6StringE(&mut o.raw[0] as *mut u8, p0, p1); }
                o
            }
            pub fn new_NS_3geo4RectEPKc(p0: ::generated::mono::geo::Rect, p1: *mut i8) -> ::generated::mono::ui::ButtonView {
                let mut o = ButtonView { raw: [0; 232] };
                unsafe { stub__ZN4mono2ui10ButtonViewC1ENS_3geo4RectEPKc(&mut o.raw[0] as *mut u8, p0, p1); }
                o
            }
            pub fn repaint(&mut self) -> () {
                unsafe { stub__ZN4mono2ui10ButtonView7repaintEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn scheduleRepaint(&mut self) -> () {
                unsafe { stub__ZN4mono2ui4View15scheduleRepaintEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn setBackground(&mut self, p0: ::generated::mono::display::Color) -> () {
                unsafe { stub__ZN4mono2ui10ButtonView13setBackgroundENS_7display5ColorE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setBorder(&mut self, p0: ::generated::mono::display::Color) -> () {
                unsafe { stub__ZN4mono2ui10ButtonView9setBorderENS_7display5ColorE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setClickCallback(&mut self, p0: *mut fn () -> ()) -> () {
                unsafe { stub__ZN4mono2ui10ButtonView16setClickCallbackEPFvvE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setFont(&mut self, p0: &::generated::MonoFont) -> () {
                unsafe { stub__ZN4mono2ui10ButtonView7setFontERK8MonoFont(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setHighlight(&mut self, p0: ::generated::mono::display::Color) -> () {
                unsafe { stub__ZN4mono2ui10ButtonView12setHighlightENS_7display5ColorE(&mut self.raw[0] as *mut u8, p0) }
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
            pub fn setText(&mut self, p0: ::generated::mono::String) -> () {
                unsafe { stub__ZN4mono2ui10ButtonView7setTextENS_6StringE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn show(&mut self) -> () {
                unsafe { stub__ZN4mono2ui13ResponderView4showEv(&mut self.raw[0] as *mut u8) }
            }
        }
        impl Drop for View {
             fn drop(&mut self) -> () {
                unsafe { stub__ZN4mono2ui4ViewD0Ev(&mut self.raw[0] as *mut u8) }
            }
        }
        impl GraphView {
            pub fn CursorActive(&mut self) -> bool {
                unsafe { stub__ZNK4mono2ui9GraphView12CursorActiveEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn DataSource(&mut self) -> *mut ::generated::mono::ui::IGraphViewDataSource {
                unsafe { stub__ZNK4mono2ui9GraphView10DataSourceEv(&mut self.raw[0] as *mut u8) }
            }
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
            pub fn new() -> ::generated::mono::ui::GraphView {
                let mut o = GraphView { raw: [0; 64] };
                unsafe { stub__ZN4mono2ui9GraphViewC1Ev(&mut o.raw[0] as *mut u8); }
                o
            }
            pub fn new_NS_3geo4RectE(p0: ::generated::mono::geo::Rect) -> ::generated::mono::ui::GraphView {
                let mut o = GraphView { raw: [0; 64] };
                unsafe { stub__ZN4mono2ui9GraphViewC1ENS_3geo4RectE(&mut o.raw[0] as *mut u8, p0); }
                o
            }
            pub fn new_NS_3geo4RectERKNS0_20IGraphViewDataSourceE(p0: ::generated::mono::geo::Rect, p1: &::generated::mono::ui::IGraphViewDataSource) -> ::generated::mono::ui::GraphView {
                let mut o = GraphView { raw: [0; 64] };
                unsafe { stub__ZN4mono2ui9GraphViewC1ENS_3geo4RectERKNS0_20IGraphViewDataSourceE(&mut o.raw[0] as *mut u8, p0, p1); }
                o
            }
            pub fn scheduleRepaint(&mut self) -> () {
                unsafe { stub__ZN4mono2ui4View15scheduleRepaintEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn setCursorActive(&mut self, p0: bool) -> () {
                unsafe { stub__ZN4mono2ui9GraphView15setCursorActiveEb(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setDataSource(&mut self, p0: &::generated::mono::ui::IGraphViewDataSource) -> () {
                unsafe { stub__ZN4mono2ui9GraphView13setDataSourceERKNS0_20IGraphViewDataSourceE(&mut self.raw[0] as *mut u8, p0) }
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
        impl IGraphViewDataSource {
            pub fn BufferLenght(&mut self) -> i32 {
                unsafe { stub__ZN4mono2ui20IGraphViewDataSource12BufferLenghtEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn DataPoint(&mut self, p0: i32) -> i32 {
                unsafe { stub__ZN4mono2ui20IGraphViewDataSource9DataPointEi(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn MaxSampleValueSpan(&mut self) -> i32 {
                unsafe { stub__ZN4mono2ui20IGraphViewDataSource18MaxSampleValueSpanEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn NewestSampleIndex(&mut self) -> i32 {
                unsafe { stub__ZN4mono2ui20IGraphViewDataSource17NewestSampleIndexEv(&mut self.raw[0] as *mut u8) }
            }
        }
        impl ImageView {
            pub fn Crop(&mut self) -> &::generated::mono::geo::Rect {
                unsafe { stub__ZNK4mono2ui9ImageView4CropEv(&mut self.raw[0] as *mut u8) }
            }
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
            pub fn new() -> ::generated::mono::ui::ImageView {
                let mut o = ImageView { raw: [0; 64] };
                unsafe { stub__ZN4mono2ui9ImageViewC1Ev(&mut o.raw[0] as *mut u8); }
                o
            }
            pub fn new_PNS_5media5ImageE(p0: *mut ::generated::mono::media::Image) -> ::generated::mono::ui::ImageView {
                let mut o = ImageView { raw: [0; 64] };
                unsafe { stub__ZN4mono2ui9ImageViewC1EPNS_5media5ImageE(&mut o.raw[0] as *mut u8, p0); }
                o
            }
            pub fn repaint(&mut self) -> () {
                unsafe { stub__ZN4mono2ui9ImageView7repaintEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn scheduleRepaint(&mut self) -> () {
                unsafe { stub__ZN4mono2ui4View15scheduleRepaintEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn setCrop_4RectE(&mut self, p0: ::generated::mono::geo::Rect) -> () {
                unsafe { stub__ZN4mono2ui9ImageView7setCropENS_3geo4RectE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setCrop_4SizeE(&mut self, p0: ::generated::mono::geo::Size) -> () {
                unsafe { stub__ZN4mono2ui9ImageView7setCropENS_3geo4SizeE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setCrop_5PointE(&mut self, p0: ::generated::mono::geo::Point) -> () {
                unsafe { stub__ZN4mono2ui9ImageView7setCropENS_3geo5PointE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setImage(&mut self, p0: *mut ::generated::mono::media::Image) -> () {
                unsafe { stub__ZN4mono2ui9ImageView8setImageEPNS_5media5ImageE(&mut self.raw[0] as *mut u8, p0) }
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
        impl OnOffButtonView {
            pub fn Activate(&mut self) -> () {
                unsafe { stub__ZN4mono14TouchResponder8ActivateEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn BackgroundColor(&mut self) -> ::generated::mono::display::Color {
                unsafe { stub__ZNK4mono2ui15OnOffButtonView15BackgroundColorEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn BorderColor(&mut self) -> ::generated::mono::display::Color {
                unsafe { stub__ZNK4mono2ui15OnOffButtonView11BorderColorEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Deactivate(&mut self) -> () {
                unsafe { stub__ZN4mono14TouchResponder10DeactivateEv(&mut self.raw[0] as *mut u8) }
            }
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
            pub fn HighlightColor(&mut self) -> ::generated::mono::display::Color {
                unsafe { stub__ZNK4mono2ui15OnOffButtonView14HighlightColorEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Position(&mut self) -> &::generated::mono::geo::Point {
                unsafe { stub__ZN4mono2ui4View8PositionEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn RespondTouchBegin(&mut self, p0: &::generated::mono::TouchEvent) -> () {
                unsafe { stub__ZN4mono14TouchResponder17RespondTouchBeginERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn RespondTouchEnd(&mut self, p0: &::generated::mono::TouchEvent) -> () {
                unsafe { stub__ZN4mono14TouchResponder15RespondTouchEndERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn RespondTouchMove(&mut self, p0: &::generated::mono::TouchEvent) -> () {
                unsafe { stub__ZN4mono14TouchResponder16RespondTouchMoveERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn RunResponderChainTouchBegin(&mut self, p0: &::generated::mono::TouchEvent) -> () {
                unsafe { stub__ZN4mono14TouchResponder27RunResponderChainTouchBeginERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn RunResponderChainTouchEnd(&mut self, p0: &::generated::mono::TouchEvent) -> () {
                unsafe { stub__ZN4mono14TouchResponder25RunResponderChainTouchEndERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn RunResponderChainTouchMove(&mut self, p0: &::generated::mono::TouchEvent) -> () {
                unsafe { stub__ZN4mono14TouchResponder26RunResponderChainTouchMoveERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
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
                unsafe { stub__ZN4mono2ui13ResponderView4hideEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn isActive(&mut self) -> bool {
                unsafe { stub__ZNK4mono2ui15OnOffButtonView8isActiveEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn new() -> ::generated::mono::ui::OnOffButtonView {
                let mut o = OnOffButtonView { raw: [0; 360] };
                unsafe { stub__ZN4mono2ui15OnOffButtonViewC1Ev(&mut o.raw[0] as *mut u8); }
                o
            }
            pub fn new_NS_3geo4RectE(p0: ::generated::mono::geo::Rect) -> ::generated::mono::ui::OnOffButtonView {
                let mut o = OnOffButtonView { raw: [0; 360] };
                unsafe { stub__ZN4mono2ui15OnOffButtonViewC1ENS_3geo4RectE(&mut o.raw[0] as *mut u8, p0); }
                o
            }
            pub fn repaint(&mut self) -> () {
                unsafe { stub__ZN4mono2ui15OnOffButtonView7repaintEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn scheduleRepaint(&mut self) -> () {
                unsafe { stub__ZN4mono2ui4View15scheduleRepaintEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn setBackground(&mut self, p0: ::generated::mono::display::Color) -> () {
                unsafe { stub__ZN4mono2ui15OnOffButtonView13setBackgroundENS_7display5ColorE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setBorder(&mut self, p0: ::generated::mono::display::Color) -> () {
                unsafe { stub__ZN4mono2ui15OnOffButtonView9setBorderENS_7display5ColorE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setClickCallback(&mut self, p0: *mut fn () -> ()) -> () {
                unsafe { stub__ZN4mono2ui18AbstractButtonView16setClickCallbackEPFvvE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setHighlight(&mut self, p0: ::generated::mono::display::Color) -> () {
                unsafe { stub__ZN4mono2ui15OnOffButtonView12setHighlightENS_7display5ColorE(&mut self.raw[0] as *mut u8, p0) }
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
                unsafe { stub__ZN4mono2ui13ResponderView4showEv(&mut self.raw[0] as *mut u8) }
            }
        }
        impl ProgressBarView {
            pub fn CurrentValue(&mut self) -> i32 {
                unsafe { stub__ZNK4mono2ui15ProgressBarView12CurrentValueEv(&mut self.raw[0] as *mut u8) }
            }
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
            pub fn Maximum(&mut self) -> i32 {
                unsafe { stub__ZNK4mono2ui15ProgressBarView7MaximumEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Minimum(&mut self) -> i32 {
                unsafe { stub__ZNK4mono2ui15ProgressBarView7MinimumEv(&mut self.raw[0] as *mut u8) }
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
            pub fn new() -> ::generated::mono::ui::ProgressBarView {
                let mut o = ProgressBarView { raw: [0; 112] };
                unsafe { stub__ZN4mono2ui15ProgressBarViewC1Ev(&mut o.raw[0] as *mut u8); }
                o
            }
            pub fn new_NS_3geo4RectE(p0: ::generated::mono::geo::Rect) -> ::generated::mono::ui::ProgressBarView {
                let mut o = ProgressBarView { raw: [0; 112] };
                unsafe { stub__ZN4mono2ui15ProgressBarViewC1ENS_3geo4RectE(&mut o.raw[0] as *mut u8, p0); }
                o
            }
            pub fn scheduleRepaint(&mut self) -> () {
                unsafe { stub__ZN4mono2ui4View15scheduleRepaintEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn setMaximum(&mut self, p0: i32) -> () {
                unsafe { stub__ZN4mono2ui15ProgressBarView10setMaximumEi(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setMinimum(&mut self, p0: i32) -> () {
                unsafe { stub__ZN4mono2ui15ProgressBarView10setMinimumEi(&mut self.raw[0] as *mut u8, p0) }
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
            pub fn setValue(&mut self, p0: i32) -> () {
                unsafe { stub__ZN4mono2ui15ProgressBarView8setValueEi(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn show(&mut self) -> () {
                unsafe { stub__ZN4mono2ui4View4showEv(&mut self.raw[0] as *mut u8) }
            }
        }
        impl ResponderView {
            pub fn Activate(&mut self) -> () {
                unsafe { stub__ZN4mono14TouchResponder8ActivateEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Deactivate(&mut self) -> () {
                unsafe { stub__ZN4mono14TouchResponder10DeactivateEv(&mut self.raw[0] as *mut u8) }
            }
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
            pub fn RespondTouchBegin(&mut self, p0: &::generated::mono::TouchEvent) -> () {
                unsafe { stub__ZN4mono14TouchResponder17RespondTouchBeginERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn RespondTouchEnd(&mut self, p0: &::generated::mono::TouchEvent) -> () {
                unsafe { stub__ZN4mono14TouchResponder15RespondTouchEndERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn RespondTouchMove(&mut self, p0: &::generated::mono::TouchEvent) -> () {
                unsafe { stub__ZN4mono14TouchResponder16RespondTouchMoveERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn RunResponderChainTouchBegin(&mut self, p0: &::generated::mono::TouchEvent) -> () {
                unsafe { stub__ZN4mono14TouchResponder27RunResponderChainTouchBeginERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn RunResponderChainTouchEnd(&mut self, p0: &::generated::mono::TouchEvent) -> () {
                unsafe { stub__ZN4mono14TouchResponder25RunResponderChainTouchEndERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn RunResponderChainTouchMove(&mut self, p0: &::generated::mono::TouchEvent) -> () {
                unsafe { stub__ZN4mono14TouchResponder26RunResponderChainTouchMoveERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
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
                unsafe { stub__ZN4mono2ui13ResponderView4hideEv(&mut self.raw[0] as *mut u8) }
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
                unsafe { stub__ZN4mono2ui13ResponderView4showEv(&mut self.raw[0] as *mut u8) }
            }
        }
        impl StatusIndicatorView {
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
            pub fn State(&mut self) -> bool {
                unsafe { stub__ZNK4mono2ui19StatusIndicatorView5StateEv(&mut self.raw[0] as *mut u8) }
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
            pub fn new() -> ::generated::mono::ui::StatusIndicatorView {
                let mut o = StatusIndicatorView { raw: [0; 48] };
                unsafe { stub__ZN4mono2ui19StatusIndicatorViewC1Ev(&mut o.raw[0] as *mut u8); }
                o
            }
            pub fn new_NS_3geo4RectE(p0: ::generated::mono::geo::Rect) -> ::generated::mono::ui::StatusIndicatorView {
                let mut o = StatusIndicatorView { raw: [0; 48] };
                unsafe { stub__ZN4mono2ui19StatusIndicatorViewC1ENS_3geo4RectE(&mut o.raw[0] as *mut u8, p0); }
                o
            }
            pub fn new_NS_3geo4RectEb(p0: ::generated::mono::geo::Rect, p1: bool) -> ::generated::mono::ui::StatusIndicatorView {
                let mut o = StatusIndicatorView { raw: [0; 48] };
                unsafe { stub__ZN4mono2ui19StatusIndicatorViewC1ENS_3geo4RectEb(&mut o.raw[0] as *mut u8, p0, p1); }
                o
            }
            pub fn scheduleRepaint(&mut self) -> () {
                unsafe { stub__ZN4mono2ui4View15scheduleRepaintEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn setOffStateColor(&mut self, p0: ::generated::mono::display::Color) -> () {
                unsafe { stub__ZN4mono2ui19StatusIndicatorView16setOffStateColorENS_7display5ColorE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setOnStateColor(&mut self, p0: ::generated::mono::display::Color) -> () {
                unsafe { stub__ZN4mono2ui19StatusIndicatorView15setOnStateColorENS_7display5ColorE(&mut self.raw[0] as *mut u8, p0) }
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
            pub fn setState(&mut self, p0: bool) -> () {
                unsafe { stub__ZN4mono2ui19StatusIndicatorView8setStateEb(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn show(&mut self) -> () {
                unsafe { stub__ZN4mono2ui4View4showEv(&mut self.raw[0] as *mut u8) }
            }
        }
        impl TextLabelView {
            pub fn Alignment(&mut self) -> i32 {
                unsafe { stub__ZNK4mono2ui13TextLabelView9AlignmentEv(&mut self.raw[0] as *mut u8) }
            }
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
            pub fn Font(&mut self) -> &::generated::MonoFont {
                unsafe { stub__ZNK4mono2ui13TextLabelView4FontEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Position(&mut self) -> &::generated::mono::geo::Point {
                unsafe { stub__ZN4mono2ui4View8PositionEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Size(&mut self) -> &::generated::mono::geo::Size {
                unsafe { stub__ZN4mono2ui4View4SizeEv(&mut self.raw[0] as *mut u8) }
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
            pub fn ViewRect(&mut self) -> &::generated::mono::geo::Rect {
                unsafe { stub__ZNK4mono2ui4View8ViewRectEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Visible(&mut self) -> bool {
                unsafe { stub__ZNK4mono2ui4View7VisibleEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn hide(&mut self) -> () {
                unsafe { stub__ZN4mono2ui4View4hideEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn new_NS_3geo4RectENS_6StringE(p0: ::generated::mono::geo::Rect, p1: ::generated::mono::String) -> ::generated::mono::ui::TextLabelView {
                let mut o = TextLabelView { raw: [0; 112] };
                unsafe { stub__ZN4mono2ui13TextLabelViewC1ENS_3geo4RectENS_6StringE(&mut o.raw[0] as *mut u8, p0, p1); }
                o
            }
            pub fn new_NS_3geo4RectEPKc(p0: ::generated::mono::geo::Rect, p1: *mut i8) -> ::generated::mono::ui::TextLabelView {
                let mut o = TextLabelView { raw: [0; 112] };
                unsafe { stub__ZN4mono2ui13TextLabelViewC1ENS_3geo4RectEPKc(&mut o.raw[0] as *mut u8, p0, p1); }
                o
            }
            pub fn new_NS_6StringE(p0: ::generated::mono::String) -> ::generated::mono::ui::TextLabelView {
                let mut o = TextLabelView { raw: [0; 112] };
                unsafe { stub__ZN4mono2ui13TextLabelViewC1ENS_6StringE(&mut o.raw[0] as *mut u8, p0); }
                o
            }
            pub fn new_PKc(p0: *mut i8) -> ::generated::mono::ui::TextLabelView {
                let mut o = TextLabelView { raw: [0; 112] };
                unsafe { stub__ZN4mono2ui13TextLabelViewC1EPKc(&mut o.raw[0] as *mut u8, p0); }
                o
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
            pub fn setPosition(&mut self, p0: ::generated::mono::geo::Point) -> () {
                unsafe { stub__ZN4mono2ui4View11setPositionENS_3geo5PointE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setRect(&mut self, p0: ::generated::mono::geo::Rect) -> () {
                unsafe { stub__ZN4mono2ui4View7setRectENS_3geo4RectE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setSize(&mut self, p0: ::generated::mono::geo::Size) -> () {
                unsafe { stub__ZN4mono2ui4View7setSizeENS_3geo4SizeE(&mut self.raw[0] as *mut u8, p0) }
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
            pub fn show(&mut self) -> () {
                unsafe { stub__ZN4mono2ui4View4showEv(&mut self.raw[0] as *mut u8) }
            }
        }
        impl TouchCalibrateView {
            pub fn Activate(&mut self) -> () {
                unsafe { stub__ZN4mono14TouchResponder8ActivateEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Deactivate(&mut self) -> () {
                unsafe { stub__ZN4mono14TouchResponder10DeactivateEv(&mut self.raw[0] as *mut u8) }
            }
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
            pub fn RespondTouchBegin(&mut self, p0: &::generated::mono::TouchEvent) -> () {
                unsafe { stub__ZN4mono2ui18TouchCalibrateView17RespondTouchBeginERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn RespondTouchEnd(&mut self, p0: &::generated::mono::TouchEvent) -> () {
                unsafe { stub__ZN4mono14TouchResponder15RespondTouchEndERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn RespondTouchMove(&mut self, p0: &::generated::mono::TouchEvent) -> () {
                unsafe { stub__ZN4mono14TouchResponder16RespondTouchMoveERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn RunResponderChainTouchBegin(&mut self, p0: &::generated::mono::TouchEvent) -> () {
                unsafe { stub__ZN4mono14TouchResponder27RunResponderChainTouchBeginERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn RunResponderChainTouchEnd(&mut self, p0: &::generated::mono::TouchEvent) -> () {
                unsafe { stub__ZN4mono14TouchResponder25RunResponderChainTouchEndERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn RunResponderChainTouchMove(&mut self, p0: &::generated::mono::TouchEvent) -> () {
                unsafe { stub__ZN4mono14TouchResponder26RunResponderChainTouchMoveERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn Size(&mut self) -> &::generated::mono::geo::Size {
                unsafe { stub__ZN4mono2ui4View4SizeEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn StartNewCalibration(&mut self) -> () {
                unsafe { stub__ZN4mono2ui18TouchCalibrateView19StartNewCalibrationEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn ViewRect(&mut self) -> &::generated::mono::geo::Rect {
                unsafe { stub__ZNK4mono2ui4View8ViewRectEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Visible(&mut self) -> bool {
                unsafe { stub__ZNK4mono2ui4View7VisibleEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn hide(&mut self) -> () {
                unsafe { stub__ZN4mono2ui13ResponderView4hideEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn new() -> ::generated::mono::ui::TouchCalibrateView {
                let mut o = TouchCalibrateView { raw: [0; 408] };
                unsafe { stub__ZN4mono2ui18TouchCalibrateViewC1Ev(&mut o.raw[0] as *mut u8); }
                o
            }
            pub fn new_NS_3geo4RectE(p0: ::generated::mono::geo::Rect) -> ::generated::mono::ui::TouchCalibrateView {
                let mut o = TouchCalibrateView { raw: [0; 408] };
                unsafe { stub__ZN4mono2ui18TouchCalibrateViewC1ENS_3geo4RectE(&mut o.raw[0] as *mut u8, p0); }
                o
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
                unsafe { stub__ZN4mono2ui18TouchCalibrateView4showEv(&mut self.raw[0] as *mut u8) }
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
            pub fn new() -> ::generated::mono::ui::View {
                let mut o = View { raw: [0; 40] };
                unsafe { stub__ZN4mono2ui4ViewC1Ev(&mut o.raw[0] as *mut u8); }
                o
            }
            pub fn new_NS_3geo4RectE(p0: ::generated::mono::geo::Rect) -> ::generated::mono::ui::View {
                let mut o = View { raw: [0; 40] };
                unsafe { stub__ZN4mono2ui4ViewC1ENS_3geo4RectE(&mut o.raw[0] as *mut u8, p0); }
                o
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
    #[allow(dead_code)]
    pub struct Console {
        raw: [u8; 8],
    }
    #[allow(dead_code)]
    pub struct IApplication {
        raw: [u8; 8],
    }
    #[allow(dead_code)]
    pub struct IApplicationContext {
        raw: [u8; 80],
    }
    #[allow(dead_code)]
    pub struct IQueueItem {
        raw: [u8; 8],
    }
    #[allow(dead_code)]
    pub struct IRunLoopTask {
        raw: [u8; 32],
    }
    #[allow(dead_code)]
    pub struct ITouchSystem {
        raw: [u8; 64],
    }
    #[allow(dead_code)]
    pub struct Queue {
        raw: [u8; 16],
    }
    #[allow(dead_code)]
    pub struct QueueInterrupt {
        raw: [u8; 464],
    }
    #[allow(dead_code)]
    pub struct String {
        raw: [u8; 24],
    }
    #[allow(dead_code)]
    pub struct Timer {
        raw: [u8; 176],
    }
    #[allow(dead_code)]
    pub struct TouchEvent {
        raw: [u8; 40],
    }
    #[allow(dead_code)]
    extern "C" {
        // mbed::InterruptIn::enable_irq()
        fn stub__ZN4mbed11InterruptIn10enable_irqEv(o: *mut u8) -> ();
        // mbed::InterruptIn::disable_irq()
        fn stub__ZN4mbed11InterruptIn11disable_irqEv(o: *mut u8) -> ();
        // mbed::InterruptIn::_irq_handler(uint32_t, gpio_irq_event)
        fn stub__ZN4mbed11InterruptIn12_irq_handlerEj14gpio_irq_event(o: *mut u8, p0: u32, p1: i32) -> ();
        // mbed::InterruptIn::mode(PinMode)
        fn stub__ZN4mbed11InterruptIn4modeE7PinMode(o: *mut u8, p0: i32) -> ();
        // mbed::InterruptIn::read()
        fn stub__ZN4mbed11InterruptIn4readEv(o: *mut u8) -> i32;
        // mono::TouchEvent::TouchEvent(mono::TouchEvent::TouchEventType, geo::Point &, mono::ITouchSystem *)
        fn stub__ZN4mono10TouchEventC1ENS0_14TouchEventTypeERNS_3geo5PointEPNS_12ITouchSystemE(o: *mut u8, p0: i32, p1: &::generated::mono::geo::Point, p2: *mut ::generated::mono::ITouchSystem) -> ();
        // mono::TouchEvent::TouchEvent(const mono::TouchEvent &)
        fn stub__ZN4mono10TouchEventC1ERKS0_(o: *mut u8, p0: &::generated::mono::TouchEvent) -> ();
        // mono::TouchEvent::TouchEvent()
        fn stub__ZN4mono10TouchEventC1Ev(o: *mut u8) -> ();
        // mono::TouchEvent::operator=(const mono::TouchEvent &)
        fn stub__ZN4mono10TouchEventaSERKS0_(o: *mut u8, p0: &::generated::mono::TouchEvent) -> &::generated::mono::TouchEvent;
        // mono::IApplication::enterRunLoop()
        fn stub__ZN4mono12IApplication12enterRunLoopEv(o: *mut u8) -> i32;
        // mono::IApplication::monoWakeFromReset()
        fn stub__ZN4mono12IApplication17monoWakeFromResetEv(o: *mut u8) -> ();
        // mono::IApplication::monoWakeFromSleep()
        fn stub__ZN4mono12IApplication17monoWakeFromSleepEv(o: *mut u8) -> ();
        // mono::IApplication::monoWillGotoSleep()
        fn stub__ZN4mono12IApplication17monoWillGotoSleepEv(o: *mut u8) -> ();
        // mono::IApplication::IApplication()
        fn stub__ZN4mono12IApplicationC1Ev(o: *mut u8) -> ();
        // mono::ITouchSystem::setCalibration(TouchCalibration &)
        fn stub__ZN4mono12ITouchSystem14setCalibrationERNS_3geo4RectE(o: *mut u8, p0: &::generated::mono::geo::Rect) -> ();
        // mono::ITouchSystem::ToScreenCoordsX(int, uint16_t)
        fn stub__ZN4mono12ITouchSystem15ToScreenCoordsXEit(o: *mut u8, p0: i32, p1: u16) -> i32;
        // mono::ITouchSystem::ToScreenCoordsY(int, uint16_t)
        fn stub__ZN4mono12ITouchSystem15ToScreenCoordsYEit(o: *mut u8, p0: i32, p1: u16) -> i32;
        // mono::ITouchSystem::processTouchInput()
        fn stub__ZN4mono12ITouchSystem17processTouchInputEv(o: *mut u8) -> ();
        // mono::ITouchSystem::CurrentCalibration()
        fn stub__ZN4mono12ITouchSystem18CurrentCalibrationEv(o: *mut u8) -> ::generated::mono::geo::Rect;
        // mono::ITouchSystem::init()
        fn stub__ZN4mono12ITouchSystem4initEv(o: *mut u8) -> ();
        // mono::QueueInterrupt::FallTimeStamp()
        fn stub__ZN4mono14QueueInterrupt13FallTimeStampEv(o: *mut u8) -> u32;
        // mono::QueueInterrupt::RiseTimeStamp()
        fn stub__ZN4mono14QueueInterrupt13RiseTimeStampEv(o: *mut u8) -> u32;
        // mono::QueueInterrupt::setDebouncing(bool)
        fn stub__ZN4mono14QueueInterrupt13setDebouncingEb(o: *mut u8, p0: bool) -> ();
        // mono::QueueInterrupt::setDebounceTimeout(int)
        fn stub__ZN4mono14QueueInterrupt18setDebounceTimeoutEi(o: *mut u8, p0: i32) -> ();
        // mono::QueueInterrupt::DeactivateUntilHandled(bool)
        fn stub__ZN4mono14QueueInterrupt22DeactivateUntilHandledEb(o: *mut u8, p0: bool) -> ();
        // mono::QueueInterrupt::fall(void (*)())
        fn stub__ZN4mono14QueueInterrupt4fallEPFvvE(o: *mut u8, p0: *mut fn () -> ()) -> ();
        // mono::QueueInterrupt::rise(void (*)())
        fn stub__ZN4mono14QueueInterrupt4riseEPFvvE(o: *mut u8, p0: *mut fn () -> ()) -> ();
        // mono::QueueInterrupt::Snapshot()
        fn stub__ZN4mono14QueueInterrupt8SnapshotEv(o: *mut u8) -> bool;
        // mono::QueueInterrupt::QueueInterrupt(PinName, PinMode)
        fn stub__ZN4mono14QueueInterruptC1E7PinName7PinMode(o: *mut u8, p0: i32, p1: i32) -> ();
        // mono::QueueInterrupt::~QueueInterrupt()
        fn stub__ZN4mono14QueueInterruptD0Ev(o: *mut u8) -> ();
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
        // mono::Queue::Next(mono::IQueueItem *)
        fn stub__ZN4mono5Queue4NextEPNS_10IQueueItemE(o: *mut u8, p0: *mut ::generated::mono::IQueueItem) -> *mut ::generated::mono::IQueueItem;
        // mono::Queue::Peek()
        fn stub__ZN4mono5Queue4PeekEv(o: *mut u8) -> *mut ::generated::mono::IQueueItem;
        // mono::Queue::Exists(mono::IQueueItem *)
        fn stub__ZN4mono5Queue6ExistsEPNS_10IQueueItemE(o: *mut u8, p0: *mut ::generated::mono::IQueueItem) -> bool;
        // mono::Queue::Length()
        fn stub__ZN4mono5Queue6LengthEv(o: *mut u8) -> u16;
        // mono::Queue::Remove(mono::IQueueItem *)
        fn stub__ZN4mono5Queue6RemoveEPNS_10IQueueItemE(o: *mut u8, p0: *mut ::generated::mono::IQueueItem) -> bool;
        // mono::Queue::Dequeue()
        fn stub__ZN4mono5Queue7DequeueEv(o: *mut u8) -> *mut ::generated::mono::IQueueItem;
        // mono::Queue::Enqueue(mono::IQueueItem *)
        fn stub__ZN4mono5Queue7EnqueueEPNS_10IQueueItemE(o: *mut u8, p0: *mut ::generated::mono::IQueueItem) -> ();
        // mono::Queue::Queue()
        fn stub__ZN4mono5QueueC1Ev(o: *mut u8) -> ();
        // mono::Timer::SingleShot()
        fn stub__ZN4mono5Timer10SingleShotEv(o: *mut u8) -> bool;
        // mono::Timer::setCallback(void (*)())
        fn stub__ZN4mono5Timer11setCallbackEPFvvE(o: *mut u8, p0: *mut fn () -> ()) -> ();
        // mono::Timer::setInterval(uint32_t)
        fn stub__ZN4mono5Timer11setIntervalEj(o: *mut u8, p0: u32) -> ();
        // mono::Timer::Stop()
        fn stub__ZN4mono5Timer4StopEv(o: *mut u8) -> ();
        // mono::Timer::Start()
        fn stub__ZN4mono5Timer5StartEv(o: *mut u8) -> ();
        // mono::Timer::Running()
        fn stub__ZN4mono5Timer7RunningEv(o: *mut u8) -> bool;
        // mono::Timer::callOnce(uint32_t, void (*)())
        fn stub__ZN4mono5Timer8callOnceEjPFvvE(o: *mut u8, p0: u32, p1: *mut fn () -> ()) -> *mut ::generated::mono::Timer;
        // mono::Timer::Timer(uint32_t, bool)
        fn stub__ZN4mono5TimerC1Ejb(o: *mut u8, p0: u32, p1: bool) -> ();
        // mono::Timer::Timer()
        fn stub__ZN4mono5TimerC1Ev(o: *mut u8) -> ();
        // mono::Timer::~Timer()
        fn stub__ZN4mono5TimerD0Ev(o: *mut u8) -> ();
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
        // mono::Console::Console(mbed::Serial *)
        fn stub__ZN4mono7ConsoleC1EPN4mbed6SerialE(o: *mut u8, p0: *mut ::generated::mbed::Serial) -> ();
        // mono::Console::operator<<(const char *)
        fn stub__ZN4mono7ConsolelsEPKc(o: *mut u8, p0: *mut i8) -> &::generated::mono::Console;
        // mono::Console::operator<<(char)
        fn stub__ZN4mono7ConsolelsEc(o: *mut u8, p0: i8) -> &::generated::mono::Console;
        // mono::Console::operator<<(float)
        fn stub__ZN4mono7ConsolelsEf(o: *mut u8, p0: f32) -> &::generated::mono::Console;
        // mono::Console::operator<<(int)
        fn stub__ZN4mono7ConsolelsEi(o: *mut u8, p0: i32) -> &::generated::mono::Console;
        // mono::QueueInterrupt::IsInterruptsWhilePendingActive()
        fn stub__ZNK4mono14QueueInterrupt30IsInterruptsWhilePendingActiveEv(o: *mut u8) -> bool;
        // mono::String::Length()
        fn stub__ZNK4mono6String6LengthEv(o: *mut u8) -> u32;
        // mono::String::operator()()
        fn stub__ZNK4mono6StringclEv(o: *mut u8) -> *mut i8;
        // mono::String::operator[](uint32_t)
        fn stub__ZNK4mono6StringixEj(o: *mut u8, p0: u32) -> i8;
    }
    impl Console {
        pub fn new(p0: *mut ::generated::mbed::Serial) -> ::generated::mono::Console {
            let mut o = Console { raw: [0; 8] };
            unsafe { stub__ZN4mono7ConsoleC1EPN4mbed6SerialE(&mut o.raw[0] as *mut u8, p0); }
            o
        }
    }
    impl Drop for QueueInterrupt {
         fn drop(&mut self) -> () {
            unsafe { stub__ZN4mono14QueueInterruptD0Ev(&mut self.raw[0] as *mut u8) }
        }
    }
    impl Drop for String {
         fn drop(&mut self) -> () {
            unsafe { stub__ZN4mono6StringD0Ev(&mut self.raw[0] as *mut u8) }
        }
    }
    impl Drop for Timer {
         fn drop(&mut self) -> () {
            unsafe { stub__ZN4mono5TimerD0Ev(&mut self.raw[0] as *mut u8) }
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
        pub fn new() -> ::generated::mono::IApplication {
            let mut o = IApplication { raw: [0; 8] };
            unsafe { stub__ZN4mono12IApplicationC1Ev(&mut o.raw[0] as *mut u8); }
            o
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
    impl ITouchSystem {
        pub fn CurrentCalibration(&mut self) -> ::generated::mono::geo::Rect {
            unsafe { stub__ZN4mono12ITouchSystem18CurrentCalibrationEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn ToScreenCoordsX(&mut self, p0: i32, p1: u16) -> i32 {
            unsafe { stub__ZN4mono12ITouchSystem15ToScreenCoordsXEit(&mut self.raw[0] as *mut u8, p0, p1) }
        }
        pub fn ToScreenCoordsY(&mut self, p0: i32, p1: u16) -> i32 {
            unsafe { stub__ZN4mono12ITouchSystem15ToScreenCoordsYEit(&mut self.raw[0] as *mut u8, p0, p1) }
        }
        pub fn init(&mut self) -> () {
            unsafe { stub__ZN4mono12ITouchSystem4initEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn processTouchInput(&mut self) -> () {
            unsafe { stub__ZN4mono12ITouchSystem17processTouchInputEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn setCalibration(&mut self, p0: &::generated::mono::geo::Rect) -> () {
            unsafe { stub__ZN4mono12ITouchSystem14setCalibrationERNS_3geo4RectE(&mut self.raw[0] as *mut u8, p0) }
        }
    }
    impl Queue {
        pub fn Dequeue(&mut self) -> *mut ::generated::mono::IQueueItem {
            unsafe { stub__ZN4mono5Queue7DequeueEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn Enqueue(&mut self, p0: *mut ::generated::mono::IQueueItem) -> () {
            unsafe { stub__ZN4mono5Queue7EnqueueEPNS_10IQueueItemE(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn Exists(&mut self, p0: *mut ::generated::mono::IQueueItem) -> bool {
            unsafe { stub__ZN4mono5Queue6ExistsEPNS_10IQueueItemE(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn Length(&mut self) -> u16 {
            unsafe { stub__ZN4mono5Queue6LengthEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn Next(&mut self, p0: *mut ::generated::mono::IQueueItem) -> *mut ::generated::mono::IQueueItem {
            unsafe { stub__ZN4mono5Queue4NextEPNS_10IQueueItemE(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn Peek(&mut self) -> *mut ::generated::mono::IQueueItem {
            unsafe { stub__ZN4mono5Queue4PeekEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn Remove(&mut self, p0: *mut ::generated::mono::IQueueItem) -> bool {
            unsafe { stub__ZN4mono5Queue6RemoveEPNS_10IQueueItemE(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn new() -> ::generated::mono::Queue {
            let mut o = Queue { raw: [0; 16] };
            unsafe { stub__ZN4mono5QueueC1Ev(&mut o.raw[0] as *mut u8); }
            o
        }
    }
    impl QueueInterrupt {
        pub fn DeactivateUntilHandled(&mut self, p0: bool) -> () {
            unsafe { stub__ZN4mono14QueueInterrupt22DeactivateUntilHandledEb(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn FallTimeStamp(&mut self) -> u32 {
            unsafe { stub__ZN4mono14QueueInterrupt13FallTimeStampEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn IsInterruptsWhilePendingActive(&mut self) -> bool {
            unsafe { stub__ZNK4mono14QueueInterrupt30IsInterruptsWhilePendingActiveEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn RiseTimeStamp(&mut self) -> u32 {
            unsafe { stub__ZN4mono14QueueInterrupt13RiseTimeStampEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn Snapshot(&mut self) -> bool {
            unsafe { stub__ZN4mono14QueueInterrupt8SnapshotEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn _irq_handler(&mut self, p0: u32, p1: i32) -> () {
            unsafe { stub__ZN4mbed11InterruptIn12_irq_handlerEj14gpio_irq_event(&mut self.raw[0] as *mut u8, p0, p1) }
        }
        pub fn disable_irq(&mut self) -> () {
            unsafe { stub__ZN4mbed11InterruptIn11disable_irqEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn enable_irq(&mut self) -> () {
            unsafe { stub__ZN4mbed11InterruptIn10enable_irqEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn fall(&mut self, p0: *mut fn () -> ()) -> () {
            unsafe { stub__ZN4mono14QueueInterrupt4fallEPFvvE(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn mode(&mut self, p0: i32) -> () {
            unsafe { stub__ZN4mbed11InterruptIn4modeE7PinMode(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn new(p0: i32, p1: i32) -> ::generated::mono::QueueInterrupt {
            let mut o = QueueInterrupt { raw: [0; 464] };
            unsafe { stub__ZN4mono14QueueInterruptC1E7PinName7PinMode(&mut o.raw[0] as *mut u8, p0, p1); }
            o
        }
        pub fn read(&mut self) -> i32 {
            unsafe { stub__ZN4mbed11InterruptIn4readEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn rise(&mut self, p0: *mut fn () -> ()) -> () {
            unsafe { stub__ZN4mono14QueueInterrupt4riseEPFvvE(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn setDebounceTimeout(&mut self, p0: i32) -> () {
            unsafe { stub__ZN4mono14QueueInterrupt18setDebounceTimeoutEi(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn setDebouncing(&mut self, p0: bool) -> () {
            unsafe { stub__ZN4mono14QueueInterrupt13setDebouncingEb(&mut self.raw[0] as *mut u8, p0) }
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
    impl Timer {
        pub fn Running(&mut self) -> bool {
            unsafe { stub__ZN4mono5Timer7RunningEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn SingleShot(&mut self) -> bool {
            unsafe { stub__ZN4mono5Timer10SingleShotEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn Start(&mut self) -> () {
            unsafe { stub__ZN4mono5Timer5StartEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn Stop(&mut self) -> () {
            unsafe { stub__ZN4mono5Timer4StopEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn callOnce(&mut self, p0: u32, p1: *mut fn () -> ()) -> *mut ::generated::mono::Timer {
            unsafe { stub__ZN4mono5Timer8callOnceEjPFvvE(&mut self.raw[0] as *mut u8, p0, p1) }
        }
        pub fn new() -> ::generated::mono::Timer {
            let mut o = Timer { raw: [0; 176] };
            unsafe { stub__ZN4mono5TimerC1Ev(&mut o.raw[0] as *mut u8); }
            o
        }
        pub fn new_jb(p0: u32, p1: bool) -> ::generated::mono::Timer {
            let mut o = Timer { raw: [0; 176] };
            unsafe { stub__ZN4mono5TimerC1Ejb(&mut o.raw[0] as *mut u8, p0, p1); }
            o
        }
        pub fn setCallback(&mut self, p0: *mut fn () -> ()) -> () {
            unsafe { stub__ZN4mono5Timer11setCallbackEPFvvE(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn setInterval(&mut self, p0: u32) -> () {
            unsafe { stub__ZN4mono5Timer11setIntervalEj(&mut self.raw[0] as *mut u8, p0) }
        }
    }
    impl TouchEvent {
        pub fn new() -> ::generated::mono::TouchEvent {
            let mut o = TouchEvent { raw: [0; 40] };
            unsafe { stub__ZN4mono10TouchEventC1Ev(&mut o.raw[0] as *mut u8); }
            o
        }
        pub fn new_NS0_14TouchEventTypeERNS_3geo5PointEPNS_12ITouchSystemE(p0: i32, p1: &::generated::mono::geo::Point, p2: *mut ::generated::mono::ITouchSystem) -> ::generated::mono::TouchEvent {
            let mut o = TouchEvent { raw: [0; 40] };
            unsafe { stub__ZN4mono10TouchEventC1ENS0_14TouchEventTypeERNS_3geo5PointEPNS_12ITouchSystemE(&mut o.raw[0] as *mut u8, p0, p1, p2); }
            o
        }
        pub fn new_RKS0_(p0: &::generated::mono::TouchEvent) -> ::generated::mono::TouchEvent {
            let mut o = TouchEvent { raw: [0; 40] };
            unsafe { stub__ZN4mono10TouchEventC1ERKS0_(&mut o.raw[0] as *mut u8, p0); }
            o
        }
    }
}
#[allow(dead_code)]
pub struct MonoFont {
    raw: [u8; 32],
}
#[allow(dead_code)]
extern "C" {
}
