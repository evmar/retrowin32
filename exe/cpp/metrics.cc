// Dump GetSystemMetrics() values.

#include "util.h"
#include <windows.h>

extern "C" void mainCRTStartup() {
    print(fmt().str("GetSystemMetrics():\r\n"));

    for (int i = 0; i < 100; ++i) {
        int metric = GetSystemMetrics(i);
        print(fmt().dec(i).str(" => ").dec(metric).nl());
    }
}
