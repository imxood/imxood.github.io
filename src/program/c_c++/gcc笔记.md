# gcc笔记

## 安装gcc9

    sudo add-apt-repository ppa:jonathonf/gcc-9.0
    sudo apt install g++-9

## attribute用法集锦

    __attribute__((weak)), 设置弱函数


## "arm-none-eabi-gdb --version"报错: libncurses.so.5: cannot open shared object file

    sudo apt install libncurses5


## arm-none-eabi-gcc 的 "-specs" gcc定义

    来自对文档的解读: gcc-arm-none-eabi-9-2020-q2-update/share/doc/gcc-arm-none-eabi/pdf/gcc/gcc.pdf

        3.19 Specifying Subprocesses and the Switches to Pass to Them

    查看默认的 specs:
        arm-none-eabi-gcc -dumpspecs

    语法:

        %command:
            向 spec 文件处理器 发起一个命令

            %include <file>
                搜索文件, 并把文件的内容插入到 spec file 当前的位置

            %include_noerr <file>
                同上, 当file not found 时, 不会报错

            %rename old_name new_name
                重命名

                一个例子：

                    %rename lib old_lib

                    *lib:
                    --start-group -lgcc -lc -leval1 --end-group %(old_lib)

                重命名一个 spec string, 也就是把一个叫"lib"的 spec string 名字 重命名叫 "old_lib"
                创建一个叫 "lib" 的 spec string, 并用"%(old_lib)"表示追加上原来的 spec string

        *[spec_name]:
            创建、覆盖 或 删除 一个 spec string

            通过 -dumpspecs 输出中可以看到，默认已经有了 asm, cpp, link 等这些 spec string 了

        +[spec_name]:
            追加 一个 spec string

        [suffix]:
            对特定后缀处理, 比如:

                .ZZ:
                    z-compile -input %i

            这表明对 .ZZ 后缀的文件 都会被传递给 "z-compile -input %i" 命令, %i 是输入文件

                .ZZ:
                @c++

            这里是 @language, 指定 c++ 的后缀别名, 和命令行参数 "-x" 很相似


        对于 spec strings 的 “%”符号的常用用法

        %(name)             在所在位置替换 spec string name 的内容
        %{S:X}              如果 gcc参数 有 "-S"选项， 那么替换 "-S" 为 "X"
        %{!S:X}             如果 gcc参数 没有 "-S"选项， 那么替换 "-S" 为 "X"
        %:function(args)    function 有很多,
                            replace-outfile 这个 spec 函数 需要两个参数, 用后一个替换前一个, 例子:
                                %{fgnu-runtime:%:replace-outfile(-lobjc -lobjc-gnu)}
        %G                  不知道啥意思: Process the libgcc spec. This is a spec string for deciding which GCC support
                            library is included on the command line to the linker.


## 解读 -spec=nosys.spec

    gcc-arm-none-eabi-9-2020-q2-update/arm-none-eabi/lib/thumb/v7-m/nofp/nosys.specs

        %rename link_gcc_c_sequence                nosys_link_gcc_c_sequence

        *nosys_libgloss:
        -lnosys

        *nosys_libc:
        %{!specs=nano.specs:-lc} %{specs=nano.specs:-lc_nano}

        *link_gcc_c_sequence:
        %(nosys_link_gcc_c_sequence) --start-group %G %(nosys_libc) %(nosys_libgloss) --end-group

    把默认的 link_gcc_c_sequence 换个名字， 叫 "nosys_link_gcc_c_sequence"
    定义了 3个 spec strings:
        nosys_libgloss, 定义了一个链接库的选项: -lnosys
        nosys_libc, 有"!"的那个, 如果 gcc参数中没有 "-specs=nano.specs" 那么把 "-specs=nano.specs" 替换为 "-lc"
                    没有"!"的那个, 如果 gcc参数中有 "-specs=nano.specs" 那么把 "-specs=nano.specs" 替换为 "-lc_nano"
        link_gcc_c_sequence, 先使用原来"link_gcc_c_sequence"定义的 spec string
                             再追加上新的 "--start-group %G %(nosys_libc) %(nosys_libgloss) --end-group"
                             并替换上面已定义的"nosys_libc"和"nosys_libgloss"


## gcc 优化

[gcc 优化](https://gcc.gnu.org/onlinedocs/gcc/Optimize-Options.html#Optimize-Options)

    gcc中指定优化级别的参数有：-O0、-O1、-O2、-O3、-Og、-Os、-Ofast

    默认为 -O0

    -Og 是在 -O1 的基础上，去掉了那些影响调试的优化，所以如果最终是为了调试程序，可以使用这个参数
