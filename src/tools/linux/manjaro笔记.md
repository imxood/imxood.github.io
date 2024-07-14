# manjaro笔记

## 下载系统镜像

    https://mirrors.tuna.tsinghua.edu.cn/osdn/storage/g/m/ma/manjaro/xfce/19.0.2/manjaro-xfce-19.0.2-200311-linux54.iso

## 写到U盘

    dd if=manjaro-xfce-19.0.2-200311-linux54.iso of=/Dev/sdb bs=16M

## 从U盘启动

    # 设置国内源

    sudo pacman-mirrors -c China
    sudo pacman -Syyu

    # 添加源

    sudo pacman -S vim
    sudo vim /etc/pacman.conf

    [archlinuxcn]
    SigLevel = Optional TrustedOnly
    Server = https://mirrors.ustc.edu.cn/archlinuxcn/$arch

    sudo pacman -Syy && sudo pacman -S archlinuxcn-keyring

    # 中文字体
    sudo pacman -S wqy-bitmapfont wqy-microhei wqy-microhei-lite wqy-zenhei

    # google输入法
    sudo pacman -S kcm-fcitx fcitx-googlepinyin

    # 搜狗输入法
    sudo pacman -S fcitx-lilydjwg-git fcitx-configtool fcitx-sogoupinyin

    ~/.xprofile
    export LC_ALL=zh_CN.UTF-8
    export GTK_IM_MODULE=fcitx
    export QT_IM_MODULE=fcitx
    export XMODIFIERS="@im=fcitx"

    # 如果搜狗有问题，卸载搜狗输入法
    sudo pacman -Rs fcitx-lilydjwg-git fcitx-configtool fcitx-sogoupinyin
    cd ~/.config
    rm -rf SogouPY SogouPY.users sogou-qimpanel fcitx

    # 安装zsh

    sudo pacman -S zsh
    sh -c "$(curl -fsSL https://raw.github.com/robbyrussell/oh-my-zsh/master/tools/install.sh)"
    # chsh -s /bin/zsha
    sudo usermod -s /bin/zsh USERNAME

    theme: mortalscumbag afowler gentoo gallois


    # 必要软件

    sudo pacman -S visual-studio-code-bin
    sudo pacman -S screenfetch
    sudo pacman -S clang gdb
    sudo pacman -S deepin-screenshot
    sudo pacman -S wps-office ttf-wps-fonts
    sudo pacman -S netease-cloud-music


    # docker
    sudo pacman -S docker docker-compose

    docker镜像加速器:
        https://cr.console.aliyun.com/cn-hangzhou/instances/mirrors
        https://www.daocloud.io/mirror

    # stm32开发环境
    sudo pacman -S arm-none-eabi-gcc arm-none-eabi-gdb arm-none-eabi-newlib

    # jdk8
    sudo pacman -S jdk8-openjdk

    sudo pacman -S yay

    AUR库:
    yay -Syy
    yay -S stm32cubemx
    yay -S deepin.com.qq.office
    yay -S deepin-wine-wechat

    https://github.com/countstarlight/deepin-wine-wechat-arch/releases

