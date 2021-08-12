# qt 学习笔记

## ubuntu 安装qt

    sudo apt-get install build-essential g++
    sudo apt-get install libgl1-mesa-dev libglu1-mesa-dev freeglut3-dev

qt creator 下载地址: http://download.qt.io/archive/qt/5.14/5.14.0

设置环境变量

    export QTDIR=$HOME/programs/Qt5.14.0/5.14.0/gcc_64
    export PATH=$QTDIR/bin:$PATH

无法输入中文

    cp /usr/lib/x86_64-linux-gnu/qt5/plugins/platforminputcontexts ~/programs/Qt5.14.0/Tools/QtCreator/lib/Qt/plugins/platforminputcontexts/


## ubuntu 源码编译qt15

    下载源码:
        http://download.qt.io/archive/qt/5.15/5.15.0/single/qt-everywhere-src-5.15.0.tar.xz


        sudo apt install clang llvm libfontconfig1-dev libfreetype6-dev libx11-dev libx11-xcb-dev libxext-dev libxfixes-dev libxi-dev libxrender-dev libxcb1-dev libxcb-glx0-dev libxcb-keysyms1-dev libxcb-image0-dev libxcb-shm0-dev libxcb-icccm4-dev libxcb-sync0-dev libxcb-xfixes0-dev libxcb-shape0-dev libxcb-randr0-dev libxcb-render-util0-dev libxcb-xinerama0-dev libxkbcommon-dev libxkbcommon-x11-dev

    ./configure -opensource -confirm-license -skip qtlocation -skip qtvirtualkeyboard

    make -j10

    sudo make install


     -skip qtlocation -skip qtwayland -skip qtscript
