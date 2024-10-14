DLL_SRC=advapi32/ bass/ ddraw/ dinput/ dsound/ gdi32/ kernel32/ ntdll/ ole32/ oleaut32/ retrowin32_test/ ucrtbase/ vcruntime140/ version/ user32/ wininet/ winmm/
DLLS=$(foreach dll,$(DLL_SRC),src/winapi/$(dll))
src/winapi/builtin.rs: Makefile derive/src/*.rs src/*.rs src/winapi/* src/winapi/*/*
	cargo run -p win32-derive -- --dll-dir dll --builtins $@ $(DLLS)
	make -C dll -j
