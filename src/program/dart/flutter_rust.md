# Flutter + Rust 环境

## 说明

- 本页用于整理 `Flutter` 与 `Rust` 混合工程的环境准备和最小集成思路.
- 更完整的桥接流程可参考 [flutter_rust_bridge](./flutter_rust_bridge.md).

## 安装准备

- 安装 Rust 工具链.
- 安装 Dart / Flutter 环境.
- 安装 `protobuf` 等桥接所需工具.
- 根据目标平台补齐 Android SDK, Windows 构建工具或其他平台依赖.

## 最小集成思路

1. 先分别确认 `flutter doctor` 和 `cargo --version` 都能正常工作.
2. 在 Flutter 工程外或工程内建立 Rust 库 crate.
3. 通过桥接工具生成 Dart 侧绑定代码.
4. 在 Flutter 侧调用生成的接口, 再处理打包和运行库分发问题.

## 常见关注点

- 平台架构要一致, 尤其是 `x64`, `arm64`, Android ABI 等目标不能混用.
- 原生库输出路径要稳定, 否则 Flutter 侧很容易在运行时找不到动态库.
- 若使用代码生成桥接, 要把生成命令纳入构建流程, 避免接口变更后两侧不同步.

## 建议阅读顺序

- 先看本页完成环境准备.
- 再看 [flutter_rust_bridge](./flutter_rust_bridge.md) 了解桥接工具链.
- 若遇到 Flutter 构建问题, 再回看 [Flutter 引擎编译](./flutter编译.md) 和 [Flutter 学习笔记](./flutter.md).
