MAKEFLAGS += --no-builtin-rules
.SUFFIXES:

DLLS=advapi32 bass ddraw dinput dsound gdi32 kernel32 ntdll ole32 oleaut32 retrowin32_test ucrtbase vcruntime140 version user32 wininet winmm

all: $(foreach dll,$(DLLS),dll/$(dll).dll)

# Gross: we want to wildcard all the .rs files in the per-dll source dir, but need to filter
# out the builtin.rs only after expanding the % variable.
# This requires Makefile's "second expansion".
.SECONDEXPANSION:
src/winapi/%/builtin.rs dll/%.s dll/%.def: Makefile derive/src/*.rs src/*.rs src/winapi/* $$(filter-out %builtin.rs,$$(wildcard src/winapi/$*/*))
	cargo run --quiet --release -p win32-derive -- --dll-dir dll src/winapi/$* && \
	touch src/winapi/$*/builtin.rs

dll/%.dll: dll/%.s dll/%.def
	clang-cl -fuse-ld=lld -target i586-pc-windows-msvc \
        dll/$*.s \
        /link /dll /def:dll/$*.def /out:dll/$*.dll \
        /Brepro /safeseh:no /noentry /nodefaultlib /subsystem:console \
        ../lib/retrowin32.lib

