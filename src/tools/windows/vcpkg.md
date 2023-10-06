git clone https://github.com/microsoft/vcpkg

.\vcpkg\bootstrap-vcpkg.bat -disableMetrics

添加 .\vcpkg 到环境变量中.

.\vcpkg\vcpkg integrate install

.\vcpkg\vcpkg search [search term]

vcpkg list 查看已经安装的库

vcpkg help triplet

.\vcpkg\vcpkg install libusb --triplet=x64-windows-static-md

## 安装 ffmpeg

vcpkg install ffmpeg:x64-windows
