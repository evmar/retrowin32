DLLS=ddraw.rs dsound.rs gdi32.rs kernel32.rs user32.rs
dll.rs: Makefile gen/src/*.rs $(DLLS)
	cargo run -p gen -- $(DLLS) > $@
