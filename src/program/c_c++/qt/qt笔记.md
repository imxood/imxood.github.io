# Qt 学习笔记

## 说明

- 本页整理 `Qt` 在 Linux 场景下的安装与基础环境配置记录.
- 当前重点是 `Ubuntu` 下安装 `Qt Creator`, 从源码编译 `Qt 5.15`, 以及中文输入问题的处理.

## Ubuntu 安装 Qt

```sh
sudo apt-get install build-essential g++
sudo apt-get install libgl1-mesa-dev libglu1-mesa-dev freeglut3-dev
```

Qt Creator 下载地址:

- <http://download.qt.io/archive/qt/5.14/5.14.0>

设置环境变量:

```sh
export QTDIR=$HOME/programs/Qt5.14.0/5.14.0/gcc_64
export PATH=$QTDIR/bin:$PATH
```

### 无法输入中文

```sh
cp /usr/lib/x86_64-linux-gnu/qt5/plugins/platforminputcontexts ~/programs/Qt5.14.0/Tools/QtCreator/lib/Qt/plugins/platforminputcontexts/
```

## Ubuntu 源码编译 Qt 5.15

下载源码:

- <http://download.qt.io/archive/qt/5.15/5.15.0/single/qt-everywhere-src-5.15.0.tar.xz>

准备依赖:

```sh
sudo apt install clang llvm libfontconfig1-dev libfreetype6-dev libx11-dev libx11-xcb-dev libxext-dev libxfixes-dev libxi-dev libxrender-dev libxcb1-dev libxcb-glx0-dev libxcb-keysyms1-dev libxcb-image0-dev libxcb-shm0-dev libxcb-icccm4-dev libxcb-sync0-dev libxcb-xfixes0-dev libxcb-shape0-dev libxcb-randr0-dev libxcb-render-util0-dev libxcb-xinerama0-dev libxkbcommon-dev libxkbcommon-x11-dev
```

配置与编译:

```sh
./configure -opensource -confirm-license -skip qtlocation -skip qtvirtualkeyboard
make -j10
sudo make install
```

可选裁剪记录:

```text
-skip qtlocation -skip qtwayland -skip qtscript
```

## 使用建议

- 如果只是做常规桌面开发, 优先评估系统包或官方安装器, 通常会比手工源码编译更省心.
- 真正需要源码编译时, 先明确是否要裁剪模块, 再一次性准备好依赖.
- Qt Creator 中文输入异常时, 优先检查输入法插件目录和系统输入法环境.
- 若后续继续整理, 可补 `qmake` / `cmake`, 信号槽, UI Designer 和部署打包相关笔记.
