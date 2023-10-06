## 创建项目

cargo new --lib test-wasm

## 设置库类型

新增文件 test-wasm\.cargo\config.toml, 添加:

[lib]
crate-type = ["cdylib"]

## 设置编译目标

在 Cargo.toml 中

[build]
target = "wasm32-unknown-unknown"

## 编译

wasm-pack build
