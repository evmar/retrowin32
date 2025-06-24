// Draw some bitmaps in different ways.

#include "util.h"
#include <windows.h>
#include <cstdint>

// Upward-pointing arrow, 16x16 monochrome.
const uint8_t ARROW[16 * 2] = {
    0b00000000, 0b00000000,
    0b00000001, 0b10000000,
    0b00000011, 0b11000000,
    0b00000111, 0b11100000,
    0b00001111, 0b11110000,
    0b00011111, 0b11111000,
    0b00111111, 0b11111100,
    0b01111111, 0b11111110,
    0b00000011, 0b11000000,
    0b00000011, 0b11000000,
    0b00000011, 0b11000000,
    0b00000011, 0b11000000,
    0b00000011, 0b11000000,
    0b00000011, 0b11000000,
    0b00000011, 0b11000000,
    0b00000000, 0b00000000,
};

uint32_t* arrow_rgba() {
    static uint32_t pixels[16 * 16];
    for (size_t row = 0; row < sizeof(ARROW); ++row) {
        uint8_t b = ARROW[row];
        for (int i = 0; i < 8; ++i) {
            uint8_t bit = (b >> (7 - i)) & 1;
            uint32_t color = (bit == 0) ? 0xFF000000 : 0xFFFFFFFF;
            pixels[row * 8 + i] = color;
        }
    }
    return pixels;
}

struct State {
    HBITMAP mono;
    HBITMAP dib;
    uint32_t* dib_pixels;

    State() {
        mono = nullptr;
        dib = nullptr;
        dib_pixels = nullptr;
    }

    void init(HDC hdc) {
        create_mono();
        create_dib(hdc, dib);
    }

    void create_mono() {
        mono = CreateBitmap(16, 16, 1, 1, ARROW);
    }

    void create_dib(HDC hdc, HBITMAP& dib) {
        BITMAPINFO info = {
            .bmiHeader = {
                .biSize = sizeof(BITMAPINFOHEADER),
                .biWidth = 16,
                .biHeight = 16,
                .biPlanes = 1,
                .biBitCount = 32,
                .biCompression = BI_RGB,
            },
        };

        dib = CreateDIBSection(hdc, &info, DIB_RGB_COLORS, (void**)&dib_pixels, nullptr, 0);
        memcpy(dib_pixels, arrow_rgba(), sizeof(uint32_t) * 16 * 16);
    }
};

static State state = {};

void paint_SetDIBitsToDevice(HDC hdc, int y, uint32_t* rgba) {
    struct Cfg {
        uint32_t startScan;
        bool invertY;
    };
    Cfg cfgs[] = {
        {0, false},
        {0, true},
    };

    for (size_t i = 0; i < sizeof(cfgs) / sizeof(cfgs[0]); ++i) {
        BITMAPINFOHEADER info = {};
        info.biSize = sizeof(BITMAPINFOHEADER);
        info.biWidth = 16;
        info.biHeight = cfgs[i].invertY ? -16 : 16;
        info.biPlanes = 1;
        info.biBitCount = 32;
        info.biCompression = BI_RGB;

        int x = 32 + static_cast<int>(i) * 32;
        SetDIBitsToDevice(
            hdc,
            x,
            y,
            16,
            16,
            0,
            static_cast<int>(cfgs[i].startScan),
            cfgs[i].startScan,
            16,
            rgba,
            reinterpret_cast<BITMAPINFO*>(&info),
            DIB_RGB_COLORS
        );
    }
}

void paint_StretchDIBits(HDC hdc, int y, uint32_t* rgba) {
    BITMAPINFOHEADER info = {};
    info.biSize = sizeof(BITMAPINFOHEADER);
    info.biWidth = 16;
    info.biHeight = 16;
    info.biPlanes = 1;
    info.biBitCount = 32;
    info.biCompression = BI_RGB;

    StretchDIBits(
        hdc,
        32,
        y,
        32,
        32,
        0,
        0,
        16,
        16,
        rgba,
        reinterpret_cast<BITMAPINFO*>(&info),
        DIB_RGB_COLORS,
        SRCCOPY
    );
}

void paint(HDC hdc) {
    if (!state.mono) {
        state.init(hdc);
    }

    HDC hdc_bitmap = CreateCompatibleDC(hdc);

    int row = 1;
    SelectObject(hdc_bitmap, state.mono);
    BitBlt(hdc, 32, row * 32, 16, 16, hdc_bitmap, 0, 0, SRCCOPY);
    row++;

    SelectObject(hdc_bitmap, state.dib);
    BitBlt(hdc, 32, row * 32, 16, 16, hdc_bitmap, 0, 0, SRCCOPY);
    row++;

    uint32_t* rgba = arrow_rgba();
    paint_SetDIBitsToDevice(hdc, row * 32, rgba);
    row++;

    paint_StretchDIBits(hdc, row * 32, rgba);

    DeleteDC(hdc_bitmap);
}

LRESULT CALLBACK wndproc(HWND hwnd, UINT msg, WPARAM wparam, LPARAM lparam) {
    switch (msg) {
        case WM_DESTROY:
            PostQuitMessage(0);
            return 0;
        case WM_LBUTTONDOWN:
            for (int i = 0; i < 32 * 2; ++i) {
                state.dib_pixels[i] = 0xFF0000FF;
            }
            InvalidateRect(hwnd, nullptr, FALSE);
            return 0;
        case WM_PAINT: {
            PAINTSTRUCT ps;
            HDC hdc = BeginPaint(hwnd, &ps);
            paint(hdc);
            EndPaint(hwnd, &ps);
            return 0;
        }
        default:
            return DefWindowProc(hwnd, msg, wparam, lparam);
    }
}

HWND create_window() {
    const char* class_name = "dibs";
    WNDCLASS wc = {
        .lpfnWndProc = wndproc,
        .hbrBackground = reinterpret_cast<HBRUSH>(COLOR_WINDOW + 1),
        .lpszClassName = class_name,
    };
    RegisterClass(&wc);

    HWND hwnd = CreateWindowEx(
        0,
        class_name,
        "title",
        WS_OVERLAPPEDWINDOW,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        400,
        300,
        nullptr,
        nullptr,
        nullptr,
        nullptr
    );
    ShowWindow(hwnd, SW_NORMAL);
    return hwnd;
}

extern "C" void mainCRTStartup() {
    HWND hwnd = create_window();
    MSG msg;
    while (GetMessage(&msg, nullptr, 0, 0)) {
        TranslateMessage(&msg);
        DispatchMessage(&msg);
    }
}
