# Dioxus

## 说明

- `Dioxus` 是 Rust 生态中的 UI / 全栈应用框架, 可覆盖 Web, Desktop, Mobile 等多平台.
- 本页记录安装, 项目初始化和 Android 环境准备的最小入口.

## 适用场景

- 使用 Rust 构建前端界面或跨平台客户端.
- 做 `SSR`, 全栈表单, 路由和组件化 UI 实验.
- 希望在 Rust 生态内统一管理界面逻辑和业务代码.

## 安装

```sh
rustup target add wasm32-unknown-unknown
cargo install dioxus-cli@0.7.0-alpha.3
```

## 创建项目

```sh
dx new test_dioxus
```

初始化时可按需选择:

- 全栈
- 路由
- tailwind

## 常见开发路径

- 纯 Web 场景: 优先从 `wasm` 目标和前端路由开始.
- 桌面应用场景: 先验证最小窗口与热重载流程.
- 全栈场景: 明确前后端边界, 再决定是否启用 SSR 和服务端逻辑.

## Android 环境

```sh
# 参考: https://dioxuslabs.com/learn/0.7/guides/platforms/mobile#android
export JAVA_HOME="D:\programs\Android\Android Studio\jbr\bin"
export ANDROID_HOME="D:\programs\Android\Sdk"
export NDK_HOME="$ANDROID_HOME/ndk/29.0.14206865"
export PATH="$PATH:$ANDROID_HOME/emulator:$ANDROID_HOME/platform-tools"

emulator -list-avds
```

## 使用建议

- 先明确目标平台, 不同平台的构建链路和依赖差异很大.
- 若只是移动端打包实验, 可对照 `Rust for Android` 一起看.
- 若是桌面壳 + Web 技术栈方案, 也可以同时对比 `Tauri` 的取舍.

## 相关文档

- [Tauri](./tauri.md)
- [Makepad](./makepad.md)
- [Rust for Android 总览](./rust%20for%20android/README.md)
