# make基本笔记

ps: 命令中的[..]代表是可选项, 加了后意义自然有了不同

## 打印变量

    $(warning $(XXX))
    $(error $(XXX))

## 判断

    ifeq ($(VALIDATION),1)
    ifdef PLATFORM
    $(error PLATFORM: $(PLATFORM) defined)
    else
    $(error PLATFORM: $(PLATFORM) undefined)
    endif
    endif

## Makefile规则

    Targets: Prerequisites[;Command]
        Command

    Command 必须以[Tab]开始, Command 可以写成多行, 通过来继行, 但行尾的后不能有空格

    当规则的 target 是一个文件, 它的任何一个依赖文件被修改后, 在执行 make <target>时这个目标文件都会被重新编译或重新连接

## 伪目标

    .PHONY : clean

    clean :
        [-]rm *.o *.bin *.lst a.out

## 变量定义

    x = yes
    x ?= yes, 如果 x 未定义, 则 x = yes

    x := $(y) yes, y只能使用前面定义的变量, 如果不要冒号, 则y的值会根据后文推导

    x += no, 追加

## 自动变量

    foo.o : foo.c
        cc -c $(CFLAGS) $^ -o $@

    $@, 目标文件
    $^, 依赖文件

## 小技巧

    @, 不显示执行命令
    -, 忽略命令执行错误

        all:
            -rm a.java
            @echo "hello"

    动态创建多个目标:

        TARGETS = $(basename $(wildcard *.c))

        $(TARGETS):%:%.c
            $(CC) $^ -o $@.bin

        all: $(TARGETS)

# make 函数

    $(filter-out a.cpp, $(SRC)), 从SRC中排除指定的内容
