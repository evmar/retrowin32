#include <windows.h>
#include <cstdint>
#include <string_view>

void print(std::string_view str) {
    WriteFile(GetStdHandle(STD_OUTPUT_HANDLE), str.data(), str.size(), nullptr, nullptr);
}

struct fmt {
    char buf[1024];
    size_t ofs;
    fmt() : ofs(0) {}

    fmt& ch(char c) {
        buf[ofs++] = c;
        return *this;
    }
    fmt& str(std::string_view str) {
        for (size_t i = 0; i < str.size(); i++) {
            buf[ofs++] = str[i];
        }
        return *this;
    }
    fmt& dec(int value) {
        return *this;
    }
    fmt& nl() {
        ch('\n');
        return *this;
    }

    operator std::string_view() const {
        return std::string_view(buf, ofs);
    }
};

size_t strlen(const char* str) {
    int len = 0;
    while (*str++) len++;
    return len;
}
