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
        if (value < 0) {
            ch('-');
            value = -value;
        }
        int start = ofs;
        do {
            ch('0' + value % 10);
            value /= 10;
        } while (value);
        for (int end = ofs; end > start + 1; ) {
            std::swap(buf[start++], buf[--end]);
        }
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

static size_t strlen(const char* str) {
    int len = 0;
    while (*str++) len++;
    return len;
}

static int memcmp(const void* ptr1, const void* ptr2, size_t num) {
    const uint8_t* p1 = static_cast<const uint8_t*>(ptr1);
    const uint8_t* p2 = static_cast<const uint8_t*>(ptr2);

    for (size_t i = 0; i < num; i++) {
        if (p1[i] != p2[i]) {
            return p1[i] - p2[i];
        }
    }
    return 0;
}
