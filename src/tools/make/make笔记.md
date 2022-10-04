# make基本笔记

ps: 命令中的[..]代表是可选项, 加了后意义自然有了不同

## 查看所有的targets

.PHONY: no_targets__ list
no_targets__:
list:
	sh -c "$(MAKE) -p no_targets__ | awk -F':' '/^[a-zA-Z0-9][^\$$#\/\\t=]*:([^=]|$$)/ {split(\$$1,A,/ /);for(i in A)print A[i]}' | grep -v '__\$$' | sort"

## 变量

    赋值
        x = yes
        x ?= yes, 如果 x 未定义 则 x = yes, 否则 不处理

        x = $(y) yes, y的值会根据前文或后文推导, 即这一行代码后面如果更新了 y 的值, 则 x 的值也会变化 (即: x变量的值将会在整个makefile的最后被确定)
        x := $(y) yes, y的值只能使用前面定义的变量

        x += yes, 追加新的值, 作为一个列表了, 如: a b c

    打印变量
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

## 自动变量

    foo.o : foo.c
        cc -c $(CFLAGS) $^ -o $@

    $@, 目标文件
    $^, 依赖文件

# make 函数

    从SRC中排除指定的内容:
        $(filter-out a.cpp, $(SRC)), 从 SRC 中排除 a.cpp

    子字符串替换:
        TARGETS=111.cpp 222.cpp 333.cpp
        OTARGETS=$(subst cpp,o,$(TARGETS)), 把 TARGETS变量 中的值中的 "cpp" 替换为 "o"

    保留文件名, 去除路径:
        $(notdir "a/b.c"), 得到 "b.c"

    保留路径, 去除文件名:
        $(dir "a/b.c"), 得到 "a/"


## 自动变量

    $<

    第一个依赖文件的名称,如果依赖目标是以模式（即“%” ）定义的，那么“$<”将是符合模式的一系列的文件集。注意，其是一个一个取出来的。

    $@

    目标文件的完整名称（目标集合）在模式规则中，如果有多个目标，那么“$@”就是匹配于目标中模式定义的集合。注意其目标是一个一个取出来的。

    $*

    不包括后缀名的当前依赖文件的名称, 这个变量表示目标模式中"%"及其之前的部分。
    如果目标是"dir/a.foo.b"，并且目标的模式是"a.%.b"，
    那么，"$*"的值就是"dir/a.foo"。这个变量对于构造有关联的文件名是比较有较。
    如果目标中没有模式的定义，那么"$*"也就不能被推导出，
    但是，如果目标文件的后缀是make所识别的，那么"$*"就是除了后缀的那一部分。
    例如：如果目标是"foo.c"，因为".c"是make所能识别的后缀名，所以，"$*"的值就是"foo"。

    $+

    所有的依赖文件，以空格分开，并以出现的先后为序，可能包含重复的依赖文件

   

    $?

    所有时间戳比目标文件晚的依赖文件，并以空格分开

    $^

    所有不重复的目标依赖文件，以空格分开（区分$+）

    -

    告诉make命令忽略所有的错误

    @

    告诉make在执行命令前不要将命令显示在标准输出上

## 模式替换

变量中的模式替换

    OBJS := $(CSRCS:%.c=$(OBJODIR)/$(subdir_path)/%.o)

规则中的模式替换