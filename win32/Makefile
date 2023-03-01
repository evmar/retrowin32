DLL_NAMES=ddraw.rs dsound.rs gdi32.rs kernel32.rs ole32.rs oleaut32.rs user32.rs winmm.rs
DLLS=$(foreach dll,$(DLL_NAMES),src/winapi/$(dll))
src/winapi/dll.rs: Makefile derive/src/*.rs src/*.rs $(DLLS)
	cargo run -p win32-derive -- $(DLLS) > $@
