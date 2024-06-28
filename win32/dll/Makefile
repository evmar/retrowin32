
all: $(wildcard *.dll)

%.dll: %.s %.def
	~/.local/opt/llvm/bin/clang-cl -fuse-ld=lld -target i586-pc-windows-msvc \
        $*.s \
        /link /dll /def:$*.def /out:$*.dll \
        /safeseh:no /noentry /nodefaultlib /subsystem:console
