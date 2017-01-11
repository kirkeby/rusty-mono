#include <new>
#include "display/ui/text_label_view.h"
#include "application_context_interface.h"
#include "mn_string.h"
#include "circle.h"
#include "display/font_interface.h"
#include "size.h"
#include "color.h"
#include "rect.h"
#include "point.h"
#include "application_run_loop_task_interface.h"
#include "application_controller_interface.h"
#include "display/ui/view.h"

/* mono::IApplication::enterRunLoop() */
extern "C"
int stub__ZN4mono12IApplication12enterRunLoopEv(void *o)
{
    return ((mono::IApplication *) o)->enterRunLoop();
}

/* mono::IApplication::monoWakeFromReset() */
extern "C"
void stub__ZN4mono12IApplication17monoWakeFromResetEv(void *o)
{
    ((mono::IApplication *) o)->monoWakeFromReset();
}

/* mono::IApplication::monoWakeFromSleep() */
extern "C"
void stub__ZN4mono12IApplication17monoWakeFromSleepEv(void *o)
{
    ((mono::IApplication *) o)->monoWakeFromSleep();
}

/* mono::IApplication::monoWillGotoSleep() */
extern "C"
void stub__ZN4mono12IApplication17monoWillGotoSleepEv(void *o)
{
    ((mono::IApplication *) o)->monoWillGotoSleep();
}

/* mono::IApplicationContext::SleepForMs(uint32_t) */
extern "C"
void stub__ZN4mono19IApplicationContext10SleepForMsEj(void *o, unsigned int a0)
{
    ((mono::IApplicationContext *) o)->SleepForMs(a0);
}

/* mono::IApplicationContext::SoftwareReset() */
extern "C"
void stub__ZN4mono19IApplicationContext13SoftwareResetEv(void *o)
{
    ((mono::IApplicationContext *) o)->SoftwareReset();
}

/* mono::IApplicationContext::EnterSleepMode() */
extern "C"
void stub__ZN4mono19IApplicationContext14EnterSleepModeEv(void *o)
{
    ((mono::IApplicationContext *) o)->EnterSleepMode();
}

/* mono::IApplicationContext::ResetOnUserButton() */
extern "C"
void stub__ZN4mono19IApplicationContext17ResetOnUserButtonEv(void *o)
{
    ((mono::IApplicationContext *) o)->ResetOnUserButton();
}

/* mono::IApplicationContext::setMonoApplication(mono::IApplication *) */
extern "C"
void stub__ZN4mono19IApplicationContext18setMonoApplicationEPNS_12IApplicationE(void *o, mono::IApplication * a0)
{
    ((mono::IApplicationContext *) o)->setMonoApplication(a0);
}

/* mono::IApplicationContext::SoftwareResetToBootloader() */
extern "C"
void stub__ZN4mono19IApplicationContext25SoftwareResetToBootloaderEv(void *o)
{
    ((mono::IApplicationContext *) o)->SoftwareResetToBootloader();
}

/* mono::IApplicationContext::SoftwareResetToApplication() */
extern "C"
void stub__ZN4mono19IApplicationContext26SoftwareResetToApplicationEv(void *o)
{
    ((mono::IApplicationContext *) o)->SoftwareResetToApplication();
}

/* mono::IApplicationContext::exec() */
extern "C"
int stub__ZN4mono19IApplicationContext4execEv(void *o)
{
    return ((mono::IApplicationContext *) o)->exec();
}

/* mono::ui::TextLabelView::setTextSize(uint8_t) */
extern "C"
void stub__ZN4mono2ui13TextLabelView11setTextSizeEh(void *o, unsigned char a0)
{
    ((mono::ui::TextLabelView *) o)->setTextSize(a0);
}

/* mono::ui::TextLabelView::setAlignment(mono::ui::TextLabelView::TextAlignment) */
extern "C"
void stub__ZN4mono2ui13TextLabelView12setAlignmentENS1_13TextAlignmentE(void *o, mono::ui::TextLabelView::TextAlignment a0)
{
    ((mono::ui::TextLabelView *) o)->setAlignment(a0);
}

/* mono::ui::TextLabelView::setTextColor(display::Color) */
extern "C"
void stub__ZN4mono2ui13TextLabelView12setTextColorENS_7display5ColorE(void *o, mono::display::Color a0)
{
    ((mono::ui::TextLabelView *) o)->setTextColor(a0);
}

