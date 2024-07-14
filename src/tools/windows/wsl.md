
# wsl使用

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

删除ubuntu

wsl --unregister ubuntu


sudo cp /etc/apt/sources.list /etc/apt/sources.list.bak
sudo sed -i 's/archive.ubuntu.com/mirrors.tuna.tsinghua.edu.cn/g' /etc/apt/sources.list
sudo sed -i 's/security.ubuntu.com/mirrors.tuna.tsinghua.edu.cn/g' /etc/apt/sources.list


## 设置桥接网络

查看网络设备:
    
Get-NetAdapter

设置桥接设备

Set-VMSwitch WSL -NetAdapterName 以太网

取消桥接

Set-VMSwitch WSL -SwitchType Internal

## 

启用WSL2

dism.exe /online /enable-feature /featurename:Microsoft-Windows-Subsystem-Linux /all /norestart
启用虚拟机平台

dism.exe /online /enable-feature /featurename:VirtualMachinePlatform /all /norestart
启用Hyper-V

dism.exe /online /enable-feature /featurename:Microsoft-Hyper-V /all /norestart
设置WSL2为默认

wsl --set-default-version 2


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
