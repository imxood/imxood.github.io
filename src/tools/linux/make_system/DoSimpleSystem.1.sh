#!/bin/bash

# 安装必要的程序
sudo apt install -y grub-pc

# 安装编译linux的必要依赖
sudo apt install libncurses5-dev gcc make git exuberant-ctags bc libssl-dev grub-pc

# 创建一个空的磁盘文件
dd if=/dev/zero of=disk.img bs=1M count=1024

# 对磁盘文件分区
echo "
n
p



w
" | fdisk disk.img



sudo losetup --list， 寻找可用的loop设备

# 把磁盘文件关联到/dev/loop7设备上
sudo losetup -o 1048576 /dev/loop7 disk.img

# 格式化/dev/loop7为ext3格式
sudo mkfs.ext3 /dev/loop7

# 挂载/dev/loop7到mnt目录
mkdir -p mnt && sudo mount -t ext3 /dev/loop7 mnt

# 安装grub
sudo grub-install --boot-directory=./mnt/boot --target=i386-pc --modules=part_msdos disk.img

# hd0表示第一个硬盘, msdos1表示该硬盘的第一个分区
cat - > ./mnt/boot/grub/grub.cfg << EOF
menuentry "FreshLinux" {
    linux (hd0,msdos1)/boot/bzImage console=tty0
    initrd (hd0,msdos1)/boot/initrd
}
EOF

# 制作简单的initrd
sudo mkinitramfs -o ./mnt/boot/initrd

# 使用默认配置，编译linux内核
# https://mirrors.edge.kernel.org/pub/linux/kernel
cd linux-4.9.179
make x86_64_defconfig && make bzImage -j10
sudo cp arch/x86/boot/bzImage ../mnt/boot

# 卸载， 并取消磁盘文件的关联
sudo umount /dev/loop7
sudo losetup -d /dev/loop7

# 启动qemu
qemu-system-x86_64 -hda disk.img -boot d -m 1024 -enable-kvm