/* mono::ui::TextLabelView::setBackground(display::Color) */
extern "C"
void stub__ZN4mono2ui13TextLabelView13setBackgroundENS_7display5ColorE(void *o, mono::display::Color a0)
{
    ((mono::ui::TextLabelView *) o)->setBackground(a0);
}

/* mono::ui::TextLabelView::scheduleRepaint() */
extern "C"
void stub__ZN4mono2ui13TextLabelView15scheduleRepaintEv(void *o)
{
    ((mono::ui::TextLabelView *) o)->scheduleRepaint();
}

/* mono::ui::TextLabelView::setBackgroundColor(display::Color) */
extern "C"
void stub__ZN4mono2ui13TextLabelView18setBackgroundColorENS_7display5ColorE(void *o, mono::display::Color a0)
{
    ((mono::ui::TextLabelView *) o)->setBackgroundColor(a0);
}

/* mono::ui::TextLabelView::repaint() */
extern "C"
void stub__ZN4mono2ui13TextLabelView7repaintEv(void *o)
{
    ((mono::ui::TextLabelView *) o)->repaint();
}

/* mono::ui::TextLabelView::setFont(const MonoFont &) */
extern "C"
void stub__ZN4mono2ui13TextLabelView7setFontERK8MonoFont(void *o, const MonoFont & a0)
{
    ((mono::ui::TextLabelView *) o)->setFont(a0);
}

/* mono::ui::TextLabelView::setText(mono::String, bool) */
extern "C"
void stub__ZN4mono2ui13TextLabelView7setTextENS_6StringEb(void *o, mono::String a0, bool a1)
{
    ((mono::ui::TextLabelView *) o)->setText(a0, a1);
}

/* mono::ui::TextLabelView::setText(display::Color) */
extern "C"
void stub__ZN4mono2ui13TextLabelView7setTextENS_7display5ColorE(void *o, mono::display::Color a0)
{
    ((mono::ui::TextLabelView *) o)->setText(a0);
}

/* mono::ui::TextLabelView::setText(const char *, bool) */
extern "C"
void stub__ZN4mono2ui13TextLabelView7setTextEPKcb(void *o, const char * a0, bool a1)
{
    ((mono::ui::TextLabelView *) o)->setText(a0, a1);
}

/* mono::ui::TextLabelView::setText(char *, bool) */
extern "C"
void stub__ZN4mono2ui13TextLabelView7setTextEPcb(void *o, char * a0, bool a1)
{
    ((mono::ui::TextLabelView *) o)->setText(a0, a1);
}

/* mono::ui::View::DisplaySize() */
extern "C"
mono::geo::Size stub__ZN4mono2ui4View11DisplaySizeEv(void *o)
{
    return ((mono::ui::View *) o)->DisplaySize();
}

/* mono::ui::View::setPosition(geo::Point) */
extern "C"
void stub__ZN4mono2ui4View11setPositionENS_3geo5PointE(void *o, mono::geo::Point a0)
{
    ((mono::ui::View *) o)->setPosition(a0);
}

/* mono::ui::View::DisplayWidth() */
extern "C"
unsigned short stub__ZN4mono2ui4View12DisplayWidthEv(void *o)
{
    return ((mono::ui::View *) o)->DisplayWidth();
}

/* mono::ui::View::DisplayHeight() */
extern "C"
unsigned short stub__ZN4mono2ui4View13DisplayHeightEv(void *o)
{
    return ((mono::ui::View *) o)->DisplayHeight();
}

/* mono::ui::View::scheduleRepaint() */
extern "C"
void stub__ZN4mono2ui4View15scheduleRepaintEv(void *o)
{
    ((mono::ui::View *) o)->scheduleRepaint();
}

/* mono::ui::View::DisplayOrientation() */
extern "C"
mono::ui::View::Orientation stub__ZN4mono2ui4View18DisplayOrientationEv(void *o)
{
    return ((mono::ui::View *) o)->DisplayOrientation();
}

/* mono::ui::View::Size() */
extern "C"
mono::geo::Size & stub__ZN4mono2ui4View4SizeEv(void *o)
{
    return ((mono::ui::View *) o)->Size();
}

