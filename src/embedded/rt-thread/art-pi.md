
## 文档

    https://www.rt-thread.org/document/site/#/rt-thread-version/rt-thread-standard/README

    开发环境: 
        硬件: Art-pi stm32h750
        开发环境: Deepin OS

## 入门

    环境：

        sudo apt-get install libncurses5-dev

        pip3 install --user scons 或 sudo apt-get install scons

        可选： sudo apt-get install qemu

    
    代码：
        
        git clone https://github.com/RT-Thread/rt-thread.git

        cd rt-thread/bsp/stm32/stm32h750-artpi-h750

    编译
        scons

    烧写：

        openocd -f board/stm32h750b-disco.cfg

        arm-none-eabi-gdb -ex "target extended-remote :3333" ./rt-thread.elf

            monitor halt
            load

        telnet 127.0.0.1 4444

            load_image ./rt-thread.elf
            reset run

## 更新 在线包

    使用 "scons --menuconfig" 后 会默认安装及初始化Env环境, 并在home目录下生成".env"目录

    执行 "source ~/.env/env.sh" 才可以使用 pkgs 命令

    pkgs --update

## 配置 wifi tcp 通信

    git clone https://github.com/RT-Thread-Studio/sdk-bsp-stm32h750-realthread-artpi.git

    cd sdk-bsp-stm32h750-realthread-artpi/projects/art_pi_wifi

    先创建链接：
        ln -s ../../rt-thread rt-thread
        ln -s ../../libraries libraries

    就可以运行 GUI 了：
        scons --menuconfig

    生成 vscode 项目：
        scons --target=vsc

    编译：
        scons -j 40

    烧写：
        st-flash write rtthread.bin 0x8000000

    烧写后， 看串口终端有了 输出. 网络初始化后,
    
    扫描 wifi:
        wifi scan

    加入热点：
        wifi join HOT-NAME HOT-PASSWORD

## flash

    Flash 存储是按块组织的, 在使用时也倾向于按块访问才更加高效.

        在写入数据时, 需要先将所写位置所属的块擦除, 不管你是不是只写一个字节. 所以如果要改写Flash中的数据, 总是先将数据所属的块缓存到内存中, 然后再在内存中改写好数据后又重新写回块, 这样就不会丢失数据, 但是开销很大.

        在读数据时, 往往也是先定位块的位置, 然后在块中顺序读取, 在不同块中 间断读取数据时非常低效的.

    但是: NOR Flash 读取数据时可以做到任意的寻址而不会有太大的花销, 它的读操作是接近于RAM的，而写操作依然延续了按块擦除然后再按块写的特点

    正因为这些特性，Flash通常用于存储不需要频繁改动的掉电不能丢失的数据
