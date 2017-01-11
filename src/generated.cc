#include <new>
#include "application_context_interface.h"
#include "mn_string.h"
#include "circle.h"
#include "point.h"
#include "application_run_loop_task_interface.h"
#include "application_controller_interface.h"

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
