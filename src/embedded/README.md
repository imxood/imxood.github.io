# 嵌入式总览

## 说明

- 本目录收录 MCU, SoC, 调试链路, 通信协议, 驱动与板级开发相关知识.
- 若内容更偏通用器件, 拆解观察和硬件档案, 应优先放在 `hardware/`.
- 若内容更偏桌面应用, Web 或通用编程语言, 应回到对应语言目录.

## 平台与芯片

- [ESP32 总览](./esp32/README.md)
- [AG32 平台总览](./ag32/README.md)
- [FPGA 总览](./fpga/README.md)
- [STM32F103 BluePill 总览](./stm32f103_bluepill/README.md)
- [RK3566 开发环境与镜像构建](./rk3566/rk3566.md)
- [CH32V307VCT6 疑难问题](./wch/ch32v307vct6.md)

## 基础主题

- [Rust 嵌入式开发环境](./Rust嵌入式.md)
- [C语言](./C语言.md)
- [GDB 用法](./gdb.md)
- [GPIO和AFIO](./GPIO和AFIO.md)
- [DMA 笔记](./dma笔记.md)
- [片上 Flash 读写](./flash.md)

## 工具链与框架

- [PlatformIO](./platformio.md)
- [TensorFlow Lite Micro](./TensorFlowLite.md)

## 板级设计与调试

- [PCB 设计](./pcb.md)
- [调试 STM32F767](./调试.md)
- [Intel HEX 文件格式](./hex.md)
- [天线匹配](./天线匹配.md)
- [TVS 管](./tvs管.md)

## 总线, 模块与电路

- [基本通信协议 笔记](./基本通信协议.md)
- [蓝牙笔记](./蓝牙笔记.md)
- [UART 笔记](./uart.md)
- [SPI 协议](./spi协议.md)
- [模块与扩展器件总览](./单片机模块.md)
- [模块与扩展器件归档](./模块/README.md)
- [嵌入式电路记录](./电路/README.md)
- [传感器总览](./传感器.md)
- [蜂鸣器](./蜂鸣器.md)
- [音频功率放大器](./音频功率放大器.md)

## 阅读路径

- 芯片平台上手优先从具体平台目录的 `README.md` 开始.
- 需要查接口协议时, 先看 `基本通信协议`, 再看具体主题页.
- 需要查模块接入时, 优先看 `模块与扩展器件总览` 和 `模块/README.md`.
- 需要看板级电路思路时, 再进入 `电路/README.md`.

## 补充记录

- [嵌入式知识总结](./总结.md)

## 目录边界

- `embedded/` 放嵌入式工程知识和开发链路.
- `hardware/` 放器件档案, 拆解记录和设备观察.
- `tools/` 放辅助开发工具和运维工具.
