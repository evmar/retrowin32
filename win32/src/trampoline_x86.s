# See doc/x86-64 section "Trampoline assembly".

# Target for 64->32bit transition.
tramp32:
    movl %edi, %esp
    calll *%esi
    lretl   # long ret to 64-bit mode

# This code is the x86 entry point for 32->64bit calls.
# We stamp out a separate copy of it per entry point.
tramp64:
    pushl %edi  # backup
    movl $0x11, %edi  # 0x11=shim index
    lcalll $0x22, $0x33  # call trans64; 0x22=selector, 0x33=address
    popl %edi
    ret

crash:
    int3
    movl $1, %eax
    jmpl *%eax
