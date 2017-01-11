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
    printf("AppController::AppController()\r\n");
    (callbacks.initialize)();
}

void AppController::monoWakeFromReset()
{
    printf("AppController::monoWakeFromReset()\r\n");
    (callbacks.wake_from_reset)();
}

void AppController::monoWillGotoSleep()
{
    printf("AppController::monoWillGotoSleep()\r\n");
    (callbacks.will_goto_sleep)();
}

void AppController::monoWakeFromSleep()
{
    printf("AppController::monoWakeFromSleep()\r\n");
    (callbacks.wake_from_sleep)();
}
