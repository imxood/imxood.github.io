# TFT_LCD ST7735 显示屏

淘宝链接: https://item.taobao.com/item.htm?spm=a21wu.12321156-tw.go-detail.1.4c3147fcPVOATP&id=522804334784

## 原理图

![](TFT_LCD%20ST7735%20显示屏/2022-02-20-16-59-01.png)

## 引脚描述

![](TFT_LCD%20ST7735%20显示屏/2022-02-20-17-02-48.png)

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
