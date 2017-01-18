#include <new>
#include "consoles.h"
#include "application_context_interface.h"
#include "display/ui/graph_view.h"
#include "rect.h"
#include "touch_event.h"
#include "size.h"
#include "circle.h"
#include "display/ui/abstract_button_view.h"
#include "point.h"
#include "display_painter.h"
#include "display/ui/on_off_button_view.h"
#include "media/image.h"
#include "display/ui/text_label_view.h"
#include "display/ui/image_view.h"
#include "queue_interrupt.h"
#include "touch_system_interface.h"
#include "mn_digital_out.h"
#include "mn_timer.h"
#include "queue.h"
#include "application_controller_interface.h"
#include "display/ui/view.h"
#include "display/ui/button_view.h"
#include "display/ui/responder_view.h"
#include "display/ui/background_view.h"
#include "display/ui/animator.h"
#include "display/ui/touch_calibrator.h"
#include "application_run_loop_task_interface.h"
#include "async.h"
#include "mn_string.h"
#include "display/font_interface.h"
#include "media/bmp_image.h"
#include "display/ui/progress_bar_view.h"
#include "color.h"
#include "display/ui/status_indicator_view.h"
#include "display/ui/console_view.h"

/* mbed::DigitalOut::is_connected() */
extern "C"
int stub__ZN4mbed10DigitalOut12is_connectedEv(void *o)
{
    return ((mbed::DigitalOut *) o)->is_connected();
}

/* mbed::DigitalOut::read() */
extern "C"
int stub__ZN4mbed10DigitalOut4readEv(void *o)
{
    return ((mbed::DigitalOut *) o)->read();
}

/* mbed::DigitalOut::write(int) */
extern "C"
void stub__ZN4mbed10DigitalOut5writeEi(void *o, int a0)
{
    ((mbed::DigitalOut *) o)->write(a0);
}

/* mbed::DigitalOut::operator=(mbed::DigitalOut &) */
extern "C"
mbed::DigitalOut & stub__ZN4mbed10DigitalOutaSERS0_(void *o, mbed::DigitalOut & a0)
{
    return ((mbed::DigitalOut *) o)->operator=(a0);
}

/* mbed::InterruptIn::enable_irq() */
extern "C"
void stub__ZN4mbed11InterruptIn10enable_irqEv(void *o)
{
    ((mbed::InterruptIn *) o)->enable_irq();
}

/* mbed::InterruptIn::disable_irq() */
extern "C"
void stub__ZN4mbed11InterruptIn11disable_irqEv(void *o)
{
    ((mbed::InterruptIn *) o)->disable_irq();
}

/* mbed::InterruptIn::_irq_handler(uint32_t, gpio_irq_event) */
extern "C"
void stub__ZN4mbed11InterruptIn12_irq_handlerEj14gpio_irq_event(void *o, unsigned int a0, gpio_irq_event a1)
{
    ((mbed::InterruptIn *) o)->_irq_handler(a0, a1);
}

/* mbed::InterruptIn::mode(PinMode) */
extern "C"
void stub__ZN4mbed11InterruptIn4modeE7PinMode(void *o, PinMode a0)
{
    ((mbed::InterruptIn *) o)->mode(a0);
}

/* mbed::InterruptIn::read() */
extern "C"
int stub__ZN4mbed11InterruptIn4readEv(void *o)
{
    return ((mbed::InterruptIn *) o)->read();
}

/* mono::TouchEvent::TouchEvent(mono::TouchEvent::TouchEventType, geo::Point &, mono::ITouchSystem *) */
extern "C"
void stub__ZN4mono10TouchEventC1ENS0_14TouchEventTypeERNS_3geo5PointEPNS_12ITouchSystemE(void *o, mono::TouchEvent::TouchEventType a0, mono::geo::Point & a1, mono::ITouchSystem * a2)
{
    new(o) mono::TouchEvent(a0, a1, a2);
}

/* mono::TouchEvent::TouchEvent(const mono::TouchEvent &) */
extern "C"
void stub__ZN4mono10TouchEventC1ERKS0_(void *o, const mono::TouchEvent & a0)
{
    new(o) mono::TouchEvent(a0);
}

/* mono::TouchEvent::TouchEvent() */
extern "C"
void stub__ZN4mono10TouchEventC1Ev(void *o)
{
    new(o) mono::TouchEvent();
}

/* mono::TouchEvent::operator=(const mono::TouchEvent &) */
extern "C"
mono::TouchEvent & stub__ZN4mono10TouchEventaSERKS0_(void *o, const mono::TouchEvent & a0)
{
    return ((mono::TouchEvent *) o)->operator=(a0);
}

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

/* mono::ITouchSystem::setCalibration(TouchCalibration &) */
extern "C"
void stub__ZN4mono12ITouchSystem14setCalibrationERNS_3geo4RectE(void *o, mono::geo::Rect & a0)
{
    ((mono::ITouchSystem *) o)->setCalibration(a0);
}

/* mono::ITouchSystem::ToScreenCoordsX(int, uint16_t) */
extern "C"
int stub__ZN4mono12ITouchSystem15ToScreenCoordsXEit(void *o, int a0, unsigned short a1)
{
    return ((mono::ITouchSystem *) o)->ToScreenCoordsX(a0, a1);
}

/* mono::ITouchSystem::ToScreenCoordsY(int, uint16_t) */
extern "C"
int stub__ZN4mono12ITouchSystem15ToScreenCoordsYEit(void *o, int a0, unsigned short a1)
{
    return ((mono::ITouchSystem *) o)->ToScreenCoordsY(a0, a1);
}

/* mono::ITouchSystem::processTouchInput() */
extern "C"
void stub__ZN4mono12ITouchSystem17processTouchInputEv(void *o)
{
    ((mono::ITouchSystem *) o)->processTouchInput();
}

/* mono::ITouchSystem::CurrentCalibration() */
extern "C"
mono::geo::Rect stub__ZN4mono12ITouchSystem18CurrentCalibrationEv(void *o)
{
    return ((mono::ITouchSystem *) o)->CurrentCalibration();
}

/* mono::ITouchSystem::init() */
extern "C"
void stub__ZN4mono12ITouchSystem4initEv(void *o)
{
    ((mono::ITouchSystem *) o)->init();
}

/* mono::QueueInterrupt::FallTimeStamp() */
extern "C"
unsigned int stub__ZN4mono14QueueInterrupt13FallTimeStampEv(void *o)
{
    return ((mono::QueueInterrupt *) o)->FallTimeStamp();
}

/* mono::QueueInterrupt::RiseTimeStamp() */
extern "C"
unsigned int stub__ZN4mono14QueueInterrupt13RiseTimeStampEv(void *o)
{
    return ((mono::QueueInterrupt *) o)->RiseTimeStamp();
}

/* mono::QueueInterrupt::setDebouncing(bool) */
extern "C"
void stub__ZN4mono14QueueInterrupt13setDebouncingEb(void *o, bool a0)
{
    ((mono::QueueInterrupt *) o)->setDebouncing(a0);
}

