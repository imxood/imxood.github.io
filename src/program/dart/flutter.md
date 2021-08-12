# Flutter 学习笔记

    export PUB_HOSTED_URL=https://pub.flutter-io.cn
    export FLUTTER_STORAGE_BASE_URL=https://storage.flutter-io.cn

    安装SDK
        https://flutter.dev/docs/get-started/install

        解压后, 添加bin目录到环境变量, 就有了 dart 和 flutter 命令了


## 桌面开发

参考[`flutter 官方教程`](https://flutter.cn/desktop)

    设置国内源:
        export PUB_HOSTED_URL=https://mirrors.tuna.tsinghua.edu.cn/dart-pub
        export FLUTTER_STORAGE_BASE_URL=https://mirrors.tuna.tsinghua.edu.cn/flutter

    flutter channel dev
    flutter upgrade

    flutter config --enable-<platform>-desktop

    // flutter config --enable-linux-desktop
    // flutter config --enable-macos-desktop
    // flutter config --enable-windows-desktop
	// flutter config --enable-web
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
