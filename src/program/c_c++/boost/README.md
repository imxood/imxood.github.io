# Boost 总览

## 说明

- 本目录保存 `Boost` 相关的环境搭建记录与示例代码.
- 当前内容更偏 Windows + `CMake` + `MinGW-w64` 工具链场景, 适合作为历史配置与示例归档入口.
- 若问题只是普通 `C++` 工具链或构建链, 应优先回到 [C 与 C++ 总览](../README.md).

## 当前内容

- [Windows 10 下 CMake + Boost + MinGW-w64 环境搭建](./win10下cmake%20boost环境搭建.md)
- `examples/` 目录中的最小示例代码, 覆盖 `program_options`, `timer`, `asio`, `websocket` 等主题.
- `CMakeLists.txt` 用于组织目录内的示例编译.

## 阅读路径

1. 先看环境搭建文档, 明确 `MinGW-w64`, `cmake`, `ninja`, `Boost` 版本约束.
2. 再看 `examples/` 中的最小示例, 理解 `Boost` 各模块的基础用法.
3. 若问题已经回到通用构建链, 再去看 [CMake 总览](../../../tools/cmake/README.md) 或其他工具链文档.

## 适用场景

- 在 Windows 上用 `MinGW-w64` 构建 `Boost`.
- 回看 `Boost.Asio`, `Boost.Program_options` 等历史示例写法.
- 排查旧工程中的 `find_package(Boost)` 版本适配问题.

## 使用建议

- 本目录保留的是历史配置与示例上下文, 不建议直接把旧版本组合当成当前默认方案.
- 若其中某个 `Boost` 模块的知识点已稳定, 建议后续单独提炼成正式专题页.
- 示例目录若继续扩张, 可再细分为“构建安装”和“模块示例”两类入口.
