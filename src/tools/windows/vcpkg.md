# vcpkg

1. 设置 vcpkg

git clone https://github.com/microsoft/vcpkg

cd vcpkg

.\bootstrap-vcpkg.bat -disableMetrics

2. 设置项目

添加 当前路径 到环境变量中.

.\vcpkg integrate install

.\vcpkg search [search term]

vcpkg list 查看已经安装的库

vcpkg help triplet

.\vcpkg\vcpkg install libusb --triplet=x64-windows-static-md

## 安装 ffmpeg

vcpkg install ffmpeg:x64-windows

## rust opengl-sys

vcpkg install openssl:x64-windows-static-md --debug

--debug 参数, 用于查看编译过程

