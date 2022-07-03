make -j
openocd -f interface/stlink.cfg -f board/stm32f103c8_blue_pill.cfg -c "program build/cubemx_lvgl.bin 0x8000000 reset exit"