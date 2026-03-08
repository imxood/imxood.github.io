# Rust 命令速记

## 说明

- 本页用于积累零散但高频的 Rust 命令.
- 如果命令已经形成完整工具链知识, 可继续回到 [Rust 工具](./rust工具.md).
- 适合在切换工具链, 新目标平台或排查构建问题时做快速速查.

## 工具链

安装 Windows 下的 `nightly GNU` 工具链:

```sh
rustup toolchain install nightly-gnu
```

查看当前工具链:

```sh
rustup show
rustup toolchain list
```

添加目标平台:

```sh
rustup target add wasm32-unknown-unknown
rustup target add aarch64-linux-android
```

## Cargo 常用命令

```sh
cargo check
cargo build
cargo build --release
cargo run
cargo test
cargo clean
```

查看依赖树:

```sh
cargo tree
cargo tree -e features
```

展开宏:

```sh
cargo expand
```

## 常见使用场景

- `cargo check`: 适合快速做语义检查, 比完整构建更快.
- `cargo build --release`: 适合确认发布构建是否能通过.
- `cargo tree`: 很适合排查重复依赖和 feature 组合问题.
- `cargo expand`: 适合排查宏展开后的真实代码结构.

## 建议阅读路径

- 命令太碎时, 先把它们按“工具链”, “构建”, “调试”, “打包”分类.
- 一旦某类命令形成稳定工作流, 优先回收至 [Rust 工具](./rust工具.md) 或相关专题页.
- 目标平台相关命令, 可继续结合 [Rust for Android 总览](./rust%20for%20android/README.md) 或 [Rust WASM](./wasm.md) 一起看.
