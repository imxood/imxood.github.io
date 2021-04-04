# blackmagic

可以参考: https://paramaggarwal.medium.com/converting-an-stm32f103-board-to-a-black-magic-probe-c013cf2cc38c

## 编译 blackmagic

    git clone https://github.com/blacksphere/blackmagic.git
    cd blackmagic
    make -j
    make clean -j && make PROBE_HOST=stlink -j

    得到:
        bootloader:
            src/blackmagic_dfu.bin
        bmp:
            src/blackmagic.bin


## Flash new bootloader

### flash烧写

    使用 stlink 调试器, 使用软件 STM32CubeProgrammer 烧写 src/blackmagic_dfu.bin


### 串口烧写

    使用 stm32loader 或是 STM32CubeProgrammer 通过 serial 烧写fw:
        设置 boot0 --> 1, boot1 --> 0, 从系统存储器启动, 即 串口下载模式

    pip3 install --user stm32loader

    stm32loader -p /dev/ttyUSB0 -e -w -v src/blackmagic_dfu.bin

## Flash BMP

    烧写之后(如果没有自动reset, 按reset按钮), 执行 dmesg, lsusb, 可以看到 usb 设备:
        ID 1d50:6017 OpenMoko, Inc. Black Magic Debug Probe (DFU)

    sudo apt install dfu-util
    sudo dfu-util -d 1d50:6017 -s 0x8002000:leave -D ./src/blackmagic.bin

    执行后会有:
        /dev/ttyACM0 和 /dev/ttyACM1

    第一个是 GDB server, 第二个是 Serial.

## 连接你的设备和调试器

    调试:
        TARGET          DEBUGGER
        GND             GND
        SWDIO           PB14
        SWCLK           PA5
        POWER           3.3V

    串口:
        TARGET          DEBUGGER
        RXD             PA3
        TXD             PA2

    可参考: src/platforms/stlink/platform.h

    /* Hardware definitions... */
    #define TDI_PORT	GPIOA
    #define TMS_PORT	GPIOB
    #define TCK_PORT	GPIOA
    #define TDO_PORT	GPIOA

    #define TDI_PIN		GPIO7
    #define TMS_PIN		GPIO14
    #define TCK_PIN		GPIO5
    #define TDO_PIN		GPIO6

    #define USBUSART USART2
    #define USBUSART_PORT GPIOA
    #define USBUSART_TX_PIN GPIO2
