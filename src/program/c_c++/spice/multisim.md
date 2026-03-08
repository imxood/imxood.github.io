# Multisim 瞬态仿真收敛问题

## 说明

- 本页记录一次 `Multisim` / `SPICE` 瞬态仿真不收敛的错误信息.
- 内容较短, 当前作为图形化仿真工具排障片段保留在 `SPICE` 目录下.

## 错误

------ 正在为 放大器-仪表放大器 - 2023年6月12日, 19:58:24 检查 SPICE 网表------
======= SPICE 网表检查完毕, 0 错误, 0 警告 =======
Error: Unable to converge during transient analysis. Consider increasing the ABSTOL, VNTOL, and RELTOL options.
Simulation canceled
See convergence help for more information

### 修复:
