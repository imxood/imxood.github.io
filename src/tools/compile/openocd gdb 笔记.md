# openocd笔记

## openocd 编译 安装

    sudo apt install libusb-1.0-0-dev

    git clone https://github.com/ntfreak/openocd.git

    编译
        ./bootstrap
        ./configure
        make -j
        sudo make install

    vscode 调试 openocd 本身， 编译：
        export CFLAGS="-O0"
        ./bootstrap
        ./configure --prefix=/develop/sources/openocd/output
        make -j
        make install

    配置：
        {
            "type": "lldb",
            "request": "launch",
            "name": "debug openocd",
            "program": "${workspaceFolder}/output/bin/openocd",
            "args": ["-f", "board/stm32h750b-disco.cfg"],
            "cwd": "${workspaceFolder}"
        },



## openocd调试stm32f7

    openocd -f board/stm32f7discovery.cfg


## 在linux中当前用户执行openocd， 报错: unable to open CMSIS-DAP device 0xc251:0xf001

    sudo vim /etc/udev/rules.d/cmsis-dap.rules, 添加一下内容(记得更改组):
        ATTRS{idVendor}=="c251", ATTRS{idProduct}=="f001", MODE="664", GROUP="maxu"

    sudo udevadm control --reload


## 常用命令

    openocd -f interface/cmsis-dap.cfg -f target/stm32h7x.cfg, 使用cmsis-dap调试stm32h750

    openocd -f board/stm32f7discovery.cfg

    telnet localhost 4444

    program <filename> [address] [verify] [reset] [exit], 写可执行程序到flash上, 只有binary images才需要address

        ex: program output.bin 0x08000000 reset; halt; reg; resume; exit

    halt, 进入挂起状态

    reg [REG [value]] , 查看或设置寄存器, 可能需要在halt状态下

    resume, 恢复到当前代码段

    reset [run|halt|init], 重置, 进入特定状态, 默认是run

    reg, 查看寄存器内容, 在halt状态下才会显示内容

    targets, 显示所有目标

## 使用gdb和openocd实现远程下载

    arm-none-eabi-gdb -q --readnow -ex "target extended-remote 3333" -ex "b main" -ex "monitor halt" -ex "load" -ex "c" -ex "monitor resume" -ex "echo ok\n" -ex "disconnect" -ex "q"
    为了远程的下载程序到sram中, 我发现用gdb在load后自动设置的pc, 用i r看是对的, 用'continue'是可以的, 但是阻塞的, 用了'monitor resume', 但是用monitor reg pc查看pc, 却是每2次中1对1错, 很有规律,,,,,,,,,,用上面的命令: 先给程序加断点,'c', 自动走到断点处, 执行'monitor resume', 退出, 就解决了


## openocd实现重启到指定地址

    gcc 编译选项:
    LDFLAGS += -Wl,-Map=output.map

    openocd -f board/stm32f7discovery.cfg -c "program output.bin ${flash_to_addr} reset; halt; reg pc ${reset_entry}; resume; exit"

# openocd 用法

如果你的设备支持openocd调试, 那么调试器连接上PC, 在PC上运行openocd, 就作为一个Server, 可调式, 可烧写

下面两种方法的过程都是一样的:
    1. load image
    2. 打印向量表的位置, 要显示2个字
    3. 挂起状态
    4. 设置sp指针, 设置pc指针
    5. 恢复

向量表的地址, 符号在startup.s中, 根据符号在map文件中找到具体的地址


