## 只产生指定字体的字体文件

lv_font_conv --bpp 4 --size 16 -o Core/Src/lv_custom_font_16.c --format lvgl --no-kerning --no-prefilter --font ../../../program/rust/code/fonts/DroidSansFallbackFull.ttf --symbols 你好

--no-prefilter - disable bitmap lines filter (XOR), used to improve compression ratio.
--no-kerning - drop kerning info to reduce size (not recommended).
