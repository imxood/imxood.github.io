
## 文档

    https://www.rt-thread.org/document/site/#/rt-thread-version/rt-thread-standard/README

    开发环境: 
        硬件: Art-pi stm32h750
        开发环境: Deepin OS

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
