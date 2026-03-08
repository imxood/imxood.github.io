# make 笔记

[GNU make](https://www.gnu.org/software/make/manual/make.html)

## 文档整理说明

- 本文档是旧版零散 `make` 记录收敛后的主文档.
- 后续统一以当前路径为准, 不再保留根路径兼容页.

## 查看所有 targets

```makefile
.PHONY: no_targets__ list
no_targets__:
list:
	sh -c "$(MAKE) -p no_targets__ | awk -F':' '/^[a-zA-Z0-9][^\$$#\/\t=]*:([^=]|$$)/ {split(\$$1,A,/ /);for(i in A)print A[i]}' | grep -v '__\$$' | sort"
```

## 变量

    赋值
        x = yes
        x ?= yes, 如果 x 未定义则赋值, 否则不处理
        x := $(y) yes, 只使用前面已经确定的变量值
        x += yes, 追加新的值

    延迟展开与立即展开
        x = $(y) yes, y 的值会根据后续定义继续变化
        x := $(y) yes, y 的值在当前行就确定

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

## Makefile 规则

    Targets: Prerequisites [;Command]
        Command

    Command 必须以 [Tab] 开始.

    当规则的 target 是一个文件时, 它的任何一个依赖文件被修改后, 在执行 `make <target>` 时这个目标文件都会被重新生成.

### 普通依赖与 order-only 依赖

    target : normal-prerequisites | order-only-prerequisites

- `normal-prerequisites` 是普通依赖.
- `order-only-prerequisites` 只保证执行顺序, 不会因为其变化而触发目标重新生成.

示例:

    test: | $(OUTPUT_DIR)

## 伪目标

```makefile
.PHONY : clean

clean :
    [-]rm *.o *.bin *.lst a.out
```

## 自动变量

```makefile
foo.o : foo.c
    cc -c $(CFLAGS) $^ -o $@
```

- `$@`, 目标文件
- `$^`, 所有依赖文件
- `$<`, 第一个依赖文件

## 常用函数

### 过滤与替换

    $(filter-out a.cpp, $(SRC)), 从 SRC 中排除 a.cpp

    patsubst: 模式替换
    obj = $(patsubst %.c, %.o, $(names))

    subst: 普通字符串替换

### foreach

    $(foreach <var>,<list>,<text>), 循环

    names := a b c d
    files := $(foreach n,$(names),$(n).o)
    得到 files 为 "a.o b.o c.o d.o"

### wildcard / notdir / dir

    wildcard: 扩展通配符
    sources = $(wildcard *.c ./sub/*.c)

    notdir: 去除路径
    names = $(notdir $(sources))

    dir: 只保留路径

### call

    call 函数是唯一一个可以创建定制化参数函数的引用函数

### MAKECMDGOALS

    "MAKECMDGOALS" 记录了命令行参数指定的终极目标列表

## 模式替换示例

    foo := a.o b.o l.a c.o
    bar := $(foo:%.o=%.c)

    sets 'bar' to 'a.c b.c l.a c.c'

## utils

```makefile
# Function to traverse directory tree and find all files matching pattern from list
# Usage: $(call search_tree,src/,*.c *.cpp)
search_tree = $(sort $(if $1,$(foreach d,$(wildcard $1/*),$(call $0,$d,$2)$(filter $(subst *,%,$2),$d))))

# recursive wildcard
rwildcard = $(foreach d,$(wildcard $1*),$(call rwildcard,$d/,$2) $(filter $(subst *,%,$2),$d))

# Same as above, but removes directory prefix
search_dir = $(patsubst $1/%,%,$(call search_tree,$1,$2))

# Generates subdirectories recursively
# Usage $(call subdirs,path)
subdirs = $(filter-out .,$1 $(if $(filter-out .,$(call dirx,$1)),$(call $0,$(call dirx,$1))))

# Directory part of path but with trailing slash removed
dirx = $(sort $(patsubst %/,%,$(dir $1)))

# Changes \ to /, removes '/./' and trailing /.
normalize_path = $(patsubst %/.,%,$(subst /./,/,$(subst \,/,$1)))

# Converts relative path to absolute path
# Usage: $(call resolve_path,root,path_list)
resolve_path = $(call normalize_path,$(foreach p,$2,$(if $(call is_abs,$p),$p,$(addprefix $1/,$p))))

# Generates a build rule for a list of targets (T represent a target within a rule)
# Usage: $(call generate_rules,build_rule_name,targets)
generate_rules = $(foreach T,$2,$(eval $(value $1)))

# Escape given list of characters in text with escape character
# Usage: $(call esc,characters,escape_with,text)
esc = $(if $(firstword $1),$(subst $(firstword $1),$2$(firstword $1),$(call $0,$(wordlist 2, 100,$1),$2,$3)),$3)

# Modifies system PATH by prepending provided arguments. No spaces are allowed
# Usage $(call make_PATH,path1 path2)
make_PATH = $(eval export PATH = $(subst $(space),,$(foreach p,$1,$(call sys_path,$p)$(PATH_SEP)))$(PATH))

# Convert path to section name prefix: change / to . and remove src.
dir_to_sect = .$(subst src.,,$(subst /,.,$(basename $(1))))
```
