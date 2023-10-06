# Linux笔记

## 自动挂载

    sudo blkid

    vim /etc/fstab

    UUID=7f02008a-bbbc-4db5-ab13-eafeacf19356 /develop ext4 defaults 0 2

    第一个数字0表示不使用bump程序对它进行备份。
    第二个数字2表示开机不优先检查此磁盘，1表示开机优先检查磁盘，用于根分区/, 2用于普通分区，0禁止磁盘检查

    重启

## issue

    xxx@xxx-pc:/media/xxx$ sudo mount -t ntfs-3g /dev/sdb4 /mnt/A/
    The disk contains an unclean file system (0, 0).
    Metadata kept in Windows cache, refused to mount.
    Falling back to read-only mount because the NTFS partition is in an
    unsafe state. Please resume and shutdown Windows fully (no hibernation
    or fast restarting.)

    修复: sudo ntfsfix /dev/sdb4


## linux 批量重命名

rename -v 's/.asm/.nasm/' *.asm

## 防火墙规则

<center><img src="imgs/iptables_rule.jpg"/></center>
<center><img src="imgs/iptables_rule_param.jpg"/></center>

-j

ACCEPT 允许数据包通过
DROP 直接丢弃数据包，不给任何回应信息
REJECT 拒绝数据包通过，必要时会给数据发送端一个响应的信息。

1. 删除INPUT链的第一条规则
    iptables -D INPUT 1

1. 拒绝进入防火墙的所有ICMP协议数据包
    iptables -I INPUT -p icmp -j REJECT

1. 允许防火墙转发除ICMP协议以外的所有数据包
    iptables -I FORWARD -p ! icmp -j ACCEPT

1. 拒绝转发来自192.168.1.10主机的数据，允许转发来自192.168.0.0/24网段的数据
    iptables -A FORWARD -s 192.168.1.11 -j REJECT
    iptables -A FORWARD -s 192.168.0.0/24 -j ACCEPT
    ps: 要把拒绝的放在前面不然就不起作用了啊

1. 丢弃从外网接口(eth1)进入防火墙本机的源地址为私网地址的数据包
    iptables -A INPUT -i eth1 -s 192.168.0.0/16 -j DROP
    iptables -A INPUT -i eth1 -s 172.16.0.0/12 -j DROP
    iptables -A INPUT -i eth1 -s 10.0.0.0/8 -j DROP

1. 只允许管理员从202.13.0.0/16网段使用SSH远程登录防火墙主机
    iptables -A INPUT -p tcp --dport 22 -s 202.13.0.0/16 -j ACCEPT
    iptables -A INPUT -p tcp --dport 22 -j DROP
    ps: -d 目的主机, -s 源主机, --dport 目的端口 --sport 来源端口

1. 允许本机开放从TCP端口20-1024提供的应用服务
    iptables -A INPUT -p tcp --dport 20:1024 -j ACCEPT
    iptables -A OUTPUT -p tcp --sport 20:1024 -j ACCEPT

1. 允许转发来自192.168.0.0/24局域网段的DNS解析请求数据包
    iptables -A FORWARD -s 192.168.0.0/24 -p udp --dport 53 -j ACCEPT
    iptables -A FORWARD -d 192.168.0.0/24 -p udp --sport 53 -j ACCEPT

1. 禁止其他主机ping防火墙主机，但是允许从防火墙上ping其他主机
    iptables -I INPUT -p icmp --icmp-type Echo-Request -j DROP
    iptables -I INPUT -p icmp --icmp-type Echo-Reply -j ACCEPT
    iptables -I INPUT -p icmp --icmp-type destination-Unreachable -j ACCEPT

1. 禁止转发来自MAC地址为00：0C：29：27：55：3F的和主机的数据包
    iptables -A FORWARD -m mac --mac-source 00:0c:29:27:55:3F -j DROP
    ps: "-m mac --mac-source"来表示数据包的源MAC地址

1. 允许防火墙本机对外开放TCP端口20、21、25、110以及被动模式FTP端口1250-1280
    iptables -A INPUT -p tcp -m multiport --dport 20,21,25,110,1250:1280 -j ACCEPT
    ps: -m multiport --dport, 指定目的端口及范围

1. 禁止转发源IP地址为192.168.1.20-192.168.1.99的TCP数据包
    iptables -A FORWARD -p tcp -m iprange --src-range 192.168.1.20-192.168.1.99 -j DROP
    ps: "-m --iprange --src-range"指定IP范围

