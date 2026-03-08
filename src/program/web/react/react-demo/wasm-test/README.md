# wasm-test 实验说明

## 说明

- 本目录是 `Vite + React` 示例工程下的 Rust / WASM 联调子工程.
- 当前目录更适合作为实验归档入口, 不承担正式知识导航职责.
- 若需要系统化了解 Rust / WebAssembly, 应优先查看 [Rust WASM](../../../../rust/wasm.md).

## 当前文件

- `Cargo.toml`: Rust `cdylib` 工程配置, 依赖 `wasm-bindgen`.
- `src/lib.rs`: 最小 Rust 导出逻辑.

## 适用场景

- 回看 `React` 工程中嵌入 Rust / WASM 子项目的目录组织方式.
- 理解早期 `wasm-bindgen` 最小工程结构.
- 对照旧项目判断前端与 Rust 子工程之间的边界.

## 使用建议

- 如果只是查前端或 React 知识点, 请回到 [Vite React 示例项目](../README.md) 或正式前端文档.
- 如果只是查 Rust / WASM 知识点, 请优先回到 [Rust WASM](../../../../rust/wasm.md).
- 若后续继续扩写, 建议把稳定结论回流到正式知识页, 本目录只保留实验上下文.
