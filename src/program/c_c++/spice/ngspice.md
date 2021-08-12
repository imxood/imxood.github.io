# ngspice笔记

## linux 编译 ngspice

    下载code:
        https://sourceforge.net/projects/ngspice/

    mkdir release && cd release
    ../configure  --with-x --with-readline=yes --disable-debug

## 源码编译

    sudo apt install bison flex automake libtool libxaw7-dev libreadline-dev

    git clone https://git.code.sf.net/p/ngspice/ngspice

    cd ngspice

    ./autogen.sh

    mkdir -p debug && cd debug

    ../configure --with-ngshared --enable-cider --enable-xspice  --with-x --enable-stepdebug --enable-cpdebug

    make -j4
    sudo make install

    cd ..
    ngspice
    source examples/soi/inv_tr.sp


    如果发生错误: Undefined symbol error 'hcomp' when using libngspice.so
    可能是这有的操作:
        Clean folder -> normal build -> shared lib build -> problem occurs.
    请考虑, 应该可以解决:
        Clean folder -> shared lib build -> OK.

## 源码分析

    var_alloc(char *name, struct variable *next)

    void cp_vset(char *varname, enum cp_types type, void *value);

        设置一个参数列表: struct variable *variables; varname是不重复的


    ngSpice_Init():

        cp_vset("rndseed", CP_NUM, 1);

        cp_vset("sharedmode", CP_BOOL, True);

        ft_cpinit(): Set some standard variables and aliases, etc, and init the ccom stuff
            cp_init(); Initialize io, cp_chars[], variable "history" in init.c
            cp_coms: 即所有的命令, 比如: print, plot, load, tran, op


    ngSpice_Circ(netlist) 根据网表创建电路
        create_circbyline()
            更新circarray
            更新完后执行inp_spsource():




    struct CKTcircuit: 电路结构体

    struct circ
