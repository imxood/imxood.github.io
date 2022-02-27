.PHONY: all clean

all:
	nasm -f elf -gdwarf -l hello.lst hello.nasm && gcc -m32 hello.o -o hello.bin

	nasm -f elf -gdwarf -l test_callstack.lst test_callstack.nasm && gcc -m32 -no-pie test_callstack.o -o test_callstack.bin

	gcc -masm=intel -S -o disassembly.s disassembly.c


bootloader_hello: bootloader_hello.nasm
# 生成开机引导程序
	nasm -f bin bootloader_hello.nasm -o $@.bin
# 以十六进制输出文件内容
	@echo ""
	@hexdump bootloader_hello.bin


clean:
	rm -f *.o *.s *.lst *.bin *.log a.out