/* mono::QueueInterrupt::setDebounceTimeout(int) */
extern "C"
void stub__ZN4mono14QueueInterrupt18setDebounceTimeoutEi(void *o, int a0)
{
    ((mono::QueueInterrupt *) o)->setDebounceTimeout(a0);
}

/* mono::QueueInterrupt::DeactivateUntilHandled(bool) */
extern "C"
void stub__ZN4mono14QueueInterrupt22DeactivateUntilHandledEb(void *o, bool a0)
{
    ((mono::QueueInterrupt *) o)->DeactivateUntilHandled(a0);
}

/* mono::QueueInterrupt::fall(void (*)()) */
extern "C"
void stub__ZN4mono14QueueInterrupt4fallEPFvvE(void *o, void (*a0)())
{
    ((mono::QueueInterrupt *) o)->fall(a0);
}

/* mono::QueueInterrupt::rise(void (*)()) */
extern "C"
void stub__ZN4mono14QueueInterrupt4riseEPFvvE(void *o, void (*a0)())
{
    ((mono::QueueInterrupt *) o)->rise(a0);
}

/* mono::QueueInterrupt::Snapshot() */
extern "C"
bool stub__ZN4mono14QueueInterrupt8SnapshotEv(void *o)
{
    return ((mono::QueueInterrupt *) o)->Snapshot();
}

/* mono::QueueInterrupt::QueueInterrupt(PinName, PinMode) */
extern "C"
void stub__ZN4mono14QueueInterruptC1E7PinName7PinMode(void *o, PinName a0, PinMode a1)
{
    new(o) mono::QueueInterrupt(a0, a1);
}

/* mono::QueueInterrupt::~QueueInterrupt() */
extern "C"
void stub__ZN4mono14QueueInterruptD0Ev(void *o)
{
    ((mono::QueueInterrupt *) o)->~QueueInterrupt();
}

/* mono::TouchResponder::Deactivate() */
extern "C"
void stub__ZN4mono14TouchResponder10DeactivateEv(void *o)
{
    ((mono::TouchResponder *) o)->Deactivate();
}

/* mono::TouchResponder::RespondTouchEnd(mono::TouchEvent &) */
extern "C"
void stub__ZN4mono14TouchResponder15RespondTouchEndERNS_10TouchEventE(void *o, mono::TouchEvent & a0)
{
    ((mono::TouchResponder *) o)->RespondTouchEnd(a0);
}

/* mono::TouchResponder::RespondTouchMove(mono::TouchEvent &) */
extern "C"
void stub__ZN4mono14TouchResponder16RespondTouchMoveERNS_10TouchEventE(void *o, mono::TouchEvent & a0)
{
    ((mono::TouchResponder *) o)->RespondTouchMove(a0);
}

/* mono::TouchResponder::RespondTouchBegin(mono::TouchEvent &) */
extern "C"
void stub__ZN4mono14TouchResponder17RespondTouchBeginERNS_10TouchEventE(void *o, mono::TouchEvent & a0)
{
    ((mono::TouchResponder *) o)->RespondTouchBegin(a0);
}

/* mono::TouchResponder::RunResponderChainTouchEnd(mono::TouchEvent &) */
extern "C"
void stub__ZN4mono14TouchResponder25RunResponderChainTouchEndERNS_10TouchEventE(void *o, mono::TouchEvent & a0)
{
    ((mono::TouchResponder *) o)->RunResponderChainTouchEnd(a0);
}

/* mono::TouchResponder::RunResponderChainTouchMove(mono::TouchEvent &) */
extern "C"
void stub__ZN4mono14TouchResponder26RunResponderChainTouchMoveERNS_10TouchEventE(void *o, mono::TouchEvent & a0)
{
    ((mono::TouchResponder *) o)->RunResponderChainTouchMove(a0);
}

/* mono::TouchResponder::RunResponderChainTouchBegin(mono::TouchEvent &) */
extern "C"
void stub__ZN4mono14TouchResponder27RunResponderChainTouchBeginERNS_10TouchEventE(void *o, mono::TouchEvent & a0)
{
    ((mono::TouchResponder *) o)->RunResponderChainTouchBegin(a0);
}

