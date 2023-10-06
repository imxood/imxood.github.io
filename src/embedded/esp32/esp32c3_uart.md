# nanoESP32-C3 开发板

## 定制波特率

Component config → ESP System Settings
    -> Channel for console output (Custom UART) -> Channel for console secondary output (No secondary console)

(21) UART TX on GPIO# (NEW)
(20) UART RX on GPIO# (NEW)
(1000000) UART console baud rate

## 去掉 console color

(Top) → Component config → Log output -> Use ANSI terminal colors in log output

## 关闭 bootloader log

Bootloader config --> Bootloader log verbosity 选定为 No output

## 关闭程序 log

Component config -> Log output -> Default log verbosity 选定为 No output

## 关闭 console

(Top) → Component config → ESP System Settings → Channel for console output
