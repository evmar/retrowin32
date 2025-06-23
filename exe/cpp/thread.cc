// Run two threads printing in parallel.
// Purpose: exercise CreateThread and threading in general.

#include "util.h"

struct ThreadParams {
    const char* name;
    int steps;
    DWORD tls_key;
};

void run_thread(const ThreadParams* params) {
    for (int i = 0; i < params->steps; ++i) {
        DWORD thread_id = GetCurrentThreadId();
        DWORD tls = reinterpret_cast<DWORD>(TlsGetValue(params->tls_key));
        print(fmt().str("thread_id=").dec(thread_id).str(" name=").str(params->name).str(" tls=").dec(tls).str(" i=").dec(i).nl());
        Sleep(1000 / params->steps);
    }
    DWORD thread_id = GetCurrentThreadId();
    DWORD tls = reinterpret_cast<DWORD>(TlsGetValue(params->tls_key));
    print(fmt().str("thread_id=").dec(thread_id).str(" name=").str(params->name).str(" tls=").dec(tls).str(" returning").nl());
}

DWORD WINAPI thread_proc(LPVOID param) {
    ThreadParams* params = reinterpret_cast<ThreadParams*>(param);
    TlsSetValue(params->tls_key, reinterpret_cast<LPVOID>(2));
    run_thread(params);
    return 0;
}

DWORD WINAPI short_thread_proc(LPVOID) {
    DWORD thread_id = GetCurrentThreadId();
    print(fmt().str("thread_id=").dec(thread_id).str(" short_thread_proc exiting").nl());
    return 0;
}

extern "C" void mainCRTStartup() {
    // Create a thread that starts and exits quickly, to exercise thread teardown.
    CreateThread(
        nullptr,
        0x1000,
        short_thread_proc,
        nullptr,
        0,
        nullptr
    );

    DWORD tls_key = TlsAlloc();
    TlsSetValue(tls_key, reinterpret_cast<LPVOID>(1));

    ThreadParams thread_params = {
        .name = "i_am_thread",
        .steps = 5,
        .tls_key = tls_key,
    };

    CreateThread(
        nullptr,
        0x8000,
        thread_proc,
        &thread_params,
        0,
        nullptr
    );

    ThreadParams main_thread = {
        .name = "main",
        .steps = 10,
        .tls_key = tls_key,
    };
    run_thread(&main_thread);
}
