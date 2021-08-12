# wsl使用

参考 https://www.niconya.com/bv/555d/

## 启用wsl

    Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Windows-Subsystem-Linux

## 下载choco

    用administrator执行powershell:
        Set-ExecutionPolicy Bypass -Scope Process -Force; iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))

    查看是否安装成功:
        choco --version

    查找软件包:
        https://chocolatey.org/search

## 下载LxRunOffline

[LxRunOffline wiki](https://github.com/DDoSolitary/LxRunOffline/wiki)

    choco install LxRunOffline

## 下载Ubuntu

    https://github.com/tianon/docker-brew-ubuntu-core/tree/dist-amd64

## 安准wsl

    LxRunOffline.exe i -n ubuntu20 -d D:\WSL\ubuntu20 -f "D:\develop\software\windows\wsl\ubuntu-focal-core-cloudimg-amd64-root.tar.gz" -s

    名称: ubuntu20
    路径: D:\WSL\ubuntu
    并创建缩略图

## 配置ubuntu

    sed -i 's/archive.ubuntu.com/mirrors.aliyun.com/g' /etc/apt/sources.list
    sed -i 's/security.ubuntu.com/mirrors.aliyun.com/g' /etc/apt/sources.list
    apt update
    apt install -y sudo vim bash-completion python3 python3-pip

    # update-alternatives --install /usr/bin/python python /usr/bin/python3.8 2

    useradd -m -s /bin/bash imxood
    新增用户: imxood

    passwd imxood
    设置密码

    usermod -aG sudo imxood
    添加到sudo组

    id -u imxood
    为了设置默认用户, 查看用户id, 应该是1000

    lxrunoffline su -n ubuntu20 -v 1000
    添加id是1000的用户为默认用户

    vim /etc/wsl.conf

    [automount]
    enabled = true
    root = /mnt/
    options = "metadata,umask=22,fmask=11"
    mountFsTab = false

    vim ~/.bashrc

    #Fix mkdir command has wrong permissions
    if grep -q Microsoft /proc/version; then
        if [ "$(umask)" == '0000' ]; then
            umask 0022
        fi
    fi
