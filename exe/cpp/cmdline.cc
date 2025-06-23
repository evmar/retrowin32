// Exercise command line functions.

#include "util.h"

extern "C" void mainCRTStartup() {
    char buf[256];

    DWORD ret = GetModuleFileNameA(NULL, buf, sizeof(buf));
    if (ret > 0) {
        print(fmt().str("GetModuleFileNameA: ").str(buf).nl());
    } else {
        print(fmt().str("GetModuleFileNameA failed: ").dec(GetLastError()).nl());
    }

    LPCSTR cmdline = GetCommandLineA();
    if (cmdline != NULL) {
        print(fmt().str("GetCommandLineA: ").str(cmdline).nl());
    } else {
        print(fmt().str("GetCommandLineA failed: ").dec(GetLastError()).nl());
    }
}
