
all: $(wildcard *.dll)

%.dll: %.s %.def
	~/.local/opt/llvm/bin/clang-cl -fuse-ld=lld -target i586-pc-windows-msvc \
        $*.s \
        /link /dll /def:$*.def /out:$*.dll \
        /Brepro /safeseh:no /noentry /nodefaultlib /subsystem:console \
        ../lib/retrowin32.lib
