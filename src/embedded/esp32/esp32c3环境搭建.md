# esp32c3 环境搭建

参考: https://docs.espressif.com/projects/esp-idf/zh_CN/latest/esp32c3/get-started/index.html

## 搭建windows环境

参考: https://docs.espressif.com/projects/esp-idf/zh_CN/latest/esp32c3/get-started/windows-setup.html

### 方式1

    下载 esp-idf, 当前版本是4.4, 使用离线安装的方式, 安装时使用默认选项

    默认会安装: 内置的 Python, 交叉编译器, OpenOCD, CMake 和 Ninja 编译工具, ESP-IDF

### 方式2

    使用 vscode, 安装 esp idf 插件, 然后, 在 命令中执行: config esp extention, 根据相应的选项下载 esp环境所需要的文件.


## 创建项目

1. 创建一个 hello-world 项目:

    使用特定的终端, 在开始菜单中打开: esp-idf 4.4 cmd

    cd C:\Espressif\frameworks\esp-idf-v4.4

    cp -r examples/get-started/hello_world .

    cd hello_world

    idf.py set-target esp32c3

    idf.py menuconfig
