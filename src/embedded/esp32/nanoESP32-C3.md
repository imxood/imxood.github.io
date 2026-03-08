# nanoESP32-C3

## 说明

- 本页作为 `nanoESP32-C3` 板级开发记录的轻量总览页.
- 当前仓库里已分散记录了调试, 串口控制台, `ESP-IDF` 常用命令和 Rust 相关内容.
- 本页的目标是把这些入口收敛成一个可快速定位问题的板级索引.

## 适用场景

- 初次拿到 `nanoESP32-C3` 开发板, 需要确认基础开发链路.
- 调试串口日志, 下载方式, 板级连线和最小样例运行问题.
- 在 `ESP-IDF`, `PlatformIO`, `Rust` 三套开发方式之间切换时做入口导航.

## 建议阅读路径

1. 环境和命令问题先看 [ESP-IDF 常用命令](./esp.md).
2. 板级串口与日志问题优先看 [nanoESP32-C3 串口与日志设置](./esp32c3_uart.md).
3. 如果是板子资料或开发板定位问题, 先看 [nanoESP32-C3 开发板](./esp32c3.md).
4. 如果是 Rust 方向, 再看 [esp-rs](./esp-rs.md) 与相关实验记录.

## 常见关注点

- 下载前先确认目标芯片型号和串口端口没有选错.
- 串口日志异常时, 优先检查输出通道, 波特率和控制台配置.
- 调试器连不上时, 先排查 `OpenOCD`, 目标配置和线序.
- 构建失败时, 先区分是 `ESP-IDF`, `Arduino/PlatformIO`, 还是 Rust 工具链问题.

## 相关主题

- [nanoESP32-C3 开发板](./esp32c3.md)
- [nanoESP32-C3 串口与日志设置](./esp32c3_uart.md)
- [ESP-IDF 常用命令](./esp.md)
- [PlatformIO Arduino 环境](../platformio.md)
- [esp-rs](./esp-rs.md)
