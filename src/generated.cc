#include <new>
#include "/home/sune/Projects/mono_framework/dist/mono/include/application_context_interface.h"
#include "/home/sune/Projects/mono_framework/dist/mono/include/application_controller_interface.h"
#include "/home/sune/Projects/mono_framework/dist/mono/include/application_run_loop.h"
#include "/home/sune/Projects/mono_framework/dist/mono/include/application_context.h"

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
