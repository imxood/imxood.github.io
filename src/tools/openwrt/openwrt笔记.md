# 试玩笔记

## openwrt 源码编译

    参考: https://openwrt.org/docs/guide-developer/quickstart-build-images

    Host系统: Ubuntu 18.04
    Wifi设备: Newifi D2

    对于 ubuntu18 or later:
        sudo apt install build-essential libncurses5-dev python unzip


    git clone https://git.openwrt.org/openwrt/openwrt.git && cd openwrt


    ./scripts/feeds update -a
    ./scripts/feeds install -a

    make menuconfig

    # 选择要编译目标设备
    Target System --> (X) MediaTek Ralink MIPS
    Subtarget --> (X) MT7621 based boards
    Target Profile --> (X) Newifi D2

    # 添加硬盘格式支持(否则无法识别mount的设备)
    Kernel modules --> Filesystems --> <*> kmod-fs-ext4 (移动硬盘EXT4格式选择)
    Kernel modules --> Filesystems --> <*> kmod-fs-vfat(FAT16 / FAT32 格式 选择)
    Kernel modules --> Filesystems --> <*> kmod-fs-ntfs (NTFS 格式 选择)

    # 添加USB相关支持
    Kernel modules --> USB Support --> <*> kmod-usb-core
    Kernel modules --> USB Support --> <*> kmod-usb-ohci
    Kernel modules --> USB Support --> <*> kmod-usb-storage
    Kernel modules --> USB Support --> <*> kmod-usb-storage-extras
    Kernel modules --> USB Support --> <*> kmod-usb2
    Kernel modules --> USB Support --> <*> kmod-usb3

    # 添加编码
    Kernel modules --> Native Language Support --> <*> kmod-nls-cp936
    Kernel modules --> Native Language Support --> <*> kmod-nls-iso8859-1
    Kernel modules --> Native Language Support --> <*> kmod-nls-utf8

    # 中文
    LuCI --> Modules --> Translation --> <*> Chinese (zh-cn)

    # Web GUI
    LuCI ->Collections -> [*] luci.....LuCI interface with Uhttpd as Webserver

    # 添加USB自动挂载
    Base system --> <*> blockd.....Block device automounting

    退出, 保存

    安装桥接工具
    ./scripts/feeds install luci-proto-relay

    vim package/base-files/files/etc/shadow, 设置密码(admin):
        root:$1$2nX0sqkM$XILd1/grLi/99Lgvp6Clz0:16922:0:99999:7:::

    make -j12 V=s

    # 生成文件位于: bin/targets/ramips/mt7621

## 烧写

    http://192.168.1.1/

    固件更新 ->

## 连接wifi

    网络 --> 无线 --> 扫描 --> 选择热点, 加入网络 --> 密码, 5G通道就选择 US 149 --> ok

## 修改ip

18的版本, 只能通过ssh, 在设备上直接修改, 因为 There is a emergency revert feature in version 18 that will go back if the web GUI cannot be reached.

vim /etc/config/network

修改interface 'lan'中的 option ipaddr '192.168.1.1'

/etc/init.d/network restart

## 设置二级路由桥接





