.PHONY: clean all

TARGETS = $(basename $(wildcard *.c))

all: $(TARGETS)


$(TARGETS):%:%.c

# 生成list汇编文件
	# gcc -masm=intel -S -o $@.s $^

# 生成C与汇编混排的列表
	gcc -c -g -Wa,-adlhn $^ >  $@.s

# 生成可执行文件和map文件
	gcc -g -o $@.bin $^ -Wl,-Map,$@.map


clean:
	rm -rf *.o *.out *.lst *.bin *.s
