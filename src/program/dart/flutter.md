# Flutter 学习笔记

    export PUB_HOSTED_URL=https://pub.flutter-io.cn
    export FLUTTER_STORAGE_BASE_URL=https://storage.flutter-io.cn

    安装环境

        https://flutter.cn/docs/get-started/install

        解压后, 添加bin目录到环境变量, 就有了 dart 和 flutter 命令了

    flutter config --no-analytics

    flutter --disable-telemetry

     dart --disable-analytics

    flutter doctor

## 桌面开发

参考[`flutter 官方教程`](https://flutter.cn/desktop)

    设置国内源:
        export PUB_HOSTED_URL=https://mirrors.tuna.tsinghua.edu.cn/dart-pub
        export FLUTTER_STORAGE_BASE_URL=https://mirrors.tuna.tsinghua.edu.cn/flutter

    <!-- flutter channel dev -->
    flutter upgrade

    flutter config --enable-<platform>-desktop

    // flutter config --enable-linux-desktop
    // flutter config --enable-macos-desktop
    // flutter config --enable-windows-desktop
    // flutter config --enable-web
    // flutter config --enable-android
    // flutter create .

    linux 桌面开发 配置:
        sudo apt-get install clang cmake ninja-build pkg-config libgtk-3-dev libblkid-dev liblzma-dev


    flutter devices
    flutter doctor

    创建一个应用
        flutter create flutter_desktop
        cd flutter_desktop

    运行:
        flutter run -d windows
        flutter run -d macos
        flutter run -d linux

    编译 release app:
        flutter build windows
        flutter build macos
        flutter build linux

    性能调试:
        flutter run -d linux --trace-skia --profile

        flutter screenshot -d linux --type=skia --observatory-uri=<Observatory-URI>

        打开https://debugger.skia.org/ (被墙了), 上传 上一个命令 到这个网站, 分析

## 安卓环境

### 下载 并安装 android studio

https://developer.android.com/studio

### 添加到 PATH 环境变量

安装 SDK / NDK / Command line tools

D:\programs\Android\Sdk\platform-tools
<!-- D:\programs\Android\Sdk\tools\bin -->

ANDROID_SDK_ROOT=D:\programs\Android\Sdk
ANDROID_NDK_ROOT=%ANDROID_SDK_ROOT%\ndk\26.3.11579264

### 设置 flutter for android 环境

flutter doctor --android-licenses

### 创建项目

flutter create start_flutter_android

### 编译项目

flutter build apk

flutter build apk --debug

### 网络不好的话, 需要配置代理

编辑 gradle.properties 文件, 追加内容:

```conf
systemProp.socks.proxyHost=127.0.0.1
systemProp.socks.proxyPort=1080

systemProp.http.proxyHost=127.0.0.1
systemProp.http.proxyPort=1080

systemProp.https.proxyHost=127.0.0.1
systemProp.https.proxyPort=1080
```

### gradle 下载失败

https://github.com/whichow/Notebook/blob/master/Development/Android/Build/%E8%A7%A3%E5%86%B3Gradle%E4%B8%8B%E8%BD%BD%E8%B6%85%E6%97%B6%E9%97%AE%E9%A2%98.md

gradle/wrapper/gradle-wrapper.properties

找到: `distributionUrl=https\://services.gradle.org/distributions/gradle-2.4-all.zip`

下载到本地, 并设置成本地文件: `distributionUrl=file:///D:/programs/gradle/gradle-7.6.3-all.zip`

```sh
./gradlew build
```

## IOS

flutter build ios

## flutter 升级

flutter upgrade