/* mono::TouchResponder::Activate() */
extern "C"
void stub__ZN4mono14TouchResponder8ActivateEv(void *o)
{
    ((mono::TouchResponder *) o)->Activate();
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

/* mono::io::DigitalOut::setMode(PinMode) */
extern "C"
void stub__ZN4mono2io10DigitalOut7setModeE7PinMode(void *o, PinMode a0)
{
    ((mono::io::DigitalOut *) o)->setMode(a0);
}

/* mono::io::DigitalOut::DigitalOut(PinName) */
extern "C"
void stub__ZN4mono2io10DigitalOutC1E7PinName(void *o, PinName a0)
{
    new(o) mono::io::DigitalOut(a0);
}

/* mono::io::DigitalOut::DigitalOut(PinName, int) */
extern "C"
void stub__ZN4mono2io10DigitalOutC1E7PinNamei(void *o, PinName a0, int a1)
{
    new(o) mono::io::DigitalOut(a0, a1);
}

/* mono::io::DigitalOut::DigitalOut(PinName, int, PinMode) */
extern "C"
void stub__ZN4mono2io10DigitalOutC1E7PinNamei7PinMode(void *o, PinName a0, int a1, PinMode a2)
{
    new(o) mono::io::DigitalOut(a0, a1, a2);
}

/* mono::io::DigitalOut::operator=(mono::io::DigitalOut &) */
extern "C"
mono::io::DigitalOut & stub__ZN4mono2io10DigitalOutaSERS1_(void *o, mono::io::DigitalOut & a0)
{
    return ((mono::io::DigitalOut *) o)->operator=(a0);
}

/* mono::io::DigitalOut::operator=(int) */
extern "C"
mono::io::DigitalOut & stub__ZN4mono2io10DigitalOutaSEi(void *o, int a0)
{
    return ((mono::io::DigitalOut *) o)->operator=(a0);
}

/* mono::ui::ButtonView::setHighlight(mono::display::Color) */
extern "C"
void stub__ZN4mono2ui10ButtonView12setHighlightENS_7display5ColorE(void *o, mono::display::Color a0)
{
    ((mono::ui::ButtonView *) o)->setHighlight(a0);
}

/* mono::ui::ButtonView::setBackground(mono::display::Color) */
extern "C"
void stub__ZN4mono2ui10ButtonView13setBackgroundENS_7display5ColorE(void *o, mono::display::Color a0)
{
    ((mono::ui::ButtonView *) o)->setBackground(a0);
}

/* mono::ui::ButtonView::setClickCallback(void (*)()) */
extern "C"
void stub__ZN4mono2ui10ButtonView16setClickCallbackEPFvvE(void *o, void (*a0)())
{
    ((mono::ui::ButtonView *) o)->setClickCallback(a0);
}

/* mono::ui::ButtonView::repaint() */
extern "C"
void stub__ZN4mono2ui10ButtonView7repaintEv(void *o)
{
    ((mono::ui::ButtonView *) o)->repaint();
}

/* mono::ui::ButtonView::setFont(const MonoFont &) */
extern "C"
void stub__ZN4mono2ui10ButtonView7setFontERK8MonoFont(void *o, const MonoFont & a0)
{
    ((mono::ui::ButtonView *) o)->setFont(a0);
}

/* mono::ui::ButtonView::setText(mono::String) */
extern "C"
void stub__ZN4mono2ui10ButtonView7setTextENS_6StringE(void *o, mono::String a0)
{
    ((mono::ui::ButtonView *) o)->setText(a0);
}

/* mono::ui::ButtonView::setBorder(mono::display::Color) */
extern "C"
void stub__ZN4mono2ui10ButtonView9setBorderENS_7display5ColorE(void *o, mono::display::Color a0)
{
    ((mono::ui::ButtonView *) o)->setBorder(a0);
}

/* mono::ui::ButtonView::ButtonView(geo::Rect, mono::String) */
extern "C"
void stub__ZN4mono2ui10ButtonViewC1ENS_3geo4RectENS_6StringE(void *o, mono::geo::Rect a0, mono::String a1)
{
    new(o) mono::ui::ButtonView(a0, a1);
}

/* mono::ui::ButtonView::ButtonView(geo::Rect, const char *) */
extern "C"
void stub__ZN4mono2ui10ButtonViewC1ENS_3geo4RectEPKc(void *o, mono::geo::Rect a0, const char * a1)
{
    new(o) mono::ui::ButtonView(a0, a1);
}

/* mono::ui::ButtonView::ButtonView() */
extern "C"
void stub__ZN4mono2ui10ButtonViewC1Ev(void *o)
{
    new(o) mono::ui::ButtonView();
}

/* mono::ui::ResponderView::hide() */
extern "C"
void stub__ZN4mono2ui13ResponderView4hideEv(void *o)
{
    ((mono::ui::ResponderView *) o)->hide();
}

/* mono::ui::ResponderView::show() */
extern "C"
void stub__ZN4mono2ui13ResponderView4showEv(void *o)
{
    ((mono::ui::ResponderView *) o)->show();
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

/* mono::ui::TextLabelView::TextLabelView(geo::Rect, mono::String) */
extern "C"
void stub__ZN4mono2ui13TextLabelViewC1ENS_3geo4RectENS_6StringE(void *o, mono::geo::Rect a0, mono::String a1)
{
    new(o) mono::ui::TextLabelView(a0, a1);
}

/* mono::ui::TextLabelView::TextLabelView(geo::Rect, const char *) */
extern "C"
void stub__ZN4mono2ui13TextLabelViewC1ENS_3geo4RectEPKc(void *o, mono::geo::Rect a0, const char * a1)
{
    new(o) mono::ui::TextLabelView(a0, a1);
}

/* mono::ui::TextLabelView::TextLabelView(mono::String) */
extern "C"
void stub__ZN4mono2ui13TextLabelViewC1ENS_6StringE(void *o, mono::String a0)
{
    new(o) mono::ui::TextLabelView(a0);
}

/* mono::ui::TextLabelView::TextLabelView(const char *) */
extern "C"
void stub__ZN4mono2ui13TextLabelViewC1EPKc(void *o, const char * a0)
{
    new(o) mono::ui::TextLabelView(a0);
}

/* mono::ui::BackgroundView::setBackgroundColor(display::Color) */
extern "C"
void stub__ZN4mono2ui14BackgroundView18setBackgroundColorENS_7display5ColorE(void *o, mono::display::Color a0)
{
    ((mono::ui::BackgroundView *) o)->setBackgroundColor(a0);
}

/* mono::ui::BackgroundView::repaint() */
extern "C"
void stub__ZN4mono2ui14BackgroundView7repaintEv(void *o)
{
    ((mono::ui::BackgroundView *) o)->repaint();
}

/* mono::ui::BackgroundView::BackgroundView(display::Color) */
extern "C"
void stub__ZN4mono2ui14BackgroundViewC1ENS_7display5ColorE(void *o, mono::display::Color a0)
{
    new(o) mono::ui::BackgroundView(a0);
}

/* mono::ui::OnOffButtonView::setHighlight(display::Color) */
extern "C"
void stub__ZN4mono2ui15OnOffButtonView12setHighlightENS_7display5ColorE(void *o, mono::display::Color a0)
{
    ((mono::ui::OnOffButtonView *) o)->setHighlight(a0);
}

/* mono::ui::OnOffButtonView::setBackground(display::Color) */
extern "C"
void stub__ZN4mono2ui15OnOffButtonView13setBackgroundENS_7display5ColorE(void *o, mono::display::Color a0)
{
    ((mono::ui::OnOffButtonView *) o)->setBackground(a0);
}

/* mono::ui::OnOffButtonView::repaint() */
extern "C"
void stub__ZN4mono2ui15OnOffButtonView7repaintEv(void *o)
{
    ((mono::ui::OnOffButtonView *) o)->repaint();
}

/* mono::ui::OnOffButtonView::setBorder(display::Color) */
extern "C"
void stub__ZN4mono2ui15OnOffButtonView9setBorderENS_7display5ColorE(void *o, mono::display::Color a0)
{
    ((mono::ui::OnOffButtonView *) o)->setBorder(a0);
}

/* mono::ui::OnOffButtonView::OnOffButtonView(geo::Rect) */
extern "C"
void stub__ZN4mono2ui15OnOffButtonViewC1ENS_3geo4RectE(void *o, mono::geo::Rect a0)
{
    new(o) mono::ui::OnOffButtonView(a0);
}

/* mono::ui::OnOffButtonView::OnOffButtonView() */
extern "C"
void stub__ZN4mono2ui15OnOffButtonViewC1Ev(void *o)
{
    new(o) mono::ui::OnOffButtonView();
}

/* mono::ui::ProgressBarView::setMaximum(int) */
extern "C"
void stub__ZN4mono2ui15ProgressBarView10setMaximumEi(void *o, int a0)
{
    ((mono::ui::ProgressBarView *) o)->setMaximum(a0);
}

/* mono::ui::ProgressBarView::setMinimum(int) */
extern "C"
void stub__ZN4mono2ui15ProgressBarView10setMinimumEi(void *o, int a0)
{
    ((mono::ui::ProgressBarView *) o)->setMinimum(a0);
}

/* mono::ui::ProgressBarView::setValue(int) */
extern "C"
void stub__ZN4mono2ui15ProgressBarView8setValueEi(void *o, int a0)
{
    ((mono::ui::ProgressBarView *) o)->setValue(a0);
}

/* mono::ui::ProgressBarView::ProgressBarView(geo::Rect) */
extern "C"
void stub__ZN4mono2ui15ProgressBarViewC1ENS_3geo4RectE(void *o, mono::geo::Rect a0)
{
    new(o) mono::ui::ProgressBarView(a0);
}

/* mono::ui::ProgressBarView::ProgressBarView() */
extern "C"
void stub__ZN4mono2ui15ProgressBarViewC1Ev(void *o)
{
    new(o) mono::ui::ProgressBarView();
}

/* mono::ui::AbstractButtonView::setClickCallback(void (*)()) */
extern "C"
void stub__ZN4mono2ui18AbstractButtonView16setClickCallbackEPFvvE(void *o, void (*a0)())
{
    ((mono::ui::AbstractButtonView *) o)->setClickCallback(a0);
}

/* mono::ui::TouchCalibrateView::RespondTouchBegin(mono::TouchEvent &) */
extern "C"
void stub__ZN4mono2ui18TouchCalibrateView17RespondTouchBeginERNS_10TouchEventE(void *o, mono::TouchEvent & a0)
{
    ((mono::ui::TouchCalibrateView *) o)->RespondTouchBegin(a0);
}

/* mono::ui::TouchCalibrateView::StartNewCalibration() */
extern "C"
void stub__ZN4mono2ui18TouchCalibrateView19StartNewCalibrationEv(void *o)
{
    ((mono::ui::TouchCalibrateView *) o)->StartNewCalibration();
}

/* mono::ui::TouchCalibrateView::show() */
extern "C"
void stub__ZN4mono2ui18TouchCalibrateView4showEv(void *o)
{
    ((mono::ui::TouchCalibrateView *) o)->show();
}

/* mono::ui::TouchCalibrateView::TouchCalibrateView(geo::Rect) */
extern "C"
void stub__ZN4mono2ui18TouchCalibrateViewC1ENS_3geo4RectE(void *o, mono::geo::Rect a0)
{
    new(o) mono::ui::TouchCalibrateView(a0);
}

/* mono::ui::TouchCalibrateView::TouchCalibrateView() */
extern "C"
void stub__ZN4mono2ui18TouchCalibrateViewC1Ev(void *o)
{
    new(o) mono::ui::TouchCalibrateView();
}

/* mono::ui::StatusIndicatorView::setOnStateColor(display::Color) */
extern "C"
void stub__ZN4mono2ui19StatusIndicatorView15setOnStateColorENS_7display5ColorE(void *o, mono::display::Color a0)
{
    ((mono::ui::StatusIndicatorView *) o)->setOnStateColor(a0);
}

/* mono::ui::StatusIndicatorView::setOffStateColor(display::Color) */
extern "C"
void stub__ZN4mono2ui19StatusIndicatorView16setOffStateColorENS_7display5ColorE(void *o, mono::display::Color a0)
{
    ((mono::ui::StatusIndicatorView *) o)->setOffStateColor(a0);
}

/* mono::ui::StatusIndicatorView::setState(bool) */
extern "C"
void stub__ZN4mono2ui19StatusIndicatorView8setStateEb(void *o, bool a0)
{
    ((mono::ui::StatusIndicatorView *) o)->setState(a0);
}

/* mono::ui::StatusIndicatorView::StatusIndicatorView(geo::Rect) */
extern "C"
void stub__ZN4mono2ui19StatusIndicatorViewC1ENS_3geo4RectE(void *o, mono::geo::Rect a0)
{
    new(o) mono::ui::StatusIndicatorView(a0);
}

/* mono::ui::StatusIndicatorView::StatusIndicatorView(geo::Rect, bool) */
extern "C"
void stub__ZN4mono2ui19StatusIndicatorViewC1ENS_3geo4RectEb(void *o, mono::geo::Rect a0, bool a1)
{
    new(o) mono::ui::StatusIndicatorView(a0, a1);
}

/* mono::ui::StatusIndicatorView::StatusIndicatorView() */
extern "C"
void stub__ZN4mono2ui19StatusIndicatorViewC1Ev(void *o)
{
    new(o) mono::ui::StatusIndicatorView();
}

/* mono::ui::IGraphViewDataSource::BufferLenght() */
extern "C"
int stub__ZN4mono2ui20IGraphViewDataSource12BufferLenghtEv(void *o)
{
    return ((mono::ui::IGraphViewDataSource *) o)->BufferLenght();
}

/* mono::ui::IGraphViewDataSource::NewestSampleIndex() */
extern "C"
int stub__ZN4mono2ui20IGraphViewDataSource17NewestSampleIndexEv(void *o)
{
    return ((mono::ui::IGraphViewDataSource *) o)->NewestSampleIndex();
}

/* mono::ui::IGraphViewDataSource::MaxSampleValueSpan() */
extern "C"
int stub__ZN4mono2ui20IGraphViewDataSource18MaxSampleValueSpanEv(void *o)
{
    return ((mono::ui::IGraphViewDataSource *) o)->MaxSampleValueSpan();
}

/* mono::ui::IGraphViewDataSource::DataPoint(int) */
extern "C"
int stub__ZN4mono2ui20IGraphViewDataSource9DataPointEi(void *o, int a0)
{
    return ((mono::ui::IGraphViewDataSource *) o)->DataPoint(a0);
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

/* mono::ui::Animator::setMoveVector(geo::Point) */
extern "C"
void stub__ZN4mono2ui8Animator13setMoveVectorENS_3geo5PointE(void *o, mono::geo::Point a0)
{
    ((mono::ui::Animator *) o)->setMoveVector(a0);
}

/* mono::ui::Animator::setDestination(geo::Point) */
extern "C"
void stub__ZN4mono2ui8Animator14setDestinationENS_3geo5PointE(void *o, mono::geo::Point a0)
{
    ((mono::ui::Animator *) o)->setDestination(a0);
}

/* mono::ui::Animator::Pause() */
extern "C"
void stub__ZN4mono2ui8Animator5PauseEv(void *o)
{
    ((mono::ui::Animator *) o)->Pause();
}

/* mono::ui::Animator::Reset() */
extern "C"
void stub__ZN4mono2ui8Animator5ResetEv(void *o)
{
    ((mono::ui::Animator *) o)->Reset();
}

/* mono::ui::Animator::Start() */
extern "C"
void stub__ZN4mono2ui8Animator5StartEv(void *o)
{
    ((mono::ui::Animator *) o)->Start();
}

/* mono::ui::Animator::Animator(mono::ui::View *) */
extern "C"
void stub__ZN4mono2ui8AnimatorC1EPNS0_4ViewE(void *o, mono::ui::View * a0)
{
    new(o) mono::ui::Animator(a0);
}

/* mono::ui::GraphView::setDataSource(const mono::ui::IGraphViewDataSource &) */
extern "C"
void stub__ZN4mono2ui9GraphView13setDataSourceERKNS0_20IGraphViewDataSourceE(void *o, const mono::ui::IGraphViewDataSource & a0)
{
    ((mono::ui::GraphView *) o)->setDataSource(a0);
}

/* mono::ui::GraphView::setCursorActive(bool) */
extern "C"
void stub__ZN4mono2ui9GraphView15setCursorActiveEb(void *o, bool a0)
{
    ((mono::ui::GraphView *) o)->setCursorActive(a0);
}

/* mono::ui::GraphView::GraphView(geo::Rect) */
extern "C"
void stub__ZN4mono2ui9GraphViewC1ENS_3geo4RectE(void *o, mono::geo::Rect a0)
{
    new(o) mono::ui::GraphView(a0);
}

/* mono::ui::GraphView::GraphView(geo::Rect, const mono::ui::IGraphViewDataSource &) */
extern "C"
void stub__ZN4mono2ui9GraphViewC1ENS_3geo4RectERKNS0_20IGraphViewDataSourceE(void *o, mono::geo::Rect a0, const mono::ui::IGraphViewDataSource & a1)
{
    new(o) mono::ui::GraphView(a0, a1);
}

/* mono::ui::GraphView::GraphView() */
extern "C"
void stub__ZN4mono2ui9GraphViewC1Ev(void *o)
{
    new(o) mono::ui::GraphView();
}

/* mono::ui::ImageView::repaint() */
extern "C"
void stub__ZN4mono2ui9ImageView7repaintEv(void *o)
{
    ((mono::ui::ImageView *) o)->repaint();
}

/* mono::ui::ImageView::setCrop(geo::Rect) */
extern "C"
void stub__ZN4mono2ui9ImageView7setCropENS_3geo4RectE(void *o, mono::geo::Rect a0)
{
    ((mono::ui::ImageView *) o)->setCrop(a0);
}

/* mono::ui::ImageView::setCrop(geo::Size) */
extern "C"
void stub__ZN4mono2ui9ImageView7setCropENS_3geo4SizeE(void *o, mono::geo::Size a0)
{
    ((mono::ui::ImageView *) o)->setCrop(a0);
}

/* mono::ui::ImageView::setCrop(geo::Point) */
extern "C"
void stub__ZN4mono2ui9ImageView7setCropENS_3geo5PointE(void *o, mono::geo::Point a0)
{
    ((mono::ui::ImageView *) o)->setCrop(a0);
}

/* mono::ui::ImageView::setImage(media::Image *) */
extern "C"
void stub__ZN4mono2ui9ImageView8setImageEPNS_5media5ImageE(void *o, mono::media::Image * a0)
{
    ((mono::ui::ImageView *) o)->setImage(a0);
}

/* mono::ui::ImageView::ImageView(media::Image *) */
extern "C"
void stub__ZN4mono2ui9ImageViewC1EPNS_5media5ImageE(void *o, mono::media::Image * a0)
{
    new(o) mono::ui::ImageView(a0);
}

/* mono::ui::ImageView::ImageView() */
extern "C"
void stub__ZN4mono2ui9ImageViewC1Ev(void *o)
{
    new(o) mono::ui::ImageView();
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

/* mono::Queue::Next(mono::IQueueItem *) */
extern "C"
mono::IQueueItem * stub__ZN4mono5Queue4NextEPNS_10IQueueItemE(void *o, mono::IQueueItem * a0)
{
    return ((mono::Queue *) o)->Next(a0);
}

/* mono::Queue::Peek() */
extern "C"
mono::IQueueItem * stub__ZN4mono5Queue4PeekEv(void *o)
{
    return ((mono::Queue *) o)->Peek();
}

/* mono::Queue::Exists(mono::IQueueItem *) */
extern "C"
bool stub__ZN4mono5Queue6ExistsEPNS_10IQueueItemE(void *o, mono::IQueueItem * a0)
{
    return ((mono::Queue *) o)->Exists(a0);
}

/* mono::Queue::Length() */
extern "C"
unsigned short stub__ZN4mono5Queue6LengthEv(void *o)
{
    return ((mono::Queue *) o)->Length();
}

/* mono::Queue::Remove(mono::IQueueItem *) */
extern "C"
bool stub__ZN4mono5Queue6RemoveEPNS_10IQueueItemE(void *o, mono::IQueueItem * a0)
{
    return ((mono::Queue *) o)->Remove(a0);
}

/* mono::Queue::Dequeue() */
extern "C"
mono::IQueueItem * stub__ZN4mono5Queue7DequeueEv(void *o)
{
    return ((mono::Queue *) o)->Dequeue();
}

/* mono::Queue::Enqueue(mono::IQueueItem *) */
extern "C"
void stub__ZN4mono5Queue7EnqueueEPNS_10IQueueItemE(void *o, mono::IQueueItem * a0)
{
    ((mono::Queue *) o)->Enqueue(a0);
}

/* mono::Queue::Queue() */
extern "C"
void stub__ZN4mono5QueueC1Ev(void *o)
{
    new(o) mono::Queue();
}

/* mono::Timer::SingleShot() */
extern "C"
bool stub__ZN4mono5Timer10SingleShotEv(void *o)
{
    return ((mono::Timer *) o)->SingleShot();
}

/* mono::Timer::setCallback(void (*)()) */
extern "C"
void stub__ZN4mono5Timer11setCallbackEPFvvE(void *o, void (*a0)())
{
    ((mono::Timer *) o)->setCallback(a0);
}

/* mono::Timer::setInterval(uint32_t) */
extern "C"
void stub__ZN4mono5Timer11setIntervalEj(void *o, unsigned int a0)
{
    ((mono::Timer *) o)->setInterval(a0);
}

/* mono::Timer::Stop() */
extern "C"
void stub__ZN4mono5Timer4StopEv(void *o)
{
    ((mono::Timer *) o)->Stop();
}

/* mono::Timer::Start() */
extern "C"
void stub__ZN4mono5Timer5StartEv(void *o)
{
    ((mono::Timer *) o)->Start();
}

/* mono::Timer::Running() */
extern "C"
bool stub__ZN4mono5Timer7RunningEv(void *o)
{
    return ((mono::Timer *) o)->Running();
}

/* mono::Timer::callOnce(uint32_t, void (*)()) */
extern "C"
mono::Timer * stub__ZN4mono5Timer8callOnceEjPFvvE(void *o, unsigned int a0, void (*a1)())
{
    return ((mono::Timer *) o)->callOnce(a0, a1);
}

/* mono::Timer::Timer(uint32_t, bool) */
extern "C"
void stub__ZN4mono5TimerC1Ejb(void *o, unsigned int a0, bool a1)
{
    new(o) mono::Timer(a0, a1);
}

/* mono::Timer::Timer() */
extern "C"
void stub__ZN4mono5TimerC1Ev(void *o)
{
    new(o) mono::Timer();
}

/* mono::Timer::~Timer() */
extern "C"
void stub__ZN4mono5TimerD0Ev(void *o)
{
    ((mono::Timer *) o)->~Timer();
}

/* mono::media::Image::SeekToHLine(int) */
extern "C"
void stub__ZN4mono5media5Image11SeekToHLineEi(void *o, int a0)
{
    ((mono::media::Image *) o)->SeekToHLine(a0);
}

/* mono::media::Image::PixelByteSize() */
extern "C"
int stub__ZN4mono5media5Image13PixelByteSizeEv(void *o)
{
    return ((mono::media::Image *) o)->PixelByteSize();
}

/* mono::media::Image::ReadPixelData(void *, int) */
extern "C"
int stub__ZN4mono5media5Image13ReadPixelDataEPvi(void *o, void * a0, int a1)
{
    return ((mono::media::Image *) o)->ReadPixelData(a0, a1);
}

/* mono::media::Image::SkipPixelData(int) */
extern "C"
int stub__ZN4mono5media5Image13SkipPixelDataEi(void *o, int a0)
{
    return ((mono::media::Image *) o)->SkipPixelData(a0);
}

/* mono::media::Image::Width() */
extern "C"
int stub__ZN4mono5media5Image5WidthEv(void *o)
{
    return ((mono::media::Image *) o)->Width();
}

/* mono::media::Image::Height() */
extern "C"
int stub__ZN4mono5media5Image6HeightEv(void *o)
{
    return ((mono::media::Image *) o)->Height();
}

/* mono::media::Image::IsValid() */
extern "C"
bool stub__ZN4mono5media5Image7IsValidEv(void *o)
{
    return ((mono::media::Image *) o)->IsValid();
}

/* mono::media::Image::~Image() */
extern "C"
void stub__ZN4mono5media5ImageD0Ev(void *o)
{
    ((mono::media::Image *) o)->~Image();
}

/* mono::media::BMPImage::SeekToHLine(int) */
extern "C"
void stub__ZN4mono5media8BMPImage11SeekToHLineEi(void *o, int a0)
{
    ((mono::media::BMPImage *) o)->SeekToHLine(a0);
}

/* mono::media::BMPImage::PixelByteSize() */
extern "C"
int stub__ZN4mono5media8BMPImage13PixelByteSizeEv(void *o)
{
    return ((mono::media::BMPImage *) o)->PixelByteSize();
}

/* mono::media::BMPImage::ReadPixelData(void *, int) */
extern "C"
int stub__ZN4mono5media8BMPImage13ReadPixelDataEPvi(void *o, void * a0, int a1)
{
    return ((mono::media::BMPImage *) o)->ReadPixelData(a0, a1);
}

/* mono::media::BMPImage::SkipPixelData(int) */
extern "C"
int stub__ZN4mono5media8BMPImage13SkipPixelDataEi(void *o, int a0)
{
    return ((mono::media::BMPImage *) o)->SkipPixelData(a0);
}

/* mono::media::BMPImage::Width() */
extern "C"
int stub__ZN4mono5media8BMPImage5WidthEv(void *o)
{
    return ((mono::media::BMPImage *) o)->Width();
}

/* mono::media::BMPImage::Height() */
extern "C"
int stub__ZN4mono5media8BMPImage6HeightEv(void *o)
{
    return ((mono::media::BMPImage *) o)->Height();
}

/* mono::media::BMPImage::IsValid() */
extern "C"
bool stub__ZN4mono5media8BMPImage7IsValidEv(void *o)
{
    return ((mono::media::BMPImage *) o)->IsValid();
}

/* mono::media::BMPImage::BMPImage(mono::String) */
extern "C"
void stub__ZN4mono5media8BMPImageC1ENS_6StringE(void *o, mono::String a0)
{
    new(o) mono::media::BMPImage(a0);
}

/* mono::media::BMPImage::BMPImage(const mono::media::BMPImage &) */
extern "C"
void stub__ZN4mono5media8BMPImageC1ERKS1_(void *o, const mono::media::BMPImage & a0)
{
    new(o) mono::media::BMPImage(a0);
}

/* mono::media::BMPImage::BMPImage() */
extern "C"
void stub__ZN4mono5media8BMPImageC1Ev(void *o)
{
    new(o) mono::media::BMPImage();
}

/* mono::media::BMPImage::~BMPImage() */
extern "C"
void stub__ZN4mono5media8BMPImageD0Ev(void *o)
{
    ((mono::media::BMPImage *) o)->~BMPImage();
}

/* mono::media::BMPImage::operator=(const mono::media::BMPImage &) */
extern "C"
mono::media::BMPImage & stub__ZN4mono5media8BMPImageaSERKS1_(void *o, const mono::media::BMPImage & a0)
{
    return ((mono::media::BMPImage *) o)->operator=(a0);
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

/* mono::Console::Console(mbed::Serial *) */
extern "C"
void stub__ZN4mono7ConsoleC1EPN4mbed6SerialE(void *o, mbed::Serial * a0)
{
    new(o) mono::Console(a0);
}

/* mono::Console::operator<<(const char *) */
extern "C"
mono::Console & stub__ZN4mono7ConsolelsEPKc(void *o, const char * a0)
{
    return ((mono::Console *) o)->operator<<(a0);
}

/* mono::Console::operator<<(char) */
extern "C"
mono::Console & stub__ZN4mono7ConsolelsEc(void *o, char a0)
{
    return ((mono::Console *) o)->operator<<(a0);
}

/* mono::Console::operator<<(float) */
extern "C"
mono::Console & stub__ZN4mono7ConsolelsEf(void *o, float a0)
{
    return ((mono::Console *) o)->operator<<(a0);
}

/* mono::Console::operator<<(int) */
extern "C"
mono::Console & stub__ZN4mono7ConsolelsEi(void *o, int a0)
{
    return ((mono::Console *) o)->operator<<(a0);
}

/* mono::display::DisplayPainter::drawAALine(const geo::Point &, const geo::Point &, bool) */
extern "C"
void stub__ZN4mono7display14DisplayPainter10drawAALineERKNS_3geo5PointES5_b(void *o, const mono::geo::Point & a0, const mono::geo::Point & a1, bool a2)
{
    ((mono::display::DisplayPainter *) o)->drawAALine(a0, a1, a2);
}

/* mono::display::DisplayPainter::drawAALine(uint16_t, uint16_t, uint16_t, uint16_t, bool) */
extern "C"
void stub__ZN4mono7display14DisplayPainter10drawAALineEttttb(void *o, unsigned short a0, unsigned short a1, unsigned short a2, unsigned short a3, bool a4)
{
    ((mono::display::DisplayPainter *) o)->drawAALine(a0, a1, a2, a3, a4);
}

/* mono::display::DisplayPainter::drawCircle(const geo::Circle &, bool) */
extern "C"
void stub__ZN4mono7display14DisplayPainter10drawCircleERKNS_3geo6CircleEb(void *o, const mono::geo::Circle & a0, bool a1)
{
    ((mono::display::DisplayPainter *) o)->drawCircle(a0, a1);
}

/* mono::display::DisplayPainter::drawCircle(uint16_t, uint16_t, uint16_t, bool) */
extern "C"
void stub__ZN4mono7display14DisplayPainter10drawCircleEtttb(void *o, unsigned short a0, unsigned short a1, unsigned short a2, bool a3)
{
    ((mono::display::DisplayPainter *) o)->drawCircle(a0, a1, a2, a3);
}

/* mono::display::DisplayPainter::setTextSize(uint8_t) */
extern "C"
void stub__ZN4mono7display14DisplayPainter11setTextSizeEh(void *o, unsigned char a0)
{
    ((mono::display::DisplayPainter *) o)->setTextSize(a0);
}

/* mono::display::DisplayPainter::drawFillRect(const geo::Rect &, bool) */
extern "C"
void stub__ZN4mono7display14DisplayPainter12drawFillRectERKNS_3geo4RectEb(void *o, const mono::geo::Rect & a0, bool a1)
{
    ((mono::display::DisplayPainter *) o)->drawFillRect(a0, a1);
}

/* mono::display::DisplayPainter::drawFillRect(uint16_t, uint16_t, uint16_t, uint16_t, bool) */
extern "C"
void stub__ZN4mono7display14DisplayPainter12drawFillRectEttttb(void *o, unsigned short a0, unsigned short a1, unsigned short a2, unsigned short a3, bool a4)
{
    ((mono::display::DisplayPainter *) o)->drawFillRect(a0, a1, a2, a3, a4);
}

/* mono::display::DisplayPainter::setLineWidth(uint8_t) */
extern "C"
void stub__ZN4mono7display14DisplayPainter12setLineWidthEh(void *o, unsigned char a0)
{
    ((mono::display::DisplayPainter *) o)->setLineWidth(a0);
}

/* mono::display::DisplayPainter::drawFillCircle(const geo::Circle &, bool) */
extern "C"
void stub__ZN4mono7display14DisplayPainter14drawFillCircleERKNS_3geo6CircleEb(void *o, const mono::geo::Circle & a0, bool a1)
{
    ((mono::display::DisplayPainter *) o)->drawFillCircle(a0, a1);
}

/* mono::display::DisplayPainter::drawFillCircle(uint16_t, uint16_t, uint16_t, bool) */
extern "C"
void stub__ZN4mono7display14DisplayPainter14drawFillCircleEtttb(void *o, unsigned short a0, unsigned short a1, unsigned short a2, bool a3)
{
    ((mono::display::DisplayPainter *) o)->drawFillCircle(a0, a1, a2, a3);
}

/* mono::display::DisplayPainter::fillCircleHelper(int16_t, int16_t, int16_t, uint8_t, int16_t, bool) */
extern "C"
void stub__ZN4mono7display14DisplayPainter16fillCircleHelperEssshsb(void *o, short a0, short a1, short a2, unsigned char a3, short a4, bool a5)
{
    ((mono::display::DisplayPainter *) o)->fillCircleHelper(a0, a1, a2, a3, a4, a5);
}

/* mono::display::DisplayPainter::setBackgroundColor(mono::display::Color) */
extern "C"
void stub__ZN4mono7display14DisplayPainter18setBackgroundColorENS0_5ColorE(void *o, mono::display::Color a0)
{
    ((mono::display::DisplayPainter *) o)->setBackgroundColor(a0);
}

/* mono::display::DisplayPainter::setForegroundColor(mono::display::Color) */
extern "C"
void stub__ZN4mono7display14DisplayPainter18setForegroundColorENS0_5ColorE(void *o, mono::display::Color a0)
{
    ((mono::display::DisplayPainter *) o)->setForegroundColor(a0);
}

/* mono::display::DisplayPainter::setRefreshCallback(void (*)()) */
extern "C"
void stub__ZN4mono7display14DisplayPainter18setRefreshCallbackEPFvvE(void *o, void (*a0)())
{
    ((mono::display::DisplayPainter *) o)->setRefreshCallback(a0);
}

/* mono::display::DisplayPainter::IsAntialiasedDrawing() */
extern "C"
bool stub__ZN4mono7display14DisplayPainter20IsAntialiasedDrawingEv(void *o)
{
    return ((mono::display::DisplayPainter *) o)->IsAntialiasedDrawing();
}

/* mono::display::DisplayPainter::useAntialiasedDrawing(bool) */
extern "C"
void stub__ZN4mono7display14DisplayPainter21useAntialiasedDrawingEb(void *o, bool a0)
{
    ((mono::display::DisplayPainter *) o)->useAntialiasedDrawing(a0);
}

/* mono::display::DisplayPainter::drawChar(uint16_t, uint16_t, char) */
extern "C"
void stub__ZN4mono7display14DisplayPainter8drawCharEttc(void *o, unsigned short a0, unsigned short a1, char a2)
{
    ((mono::display::DisplayPainter *) o)->drawChar(a0, a1, a2);
}

/* mono::display::DisplayPainter::drawLine(const geo::Point &, const geo::Point &, bool) */
extern "C"
void stub__ZN4mono7display14DisplayPainter8drawLineERKNS_3geo5PointES5_b(void *o, const mono::geo::Point & a0, const mono::geo::Point & a1, bool a2)
{
    ((mono::display::DisplayPainter *) o)->drawLine(a0, a1, a2);
}

/* mono::display::DisplayPainter::drawLine(uint16_t, uint16_t, uint16_t, uint16_t, bool) */
extern "C"
void stub__ZN4mono7display14DisplayPainter8drawLineEttttb(void *o, unsigned short a0, unsigned short a1, unsigned short a2, unsigned short a3, bool a4)
{
    ((mono::display::DisplayPainter *) o)->drawLine(a0, a1, a2, a3, a4);
}

/* mono::display::DisplayPainter::drawRect(const geo::Rect &, bool) */
extern "C"
void stub__ZN4mono7display14DisplayPainter8drawRectERKNS_3geo4RectEb(void *o, const mono::geo::Rect & a0, bool a1)
{
    ((mono::display::DisplayPainter *) o)->drawRect(a0, a1);
}

/* mono::display::DisplayPainter::drawRect(uint16_t, uint16_t, uint16_t, uint16_t, bool) */
extern "C"
void stub__ZN4mono7display14DisplayPainter8drawRectEttttb(void *o, unsigned short a0, unsigned short a1, unsigned short a2, unsigned short a3, bool a4)
{
    ((mono::display::DisplayPainter *) o)->drawRect(a0, a1, a2, a3, a4);
}

/* mono::display::DisplayPainter::drawHLine(uint16_t, uint16_t, uint16_t, bool) */
extern "C"
void stub__ZN4mono7display14DisplayPainter9drawHLineEtttb(void *o, unsigned short a0, unsigned short a1, unsigned short a2, bool a3)
{
    ((mono::display::DisplayPainter *) o)->drawHLine(a0, a1, a2, a3);
}

/* mono::display::DisplayPainter::drawPixel(const geo::Point &, bool) */
extern "C"
void stub__ZN4mono7display14DisplayPainter9drawPixelERKNS_3geo5PointEb(void *o, const mono::geo::Point & a0, bool a1)
{
    ((mono::display::DisplayPainter *) o)->drawPixel(a0, a1);
}

/* mono::display::DisplayPainter::drawPixel(uint16_t, uint16_t, bool) */
extern "C"
void stub__ZN4mono7display14DisplayPainter9drawPixelEttb(void *o, unsigned short a0, unsigned short a1, bool a2)
{
    ((mono::display::DisplayPainter *) o)->drawPixel(a0, a1, a2);
}

/* mono::display::DisplayPainter::drawPixel(uint16_t, uint16_t, uint8_t, bool) */
extern "C"
void stub__ZN4mono7display14DisplayPainter9drawPixelEtthb(void *o, unsigned short a0, unsigned short a1, unsigned char a2, bool a3)
{
    ((mono::display::DisplayPainter *) o)->drawPixel(a0, a1, a2, a3);
}

/* mono::display::DisplayPainter::drawVLine(uint16_t, uint16_t, uint16_t, bool) */
extern "C"
void stub__ZN4mono7display14DisplayPainter9drawVLineEtttb(void *o, unsigned short a0, unsigned short a1, unsigned short a2, bool a3)
{
    ((mono::display::DisplayPainter *) o)->drawVLine(a0, a1, a2, a3);
}

/* mono::display::DisplayPainter::DisplayPainter(mono::display::IDisplayController *, bool) */
extern "C"
void stub__ZN4mono7display14DisplayPainterC1EPNS0_18IDisplayControllerEb(void *o, mono::display::IDisplayController * a0, bool a1)
{
    new(o) mono::display::DisplayPainter(a0, a1);
}

/* mono::display::DisplayPainter::~DisplayPainter() */
extern "C"
void stub__ZN4mono7display14DisplayPainterD0Ev(void *o)
{
    ((mono::display::DisplayPainter *) o)->~DisplayPainter();
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

/* mono::QueueInterrupt::IsInterruptsWhilePendingActive() */
extern "C"
bool stub__ZNK4mono14QueueInterrupt30IsInterruptsWhilePendingActiveEv(void *o)
{
    return ((mono::QueueInterrupt *) o)->IsInterruptsWhilePendingActive();
}

/* mono::ui::ButtonView::TextLabel() */
extern "C"
const mono::ui::TextLabelView & stub__ZNK4mono2ui10ButtonView9TextLabelEv(void *o)
{
    return ((mono::ui::ButtonView *) o)->TextLabel();
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

/* mono::ui::BackgroundView::Color() */
extern "C"
mono::display::Color stub__ZNK4mono2ui14BackgroundView5ColorEv(void *o)
{
    return ((mono::ui::BackgroundView *) o)->Color();
}

/* mono::ui::OnOffButtonView::BorderColor() */
extern "C"
mono::display::Color stub__ZNK4mono2ui15OnOffButtonView11BorderColorEv(void *o)
{
    return ((mono::ui::OnOffButtonView *) o)->BorderColor();
}

/* mono::ui::OnOffButtonView::HighlightColor() */
extern "C"
mono::display::Color stub__ZNK4mono2ui15OnOffButtonView14HighlightColorEv(void *o)
{
    return ((mono::ui::OnOffButtonView *) o)->HighlightColor();
}

/* mono::ui::OnOffButtonView::BackgroundColor() */
extern "C"
mono::display::Color stub__ZNK4mono2ui15OnOffButtonView15BackgroundColorEv(void *o)
{
    return ((mono::ui::OnOffButtonView *) o)->BackgroundColor();
}

/* mono::ui::OnOffButtonView::isActive() */
extern "C"
bool stub__ZNK4mono2ui15OnOffButtonView8isActiveEv(void *o)
{
    return ((mono::ui::OnOffButtonView *) o)->isActive();
}

/* mono::ui::ProgressBarView::CurrentValue() */
extern "C"
int stub__ZNK4mono2ui15ProgressBarView12CurrentValueEv(void *o)
{
    return ((mono::ui::ProgressBarView *) o)->CurrentValue();
}

/* mono::ui::ProgressBarView::Maximum() */
extern "C"
int stub__ZNK4mono2ui15ProgressBarView7MaximumEv(void *o)
{
    return ((mono::ui::ProgressBarView *) o)->Maximum();
}

/* mono::ui::ProgressBarView::Minimum() */
extern "C"
int stub__ZNK4mono2ui15ProgressBarView7MinimumEv(void *o)
{
    return ((mono::ui::ProgressBarView *) o)->Minimum();
}

/* mono::ui::StatusIndicatorView::State() */
extern "C"
bool stub__ZNK4mono2ui19StatusIndicatorView5StateEv(void *o)
{
    return ((mono::ui::StatusIndicatorView *) o)->State();
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

/* mono::ui::GraphView::DataSource() */
extern "C"
mono::ui::IGraphViewDataSource * stub__ZNK4mono2ui9GraphView10DataSourceEv(void *o)
{
    return ((mono::ui::GraphView *) o)->DataSource();
}

/* mono::ui::GraphView::CursorActive() */
extern "C"
bool stub__ZNK4mono2ui9GraphView12CursorActiveEv(void *o)
{
    return ((mono::ui::GraphView *) o)->CursorActive();
}

/* mono::ui::ImageView::Crop() */
extern "C"
const mono::geo::Rect & stub__ZNK4mono2ui9ImageView4CropEv(void *o)
{
    return ((mono::ui::ImageView *) o)->Crop();
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

/* mono::display::DisplayPainter::CanvasWidth() */
extern "C"
unsigned short stub__ZNK4mono7display14DisplayPainter11CanvasWidthEv(void *o)
{
    return ((mono::display::DisplayPainter *) o)->CanvasWidth();
}

/* mono::display::DisplayPainter::CanvasHeight() */
extern "C"
unsigned short stub__ZNK4mono7display14DisplayPainter12CanvasHeightEv(void *o)
{
    return ((mono::display::DisplayPainter *) o)->CanvasHeight();
}

/* mono::display::DisplayPainter::BackgroundColor() */
extern "C"
mono::display::Color stub__ZNK4mono7display14DisplayPainter15BackgroundColorEv(void *o)
{
    return ((mono::display::DisplayPainter *) o)->BackgroundColor();
}

/* mono::display::DisplayPainter::ForegroundColor() */
extern "C"
mono::display::Color stub__ZNK4mono7display14DisplayPainter15ForegroundColorEv(void *o)
{
    return ((mono::display::DisplayPainter *) o)->ForegroundColor();
}

/* mono::display::DisplayPainter::DisplayController() */
extern "C"
mono::display::IDisplayController * stub__ZNK4mono7display14DisplayPainter17DisplayControllerEv(void *o)
{
    return ((mono::display::DisplayPainter *) o)->DisplayController();
}

/* mono::display::DisplayPainter::TextSize() */
extern "C"
unsigned char stub__ZNK4mono7display14DisplayPainter8TextSizeEv(void *o)
{
    return ((mono::display::DisplayPainter *) o)->TextSize();
}

/* mono::display::DisplayPainter::LineWidth() */
extern "C"
unsigned char stub__ZNK4mono7display14DisplayPainter9LineWidthEv(void *o)
{
    return ((mono::display::DisplayPainter *) o)->LineWidth();
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