1. 禁止转发与正常TCP连接无关的非—syn请求数据包
    iptables -A FORWARD -m state --state NEW -p tcp ! --syn -j DROP
    ps: "-m state"表示数据包的连接状态, "NEW"表示与任何连接无关的，新的嘛

1. 拒绝访问防火墙的新数据包，但允许响应连接或与已有连接相关的数据包
    iptables -A INPUT -p tcp -m state --state NEW -j DROP
    iptables -A INPUT -p tcp -m state --state ESTABLISHED,RELATED -j ACCEPT
    ps: "ESTABLISHED"表示已经响应请求或者已经建立连接的数据包，"RELATED"表示与已建立的连接有相关性的，比如FTP数据连接等

## CIDR子网划分

    IP地址 = 网络前缀 + 主机号

    192.168.0.7/18 = 11000000 10101000 00000000 00000111
    18, 即连续18个1, 也就是网络地址占18位, 后14位是主机号

    最小地址是：192.168.0.0      = 11000000 00001110 00000000 00000000 
    最大地址是：192.168.63.255   = 11000000 00001110 00111111 11111111
    子网掩码是：255.255.192.0    = 11111111 11111111 11000000 00000000

## video card

    sudo ubuntu-drivers devices
    sudo ubuntu-drivers autoinstall

## ssh 免密登录

生成证书:

    ssh-keygen -t rsa

拷贝公钥到服务器上:

    ssh-copy-id USER@HOST

## 指定用户登出

    pkill -kill -t tty

## grep技巧, -r 递归, -i 忽略大小写, -n 显示行数, --exclude-dir 排除目录, -E 表示可扩展的正则表达式

    sudo grep "ss-qt5" / -r -i -n --exclude-dir={proc,sys}
    echo aabbccddbbee | awk '{split($0,arr,"bb");for(i in arr) print arr[i]}'

## 查看所有网卡

    ifconfig -a
    cat /proc/net/dev

## ubuntu 重启网络

    sudo service networking restart

## 设置开机启动以太网eth0

    sudo vim /etc/network/interfaces
    自启动eth0, 并设置eth0为ip4, 自动分配ip:

        auto eth0
        iface eth0 inet dhcp

    自启动eth0, 并设置eth0为ip4, 静态ip:
        auto eth0
        iface eth0 inet static
            address 192.10.1.2
            netmask 255.255.255.0
            gateway 192.10.1.254

## ssh安装

    sudo apt-get install openssh-server
    sudo vim /etc/ssh/sshd_config
    把配置文件中的"PermitRootLogin without-password"加一个"#"号,
    再增加一句"PermitRootLogin yes",
    保存，修改成功。

## Linux系统如何查看版本信息

    打印系统信息, 包括: -s 内核名称, -r 内核版本, -n 网络节点名称(hostname), -o 操作系统(eq: GNU/Linux), -p 处理器结构
    uname -a
    显示内核版本:            cat /proc/version
    显示发行版本信息:         cat /etc/issue
                           lsb_release -a

## ubuntu搜索已安装软件

    dpkg -l

## 查看当前目录大小

    du -sh

### 用户管理

#### 添加用户

sudo useradd -m -s /bin/bash xxx 添加用户 xxx

-m 自动创建home目录, home模板来自/etc/skel

-s 指定bash, 若不指定, 则默认是/bin/sh, 没有tab自动补全及当前路径

#### 修改用户组

sudo usermod -a -G sudo,adm,dialout xxx 给用户 xxx 追加sudo,adm,dialout组

-a, 追加, 与 -G 一起使用, 追加组

#### 删除用户

sudo userdel xxx, 删除用户, 不删除用户目录, 若加上-r会把用户目录一起删掉

useradd 注：添加用户
passwd 注：为用户设置密码
usermod 注：修改用户命令，可以通过usermod 来修改登录名、用户的家目录等等；
id 注：查看用户的UID、GID及所归属的用户组
groupadd 注：添加用户组；
groupdel 注：删除用户组；
groupmod 注：修改用户组信息
groups 注：显示用户所属的用户组


#### 查询永续信息

    <!-- peak adm dialout cdrom sudo dip plugdev netdev lpadmin -->

    cat /etc/passwd 查看Linux下所有用户
        root:x:0:0:root:/root:/bin/bash
        xxx:x:1001:0::/home/xxx:/bin/bash
        ...
        username:password:uid:gid:allname:homedir:shell

        uid是0，就表示是超级管理员

    cat /etc/shadow 查看用户的密码加密内容

    cat /etc/group 查看用户的组信息:
        root:x:0:
        daemon:x:1:
        bin:x:2:
        sys:x:3:
        adm:x:4:syslog,peak
        ...
        groupname:password:gid:members

### fatal error: boost/shared_ptr.hpp: 没有那个文件或目录

	sudo apt-get install --no-install-recommends libboost-all-dev

### 架构

    dpkg --print-architecture， 显示系统架构
    dpkg --print-foreign-architectures， 显示其它系统架构

    dpkg --print-foreign-architectures | awk '{print $1}', 显示第一列

    sudo apt-get remove --purge `dpkg --get-selections | grep arm64 | awk '{print $1}'`
    sudo dpkg --remove-architecture arm64

    如果移除架构不成功:
    sudo apt-get remove .*:arm64
    sudo dpkg --remove-architecture arm64

    ## 移除i386
    dpkg -l | grep i386 | awk '{print $2}' | xargs sudo apt-get purge -y
    sudo dpkg --remove-architecture i386

### 查询相关

    readlink -f .,获取路径
    dirname .,获取目录名
    basename .,获取文件名

    pkg-config --modversion opencv 查看opencv是否安装

    sudo fdisk -l /dev/mmcblk1p1 查看设备信息
    df -lh  查看当前挂在的文件系统
    cat /proc/mounts 查看已挂在的文件系统详情
    lsblk 查看块设备信息

### 挂载

    fuser -km /media/peak/3930-38331 查看并杀死正在使用的指定挂载文件系统
    umount /media/peak/3930-38331 卸载文件系统
    mount -t vfat /dev/mmcblk1p1 ~/sdisk
    sudo mount -o remount,rw,dir_mode=0777,file_mode=0777 /dev/mmcblk1p1 ~/sdisk 重新挂载
    mount -t vfat -o uid=1002,gid=1002 /dev/mmcblk1p1 ~/sdisk
    sudo vim /etc/fstab

    UUID=024C92894C92775F   /win/d   ntfs-3g   locale=zh_CN.UTF-8,uid=peak,gid=peak   0   0

    对于挂载不同格式的磁盘，要注意options是不同的，比如说：ext4没有uid,gid,umask=0002

    options: 有权限前提下: rw,可读写, exec,可执行

    查看挂在的磁盘UUID:
    ls -all /dev/disk/by-uuid

    result=$(df -l | grep /dev/mmcblk1p1)
    if test -n "$result"; then
        result=$(echo "$result" | awk '{print $6}')
        fuser -km $result
        umount $result
    fi

### umask

    处理默认权限，shell中执行umask命令,查看当前用户的umask:0002
    创建目录时, 目录的默认权限是:7-0,7-0,7-2 即775
    创建文件时，默认不具有执行权限，则默认权限是:6-0,6-0,6-2 即664

### 查看usb设备的具体信息

    插拔USB， 查看usb信息：
        dmesg -w

    编写设备规则
        sudo vim /etc/udev/rules.d/myusb.rules

        添加：
            ATTRS{idVendor}=="0d28", ATTRS{idProduct}=="0204", MODE:="0666", SYMLINK+="esp32c3"

        MODE:="0666" 设置每个人都有读写权限

        sudo service udev reload
        sudo service udev restart

        再插拔一次 USB

### linux中的常见脚本

    /etc/fstab
    /etc/init.d/rcS
    /etc/rc.d/rc.local

### ping的原理是什么

### Show the status of modules in the Linux Kernel

    lsmod

### 查看已安装的驱动

    cat /lib/modules/$(uname -r)/modules.builtin

### wget

    -P  下载到指定目录

### 添加自启动服务

    sudo chmod 755 new_service.sh
    sudo update-rc.d new_service.sh defaults 90
    sudo update-rc.d -f new_service.sh remove

    sudo systemctl daemon-reload
    systemctl status RobotBringup.service

### 查看网络端口占用的两种方法, 第二种最为详细,相关的打开都会显示

    ss -nl | grep 10086
    lsof -i:10086,  list open files

### 设置gnome应用的自动启动属性, .desktop文件位于~/.autostart目录中

    X-GNOME-Autostart-enabled=true

### ubuntu17 设置关闭,最大化,最小化按钮

    左边:
    gsettings set org.gnome.desktop.wm.preferences button-layout 'close,maximize,minimize:'
    右边：
    gsettings set org.gnome.desktop.wm.preferences button-layout ':close,maximize,minimize'

