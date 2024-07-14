# virtualbox 使用指南

## 安装

    linux:
        yay -S virtualbox linux56-virtualbox-host-modules virtualbox-ext-oracle
        sudo gpasswd -a $USER vboxusers
        sudo vboxreload
        reboot now