/* mono::ui::View::hide() */
extern "C"
void stub__ZN4mono2ui4View4hideEv(void *o)
{
    ((mono::ui::View *) o)->hide();
}

/* mono::ui::View::show() */
extern "C"
void stub__ZN4mono2ui4View4showEv(void *o)
{
    ((mono::ui::View *) o)->show();
}

/* mono::ui::View::setRect(geo::Rect) */
extern "C"
void stub__ZN4mono2ui4View7setRectENS_3geo4RectE(void *o, mono::geo::Rect a0)
{
    ((mono::ui::View *) o)->setRect(a0);
}

/* mono::ui::View::setSize(geo::Size) */
extern "C"
void stub__ZN4mono2ui4View7setSizeENS_3geo4SizeE(void *o, mono::geo::Size a0)
{
    ((mono::ui::View *) o)->setSize(a0);
}

/* mono::ui::View::Position() */
extern "C"
mono::geo::Point & stub__ZN4mono2ui4View8PositionEv(void *o)
{
    return ((mono::ui::View *) o)->Position();
}

/* mono::ui::View::~View() */
extern "C"
void stub__ZN4mono2ui4ViewD0Ev(void *o)
{
    ((mono::ui::View *) o)->~View();
}

/* mono::geo::Rect::Area() */
extern "C"
int stub__ZN4mono3geo4Rect4AreaEv(void *o)
{
    return ((mono::geo::Rect *) o)->Area();
}

/* mono::geo::Rect::Size() */
extern "C"
mono::geo::Size & stub__ZN4mono3geo4Rect4SizeEv(void *o)
{
    return ((mono::geo::Rect *) o)->Size();
}

/* mono::geo::Rect::setSize(class Size) */
extern "C"
void stub__ZN4mono3geo4Rect7setSizeENS0_4SizeE(void *o, mono::geo::Size a0)
{
    ((mono::geo::Rect *) o)->setSize(a0);
}

/* mono::geo::Rect::setPoint(class Point) */
extern "C"
void stub__ZN4mono3geo4Rect8setPointENS0_5PointE(void *o, mono::geo::Point a0)
{
    ((mono::geo::Rect *) o)->setPoint(a0);
}

/* mono::geo::Rect::Rect(const mono::geo::Rect &) */
extern "C"
void stub__ZN4mono3geo4RectC1ERKS1_(void *o, const mono::geo::Rect & a0)
{
    new(o) mono::geo::Rect(a0);
}

/* mono::geo::Rect::Rect(mono::geo::Point &, mono::geo::Size &) */
extern "C"
void stub__ZN4mono3geo4RectC1ERNS0_5PointERNS0_4SizeE(void *o, mono::geo::Point & a0, mono::geo::Size & a1)
{
    new(o) mono::geo::Rect(a0, a1);
}

/* mono::geo::Rect::Rect(int, int, int, int) */
extern "C"
void stub__ZN4mono3geo4RectC1Eiiii(void *o, int a0, int a1, int a2, int a3)
{
    new(o) mono::geo::Rect(a0, a1, a2, a3);
}

/* mono::geo::Rect::Rect() */
extern "C"
void stub__ZN4mono3geo4RectC1Ev(void *o)
{
    new(o) mono::geo::Rect();
}

/* mono::geo::Rect::operator=(mono::geo::Rect) */
extern "C"
mono::geo::Rect & stub__ZN4mono3geo4RectaSES1_(void *o, mono::geo::Rect a0)
{
    return ((mono::geo::Rect *) o)->operator=(a0);
}

/* mono::geo::Size::setWidth(int) */
extern "C"
void stub__ZN4mono3geo4Size8setWidthEi(void *o, int a0)
{
    ((mono::geo::Size *) o)->setWidth(a0);
}

/* mono::geo::Size::setHeight(int) */
extern "C"
void stub__ZN4mono3geo4Size9setHeightEi(void *o, int a0)
{
    ((mono::geo::Size *) o)->setHeight(a0);
}

/* mono::geo::Size::Size(const mono::geo::Size &) */
extern "C"
void stub__ZN4mono3geo4SizeC1ERKS1_(void *o, const mono::geo::Size & a0)
{
    new(o) mono::geo::Size(a0);
}

/* mono::geo::Size::Size(int, int) */
extern "C"
void stub__ZN4mono3geo4SizeC1Eii(void *o, int a0, int a1)
{
    new(o) mono::geo::Size(a0, a1);
}

