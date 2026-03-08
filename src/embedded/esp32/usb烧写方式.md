# ESP32 USB 烧写与日志输出

## 说明

- 本页记录通过 `USB Serial/JTAG Controller` 进行 ESP32 烧写和日志输出的基本配置方法.
- 适合作为 `idf.py dfu`, `flash`, `monitor` 的快速参考.

## 使用 USB 进行 烧写 与 日志输出

参考: https://docs.espressif.com/projects/esp-idf/zh_CN/latest/esp32s3/api-guides/dfu.html

    idf.py menuconfig

        Component config → ESP System Settings → Channel for console output

        (X) USB Serial/JTAG Controller

    idf.py dfu, 开始编译

    idf.py -p COM8 flash, 使用 USB串口下载

    idf.py -p COM8 monitor
