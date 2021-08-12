# 使用qemu_stm32

## 安装依赖

    sudo apt-get install -y --no-install-recommends binutils-arm-none-eabi ca-certificates libnewlib-arm-none-eabi findutils gcc git libglib2.0-dev libfdt-dev libpixman-1-dev make openssh-client pkgconf python zlib1g-d

## 下载code

    git clone https://github.com/beckus/qemu_stm32.git

## 编译

    cd qemu_stm32 && mkdir -p build && cd build
    ../configure --disable-werror --enable-debug --target-list="arm-softmmu" --extra-cflags=-DSTM32_UART_NO_BAUD_DELAY --extra-cflags=-DSTM32_UART_ENABLE_OVERRUN --disable-gtk
    make -j4
    sudo make install

