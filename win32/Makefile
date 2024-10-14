DLL_SRC=advapi32.rs bass.rs ddraw/ dinput.rs dsound.rs gdi32/ kernel32/ ntdll.rs ole32.rs oleaut32.rs retrowin32_test.rs ucrtbase.rs vcruntime140.rs version.rs user32/ wininet.rs winmm/
DLLS=$(foreach dll,$(DLL_SRC),src/winapi/$(dll))
src/winapi/builtin.rs: Makefile derive/src/*.rs src/*.rs src/winapi/* src/winapi/*/*
	cargo run -p win32-derive -- --dll-dir dll --builtins $@ $(DLLS)
	make -C dll -j
