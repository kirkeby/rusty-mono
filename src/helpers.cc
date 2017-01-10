#include <mono.h>

typedef struct {
    void (*initialize)();
    void (*wake_from_reset)();
    void (*will_goto_sleep)();
    void (*wake_from_sleep)();
} mono_callbacks;

class AppController : public mono::IApplication {
private:
    mono_callbacks callbacks;

public:
    AppController(mono_callbacks);
    void monoWakeFromReset();
    void monoWillGotoSleep();
    void monoWakeFromSleep();
};

extern "C"
void run_mono(mono_callbacks callbacks)
{
    AppController app(callbacks);
    mono::IApplicationContext::Instance->setMonoApplication(&app);
    app.enterRunLoop();
}


AppController::AppController(mono_callbacks cb) : callbacks(cb)
{
    (callbacks.initialize)();
}

void AppController::monoWakeFromReset()
{
    (callbacks.wake_from_reset)();
}

void AppController::monoWillGotoSleep()
{
    (callbacks.will_goto_sleep)();
}

void AppController::monoWakeFromSleep()
{
    (callbacks.wake_from_sleep)();
}
