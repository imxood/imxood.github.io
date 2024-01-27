# rust 源码编译

## 准备

### 使用 choco 下载依赖

使用管理员权限:

Set-ExecutionPolicy Bypass -Scope Process -Force; iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))

choco -v

### 使用 choco 安装必要软件

choco install -y cmake llvm python3 git

添加 cmake 到 系统变量 PATH: C:\Program Files\CMake\bin

## ninja

python -m pip install ninja

添加 ninja 路径 到环境变量: C:\Python312\Lib\site-packages\ninja\data\bin

## rust 源码

git clone https://github.com/rust-lang/rust.git

cd rust

./x setup

    选择 "e dist", 并选择 忽略 "Ci", 将生成 config.toml 文件

追加以下信息 到 config.toml

```conf
[llvm]
# don't download and build llvm
download-ci-llvm = false

[build]
compiler-docs = false

cargo-native-static = true

[rust]

debug = true
debuginfo-level = 2

debug-logging = true

incremental = true
```

为了加快编译, 添加配置到 Cargo.toml

```
[profile.release.package.rustc_mir_build]
opt-level = 0

[profile.release.package.rustc_driver]
opt-level = 0
```

./x build

    使用 stage0 compiler 编译 std

    使用 stage0 compiler 编译 rustc, 生成 stage1 compiler

    使用 stage1 compiler 编译 std

    rustup toolchain link stage1 build/host/stage1, 创建工具链

    rustup override set stage1, 指定 项目 使用的 toolchain, 可以直接: cargo build

    rustup default stage1, 指定 全局的 toolchain

    cargo +stage1 build, 临时使用 stage1

./x build --keep-stage 1, 再次编译时, 不添加 --keep-stage 时, 默认会清除 stage1 和 stage2, 但修改了 std库 应该不能有这个选项.

./x build proc-macro-srv-cli

./x build --stage 2 proc-macro-srv-cli

    rust-analyzer and IntelliJ Rust plugin 使用一个叫 rust-analyzer-proc-macro-srv 的组件 处理 过程宏.

./x build --stage 2 compiler/rustc

    使用 stage1 compiler 编译 rustc, 生成 stage2 compiler

    使用 stage2 compiler 编译 std

./x doc

./x doc --stage 1

    编译文档

## 项目配置 (使用 vscode)

下面 第 2 3 步 是为了配置 rust-analyzer, 不配置的话, vscode中 代码提示 宏展开 会出错.

1. rustup override set stage2, 指定项目默认的 toolchain (非全局), 会设置在 ${ENV:USER}\.rustup\settings.toml

2. 设置 rust-analyzer

修改配置 .vscode\settings.json, 添加两行 (确保路径存在):

    "rust-analyzer.procMacro.server": "D:/develop/rust/rust/build/x86_64-pc-windows-msvc/stage2-tools-bin/rust-analyzer-proc-macro-srv.exe",
    "rust-analyzer.procMacro.enable": true

3. 添加下列路径 到 PATH 环境变量 (rust-analyzer-proc-macro-srv.exe 依赖的动态库路径):

D:\work\rust_projects\rust\build\x86_64-pc-windows-msvc\stage1-std\x86_64-pc-windows-msvc\release\deps

分析的过程如下:

    rust-analyzer-proc-macro-srv.exe 打开闪退.

    使用 visual studio 的命令行工具, 如: 在 "开始" 中 搜索 "Powershell", 出现的 " Developer Powershell for vs 2022"

        dumpbin /dependents D:/develop/rust/rust/build/x86_64-pc-windows-msvc/stage2-tools-bin/rust-analyzer-proc-macro-srv.exe

    在输出的内容中, 有如下:

    ```
    Image has the following dependencies:

        kernel32.dll
        rustc_driver-425c7e0f724b33ed.dll
        std-b1b46c42ec629488.dll
        KERNEL32.dll
    ```

    经过搜索 发现, 这两个路径位于: D:\work\rust_projects\rust\build\x86_64-pc-windows-msvc\stage1-std\x86_64-pc-windows-msvc\release\deps

## 在 rustc 项目中 配置 vscode rust-analyzer

./x setup vscode

## 快速检测错误

./x check src/tools/rustdoc

## 给出建议

./x suggest

## 快速的编译

./x build library

./x build library --keep-stage 1

## 调试技巧

### 调试 类型布局

```rs
#![feature(rustc_attrs)]

#[rustc_layout(debug)]
type T<'a> = &'a u32;
```
