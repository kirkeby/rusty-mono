#include <new>
#include "/home/sune/Projects/mono_framework/src/date_time.h"
#include "/home/sune/Projects/mono_framework/src/buzzer_interface.h"
#include "/home/sune/Projects/mono_framework/src/settings_interface.h"
#include "/home/sune/Projects/mono_framework/src/application_context_interface.h"
#include "/home/sune/Projects/mono_framework/src/mono_accelerometer.h"
#include "/home/sune/Projects/mono_framework/src/power_subsystem_interface.h"
#include "/home/sune/Projects/mono_framework/src/network_request.h"
#include "/home/sune/Projects/mono_framework/src/mn_digital_out.h"
#include "/home/sune/Projects/mono_framework/src/power_aware_interface.h"
#include "/home/sune/Projects/mono_framework/src/application_controller_interface.h"
#include "/home/sune/Projects/mono_framework/src/pct2075_temperature.h"
#include "/home/sune/Projects/mono_framework/src/mn_serial.h"
#include "/home/sune/Projects/mono_framework/src/dns_resolver.h"
#include "/home/sune/Projects/mono_framework/src/mono_rtc.h"
#include "/home/sune/Projects/mono_framework/src/regex.h"
#include "/home/sune/Projects/mono_framework/src/touch_event.h"
#include "/home/sune/Projects/mono_framework/src/touch_system_interface.h"
#include "/home/sune/Projects/mono_framework/src/accelerometer_interface.h"
#include "/home/sune/Projects/mono_framework/src/power_fenced_peripheral_interface.h"
#include "/home/sune/Projects/mono_framework/src/application_context.h"
#include "/home/sune/Projects/mono_framework/src/rtc_interface.h"
#include "/home/sune/Projects/mono_framework/src/act8600_power_system.h"
#include "/home/sune/Projects/mono_framework/src/mn_timer.h"
#include "/home/sune/Projects/mono_framework/src/size.h"
#include "/home/sune/Projects/mono_framework/src/queue_interrupt.h"
#include "/home/sune/Projects/mono_framework/src/mono_touch_system.h"
#include "/home/sune/Projects/mono_framework/src/mn_string.h"
#include "/home/sune/Projects/mono_framework/src/http_client.h"
#include "/home/sune/Projects/mono_framework/src/queue.h"
#include "/home/sune/Projects/mono_framework/src/application_run_loop.h"
#include "/home/sune/Projects/mono_framework/src/consoles.h"
#include "/home/sune/Projects/mono_framework/src/mono_power_management.h"
#include "/home/sune/Projects/mono_framework/src/point.h"
#include "/home/sune/Projects/mono_framework/src/touch_responder.h"
#include "/home/sune/Projects/mono_framework/src/mono_buzzer.h"
#include "/home/sune/Projects/mono_framework/src/http_post_client.h"
#include "/home/sune/Projects/mono_framework/src/rect.h"
#include "/home/sune/Projects/mono_framework/src/temperature_interface.h"
#include "/home/sune/Projects/mono_framework/src/at30ts74_temperature.h"
#include "/home/sune/Projects/mono_framework/src/circle.h"
#include "/home/sune/Projects/mono_framework/src/power_management_interface.h"

/* mono::AppRunLoop::CheckUsbDtr() */
extern "C"
void stub__ZN4mono10AppRunLoop11CheckUsbDtrEv(void *o)
{
    ((mono::AppRunLoop *) o)->CheckUsbDtr();
}

/* mono::AppRunLoop::addDynamicTask(mono::IRunLoopTask *) */
extern "C"
bool stub__ZN4mono10AppRunLoop14addDynamicTaskEPNS_12IRunLoopTaskE(void *o, mono::IRunLoopTask * a0)
{
    return ((mono::AppRunLoop *) o)->addDynamicTask(a0);
}

/* mono::AppRunLoop::removeDynamicTask(mono::IRunLoopTask *) */
extern "C"
bool stub__ZN4mono10AppRunLoop17removeDynamicTaskEPNS_12IRunLoopTaskE(void *o, mono::IRunLoopTask * a0)
{
    return ((mono::AppRunLoop *) o)->removeDynamicTask(a0);
}

/* mono::AppRunLoop::setResetOnUserButton(bool) */
extern "C"
void stub__ZN4mono10AppRunLoop20setResetOnUserButtonEb(void *o, bool a0)
{
    ((mono::AppRunLoop *) o)->setResetOnUserButton(a0);
}

/* mono::AppRunLoop::exec() */
extern "C"
void stub__ZN4mono10AppRunLoop4execEv(void *o)
{
    ((mono::AppRunLoop *) o)->exec();
}

/* mono::AppRunLoop::quit() */
extern "C"
void stub__ZN4mono10AppRunLoop4quitEv(void *o)
{
    ((mono::AppRunLoop *) o)->quit();
}

/* mono::AppRunLoop::AppRunLoop() */
extern "C"
void stub__ZN4mono10AppRunLoopC1Ev(void *o)
{
    new(o) mono::AppRunLoop();
}

/* mono::IRTCSystem::setupRtcSystem() */
extern "C"
void stub__ZN4mono10IRTCSystem14setupRtcSystemEv(void *o)
{
    ((mono::IRTCSystem *) o)->setupRtcSystem();
}

/* mono::IRTCSystem::stopRtc() */
extern "C"
void stub__ZN4mono10IRTCSystem7stopRtcEv(void *o)
{
    ((mono::IRTCSystem *) o)->stopRtc();
}

/* mono::IRTCSystem::startRtc() */
extern "C"
void stub__ZN4mono10IRTCSystem8startRtcEv(void *o)
{
    ((mono::IRTCSystem *) o)->startRtc();
}

/* mono::TouchEvent::TouchEvent(mono::TouchEvent::TouchEventType, geo::Point &, mono::ITouchSystem *) */
extern "C"
void stub__ZN4mono10TouchEventC1ENS0_14TouchEventTypeERNS_3geo5PointEPNS_12ITouchSystemE(void *o, mono::TouchEvent::TouchEventType a0, geo::Point & a1, mono::ITouchSystem * a2)
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

/* mono::IApplication::IApplication() */
extern "C"
void stub__ZN4mono12IApplicationC1Ev(void *o)
{
    new(o) mono::IApplication();
}

