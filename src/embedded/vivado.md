# Vivado

## 说明

- `Vivado` 是 Xilinx FPGA 常用开发工具链, 适合做工程创建, 约束配置, 综合, 实现和下载调试.
- 当前记录以 `XC7A35` 开发板为起点.

## 环境准备

- 安装对应版本的 `Vivado`, 当前笔记基于 `Vivado 2018`.
- 若开发板不在默认列表中, 还需要额外安装板卡描述文件.

板卡文件参考:

- https://github.com/Digilent/vivado-boards

常见放置路径示例:

```text
/develop/programs/Xilinx/Vivado/2018.3/data/boards/board_files
```

## 常见工作流

- 创建工程并选择目标器件或板卡.
- 添加 `Verilog` / `VHDL` 源码和约束文件.
- 运行综合与实现.
- 生成 bitstream 后下载到开发板.

## 排查建议

- 先确认器件型号和约束文件是否匹配.
- 时序不过时, 优先检查时钟定义, IO 约束和跨时钟域处理.
- 下载失败时, 再检查 JTAG 连接, 驱动和硬件供电.

## 相关文档

- [FPGA 总览](./fpga/README.md)
- [Verilog / FPGA 要点](./fpga/fpga.md)
