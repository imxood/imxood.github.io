# ESP32 总览

## 说明

- 本目录收录 `ESP32` 系列芯片相关的环境搭建, 调试, 日志, Rust 与应用记录.
- 当前内容以 `ESP32-C3` 为主, 后续可继续按芯片型号和主题收敛为更稳定的入口结构.

## 快速入口

- [ESP-IDF 常用命令](./esp.md)
- [ESP32-C3 环境搭建](./esp32c3环境搭建.md)
- [nanoESP32-C3](./nanoESP32-C3.md)
- [nanoESP32-C3 串口与日志设置](./esp32c3_uart.md)
- [nanoESP32-C3 OpenOCD 调试](./esp32c3.md)

## Rust 相关

- [esp-rs](./esp-rs.md)
- [esp32c3 rust 学习笔记](./esp32c3-rust学习笔记.md)
- [ESP32-S3 Rust 环境](./esp32s3-rust.md)
- [espup Rust 环境](./espup-rust.md)
- [esp-rs Rust 环境分析](./rust环境分析.md)

## 无线与系统

- [蓝牙相关总结](./esp32c3_ble.md)
- [ESP32 USB 烧写与日志输出](./usb烧写方式.md)
- [应用程序的启动流程](./应用程序的启动流程.md)

## 建议阅读路径

1. 初次上手优先看 [ESP32-C3 环境搭建](./esp32c3环境搭建.md) 和 [ESP-IDF 常用命令](./esp.md).
2. 板级联调阶段重点看 `nanoESP32-C3` 和串口日志设置页面.
3. 做 `Rust` 方向时, 从 `esp-rs` 与 `espup Rust 环境` 开始, 再看具体芯片实验记录.
4. 做无线功能时, 优先看 `蓝牙相关总结`, 再结合官方 API 文档核对广播, GATT 和地址配置细节.

## 常见关注点

- 区分 `ESP-IDF`, `Arduino`, `Rust` 三套开发链路, 避免环境变量和工具链相互污染.
- Windows 下若串口, 烧录或监视器异常, 先检查驱动, 端口占用和目标芯片设置.
- 芯片型号切换时, 先确认目标架构, Flash 配置, 下载方式和板级管脚定义是否一致.
