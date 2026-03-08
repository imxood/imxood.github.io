# wasm-bindgen CLI

## 说明

- `wasm-bindgen` 常用于把 Rust 生成的 `.wasm` 模块包装成更方便给 Web 侧调用的 JavaScript 接口.
- 若项目已使用 `trunk` 或 `wasm-pack`, 也可以根据工具链选择更适合的工作流.
- 本页更偏 `wasm-bindgen CLI` 的最小使用与目标平台区分.

## 安装

```sh
cargo install -f wasm-bindgen-cli
```

## 基本用法

```sh
wasm-bindgen [options] ./target/wasm32-unknown-unknown/release/crate.wasm
```

## 常见输出方式

```sh
wasm-bindgen --target web --out-dir ./pkg ./target/wasm32-unknown-unknown/release/app.wasm
wasm-bindgen --target bundler --out-dir ./pkg ./target/wasm32-unknown-unknown/release/app.wasm
wasm-bindgen --target nodejs --out-dir ./pkg ./target/wasm32-unknown-unknown/release/app.wasm
```

- `web`: 直接面向浏览器原生模块加载场景.
- `bundler`: 适合配合 `Vite`, `Webpack` 等前端构建工具.
- `nodejs`: 适合命令行或服务端脚本环境.

## 典型流程

1. 先用 `cargo build --target wasm32-unknown-unknown --release` 生成原始 `.wasm`.
2. 再用 `wasm-bindgen` 生成 JS 胶水代码和 TypeScript 声明.
3. 最后由前端项目或运行时环境加载输出目录中的文件.

## 适合场景

- 已经有现成 Rust crate, 想自己控制输出目录和打包方式.
- 需要明确区分浏览器, bundler 和 Node.js 三种目标环境.
- 想把生成的产物接入已有前端工程, 而不是重新搭完整模板.

## 使用建议

- 若项目只是常规 WebAssembly 前端应用, 优先评估 `trunk` 或 `wasm-pack`, 通常会更省心.
- 若需要精细控制输出目录, 模块目标和集成方式, `wasm-bindgen CLI` 会更灵活.
- 浏览器侧问题排查时, 先确认目标平台, 生成产物路径和静态资源服务方式是否一致.
- 若后续继续整理, 可补 `wasm-pack`, `trunk`, `worker` 场景和 TS 类型生成实践.
