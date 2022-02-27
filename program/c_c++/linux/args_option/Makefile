.PHONY: clean all

TARGETS = $(basename $(wildcard *.c))

all: $(TARGETS)


$(TARGETS):%:%.c

# 生成list汇编文件
	gcc -masm=intel -S -o $@.s $^

# 生成可执行文件, 加.bin后缀, 是为了便于clean
	gcc -g -o $@.bin $^

clean:
	rm -rf *.o *.out *.lst *.bin *.s
