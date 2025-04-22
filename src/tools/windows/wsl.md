# wsl 使用

##

启用 WSL2
dism.exe /online /enable-feature /featurename:Microsoft-Windows-Subsystem-Linux /all /norestart

启用虚拟机平台
dism.exe /online /enable-feature /featurename:VirtualMachinePlatform /all /norestart

启用 Hyper-V
dism.exe /online /enable-feature /featurename:Microsoft-Hyper-V /all /norestart

设置 WSL2 为默认
wsl --set-default-version 2

## 遇到错误

Installing, this may take a few minutes...
WslRegisterDistribution failed with error: 0x800701bc
Error: 0x800701bc WSL 2 ??????????????????

解决方法 (下载 Linux 内核更新包):

https://learn.microsoft.com/zh-cn/windows/wsl/install-manual#step-4---download-the-linux-kernel-update-package

## 启用 wsl

    Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Windows-Subsystem-Linux

##

netsh interface ip show config

netsh interface ip set address "vEthernet (Default Switch)" dhcp

netsh interface ip set address name="Network Interface Name" static [IP address] [Subnet Mask] [Gateway]

##

https://docs.microsoft.com/zh-cn/virtualization/hyper-v-on-windows/user-guide/setup-nat-network

1. 以管理员身份打开 PowerShell 控制台

2. 创建内部交换机

New-VMSwitch -SwitchName "SwitchName" -SwitchType Internal

3. 查找刚创建的虚拟交换机的接口索引

Get-NetAdapter

4. 使用 New-NetIPAddress 配置 NAT 网关

New-NetIPAddress -IPAddress 192.168.0.1 -PrefixLength 24 -InterfaceIndex 24

5. 使用 New-NetNat 配置 NAT 网络

New-NetNat -Name MyNATnetwork -InternalIPInterfaceAddressPrefix 192.168.0.0/24

##

sudo vim /etc/wsl.conf

添加:

[network]
generateResolvConf = false

sudo ip addr del $(ip addr show eth0 | grep 'inet\b' | awk '{print $2}' | head -n 1) dev eth0
sudo ip addr add 172.16.0.19/24 broadcast 172.16.0.255 dev eth0

sudo ip route add 0.0.0.0/0 via 172.16.0.1 dev eth0

sudo vim /etc/resolv.conf

修改为
nameserver 172.16.0.1

sudo crontab -e

新增:

@reboot /usr/sbin/netplan apply

##

ip addr | grep eth0

cat /etc/resolv.conf

输出:
...
nameserver 172.19.0.1

修改 172.19.0.1 到:

修改

##

wsl --list --online
wsl --install -d ubuntu

wsl --list --all -v

删除 ubuntu
wsl --unregister ubuntu
wsl --install -d Ubuntu-22.04

设置默认用户

wsl --shutdown
ubuntu2204.exe config --default-user root

设置默认 Ubuntu-22.04
wsl --setdefault Ubuntu-22.04

/setdefaultuser

sudo cp /etc/apt/sources.list /etc/apt/sources.list.bak
sudo sed -i 's/archive.ubuntu.com/mirrors.tuna.tsinghua.edu.cn/g' /etc/apt/sources.list
sudo sed -i 's/security.ubuntu.com/mirrors.tuna.tsinghua.edu.cn/g' /etc/apt/sources.list

## 设置自动补全

Ubuntu-22.04, root 用户默认没有自动补全

https://www.cnblogs.com/chencoolandclear/p/16464454.html

## 设置桥接网络

在 "Hyper-V 管理器" 中

新增一个虚拟交换机 VETH

设置 外部网络, 选择指定网卡

在 Windows 的 ~/.wslconfig 中, 设置

[wsl2]
networkingMode = bridged
vmSwitch = "VETH"
ipv6 = true

这种方式 仍有效, 但 微软已弃用

## 设置镜像网络

在 Windows 的 ~/.wslconfig 中, 设置

[wsl2]
networkingMode = mirrored

就可以直接访问 127.0.0.1 了

