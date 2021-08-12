# make笔记

[GNU make](https://www.gnu.org/software/make/manual/make.html)

    $@, 目标文件
    $^, 所有的依赖文件
    $<, 第一个依赖文件

    $(foreach <var>,<list>,<text>), 循环

    names := a b c d
    files := $(foreach n,$(names),$(n).o)
    得到files为 "a.o b.o c.o d.o"


    foo := a.o b.o l.a c.o
    bar := $(foo:%.o=%.c)

    sets 'bar' to 'a.c b.c l.a c.c'



    target : normal-prerequisites | order-only-prerequisites,
    需要执行某个或某些规则，但不能引起生成目标被重新生成

    normal-prerequisites, 就是普通的target, 可省略
    order-only-prerequisites, 不会重新生成的目标

    test: | $(OUTPUT_DIR)


    subst

    wildcard : 扩展通配符
    sources=$(wildcard *.c ./sub/*.c), 获取所有的.c文件

    notdir ： 去除路径
    names=$(notdir $(sources)), 不含路径, 只有文件名

    dir : 只要路径

    patsubst ：替换通配符
    obj=$(patsubst %.c,%.o,$(names)), 替换后缀



    "MAKECMDGOALS" ，该变量记录了命令行参数指定的终极目标列表



    call函数是唯一一个可以创建定制化参数函数的引用函数


## utils


    # Function to traverse directory tree and find all files matching pattern from list
    # Usage: $(call search_tree,src/,*.c *.cpp)
    search_tree = \
    $(sort $(if $1,$(foreach d,$(wildcard $1/*),$(call $0,$d,$2)$(filter $(subst *,%,$2),$d))))

    # recursive wildcard
    rwildcard= \
    $(foreach d,$(wildcard $1*),$(call rwildcard,$d/,$2) $(filter $(subst *,%,$2),$d))

    # Same as above, but removes directory prefix
    search_dir = \
    $(patsubst $1/%,%,$(call search_tree,$1,$2))

    # Generates subdirectories recursively
    # Usage $(call subdirs,path)
    subdirs = \
    $(filter-out .,$1 $(if $(filter-out .,$(call dirx,$1)),$(call $0,$(call dirx,$1))))

    # Directory part of path but with trailing slash removed
    dirx = \
    $(sort $(patsubst %/,%,$(dir $1)))

    # Changes \ to /, removes '/./' and trailing /.
    normalize_path = \
    $(patsubst %/.,%,$(subst /./,/,$(subst \,/,$1)))

    # Converts relative path to absolute path
    # Usage: $(call resolve_path,root,path_list)
    resolve_path = \
    $(call normalize_path,$(foreach p,$2,$(if $(call is_abs,$p),$p,$(addprefix $1/,$p))))

    # Generates a build rule for a list of targets (T represent a target within a rule)
    # Usage: $(call generate_rules,build_rule_name,targets)
    generate_rules = $(foreach T,$2,$(eval $(value $1)))

    # Escape given list of characters in text with escape character
    # Usage: $(call esc,characters,escape_with,text)
    esc = $(if $(firstword $1),$(subst $(firstword $1),$2$(firstword $1),$(call $0,$(wordlist 2, 100,$1),$2,$3)),$3)

    # Modiefies system PATH by prepending provided arguments. No spaces are allowed
    # Usage $(call make_PATH,path1 path2)
    make_PATH = $(eval export PATH = $(subst $(space),,$(foreach p,$1,$(call sys_path,$p)$(PATH_SEP)))$(PATH))

    # Convert path to section name prefix : change / to . and remove src.
    dir_to_sect = \
    .$(subst src.,,$(subst /,.,$(basename $(1))))


