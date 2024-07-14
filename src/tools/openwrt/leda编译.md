## manjaro 安装编译环境

    yay -S openwrt-devel

## 下载代码

    git clone https://github.com/coolsnowwolf/lede
    cd lede

    ./scripts/feeds update -a
    ./scripts/feeds install -a

    make menuconfig

    make -j11 download V=s

    make -j11 V=s
