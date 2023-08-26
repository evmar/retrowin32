DLL_SRC=bass.rs ddraw.rs dsound.rs gdi32.rs kernel32/ ole32.rs oleaut32.rs retrowin32.rs user32.rs winmm.rs
DLLS=$(foreach dll,$(DLL_SRC),src/winapi/$(dll))
src/winapi/builtin.rs: Makefile derive/src/*.rs src/*.rs src/winapi/* src/winapi/*/*
	cargo run -p win32-derive -- $(DLLS) > $@
