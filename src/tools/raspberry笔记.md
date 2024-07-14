# raspberry笔记

## 使用 imager 烧写TF卡

    https://mirrors.tuna.tsinghua.edu.cn/raspbian-images/raspbian_lite/images/raspbian_lite-2020-02-14/2020-02-13-raspbian-buster-lite.zip

## 烧写后, 先在boot分区中,添加一个文件名是"ssh"的文件, 无后缀, 空文件, 这样会默认开启ssh, 这样可以不用显示屏

## 修改密码

## 修改软件源, 并更新源

    /etc/apt/sources.list
    
    # deb http://raspbian.raspberrypi.org/raspbian/ buster main contrib non-free rpi
    deb http://mirrors.aliyun.com/raspbian/raspbian/ buster main contrib non-free rpi
    
    /etc/apt/sources.list.d/raspi.list
    
    # deb http://archive.raspberrypi.org/debian/ buster main
    deb http://mirrors.ustc.edu.cn/archive.raspberrypi.org/debian/ buster main

## reboot

## 安装OMV

    在树莓派上安装OMV:
    https://github.com/OpenMediaVault-Plugin-Developers/docs
    
## reboot
    
## issues

    1. 不知为何刚安装好的树莓派上无法访问github上的资源

    --2020-04-18 04:01:08--  https://github.com/OpenMediaVault-Plugin-Developers/installScript/raw/master/install
    Resolving github.com (github.com)... 13.250.177.223
    Connecting to github.com (github.com)|13.250.177.223|:443... connected.
    HTTP request sent, awaiting response... 302 Found
    Location: https://raw.githubusercontent.com/OpenMediaVault-Plugin-Developers/installScript/master/install [following]
    --2020-04-18 04:01:09--  https://raw.githubusercontent.com/OpenMediaVault-Plugin-Developers/installScript/master/install
    Resolving raw.githubusercontent.com (raw.githubusercontent.com)... 0.0.0.0, ::
    Connecting to raw.githubusercontent.com (raw.githubusercontent.com)|0.0.0.0|:443... failed: Connection refused.
    Connecting to raw.githubusercontent.com (raw.githubusercontent.com)|::|:443... failed: Connection refused.
    
    处理方法: 使用代理, 配置 http_proxy, https_proxy
    
## 挂载硬盘, 设置共享, 添加用户权限


## 2021-1201

    1. 系统镜像

        http://mirrors.ustc.edu.cn/raspberry-pi-os-images/raspios_armhf/images/raspios_armhf-2021-11-08/

        基本桌面环境

    2. 使用 Etcher 烧写镜像

        deepin 应用市场安装 Etcher