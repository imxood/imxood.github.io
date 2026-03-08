# nanoESP32-C3 串口与日志设置

## 说明

- 本文记录 `ESP32-C3` 在 `ESP-IDF` 下的控制台串口, 波特率与日志输出相关设置.
- 适合在调试串口冲突, 日志过多或产线静默启动时参考.

## 定制控制台波特率

在 `menuconfig` 中可调整:

```text
Component config -> ESP System Settings
  -> Channel for console output (Custom UART)
  -> Channel for console secondary output (No secondary console)
```

典型配置示例:

- `UART TX on GPIO# = 21`
- `UART RX on GPIO# = 20`
- `UART console baud rate = 1000000`

## 关闭颜色输出

```text
Component config -> Log output -> Use ANSI terminal colors in log output
```

## 关闭 bootloader log

```text
Bootloader config -> Bootloader log verbosity -> No output
```

## 关闭程序 log

```text
Component config -> Log output -> Default log verbosity -> No output
```

## 关闭 console

```text
Component config -> ESP System Settings -> Channel for console output
```

## 使用建议

- 先统一下载口, 日志口和业务串口的分工, 避免引脚冲突.
- 若高波特率下日志异常, 优先检查 USB 转串口链路与终端配置.
- 量产或低噪声场景可关闭大部分日志, 但建议保留必要的错误级输出.
