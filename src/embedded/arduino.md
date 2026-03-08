# Arduino 接口笔记

## 说明

- 本文记录 `STM32F746` 开发板上 Arduino 兼容接口与实际 MCU 引脚的对应查找方法.
- 更偏模块归类的内容, 可回到 [模块与扩展器件总览](./单片机模块.md).

## 引脚编号查找位置

可先查看对应开发板变体文件, 例如:

```text
variants/STM32F7xx/F746B(E-G)T_F746N(E-G)H_F750N8H_F756BGT_F756NGH/variant_generic.cpp
```

重点关注其中的数组:

- `digitalPin`
- `analogInputPin`

## 查找方法

- 先从原理图确认 Arduino 接口对应的 MCU 端口, 例如 `PA_15`.
- 再到 `digitalPin` 或 `analogInputPin` 数组中查找该端口的位置.
- 数组中的索引, 就是 Arduino 代码里使用的引脚编号.

例如:

- 若 `PA_15` 在 `digitalPin` 数组中的索引为 `15`, 则 Arduino 代码中使用的引脚号就是 `15`.

## 排查建议

- 先确认所选 `board` 与 `variant` 是否一致.
- 区分“Arduino 引脚编号”和“MCU 原始端口名”, 不要混用.
- 若某个引脚不可用, 还要检查其是否被复用为下载, 调试, 晶振或其他外设功能.