### 时间命令

    date +%Y-%m-%d.%H:%M:%S
    输出:2018-02-17.23:35:45
    设置时间:
    date -s "2018-04-17 8:00:00"

### 查看声卡设备

	aplay -l    list all soundcards and digital audio devices
    aplay -L    list device names
    cat /proc/asound/cards

### 查看可执行程序的链接信息

    readelf -a programName

    -d, 可显示目标程序的动态库
    readelf -d audio_proc_test

### 查看所有的总线设备

    lspci

### 显示系统的版本信息, -s, short 简写, 只显示值

    lsb_release -a

### 显示所有的块设备

    lsblk

### updates and queries runlevel information for system services 更新或请求系统服务的运行级别的信息

    chkconfig --list

### 让httpd 在机器启动的时候在运行级别上启动

    chkconfig --level 345 httpd on
<!--stackedit_data:
eyJoaXN0b3J5IjpbMTc2NDcwMTI1NV19
-->

## apt proxy

    sudo vim /etc/apt/apt.conf
    添加(带有分号):
        Acquire::http::proxy "http://127.0.0.1:8123";
        Acquire::https::proxy "https://127.0.0.1:8123";

    或者 安装软件时:
        sudo apt -o Acquire::http::proxy="http://127.0.0.1:1080" install xxx

### fdisk

    p、打印分区表。
    n、新建一个新分区。
    d、删除一个分区。
    q、退出不保存。
    w、把分区写进分区表，保存并退出。

    U盘分三个区，1，cento live 2，
    sudo mkfs.ext4 /dev/sdc1
    sudo dd if=CentOS-7-x86_64-LiveGNOME-1708.iso of=/dev/sdc1 bs=16M

    # update part
    sudo partprobe /dev/sdc

    1  umount /dev/sdc1
    2  umount /dev/sdc2
    3  mount   /dev/sdc1  /tmp/boot
    4  mkdir /tmp/boot
    5  mount   /dev/sdc1  /tmp/boot
    6  grub-install --root-directory=/tmp/boot --no-floppy /dev/sdc
    7  ll /tmp/boot/
    8  grub-install --root-directory=/tmp/boot --no-floppy /dev/sdc
    9  umount /dev/sdc1
    10  lsblk
    11  ls
    12  fdisk /dev/sdc
    13  partprobe
    14  mkfs.vfat -F 32 -n MULTIBOOT /dev/sdc1
    15  umount /dev/sdc1
    16  mkfs.vfat -F 32 -n MULTIBOOT /dev/sdc1
    17  ll /media/
    18  lsblk
    19  grub-install --force --no-floppy --root-directory=/media/mx/MULTIBOOT /dev/sdc1
    20  cd /media/mx/MULTIBOOT/
    21  ls
    22  cd boot/grub/
    23  wget http://pendrivelinux.com/downloads/multibootlinux/grub.cfg
    24  ls
    25  cat grub.cfg
    26  history

    # mbr转gpt之前先执行下面,否则: grub-install: warning: Attempting to install GRUB to a disk with multiple partition labels.  This is not supported yet..

    fdisk 创建的分区是从2048开始，所以，如果使用fdisk创建bootable分区的话，还需要删除1-2047个block的内容:
    sudo dd if=/dev/zero of=/dev/sdc seek=1 count=2047

    # mbr转gpt
    sudo parted /dev/sdc
    mklabel gpt
    # 显示分区情况
    print
    q

    U盘引导分区制作,U盘多系统

    参考: https://wiki.archlinux.org/index.php/GRUB_(%E7%AE%80%E4%BD%93%E4%B8%AD%E6%96%87)

    我的U盘是32G的

    使用parted命令把U盘mbr格式转换成gpt格式,使用gdisk对gpt格式操作

    1, 创建分区

    sudo gdisk /dev/sdc

    1) BIOS boot分区，类型：ef02(bios_grub), 1MB空间

        Command (? for help)：n

        Partition number (1-128, default 1): Enter

        First sector：Enter

        Last sector: +1M

        Hex code or GUID: L

        <!-- 找到BIOS boot partition(ef02) -->
        Hex code or GUID: ef02

        Command (? for help)：p

        Number  Start (sector)    End (sector)  Size       Code  Name
        2048            4095   1024.0 KiB  EF02  BIOS boot partition

    2) 创建ESP分区(即EFI System Partition, EFI系统分区), +512M

        Command (? for help)：n

        Partition number (2-128, default 2): Enter

        First sector：Enter

        Last sector: +8G

        Hex code or GUID: Enter

    3) 创建一个适合大小合适格式的分区,fat32,ext4都可以,就是一个数据分区了,放系统镜像,或者是个人数据:

        Command (? for help)：n

        Partition number (3-128, default 3): Enter

        First sector：Enter

        Last sector: +8G

        Hex code or GUID: Enter

    这时:

        Command (? for help)：p

        Number  Start (sector)    End (sector)  Size       Code  Name
        1            2048            4095   1024.0 KiB  EF02  BIOS boot partition
        2            4096         1052671   512.0 MiB   8300  Linux filesystem
        3         1052672        17829887   8.0 GiB     8300  Linux filesystem


        <!-- 保存,退出 -->
        w
        q

        <!-- 通知系统更新分区表信息 -->
        sudo partprobe

        <!-- 格式化文件系统: -->

        sudo mkfs.vfat /dev/sdc2

        <!-- /dev/sdc3: fat32,ext4都可以 -->
        sudo mkfs.vfat /dev/sdc3

    2, 使用grub-install命令,生成开机引导程序至指定分区(/dev/sdc2)

        # grub相关程序
        sudo apt install -y grub-pc grub-efi-amd64

        <!-- 前面两步创建的是两个分区: /dev/sdc1, /dev/sdc2, 可以执行命令查看:lsblk /dev/sdc -->
        <!-- 指定权限是便于操作,无需sudo等操作 -->
        sudo mount -o uid=$USER,gid=$USER /dev/sdc2 /boot
        <!-- sudo mount -o uid=$USER,gid=$USER /dev/sdc3 /mnt -->

        sudo grub-install --target=i386-pc --boot-directory=/mnt/dev/boot --efi-directory=/mnt/dev/boot --bootloader-id=grub /dev/sdc

        "No suitable mode found" error:
        sudo cp /usr/share/grub/unicode.pf2 /boot/grub


    sudo mount -o uid=$USER,gid=$USER /dev/sdc2 /mnt/dev
    sudo grub-install --target=i386-pc --boot-directory=/mnt/dev/boot --efi-directory=/mnt/dev/boot --bootloader-id=grub /dev/sdc

    cp /usr/share/grub/{unicode.pf2,ascii.pf2} /mnt/dev/boot/grub

    <!-- sudo grub-install --target=i386-pc --no-floppy --boot-directory=/mnt/dev/boot /dev/sdc -->