/* mono::ITouchSystem::setCalibration(TouchCalibration &) */
extern "C"
void stub__ZN4mono12ITouchSystem14setCalibrationERNS_3geo4RectE(void *o, TouchCalibration & a0)
{
    ((mono::ITouchSystem *) o)->setCalibration(a0);
}

/* mono::ITouchSystem::ToScreenCoordsX(int, uint16_t) */
extern "C"
int stub__ZN4mono12ITouchSystem15ToScreenCoordsXEit(void *o, int a0, uint16_t a1)
{
    return ((mono::ITouchSystem *) o)->ToScreenCoordsX(a0, a1);
}

/* mono::ITouchSystem::ToScreenCoordsY(int, uint16_t) */
extern "C"
int stub__ZN4mono12ITouchSystem15ToScreenCoordsYEit(void *o, int a0, uint16_t a1)
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
TouchCalibration stub__ZN4mono12ITouchSystem18CurrentCalibrationEv(void *o)
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
int stub__ZN4mono14QueueInterrupt13FallTimeStampEv(void *o)
{
    return ((mono::QueueInterrupt *) o)->FallTimeStamp();
}

/* mono::QueueInterrupt::RiseTimeStamp() */
extern "C"
int stub__ZN4mono14QueueInterrupt13RiseTimeStampEv(void *o)
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
void stub__ZN4mono14QueueInterrupt4fallEPFvvE(void *o, void (*)() a0)
{
    ((mono::QueueInterrupt *) o)->fall(a0);
}

/* mono::QueueInterrupt::rise(void (*)()) */
extern "C"
void stub__ZN4mono14QueueInterrupt4riseEPFvvE(void *o, void (*)() a0)
{
    ((mono::QueueInterrupt *) o)->rise(a0);
}

/* mono::QueueInterrupt::Snapshot() */
extern "C"
bool stub__ZN4mono14QueueInterrupt8SnapshotEv(void *o)
{
    return ((mono::QueueInterrupt *) o)->Snapshot();
}

/* mono::QueueInterrupt::QueueInterrupt(int, int) */
extern "C"
void stub__ZN4mono14QueueInterruptC1Eii(void *o, int a0, int a1)
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

/* mono::TouchResponder::TouchResponder() */
extern "C"
void stub__ZN4mono14TouchResponderC1Ev(void *o)
{
    new(o) mono::TouchResponder();
}

/* mono::MonoTouchSystem::setCalibration(TouchCalibration &) */
extern "C"
void stub__ZN4mono15MonoTouchSystem14setCalibrationERNS_3geo4RectE(void *o, TouchCalibration & a0)
{
    ((mono::MonoTouchSystem *) o)->setCalibration(a0);
}

/* mono::MonoTouchSystem::ToScreenCoordsX(int, uint16_t) */
extern "C"
int stub__ZN4mono15MonoTouchSystem15ToScreenCoordsXEit(void *o, int a0, uint16_t a1)
{
    return ((mono::MonoTouchSystem *) o)->ToScreenCoordsX(a0, a1);
}

/* mono::MonoTouchSystem::ToScreenCoordsY(int, uint16_t) */
extern "C"
int stub__ZN4mono15MonoTouchSystem15ToScreenCoordsYEit(void *o, int a0, uint16_t a1)
{
    return ((mono::MonoTouchSystem *) o)->ToScreenCoordsY(a0, a1);
}

/* mono::MonoTouchSystem::processTouchInput() */
extern "C"
void stub__ZN4mono15MonoTouchSystem17processTouchInputEv(void *o)
{
    ((mono::MonoTouchSystem *) o)->processTouchInput();
}

/* mono::MonoTouchSystem::CurrentCalibration() */
extern "C"
TouchCalibration stub__ZN4mono15MonoTouchSystem18CurrentCalibrationEv(void *o)
{
    return ((mono::MonoTouchSystem *) o)->CurrentCalibration();
}

/* mono::MonoTouchSystem::init() */
extern "C"
void stub__ZN4mono15MonoTouchSystem4initEv(void *o)
{
    ((mono::MonoTouchSystem *) o)->init();
}

/* mono::MonoTouchSystem::MonoTouchSystem() */
extern "C"
void stub__ZN4mono15MonoTouchSystemC1Ev(void *o)
{
    new(o) mono::MonoTouchSystem();
}

/* mono::ApplicationContext::setMonoApplication(mono::IApplication *) */
extern "C"
void stub__ZN4mono18ApplicationContext18setMonoApplicationEPNS_12IApplicationE(void *o, mono::IApplication * a0)
{
    ((mono::ApplicationContext *) o)->setMonoApplication(a0);
}

/* mono::ApplicationContext::exec() */
extern "C"
int stub__ZN4mono18ApplicationContext4execEv(void *o)
{
    return ((mono::ApplicationContext *) o)->exec();
}

/* mono::IApplicationContext::SleepForMs(uint32_t) */
extern "C"
void stub__ZN4mono19IApplicationContext10SleepForMsEj(void *o, uint32_t a0)
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

/* mono::io::DigitalOut::setMode(int) */
extern "C"
void stub__ZN4mono2io10DigitalOut7setModeEi(void *o, int a0)
{
    ((mono::io::DigitalOut *) o)->setMode(a0);
}

/* mono::io::DigitalOut::DigitalOut(int) */
extern "C"
void stub__ZN4mono2io10DigitalOutC1Ei(void *o, int a0)
{
    new(o) mono::io::DigitalOut(a0);
}

/* mono::io::Serial::DTR() */
extern "C"
bool stub__ZN4mono2io6Serial3DTREv(void *o)
{
    return ((mono::io::Serial *) o)->DTR();
}

/* mono::io::Serial::IsReady() */
extern "C"
bool stub__ZN4mono2io6Serial7IsReadyEv(void *o)
{
    return ((mono::io::Serial *) o)->IsReady();
}

/* mono::io::Serial::Serial() */
extern "C"
void stub__ZN4mono2io6SerialC1Ev(void *o)
{
    new(o) mono::io::Serial();
}

