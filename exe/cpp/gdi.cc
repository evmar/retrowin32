// Create a window and paint on it.
// Purpose: exercise some GDI basics like CreateWindow and message loop.

#include "util.h"
#include <windows.h>

const int ID_QUITBUTTON = 1;

LRESULT CALLBACK wndproc(HWND hwnd, UINT msg, WPARAM wparam, LPARAM lparam) {
    switch (msg) {
        case WM_DESTROY:
            PostQuitMessage(0);
            return 0;

        case WM_PAINT: {
            PAINTSTRUCT ps;
            HDC hdc = BeginPaint(hwnd, &ps);
            FillRect(hdc, &ps.rcPaint, (HBRUSH)(COLOR_WINDOW + 1));
            EndPaint(hwnd, &ps);
            return 0;
        }

        case WM_COMMAND:
            if (LOWORD(wparam) == ID_QUITBUTTON) {
                PostQuitMessage(0);
                return 0;
            }
            break;

        default:
            break;
    }
    return DefWindowProcA(hwnd, msg, wparam, lparam);
}

HFONT get_default_font() {
    return (HFONT)GetStockObject(DEFAULT_GUI_FONT);
}

HWND create_window() {
    HFONT hfont = get_default_font();

    WNDCLASSA wc = {
        .lpfnWndProc = wndproc,
        .lpszClassName = "gdi",
    };

    RegisterClassA(&wc);

    HWND hwnd = CreateWindowExA(
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

    if (!hwnd) {
        print(fmt().str("CreateWindowExA failed").nl());
        return NULL;
    }

    HWND button = CreateWindowExA(
        0,
        "BUTTON",
        "quit",
        WS_TABSTOP | WS_VISIBLE | WS_CHILD | BS_DEFPUSHBUTTON,
        10,
        10,
        100,
        30,
        hwnd,
        (HMENU)ID_QUITBUTTON,
        NULL,
        NULL
    );

    if (!button) {
        print(fmt().str("Button creation failed").nl());
        return NULL;
    }

    SendMessageA(button, WM_SETFONT, (WPARAM)hfont, 0);
    ShowWindow(hwnd, SW_NORMAL);

    return hwnd;
}

extern "C" void mainCRTStartup() {
    HWND hwnd = create_window();
    if (!hwnd) {
        return;
    }

    MSG msg;
    while (GetMessageA(&msg, NULL, 0, 0) > 0) {
        TranslateMessage(&msg);
        DispatchMessageA(&msg);
    }
}
