# Rust 问题汇总

## 说明

- 本页用于收集 Rust 开发中的高频问题与排查线索.
- 当前仓库中的问题与技巧仍主要分散在专题页中, 本页先作为统一入口保留.

## 常见方向

- 环境和工具链问题: [Rust 工具](./rust工具.md), [Rust 命令速记](./rust命令.md)
- 调试问题: [Rust 调试技巧](./rust调试技巧.md)
- FFI 与绑定问题: [bindgen](./bindgen.md), [windows-rs](./windows-rs.md), [libc 与 C 运行时互操作](./libc.md)
- WebAssembly 问题: [Rust for WebAssembly](./rust%20for%20wasm.md), [wasm-bindgen CLI](./wasm.md)
- GUI 与应用问题: [Dioxus](./dioxus.md), [Tauri](./tauri.md), [Makepad](./makepad.md)

## 建议排查顺序

1. 先判断问题属于环境, 编译, 运行时还是架构设计层面.
2. 再回到对应专题页, 不要把所有问题都堆在一个总表里排查.
3. 若问题涉及跨语言, 多平台或工具链组合, 优先缩小最小复现范围.
4. 若问题和版本强相关, 记得记录 Rust 版本, crate 版本和目标平台.

## 相关文档

- [Rust 总览](./rust.md)
- [Rust 要点](./rust要点.md)
- [Rust 工具](./rust工具.md)
- [Rust 调试技巧](./rust调试技巧.md)
