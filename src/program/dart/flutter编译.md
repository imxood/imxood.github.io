## flutter源码编译 x86架构

```
1. visual studio 安装 c++环境.

如果提示报错: 缺少 msdia140.dll

应该自动包含: Windows Software Development Kit

添加 Debugging Tools for Windows.

2. 环境变量

$env:DEPOT_TOOLS_WIN_TOOLCHAIN="0"
$env:GYP_MSVS_OVERRIDE_PATH="D:\Program Files\Microsoft Visual Studio\2022\Community"

3. 下载代码

gclient config https://github.com/flutter/engine.git@375834305d15a70b16f8a66eadb0b61ed2c5bf24

gclient sync --no-history -v

4. 编译

.\flutter\tools\gn --target-os=win --windows-cpu=x86 --runtime-mode=jit_release --no-goma

无法编译 x86 架构

```
