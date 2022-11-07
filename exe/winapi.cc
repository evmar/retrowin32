// Currently unused: demo of using only the Windows API, no C runtime.

#define WIN32_LEAN_AND_MEAN
#define STRICT
#include <windows.h>

void mainCRTStartup(void) {
  auto hStdout = GetStdHandle(STD_OUTPUT_HANDLE);
  static const char buf[] = "hello\n";
  bool ok = WriteFile(hStdout, buf, sizeof(buf) - 1, nullptr, nullptr);
}