## gdb 连接 openocd, 使用记录

    调试文件需要在开发机上

    (gdb) target remote 10.239.71.34:3333
    Remote debugging using 10.239.71.34:3333
    0x7000dd5a in ?? ()
    (gdb) load app.elf
    Loading section text, size 0x1e8 lma 0x60000000
    Loading section text_2, size 0x1e14e lma 0x600001e8
    Loading section .ARM.exidx, size 0x8 lma 0x6001e338
    Loading section sw_isr_table, size 0x350 lma 0x6001e340
    Loading section devconfig, size 0x294 lma 0x6001e690
    Loading section shell_root_cmds_sections, size 0xc0 lma 0x6001e924
    Loading section rodata, size 0x2161c lma 0x6001e9e4
    Loading section datas, size 0x2079 lma 0x60050498
    Loading section initlevel, size 0x194 lma 0x60052514
    Loading section _static_thread_area, size 0x60 lma 0x600526a8
    Loading section _k_mem_slab_area, size 0x54 lma 0x60052708
    Loading section _k_mem_pool_area, size 0x1c lma 0x6005275c
    Loading section _k_sem_area, size 0x168 lma 0x60052778
    Loading section _k_mutex_area, size 0xc8 lma 0x600528e0
    Loading section _k_queue_area, size 0x14 lma 0x600529a8
    Start address 0x60011690, load size 271647
    Transfer rate: 34 KB/sec, 9054 bytes/write.
    (gdb) x/2xw 0x60000000
    0x60000000 <_vector_table>:	0x6004e5a0	0x60011691
    (gdb) set $pc=0x60011691
    (gdb) set $sp=0x6004e5a0
    (gdb) c
    Continuing.


## telnet连接openocd 使用记录

    用telnet这种办法是远程操作, 调试文件需要在远程, 文件路径是启动openocd时的路径

    mx@maxu-pc:/data$ telnet 10.239.71.34 4444

    Trying 10.239.71.34...
    Connected to 10.239.71.34.
    Escape character is '^]'.
    Open On-Chip Debugger
    >
    > load_image ./bin/app.elf
    456784 bytes written at address 0x60000000
    5368 bytes written at address 0x6006f850
    downloaded 462152 bytes in 13.153211s (34.313 KiB/s)
    >
    > halt
    >
    > targets
        TargetName         Type       Endian TapName            State
    --  ------------------ ---------- ------ ------------------ ------------
    0* ose.cpu            cortex_m   little ose.cpu            halted
    >
    >
    > ose.cpu mdw 0x60000000 2
    0x60000000 60100000 600004b5                   ...`...`
    > reg sp 0x60100000
    sp (/32): 0x60100000
    > reg pc 0x600004b5
    pc (/32): 0x600004B5
    >
    > resume

    关于"ose.cpu mdw", 这样内存读写命令, 可以通过"help mem"的形式搜索, 有很多类似的命令, 都可以实现查看内存



## 自定义proc

    stmh7_cmsis.cfg:

        source [find interface/cmsis-dap.cfg]

        transport select swd

        source [find target/stm32h7x.cfg]

        proc flash () {
            init
            halt
            flash write_image erase rtthread.bin 0x08000000
            reset run
            shutdown
        }

    使用:
        openocd -f stmh7_cmsis.cfg -c "flash ()"

## 使用 cmsisdap 调试 stm32

    choco install make openocd gcc-arm-embedded

    创建文件 dap-stm32.cfg, 添加以下内容:

        interface cmsis-dap
        transport select swd
        source [find target/stm32f1x.cfg]

    烧写程序:
        openocd -f ocd-stm32.cfg -c "program build/stm32f103rct6.bin 0x8000000 reset exit"

    打开 launch.json, 新添加一项调试配置:

	{
		"name": "Cortex Debug",
		"cwd": "${workspaceRoot}",
		"executable": "build/stm32f103rct6.elf",
		"request": "launch",
		"type": "cortex-debug",
		"servertype": "openocd",
		"configFiles": [
			"dap-stm32.cfg"
		]
	}

    必要时, 可以添加 armToolchainPath, 但是 openocd 的路径好像不能添加, 只能设置环境变量

## arm-none-eabi-gdb

    info target
    info mem 显示内存情况， 包括 ROM 和 RAM

    where 显示当前程序执行位置

    monitor target
    monitor targets

    monitor flash banks 查看 flash
