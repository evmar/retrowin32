# See doc/x86-64 section "Trampoline assembly".
#
# This code is the target for the 32->64bit transition
# and calls a 64-bit address given on the stack then
# does a far return (back to 32-bit mode).
#
# We embed a single copy of this function verbatim,
# calling it from each instance of the trampoline.

call64:
    # stack contents on entry:
    #   0   32-bit trampoline return address
    #   4   32-bit segment selector
    #   8   64-bit shim target addr (8 bytes)
    #   16  exe return address
    #   20... argn passed from exe

    # we're calling an 'extern "C"' Rust function like
    #   pub extern "C" fn WriteFile(machine: &mut Machine, esp: u32) {
    # calling convention args go in
    #   rdi, rsi, rdx, rcx
    # 32-bit stdcall: callee may clobber eax/ecx/edx,
    # 64-bit ccall: callee preserves rbx/rsp/rbp and others
    # so back up rdi/rsi here
    pushq %rdi
    pushq %rsi
    movq $0x123456789, %rdi   # machine
    leaq 32(%rsp), %rsi  # esp
    callq *24(%rsp)
    popq %rsi
    popq %rdi

    # We want 'far ret', https://www.felixcloutier.com/x86/ret opcode cb
    # It is only available in Clang if we use att syntax(!)
    # Note that it pops 32-bit CS/EIP, even though we are in 64-bit mode.
    lret $8  # ret to 0:4, clean target addr
