pub mod mono {
    pub mod geo {
        pub struct Circle {
            raw: [u8; 12],
        }
        pub struct Point {
            raw: [u8; 8],
        }
        pub struct Rect {
            raw: [u8; 16],
        }
        pub struct Size {
            raw: [u8; 8],
        }
        extern "C" {
            // mono::geo::Rect::Area()
            fn stub__ZN4mono3geo4Rect4AreaEv(o: *mut u8) -> i32;
            // mono::geo::Rect::Size()
            fn stub__ZN4mono3geo4Rect4SizeEv(o: *mut u8) -> &::mono::geo::Size;
            // mono::geo::Rect::setSize(class Size)
            fn stub__ZN4mono3geo4Rect7setSizeENS0_4SizeE(o: *mut u8, p0: ::mono::geo::Size) -> ();
            // mono::geo::Rect::setPoint(class Point)
            fn stub__ZN4mono3geo4Rect8setPointENS0_5PointE(o: *mut u8, p0: ::mono::geo::Point) -> ();
            // mono::geo::Rect::Rect(const mono::geo::Rect &)
            fn stub__ZN4mono3geo4RectC1ERKS1_(o: *mut u8, p0: &const ::mono::geo::Rect) -> ();
            // mono::geo::Rect::Rect(mono::geo::Point &, mono::geo::Size &)
            fn stub__ZN4mono3geo4RectC1ERNS0_5PointERNS0_4SizeE(o: *mut u8, p0: &::mono::geo::Point, p1: &::mono::geo::Size) -> ();
            // mono::geo::Rect::Rect(int, int, int, int)
            fn stub__ZN4mono3geo4RectC1Eiiii(o: *mut u8, p0: i32, p1: i32, p2: i32, p3: i32) -> ();
            // mono::geo::Rect::Rect()
            fn stub__ZN4mono3geo4RectC1Ev(o: *mut u8) -> ();
            // mono::geo::Rect::operator=(mono::geo::Rect)
            fn stub__ZN4mono3geo4RectaSES1_(o: *mut u8, p0: ::mono::geo::Rect) -> &::mono::geo::Rect;
            // mono::geo::Size::setWidth(int)
            fn stub__ZN4mono3geo4Size8setWidthEi(o: *mut u8, p0: i32) -> ();
            // mono::geo::Size::setHeight(int)
            fn stub__ZN4mono3geo4Size9setHeightEi(o: *mut u8, p0: i32) -> ();
            // mono::geo::Size::Size(const mono::geo::Size &)
            fn stub__ZN4mono3geo4SizeC1ERKS1_(o: *mut u8, p0: &const ::mono::geo::Size) -> ();
            // mono::geo::Size::Size(int, int)
            fn stub__ZN4mono3geo4SizeC1Eii(o: *mut u8, p0: i32, p1: i32) -> ();
            // mono::geo::Size::Size()
            fn stub__ZN4mono3geo4SizeC1Ev(o: *mut u8) -> ();
            // mono::geo::Size::operator=(const mono::geo::Size &)
            fn stub__ZN4mono3geo4SizeaSERKS1_(o: *mut u8, p0: &const ::mono::geo::Size) -> &::mono::geo::Size;
            // mono::geo::Point::setX(int)
            fn stub__ZN4mono3geo5Point4setXEi(o: *mut u8, p0: i32) -> ();
            // mono::geo::Point::setY(int)
            fn stub__ZN4mono3geo5Point4setYEi(o: *mut u8, p0: i32) -> ();
            // mono::geo::Point::appendX(int)
            fn stub__ZN4mono3geo5Point7appendXEi(o: *mut u8, p0: i32) -> ();
            // mono::geo::Point::appendY(int)
            fn stub__ZN4mono3geo5Point7appendYEi(o: *mut u8, p0: i32) -> ();
            // mono::geo::Point::Point(const mono::geo::Point &)
            fn stub__ZN4mono3geo5PointC1ERKS1_(o: *mut u8, p0: &const ::mono::geo::Point) -> ();
            // mono::geo::Point::Point(int, int)
            fn stub__ZN4mono3geo5PointC1Eii(o: *mut u8, p0: i32, p1: i32) -> ();
            // mono::geo::Point::Point()
            fn stub__ZN4mono3geo5PointC1Ev(o: *mut u8) -> ();
            // mono::geo::Point::operator=(const mono::geo::Point &)
            fn stub__ZN4mono3geo5PointaSERKS1_(o: *mut u8, p0: &const ::mono::geo::Point) -> &::mono::geo::Point;
            // mono::geo::Circle::Circle(mono::geo::Point, uint32_t)
            fn stub__ZN4mono3geo6CircleC1ENS0_5PointEj(o: *mut u8, p0: ::mono::geo::Point, p1: u32) -> ();
            // mono::geo::Circle::Circle(int, int, uint32_t)
            fn stub__ZN4mono3geo6CircleC1Eiij(o: *mut u8, p0: i32, p1: i32, p2: u32) -> ();
            // mono::geo::Circle::Circle()
            fn stub__ZN4mono3geo6CircleC1Ev(o: *mut u8) -> ();
            // mono::geo::Rect::LowerRight()
            fn stub__ZNK4mono3geo4Rect10LowerRightEv(o: *mut u8) -> ::mono::geo::Point;
            // mono::geo::Rect::UpperRight()
            fn stub__ZNK4mono3geo4Rect10UpperRightEv(o: *mut u8) -> ::mono::geo::Point;
            // mono::geo::Rect::X2()
            fn stub__ZNK4mono3geo4Rect2X2Ev(o: *mut u8) -> i32;
            // mono::geo::Rect::Y2()
            fn stub__ZNK4mono3geo4Rect2Y2Ev(o: *mut u8) -> i32;
            // mono::geo::Rect::crop(const mono::geo::Rect &)
            fn stub__ZNK4mono3geo4Rect4cropERKS1_(o: *mut u8, p0: &const ::mono::geo::Rect) -> ::mono::geo::Rect;
            // mono::geo::Rect::Point()
            fn stub__ZNK4mono3geo4Rect5PointEv(o: *mut u8) -> ::mono::geo::Point;
            // mono::geo::Rect::Center()
            fn stub__ZNK4mono3geo4Rect6CenterEv(o: *mut u8) -> ::mono::geo::Point;
            // mono::geo::Rect::ToString()
            fn stub__ZNK4mono3geo4Rect8ToStringEv(o: *mut u8) -> ::mono::String;
            // mono::geo::Rect::contains(const mono::geo::Rect &)
            fn stub__ZNK4mono3geo4Rect8containsERKS1_(o: *mut u8, p0: &const ::mono::geo::Rect) -> bool;
            // mono::geo::Rect::contains(class Point &)
            fn stub__ZNK4mono3geo4Rect8containsERNS0_5PointE(o: *mut u8, p0: &::mono::geo::Point) -> bool;
            // mono::geo::Rect::LowerLeft()
            fn stub__ZNK4mono3geo4Rect9LowerLeftEv(o: *mut u8) -> ::mono::geo::Point;
            // mono::geo::Rect::UpperLeft()
            fn stub__ZNK4mono3geo4Rect9UpperLeftEv(o: *mut u8) -> ::mono::geo::Point;
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
            pub fn new_RKS1_(p0: &const ::mono::geo::Point) -> ::mono::geo::Point {
                
            let mut o = ::mono::geo::Point { raw: Default::default() };
            unsafe { stub__ZN4mono3geo5PointC1ERKS1_(&mut o.raw[0] as *mut u8, p0); }
            o
            
            }
            pub fn new_ii(p0: i32, p1: i32) -> ::mono::geo::Point {
                
            let mut o = ::mono::geo::Point { raw: Default::default() };
            unsafe { stub__ZN4mono3geo5PointC1Eii(&mut o.raw[0] as *mut u8, p0, p1); }
            o
            
            }
            pub fn operator=(&mut self, p0: &const ::mono::geo::Point) -> &::mono::geo::Point {
                unsafe { stub__ZN4mono3geo5PointaSERKS1_(&mut self.raw[0] as *mut u8, p0) }
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
        impl Rect {
            pub fn Area(&mut self) -> i32 {
                unsafe { stub__ZN4mono3geo4Rect4AreaEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Center(&mut self) -> ::mono::geo::Point {
                unsafe { stub__ZNK4mono3geo4Rect6CenterEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn LowerLeft(&mut self) -> ::mono::geo::Point {
                unsafe { stub__ZNK4mono3geo4Rect9LowerLeftEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn LowerRight(&mut self) -> ::mono::geo::Point {
                unsafe { stub__ZNK4mono3geo4Rect10LowerRightEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Point(&mut self) -> ::mono::geo::Point {
                unsafe { stub__ZNK4mono3geo4Rect5PointEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Size(&mut self) -> &::mono::geo::Size {
                unsafe { stub__ZN4mono3geo4Rect4SizeEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn ToString(&mut self) -> ::mono::String {
                unsafe { stub__ZNK4mono3geo4Rect8ToStringEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn UpperLeft(&mut self) -> ::mono::geo::Point {
                unsafe { stub__ZNK4mono3geo4Rect9UpperLeftEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn UpperRight(&mut self) -> ::mono::geo::Point {
                unsafe { stub__ZNK4mono3geo4Rect10UpperRightEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn X2(&mut self) -> i32 {
                unsafe { stub__ZNK4mono3geo4Rect2X2Ev(&mut self.raw[0] as *mut u8) }
            }
            pub fn Y2(&mut self) -> i32 {
                unsafe { stub__ZNK4mono3geo4Rect2Y2Ev(&mut self.raw[0] as *mut u8) }
            }
            pub fn contains_KS1_(&mut self, p0: &const ::mono::geo::Rect) -> bool {
                unsafe { stub__ZNK4mono3geo4Rect8containsERKS1_(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn contains_NS0_5PointE(&mut self, p0: &::mono::geo::Point) -> bool {
                unsafe { stub__ZNK4mono3geo4Rect8containsERNS0_5PointE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn crop(&mut self, p0: &const ::mono::geo::Rect) -> ::mono::geo::Rect {
                unsafe { stub__ZNK4mono3geo4Rect4cropERKS1_(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn new() -> ::mono::geo::Rect {
                
            let mut o = ::mono::geo::Rect { raw: Default::default() };
            unsafe { stub__ZN4mono3geo4RectC1Ev(&mut o.raw[0] as *mut u8); }
            o
            
            }
            pub fn new_RKS1_(p0: &const ::mono::geo::Rect) -> ::mono::geo::Rect {
                
            let mut o = ::mono::geo::Rect { raw: Default::default() };
            unsafe { stub__ZN4mono3geo4RectC1ERKS1_(&mut o.raw[0] as *mut u8, p0); }
            o
            
            }
            pub fn new_RNS0_5PointERNS0_4SizeE(p0: &::mono::geo::Point, p1: &::mono::geo::Size) -> ::mono::geo::Rect {
                
            let mut o = ::mono::geo::Rect { raw: Default::default() };
            unsafe { stub__ZN4mono3geo4RectC1ERNS0_5PointERNS0_4SizeE(&mut o.raw[0] as *mut u8, p0, p1); }
            o
            
            }
            pub fn new_iiii(p0: i32, p1: i32, p2: i32, p3: i32) -> ::mono::geo::Rect {
                
            let mut o = ::mono::geo::Rect { raw: Default::default() };
            unsafe { stub__ZN4mono3geo4RectC1Eiiii(&mut o.raw[0] as *mut u8, p0, p1, p2, p3); }
            o
            
            }
            pub fn operator=(&mut self, p0: ::mono::geo::Rect) -> &::mono::geo::Rect {
                unsafe { stub__ZN4mono3geo4RectaSES1_(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setPoint(&mut self, p0: ::mono::geo::Point) -> () {
                unsafe { stub__ZN4mono3geo4Rect8setPointENS0_5PointE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setSize(&mut self, p0: ::mono::geo::Size) -> () {
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
            pub fn new() -> ::mono::geo::Size {
                
            let mut o = ::mono::geo::Size { raw: Default::default() };
            unsafe { stub__ZN4mono3geo4SizeC1Ev(&mut o.raw[0] as *mut u8); }
            o
            
            }
            pub fn new_RKS1_(p0: &const ::mono::geo::Size) -> ::mono::geo::Size {
                
            let mut o = ::mono::geo::Size { raw: Default::default() };
            unsafe { stub__ZN4mono3geo4SizeC1ERKS1_(&mut o.raw[0] as *mut u8, p0); }
            o
            
            }
            pub fn new_ii(p0: i32, p1: i32) -> ::mono::geo::Size {
                
            let mut o = ::mono::geo::Size { raw: Default::default() };
            unsafe { stub__ZN4mono3geo4SizeC1Eii(&mut o.raw[0] as *mut u8, p0, p1); }
            o
            
            }
            pub fn operator=(&mut self, p0: &const ::mono::geo::Size) -> &::mono::geo::Size {
                unsafe { stub__ZN4mono3geo4SizeaSERKS1_(&mut self.raw[0] as *mut u8, p0) }
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
        pub struct DigitalOut {
            raw: [u8; 1],
        }
        pub struct ISettings {
            raw: [u8; 8],
        }
        pub struct Serial {
            raw: [u8; 8],
        }
        extern "C" {
            // mono::io::DigitalOut::setMode(int)
            fn stub__ZN4mono2io10DigitalOut7setModeEi(o: *mut u8, p0: i32) -> ();
            // mono::io::DigitalOut::DigitalOut(int)
            fn stub__ZN4mono2io10DigitalOutC1Ei(o: *mut u8, p0: i32) -> ();
            // mono::io::Serial::DTR()
            fn stub__ZN4mono2io6Serial3DTREv(o: *mut u8) -> bool;
            // mono::io::Serial::IsReady()
            fn stub__ZN4mono2io6Serial7IsReadyEv(o: *mut u8) -> bool;
            // mono::io::Serial::Serial()
            fn stub__ZN4mono2io6SerialC1Ev(o: *mut u8) -> ();
            // mono::io::ISettings::read(mono::io::ISettings::SettingLocations, mono::io::ISettings::SettingSizes, void *)
            fn stub__ZN4mono2io9ISettings4readENS1_16SettingLocationsENS1_12SettingSizesEPv(o: *mut u8, p0: i32, p1: i32, p2: *()) -> i32;
            // mono::io::ISettings::write(mono::io::ISettings::SettingLocations, mono::io::ISettings::SettingSizes, const void *)
            fn stub__ZN4mono2io9ISettings5writeENS1_16SettingLocationsENS1_12SettingSizesEPKv(o: *mut u8, p0: i32, p1: i32, p2: *const ()) -> i32;
            // mono::io::ISettings::size()
            fn stub__ZNK4mono2io9ISettings4sizeEv(o: *mut u8) -> i32;
        }
        impl DigitalOut {
            pub fn new(p0: i32) -> ::mono::io::DigitalOut {
                
            let mut o = ::mono::io::DigitalOut { raw: Default::default() };
            unsafe { stub__ZN4mono2io10DigitalOutC1Ei(&mut o.raw[0] as *mut u8, p0); }
            o
            
            }
            pub fn setMode(&mut self, p0: i32) -> () {
                unsafe { stub__ZN4mono2io10DigitalOut7setModeEi(&mut self.raw[0] as *mut u8, p0) }
            }
        }
        impl ISettings {
            pub fn read(&mut self, p0: i32, p1: i32, p2: *()) -> i32 {
                unsafe { stub__ZN4mono2io9ISettings4readENS1_16SettingLocationsENS1_12SettingSizesEPv(&mut self.raw[0] as *mut u8, p0, p1, p2) }
            }
            pub fn size(&mut self) -> i32 {
                unsafe { stub__ZNK4mono2io9ISettings4sizeEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn write(&mut self, p0: i32, p1: i32, p2: *const ()) -> i32 {
                unsafe { stub__ZN4mono2io9ISettings5writeENS1_16SettingLocationsENS1_12SettingSizesEPKv(&mut self.raw[0] as *mut u8, p0, p1, p2) }
            }
        }
        impl Serial {
            pub fn DTR(&mut self) -> bool {
                unsafe { stub__ZN4mono2io6Serial3DTREv(&mut self.raw[0] as *mut u8) }
            }
            pub fn IsReady(&mut self) -> bool {
                unsafe { stub__ZN4mono2io6Serial7IsReadyEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn new() -> ::mono::io::Serial {
                
            let mut o = ::mono::io::Serial { raw: Default::default() };
            unsafe { stub__ZN4mono2io6SerialC1Ev(&mut o.raw[0] as *mut u8); }
            o
            
            }
        }
    }
    pub mod network {
        pub struct CompletionEvent {
            raw: [u8; 8],
        }
        pub struct DnsResolver {
            raw: [u8; 1],
        }
        pub struct ErrorEvent {
            raw: [u8; 16],
        }
        pub struct HttpClient {
            raw: [u8; 1],
        }
        pub struct HttpPostClient {
            raw: [u8; 1],
        }
        pub struct HttpResponseData {
            raw: [u8; 1],
        }
        pub struct INetworkRequest {
            raw: [u8; 24],
        }
        pub struct StateChangeEvent {
            raw: [u8; 16],
        }
        extern "C" {
            // mono::network::HttpClient::HttpResponseData::HttpResponseData(mono::network::HttpClient *)
            fn stub__ZN4mono7network10HttpClient16HttpResponseDataC1EPS1_(o: *mut u8, p0: *::mono::network::HttpClient) -> ();
            // mono::network::HttpClient::HttpResponseData::operator=(const mono::network::HttpClient::HttpResponseData &)
            fn stub__ZN4mono7network10HttpClient16HttpResponseDataaSERKS2_(o: *mut u8, p0: &const ::mono::network::HttpClient::HttpResponseData) -> &::mono::network::HttpClient::HttpResponseData;
            // mono::network::HttpClient::HttpClient(const mono::network::HttpClient &)
            fn stub__ZN4mono7network10HttpClientC1ERKS1_(o: *mut u8, p0: &const ::mono::network::HttpClient) -> ();
            // mono::network::HttpClient::HttpClient()
            fn stub__ZN4mono7network10HttpClientC1Ev(o: *mut u8) -> ();
            // mono::network::HttpClient::~HttpClient()
            fn stub__ZN4mono7network10HttpClientD0Ev(o: *mut u8) -> ();
            // mono::network::HttpClient::operator=(const mono::network::HttpClient &)
            fn stub__ZN4mono7network10HttpClientaSERKS1_(o: *mut u8, p0: &const ::mono::network::HttpClient) -> &::mono::network::HttpClient;
            // mono::network::DnsResolver::DnsResolver(const mono::network::DnsResolver &)
            fn stub__ZN4mono7network11DnsResolverC1ERKS1_(o: *mut u8, p0: &const ::mono::network::DnsResolver) -> ();
            // mono::network::DnsResolver::DnsResolver()
            fn stub__ZN4mono7network11DnsResolverC1Ev(o: *mut u8) -> ();
            // mono::network::DnsResolver::~DnsResolver()
            fn stub__ZN4mono7network11DnsResolverD0Ev(o: *mut u8) -> ();
            // mono::network::DnsResolver::operator=(const mono::network::DnsResolver &)
            fn stub__ZN4mono7network11DnsResolveraSERKS1_(o: *mut u8, p0: &const ::mono::network::DnsResolver) -> &::mono::network::DnsResolver;
            // mono::network::HttpPostClient::post()
            fn stub__ZN4mono7network14HttpPostClient4postEv(o: *mut u8) -> ();
            // mono::network::HttpPostClient::HttpPostClient(const mono::network::HttpPostClient &)
            fn stub__ZN4mono7network14HttpPostClientC1ERKS1_(o: *mut u8, p0: &const ::mono::network::HttpPostClient) -> ();
            // mono::network::HttpPostClient::HttpPostClient()
            fn stub__ZN4mono7network14HttpPostClientC1Ev(o: *mut u8) -> ();
            // mono::network::HttpPostClient::operator=(const mono::network::HttpPostClient &)
            fn stub__ZN4mono7network14HttpPostClientaSERKS1_(o: *mut u8, p0: &const ::mono::network::HttpPostClient) -> &::mono::network::HttpPostClient;
            // mono::network::INetworkRequest::~INetworkRequest()
            fn stub__ZN4mono7network15INetworkRequestD0Ev(o: *mut u8) -> ();
            // mono::network::DnsResolver::DomainName()
            fn stub__ZNK4mono7network11DnsResolver10DomainNameEv(o: *mut u8) -> i32;
            // mono::network::DnsResolver::IpAddress()
            fn stub__ZNK4mono7network11DnsResolver9IpAddressEv(o: *mut u8) -> i32;
            // mono::network::DnsResolver::IpVersion()
            fn stub__ZNK4mono7network11DnsResolver9IpVersionEv(o: *mut u8) -> i32;
            // mono::network::INetworkRequest::IsCompleted()
            fn stub__ZNK4mono7network15INetworkRequest11IsCompletedEv(o: *mut u8) -> bool;
            // mono::network::INetworkRequest::State()
            fn stub__ZNK4mono7network15INetworkRequest5StateEv(o: *mut u8) -> i32;
            // mono::network::INetworkRequest::HasFailed()
            fn stub__ZNK4mono7network15INetworkRequest9HasFailedEv(o: *mut u8) -> bool;
        }
        impl DnsResolver {
            pub fn DomainName(&mut self) -> i32 {
                unsafe { stub__ZNK4mono7network11DnsResolver10DomainNameEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn IpAddress(&mut self) -> i32 {
                unsafe { stub__ZNK4mono7network11DnsResolver9IpAddressEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn IpVersion(&mut self) -> i32 {
                unsafe { stub__ZNK4mono7network11DnsResolver9IpVersionEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn new() -> ::mono::network::DnsResolver {
                
            let mut o = ::mono::network::DnsResolver { raw: Default::default() };
            unsafe { stub__ZN4mono7network11DnsResolverC1Ev(&mut o.raw[0] as *mut u8); }
            o
            
            }
            pub fn new_RKS1_(p0: &const ::mono::network::DnsResolver) -> ::mono::network::DnsResolver {
                
            let mut o = ::mono::network::DnsResolver { raw: Default::default() };
            unsafe { stub__ZN4mono7network11DnsResolverC1ERKS1_(&mut o.raw[0] as *mut u8, p0); }
            o
            
            }
            pub fn operator=(&mut self, p0: &const ::mono::network::DnsResolver) -> &::mono::network::DnsResolver {
                unsafe { stub__ZN4mono7network11DnsResolveraSERKS1_(&mut self.raw[0] as *mut u8, p0) }
            }
        }
        impl Drop for DnsResolver {
             fn drop(&mut self) -> () {
                unsafe { stub__ZN4mono7network11DnsResolverD0Ev(&mut self.raw[0] as *mut u8) }
            }
        }
        impl Drop for HttpClient {
             fn drop(&mut self) -> () {
                unsafe { stub__ZN4mono7network10HttpClientD0Ev(&mut self.raw[0] as *mut u8) }
            }
        }
        impl Drop for INetworkRequest {
             fn drop(&mut self) -> () {
                unsafe { stub__ZN4mono7network15INetworkRequestD0Ev(&mut self.raw[0] as *mut u8) }
            }
        }
        impl HttpClient {
            pub fn new() -> ::mono::network::HttpClient {
                
            let mut o = ::mono::network::HttpClient { raw: Default::default() };
            unsafe { stub__ZN4mono7network10HttpClientC1Ev(&mut o.raw[0] as *mut u8); }
            o
            
            }
            pub fn new_RKS1_(p0: &const ::mono::network::HttpClient) -> ::mono::network::HttpClient {
                
            let mut o = ::mono::network::HttpClient { raw: Default::default() };
            unsafe { stub__ZN4mono7network10HttpClientC1ERKS1_(&mut o.raw[0] as *mut u8, p0); }
            o
            
            }
            pub fn operator=(&mut self, p0: &const ::mono::network::HttpClient) -> &::mono::network::HttpClient {
                unsafe { stub__ZN4mono7network10HttpClientaSERKS1_(&mut self.raw[0] as *mut u8, p0) }
            }
        }
        impl HttpPostClient {
            pub fn new() -> ::mono::network::HttpPostClient {
                
            let mut o = ::mono::network::HttpPostClient { raw: Default::default() };
            unsafe { stub__ZN4mono7network14HttpPostClientC1Ev(&mut o.raw[0] as *mut u8); }
            o
            
            }
            pub fn new_RKS1_(p0: &const ::mono::network::HttpPostClient) -> ::mono::network::HttpPostClient {
                
            let mut o = ::mono::network::HttpPostClient { raw: Default::default() };
            unsafe { stub__ZN4mono7network14HttpPostClientC1ERKS1_(&mut o.raw[0] as *mut u8, p0); }
            o
            
            }
            pub fn operator=(&mut self, p0: &const ::mono::network::HttpPostClient) -> &::mono::network::HttpPostClient {
                unsafe { stub__ZN4mono7network14HttpPostClientaSERKS1_(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn post(&mut self) -> () {
                unsafe { stub__ZN4mono7network14HttpPostClient4postEv(&mut self.raw[0] as *mut u8) }
            }
        }
        impl HttpResponseData {
            pub fn new(p0: *::mono::network::HttpClient) -> ::mono::network::HttpClient::HttpResponseData {
                
            let mut o = ::mono::network::HttpClient::HttpResponseData { raw: Default::default() };
            unsafe { stub__ZN4mono7network10HttpClient16HttpResponseDataC1EPS1_(&mut o.raw[0] as *mut u8, p0); }
            o
            
            }
            pub fn operator=(&mut self, p0: &const ::mono::network::HttpClient::HttpResponseData) -> &::mono::network::HttpClient::HttpResponseData {
                unsafe { stub__ZN4mono7network10HttpClient16HttpResponseDataaSERKS2_(&mut self.raw[0] as *mut u8, p0) }
            }
        }
        impl INetworkRequest {
            pub fn HasFailed(&mut self) -> bool {
                unsafe { stub__ZNK4mono7network15INetworkRequest9HasFailedEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn IsCompleted(&mut self) -> bool {
                unsafe { stub__ZNK4mono7network15INetworkRequest11IsCompletedEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn State(&mut self) -> i32 {
                unsafe { stub__ZNK4mono7network15INetworkRequest5StateEv(&mut self.raw[0] as *mut u8) }
            }
        }
    }
    pub mod power {
        pub struct ACT8600PowerSystem {
            raw: [u8; 1],
        }
        pub struct IPowerAware {
            raw: [u8; 24],
        }
        pub struct IPowerFencedPeripheral {
            raw: [u8; 24],
        }
        pub struct IPowerManagement {
            raw: [u8; 24],
        }
        pub struct IPowerSubSystem {
            raw: [u8; 1],
        }
        pub struct MonoPowerManagement {
            raw: [u8; 1],
        }
        extern "C" {
            // mono::power::IPowerAware::OnSystemBatteryLow()
            fn stub__ZN4mono5power11IPowerAware18OnSystemBatteryLowEv(o: *mut u8) -> ();
            // mono::power::IPowerAware::onSystemEnterSleep()
            fn stub__ZN4mono5power11IPowerAware18onSystemEnterSleepEv(o: *mut u8) -> ();
            // mono::power::IPowerAware::onSystemPowerOnReset()
            fn stub__ZN4mono5power11IPowerAware20onSystemPowerOnResetEv(o: *mut u8) -> ();
            // mono::power::IPowerAware::onSystemWakeFromSleep()
            fn stub__ZN4mono5power11IPowerAware21onSystemWakeFromSleepEv(o: *mut u8) -> ();
            // mono::power::IPowerSubSystem::ChargeStatus()
            fn stub__ZN4mono5power15IPowerSubSystem12ChargeStatusEv(o: *mut u8) -> i32;
            // mono::power::IPowerSubSystem::IsPowerFenced()
            fn stub__ZN4mono5power15IPowerSubSystem13IsPowerFencedEv(o: *mut u8) -> bool;
            // mono::power::IPowerSubSystem::IsUSBCharging()
            fn stub__ZN4mono5power15IPowerSubSystem13IsUSBChargingEv(o: *mut u8) -> bool;
            // mono::power::IPowerSubSystem::setPowerFence(bool)
            fn stub__ZN4mono5power15IPowerSubSystem13setPowerFenceEb(o: *mut u8, p0: bool) -> ();
            // mono::power::IPowerSubSystem::onSystemEnterSleep()
            fn stub__ZN4mono5power15IPowerSubSystem18onSystemEnterSleepEv(o: *mut u8) -> ();
            // mono::power::IPowerSubSystem::onSystemPowerOnReset()
            fn stub__ZN4mono5power15IPowerSubSystem20onSystemPowerOnResetEv(o: *mut u8) -> ();
            // mono::power::IPowerSubSystem::onSystemWakeFromSleep()
            fn stub__ZN4mono5power15IPowerSubSystem21onSystemWakeFromSleepEv(o: *mut u8) -> ();
            // mono::power::IPowerSubSystem::IsPowerOk()
            fn stub__ZN4mono5power15IPowerSubSystem9IsPowerOkEv(o: *mut u8) -> bool;
            // mono::power::IPowerManagement::EnterSleep()
            fn stub__ZN4mono5power16IPowerManagement10EnterSleepEv(o: *mut u8) -> ();
            // mono::power::IPowerManagement::AppendToPowerAwareQueue(mono::power::IPowerAware *)
            fn stub__ZN4mono5power16IPowerManagement23AppendToPowerAwareQueueEPNS0_11IPowerAwareE(o: *mut u8, p0: *::mono::power::IPowerAware) -> ();
            // mono::power::IPowerManagement::RemoveFromPowerAwareQueue(mono::power::IPowerAware *)
            fn stub__ZN4mono5power16IPowerManagement25RemoveFromPowerAwareQueueEPNS0_11IPowerAwareE(o: *mut u8, p0: *::mono::power::IPowerAware) -> bool;
            // mono::power::ACT8600PowerSystem::USBOTGPower()
            fn stub__ZN4mono5power18ACT8600PowerSystem11USBOTGPowerEv(o: *mut u8) -> bool;
            // mono::power::ACT8600PowerSystem::ChargeStatus()
            fn stub__ZN4mono5power18ACT8600PowerSystem12ChargeStatusEv(o: *mut u8) -> i32;
            // mono::power::ACT8600PowerSystem::SystemStatus()
            fn stub__ZN4mono5power18ACT8600PowerSystem12SystemStatusEv(o: *mut u8) -> i32;
            // mono::power::ACT8600PowerSystem::IsPowerFenced()
            fn stub__ZN4mono5power18ACT8600PowerSystem13IsPowerFencedEv(o: *mut u8) -> bool;
            // mono::power::ACT8600PowerSystem::setPowerFence(bool)
            fn stub__ZN4mono5power18ACT8600PowerSystem13setPowerFenceEb(o: *mut u8, p0: bool) -> ();
            // mono::power::ACT8600PowerSystem::powerOffUnused()
            fn stub__ZN4mono5power18ACT8600PowerSystem14powerOffUnusedEv(o: *mut u8) -> ();
            // mono::power::ACT8600PowerSystem::setUSBOTGPower(bool)
            fn stub__ZN4mono5power18ACT8600PowerSystem14setUSBOTGPowerEb(o: *mut u8, p0: bool) -> bool;
            // mono::power::ACT8600PowerSystem::onSystemEnterSleep()
            fn stub__ZN4mono5power18ACT8600PowerSystem18onSystemEnterSleepEv(o: *mut u8) -> ();
            // mono::power::ACT8600PowerSystem::onSystemPowerOnReset()
            fn stub__ZN4mono5power18ACT8600PowerSystem20onSystemPowerOnResetEv(o: *mut u8) -> ();
            // mono::power::ACT8600PowerSystem::onSystemWakeFromSleep()
            fn stub__ZN4mono5power18ACT8600PowerSystem21onSystemWakeFromSleepEv(o: *mut u8) -> ();
            // mono::power::ACT8600PowerSystem::SystemVoltageThreshold()
            fn stub__ZN4mono5power18ACT8600PowerSystem22SystemVoltageThresholdEv(o: *mut u8) -> i32;
            // mono::power::ACT8600PowerSystem::setSystemMonitorInterrupt(bool)
            fn stub__ZN4mono5power18ACT8600PowerSystem25setSystemMonitorInterruptEb(o: *mut u8, p0: bool) -> ();
            // mono::power::ACT8600PowerSystem::setSystemVoltageThreshold(mono::power::ACT8600PowerSystem::SystemVoltageLevels)
            fn stub__ZN4mono5power18ACT8600PowerSystem25setSystemVoltageThresholdENS1_19SystemVoltageLevelsE(o: *mut u8, p0: i32) -> ();
            // mono::power::ACT8600PowerSystem::USBOTG()
            fn stub__ZN4mono5power18ACT8600PowerSystem6USBOTGEv(o: *mut u8) -> i32;
            // mono::power::ACT8600PowerSystem::IsPowerOk()
            fn stub__ZN4mono5power18ACT8600PowerSystem9IsPowerOkEv(o: *mut u8) -> bool;
            // mono::power::ACT8600PowerSystem::powerReg8(bool)
            fn stub__ZN4mono5power18ACT8600PowerSystem9powerReg8Eb(o: *mut u8, p0: bool) -> ();
            // mono::power::ACT8600PowerSystem::ACT8600PowerSystem()
            fn stub__ZN4mono5power18ACT8600PowerSystemC1Ev(o: *mut u8) -> ();
            // mono::power::MonoPowerManagement::EnterSleep(bool)
            fn stub__ZN4mono5power19MonoPowerManagement10EnterSleepEb(o: *mut u8, p0: bool) -> ();
            // mono::power::MonoPowerManagement::EnterSleep()
            fn stub__ZN4mono5power19MonoPowerManagement10EnterSleepEv(o: *mut u8) -> ();
            // mono::power::MonoPowerManagement::MonoPowerManagement()
            fn stub__ZN4mono5power19MonoPowerManagementC1Ev(o: *mut u8) -> ();
            // mono::power::IPowerFencedPeripheral::PowerCameUp()
            fn stub__ZN4mono5power22IPowerFencedPeripheral11PowerCameUpEv(o: *mut u8) -> ();
            // mono::power::IPowerFencedPeripheral::ReleasePower()
            fn stub__ZN4mono5power22IPowerFencedPeripheral12ReleasePowerEv(o: *mut u8) -> bool;
            // mono::power::IPowerFencedPeripheral::RequestPower()
            fn stub__ZN4mono5power22IPowerFencedPeripheral12RequestPowerEv(o: *mut u8) -> bool;
            // mono::power::IPowerFencedPeripheral::PowerGoesDown()
            fn stub__ZN4mono5power22IPowerFencedPeripheral13PowerGoesDownEv(o: *mut u8) -> ();
            // mono::power::IPowerFencedPeripheral::ForceTogglePower()
            fn stub__ZN4mono5power22IPowerFencedPeripheral16ForceTogglePowerEv(o: *mut u8) -> bool;
            // mono::power::IPowerFencedPeripheral::IPowerFencedPeripheral()
            fn stub__ZN4mono5power22IPowerFencedPeripheralC1Ev(o: *mut u8) -> ();
            // mono::power::IPowerFencedPeripheral::HasPower()
            fn stub__ZNK4mono5power22IPowerFencedPeripheral8HasPowerEv(o: *mut u8) -> bool;
        }
        impl ACT8600PowerSystem {
            pub fn ChargeStatus(&mut self) -> i32 {
                unsafe { stub__ZN4mono5power18ACT8600PowerSystem12ChargeStatusEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn IsPowerFenced(&mut self) -> bool {
                unsafe { stub__ZN4mono5power18ACT8600PowerSystem13IsPowerFencedEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn IsPowerOk(&mut self) -> bool {
                unsafe { stub__ZN4mono5power18ACT8600PowerSystem9IsPowerOkEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn SystemStatus(&mut self) -> i32 {
                unsafe { stub__ZN4mono5power18ACT8600PowerSystem12SystemStatusEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn SystemVoltageThreshold(&mut self) -> i32 {
                unsafe { stub__ZN4mono5power18ACT8600PowerSystem22SystemVoltageThresholdEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn USBOTG(&mut self) -> i32 {
                unsafe { stub__ZN4mono5power18ACT8600PowerSystem6USBOTGEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn USBOTGPower(&mut self) -> bool {
                unsafe { stub__ZN4mono5power18ACT8600PowerSystem11USBOTGPowerEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn new() -> ::mono::power::ACT8600PowerSystem {
                
            let mut o = ::mono::power::ACT8600PowerSystem { raw: Default::default() };
            unsafe { stub__ZN4mono5power18ACT8600PowerSystemC1Ev(&mut o.raw[0] as *mut u8); }
            o
            
            }
            pub fn onSystemEnterSleep(&mut self) -> () {
                unsafe { stub__ZN4mono5power18ACT8600PowerSystem18onSystemEnterSleepEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn onSystemPowerOnReset(&mut self) -> () {
                unsafe { stub__ZN4mono5power18ACT8600PowerSystem20onSystemPowerOnResetEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn onSystemWakeFromSleep(&mut self) -> () {
                unsafe { stub__ZN4mono5power18ACT8600PowerSystem21onSystemWakeFromSleepEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn powerOffUnused(&mut self) -> () {
                unsafe { stub__ZN4mono5power18ACT8600PowerSystem14powerOffUnusedEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn powerReg8(&mut self, p0: bool) -> () {
                unsafe { stub__ZN4mono5power18ACT8600PowerSystem9powerReg8Eb(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setPowerFence(&mut self, p0: bool) -> () {
                unsafe { stub__ZN4mono5power18ACT8600PowerSystem13setPowerFenceEb(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setSystemMonitorInterrupt(&mut self, p0: bool) -> () {
                unsafe { stub__ZN4mono5power18ACT8600PowerSystem25setSystemMonitorInterruptEb(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setSystemVoltageThreshold(&mut self, p0: i32) -> () {
                unsafe { stub__ZN4mono5power18ACT8600PowerSystem25setSystemVoltageThresholdENS1_19SystemVoltageLevelsE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn setUSBOTGPower(&mut self, p0: bool) -> bool {
                unsafe { stub__ZN4mono5power18ACT8600PowerSystem14setUSBOTGPowerEb(&mut self.raw[0] as *mut u8, p0) }
            }
        }
        impl IPowerAware {
            pub fn OnSystemBatteryLow(&mut self) -> () {
                unsafe { stub__ZN4mono5power11IPowerAware18OnSystemBatteryLowEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn onSystemEnterSleep(&mut self) -> () {
                unsafe { stub__ZN4mono5power11IPowerAware18onSystemEnterSleepEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn onSystemPowerOnReset(&mut self) -> () {
                unsafe { stub__ZN4mono5power11IPowerAware20onSystemPowerOnResetEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn onSystemWakeFromSleep(&mut self) -> () {
                unsafe { stub__ZN4mono5power11IPowerAware21onSystemWakeFromSleepEv(&mut self.raw[0] as *mut u8) }
            }
        }
        impl IPowerFencedPeripheral {
            pub fn ForceTogglePower(&mut self) -> bool {
                unsafe { stub__ZN4mono5power22IPowerFencedPeripheral16ForceTogglePowerEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn HasPower(&mut self) -> bool {
                unsafe { stub__ZNK4mono5power22IPowerFencedPeripheral8HasPowerEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn PowerCameUp(&mut self) -> () {
                unsafe { stub__ZN4mono5power22IPowerFencedPeripheral11PowerCameUpEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn PowerGoesDown(&mut self) -> () {
                unsafe { stub__ZN4mono5power22IPowerFencedPeripheral13PowerGoesDownEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn ReleasePower(&mut self) -> bool {
                unsafe { stub__ZN4mono5power22IPowerFencedPeripheral12ReleasePowerEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn RequestPower(&mut self) -> bool {
                unsafe { stub__ZN4mono5power22IPowerFencedPeripheral12RequestPowerEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn new() -> ::mono::power::IPowerFencedPeripheral {
                
            let mut o = ::mono::power::IPowerFencedPeripheral { raw: Default::default() };
            unsafe { stub__ZN4mono5power22IPowerFencedPeripheralC1Ev(&mut o.raw[0] as *mut u8); }
            o
            
            }
        }
        impl IPowerManagement {
            pub fn AppendToPowerAwareQueue(&mut self, p0: *::mono::power::IPowerAware) -> () {
                unsafe { stub__ZN4mono5power16IPowerManagement23AppendToPowerAwareQueueEPNS0_11IPowerAwareE(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn EnterSleep(&mut self) -> () {
                unsafe { stub__ZN4mono5power16IPowerManagement10EnterSleepEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn RemoveFromPowerAwareQueue(&mut self, p0: *::mono::power::IPowerAware) -> bool {
                unsafe { stub__ZN4mono5power16IPowerManagement25RemoveFromPowerAwareQueueEPNS0_11IPowerAwareE(&mut self.raw[0] as *mut u8, p0) }
            }
        }
        impl IPowerSubSystem {
            pub fn ChargeStatus(&mut self) -> i32 {
                unsafe { stub__ZN4mono5power15IPowerSubSystem12ChargeStatusEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn IsPowerFenced(&mut self) -> bool {
                unsafe { stub__ZN4mono5power15IPowerSubSystem13IsPowerFencedEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn IsPowerOk(&mut self) -> bool {
                unsafe { stub__ZN4mono5power15IPowerSubSystem9IsPowerOkEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn IsUSBCharging(&mut self) -> bool {
                unsafe { stub__ZN4mono5power15IPowerSubSystem13IsUSBChargingEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn onSystemEnterSleep(&mut self) -> () {
                unsafe { stub__ZN4mono5power15IPowerSubSystem18onSystemEnterSleepEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn onSystemPowerOnReset(&mut self) -> () {
                unsafe { stub__ZN4mono5power15IPowerSubSystem20onSystemPowerOnResetEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn onSystemWakeFromSleep(&mut self) -> () {
                unsafe { stub__ZN4mono5power15IPowerSubSystem21onSystemWakeFromSleepEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn setPowerFence(&mut self, p0: bool) -> () {
                unsafe { stub__ZN4mono5power15IPowerSubSystem13setPowerFenceEb(&mut self.raw[0] as *mut u8, p0) }
            }
        }
        impl MonoPowerManagement {
            pub fn EnterSleep(&mut self) -> () {
                unsafe { stub__ZN4mono5power19MonoPowerManagement10EnterSleepEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn EnterSleep_b(&mut self, p0: bool) -> () {
                unsafe { stub__ZN4mono5power19MonoPowerManagement10EnterSleepEb(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn new() -> ::mono::power::MonoPowerManagement {
                
            let mut o = ::mono::power::MonoPowerManagement { raw: Default::default() };
            unsafe { stub__ZN4mono5power19MonoPowerManagementC1Ev(&mut o.raw[0] as *mut u8); }
            o
            
            }
        }
    }
    pub mod sensor {
        pub struct AT30TS74Temperature {
            raw: [u8; 16],
        }
        pub struct IAccelerometer {
            raw: [u8; 8],
        }
        pub struct IBuzzer {
            raw: [u8; 8],
        }
        pub struct ITemperature {
            raw: [u8; 8],
        }
        pub struct MMAAccelerometer {
            raw: [u8; 8],
        }
        pub struct MonoBuzzer {
            raw: [u8; 8],
        }
        pub struct PCT2075Temperature {
            raw: [u8; 1],
        }
        extern "C" {
            // mono::sensor::MonoBuzzer::buzzKill()
            fn stub__ZN4mono6sensor10MonoBuzzer8buzzKillEv(o: *mut u8) -> ();
            // mono::sensor::MonoBuzzer::buzzAsync(uint32_t)
            fn stub__ZN4mono6sensor10MonoBuzzer9buzzAsyncEj(o: *mut u8, p0: u32) -> ();
            // mono::sensor::MonoBuzzer::MonoBuzzer()
            fn stub__ZN4mono6sensor10MonoBuzzerC1Ev(o: *mut u8) -> ();
            // mono::sensor::ITemperature::ReadMilliCelcius()
            fn stub__ZN4mono6sensor12ITemperature16ReadMilliCelciusEv(o: *mut u8) -> i32;
            // mono::sensor::ITemperature::Read()
            fn stub__ZN4mono6sensor12ITemperature4ReadEv(o: *mut u8) -> i32;
            // mono::sensor::IAccelerometer::Stop()
            fn stub__ZN4mono6sensor14IAccelerometer4StopEv(o: *mut u8) -> ();
            // mono::sensor::IAccelerometer::Start()
            fn stub__ZN4mono6sensor14IAccelerometer5StartEv(o: *mut u8) -> ();
            // mono::sensor::IAccelerometer::IsActive()
            fn stub__ZN4mono6sensor14IAccelerometer8IsActiveEv(o: *mut u8) -> bool;
            // mono::sensor::IAccelerometer::rawXAxis()
            fn stub__ZN4mono6sensor14IAccelerometer8rawXAxisEv(o: *mut u8) -> i16;
            // mono::sensor::MMAAccelerometer::Stop()
            fn stub__ZN4mono6sensor16MMAAccelerometer4StopEv(o: *mut u8) -> ();
            // mono::sensor::MMAAccelerometer::Start()
            fn stub__ZN4mono6sensor16MMAAccelerometer5StartEv(o: *mut u8) -> ();
            // mono::sensor::MMAAccelerometer::IsActive()
            fn stub__ZN4mono6sensor16MMAAccelerometer8IsActiveEv(o: *mut u8) -> bool;
            // mono::sensor::MMAAccelerometer::rawXAxis()
            fn stub__ZN4mono6sensor16MMAAccelerometer8rawXAxisEv(o: *mut u8) -> i16;
            // mono::sensor::MMAAccelerometer::MMAAccelerometer()
            fn stub__ZN4mono6sensor16MMAAccelerometerC1Ev(o: *mut u8) -> ();
            // mono::sensor::PCT2075Temperature::Read()
            fn stub__ZN4mono6sensor18PCT2075Temperature4ReadEv(o: *mut u8) -> i32;
            // mono::sensor::PCT2075Temperature::PCT2075Temperature()
            fn stub__ZN4mono6sensor18PCT2075TemperatureC1Ev(o: *mut u8) -> ();
            // mono::sensor::AT30TS74Temperature::ReadMilliCelcius()
            fn stub__ZN4mono6sensor19AT30TS74Temperature16ReadMilliCelciusEv(o: *mut u8) -> i32;
            // mono::sensor::AT30TS74Temperature::Read()
            fn stub__ZN4mono6sensor19AT30TS74Temperature4ReadEv(o: *mut u8) -> i32;
            // mono::sensor::AT30TS74Temperature::AT30TS74Temperature()
            fn stub__ZN4mono6sensor19AT30TS74TemperatureC1Ev(o: *mut u8) -> ();
            // mono::sensor::IBuzzer::buzzKill()
            fn stub__ZN4mono6sensor7IBuzzer8buzzKillEv(o: *mut u8) -> ();
            // mono::sensor::IBuzzer::buzzAsync(uint32_t)
            fn stub__ZN4mono6sensor7IBuzzer9buzzAsyncEj(o: *mut u8, p0: u32) -> ();
            // mono::sensor::IBuzzer::buzzAsync(uint32_t, void (*)())
            fn stub__ZN4mono6sensor7IBuzzer9buzzAsyncEjPFvvE(o: *mut u8, p0: u32, p1: *::) -> ();
        }
        impl AT30TS74Temperature {
            pub fn Read(&mut self) -> i32 {
                unsafe { stub__ZN4mono6sensor19AT30TS74Temperature4ReadEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn ReadMilliCelcius(&mut self) -> i32 {
                unsafe { stub__ZN4mono6sensor19AT30TS74Temperature16ReadMilliCelciusEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn new() -> ::mono::sensor::AT30TS74Temperature {
                
            let mut o = ::mono::sensor::AT30TS74Temperature { raw: Default::default() };
            unsafe { stub__ZN4mono6sensor19AT30TS74TemperatureC1Ev(&mut o.raw[0] as *mut u8); }
            o
            
            }
        }
        impl IAccelerometer {
            pub fn IsActive(&mut self) -> bool {
                unsafe { stub__ZN4mono6sensor14IAccelerometer8IsActiveEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Start(&mut self) -> () {
                unsafe { stub__ZN4mono6sensor14IAccelerometer5StartEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Stop(&mut self) -> () {
                unsafe { stub__ZN4mono6sensor14IAccelerometer4StopEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn rawXAxis(&mut self) -> i16 {
                unsafe { stub__ZN4mono6sensor14IAccelerometer8rawXAxisEv(&mut self.raw[0] as *mut u8) }
            }
        }
        impl IBuzzer {
            pub fn buzzAsync_(&mut self, p0: u32) -> () {
                unsafe { stub__ZN4mono6sensor7IBuzzer9buzzAsyncEj(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn buzzAsync_PFvvE(&mut self, p0: u32, p1: *::) -> () {
                unsafe { stub__ZN4mono6sensor7IBuzzer9buzzAsyncEjPFvvE(&mut self.raw[0] as *mut u8, p0, p1) }
            }
            pub fn buzzKill(&mut self) -> () {
                unsafe { stub__ZN4mono6sensor7IBuzzer8buzzKillEv(&mut self.raw[0] as *mut u8) }
            }
        }
        impl ITemperature {
            pub fn Read(&mut self) -> i32 {
                unsafe { stub__ZN4mono6sensor12ITemperature4ReadEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn ReadMilliCelcius(&mut self) -> i32 {
                unsafe { stub__ZN4mono6sensor12ITemperature16ReadMilliCelciusEv(&mut self.raw[0] as *mut u8) }
            }
        }
        impl MMAAccelerometer {
            pub fn IsActive(&mut self) -> bool {
                unsafe { stub__ZN4mono6sensor16MMAAccelerometer8IsActiveEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Start(&mut self) -> () {
                unsafe { stub__ZN4mono6sensor16MMAAccelerometer5StartEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn Stop(&mut self) -> () {
                unsafe { stub__ZN4mono6sensor16MMAAccelerometer4StopEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn new() -> ::mono::sensor::MMAAccelerometer {
                
            let mut o = ::mono::sensor::MMAAccelerometer { raw: Default::default() };
            unsafe { stub__ZN4mono6sensor16MMAAccelerometerC1Ev(&mut o.raw[0] as *mut u8); }
            o
            
            }
            pub fn rawXAxis(&mut self) -> i16 {
                unsafe { stub__ZN4mono6sensor16MMAAccelerometer8rawXAxisEv(&mut self.raw[0] as *mut u8) }
            }
        }
        impl MonoBuzzer {
            pub fn buzzAsync(&mut self, p0: u32) -> () {
                unsafe { stub__ZN4mono6sensor10MonoBuzzer9buzzAsyncEj(&mut self.raw[0] as *mut u8, p0) }
            }
            pub fn buzzKill(&mut self) -> () {
                unsafe { stub__ZN4mono6sensor10MonoBuzzer8buzzKillEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn new() -> ::mono::sensor::MonoBuzzer {
                
            let mut o = ::mono::sensor::MonoBuzzer { raw: Default::default() };
            unsafe { stub__ZN4mono6sensor10MonoBuzzerC1Ev(&mut o.raw[0] as *mut u8); }
            o
            
            }
        }
        impl PCT2075Temperature {
            pub fn Read(&mut self) -> i32 {
                unsafe { stub__ZN4mono6sensor18PCT2075Temperature4ReadEv(&mut self.raw[0] as *mut u8) }
            }
            pub fn new() -> ::mono::sensor::PCT2075Temperature {
                
            let mut o = ::mono::sensor::PCT2075Temperature { raw: Default::default() };
            unsafe { stub__ZN4mono6sensor18PCT2075TemperatureC1Ev(&mut o.raw[0] as *mut u8); }
            o
            
            }
        }
    }
    pub struct AppRunLoop {
        raw: [u8],
    }
    pub struct ApplicationContext {
        raw: [u8],
    }
    pub struct Console {
        raw: [u8; 1],
    }
    pub struct DateTime {
        raw: [u8; 16],
    }
    pub struct IApplication {
        raw: [u8; 8],
    }
    pub struct IApplicationContext {
        raw: [u8; 1],
    }
    pub struct IQueueItem {
        raw: [u8; 8],
    }
    pub struct IRTCSystem {
        raw: [u8; 8],
    }
    pub struct IRunLoopTask {
        raw: [u8; 32],
    }
    pub struct ITouchSystem {
        raw: [u8; 64],
    }
    pub struct MonoRTC {
        raw: [u8; 8],
    }
    pub struct MonoTouchSystem {
        raw: [u8; 112],
    }
    pub struct Queue {
        raw: [u8; 16],
    }
    pub struct QueueInterrupt {
        raw: [u8; 1],
    }
    pub struct Regex {
        raw: [u8; 32],
    }
    pub struct String {
        raw: [u8; 24],
    }
    pub struct Timer {
        raw: [u8; 1],
    }
    pub struct TouchEvent {
        raw: [u8; 40],
    }
    pub struct TouchResponder {
        raw: [u8; 16],
    }
    extern "C" {
        // mono::AppRunLoop::CheckUsbDtr()
        fn stub__ZN4mono10AppRunLoop11CheckUsbDtrEv(o: *mut u8) -> ();
        // mono::AppRunLoop::addDynamicTask(mono::IRunLoopTask *)
        fn stub__ZN4mono10AppRunLoop14addDynamicTaskEPNS_12IRunLoopTaskE(o: *mut u8, p0: *::mono::IRunLoopTask) -> bool;
        // mono::AppRunLoop::removeDynamicTask(mono::IRunLoopTask *)
        fn stub__ZN4mono10AppRunLoop17removeDynamicTaskEPNS_12IRunLoopTaskE(o: *mut u8, p0: *::mono::IRunLoopTask) -> bool;
        // mono::AppRunLoop::setResetOnUserButton(bool)
        fn stub__ZN4mono10AppRunLoop20setResetOnUserButtonEb(o: *mut u8, p0: bool) -> ();
        // mono::AppRunLoop::exec()
        fn stub__ZN4mono10AppRunLoop4execEv(o: *mut u8) -> ();
        // mono::AppRunLoop::quit()
        fn stub__ZN4mono10AppRunLoop4quitEv(o: *mut u8) -> ();
        // mono::AppRunLoop::AppRunLoop()
        fn stub__ZN4mono10AppRunLoopC1Ev(o: *mut u8) -> ();
        // mono::IRTCSystem::setupRtcSystem()
        fn stub__ZN4mono10IRTCSystem14setupRtcSystemEv(o: *mut u8) -> ();
        // mono::IRTCSystem::stopRtc()
        fn stub__ZN4mono10IRTCSystem7stopRtcEv(o: *mut u8) -> ();
        // mono::IRTCSystem::startRtc()
        fn stub__ZN4mono10IRTCSystem8startRtcEv(o: *mut u8) -> ();
        // mono::TouchEvent::TouchEvent(mono::TouchEvent::TouchEventType, geo::Point &, mono::ITouchSystem *)
        fn stub__ZN4mono10TouchEventC1ENS0_14TouchEventTypeERNS_3geo5PointEPNS_12ITouchSystemE(o: *mut u8, p0: i32, p1: &::mono::geo::Point, p2: *::mono::ITouchSystem) -> ();
        // mono::TouchEvent::TouchEvent(const mono::TouchEvent &)
        fn stub__ZN4mono10TouchEventC1ERKS0_(o: *mut u8, p0: &const ::mono::TouchEvent) -> ();
        // mono::TouchEvent::TouchEvent()
        fn stub__ZN4mono10TouchEventC1Ev(o: *mut u8) -> ();
        // mono::TouchEvent::operator=(const mono::TouchEvent &)
        fn stub__ZN4mono10TouchEventaSERKS0_(o: *mut u8, p0: &const ::mono::TouchEvent) -> &::mono::TouchEvent;
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
        fn stub__ZN4mono12ITouchSystem14setCalibrationERNS_3geo4RectE(o: *mut u8, p0: &::mono::geo::Rect) -> ();
        // mono::ITouchSystem::ToScreenCoordsX(int, uint16_t)
        fn stub__ZN4mono12ITouchSystem15ToScreenCoordsXEit(o: *mut u8, p0: i32, p1: u16) -> i32;
        // mono::ITouchSystem::ToScreenCoordsY(int, uint16_t)
        fn stub__ZN4mono12ITouchSystem15ToScreenCoordsYEit(o: *mut u8, p0: i32, p1: u16) -> i32;
        // mono::ITouchSystem::processTouchInput()
        fn stub__ZN4mono12ITouchSystem17processTouchInputEv(o: *mut u8) -> ();
        // mono::ITouchSystem::CurrentCalibration()
        fn stub__ZN4mono12ITouchSystem18CurrentCalibrationEv(o: *mut u8) -> ::mono::geo::Rect;
        // mono::ITouchSystem::init()
        fn stub__ZN4mono12ITouchSystem4initEv(o: *mut u8) -> ();
        // mono::QueueInterrupt::FallTimeStamp()
        fn stub__ZN4mono14QueueInterrupt13FallTimeStampEv(o: *mut u8) -> i32;
        // mono::QueueInterrupt::RiseTimeStamp()
        fn stub__ZN4mono14QueueInterrupt13RiseTimeStampEv(o: *mut u8) -> i32;
        // mono::QueueInterrupt::setDebouncing(bool)
        fn stub__ZN4mono14QueueInterrupt13setDebouncingEb(o: *mut u8, p0: bool) -> ();
        // mono::QueueInterrupt::setDebounceTimeout(int)
        fn stub__ZN4mono14QueueInterrupt18setDebounceTimeoutEi(o: *mut u8, p0: i32) -> ();
        // mono::QueueInterrupt::DeactivateUntilHandled(bool)
        fn stub__ZN4mono14QueueInterrupt22DeactivateUntilHandledEb(o: *mut u8, p0: bool) -> ();
        // mono::QueueInterrupt::fall(void (*)())
        fn stub__ZN4mono14QueueInterrupt4fallEPFvvE(o: *mut u8, p0: *::) -> ();
        // mono::QueueInterrupt::rise(void (*)())
        fn stub__ZN4mono14QueueInterrupt4riseEPFvvE(o: *mut u8, p0: *::) -> ();
        // mono::QueueInterrupt::Snapshot()
        fn stub__ZN4mono14QueueInterrupt8SnapshotEv(o: *mut u8) -> bool;
        // mono::QueueInterrupt::QueueInterrupt(int, int)
        fn stub__ZN4mono14QueueInterruptC1Eii(o: *mut u8, p0: i32, p1: i32) -> ();
        // mono::QueueInterrupt::~QueueInterrupt()
        fn stub__ZN4mono14QueueInterruptD0Ev(o: *mut u8) -> ();
        // mono::TouchResponder::Deactivate()
        fn stub__ZN4mono14TouchResponder10DeactivateEv(o: *mut u8) -> ();
        // mono::TouchResponder::RespondTouchEnd(mono::TouchEvent &)
        fn stub__ZN4mono14TouchResponder15RespondTouchEndERNS_10TouchEventE(o: *mut u8, p0: &::mono::TouchEvent) -> ();
        // mono::TouchResponder::RespondTouchMove(mono::TouchEvent &)
        fn stub__ZN4mono14TouchResponder16RespondTouchMoveERNS_10TouchEventE(o: *mut u8, p0: &::mono::TouchEvent) -> ();
        // mono::TouchResponder::RespondTouchBegin(mono::TouchEvent &)
        fn stub__ZN4mono14TouchResponder17RespondTouchBeginERNS_10TouchEventE(o: *mut u8, p0: &::mono::TouchEvent) -> ();
        // mono::TouchResponder::RunResponderChainTouchEnd(mono::TouchEvent &)
        fn stub__ZN4mono14TouchResponder25RunResponderChainTouchEndERNS_10TouchEventE(o: *mut u8, p0: &::mono::TouchEvent) -> ();
        // mono::TouchResponder::RunResponderChainTouchMove(mono::TouchEvent &)
        fn stub__ZN4mono14TouchResponder26RunResponderChainTouchMoveERNS_10TouchEventE(o: *mut u8, p0: &::mono::TouchEvent) -> ();
        // mono::TouchResponder::RunResponderChainTouchBegin(mono::TouchEvent &)
        fn stub__ZN4mono14TouchResponder27RunResponderChainTouchBeginERNS_10TouchEventE(o: *mut u8, p0: &::mono::TouchEvent) -> ();
        // mono::TouchResponder::Activate()
        fn stub__ZN4mono14TouchResponder8ActivateEv(o: *mut u8) -> ();
        // mono::TouchResponder::TouchResponder()
        fn stub__ZN4mono14TouchResponderC1Ev(o: *mut u8) -> ();
        // mono::MonoTouchSystem::setCalibration(TouchCalibration &)
        fn stub__ZN4mono15MonoTouchSystem14setCalibrationERNS_3geo4RectE(o: *mut u8, p0: &::mono::geo::Rect) -> ();
        // mono::MonoTouchSystem::ToScreenCoordsX(int, uint16_t)
        fn stub__ZN4mono15MonoTouchSystem15ToScreenCoordsXEit(o: *mut u8, p0: i32, p1: u16) -> i32;
        // mono::MonoTouchSystem::ToScreenCoordsY(int, uint16_t)
        fn stub__ZN4mono15MonoTouchSystem15ToScreenCoordsYEit(o: *mut u8, p0: i32, p1: u16) -> i32;
        // mono::MonoTouchSystem::processTouchInput()
        fn stub__ZN4mono15MonoTouchSystem17processTouchInputEv(o: *mut u8) -> ();
        // mono::MonoTouchSystem::CurrentCalibration()
        fn stub__ZN4mono15MonoTouchSystem18CurrentCalibrationEv(o: *mut u8) -> ::mono::geo::Rect;
        // mono::MonoTouchSystem::init()
        fn stub__ZN4mono15MonoTouchSystem4initEv(o: *mut u8) -> ();
        // mono::MonoTouchSystem::MonoTouchSystem()
        fn stub__ZN4mono15MonoTouchSystemC1Ev(o: *mut u8) -> ();
        // mono::ApplicationContext::setMonoApplication(mono::IApplication *)
        fn stub__ZN4mono18ApplicationContext18setMonoApplicationEPNS_12IApplicationE(o: *mut u8, p0: *::mono::IApplication) -> ();
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
        fn stub__ZN4mono19IApplicationContext18setMonoApplicationEPNS_12IApplicationE(o: *mut u8, p0: *::mono::IApplication) -> ();
        // mono::IApplicationContext::SoftwareResetToBootloader()
        fn stub__ZN4mono19IApplicationContext25SoftwareResetToBootloaderEv(o: *mut u8) -> ();
        // mono::IApplicationContext::SoftwareResetToApplication()
        fn stub__ZN4mono19IApplicationContext26SoftwareResetToApplicationEv(o: *mut u8) -> ();
        // mono::IApplicationContext::exec()
        fn stub__ZN4mono19IApplicationContext4execEv(o: *mut u8) -> i32;
        // mono::Queue::Next(mono::IQueueItem *)
        fn stub__ZN4mono5Queue4NextEPNS_10IQueueItemE(o: *mut u8, p0: *::mono::IQueueItem) -> *::mono::IQueueItem;
        // mono::Queue::Peek()
        fn stub__ZN4mono5Queue4PeekEv(o: *mut u8) -> *::mono::IQueueItem;
        // mono::Queue::Exists(mono::IQueueItem *)
        fn stub__ZN4mono5Queue6ExistsEPNS_10IQueueItemE(o: *mut u8, p0: *::mono::IQueueItem) -> bool;
        // mono::Queue::Length()
        fn stub__ZN4mono5Queue6LengthEv(o: *mut u8) -> u16;
        // mono::Queue::Remove(mono::IQueueItem *)
        fn stub__ZN4mono5Queue6RemoveEPNS_10IQueueItemE(o: *mut u8, p0: *::mono::IQueueItem) -> bool;
        // mono::Queue::Dequeue()
        fn stub__ZN4mono5Queue7DequeueEv(o: *mut u8) -> *::mono::IQueueItem;
        // mono::Queue::Enqueue(mono::IQueueItem *)
        fn stub__ZN4mono5Queue7EnqueueEPNS_10IQueueItemE(o: *mut u8, p0: *::mono::IQueueItem) -> ();
        // mono::Queue::Queue()
        fn stub__ZN4mono5QueueC1Ev(o: *mut u8) -> ();
        // mono::Regex::Match(mono::String, Capture *, uint32_t)
        fn stub__ZN4mono5Regex5MatchENS_6StringEP8slre_capj(o: *mut u8, p0: ::mono::String, p1: *::::slre_cap, p2: u32) -> bool;
        // mono::Regex::Value(Capture &)
        fn stub__ZN4mono5Regex5ValueER8slre_cap(o: *mut u8, p0: &::::slre_cap) -> ::mono::String;
        // mono::Regex::IsMatch(mono::String)
        fn stub__ZN4mono5Regex7IsMatchENS_6StringE(o: *mut u8, p0: ::mono::String) -> bool;
        // mono::Regex::Regex(mono::String)
        fn stub__ZN4mono5RegexC1ENS_6StringE(o: *mut u8, p0: ::mono::String) -> ();
        // mono::Regex::Regex()
        fn stub__ZN4mono5RegexC1Ev(o: *mut u8) -> ();
        // mono::Timer::SingleShot()
        fn stub__ZN4mono5Timer10SingleShotEv(o: *mut u8) -> bool;
        // mono::Timer::setCallback(void (*)())
        fn stub__ZN4mono5Timer11setCallbackEPFvvE(o: *mut u8, p0: *::) -> ();
        // mono::Timer::setInterval(int)
        fn stub__ZN4mono5Timer11setIntervalEi(o: *mut u8, p0: i32) -> ();
        // mono::Timer::Stop()
        fn stub__ZN4mono5Timer4StopEv(o: *mut u8) -> ();
        // mono::Timer::Start()
        fn stub__ZN4mono5Timer5StartEv(o: *mut u8) -> ();
        // mono::Timer::Running()
        fn stub__ZN4mono5Timer7RunningEv(o: *mut u8) -> bool;
        // mono::Timer::Timer()
        fn stub__ZN4mono5TimerC1Ev(o: *mut u8) -> ();
        // mono::Timer::~Timer()
        fn stub__ZN4mono5TimerD0Ev(o: *mut u8) -> ();
        // mono::String::preAllocbytes(uint32_t)
        fn stub__ZN4mono6String13preAllocbytesEj(o: *mut u8, p0: u32) -> ();
        // mono::String::Format(const char *, ...)
        fn stub__ZN4mono6String6FormatEPKcz(o: *mut u8, p0: *const i8) -> ::mono::String;
        // mono::String::String(const char *)
        fn stub__ZN4mono6StringC1EPKc(o: *mut u8, p0: *const i8) -> ();
        // mono::String::String(char *)
        fn stub__ZN4mono6StringC1EPc(o: *mut u8, p0: *i8) -> ();
        // mono::String::String(char *, uint32_t)
        fn stub__ZN4mono6StringC1EPcj(o: *mut u8, p0: *i8, p1: u32) -> ();
        // mono::String::String(const mono::String &)
        fn stub__ZN4mono6StringC1ERKS0_(o: *mut u8, p0: &const ::mono::String) -> ();
        // mono::String::String(uint32_t)
        fn stub__ZN4mono6StringC1Ej(o: *mut u8, p0: u32) -> ();
        // mono::String::String()
        fn stub__ZN4mono6StringC1Ev(o: *mut u8) -> ();
        // mono::String::~String()
        fn stub__ZN4mono6StringD0Ev(o: *mut u8) -> ();
        // mono::String::operator=(const char *)
        fn stub__ZN4mono6StringaSEPKc(o: *mut u8, p0: *const i8) -> &::mono::String;
        // mono::String::operator=(const mono::String &)
        fn stub__ZN4mono6StringaSERKS0_(o: *mut u8, p0: &const ::mono::String) -> &::mono::String;
        // mono::String::operator==(const char *)
        fn stub__ZN4mono6StringeqEPKc(o: *mut u8, p0: *const i8) -> bool;
        // mono::String::operator==(const mono::String &)
        fn stub__ZN4mono6StringeqERKS0_(o: *mut u8, p0: &const ::mono::String) -> bool;
        // mono::String::operator!=(const char *)
        fn stub__ZN4mono6StringneEPKc(o: *mut u8, p0: *const i8) -> bool;
        // mono::String::operator!=(const mono::String &)
        fn stub__ZN4mono6StringneERKS0_(o: *mut u8, p0: &const ::mono::String) -> bool;
        // mono::Console::Console(int *)
        fn stub__ZN4mono7ConsoleC1EPi(o: *mut u8, p0: *i32) -> ();
        // mono::Console::operator<<(const char *)
        fn stub__ZN4mono7ConsolelsEPKc(o: *mut u8, p0: *const i8) -> &::mono::Console;
        // mono::Console::operator<<(char)
        fn stub__ZN4mono7ConsolelsEc(o: *mut u8, p0: i8) -> &::mono::Console;
        // mono::Console::operator<<(float)
        fn stub__ZN4mono7ConsolelsEf(o: *mut u8, p0: f32) -> &::mono::Console;
        // mono::Console::operator<<(int)
        fn stub__ZN4mono7ConsolelsEi(o: *mut u8, p0: i32) -> &::mono::Console;
        // mono::MonoRTC::setupRtcSystem()
        fn stub__ZN4mono7MonoRTC14setupRtcSystemEv(o: *mut u8) -> ();
        // mono::MonoRTC::stopRtc()
        fn stub__ZN4mono7MonoRTC7stopRtcEv(o: *mut u8) -> ();
        // mono::MonoRTC::startRtc()
        fn stub__ZN4mono7MonoRTC8startRtcEv(o: *mut u8) -> ();
        // mono::DateTime::isLeapYear(uint16_t)
        fn stub__ZN4mono8DateTime10isLeapYearEt(o: *mut u8, p0: u16) -> bool;
        // mono::DateTime::fromISO8601(mono::String)
        fn stub__ZN4mono8DateTime11fromISO8601ENS_6StringE(o: *mut u8, p0: ::mono::String) -> ::mono::DateTime;
        // mono::DateTime::setSystemDateTime(mono::DateTime)
        fn stub__ZN4mono8DateTime17setSystemDateTimeES0_(o: *mut u8, p0: ::mono::DateTime) -> ();
        // mono::DateTime::incrementSystemClock()
        fn stub__ZN4mono8DateTime20incrementSystemClockEv(o: *mut u8) -> ();
        // mono::DateTime::now()
        fn stub__ZN4mono8DateTime3nowEv(o: *mut u8) -> ::mono::DateTime;
        // mono::DateTime::maxValue()
        fn stub__ZN4mono8DateTime8maxValueEv(o: *mut u8) -> ::mono::DateTime;
        // mono::DateTime::minValue()
        fn stub__ZN4mono8DateTime8minValueEv(o: *mut u8) -> ::mono::DateTime;
        // mono::DateTime::DateTime(const mono::DateTime &)
        fn stub__ZN4mono8DateTimeC1ERKS0_(o: *mut u8, p0: &const ::mono::DateTime) -> ();
        // mono::DateTime::DateTime(uint16_t, uint8_t, uint8_t, uint8_t, uint8_t, uint8_t, mono::DateTime::TimeTypes)
        fn stub__ZN4mono8DateTimeC1EthhhhhNS0_9TimeTypesE(o: *mut u8, p0: u16, p1: u8, p2: u8, p3: u8, p4: u8, p5: u8, p6: i32) -> ();
        // mono::DateTime::DateTime()
        fn stub__ZN4mono8DateTimeC1Ev(o: *mut u8) -> ();
        // mono::DateTime::operator=(const mono::DateTime &)
        fn stub__ZN4mono8DateTimeaSERKS0_(o: *mut u8, p0: &const ::mono::DateTime) -> &::mono::DateTime;
        // mono::QueueInterrupt::IsInterruptsWhilePendingActive()
        fn stub__ZNK4mono14QueueInterrupt30IsInterruptsWhilePendingActiveEv(o: *mut u8) -> bool;
        // mono::String::Length()
        fn stub__ZNK4mono6String6LengthEv(o: *mut u8) -> u32;
        // mono::String::operator()()
        fn stub__ZNK4mono6StringclEv(o: *mut u8) -> *i8;
        // mono::String::operator[](uint32_t)
        fn stub__ZNK4mono6StringixEj(o: *mut u8, p0: u32) -> i8;
        // mono::DateTime::addMinutes(int)
        fn stub__ZNK4mono8DateTime10addMinutesEi(o: *mut u8, p0: i32) -> ::mono::DateTime;
        // mono::DateTime::addSeconds(int)
        fn stub__ZNK4mono8DateTime10addSecondsEi(o: *mut u8, p0: i32) -> ::mono::DateTime;
        // mono::DateTime::toDateString()
        fn stub__ZNK4mono8DateTime12toDateStringEv(o: *mut u8) -> ::mono::String;
        // mono::DateTime::toTimeString()
        fn stub__ZNK4mono8DateTime12toTimeStringEv(o: *mut u8) -> ::mono::String;
        // mono::DateTime::addDays(int)
        fn stub__ZNK4mono8DateTime7addDaysEi(o: *mut u8, p0: i32) -> ::mono::DateTime;
        // mono::DateTime::isValid()
        fn stub__ZNK4mono8DateTime7isValidEv(o: *mut u8) -> bool;
        // mono::DateTime::addHours(int)
        fn stub__ZNK4mono8DateTime8addHoursEi(o: *mut u8, p0: i32) -> ::mono::DateTime;
        // mono::DateTime::toString()
        fn stub__ZNK4mono8DateTime8toStringEv(o: *mut u8) -> ::mono::String;
        // mono::DateTime::toISO8601()
        fn stub__ZNK4mono8DateTime9toISO8601Ev(o: *mut u8) -> ::mono::String;
        // mono::DateTime::toUtcTime()
        fn stub__ZNK4mono8DateTime9toUtcTimeEv(o: *mut u8) -> ::mono::DateTime;
        // mono::DateTime::operator==(const mono::DateTime &)
        fn stub__ZNK4mono8DateTimeeqERKS0_(o: *mut u8, p0: &const ::mono::DateTime) -> bool;
        // mono::DateTime::operator>=(const mono::DateTime &)
        fn stub__ZNK4mono8DateTimegeERKS0_(o: *mut u8, p0: &const ::mono::DateTime) -> bool;
        // mono::DateTime::operator>(const mono::DateTime &)
        fn stub__ZNK4mono8DateTimegtERKS0_(o: *mut u8, p0: &const ::mono::DateTime) -> bool;
        // mono::DateTime::operator<=(const mono::DateTime &)
        fn stub__ZNK4mono8DateTimeleERKS0_(o: *mut u8, p0: &const ::mono::DateTime) -> bool;
        // mono::DateTime::operator<(const mono::DateTime &)
        fn stub__ZNK4mono8DateTimeltERKS0_(o: *mut u8, p0: &const ::mono::DateTime) -> bool;
        // mono::DateTime::operator!=(const mono::DateTime &)
        fn stub__ZNK4mono8DateTimeneERKS0_(o: *mut u8, p0: &const ::mono::DateTime) -> bool;
    }
    impl AppRunLoop {
        pub fn CheckUsbDtr(&mut self) -> () {
            unsafe { stub__ZN4mono10AppRunLoop11CheckUsbDtrEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn addDynamicTask(&mut self, p0: *::mono::IRunLoopTask) -> bool {
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
        pub fn removeDynamicTask(&mut self, p0: *::mono::IRunLoopTask) -> bool {
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
        pub fn setMonoApplication(&mut self, p0: *::mono::IApplication) -> () {
            unsafe { stub__ZN4mono18ApplicationContext18setMonoApplicationEPNS_12IApplicationE(&mut self.raw[0] as *mut u8, p0) }
        }
    }
    impl Console {
        pub fn new(p0: *i32) -> ::mono::Console {
            
        let mut o = ::mono::Console { raw: Default::default() };
        unsafe { stub__ZN4mono7ConsoleC1EPi(&mut o.raw[0] as *mut u8, p0); }
        o
        
        }
        pub fn operator<<_PKc(&mut self, p0: *const i8) -> &::mono::Console {
            unsafe { stub__ZN4mono7ConsolelsEPKc(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn operator<<_c(&mut self, p0: i8) -> &::mono::Console {
            unsafe { stub__ZN4mono7ConsolelsEc(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn operator<<_f(&mut self, p0: f32) -> &::mono::Console {
            unsafe { stub__ZN4mono7ConsolelsEf(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn operator<<_i(&mut self, p0: i32) -> &::mono::Console {
            unsafe { stub__ZN4mono7ConsolelsEi(&mut self.raw[0] as *mut u8, p0) }
        }
    }
    impl DateTime {
        pub fn addDays(&mut self, p0: i32) -> ::mono::DateTime {
            unsafe { stub__ZNK4mono8DateTime7addDaysEi(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn addHours(&mut self, p0: i32) -> ::mono::DateTime {
            unsafe { stub__ZNK4mono8DateTime8addHoursEi(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn addMinutes(&mut self, p0: i32) -> ::mono::DateTime {
            unsafe { stub__ZNK4mono8DateTime10addMinutesEi(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn addSeconds(&mut self, p0: i32) -> ::mono::DateTime {
            unsafe { stub__ZNK4mono8DateTime10addSecondsEi(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn fromISO8601(&mut self, p0: ::mono::String) -> ::mono::DateTime {
            unsafe { stub__ZN4mono8DateTime11fromISO8601ENS_6StringE(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn incrementSystemClock(&mut self) -> () {
            unsafe { stub__ZN4mono8DateTime20incrementSystemClockEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn isLeapYear(&mut self, p0: u16) -> bool {
            unsafe { stub__ZN4mono8DateTime10isLeapYearEt(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn isValid(&mut self) -> bool {
            unsafe { stub__ZNK4mono8DateTime7isValidEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn maxValue(&mut self) -> ::mono::DateTime {
            unsafe { stub__ZN4mono8DateTime8maxValueEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn minValue(&mut self) -> ::mono::DateTime {
            unsafe { stub__ZN4mono8DateTime8minValueEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn new() -> ::mono::DateTime {
            
        let mut o = ::mono::DateTime { raw: Default::default() };
        unsafe { stub__ZN4mono8DateTimeC1Ev(&mut o.raw[0] as *mut u8); }
        o
        
        }
        pub fn new_RKS0_(p0: &const ::mono::DateTime) -> ::mono::DateTime {
            
        let mut o = ::mono::DateTime { raw: Default::default() };
        unsafe { stub__ZN4mono8DateTimeC1ERKS0_(&mut o.raw[0] as *mut u8, p0); }
        o
        
        }
        pub fn new_thhhhhNS0_9TimeTypesE(p0: u16, p1: u8, p2: u8, p3: u8, p4: u8, p5: u8, p6: i32) -> ::mono::DateTime {
            
        let mut o = ::mono::DateTime { raw: Default::default() };
        unsafe { stub__ZN4mono8DateTimeC1EthhhhhNS0_9TimeTypesE(&mut o.raw[0] as *mut u8, p0, p1, p2, p3, p4, p5, p6); }
        o
        
        }
        pub fn now(&mut self) -> ::mono::DateTime {
            unsafe { stub__ZN4mono8DateTime3nowEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn operator!=(&mut self, p0: &const ::mono::DateTime) -> bool {
            unsafe { stub__ZNK4mono8DateTimeneERKS0_(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn operator<(&mut self, p0: &const ::mono::DateTime) -> bool {
            unsafe { stub__ZNK4mono8DateTimeltERKS0_(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn operator<=(&mut self, p0: &const ::mono::DateTime) -> bool {
            unsafe { stub__ZNK4mono8DateTimeleERKS0_(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn operator=(&mut self, p0: &const ::mono::DateTime) -> &::mono::DateTime {
            unsafe { stub__ZN4mono8DateTimeaSERKS0_(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn operator==(&mut self, p0: &const ::mono::DateTime) -> bool {
            unsafe { stub__ZNK4mono8DateTimeeqERKS0_(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn operator>(&mut self, p0: &const ::mono::DateTime) -> bool {
            unsafe { stub__ZNK4mono8DateTimegtERKS0_(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn operator>=(&mut self, p0: &const ::mono::DateTime) -> bool {
            unsafe { stub__ZNK4mono8DateTimegeERKS0_(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn setSystemDateTime(&mut self, p0: ::mono::DateTime) -> () {
            unsafe { stub__ZN4mono8DateTime17setSystemDateTimeES0_(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn toDateString(&mut self) -> ::mono::String {
            unsafe { stub__ZNK4mono8DateTime12toDateStringEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn toISO8601(&mut self) -> ::mono::String {
            unsafe { stub__ZNK4mono8DateTime9toISO8601Ev(&mut self.raw[0] as *mut u8) }
        }
        pub fn toString(&mut self) -> ::mono::String {
            unsafe { stub__ZNK4mono8DateTime8toStringEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn toTimeString(&mut self) -> ::mono::String {
            unsafe { stub__ZNK4mono8DateTime12toTimeStringEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn toUtcTime(&mut self) -> ::mono::DateTime {
            unsafe { stub__ZNK4mono8DateTime9toUtcTimeEv(&mut self.raw[0] as *mut u8) }
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
        pub fn new() -> ::mono::IApplication {
            
        let mut o = ::mono::IApplication { raw: Default::default() };
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
        pub fn setMonoApplication(&mut self, p0: *::mono::IApplication) -> () {
            unsafe { stub__ZN4mono19IApplicationContext18setMonoApplicationEPNS_12IApplicationE(&mut self.raw[0] as *mut u8, p0) }
        }
    }
    impl IRTCSystem {
        pub fn setupRtcSystem(&mut self) -> () {
            unsafe { stub__ZN4mono10IRTCSystem14setupRtcSystemEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn startRtc(&mut self) -> () {
            unsafe { stub__ZN4mono10IRTCSystem8startRtcEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn stopRtc(&mut self) -> () {
            unsafe { stub__ZN4mono10IRTCSystem7stopRtcEv(&mut self.raw[0] as *mut u8) }
        }
    }
    impl ITouchSystem {
        pub fn CurrentCalibration(&mut self) -> ::mono::geo::Rect {
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
        pub fn setCalibration(&mut self, p0: &::mono::geo::Rect) -> () {
            unsafe { stub__ZN4mono12ITouchSystem14setCalibrationERNS_3geo4RectE(&mut self.raw[0] as *mut u8, p0) }
        }
    }
    impl MonoRTC {
        pub fn setupRtcSystem(&mut self) -> () {
            unsafe { stub__ZN4mono7MonoRTC14setupRtcSystemEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn startRtc(&mut self) -> () {
            unsafe { stub__ZN4mono7MonoRTC8startRtcEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn stopRtc(&mut self) -> () {
            unsafe { stub__ZN4mono7MonoRTC7stopRtcEv(&mut self.raw[0] as *mut u8) }
        }
    }
    impl MonoTouchSystem {
        pub fn CurrentCalibration(&mut self) -> ::mono::geo::Rect {
            unsafe { stub__ZN4mono15MonoTouchSystem18CurrentCalibrationEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn ToScreenCoordsX(&mut self, p0: i32, p1: u16) -> i32 {
            unsafe { stub__ZN4mono15MonoTouchSystem15ToScreenCoordsXEit(&mut self.raw[0] as *mut u8, p0, p1) }
        }
        pub fn ToScreenCoordsY(&mut self, p0: i32, p1: u16) -> i32 {
            unsafe { stub__ZN4mono15MonoTouchSystem15ToScreenCoordsYEit(&mut self.raw[0] as *mut u8, p0, p1) }
        }
        pub fn init(&mut self) -> () {
            unsafe { stub__ZN4mono15MonoTouchSystem4initEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn new() -> ::mono::MonoTouchSystem {
            
        let mut o = ::mono::MonoTouchSystem { raw: Default::default() };
        unsafe { stub__ZN4mono15MonoTouchSystemC1Ev(&mut o.raw[0] as *mut u8); }
        o
        
        }
        pub fn processTouchInput(&mut self) -> () {
            unsafe { stub__ZN4mono15MonoTouchSystem17processTouchInputEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn setCalibration(&mut self, p0: &::mono::geo::Rect) -> () {
            unsafe { stub__ZN4mono15MonoTouchSystem14setCalibrationERNS_3geo4RectE(&mut self.raw[0] as *mut u8, p0) }
        }
    }
    impl Queue {
        pub fn Dequeue(&mut self) -> *::mono::IQueueItem {
            unsafe { stub__ZN4mono5Queue7DequeueEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn Enqueue(&mut self, p0: *::mono::IQueueItem) -> () {
            unsafe { stub__ZN4mono5Queue7EnqueueEPNS_10IQueueItemE(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn Exists(&mut self, p0: *::mono::IQueueItem) -> bool {
            unsafe { stub__ZN4mono5Queue6ExistsEPNS_10IQueueItemE(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn Length(&mut self) -> u16 {
            unsafe { stub__ZN4mono5Queue6LengthEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn Next(&mut self, p0: *::mono::IQueueItem) -> *::mono::IQueueItem {
            unsafe { stub__ZN4mono5Queue4NextEPNS_10IQueueItemE(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn Peek(&mut self) -> *::mono::IQueueItem {
            unsafe { stub__ZN4mono5Queue4PeekEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn Remove(&mut self, p0: *::mono::IQueueItem) -> bool {
            unsafe { stub__ZN4mono5Queue6RemoveEPNS_10IQueueItemE(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn new() -> ::mono::Queue {
            
        let mut o = ::mono::Queue { raw: Default::default() };
        unsafe { stub__ZN4mono5QueueC1Ev(&mut o.raw[0] as *mut u8); }
        o
        
        }
    }
    impl QueueInterrupt {
        pub fn DeactivateUntilHandled(&mut self, p0: bool) -> () {
            unsafe { stub__ZN4mono14QueueInterrupt22DeactivateUntilHandledEb(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn FallTimeStamp(&mut self) -> i32 {
            unsafe { stub__ZN4mono14QueueInterrupt13FallTimeStampEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn IsInterruptsWhilePendingActive(&mut self) -> bool {
            unsafe { stub__ZNK4mono14QueueInterrupt30IsInterruptsWhilePendingActiveEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn RiseTimeStamp(&mut self) -> i32 {
            unsafe { stub__ZN4mono14QueueInterrupt13RiseTimeStampEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn Snapshot(&mut self) -> bool {
            unsafe { stub__ZN4mono14QueueInterrupt8SnapshotEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn fall(&mut self, p0: *::) -> () {
            unsafe { stub__ZN4mono14QueueInterrupt4fallEPFvvE(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn new(p0: i32, p1: i32) -> ::mono::QueueInterrupt {
            
        let mut o = ::mono::QueueInterrupt { raw: Default::default() };
        unsafe { stub__ZN4mono14QueueInterruptC1Eii(&mut o.raw[0] as *mut u8, p0, p1); }
        o
        
        }
        pub fn rise(&mut self, p0: *::) -> () {
            unsafe { stub__ZN4mono14QueueInterrupt4riseEPFvvE(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn setDebounceTimeout(&mut self, p0: i32) -> () {
            unsafe { stub__ZN4mono14QueueInterrupt18setDebounceTimeoutEi(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn setDebouncing(&mut self, p0: bool) -> () {
            unsafe { stub__ZN4mono14QueueInterrupt13setDebouncingEb(&mut self.raw[0] as *mut u8, p0) }
        }
    }
    impl Regex {
        pub fn IsMatch(&mut self, p0: ::mono::String) -> bool {
            unsafe { stub__ZN4mono5Regex7IsMatchENS_6StringE(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn Match(&mut self, p0: ::mono::String, p1: *::::slre_cap, p2: u32) -> bool {
            unsafe { stub__ZN4mono5Regex5MatchENS_6StringEP8slre_capj(&mut self.raw[0] as *mut u8, p0, p1, p2) }
        }
        pub fn Value(&mut self, p0: &::::slre_cap) -> ::mono::String {
            unsafe { stub__ZN4mono5Regex5ValueER8slre_cap(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn new() -> ::mono::Regex {
            
        let mut o = ::mono::Regex { raw: Default::default() };
        unsafe { stub__ZN4mono5RegexC1Ev(&mut o.raw[0] as *mut u8); }
        o
        
        }
        pub fn new_NS_6StringE(p0: ::mono::String) -> ::mono::Regex {
            
        let mut o = ::mono::Regex { raw: Default::default() };
        unsafe { stub__ZN4mono5RegexC1ENS_6StringE(&mut o.raw[0] as *mut u8, p0); }
        o
        
        }
    }
    impl String {
        pub fn Format(&mut self, p0: *const i8) -> ::mono::String {
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
        pub fn new_PKc(p0: *const i8) -> ::mono::String {
            
        let mut o = ::mono::String { raw: Default::default() };
        unsafe { stub__ZN4mono6StringC1EPKc(&mut o.raw[0] as *mut u8, p0); }
        o
        
        }
        pub fn new_Pc(p0: *i8) -> ::mono::String {
            
        let mut o = ::mono::String { raw: Default::default() };
        unsafe { stub__ZN4mono6StringC1EPc(&mut o.raw[0] as *mut u8, p0); }
        o
        
        }
        pub fn new_Pcj(p0: *i8, p1: u32) -> ::mono::String {
            
        let mut o = ::mono::String { raw: Default::default() };
        unsafe { stub__ZN4mono6StringC1EPcj(&mut o.raw[0] as *mut u8, p0, p1); }
        o
        
        }
        pub fn new_RKS0_(p0: &const ::mono::String) -> ::mono::String {
            
        let mut o = ::mono::String { raw: Default::default() };
        unsafe { stub__ZN4mono6StringC1ERKS0_(&mut o.raw[0] as *mut u8, p0); }
        o
        
        }
        pub fn new_j(p0: u32) -> ::mono::String {
            
        let mut o = ::mono::String { raw: Default::default() };
        unsafe { stub__ZN4mono6StringC1Ej(&mut o.raw[0] as *mut u8, p0); }
        o
        
        }
        pub fn operator!=_PKc(&mut self, p0: *const i8) -> bool {
            unsafe { stub__ZN4mono6StringneEPKc(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn operator!=_RKS0_(&mut self, p0: &const ::mono::String) -> bool {
            unsafe { stub__ZN4mono6StringneERKS0_(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn operator()(&mut self) -> *i8 {
            unsafe { stub__ZNK4mono6StringclEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn operator==_PKc(&mut self, p0: *const i8) -> bool {
            unsafe { stub__ZN4mono6StringeqEPKc(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn operator==_RKS0_(&mut self, p0: &const ::mono::String) -> bool {
            unsafe { stub__ZN4mono6StringeqERKS0_(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn operator=_PKc(&mut self, p0: *const i8) -> &::mono::String {
            unsafe { stub__ZN4mono6StringaSEPKc(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn operator=_RKS0_(&mut self, p0: &const ::mono::String) -> &::mono::String {
            unsafe { stub__ZN4mono6StringaSERKS0_(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn operator[](&mut self, p0: u32) -> i8 {
            unsafe { stub__ZNK4mono6StringixEj(&mut self.raw[0] as *mut u8, p0) }
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
        pub fn new() -> ::mono::Timer {
            
        let mut o = ::mono::Timer { raw: Default::default() };
        unsafe { stub__ZN4mono5TimerC1Ev(&mut o.raw[0] as *mut u8); }
        o
        
        }
        pub fn setCallback(&mut self, p0: *::) -> () {
            unsafe { stub__ZN4mono5Timer11setCallbackEPFvvE(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn setInterval(&mut self, p0: i32) -> () {
            unsafe { stub__ZN4mono5Timer11setIntervalEi(&mut self.raw[0] as *mut u8, p0) }
        }
    }
    impl TouchEvent {
        pub fn new() -> ::mono::TouchEvent {
            
        let mut o = ::mono::TouchEvent { raw: Default::default() };
        unsafe { stub__ZN4mono10TouchEventC1Ev(&mut o.raw[0] as *mut u8); }
        o
        
        }
        pub fn new_NS0_14TouchEventTypeERNS_3geo5PointEPNS_12ITouchSystemE(p0: i32, p1: &::mono::geo::Point, p2: *::mono::ITouchSystem) -> ::mono::TouchEvent {
            
        let mut o = ::mono::TouchEvent { raw: Default::default() };
        unsafe { stub__ZN4mono10TouchEventC1ENS0_14TouchEventTypeERNS_3geo5PointEPNS_12ITouchSystemE(&mut o.raw[0] as *mut u8, p0, p1, p2); }
        o
        
        }
        pub fn new_RKS0_(p0: &const ::mono::TouchEvent) -> ::mono::TouchEvent {
            
        let mut o = ::mono::TouchEvent { raw: Default::default() };
        unsafe { stub__ZN4mono10TouchEventC1ERKS0_(&mut o.raw[0] as *mut u8, p0); }
        o
        
        }
        pub fn operator=(&mut self, p0: &const ::mono::TouchEvent) -> &::mono::TouchEvent {
            unsafe { stub__ZN4mono10TouchEventaSERKS0_(&mut self.raw[0] as *mut u8, p0) }
        }
    }
    impl TouchResponder {
        pub fn Activate(&mut self) -> () {
            unsafe { stub__ZN4mono14TouchResponder8ActivateEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn Deactivate(&mut self) -> () {
            unsafe { stub__ZN4mono14TouchResponder10DeactivateEv(&mut self.raw[0] as *mut u8) }
        }
        pub fn RespondTouchBegin(&mut self, p0: &::mono::TouchEvent) -> () {
            unsafe { stub__ZN4mono14TouchResponder17RespondTouchBeginERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn RespondTouchEnd(&mut self, p0: &::mono::TouchEvent) -> () {
            unsafe { stub__ZN4mono14TouchResponder15RespondTouchEndERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn RespondTouchMove(&mut self, p0: &::mono::TouchEvent) -> () {
            unsafe { stub__ZN4mono14TouchResponder16RespondTouchMoveERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn RunResponderChainTouchBegin(&mut self, p0: &::mono::TouchEvent) -> () {
            unsafe { stub__ZN4mono14TouchResponder27RunResponderChainTouchBeginERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn RunResponderChainTouchEnd(&mut self, p0: &::mono::TouchEvent) -> () {
            unsafe { stub__ZN4mono14TouchResponder25RunResponderChainTouchEndERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn RunResponderChainTouchMove(&mut self, p0: &::mono::TouchEvent) -> () {
            unsafe { stub__ZN4mono14TouchResponder26RunResponderChainTouchMoveERNS_10TouchEventE(&mut self.raw[0] as *mut u8, p0) }
        }
        pub fn new() -> ::mono::TouchResponder {
            
        let mut o = ::mono::TouchResponder { raw: Default::default() };
        unsafe { stub__ZN4mono14TouchResponderC1Ev(&mut o.raw[0] as *mut u8); }
        o
        
        }
    }
}
pub struct ITouchSystem {
    raw: [u8],
}
extern "C" {
}