/* mono::geo::Size::Size() */
extern "C"
void stub__ZN4mono3geo4SizeC1Ev(void *o)
{
    new(o) mono::geo::Size();
}

/* mono::geo::Size::operator=(const mono::geo::Size &) */
extern "C"
mono::geo::Size & stub__ZN4mono3geo4SizeaSERKS1_(void *o, const mono::geo::Size & a0)
{
    return ((mono::geo::Size *) o)->operator=(a0);
}

/* mono::geo::Point::setX(int) */
extern "C"
void stub__ZN4mono3geo5Point4setXEi(void *o, int a0)
{
    ((mono::geo::Point *) o)->setX(a0);
}

/* mono::geo::Point::setY(int) */
extern "C"
void stub__ZN4mono3geo5Point4setYEi(void *o, int a0)
{
    ((mono::geo::Point *) o)->setY(a0);
}

/* mono::geo::Point::appendX(int) */
extern "C"
void stub__ZN4mono3geo5Point7appendXEi(void *o, int a0)
{
    ((mono::geo::Point *) o)->appendX(a0);
}

/* mono::geo::Point::appendY(int) */
extern "C"
void stub__ZN4mono3geo5Point7appendYEi(void *o, int a0)
{
    ((mono::geo::Point *) o)->appendY(a0);
}

/* mono::geo::Point::Point(const mono::geo::Point &) */
extern "C"
void stub__ZN4mono3geo5PointC1ERKS1_(void *o, const mono::geo::Point & a0)
{
    new(o) mono::geo::Point(a0);
}

/* mono::geo::Point::Point(int, int) */
extern "C"
void stub__ZN4mono3geo5PointC1Eii(void *o, int a0, int a1)
{
    new(o) mono::geo::Point(a0, a1);
}

/* mono::geo::Point::Point() */
extern "C"
void stub__ZN4mono3geo5PointC1Ev(void *o)
{
    new(o) mono::geo::Point();
}

/* mono::geo::Point::operator=(const mono::geo::Point &) */
extern "C"
mono::geo::Point & stub__ZN4mono3geo5PointaSERKS1_(void *o, const mono::geo::Point & a0)
{
    return ((mono::geo::Point *) o)->operator=(a0);
}

/* mono::geo::Circle::Circle(mono::geo::Point, uint32_t) */
extern "C"
void stub__ZN4mono3geo6CircleC1ENS0_5PointEj(void *o, mono::geo::Point a0, unsigned int a1)
{
    new(o) mono::geo::Circle(a0, a1);
}

/* mono::geo::Circle::Circle(int, int, uint32_t) */
extern "C"
void stub__ZN4mono3geo6CircleC1Eiij(void *o, int a0, int a1, unsigned int a2)
{
    new(o) mono::geo::Circle(a0, a1, a2);
}

/* mono::geo::Circle::Circle() */
extern "C"
void stub__ZN4mono3geo6CircleC1Ev(void *o)
{
    new(o) mono::geo::Circle();
}

/* mono::String::preAllocbytes(uint32_t) */
extern "C"
void stub__ZN4mono6String13preAllocbytesEj(void *o, unsigned int a0)
{
    ((mono::String *) o)->preAllocbytes(a0);
}

/* mono::String::Format(const char *, ...) */
extern "C"
mono::String stub__ZN4mono6String6FormatEPKcz(void *o, const char * a0)
{
    return ((mono::String *) o)->Format(a0);
}

/* mono::String::String(const char *) */
extern "C"
void stub__ZN4mono6StringC1EPKc(void *o, const char * a0)
{
    new(o) mono::String(a0);
}

/* mono::String::String(char *) */
extern "C"
void stub__ZN4mono6StringC1EPc(void *o, char * a0)
{
    new(o) mono::String(a0);
}

/* mono::String::String(char *, uint32_t) */
extern "C"
void stub__ZN4mono6StringC1EPcj(void *o, char * a0, unsigned int a1)
{
    new(o) mono::String(a0, a1);
}

/* mono::String::String(const mono::String &) */
extern "C"
void stub__ZN4mono6StringC1ERKS0_(void *o, const mono::String & a0)
{
    new(o) mono::String(a0);
}

/* mono::String::String(uint32_t) */
extern "C"
void stub__ZN4mono6StringC1Ej(void *o, unsigned int a0)
{
    new(o) mono::String(a0);
}

