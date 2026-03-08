# egui for Android

## 说明

- 记录基于 `android-activity` 示例在 Android 上运行 `egui` 的最小流程.
- 适合验证 `cargo-ndk` / `cargo-apk` 与 Gradle 的打包链路是否已经连通.
- 如果只是做 Android 基础环境准备, 应优先先看 [基本环境设置](./基本环境设置.md).

## 适用场景

- 快速验证 Rust GUI 在 Android 上能否完成编译和打包.
- 对照 `cargo-ndk` 和 `cargo-apk` 两套构建方式的差异.
- 在代理环境下排查 Gradle 依赖下载问题.

## 前置环境

1. 下载 Android Studio, 并用它安装 SDK / NDK.
2. 设置环境变量:

```sh
ANDROID_SDK_ROOT=D:\programs\Android\Sdk
ANDROID_NDK_ROOT=%ANDROID_SDK_ROOT%
dk.1.7779620
```

3. 安装编译工具:

```sh
rustup target add aarch64-linux-android
cargo install cargo-ndk
```

## 下载示例

```sh
git clone https://github.com/rib/android-activity.git
cd examples/agdk-egui
```

## 编译方式

使用 `cargo-ndk` + Gradle:

```sh
cargo ndk -t arm64-v8a -o app/src/main/jniLibs/ build
./gradlew build
```

使用 `cargo-apk`:

```sh
cargo apk build
cargo apk run
```

## 代理配置

如果网络较差, 可在 `gradle.properties` 中追加:

```conf
systemProp.socks.proxyHost=127.0.0.1
systemProp.socks.proxyPort=1080
systemProp.http.proxyHost=127.0.0.1
systemProp.http.proxyPort=1080
systemProp.https.proxyHost=127.0.0.1
systemProp.https.proxyPort=1080
```

重新构建:

```sh
./gradlew build
```

## 常见关注点

- 打包失败时, 先检查 Android SDK / NDK 路径和目标架构是否一致.
- Gradle 拉依赖失败时, 再考虑代理, 仓库源和证书环境问题.
- 如果只是想验证链路是否通, 优先先跑最小示例, 不要一开始叠加复杂业务代码.

## 相关文档

- [Rust for Android 总览](./README.md)
- [基本环境设置](./基本环境设置.md)
- [Tauri for Android](./tauri.md)
