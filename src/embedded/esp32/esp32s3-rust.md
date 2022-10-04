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

    一定要记得, 前两步中, 都会有设置环境变量. 否则找不到 rust环境 或者 esp toolchain.

    cd 生成的项目路径

    cargo build --release

6. 烧写

    espflash COM24 .\target\xtensa-esp32s3-espidf\release\esp32s3-demo --speed 921600
    espmonitor COM24