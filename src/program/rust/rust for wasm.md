# Rust for WebAssembly

## 说明

- 本页整理 Rust 编译到 WebAssembly 的最小流程与工具选型.
- 适合刚开始做浏览器端, 前端集成或轻量跨平台模块时快速建立正确工程骨架.
- 若你已经拿到了 `.wasm` 文件并只想做 JS 胶水层处理, 可继续看 [wasm-bindgen CLI](./wasm.md).

## 适用场景

- 把 Rust 逻辑编译成浏览器可加载的 `WebAssembly` 模块.
- 在前端工程中复用 Rust 的解析, 算法或性能敏感逻辑.
- 使用 `Trunk` 或 `wasm-pack` 快速搭建 Rust + Web 的开发流程.

## 最小工程准备

### 1. 创建库项目

```sh
cargo new --lib test-wasm
cd test-wasm
```

- WebAssembly 模块通常以库项目形式输出, 便于给 JS 或宿主环境调用.

### 2. 在 `Cargo.toml` 中声明库类型

```toml
[lib]
crate-type = ["cdylib"]
```

- `cdylib` 适合输出给外部宿主使用的动态库产物, 也是生成 `.wasm` 的常见配置.
- 这一项应放在 `Cargo.toml`, 不是 `.cargo/config.toml`.

### 3. 安装目标平台

```sh
rustup target add wasm32-unknown-unknown
```

- `wasm32-unknown-unknown` 是最常见的通用 WebAssembly 目标.

## 常见构建方式

### 方式 1: 使用 `wasm-pack`

```sh
cargo install wasm-pack
wasm-pack build --target web
```

- 适合快速生成浏览器可用的包结构.
- `--target web` 适合直接在浏览器中通过 ES Module 加载.

### 方式 2: 手动 `cargo build` + `wasm-bindgen`

```sh
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --target web --out-dir ./pkg ./target/wasm32-unknown-unknown/release/test_wasm.wasm
```

- 适合需要精细控制输出目录, 模块目标和生成结果时使用.
- 这条链路和 [wasm-bindgen CLI](./wasm.md) 是直接配套关系.

### 方式 3: 使用 `Trunk`

```sh
cargo install trunk
trunk serve
```

- 适合项目中已经有 `index.html` 的前端应用型工程.
- `Trunk` 会负责静态资源, `.wasm` 构建和本地开发服务器, 更适合做页面应用.

## 推荐目录与配置理解

### `Cargo.toml`

- 放和包本身相关的配置, 例如 `[package]`, `[dependencies]`, `[lib]`.
- `crate-type = ["cdylib"]` 这类配置应写在这里.

### `.cargo/config.toml`

- 放 Cargo 行为配置, 例如默认 target, linker, alias.
- 如果你希望默认构建到 WebAssembly, 可以这样写:

```toml
[build]
target = "wasm32-unknown-unknown"
```

- 这一项不应写到 `Cargo.toml` 的 `[build]` 中.

## 工具怎么选

### 选 `wasm-pack`

- 希望快速出一个前端可消费的包.
- 希望少记命令, 先把最小闭环跑通.

### 选 `wasm-bindgen CLI`

- 已经有自己的构建流程.
- 需要明确控制 `web`, `bundler`, `nodejs` 等输出目标.

### 选 `Trunk`

- 工程本身就是 Rust 驱动的 Web 应用.
- 项目根目录已有 `index.html`, 想要更接近前端 dev server 体验.

## 常见注意点

- 不是所有 Rust crate 都能直接编译到 WebAssembly, 尤其是依赖线程, 文件系统或原生系统 API 的库.
- 如果项目要被浏览器调用, 通常还需要引入 `wasm-bindgen` 导出函数接口.
- 调试构建问题时, 先确认目标平台, `crate-type`, 工具链命令和输出路径是否一致.

## 相关文档

- [wasm-bindgen CLI](./wasm.md)
- [Rust 问题汇总](./rust问题汇总.md)
- [Rust 总览](./rust.md)
