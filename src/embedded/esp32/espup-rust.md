# espup Rust 环境

## 说明

- 本页记录使用 `espup` 搭建 ESP Rust 工具链和生成 `esp-idf-template` 示例工程的过程.
- 内容主要面向 `ESP32-C3` / `ESP32-S3` 方向, 其中版本约束较强, 需要结合当前 `ESP-IDF` 版本重新核对.

- 本页中的 master 指向经核对仍存在的 ESP-IDF 上游分支, 不是默认分支命名建议.

## espup

https://github.com/esp-rs/espup

cargo install espup --git https://github.com/esp-rs/espup

espup install -t esp32c3 -e master

当前记录可用的版本是 v4.4. 如果上游版本策略变化, 请以官方文档和当前模板要求为准.

## hello world

    cargo generate --git https://github.com/esp-rs/esp-idf-template cargo

        参数:
            project name: esp32s3-hello
            ESP-IDF native build version: v4.4
            MCU: esp32s3
            Configure project to use Dev Containers: false
            STD support: true

    cd esp32s3-hello
    cargo build --release

    espflash COM24 .\target\xtensa-esp32s3-espidf\release\esp32s3-hello --speed 2000000 --monitor