/* mono::io::ISettings::read(mono::io::ISettings::SettingLocations, mono::io::ISettings::SettingSizes, void *) */
extern "C"
int stub__ZN4mono2io9ISettings4readENS1_16SettingLocationsENS1_12SettingSizesEPv(void *o, mono::io::ISettings::SettingLocations a0, mono::io::ISettings::SettingSizes a1, void * a2)
{
    return ((mono::io::ISettings *) o)->read(a0, a1, a2);
}

/* mono::io::ISettings::write(mono::io::ISettings::SettingLocations, mono::io::ISettings::SettingSizes, const void *) */
extern "C"
int stub__ZN4mono2io9ISettings5writeENS1_16SettingLocationsENS1_12SettingSizesEPKv(void *o, mono::io::ISettings::SettingLocations a0, mono::io::ISettings::SettingSizes a1, const void * a2)
{
    return ((mono::io::ISettings *) o)->write(a0, a1, a2);
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
void stub__ZN4mono3geo4Rect7setSizeENS0_4SizeE(void *o, class Size a0)
{
    ((mono::geo::Rect *) o)->setSize(a0);
}

/* mono::geo::Rect::setPoint(class Point) */
extern "C"
void stub__ZN4mono3geo4Rect8setPointENS0_5PointE(void *o, class Point a0)
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
void stub__ZN4mono3geo6CircleC1ENS0_5PointEj(void *o, mono::geo::Point a0, uint32_t a1)
{
    new(o) mono::geo::Circle(a0, a1);
}

/* mono::geo::Circle::Circle(int, int, uint32_t) */
extern "C"
void stub__ZN4mono3geo6CircleC1Eiij(void *o, int a0, int a1, uint32_t a2)
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
uint16_t stub__ZN4mono5Queue6LengthEv(void *o)
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

/* mono::Regex::Match(mono::String, Capture *, uint32_t) */
extern "C"
bool stub__ZN4mono5Regex5MatchENS_6StringEP8slre_capj(void *o, mono::String a0, Capture * a1, uint32_t a2)
{
    return ((mono::Regex *) o)->Match(a0, a1, a2);
}

/* mono::Regex::Value(Capture &) */
extern "C"
mono::String stub__ZN4mono5Regex5ValueER8slre_cap(void *o, Capture & a0)
{
    return ((mono::Regex *) o)->Value(a0);
}

/* mono::Regex::IsMatch(mono::String) */
extern "C"
bool stub__ZN4mono5Regex7IsMatchENS_6StringE(void *o, mono::String a0)
{
    return ((mono::Regex *) o)->IsMatch(a0);
}

/* mono::Regex::Regex(mono::String) */
extern "C"
void stub__ZN4mono5RegexC1ENS_6StringE(void *o, mono::String a0)
{
    new(o) mono::Regex(a0);
}

/* mono::Regex::Regex() */
extern "C"
void stub__ZN4mono5RegexC1Ev(void *o)
{
    new(o) mono::Regex();
}

/* mono::Timer::SingleShot() */
extern "C"
bool stub__ZN4mono5Timer10SingleShotEv(void *o)
{
    return ((mono::Timer *) o)->SingleShot();
}

/* mono::Timer::setCallback(void (*)()) */
extern "C"
void stub__ZN4mono5Timer11setCallbackEPFvvE(void *o, void (*)() a0)
{
    ((mono::Timer *) o)->setCallback(a0);
}

/* mono::Timer::setInterval(int) */
extern "C"
void stub__ZN4mono5Timer11setIntervalEi(void *o, int a0)
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

/* mono::power::IPowerAware::OnSystemBatteryLow() */
extern "C"
void stub__ZN4mono5power11IPowerAware18OnSystemBatteryLowEv(void *o)
{
    ((mono::power::IPowerAware *) o)->OnSystemBatteryLow();
}

/* mono::power::IPowerAware::onSystemEnterSleep() */
extern "C"
void stub__ZN4mono5power11IPowerAware18onSystemEnterSleepEv(void *o)
{
    ((mono::power::IPowerAware *) o)->onSystemEnterSleep();
}

/* mono::power::IPowerAware::onSystemPowerOnReset() */
extern "C"
void stub__ZN4mono5power11IPowerAware20onSystemPowerOnResetEv(void *o)
{
    ((mono::power::IPowerAware *) o)->onSystemPowerOnReset();
}

/* mono::power::IPowerAware::onSystemWakeFromSleep() */
extern "C"
void stub__ZN4mono5power11IPowerAware21onSystemWakeFromSleepEv(void *o)
{
    ((mono::power::IPowerAware *) o)->onSystemWakeFromSleep();
}

/* mono::power::IPowerSubSystem::ChargeStatus() */
extern "C"
mono::power::IPowerSubSystem::ChargeState stub__ZN4mono5power15IPowerSubSystem12ChargeStatusEv(void *o)
{
    return ((mono::power::IPowerSubSystem *) o)->ChargeStatus();
}

/* mono::power::IPowerSubSystem::IsPowerFenced() */
extern "C"
bool stub__ZN4mono5power15IPowerSubSystem13IsPowerFencedEv(void *o)
{
    return ((mono::power::IPowerSubSystem *) o)->IsPowerFenced();
}

/* mono::power::IPowerSubSystem::IsUSBCharging() */
extern "C"
bool stub__ZN4mono5power15IPowerSubSystem13IsUSBChargingEv(void *o)
{
    return ((mono::power::IPowerSubSystem *) o)->IsUSBCharging();
}

/* mono::power::IPowerSubSystem::setPowerFence(bool) */
extern "C"
void stub__ZN4mono5power15IPowerSubSystem13setPowerFenceEb(void *o, bool a0)
{
    ((mono::power::IPowerSubSystem *) o)->setPowerFence(a0);
}

/* mono::power::IPowerSubSystem::onSystemEnterSleep() */
extern "C"
void stub__ZN4mono5power15IPowerSubSystem18onSystemEnterSleepEv(void *o)
{
    ((mono::power::IPowerSubSystem *) o)->onSystemEnterSleep();
}

/* mono::power::IPowerSubSystem::onSystemPowerOnReset() */
extern "C"
void stub__ZN4mono5power15IPowerSubSystem20onSystemPowerOnResetEv(void *o)
{
    ((mono::power::IPowerSubSystem *) o)->onSystemPowerOnReset();
}

/* mono::power::IPowerSubSystem::onSystemWakeFromSleep() */
extern "C"
void stub__ZN4mono5power15IPowerSubSystem21onSystemWakeFromSleepEv(void *o)
{
    ((mono::power::IPowerSubSystem *) o)->onSystemWakeFromSleep();
}

/* mono::power::IPowerSubSystem::IsPowerOk() */
extern "C"
bool stub__ZN4mono5power15IPowerSubSystem9IsPowerOkEv(void *o)
{
    return ((mono::power::IPowerSubSystem *) o)->IsPowerOk();
}

/* mono::power::IPowerManagement::EnterSleep() */
extern "C"
void stub__ZN4mono5power16IPowerManagement10EnterSleepEv(void *o)
{
    ((mono::power::IPowerManagement *) o)->EnterSleep();
}

/* mono::power::IPowerManagement::AppendToPowerAwareQueue(mono::power::IPowerAware *) */
extern "C"
void stub__ZN4mono5power16IPowerManagement23AppendToPowerAwareQueueEPNS0_11IPowerAwareE(void *o, mono::power::IPowerAware * a0)
{
    ((mono::power::IPowerManagement *) o)->AppendToPowerAwareQueue(a0);
}

/* mono::power::IPowerManagement::RemoveFromPowerAwareQueue(mono::power::IPowerAware *) */
extern "C"
bool stub__ZN4mono5power16IPowerManagement25RemoveFromPowerAwareQueueEPNS0_11IPowerAwareE(void *o, mono::power::IPowerAware * a0)
{
    return ((mono::power::IPowerManagement *) o)->RemoveFromPowerAwareQueue(a0);
}

/* mono::power::ACT8600PowerSystem::USBOTGPower() */
extern "C"
bool stub__ZN4mono5power18ACT8600PowerSystem11USBOTGPowerEv(void *o)
{
    return ((mono::power::ACT8600PowerSystem *) o)->USBOTGPower();
}

/* mono::power::ACT8600PowerSystem::ChargeStatus() */
extern "C"
mono::power::IPowerSubSystem::ChargeState stub__ZN4mono5power18ACT8600PowerSystem12ChargeStatusEv(void *o)
{
    return ((mono::power::ACT8600PowerSystem *) o)->ChargeStatus();
}

/* mono::power::ACT8600PowerSystem::SystemStatus() */
extern "C"
int stub__ZN4mono5power18ACT8600PowerSystem12SystemStatusEv(void *o)
{
    return ((mono::power::ACT8600PowerSystem *) o)->SystemStatus();
}

/* mono::power::ACT8600PowerSystem::IsPowerFenced() */
extern "C"
bool stub__ZN4mono5power18ACT8600PowerSystem13IsPowerFencedEv(void *o)
{
    return ((mono::power::ACT8600PowerSystem *) o)->IsPowerFenced();
}

/* mono::power::ACT8600PowerSystem::setPowerFence(bool) */
extern "C"
void stub__ZN4mono5power18ACT8600PowerSystem13setPowerFenceEb(void *o, bool a0)
{
    ((mono::power::ACT8600PowerSystem *) o)->setPowerFence(a0);
}

/* mono::power::ACT8600PowerSystem::powerOffUnused() */
extern "C"
void stub__ZN4mono5power18ACT8600PowerSystem14powerOffUnusedEv(void *o)
{
    ((mono::power::ACT8600PowerSystem *) o)->powerOffUnused();
}

/* mono::power::ACT8600PowerSystem::setUSBOTGPower(bool) */
extern "C"
bool stub__ZN4mono5power18ACT8600PowerSystem14setUSBOTGPowerEb(void *o, bool a0)
{
    return ((mono::power::ACT8600PowerSystem *) o)->setUSBOTGPower(a0);
}

/* mono::power::ACT8600PowerSystem::onSystemEnterSleep() */
extern "C"
void stub__ZN4mono5power18ACT8600PowerSystem18onSystemEnterSleepEv(void *o)
{
    ((mono::power::ACT8600PowerSystem *) o)->onSystemEnterSleep();
}

/* mono::power::ACT8600PowerSystem::onSystemPowerOnReset() */
extern "C"
void stub__ZN4mono5power18ACT8600PowerSystem20onSystemPowerOnResetEv(void *o)
{
    ((mono::power::ACT8600PowerSystem *) o)->onSystemPowerOnReset();
}

/* mono::power::ACT8600PowerSystem::onSystemWakeFromSleep() */
extern "C"
void stub__ZN4mono5power18ACT8600PowerSystem21onSystemWakeFromSleepEv(void *o)
{
    ((mono::power::ACT8600PowerSystem *) o)->onSystemWakeFromSleep();
}

/* mono::power::ACT8600PowerSystem::SystemVoltageThreshold() */
extern "C"
mono::power::ACT8600PowerSystem::SystemVoltageLevels stub__ZN4mono5power18ACT8600PowerSystem22SystemVoltageThresholdEv(void *o)
{
    return ((mono::power::ACT8600PowerSystem *) o)->SystemVoltageThreshold();
}

/* mono::power::ACT8600PowerSystem::setSystemMonitorInterrupt(bool) */
extern "C"
void stub__ZN4mono5power18ACT8600PowerSystem25setSystemMonitorInterruptEb(void *o, bool a0)
{
    ((mono::power::ACT8600PowerSystem *) o)->setSystemMonitorInterrupt(a0);
}

/* mono::power::ACT8600PowerSystem::setSystemVoltageThreshold(mono::power::ACT8600PowerSystem::SystemVoltageLevels) */
extern "C"
void stub__ZN4mono5power18ACT8600PowerSystem25setSystemVoltageThresholdENS1_19SystemVoltageLevelsE(void *o, mono::power::ACT8600PowerSystem::SystemVoltageLevels a0)
{
    ((mono::power::ACT8600PowerSystem *) o)->setSystemVoltageThreshold(a0);
}

/* mono::power::ACT8600PowerSystem::USBOTG() */
extern "C"
int stub__ZN4mono5power18ACT8600PowerSystem6USBOTGEv(void *o)
{
    return ((mono::power::ACT8600PowerSystem *) o)->USBOTG();
}

/* mono::power::ACT8600PowerSystem::IsPowerOk() */
extern "C"
bool stub__ZN4mono5power18ACT8600PowerSystem9IsPowerOkEv(void *o)
{
    return ((mono::power::ACT8600PowerSystem *) o)->IsPowerOk();
}

/* mono::power::ACT8600PowerSystem::powerReg8(bool) */
extern "C"
void stub__ZN4mono5power18ACT8600PowerSystem9powerReg8Eb(void *o, bool a0)
{
    ((mono::power::ACT8600PowerSystem *) o)->powerReg8(a0);
}

/* mono::power::ACT8600PowerSystem::ACT8600PowerSystem() */
extern "C"
void stub__ZN4mono5power18ACT8600PowerSystemC1Ev(void *o)
{
    new(o) mono::power::ACT8600PowerSystem();
}

/* mono::power::MonoPowerManagement::EnterSleep(bool) */
extern "C"
void stub__ZN4mono5power19MonoPowerManagement10EnterSleepEb(void *o, bool a0)
{
    ((mono::power::MonoPowerManagement *) o)->EnterSleep(a0);
}

/* mono::power::MonoPowerManagement::EnterSleep() */
extern "C"
void stub__ZN4mono5power19MonoPowerManagement10EnterSleepEv(void *o)
{
    ((mono::power::MonoPowerManagement *) o)->EnterSleep();
}

/* mono::power::MonoPowerManagement::MonoPowerManagement() */
extern "C"
void stub__ZN4mono5power19MonoPowerManagementC1Ev(void *o)
{
    new(o) mono::power::MonoPowerManagement();
}

/* mono::power::IPowerFencedPeripheral::PowerCameUp() */
extern "C"
void stub__ZN4mono5power22IPowerFencedPeripheral11PowerCameUpEv(void *o)
{
    ((mono::power::IPowerFencedPeripheral *) o)->PowerCameUp();
}

/* mono::power::IPowerFencedPeripheral::ReleasePower() */
extern "C"
bool stub__ZN4mono5power22IPowerFencedPeripheral12ReleasePowerEv(void *o)
{
    return ((mono::power::IPowerFencedPeripheral *) o)->ReleasePower();
}

/* mono::power::IPowerFencedPeripheral::RequestPower() */
extern "C"
bool stub__ZN4mono5power22IPowerFencedPeripheral12RequestPowerEv(void *o)
{
    return ((mono::power::IPowerFencedPeripheral *) o)->RequestPower();
}

/* mono::power::IPowerFencedPeripheral::PowerGoesDown() */
extern "C"
void stub__ZN4mono5power22IPowerFencedPeripheral13PowerGoesDownEv(void *o)
{
    ((mono::power::IPowerFencedPeripheral *) o)->PowerGoesDown();
}

/* mono::power::IPowerFencedPeripheral::ForceTogglePower() */
extern "C"
bool stub__ZN4mono5power22IPowerFencedPeripheral16ForceTogglePowerEv(void *o)
{
    return ((mono::power::IPowerFencedPeripheral *) o)->ForceTogglePower();
}

/* mono::power::IPowerFencedPeripheral::IPowerFencedPeripheral() */
extern "C"
void stub__ZN4mono5power22IPowerFencedPeripheralC1Ev(void *o)
{
    new(o) mono::power::IPowerFencedPeripheral();
}

/* mono::String::preAllocbytes(uint32_t) */
extern "C"
void stub__ZN4mono6String13preAllocbytesEj(void *o, uint32_t a0)
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
void stub__ZN4mono6StringC1EPcj(void *o, char * a0, uint32_t a1)
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
void stub__ZN4mono6StringC1Ej(void *o, uint32_t a0)
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

/* mono::sensor::MonoBuzzer::buzzKill() */
extern "C"
void stub__ZN4mono6sensor10MonoBuzzer8buzzKillEv(void *o)
{
    ((mono::sensor::MonoBuzzer *) o)->buzzKill();
}

/* mono::sensor::MonoBuzzer::buzzAsync(uint32_t) */
extern "C"
void stub__ZN4mono6sensor10MonoBuzzer9buzzAsyncEj(void *o, uint32_t a0)
{
    ((mono::sensor::MonoBuzzer *) o)->buzzAsync(a0);
}

/* mono::sensor::MonoBuzzer::MonoBuzzer() */
extern "C"
void stub__ZN4mono6sensor10MonoBuzzerC1Ev(void *o)
{
    new(o) mono::sensor::MonoBuzzer();
}

/* mono::sensor::ITemperature::ReadMilliCelcius() */
extern "C"
int stub__ZN4mono6sensor12ITemperature16ReadMilliCelciusEv(void *o)
{
    return ((mono::sensor::ITemperature *) o)->ReadMilliCelcius();
}

/* mono::sensor::ITemperature::Read() */
extern "C"
int stub__ZN4mono6sensor12ITemperature4ReadEv(void *o)
{
    return ((mono::sensor::ITemperature *) o)->Read();
}

/* mono::sensor::IAccelerometer::Stop() */
extern "C"
void stub__ZN4mono6sensor14IAccelerometer4StopEv(void *o)
{
    ((mono::sensor::IAccelerometer *) o)->Stop();
}

/* mono::sensor::IAccelerometer::Start() */
extern "C"
void stub__ZN4mono6sensor14IAccelerometer5StartEv(void *o)
{
    ((mono::sensor::IAccelerometer *) o)->Start();
}

/* mono::sensor::IAccelerometer::IsActive() */
extern "C"
bool stub__ZN4mono6sensor14IAccelerometer8IsActiveEv(void *o)
{
    return ((mono::sensor::IAccelerometer *) o)->IsActive();
}

/* mono::sensor::IAccelerometer::rawXAxis() */
extern "C"
int16_t stub__ZN4mono6sensor14IAccelerometer8rawXAxisEv(void *o)
{
    return ((mono::sensor::IAccelerometer *) o)->rawXAxis();
}

/* mono::sensor::MMAAccelerometer::Stop() */
extern "C"
void stub__ZN4mono6sensor16MMAAccelerometer4StopEv(void *o)
{
    ((mono::sensor::MMAAccelerometer *) o)->Stop();
}

/* mono::sensor::MMAAccelerometer::Start() */
extern "C"
void stub__ZN4mono6sensor16MMAAccelerometer5StartEv(void *o)
{
    ((mono::sensor::MMAAccelerometer *) o)->Start();
}

/* mono::sensor::MMAAccelerometer::IsActive() */
extern "C"
bool stub__ZN4mono6sensor16MMAAccelerometer8IsActiveEv(void *o)
{
    return ((mono::sensor::MMAAccelerometer *) o)->IsActive();
}

/* mono::sensor::MMAAccelerometer::rawXAxis() */
extern "C"
int16_t stub__ZN4mono6sensor16MMAAccelerometer8rawXAxisEv(void *o)
{
    return ((mono::sensor::MMAAccelerometer *) o)->rawXAxis();
}

/* mono::sensor::MMAAccelerometer::MMAAccelerometer() */
extern "C"
void stub__ZN4mono6sensor16MMAAccelerometerC1Ev(void *o)
{
    new(o) mono::sensor::MMAAccelerometer();
}

/* mono::sensor::PCT2075Temperature::Read() */
extern "C"
int stub__ZN4mono6sensor18PCT2075Temperature4ReadEv(void *o)
{
    return ((mono::sensor::PCT2075Temperature *) o)->Read();
}

/* mono::sensor::PCT2075Temperature::PCT2075Temperature() */
extern "C"
void stub__ZN4mono6sensor18PCT2075TemperatureC1Ev(void *o)
{
    new(o) mono::sensor::PCT2075Temperature();
}

/* mono::sensor::AT30TS74Temperature::ReadMilliCelcius() */
extern "C"
int stub__ZN4mono6sensor19AT30TS74Temperature16ReadMilliCelciusEv(void *o)
{
    return ((mono::sensor::AT30TS74Temperature *) o)->ReadMilliCelcius();
}

/* mono::sensor::AT30TS74Temperature::Read() */
extern "C"
int stub__ZN4mono6sensor19AT30TS74Temperature4ReadEv(void *o)
{
    return ((mono::sensor::AT30TS74Temperature *) o)->Read();
}

/* mono::sensor::AT30TS74Temperature::AT30TS74Temperature() */
extern "C"
void stub__ZN4mono6sensor19AT30TS74TemperatureC1Ev(void *o)
{
    new(o) mono::sensor::AT30TS74Temperature();
}

/* mono::sensor::IBuzzer::buzzKill() */
extern "C"
void stub__ZN4mono6sensor7IBuzzer8buzzKillEv(void *o)
{
    ((mono::sensor::IBuzzer *) o)->buzzKill();
}

/* mono::sensor::IBuzzer::buzzAsync(uint32_t) */
extern "C"
void stub__ZN4mono6sensor7IBuzzer9buzzAsyncEj(void *o, uint32_t a0)
{
    ((mono::sensor::IBuzzer *) o)->buzzAsync(a0);
}

/* mono::sensor::IBuzzer::buzzAsync(uint32_t, void (*)()) */
extern "C"
void stub__ZN4mono6sensor7IBuzzer9buzzAsyncEjPFvvE(void *o, uint32_t a0, void (*)() a1)
{
    ((mono::sensor::IBuzzer *) o)->buzzAsync(a0, a1);
}

/* mono::Console::Console(int *) */
extern "C"
void stub__ZN4mono7ConsoleC1EPi(void *o, int * a0)
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

/* mono::MonoRTC::setupRtcSystem() */
extern "C"
void stub__ZN4mono7MonoRTC14setupRtcSystemEv(void *o)
{
    ((mono::MonoRTC *) o)->setupRtcSystem();
}

/* mono::MonoRTC::stopRtc() */
extern "C"
void stub__ZN4mono7MonoRTC7stopRtcEv(void *o)
{
    ((mono::MonoRTC *) o)->stopRtc();
}

/* mono::MonoRTC::startRtc() */
extern "C"
void stub__ZN4mono7MonoRTC8startRtcEv(void *o)
{
    ((mono::MonoRTC *) o)->startRtc();
}

/* mono::network::HttpClient::HttpResponseData::HttpResponseData(mono::network::HttpClient *) */
extern "C"
void stub__ZN4mono7network10HttpClient16HttpResponseDataC1EPS1_(void *o, mono::network::HttpClient * a0)
{
    new(o) mono::network::HttpClient::HttpResponseData(a0);
}

/* mono::network::HttpClient::HttpResponseData::operator=(const mono::network::HttpClient::HttpResponseData &) */
extern "C"
mono::network::HttpClient::HttpResponseData & stub__ZN4mono7network10HttpClient16HttpResponseDataaSERKS2_(void *o, const mono::network::HttpClient::HttpResponseData & a0)
{
    return ((mono::network::HttpClient::HttpResponseData *) o)->operator=(a0);
}

/* mono::network::HttpClient::HttpClient(const mono::network::HttpClient &) */
extern "C"
void stub__ZN4mono7network10HttpClientC1ERKS1_(void *o, const mono::network::HttpClient & a0)
{
    new(o) mono::network::HttpClient(a0);
}

/* mono::network::HttpClient::HttpClient() */
extern "C"
void stub__ZN4mono7network10HttpClientC1Ev(void *o)
{
    new(o) mono::network::HttpClient();
}

/* mono::network::HttpClient::~HttpClient() */
extern "C"
void stub__ZN4mono7network10HttpClientD0Ev(void *o)
{
    ((mono::network::HttpClient *) o)->~HttpClient();
}

/* mono::network::HttpClient::operator=(const mono::network::HttpClient &) */
extern "C"
mono::network::HttpClient & stub__ZN4mono7network10HttpClientaSERKS1_(void *o, const mono::network::HttpClient & a0)
{
    return ((mono::network::HttpClient *) o)->operator=(a0);
}

/* mono::network::DnsResolver::DnsResolver(const mono::network::DnsResolver &) */
extern "C"
void stub__ZN4mono7network11DnsResolverC1ERKS1_(void *o, const mono::network::DnsResolver & a0)
{
    new(o) mono::network::DnsResolver(a0);
}

/* mono::network::DnsResolver::DnsResolver() */
extern "C"
void stub__ZN4mono7network11DnsResolverC1Ev(void *o)
{
    new(o) mono::network::DnsResolver();
}

/* mono::network::DnsResolver::~DnsResolver() */
extern "C"
void stub__ZN4mono7network11DnsResolverD0Ev(void *o)
{
    ((mono::network::DnsResolver *) o)->~DnsResolver();
}

/* mono::network::DnsResolver::operator=(const mono::network::DnsResolver &) */
extern "C"
mono::network::DnsResolver & stub__ZN4mono7network11DnsResolveraSERKS1_(void *o, const mono::network::DnsResolver & a0)
{
    return ((mono::network::DnsResolver *) o)->operator=(a0);
}

/* mono::network::HttpPostClient::post() */
extern "C"
void stub__ZN4mono7network14HttpPostClient4postEv(void *o)
{
    ((mono::network::HttpPostClient *) o)->post();
}

/* mono::network::HttpPostClient::HttpPostClient(const mono::network::HttpPostClient &) */
extern "C"
void stub__ZN4mono7network14HttpPostClientC1ERKS1_(void *o, const mono::network::HttpPostClient & a0)
{
    new(o) mono::network::HttpPostClient(a0);
}

/* mono::network::HttpPostClient::HttpPostClient() */
extern "C"
void stub__ZN4mono7network14HttpPostClientC1Ev(void *o)
{
    new(o) mono::network::HttpPostClient();
}

/* mono::network::HttpPostClient::operator=(const mono::network::HttpPostClient &) */
extern "C"
mono::network::HttpPostClient & stub__ZN4mono7network14HttpPostClientaSERKS1_(void *o, const mono::network::HttpPostClient & a0)
{
    return ((mono::network::HttpPostClient *) o)->operator=(a0);
}

/* mono::network::INetworkRequest::~INetworkRequest() */
extern "C"
void stub__ZN4mono7network15INetworkRequestD0Ev(void *o)
{
    ((mono::network::INetworkRequest *) o)->~INetworkRequest();
}

/* mono::DateTime::isLeapYear(uint16_t) */
extern "C"
bool stub__ZN4mono8DateTime10isLeapYearEt(void *o, uint16_t a0)
{
    return ((mono::DateTime *) o)->isLeapYear(a0);
}

/* mono::DateTime::fromISO8601(mono::String) */
extern "C"
mono::DateTime stub__ZN4mono8DateTime11fromISO8601ENS_6StringE(void *o, mono::String a0)
{
    return ((mono::DateTime *) o)->fromISO8601(a0);
}

/* mono::DateTime::setSystemDateTime(mono::DateTime) */
extern "C"
void stub__ZN4mono8DateTime17setSystemDateTimeES0_(void *o, mono::DateTime a0)
{
    ((mono::DateTime *) o)->setSystemDateTime(a0);
}

/* mono::DateTime::incrementSystemClock() */
extern "C"
void stub__ZN4mono8DateTime20incrementSystemClockEv(void *o)
{
    ((mono::DateTime *) o)->incrementSystemClock();
}

/* mono::DateTime::now() */
extern "C"
mono::DateTime stub__ZN4mono8DateTime3nowEv(void *o)
{
    return ((mono::DateTime *) o)->now();
}

/* mono::DateTime::maxValue() */
extern "C"
mono::DateTime stub__ZN4mono8DateTime8maxValueEv(void *o)
{
    return ((mono::DateTime *) o)->maxValue();
}

/* mono::DateTime::minValue() */
extern "C"
mono::DateTime stub__ZN4mono8DateTime8minValueEv(void *o)
{
    return ((mono::DateTime *) o)->minValue();
}

/* mono::DateTime::DateTime(const mono::DateTime &) */
extern "C"
void stub__ZN4mono8DateTimeC1ERKS0_(void *o, const mono::DateTime & a0)
{
    new(o) mono::DateTime(a0);
}

/* mono::DateTime::DateTime(uint16_t, uint8_t, uint8_t, uint8_t, uint8_t, uint8_t, mono::DateTime::TimeTypes) */
extern "C"
void stub__ZN4mono8DateTimeC1EthhhhhNS0_9TimeTypesE(void *o, uint16_t a0, uint8_t a1, uint8_t a2, uint8_t a3, uint8_t a4, uint8_t a5, mono::DateTime::TimeTypes a6)
{
    new(o) mono::DateTime(a0, a1, a2, a3, a4, a5, a6);
}

/* mono::DateTime::DateTime() */
extern "C"
void stub__ZN4mono8DateTimeC1Ev(void *o)
{
    new(o) mono::DateTime();
}

/* mono::DateTime::operator=(const mono::DateTime &) */
extern "C"
mono::DateTime & stub__ZN4mono8DateTimeaSERKS0_(void *o, const mono::DateTime & a0)
{
    return ((mono::DateTime *) o)->operator=(a0);
}

/* mono::QueueInterrupt::IsInterruptsWhilePendingActive() */
extern "C"
bool stub__ZNK4mono14QueueInterrupt30IsInterruptsWhilePendingActiveEv(void *o)
{
    return ((mono::QueueInterrupt *) o)->IsInterruptsWhilePendingActive();
}

/* mono::io::ISettings::size() */
extern "C"
int stub__ZNK4mono2io9ISettings4sizeEv(void *o)
{
    return ((mono::io::ISettings *) o)->size();
}

/* mono::geo::Rect::LowerRight() */
extern "C"
class Point stub__ZNK4mono3geo4Rect10LowerRightEv(void *o)
{
    return ((mono::geo::Rect *) o)->LowerRight();
}

/* mono::geo::Rect::UpperRight() */
extern "C"
class Point stub__ZNK4mono3geo4Rect10UpperRightEv(void *o)
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
class Point stub__ZNK4mono3geo4Rect6CenterEv(void *o)
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
bool stub__ZNK4mono3geo4Rect8containsERNS0_5PointE(void *o, class Point & a0)
{
    return ((mono::geo::Rect *) o)->contains(a0);
}

/* mono::geo::Rect::LowerLeft() */
extern "C"
class Point stub__ZNK4mono3geo4Rect9LowerLeftEv(void *o)
{
    return ((mono::geo::Rect *) o)->LowerLeft();
}

/* mono::geo::Rect::UpperLeft() */
extern "C"
class Point stub__ZNK4mono3geo4Rect9UpperLeftEv(void *o)
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
uint32_t stub__ZNK4mono3geo5Point3AbsEv(void *o)
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
uint32_t stub__ZNK4mono3geo6Circle6RadiusEv(void *o)
{
    return ((mono::geo::Circle *) o)->Radius();
}

/* mono::power::IPowerFencedPeripheral::HasPower() */
extern "C"
bool stub__ZNK4mono5power22IPowerFencedPeripheral8HasPowerEv(void *o)
{
    return ((mono::power::IPowerFencedPeripheral *) o)->HasPower();
}

/* mono::String::Length() */
extern "C"
uint32_t stub__ZNK4mono6String6LengthEv(void *o)
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
char stub__ZNK4mono6StringixEj(void *o, uint32_t a0)
{
    return ((mono::String *) o)->operator[](a0);
}

/* mono::network::DnsResolver::DomainName() */
extern "C"
int stub__ZNK4mono7network11DnsResolver10DomainNameEv(void *o)
{
    return ((mono::network::DnsResolver *) o)->DomainName();
}

/* mono::network::DnsResolver::IpAddress() */
extern "C"
int stub__ZNK4mono7network11DnsResolver9IpAddressEv(void *o)
{
    return ((mono::network::DnsResolver *) o)->IpAddress();
}

/* mono::network::DnsResolver::IpVersion() */
extern "C"
mono::network::DnsResolver::IpVersions stub__ZNK4mono7network11DnsResolver9IpVersionEv(void *o)
{
    return ((mono::network::DnsResolver *) o)->IpVersion();
}

/* mono::network::INetworkRequest::IsCompleted() */
extern "C"
bool stub__ZNK4mono7network15INetworkRequest11IsCompletedEv(void *o)
{
    return ((mono::network::INetworkRequest *) o)->IsCompleted();
}

/* mono::network::INetworkRequest::State() */
extern "C"
mono::network::INetworkRequest::States stub__ZNK4mono7network15INetworkRequest5StateEv(void *o)
{
    return ((mono::network::INetworkRequest *) o)->State();
}

/* mono::network::INetworkRequest::HasFailed() */
extern "C"
bool stub__ZNK4mono7network15INetworkRequest9HasFailedEv(void *o)
{
    return ((mono::network::INetworkRequest *) o)->HasFailed();
}

/* mono::DateTime::addMinutes(int) */
extern "C"
mono::DateTime stub__ZNK4mono8DateTime10addMinutesEi(void *o, int a0)
{
    return ((mono::DateTime *) o)->addMinutes(a0);
}

/* mono::DateTime::addSeconds(int) */
extern "C"
mono::DateTime stub__ZNK4mono8DateTime10addSecondsEi(void *o, int a0)
{
    return ((mono::DateTime *) o)->addSeconds(a0);
}

/* mono::DateTime::toDateString() */
extern "C"
mono::String stub__ZNK4mono8DateTime12toDateStringEv(void *o)
{
    return ((mono::DateTime *) o)->toDateString();
}

/* mono::DateTime::toTimeString() */
extern "C"
mono::String stub__ZNK4mono8DateTime12toTimeStringEv(void *o)
{
    return ((mono::DateTime *) o)->toTimeString();
}

/* mono::DateTime::addDays(int) */
extern "C"
mono::DateTime stub__ZNK4mono8DateTime7addDaysEi(void *o, int a0)
{
    return ((mono::DateTime *) o)->addDays(a0);
}

/* mono::DateTime::isValid() */
extern "C"
bool stub__ZNK4mono8DateTime7isValidEv(void *o)
{
    return ((mono::DateTime *) o)->isValid();
}

/* mono::DateTime::addHours(int) */
extern "C"
mono::DateTime stub__ZNK4mono8DateTime8addHoursEi(void *o, int a0)
{
    return ((mono::DateTime *) o)->addHours(a0);
}

/* mono::DateTime::toString() */
extern "C"
mono::String stub__ZNK4mono8DateTime8toStringEv(void *o)
{
    return ((mono::DateTime *) o)->toString();
}

/* mono::DateTime::toISO8601() */
extern "C"
mono::String stub__ZNK4mono8DateTime9toISO8601Ev(void *o)
{
    return ((mono::DateTime *) o)->toISO8601();
}

/* mono::DateTime::toUtcTime() */
extern "C"
mono::DateTime stub__ZNK4mono8DateTime9toUtcTimeEv(void *o)
{
    return ((mono::DateTime *) o)->toUtcTime();
}

/* mono::DateTime::operator==(const mono::DateTime &) */
extern "C"
bool stub__ZNK4mono8DateTimeeqERKS0_(void *o, const mono::DateTime & a0)
{
    return ((mono::DateTime *) o)->operator==(a0);
}

/* mono::DateTime::operator>=(const mono::DateTime &) */
extern "C"
bool stub__ZNK4mono8DateTimegeERKS0_(void *o, const mono::DateTime & a0)
{
    return ((mono::DateTime *) o)->operator>=(a0);
}

/* mono::DateTime::operator>(const mono::DateTime &) */
extern "C"
bool stub__ZNK4mono8DateTimegtERKS0_(void *o, const mono::DateTime & a0)
{
    return ((mono::DateTime *) o)->operator>(a0);
}

/* mono::DateTime::operator<=(const mono::DateTime &) */
extern "C"
bool stub__ZNK4mono8DateTimeleERKS0_(void *o, const mono::DateTime & a0)
{
    return ((mono::DateTime *) o)->operator<=(a0);
}

/* mono::DateTime::operator<(const mono::DateTime &) */
extern "C"
bool stub__ZNK4mono8DateTimeltERKS0_(void *o, const mono::DateTime & a0)
{
    return ((mono::DateTime *) o)->operator<(a0);
}

/* mono::DateTime::operator!=(const mono::DateTime &) */
extern "C"
bool stub__ZNK4mono8DateTimeneERKS0_(void *o, const mono::DateTime & a0)
{
    return ((mono::DateTime *) o)->operator!=(a0);
}
