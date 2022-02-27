# TFT_LCD ST7735 显示屏

淘宝链接: https://item.taobao.com/item.htm?spm=a21wu.12321156-tw.go-detail.1.4c3147fcPVOATP&id=522804334784

## 原理图

![](TFT_LCD%20ST7735%20显示屏/2022-02-20-16-59-01.png)

## 引脚描述

![](TFT_LCD%20ST7735%20显示屏/2022-02-20-17-02-48.png)

## ST7735 笔记

简单看一下 ST7735 的文档，我这个显示屏模块的电路设计决定了 使用的是 4线串口通信 (CS SCL SDA D/C) + RES

屏幕分辨率为 132 * 162 像素点

每一次传输的过程一般是: 先发送命令 再发送参数 (Param Data 都应该是参数)

9.4 Data Transfer Break and Recovery

如果数据传输中 RES 产生了一个 break (低电平), 那么 driver 将拒绝前面传输的bits, 并重置接口, 直到 RES 为高电平的状态时, 准备下一次的命令数据传输. 过程如下:
![](TFT_LCD%20ST7735%20显示屏/2022-02-27-11-09-22.png)

如果数据传输中 CS 产生了一个 break (下降沿), 那么 driver 将拒绝前面传输的bits, 并重置接口, 直到 CS 为高电平的状态时, 重新上一次的事务(命令或参数), 过程如下:
![](TFT_LCD%20ST7735%20显示屏/2022-02-27-11-08-58.png)

想让一个屏幕旋转 90°, 则需要 X.Y 数据互换

想让一个屏幕旋转 270°, 则需要 X.Y 数据互换, 且 Y数据 从下到上

![](TFT_LCD%20ST7735%20显示屏/2022-02-27-17-43-22.png)

## esp32c3 应用

使用 esp32c3 驱动 显示屏, 由于 spi0 和 spi1 只能用于存储器操作, 故只能使用 spi2 作为通用spi设备

![](TFT_LCD%20ST7735%20显示屏/2022-02-20-17-10-07.png)

通过 esp32-c3_datasheet_cn.pdf 查看 SPI2 引脚定义

![](TFT_LCD%20ST7735%20显示屏/2022-02-20-17-13-50.png)

FSPICLK -> IO6
FSPID   -> IO7
FSPIQ   -> IO2
...............?????????????...........


根据文档说的, 最后我找了一天的引脚, 都没成功找到, 在 Youtube 上找到一个 Arduino 的项目, 成功确定了引脚 >_<

最后选择的引脚是

SCL  ->  SPI_CLK  -> IO4
SDA  ->  SPI_MOSI -> IO6
RES  ->  GPIO     -> IO9
DC   ->  GPIO     -> IO8
CS   ->  SPI_CS   -> IO10

## Arduino

安装两个库:
Adafruit_GFX
Adafruit ST7735 and Adafruit ST7789 Library

并安装 esp32 boards

[参考代码](./codes/esp32c3_st7735_demo/esp32c3_st7735_demo.ino)