/* mono::String::String() */
extern "C"
void stub__ZN4mono6StringC1Ev(void *o)
{
    new(o) mono::String();
}

/* mono::String::~String() */
extern "C"
void stub__ZN4mono6StringD0Ev(void *o)
{
    ((mono::String *) o)->~String();
}

/* mono::String::operator=(const char *) */
extern "C"
mono::String & stub__ZN4mono6StringaSEPKc(void *o, const char * a0)
{
    return ((mono::String *) o)->operator=(a0);
}

/* mono::String::operator=(const mono::String &) */
extern "C"
mono::String & stub__ZN4mono6StringaSERKS0_(void *o, const mono::String & a0)
{
    return ((mono::String *) o)->operator=(a0);
}

/* mono::String::operator==(const char *) */
extern "C"
bool stub__ZN4mono6StringeqEPKc(void *o, const char * a0)
{
    return ((mono::String *) o)->operator==(a0);
}

/* mono::String::operator==(const mono::String &) */
extern "C"
bool stub__ZN4mono6StringeqERKS0_(void *o, const mono::String & a0)
{
    return ((mono::String *) o)->operator==(a0);
}

/* mono::String::operator!=(const char *) */
extern "C"
bool stub__ZN4mono6StringneEPKc(void *o, const char * a0)
{
    return ((mono::String *) o)->operator!=(a0);
}

/* mono::String::operator!=(const mono::String &) */
extern "C"
bool stub__ZN4mono6StringneERKS0_(void *o, const mono::String & a0)
{
    return ((mono::String *) o)->operator!=(a0);
}

/* mono::display::Color::BytePointer() */
extern "C"
unsigned char * stub__ZN4mono7display5Color11BytePointerEv(void *o)
{
    return ((mono::display::Color *) o)->BytePointer();
}

/* mono::display::Color::Color(const mono::display::Color &) */
extern "C"
void stub__ZN4mono7display5ColorC1ERKS1_(void *o, const mono::display::Color & a0)
{
    new(o) mono::display::Color(a0);
}

/* mono::display::Color::Color(uint8_t, uint8_t, uint8_t) */
extern "C"
void stub__ZN4mono7display5ColorC1Ehhh(void *o, unsigned char a0, unsigned char a1, unsigned char a2)
{
    new(o) mono::display::Color(a0, a1, a2);
}

/* mono::display::Color::Color(const int) */
extern "C"
void stub__ZN4mono7display5ColorC1Ei(void *o, const int a0)
{
    new(o) mono::display::Color(a0);
}

/* mono::display::Color::Color() */
extern "C"
void stub__ZN4mono7display5ColorC1Ev(void *o)
{
    new(o) mono::display::Color();
}

/* mono::display::Color::operator=(mono::display::Color) */
extern "C"
unsigned short stub__ZN4mono7display5ColoraSES1_(void *o, mono::display::Color a0)
{
    return ((mono::display::Color *) o)->operator=(a0);
}

/* mono::display::Color::operator==(const mono::display::Color &) */
extern "C"
bool stub__ZN4mono7display5ColoreqERKS1_(void *o, const mono::display::Color & a0)
{
    return ((mono::display::Color *) o)->operator==(a0);
}

/* mono::display::Color::operator*(const mono::display::Color &) */
extern "C"
mono::display::Color stub__ZN4mono7display5ColormlERKS1_(void *o, const mono::display::Color & a0)
{
    return ((mono::display::Color *) o)->operator*(a0);
}

/* mono::display::Color::operator!=(const mono::display::Color &) */
extern "C"
bool stub__ZN4mono7display5ColorneERKS1_(void *o, const mono::display::Color & a0)
{
    return ((mono::display::Color *) o)->operator!=(a0);
}

/* mono::display::Color::operator+(const mono::display::Color &) */
extern "C"
mono::display::Color stub__ZN4mono7display5ColorplERKS1_(void *o, const mono::display::Color & a0)
{
    return ((mono::display::Color *) o)->operator+(a0);
}

/* mono::ui::TextLabelView::TextPixelWidth() */
extern "C"
unsigned short stub__ZNK4mono2ui13TextLabelView14TextPixelWidthEv(void *o)
{
    return ((mono::ui::TextLabelView *) o)->TextPixelWidth();
}

