# nanoESP32-C3 开发板

## 使用 openocd 调试 esp32c3

### 编译 hidapi

    编译 openocd-esp32 失败:
        configure: error: hidapi is required for the CMSIS-DAP Compliant Debugger

    编译 hidapi:

    git clone https://github.com/Dashlane/hidapi.git
    cd hidapi
    ./bootstrap
    ./configure --enable-static --disable-shared
    make clean
    make
    sudo make install

### 编译 openocd-esp32

    git clone https://github.com/espressif/openocd-esp32.git
    cd openocd-esp32
    ./bootstrap
    ./configure --enable-cmsis-dap
    make -j
    sudo make install

    sudo systemctl restart udev

## Burn the efuse

the efuse JTAG_SEL_ENABLE should be burned to enable the jtag function.

    espefuse.py -p /dev/ttyACM0 burn_efuse JTAG_SEL_ENABLE

## 启动 openocd

    设置 GPIO10 到 GND 用于选择 GPIO function 到 JTAG, 重新给开发板上电

    openocd -f interface/cmsis-dap.cfg -f target/esp32c3.cfg -c "adapter_khz 10000"

    能调试, 但有两个大问题: 1.速度特别慢,esp的日志输出需要12s，LED电平翻转需要3s 2.openocd启动后,gdb可能有5次只能成功连上1到2次 体验极差,还是打Log最方便了...
