# AGM 平台与工具链

## 说明

- 本文记录 `AGM` 平台相关的 FPGA 开发环境搭建与项目编译流程.
- 当前重点包括 `Quartus II`, `ModelSim`, `verilog-format` 和项目烧写流程.

## 环境准备

### Quartus II 与 USB Blaster

- 安装 `Quartus II 13.1` 官方安装包.
- 安装 `USB Blaster` 驱动.
- 如有历史环境需求, 再按项目要求处理兼容性或授权问题.

### ModelSim

- 安装 `modelsim-win64-10.4-se`.
- 具体版本需与项目环境保持一致.

## VSCode 中编辑 Verilog

### verilog-format

- 项目地址: https://github.com/ericsonj/verilog-format
- 若直接使用存在问题, 需要按项目说明修改后重新编译.

常见步骤:

```sh
mvn clean package
```

执行 `target/verilog-format.exe` 前, 需要确保本机存在 `JDK` 环境.

## 其他工具

### iverilog

- Windows 下载地址: https://bleyer.org/icarus/
- 插件安装说明: https://www.youtube.com/watch?v=vN1wzM0NO4c

## AGM 项目流程

典型流程示例:

1. 使用 `pio unlock flash`
2. 进入烧写模式
3. 在 `PlatformIO` 中执行 `Upload LOGIC` 等自定义流程
4. 进入正常模式

### CustomIP

在 `platformio.ini` 中可配置:

```ini
ip_name = custom_ip
logic_dir = logic
```

常见流程:

- 执行 `Prepare LOGIC` 生成 `logic` 目录.
- 使用 `QuartusII` 打开 `logic/example_board.qpf` 编译.
- 或使用 `Supra` 编译 `logic/example_board.proj`.
