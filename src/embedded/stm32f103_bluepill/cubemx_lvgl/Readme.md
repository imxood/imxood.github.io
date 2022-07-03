## 只产生指定字体的字体文件

工具: https://github.com/lvgl/lv_font_conv

lv_font_conv --bpp 4 --size 16 -o Core/Src/lv_custom_font_16.c --format lvgl --no-kerning --no-prefilter --font ../../../program/rust/code/fonts/DroidSansFallbackFull.ttf --symbols 你好

--no-prefilter - disable bitmap lines filter (XOR), used to improve compression ratio.
--no-kerning - drop kerning info to reduce size (not recommended).

## st7735 驱动

https://github.com/RobertoBenjami/stm32_graphics_display_drivers.git

使用部分代码

st7735 引脚说明

SCL  ->  SPI_CLK
SDA  ->  SPI_MOSI
RES  ->  GPIO
DC   ->  GPIO, RS, Data/Command
CS   ->  SPI_CS
