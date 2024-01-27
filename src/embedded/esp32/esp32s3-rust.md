1. 安装 esp32 的 toolchain

    https://dl.espressif.cn/dl/esp-idf/

    在 windows 上, 可以直接使用 离线安装的方式安装, 避免各种麻烦的事情发生.

2. 安装 rust target for esp32s3

    根据 https://github.com/esp-rs/rust-build 的说明.

    git clone https://github.com/esp-rs/rust-build.git
    cd rust-build
    ./Install-RustToolchain.ps1
    ./Export-EspRust.ps1

3. 安装工具

    cargo install cargo-generate ldproxy espflash espmonitor

4. 生成项目

    cargo generate --vcs none --git https://github.com/esp-rs/esp-idf-template cargo

    参数:
    ESP-IDF native build version: v4.4
    MCU: esp32s3
    Configure project to use Dev Containers: false
    STD support: true

5. 编译

    一定要记得, 前两步中, 都会有设置环境变量. 否则找不到 rust 环境 或者 esp toolchain.

    cd 生成的项目路径

    cargo build --release

6. 烧写

    espflash COM24 .\target\xtensa-esp32s3-espidf\release\esp32s3-demo --speed 921600
    espmonitor COM24

## 2023.11.09

1. 用 esp 的 windows 安装器 在线安装的方式, 安装了 master 版本的 idf (目前是 ESP-IDF 5.3 版本)

2. 使用 espup 安装 rust 工具链

```sh
cargo install espup

# 这一步会在 USER 的PATH环境变量下 增加几十个路径, 极大可能会影响其他依赖 clang 的程序运行, 我的做法是 直接删掉 这些新增的与esp相关的路径. 不要删错了!!!
espup install esp32c3
```

3. 打开安装 idf 后产生快捷方式 ESP-IDF 5.3 Powershell

在新终端中, 执行 `~\export-esp.ps1`

4. (可选: 设置代理 加快下载模版), 创建模版项目

```sh
cargo generate esp-rs/esp-idf-template cargo
```

5. 编译

```sh
cd 进入项目路径

cmake build -vvv (由于环境变量中已经包含 idf 的路径, 所以不需要 下载 idf 到 .embuild 目录了)

cmake run
```
