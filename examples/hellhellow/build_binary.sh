riscv64-unknown-linux-gnu-as -march=rv32im -o prog.o prog.s
riscv64-unknown-linux-gnu-ld -m elf32lriscv -o prog.elf prog.o
riscv64-unknown-linux-gnu-ld -m elf32lriscv -T link.ld -o prog.elf prog.o