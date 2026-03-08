# esp-rs

## 说明

- 本页记录基于 `esp-rs` 生态在 `ESP32-C3` 上跑通最小 Rust 无线示例的流程.
- 当前重点不是完整项目模板, 而是快速验证 `cargo`, `espflash`, 示例工程和功能 feature 是否已经连通.
- 如果只是做工具链安装, 应优先结合 [espup rust](./espup-rust.md) 与 [rust 环境分析](./rust环境分析.md) 一起看.

## 核心结论

- 想快速验证 `ESP32-C3 + Rust` 是否可用, 最省事的方式是先跑官方或社区最小示例.
- 无线相关示例通常依赖 `feature` 组合, 构建前先确认目标芯片, 示例名称和使能项一致.
- 烧录问题优先检查串口, 芯片识别, 波特率和目标二进制是否匹配当前板子.

## 最小验证流程

### 安装烧录工具

```sh
cargo install cargo-espflash
```

### 获取示例仓库

```sh
git clone https://github.com/esp-rs/esp-wifi.git
cd esp-wifi/examples-esp32c3
```

### 构建示例

```sh
cargo build --features "wifi,embedded-svc"
```

### 烧录并运行

```sh
cargo espflash --features "wifi,embedded-svc" --example dhcp --speed 2000000
```

## 使用场景

- 验证 `ESP32-C3` 的 Rust 工具链是否已经可用.
- 快速确认 `esp-wifi` 示例在当前环境中能否完成构建和烧录.
- 作为后续接入 `Wi-Fi`, `DHCP`, `embedded-svc` 的最小起点.

## 常见排查方向

- 构建报错时, 先检查 Rust 目标平台, `espup` 版本和示例仓库分支是否匹配.
- 如果是 feature 相关报错, 优先确认当前示例是否真的支持 `wifi,embedded-svc` 这一组合.
- 烧录失败时, 先确认开发板端口, 驱动和下载模式是否正确.
- 运行后没有预期日志时, 再去检查串口监视器和日志配置.

## 相关文档

- [ESP32 总览](./README.md)
- [espup rust](./espup-rust.md)
- [rust 环境分析](./rust环境分析.md)
- [nanoESP32-C3](./nanoESP32-C3.md)
