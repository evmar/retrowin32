#!/bin/sh

for asm in *.s; do
    module=$(basename $asm .s)
    ~/.local/opt/llvm/bin/clang-cl -fuse-ld=lld -target i586-pc-windows-msvc \
        -vctoolsdir $xwin_path/crt -winsdkdir $xwin_path/sdk \
        $asm \
        /link /dll /def:$module.def /out:$module.dll \
        /safeseh:no /noentry /nodefaultlib /subsystem:console
done
