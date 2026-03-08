# MXSpice 归档说明

## 说明

- 本目录是一个基于 `Qt Widgets + ngspice + Boost` 的历史实验工程.
- 从 `CMakeLists.txt` 可见, 工程名为 `MXSpice`, 主要依赖 `Qt5::Widgets`, `Boost::system` 和 `ngspice`.
- 当前更适合作为“工程结构归档”, 不建议直接作为新项目模板.

## 目录特征

- `main.cpp`, `mainwindow.*`: Qt 主窗口入口.
- `ngspice.*`: `ngspice` 相关封装.
- `spicereporter.*`, `customerevent.*`: 事件与仿真结果处理相关代码.
- `CMakeLists.txt`: 构建入口.

## 适用场景

- 回看 Qt 与仿真库联动的桌面工程组织方式.
- 对照学习历史 Qt Widgets + CMake 项目的依赖写法.
- 梳理 `ngspice` 集成在 GUI 程序中的最小结构.

## 构建线索

从工程配置可以看到:

- 依赖 `Qt5 Widgets`.
- Unix 环境下通过 `pkg-config` 查找 `ngspice`.
- 使用 `CMAKE_AUTOUIC`, `CMAKE_AUTOMOC`, `CMAKE_AUTORCC`.

## 使用建议

- 若只是查 Qt 基础知识, 优先回到上层 [Qt 总览](../README.md) 与 [Qt 学习笔记](../qt笔记.md).
- 若要真正运行这个工程, 先确认本机 Qt 版本, `ngspice` 头文件与库路径是否可用.
- 这类历史工程更适合抽取设计思路, 不建议直接作为现代项目骨架照搬.
