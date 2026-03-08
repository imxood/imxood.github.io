# rusb

## 说明

- `rusb` 是 `libusb` 的 Rust 封装, 适合做跨平台 USB 设备枚举, 打开设备和基础收发实验.
- 本页记录 Windows + `vcpkg` 环境下跑通官方示例的最小步骤, 以及常见排查点.

## 适用场景

- 枚举当前主机上的 USB 设备.
- 在用户态程序中直接与特定 USB 设备通信.
- 为后续 `HID`, 自定义设备协议或板级工具开发做基础验证.

## 安装 `libusb`

```powershell
vcpkg install libusb --triplet=x64-windows
vcpkg install libusb --triplet=x86-windows
```

## 环境变量

```powershell
$env:VCPKGRS_TRIPLET="x64-windows"
$env:VCPKGRS_DYNAMIC=1
```

## 下载与运行示例

```sh
git clone https://github.com/a1ien/rusb.git
cd rusb
cargo run --example read_device
```

## 需要的运行库

将以下文件复制到项目根目录或确保其在运行时可被找到:

- `vcpkg\installed\x64-windows\bin\libusb-1.0.dll`
- `vcpkg\installed\x64-windows\lib\libusb-1.0.lib`

## 常见排查点

- 架构必须一致, 例如程序为 `x64`, 则 `libusb` 也应使用 `x64-windows`.
- Windows 上若设备无法打开, 先确认该设备是否已绑定兼容 `libusb` 的驱动.
- 若只是想先验证环境, 优先运行官方示例做枚举, 不要一开始就接入完整业务逻辑.
- 如果编译阶段找不到依赖, 优先检查 `VCPKGRS_TRIPLET` 与 `VCPKGRS_DYNAMIC` 是否生效.

## 代码中可显式设置的变量

```rust
std::env::set_var("VCPKGRS_TRIPLET", "x64-windows");
std::env::set_var("VCPKGRS_DYNAMIC", "1");

// std::env::set_var("TARGET", "x86_64-pc-windows-msvc");
// std::env::set_var("HOST", "x64-windows-static");
// std::env::set_var("OPT_LEVEL", "1");
// std::env::set_var("CARGO_CFG_TARGET_FEATURE", "crt-static");
// std::env::set_var("VCPKGRS_TRIPLET", "x64-windows-static-md");
```

## 建议阅读路径

- 先跑官方示例确认 `libusb + rusb` 环境正确.
- 如果需要回看仓库内的早期 USB 验证目录, 可继续看 [test_usb 实验目录说明](../test_usb/README.md).
- 再根据目标设备补充 `VID/PID`, 权限和驱动层面的适配.
- 如果问题落在 USB 抓包或链路排查, 可配合 [Wireshark](../../../tools/wireshark.md) 一起看.
