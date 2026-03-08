# Rust for Android 总览

## 说明

- 本目录收录 Android SDK / NDK 环境搭建, adb 调试, 以及 Rust GUI / 应用框架在 Android 上的实验记录.
- 内容以“跑通最小链路”的历史实测为主, 版本信息偏旧, 使用时请结合当前 Android Studio 与 NDK 版本核对.

## 常用入口

- [基本环境设置](./基本环境设置.md)
- [Tauri for Android](./tauri.md)
- [egui for Android](./egui_ardroid.md)

## 使用建议

- 先完成 Android Studio, SDK, NDK, adb 与设备连接配置.
- 再按技术路线选择 `Tauri` 或 `egui/android-activity` 继续验证.
- 如果只是验证移动端链路, 建议先跑最小示例, 不要一开始叠加复杂业务.