/* mono::ui::TextLabelView::TextPixelHeight() */
extern "C"
unsigned short stub__ZNK4mono2ui13TextLabelView15TextPixelHeightEv(void *o)
{
    return ((mono::ui::TextLabelView *) o)->TextPixelHeight();
}

/* mono::ui::TextLabelView::Font() */
extern "C"
const MonoFont & stub__ZNK4mono2ui13TextLabelView4FontEv(void *o)
{
    return ((mono::ui::TextLabelView *) o)->Font();
}

/* mono::ui::TextLabelView::TextSize() */
extern "C"
unsigned char stub__ZNK4mono2ui13TextLabelView8TextSizeEv(void *o)
{
    return ((mono::ui::TextLabelView *) o)->TextSize();
}

/* mono::ui::TextLabelView::Alignment() */
extern "C"
mono::ui::TextLabelView::TextAlignment stub__ZNK4mono2ui13TextLabelView9AlignmentEv(void *o)
{
    return ((mono::ui::TextLabelView *) o)->Alignment();
}

/* mono::ui::TextLabelView::TextColor() */
extern "C"
mono::display::Color stub__ZNK4mono2ui13TextLabelView9TextColorEv(void *o)
{
    return ((mono::ui::TextLabelView *) o)->TextColor();
}

/* mono::ui::View::Visible() */
extern "C"
bool stub__ZNK4mono2ui4View7VisibleEv(void *o)
{
    return ((mono::ui::View *) o)->Visible();
}

/* mono::ui::View::ViewRect() */
extern "C"
const mono::geo::Rect & stub__ZNK4mono2ui4View8ViewRectEv(void *o)
{
    return ((mono::ui::View *) o)->ViewRect();
}

/* mono::geo::Rect::LowerRight() */
extern "C"
mono::geo::Point stub__ZNK4mono3geo4Rect10LowerRightEv(void *o)
{
    return ((mono::geo::Rect *) o)->LowerRight();
}

/* mono::geo::Rect::UpperRight() */
extern "C"
mono::geo::Point stub__ZNK4mono3geo4Rect10UpperRightEv(void *o)
{
    return ((mono::geo::Rect *) o)->UpperRight();
}

/* mono::geo::Rect::X2() */
extern "C"
int stub__ZNK4mono3geo4Rect2X2Ev(void *o)
{
    return ((mono::geo::Rect *) o)->X2();
}

/* mono::geo::Rect::Y2() */
extern "C"
int stub__ZNK4mono3geo4Rect2Y2Ev(void *o)
{
    return ((mono::geo::Rect *) o)->Y2();
}

/* mono::geo::Rect::crop(const mono::geo::Rect &) */
extern "C"
mono::geo::Rect stub__ZNK4mono3geo4Rect4cropERKS1_(void *o, const mono::geo::Rect & a0)
{
    return ((mono::geo::Rect *) o)->crop(a0);
}

/* mono::geo::Rect::Point() */
extern "C"
mono::geo::Point stub__ZNK4mono3geo4Rect5PointEv(void *o)
{
    return ((mono::geo::Rect *) o)->Point();
}

/* mono::geo::Rect::Center() */
extern "C"
mono::geo::Point stub__ZNK4mono3geo4Rect6CenterEv(void *o)
{
    return ((mono::geo::Rect *) o)->Center();
}

/* mono::geo::Rect::ToString() */
extern "C"
mono::String stub__ZNK4mono3geo4Rect8ToStringEv(void *o)
{
    return ((mono::geo::Rect *) o)->ToString();
}

/* mono::geo::Rect::contains(const mono::geo::Rect &) */
extern "C"
bool stub__ZNK4mono3geo4Rect8containsERKS1_(void *o, const mono::geo::Rect & a0)
{
    return ((mono::geo::Rect *) o)->contains(a0);
}

/* mono::geo::Rect::contains(class Point &) */
extern "C"
bool stub__ZNK4mono3geo4Rect8containsERNS0_5PointE(void *o, mono::geo::Point & a0)
{
    return ((mono::geo::Rect *) o)->contains(a0);
}

/* mono::geo::Rect::LowerLeft() */
extern "C"
mono::geo::Point stub__ZNK4mono3geo4Rect9LowerLeftEv(void *o)
{
    return ((mono::geo::Rect *) o)->LowerLeft();
}

