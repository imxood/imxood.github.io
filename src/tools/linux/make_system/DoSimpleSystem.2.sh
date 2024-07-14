#!/bin/bash

mkdir /mnt/chrootdir


mount /dev/sda1 /mnt/chrootdir


for dir in proc dev sys etc bin sbin var usr lib lib64 tmp; do


 mkdir /mnt/chrootdir/$dir && mount --bind /$dir /mnt/chrootdir/$dir


done


chroot /mnt/chrootdir


update-grub2 # inside chroot






# 引导程序
sudo apt install -y grub-efi

# 多线程下载工具
sudo apt install -y aria2

# ??
# sudo apt install -y grub efibootmgr

# 多分区的镜像文件挂载
sudo apt install -y kpartx

# 安装编译linux的必要依赖
sudo apt install libncurses5-dev gcc make git exuberant-ctags bc libssl-dev grub-pc

# 创建一个空的磁盘文件 10GB
dd if=/dev/zero of=disk.img bs=1M count=20480

# 对磁盘文件分区
# 1. esp(512M) 2. swap(4G) 3. /(10G) 4. /home(Other)
echo "
o
y
n


+512M
ef00
n


+10G

n


+5G

n



8200
p
w
y
q
" | gdisk disk.img

# 如果没有loopXpX, 即没有成功, a则查看是否设备已经使用
sudo kpartx -asv disk.img && ll /dev/mapper/loop*p*

sudo mkfs.fat -F 32 /dev/mapper/loop*p1
# mkfs.fat unable to get drive geometry, using default 255/63,,, Why ??

sudo mkfs.ext4 /dev/mapper/loop*p2
sudo mkfs.ext4 /dev/mapper/loop*p3

# 查看当前的分区情况
sudo parted disk.img print

# 挂载
mkdir -p mnt && sudo mount -v -t ext4 /dev/mapper/loop*p2 mnt

sudo mkdir -p mnt/boot && sudo mount /dev/mapper/loop*p1 mnt/boot
sudo mkdir -p mnt/home && sudo mount /dev/mapper/loop*p3 mnt/home


# 安装grub
sudo grub-install --target=x86_64-efi --efi-directory=mnt/boot --bootloader-id=grub --boot-directory=mnt/boot --debug
# 会在mnt/boot/EFI/中创建grub/grubx64.efi
# 并生成mnt/boot/grub(固定的grub目录名, 不是bootloader-id)

sudo grub-install --target=x86_64-efi --efi-directory=mnt/boot --bootloader-id=grub1 --boot-directory=mnt/boot --debug
# 扫描当前系统可启动区域创建grub.cfg
sudo grub-mkconfig -o mnt/boot/grub/grub.cfg

# 编写grub.cfg, 扫描当前系统可启动
# sudo mkdir -p mnt/boot/grub

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
# https://mirrors.edge.kernel.org/pub/linux/kernel/v4.x/linux-4.9.179.tar.gz
aria2c -s16 -x16 -j16 https://mirrors.edge.kernel.org/pub/linux/kernel/v4.x/linux-4.9.179.tar.gz
tar -xvf linux-4.9.179.tar.gz && cd linux-4.9.179
make -C linux-4.9.179 x86_64_defconfig && make -C linux-4.9.179 bzImage -j10
sudo cp linux-4.9.179/arch/x86/boot/bzImage mnt/boot

# 卸载， 并取消磁盘文件的关联
sudo umount /dev/mapper/loop*p3
sudo umount /dev/mapper/loop*p1
sudo umount /dev/mapper/loop*p2

sudo kpartx -d disk.img

# 启动qemu
qemu-system-x86_64 -hda disk.img -boot d -m 1024 -enable-kvm
