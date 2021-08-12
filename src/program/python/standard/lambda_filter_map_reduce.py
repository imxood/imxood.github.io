#!/usr/bin/env python3
import os
import sys
from functools import reduce

command = '~/programs/zephyr-sdk/arm-zephyr-eabi/bin/arm-zephyr-eabi-gcc -DBUILD_VERSION=zephyr-v1.14.0-1767-g46ea1df2ec54 -DKERNEL -DSTM32F769xx -DUSE_FULL_LL_DRIVER -DUSE_HAL_DRIVER -D_FORTIFY_SOURCE=2 -D__ZEPHYR__=1 -I/home/maxu/develop/sources/zephyrproject/zephyr/kernel/include -I/home/maxu/develop/sources/zephyrproject/zephyr/arch/arm/include -I/home/maxu/develop/sources/zephyrproject/zephyr/include -I/home/maxu/develop/sources/zephyrproject/zephyr/include/drivers -Izephyr/include/generated -I/home/maxu/develop/sources/zephyrproject/zephyr/soc/arm/st_stm32/stm32f7 -I/home/maxu/develop/sources/zephyrproject/zephyr/lib/libc/minimal/include -I/home/maxu/develop/sources/zephyrproject/zephyr/drivers -I/home/maxu/develop/sources/zephyrproject/zephyr/ext/lib/fnmatch/. -I/home/maxu/develop/sources/zephyrproject/zephyr/ext/hal/cmsis/Include -I/home/maxu/develop/sources/zephyrproject/zephyr/subsys/testsuite/include -I/home/maxu/develop/sources/zephyrproject/zephyr/subsys/testsuite/ztest/include -I/home/maxu/develop/sources/zephyrproject/modules/hal/stm32/stm32cube/stm32f7xx/soc -I/home/maxu/develop/sources/zephyrproject/modules/hal/stm32/stm32cube/stm32f7xx/drivers/include -I/home/maxu/develop/sources/zephyrproject/modules/hal/stm32/stm32cube/stm32f7xx/drivers/include/Legacy -isystem /home/maxu/programs/zephyr-sdk/arm-zephyr-eabi/bin/../lib/gcc/arm-zephyr-eabi/8.3.0/include -isystem /home/maxu/programs/zephyr-sdk/arm-zephyr-eabi/bin/../lib/gcc/arm-zephyr-eabi/8.3.0/include-fixed  -g   -Og -nostdinc --imacros=/home/maxu/develop/sources/zephyrproject/zephyr/output/zephyr/include/generated/autoconf.h --imacros=/home/maxu/develop/sources/zephyrproject/zephyr/include/toolchain/zephyr_stdint.h -ffreestanding -fno-common -g -mthumb -mcpu=cortex-m7 -Wall -Wformat -Wformat-security -Wno-format-zero-length -Wno-main -Wno-pointer-sign -Wpointer-arith -Wno-unused-but-set-variable -Werror=implicit-int -fno-asynchronous-unwind-tables -fno-pie -fno-pic -fno-strict-overflow -fno-reorder-functions -fno-defer-pop -fmacro-prefix-map=/home/maxu/develop/sources/zephyrproject/zephyr/stm32f7_app=CMAKE_SOURCE_DIR -fmacro-prefix-map=/home/maxu/develop/sources/zephyrproject/zephyr=ZEPHYR_BASE -ffunction-sections -fdata-sections -mabi=aapcs -std=c99 -o CMakeFiles/app.dir/sys/pinmux.c.obj   -c /home/maxu/develop/sources/zephyrproject/zephyr/stm32f7_app/sys/pinmux.c'

# Use Lambda, Filter, Map

v = map(lambda x: x[2:], filter(lambda x: x.startswith('-D'), command.split()))
print(v)

for va in v:
    print(va)

defines = list(map(lambda x: x[2:], filter(lambda x: x.startswith('-D'), command.split())))

print(defines)

# Use Reduce

# Calculate 1 + 2 + 3 + 4
lst = [1, 2, 3, 4]
print(reduce(lambda x,y: x + y, lst))