## mbr转gpt

    sudo parted /dev/sdc----->mklabel gpt

## linux查看设备命令

    # uname -a               # 查看内核/操作系统/CPU信息
    # head -n 1 /etc/issue   # 查看操作系统版本
    # cat /proc/cpuinfo      # 查看CPU信息
    # hostname               # 查看计算机名
    # lspci -tv              # 列出所有PCI设备
    # lsusb -tv              # 列出所有USB设备
    # lsmod                  # 列出加载的内核模块
    # env                    # 查看环境变量
    cat /proc/asound/cards 查看声卡设备
    资源

    # free -m                # 查看内存使用量和交换区使用量
    # df -h                  # 查看各分区使用情况
    # du -sh <目录名>        # 查看指定目录的大小
    # grep MemTotal /proc/meminfo   # 查看内存总量
    # grep MemFree /proc/meminfo    # 查看空闲内存量
    # uptime                 # 查看系统运行时间、用户数、负载
    # cat /proc/loadavg      # 查看系统负载
    磁盘和分区

    # mount | column -t      # 查看挂接的分区状态
    # fdisk -l               # 查看所有分区
    # swapon -s              # 查看所有交换分区
    # hdparm -i /dev/hda     # 查看磁盘参数(仅适用于IDE设备)
    # dmesg | grep IDE       # 查看启动时IDE设备检测状况
    网络

    # ifconfig               # 查看所有网络接口的属性
    # iptables -L            # 查看防火墙设置
    # route -n               # 查看路由表
    # netstat -lntp          # 查看所有监听端口
    # netstat -antp          # 查看所有已经建立的连接
    # netstat -s             # 查看网络统计信息
    进程

    # ps -ef                 # 查看所有进程
    # top                    # 实时显示进程状态
    用户

    # w                      # 查看活动用户
    # id <用户名>            # 查看指定用户信息
    # last                   # 查看用户登录日志
    # cut -d: -f1 /etc/passwd   # 查看系统所有用户
    # cut -d: -f1 /etc/group    # 查看系统所有组
    # crontab -l             # 查看当前用户的计划任务
    服务

    # chkconfig --list       # 列出所有系统服务
    # chkconfig --list | grep on    # 列出所有启动的系统服务
    程序

    # rpm -qa                # 查看所有安装的软件包

