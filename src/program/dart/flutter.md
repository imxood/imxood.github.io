# Flutter 学习笔记

## 说明

- 本页记录 Flutter 环境初始化, 桌面开发, Android 开发和常见代理配置.
- 内容横跨 Windows 与 Linux 场景, 更适合作为综合备忘录.

## 基础环境

国内镜像环境变量示例:

```sh
export PUB_HOSTED_URL=https://pub.flutter-io.cn
export FLUTTER_STORAGE_BASE_URL=https://storage.flutter-io.cn
```

安装说明:

- https://flutter.cn/docs/get-started/install

常用初始化命令:

```sh
flutter config --no-analytics
flutter --disable-telemetry
dart --disable-analytics
flutter doctor
```

## 桌面开发

参考:

- [Flutter 官方桌面教程](https://flutter.cn/desktop)

国内镜像可切换为:

```sh
export PUB_HOSTED_URL=https://mirrors.tuna.tsinghua.edu.cn/dart-pub
export FLUTTER_STORAGE_BASE_URL=https://mirrors.tuna.tsinghua.edu.cn/flutter
```

启用桌面能力:

```sh
flutter upgrade
flutter config --enable-<platform>-desktop
```

常见平台选项:

- `flutter config --enable-linux-desktop`
- `flutter config --enable-macos-desktop`
- `flutter config --enable-windows-desktop`
- `flutter config --enable-web`
- `flutter config --enable-android`

Linux 桌面开发常见依赖:

```sh
sudo apt-get install clang cmake ninja-build pkg-config libgtk-3-dev libblkid-dev liblzma-dev
```

创建与运行应用:

```sh
flutter create flutter_desktop
cd flutter_desktop
flutter run -d windows
flutter run -d macos
flutter run -d linux
```

构建 release:

```sh
flutter build windows
flutter build macos
flutter build linux
```

性能调试:

```sh
flutter run -d linux --trace-skia --profile
flutter screenshot -d linux --type=skia --observatory-uri=<Observatory-URI>
```

## Android 环境

Android Studio 下载:

- https://developer.android.com/studio

常见环境变量:

```text
D:\programs\Android\Sdk\platform-tools
ANDROID_SDK_ROOT=D:\programs\Android\Sdk
ANDROID_NDK_ROOT=%ANDROID_SDK_ROOT%\ndk\26.3.11579264
```

初始化 Android 开发环境:

```sh
flutter doctor --android-licenses
flutter create start_flutter_android
flutter build apk
flutter build apk --debug
```

## 网络不好的话配置代理

在 `gradle.properties` 中追加:

```conf
systemProp.socks.proxyHost=127.0.0.1
systemProp.socks.proxyPort=1080
systemProp.http.proxyHost=127.0.0.1
systemProp.http.proxyPort=1080
systemProp.https.proxyHost=127.0.0.1
systemProp.https.proxyPort=1080
```

## Gradle 下载失败

参考:

- https://github.com/whichow/Notebook/blob/master/Development/Android/Build/%E8%A7%A3%E5%86%B3Gradle%E4%B8%8B%E8%BD%BD%E8%B6%85%E6%97%B6%E9%97%AE%E9%A2%98.md

可在 `gradle/wrapper/gradle-wrapper.properties` 中把远程 `distributionUrl` 改成本地文件路径, 例如:

```text
distributionUrl=file:///D:/programs/gradle/gradle-7.6.3-all.zip
```

然后执行:

```sh
./gradlew build
```

## 其他

- iOS 构建: `flutter build ios`
- Flutter 升级: `flutter upgrade`
