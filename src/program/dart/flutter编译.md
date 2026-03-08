# Flutter 引擎编译

## 说明

- 本页记录一个较早期的 Flutter Engine Windows x86 编译尝试.
- 当前更适合作为历史链路记录, 使用时请结合当前 Flutter Engine 版本核对.

## Flutter 源码编译 x86 架构

### 1. 安装 Windows 构建环境

- 在 Visual Studio 中安装 C++ 开发环境.
- 如果提示缺少 `msdia140.dll`, 通常还需要确认以下组件:
  - `Windows Software Development Kit`
  - `Debugging Tools for Windows`

### 2. 设置环境变量

```powershell
$env:DEPOT_TOOLS_WIN_TOOLCHAIN="0"
$env:GYP_MSVS_OVERRIDE_PATH="D:\Program Files\Microsoft Visual Studio\2022\Community"
```

### 3. 下载代码

```powershell
gclient config https://github.com/flutter/engine.git@375834305d15a70b16f8a66eadb0b61ed2c5bf24
gclient sync --no-history -v
```

### 4. 生成与编译

```powershell
.\flutter\tools\gn --target-os=win --windows-cpu=x86 --runtime-mode=jit_release --no-goma
```

## 记录结论

- 当时的记录结论是: `x86` 架构未能顺利编译通过.
- 如果后续继续排查, 需要优先确认对应提交版本是否仍支持该目标架构.

## 后续可补

- `depot_tools` 完整安装流程.
- `ninja` 构建命令.
- 失败日志与依赖版本对应关系.
