# GDB 用法

参考: [100个gdb小技巧](https://wizardforcel.gitbooks.io/100-gdb-tips/content/part13.html)

## GDB 命令行中调试

    连接stm32设备:
        openocd -f interface/stlink.cfg -f board/stm32f746g-disco.cfg,

    运行GDB:
        arm-none-eabi-gdb -q target/thumbv7em-none-eabihf/debug/app

    连接 openocd server:
        target remote :3333

    加载程序:
        load

    显示内存:
        x/2xw 0x60000000

    设置寄存器的值:
        set $pc=0x60011691
        set $sp=0x6004e5a0

    继续:
        c

    显示变量:
        info local VARIABLE_NAME

## gdb命令的参数

    设置
    --command=FILE
