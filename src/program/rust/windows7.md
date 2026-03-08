# Rust for Windows 7

## 说明

- 本页记录 Rust 项目在 Windows 7 环境下可用的工具链版本与兼容处理方式.
- 重点包括固定 `rustup` 版本, 使用 `thunk` 适配旧系统 API, 以及用 `YY-Thunks` 分析可执行文件兼容性.

## windows7 中支持的 target

rustup install 1.77.2

指定项目的默认目标 `rustup override set 1.77.2-x86_64-pc-windows-msvc`

或者, 创建文件 `rust-toolchain.toml`, 写入:

```toml
[toolchain]
channel = "1.77.2-x86_64-pc-windows-msvc"
```

## 使用 thunk 的方式

https://github.com/felixmaker/thunk

cargo install thunk-cli

### 安装依赖的环境

(1) 解压 https://github.com/Chuyu-Team/VC-LTL5/releases/download/v5.2.2/VC-LTL-Binary.7z

设置环境变量 VC_LTL 到解压后的目录

(2) 解压 https://github.com/Chuyu-Team/YY-Thunks/releases/download/v1.1.8/YY-Thunks-Objs.zip

设置环境变量 YY_THUNKS 到解压后的目录, 目录下有 objs 目录

### 编译

thunk --help, 可以看到 可以支持 xp / win7

thunk --os win7 --arch x64 -- --release

### windows 可执行程序, windows api 分析

https://github.com/Chuyu-Team/YY-Thunks/releases

YY.Depends.Analyzer.exe 可以分析 编译出的可执行程序 有哪些 windows api, 及 支持的 windows api 的系统型号

使用: .\YY.Depends.Analyzer.exe "D:\Temp\virtualbox_share\learn_image_viewer_win7_64\learn_image_viewer.exe"

根据输出的提示, 输出的分析文件为: learn_image_viewer.exe.md

可以看到 windows api 下的接口, 是否支持目标平台. 如果不满足的话, 且 "YY-Thunks Ready" 为 "No", 那么 应该无法运行
