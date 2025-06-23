#include "util.h"
#include <ddraw.h>

LRESULT CALLBACK WndProc(HWND hwnd, UINT msg, WPARAM wparam, LPARAM lparam) {
    // print(fmt().str("msg: ").hex(msg).str(" ").str(windows_message_to_str(msg)).str(" {").nl());
    LRESULT ret = DefWindowProc(hwnd, msg, wparam, lparam);
    // print(fmt().str("} msg: ").hex(msg).str(" ").str(windows_message_to_str(msg)).nl());
    return ret;
}

void mainloop() {
    MSG msg;
    while (GetMessage(&msg, NULL, 0, 0) > 0) {
        TranslateMessage(&msg);
        DispatchMessage(&msg);
    }
}

extern "C" void mainCRTStartup() {
    WNDCLASS wc = {
        .lpfnWndProc = WndProc,
        .lpszClassName = "ddraw",
    };

    RegisterClass(&wc);

    print(fmt().str("CreateWindowEx").nl());
    HWND hwnd = CreateWindowEx(
        0,
        wc.lpszClassName,
        "title",
        WS_OVERLAPPED,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        400,
        300,
        NULL,
        NULL,
        NULL,
        NULL
    );

    if (hwnd == NULL) {
        print(fmt().str("CreateWindowEx failed").nl());
        return;
    }

    print(fmt().str("DirectDrawCreate").nl());
    IDirectDraw* ddraw = nullptr;
    HRESULT result = DirectDrawCreate(NULL, &ddraw, NULL);
    if (FAILED(result)) {
        print(fmt().str("DirectDrawCreate failed").nl());
        return;
    }

    print(fmt().str("SetCooperativeLevel").nl());
    result = ddraw->SetCooperativeLevel(hwnd, DDSCL_NORMAL | DDSCL_EXCLUSIVE);
    if (FAILED(result)) {
        print(fmt().str("SetCooperativeLevel failed").nl());
        return;
    }

    print(fmt().str("SetDisplayMode").nl());
    result = ddraw->SetDisplayMode(320, 200, 32);
    if (FAILED(result)) {
        print(fmt().str("SetDisplayMode failed").nl());
        return;
    }

    DDSURFACEDESC desc = {
        .dwSize = sizeof(DDSURFACEDESC),
        .dwFlags = DDSD_BACKBUFFERCOUNT | DDSD_CAPS,
        .dwBackBufferCount = 2,
        .ddsCaps = DDSCAPS {
            .dwCaps = DDSCAPS_PRIMARYSURFACE | DDSCAPS_VIDEOMEMORY | DDSCAPS_COMPLEX,
        },
    };

    IDirectDrawSurface* surf = nullptr;
    print(fmt().str("CreateSurface").nl());
    result = ddraw->CreateSurface(&desc, &surf, NULL);
    if (FAILED(result)) {
        print(fmt().str("CreateSurface failed").nl());
        return;
    }

    mainloop();
}
