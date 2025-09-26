    .section .text
    .globl _start
_start:
    # r29 (x29) ← 5
    li x29, 5

    # r30 (x30) ← 100
    li x30, 100

    # [x30] ← x29
    sw x29, 0(x30)

    li a7, 93        # Linux RISC-V: exit syscall number
    li a0, 0         # return code 0
    ecall
