# HarmonyOS 嵌入式仿真

## 说明

- 本文记录 `HarmonyOS` 在 `stm32f4` 相关仿真环境下的依赖安装与问题排查.
- 当前更偏环境搭建笔记, 适合作为第一次把仿真链路跑通时的速查页.
- 如果后续继续整理, 建议再补“示例工程入口”和“实际构建输出”两部分内容.

## 适用场景

- 在 Linux 环境下准备 `HarmonyOS` 的嵌入式仿真工具链.
- 遇到 `Node.js`, `xpm`, `qemu-arm`, `openocd` 或系统依赖缺失时快速排查.
- 想先把仿真环境跑通, 再进入具体工程构建和调试阶段.

## 环境安装

### 安装 Node.js

```sh
curl -sL https://deb.nodesource.com/setup_14.x | sudo -E bash -
sudo apt-get install -y nodejs
```

### 安装 xpm 与仿真工具

```sh
sudo npm install --global xpm
xpm install --global @xpack-dev-tools/qemu-arm@latest
xpm install --global @xpack-dev-tools/openocd@latest
```

## 常见报错

### `dash` 兼容问题

```sh
sudo dpkg-reconfigure dash
```

选择 `No`.

### `alsa/asoundlib.h` 缺失

```sh
sudo apt-get install libasound2-dev
```

### `asm/unistd.h` 或构建依赖缺失

```sh
sudo apt install pkg-config
```

## 排查思路

- 先区分是系统依赖缺失, 还是工具链版本不匹配.
- 如果问题发生在安装阶段, 优先检查 `npm`, `xpm` 和系统包是否完整.
- 如果问题发生在构建阶段, 再进一步核对交叉编译环境和示例工程配置.

## 相关文档

- [STM32F103 Bluepill 总览](./stm32f103_bluepill/README.md)
- [OpenOCD 与 GDB](../tools/compile/openocd%20gdb%20笔记.md)