## 常用命令整理如下

    查看主板的序列号: dmidecode | grep -i ’serial number’

    用硬件检测程序kuduz探测新硬件：service kudzu start ( or restart)

    查看CPU信息：cat /proc/cpuinfo [dmesg | grep -i 'cpu'][dmidecode -t processor]

    查看内存信息：cat /proc/meminfo [free -m][vmstat]

    查看板卡信息：cat /proc/pci

    查看显卡/声卡信息:
        lspci | grep -i 'VGA'
        dmesg | grep -i 'VGA'

    查看网卡信息：dmesg | grep -i ‘eth’[cat /etc/sysconfig/hwconf | grep -i eth][lspci | grep -i 'eth']
    <!--more-->
    查看PCI信息：lspci (相比cat /proc/pci更直观）

    查看USB设备：cat /proc/bus/usb/devices

    查看键盘和鼠标:cat /proc/bus/input/devices

    查看系统硬盘信息和使用情况：fdisk & disk – l & df

    查看各设备的中断请求(IRQ):cat /proc/interrupts

    查看系统体系结构：uname -a

    查看及启动系统的32位或64位内核模式：isalist –v [isainfo –v][isainfo –b]

    dmidecode查看硬件信息，包括bios、cpu、内存等信息

    测定当前的显示器刷新频率：/usr/sbin/ffbconfig –rev \?

    查看系统配置：/usr/platform/sun4u/sbin/prtdiag –v

    查看当前系统中已经应用的补丁：showrev –p

    显示当前的运行级别：who –rH

    查看当前的bind版本信息：nslookup –class=chaos –q=txt version.bind

    dmesg | more 查看硬件信息
    lspci 显示外设信息, 如usb，网卡等信息
    lsnod 查看已加载的驱动
    lshw
    psrinfo -v 查看当前处理器的类型和速度（主频）
    prtconf -v 打印当前的OBP版本号
    iostat –E 查看硬盘物理信息(vendor, RPM, Capacity)
    prtvtoc /dev/rdsk/c0t0d0s 查看磁盘的几何参数和分区信息
    df –F ufs –o i 显示已经使用和未使用的i-node数目
    isalist –v

    对于"/proc"中文件可使用文件查看命令浏览其内容，文件中包含系统特定信息：
    Cpuinfo 主机CPU信息
    Dma 主机DMA通道信息
    Filesystems 文件系统信息
    Interrupts 主机中断信息
    Ioprots 主机I/O端口号信息
    Meninfo 主机内存信息
    Version Linux内存版本信息

    备注： proc – process information pseudo-filesystem 进程信息伪装文件系统

    修改系统时间:
    sudo date +'%Y%m%d %H%M' --set='20180412 1049'

### synclient触摸板设置

    synclient -l 查看所有设置

    https://wiki.archlinux.org/index.php/Touchpad_Synaptics_(%E7%AE%80%E4%BD%93%E4%B8%AD%E6%96%87)

## 查询系统安装的gcc版本

    rpm -q gcc

    cat /lib/modules/$(uname -r)/modules.builtin

    cat /etc/filesystems

    df -h 查看磁盘用量

    mount -o 这里的options,查找方式, 比如搜索: xfs File Formats Manual

    grub2-set-default 0 && init 6

## u盘制作grub2引导

    为了避免出错:grub2-install: error: will not proceed with blocklists.先执行:
    sudo dd if=/dev/zero of=/dev/sdc seek=1 count=2047

    sudo umount /dev/sdc*
    sudo fdisk /dev/sdc

    <!-- 删除所有分区,创键一个/dev/sdc1 -->

    sudo mount /dev/sdc1 /tmp/pe
    sudo grub-install --root-directory=/tmp/pe --efi-directory=/tmp/pe/boot /dev/sdc

    出现这个错：
    解决:

    再重新: sudo grub2-install ...

    sudo grub2-mkconfig -o /tmp/pe/boot/grub2/grub.cfg

    yum install qemu-system-x86
    qemu-system-x86_64 -machine accel=kvm:tcg -m 2048 -hda /dev/sdc

    yum install ntfs-3g
    grub2-mkconfig -o /boot/grub2/grub.cfg
    会有如下信息:
    Generating grub configuration file ...
    Found linux image: /boot/vmlinuz-4.16.0-1.el7.elrepo.x86_64
    Found initrd image: /boot/initramfs-4.16.0-1.el7.elrepo.x86_64.img
    Found linux image: /boot/vmlinuz-3.10.0-693.21.1.el7.x86_64
    Found initrd image: /boot/initramfs-3.10.0-693.21.1.el7.x86_64.img
    Found linux image: /boot/vmlinuz-3.10.0-693.el7.x86_64
    Found initrd image: /boot/initramfs-3.10.0-693.el7.x86_64.img
    Found linux image: /boot/vmlinuz-0-rescue-d7f616c52b4d4d7196862f3fc93f2cf7
    Found initrd image: /boot/initramfs-0-rescue-d7f616c52b4d4d7196862f3fc93f2cf7.img
    Found Windows 10 (loader) on /dev/sdb1
    done
    最后第二行，就是配置windows10的引导项.

    ls *.ko | xargs -I {} cp {} /lib/modules

    cat test.txt | sed '2d' 删除第2行
    cat test.txt | sed '2d;3d' 删除第2行和第3行
    cat test.txt | sed '2,4d' 删除第2~4行
    cat test.txt | sed '2,$d' 删除第2~最后1行
    cat test.txt | sed -n '2p' 只要第2行

    $(test)， 表示执行test命令并接收返回值
    ${test}，便是test这个变量的内容

## 查看本地化设置

    localectl

## 设置本地化参数

    sudo localectl set-locale LANG=zh_CN.UTF-8
    sudo localectl set-keymap zh_CN

    sudo vim /var/lib/locales/supported.d/local
    zh_CN.GBK GBK
    zh_CN.GB2312 GB2312
    zh_CN.GB18030 GB18030

    使其生效：
    sudo dpkg-reconfigure locales

## unzip乱码问题

    UNZIP="-O CP936"
    ZIPINFO="-O CP936"

    unzip -O CP936 fileName

## 显示当前主机的信息

    hostnamectl

## 设置主机名

    sudo hostnamectl set-hostname rhel7

## 列出当前session

    $ loginctl list-sessions

## 列出当前登录用户

    $ loginctl list-users

## 列出显示指定用户的信息

    $ loginctl show-user ruanyf

## 查看启动耗时

    $ systemd-analyze

## 查看每个服务的启动耗时

    $ systemd-analyze blame

## 显示瀑布状的启动过程流

    $ systemd-analyze critical-chain

## 显示指定服务的启动流

    $ systemd-analyze critical-chain atd.service

## 给当前用户追加dialout用户组

    sudo usermod -a -G dialout,sudo,adm $USER

## 重命名

    rename 原字符串 目标字符串 文件
    rename main1.c main.c main1.c: 把main1.c替换成main.c
    rename "s/笔记-//" *: 去掉文件名中的'笔记-'

## 无法获得锁 /var/lib/apt/lists/lock

    sudo rm /var/lib/apt/lists/lock
    sudo rm /var/cache/apt/archives/lock

## 认证失败

	sudo adduser "$USER" netdev

## dd命令创建虚拟设备文件

    1. 生成一个device_file.img文件,大小: 1k byte * 1000 000 = 1 Gb
    dd if=/dev/zero of=device_file.img bs=1k count=1000000

    2. 格式化文件
    mkfs.ext4 device_file.img

    3. 挂载文件
    sudo mount device_file.img /mnt

    然后通过lsblk,即可查看到该设备:
    loop0    7:0    0 976.6M  0 loop /mnt

## 显示系统中的中文字体

    fc-list :lang=zh

## 解决ubuntu使用命令sudo apt -get install 安装东西时出现"E: Sub-process /usr/bin/dpkg returned an error code (1) "的错误

    sudo mv /var/lib/dpkg/info /var/lib/dpkg/info.back
    sudo mkdir /var/lib/dpkg/info

## 查看进程状态

	lsof -p 21943
	sudo ls -l /proc/21943/fd

## update-locale example

    update-locale LANG=en_CA.UTF-8 LANGUAGE
    # sets LANG to en_CA.UTF-8 and removes definitions for LANGUAGE.

## .ssh/config 给git配置proxy

	Host github.com
		HostName github.com
		User git
		ProxyCommand nc -v -x 127.0.0.1:1080 %h %p

## install boot-repair

    # 添加软件源并更新
    sudo add-apt-repository ppa:yannubuntu/boot-repair &&　apt-get update
    ＃　安装boot-repair并启动软件
    sudo apt install -y boot-repair && boot-repair

## pip 升级问题

    Traceback (most recent call last):
    File "/usr/bin/pip3", line 9, in <module>
        from pip import main
    ImportError: cannot import name 'main'

    curl https://bootstrap.pypa.io/get-pip.py | python3 - --user

## pip setuptools 版本太低

    pkg_resources.VersionConflict: (setuptools 20.7.0 (/usr/lib/python3/dist-packages), Requirement.parse('setuptools>=40.0'))
    pip3 install -U --user setuptools

## 更改硬件时间

    timedatectl set-local-rtc 1 --adjust-system-clock

## manjora 笔记本合盖不关屏幕

    sudo vim /etc/UPower/UPower.conf

    修改 IgnoreLid=false 为 IgnoreLid=true

    sudo systemctl restart upower.service

## apt-cacher

	sudo apt install apt-cacher

	使用默认的 daemon 方式

	/etc/apt-cacher/apt-cacher.conf:
		allowed_hosts = *

	systemctl status apt-cacher.service
	sudo lsof -i:3142
	ss -tl | grep 3142

	sudo systemctl enable --now apt-cacher


##  防火墙

	sudo ufw status
	sudo ufw enable

	设置默认策略:
		sudo ufw default allow outgoing
		sudo ufw default deny incoming

	允许访问:

		开放 ssh 的端口:
			sudo ufw allow ssh

		允许 tcp/2222 端口:
			sudo ufw allow 2222/tcp

		允许特定的端口:
			sudo ufw allow 80/tcp comment 'accept Apache'
			sudo ufw allow 443/tcp comment 'accept HTTPS connections'

		允许 UDP/1194 (OpenVPN) server:
			sudo ufw allow 1194/udp comment 'OpenVPN server'

		允许端口范围:
			sudo ufw allow 3000:4000/tcp
			sudo ufw allow 3000:4000/udp

		允许ip:
			sudo ufw allow from 104.22.10.214

		允许 ip为104.22.11.213 端口为25 的 tcp协议:
			sudo ufw allow from 104.22.11.213 to any port 25 proto tcp

		允许 目的地为 222.222.222.222:
			sudo ufw allow from 104.22.11.213 to 222.222.222.222 port 25 proto tcp

	拒绝访问:

		sudo ufw deny from 203.5.1.43
		sudo ufw deny from 103.13.42.13/29
		sudo ufw deny from 1.1.1.2 to any port 22 proto tcp

	删除rule:
		sudo ufw status numbered
		sudo ufw delete 6
		sudo ufw status numbered

	其它:
		sudo ufw reset
		sudo ufw reload
		sudo more /var/log/ufw.log
		sudo tail -f /var/log/ufw.log
		sudo ufw show listening
		sudo ufw show added


## 安装 squid 代理客户端

	sudo apt install squid

	systemctl status squid


## 切换 display manager

    先查看当前的dm是:
        systemctl status display-manager.service

    Main PID: 1018 (sddm) or Main PID: 1018 (gdm3)

    重置:
        sudo dpkg-reconfigure gdm3


## ubuntu20 使用 deepin 桌面

    sudo add-apt-repository ppa:ubuntudde-dev/stable
    sudo apt install ubuntudde-dde

    选择 display manager 为 gdm3 或者 执行:
        systemctl status display-manager.service, 查看当前的显示管理器为:  Main PID: 1758 (sddm)
        sudo dpkg-reconfigure sddm, 选择 gdm3

## ldd, list dynamic dependencyies, 列出动态依赖, 动态库

    比如:
        xxx@xxx-pc:/develop/sources/stlink/build$ ldd /usr/local/bin/st-info
        linux-vdso.so.1 (0x00007ffe16f92000)
        libstlink.so.1 => not found
        libusb-1.0.so.0 => /lib/x86_64-linux-gnu/libusb-1.0.so.0 (0x00007f469507d000)
        libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f4694ebc000)
        libudev.so.1 => /lib/x86_64-linux-gnu/libudev.so.1 (0x00007f4694e96000)
        libpthread.so.0 => /lib/x86_64-linux-gnu/libpthread.so.0 (0x00007f4694e75000)
        /lib64/ld-linux-x86-64.so.2 (0x00007f46950be000)
        librt.so.1 => /lib/x86_64-linux-gnu/librt.so.1 (0x00007f4694e6b000)
