// Subcommands exercise various ways a program can fail.

#include "util.h"

extern "C" void mainCRTStartup() {
    LPCSTR cmdlinep = GetCommandLineA();
    if (cmdlinep == NULL) {
        print(fmt().str("GetCommandLineA failed: ").dec(GetLastError()).nl());
        ExitProcess(1);
    }
    std::string_view cmdline(cmdlinep);

    std::string_view mode;
    size_t space = 0;
    for (; space < cmdline.size(); space++) {
        if (cmdline[space] == ' ') {
            mode = std::string_view(cmdline.data() + space + 1, cmdline.size() - space - 1);
            break;
        }
    }
    if (mode.empty()) {
        print(fmt().str("usage: errors.exe <mode>").nl());
        ExitProcess(1);
    }

    if (mode == "exit") {
        print(fmt().str("expect: exit code 2").nl());
        ExitProcess(2);
    } else if (mode == "write-null") {
        print(fmt().str("writing mem[0]").nl());
        uint32_t* ptr = reinterpret_cast<uint32_t*>(0);
        *ptr = 1;
    } else if (mode == "write-high") {
        print(fmt().str("writing mem[0xFFFF_F000]").nl());
        uint32_t* ptr = reinterpret_cast<uint32_t*>(0xFFFFF000);
        *ptr = 1;
    } else {
        print(fmt().str("unknown mode: ").str(mode).nl());
        ExitProcess(1);
    }
}