/* mono::geo::Rect::UpperLeft() */
extern "C"
mono::geo::Point stub__ZNK4mono3geo4Rect9UpperLeftEv(void *o)
{
    return ((mono::geo::Rect *) o)->UpperLeft();
}

/* mono::geo::Size::Width() */
extern "C"
int stub__ZNK4mono3geo4Size5WidthEv(void *o)
{
    return ((mono::geo::Size *) o)->Width();
}

/* mono::geo::Size::Height() */
extern "C"
int stub__ZNK4mono3geo4Size6HeightEv(void *o)
{
    return ((mono::geo::Size *) o)->Height();
}

/* mono::geo::Point::X() */
extern "C"
int stub__ZNK4mono3geo5Point1XEv(void *o)
{
    return ((mono::geo::Point *) o)->X();
}

/* mono::geo::Point::Y() */
extern "C"
int stub__ZNK4mono3geo5Point1YEv(void *o)
{
    return ((mono::geo::Point *) o)->Y();
}

/* mono::geo::Point::Abs() */
extern "C"
unsigned int stub__ZNK4mono3geo5Point3AbsEv(void *o)
{
    return ((mono::geo::Point *) o)->Abs();
}

/* mono::geo::Point::toString() */
extern "C"
mono::String stub__ZNK4mono3geo5Point8toStringEv(void *o)
{
    return ((mono::geo::Point *) o)->toString();
}

/* mono::geo::Circle::Radius() */
extern "C"
unsigned int stub__ZNK4mono3geo6Circle6RadiusEv(void *o)
{
    return ((mono::geo::Circle *) o)->Radius();
}

/* mono::String::Length() */
extern "C"
unsigned int stub__ZNK4mono6String6LengthEv(void *o)
{
    return ((mono::String *) o)->Length();
}

/* mono::String::operator()() */
extern "C"
char * stub__ZNK4mono6StringclEv(void *o)
{
    return ((mono::String *) o)->operator()();
}

/* mono::String::operator[](uint32_t) */
extern "C"
char stub__ZNK4mono6StringixEj(void *o, unsigned int a0)
{
    return ((mono::String *) o)->operator[](a0);
}

/* mono::display::Color::alphaBlend(uint8_t, const mono::display::Color &) */
extern "C"
mono::display::Color stub__ZNK4mono7display5Color10alphaBlendEhRKS1_(void *o, unsigned char a0, const mono::display::Color & a1)
{
    return ((mono::display::Color *) o)->alphaBlend(a0, a1);
}

/* mono::display::Color::blendAdditive(mono::display::Color) */
extern "C"
mono::display::Color stub__ZNK4mono7display5Color13blendAdditiveES1_(void *o, mono::display::Color a0)
{
    return ((mono::display::Color *) o)->blendAdditive(a0);
}

/* mono::display::Color::blendMultiply(mono::display::Color) */
extern "C"
mono::display::Color stub__ZNK4mono7display5Color13blendMultiplyES1_(void *o, mono::display::Color a0)
{
    return ((mono::display::Color *) o)->blendMultiply(a0);
}

/* mono::display::Color::Red() */
extern "C"
unsigned char stub__ZNK4mono7display5Color3RedEv(void *o)
{
    return ((mono::display::Color *) o)->Red();
}

/* mono::display::Color::Blue() */
extern "C"
unsigned char stub__ZNK4mono7display5Color4BlueEv(void *o)
{
    return ((mono::display::Color *) o)->Blue();
}

/* mono::display::Color::Green() */
extern "C"
unsigned char stub__ZNK4mono7display5Color5GreenEv(void *o)
{
    return ((mono::display::Color *) o)->Green();
}

/* mono::display::Color::scale(uint8_t) */
extern "C"
mono::display::Color stub__ZNK4mono7display5Color5scaleEh(void *o, unsigned char a0)
{
    return ((mono::display::Color *) o)->scale(a0);
}

/* mono::display::Color::invert() */
extern "C"
mono::display::Color stub__ZNK4mono7display5Color6invertEv(void *o)
{
    return ((mono::display::Color *) o)->invert();
}

/* mono::display::Color::toString() */
extern "C"
mono::String stub__ZNK4mono7display5Color8toStringEv(void *o)
{
    return ((mono::display::Color *) o)->toString();
}