## 下载 choco

    用administrator执行powershell:
        Set-ExecutionPolicy Bypass -Scope Process -Force; iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))

    查看是否安装成功:
        choco --version

    查找软件包:
        https://chocolatey.org/search

## 下载 LxRunOffline

[LxRunOffline wiki](https://github.com/DDoSolitary/LxRunOffline/wiki)

    choco install LxRunOffline

## 下载 Ubuntu

    https://github.com/tianon/docker-brew-ubuntu-core/tree/dist-amd64

## 安准 wsl

    LxRunOffline.exe i -n ubuntu20 -d D:\WSL\ubuntu20 -f "D:\develop\software\windows\wsl\ubuntu-focal-core-cloudimg-amd64-root.tar.gz" -s

    名称: ubuntu20
    路径: D:\WSL\ubuntu
    并创建缩略图

## 配置 ubuntu

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

## 指定用户登录

wsl -u xxx

# 20241222

wsl --list --online
wsl --install -d ubuntu

wsl --list --all -v

<!-- 移除 -->

wsl --unregister Ubuntu-18.04

wsl --install -d Ubuntu-18.04

## wsl 使用代理

根据 ip addr 查看 wsl 中 linux 的 ip, 在 windows 中 ipconfig 查看该网段下的 ip.

在当前 wsl 的 linux 用户的 bash 环境中设置 ~/.bashrc:

export ALL_PROXY=http://172.29.160.1:1080

## 重命名主机名

sudo hostnamectl set-hostname NEW_HOSTNAME

<!-- export LIBGL_ALWAYS_INDIRECT=1
export DISPLAY=172.29.160.1:0 -->

## 设置国内软件源

wget -qO- https://linuxmirrors.cn/main.sh | sudo bash

## 使用 UI

sudo apt install xfce4 xfce4-terminal gtk2-engines-pixbuf

## 更改默认终端

sudo apt remove gnome-terminal

sudo update-alternatives --config x-terminal-emulator

##

sudo add-apt-repository ppa:deadsnakes/ppa

sudo apt install python3.8 python3-pip

sudo update-alternatives --install /usr/bin/python3 python3 /usr/bin/python3.6 1
sudo update-alternatives --install /usr/bin/python3 python3 /usr/bin/python3.8 1

sudo update-alternatives --list python3

sudo update-alternatives --config python3

当执行 python3 -c "import apt_pkg" 时 报错: No module named 'apt_pkg'

修复:

cd /usr/lib/python3/dist-packages

sudo mv apt_pkg.cpython-36m-x86_64-linux-gnu.so apt_pkg.so

设置清华源:

python3 -m pip install --upgrade pip
python3 -m pip config set global.index-url https://mirrors.tuna.tsinghua.edu.cn/pypi/web/simple

## 安装新的 llvm

wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | sudo apt-key add -

echo "deb http://apt.llvm.org/bionic/ llvm-toolchain-bionic main" | sudo tee -a /etc/apt/sources.list
sudo apt-get update

sudo apt install llvm-19-dev

## 编译 Mesa 显卡驱动

### 依赖

sudo apt install libclc-dev python3-setuptools \
 libdrm-dev libelf-dev libgbm-dev libpciaccess-dev libx11-dev \
 libxext-dev libxdamage-dev libxfixes-dev libxrandr-dev \
 libglvnd-dev libvulkan-dev libwayland-dev wayland-protocols

 <!-- glslang-tools -->

pip3 install meson ninja cmake mako

cargo install bindgen-cli

### 安装 glslang

sudo apt-get install vulkan-tools

低于 ubuntu19 需要编译

git clone https://github.com/KhronosGroup/glslang.git

cd glslang

git checkout 7.10.2984

python3 update_glslang_sources.py

mkdir build && cd build

cmake ..

make -j4

sudo make install

### libdrm 版本过低 手动编译

git clone https://gitlab.freedesktop.org/mesa/drm.git

cd drm

git checkout libdrm-2.4.124

meson builddir/

ninja -C builddir/ install

### 代码

git clone https://gitlab.freedesktop.org/mesa/mesa.git

cd mesa

git checkout 24.3

meson setup build
ninja -C build/
sudo ninja -C build/ install
