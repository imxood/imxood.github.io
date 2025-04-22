## vscode platformio arduino 环境

vscode 插件中安装 platformio

创建新项目, 设置 esp32 项目

### 添加库

在 Library 中搜索并添加到项目中

### 修改默认 idf 版本

需要注意的是: platformio 当前使用的 esp32 idf 的版本是 4.4.7?

修改:

在 platformio.ini 文件:

设置:

platform = https://github.com/pioarduino/platform-espressif32/releases/download/53.03.11/platform-espressif32.zip

即可使用 5.3.2.241224 版本

### 设置 monitor 的波特率

修改 platformio.ini 文件:

monitor_speed = 115200

### platformio.ini 的更多设置, 参考

https://docs.platformio.org/en/latest/platforms/espressif32.html
