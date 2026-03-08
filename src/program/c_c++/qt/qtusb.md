# QtUsb

## 说明

- `QtUsb` 是一个把 USB 访问封装到 Qt 风格接口中的第三方库.
- 适合在 Qt 应用里快速接入基于 `libusb` / `hidapi` 的设备访问能力.

## 适用场景

- 在 Qt 桌面程序中做 USB 设备枚举和基础通信.
- 需要用 Qt 信号槽和对象模型封装底层设备访问逻辑.
- 想先快速验证 USB 访问能力, 再决定是否直接下沉到原生 `libusb` 层.

## 获取源码

```sh
git clone https://github.com/fpoussin/QtUsb.git
```

## 安装依赖

```sh
sudo apt install libusb-1.0-0-dev libhidapi-dev
```

## 编译方式

- 在 Qt Creator 中导入项目.
- 解析依赖后直接编译即可.

## 常见注意点

- 若目标平台不同, 还要进一步核查底层 `libusb` / `hidapi` 的可用性和设备权限.
- Linux 下如果设备无法访问, 优先检查 `udev` 规则.
- 如果只是验证底层 USB 能否工作, 也可以先用更直接的 `libusb` 示例做独立排查.
