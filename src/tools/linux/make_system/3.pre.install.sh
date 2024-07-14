#!/bin/bash

set -e -x

export LFS=/mnt/lfs

mkdir -pv $LFS/sources

# 可写和粘滞(sticky)权限
chmod -v a+wt $LFS/sources

# LFS Packages Sources
aria2c -c -x 5 -s 5 http://ftp.lfs-matrix.net/pub/lfs/lfs-packages/lfs-packages-8.0.tar -d $LFS/sources
tar -xvf $LFS/sources/lfs-packages-8.0.tar --strip-components=1 -C $LFS/sources

# LFS BOOK
mkdir -pv $LFS/LFSbooks
svn co svn://svn.linuxfromscratch.org/LFS/tags/8.0 $LFS/LFSbooks

vim $LFS/LFSbooks/packages.ent
# line364: "7ee67b16b345b43cd34ed481792169ed" --> "3cb3d36089f2d6ab19c7c0f3282de1c5"

# JHALFS
aria2c -c -x 5 -s 5 https://github.com/ojab/jhalfs/archive/2.4.tar.gz -d $LFS/sources
tar -xvf $LFS/sources/jhalfs-2.4.tar.gz -C $LFS/sources


mkdir -pv $LFS/LFSBuild

# add user 'jhalfs'
groupadd jhalfs
useradd -s /bin/bash -g jhalfs -m -k /dev/null jhalfs
passwd jhalfs

sh -c 'echo "jhalfs ALL=(ALL) ALL" >> /etc/sudoers'


cd $LFS/sources/jhalfs-2.4

make

# Book Settings
#     --> Release(SVN): Work Copy
#     --> Loc of working copy(mandatory): /mnt/lfs/LFSbooks

# General Settings
#     --> Build Directory: /mnt/lfs/LFSBuild
#     --> Retrive source files / Package Archive Directory: /mnt/lfs/sources
#     # 如果选中该选项,这样在编译前会清除 LFSBuild 文件夹,以节约空间
#     --> [*] Rebuild files

# Build Settings
#     --> [*] Create a log of installed files for each package
#     --> [*] Install Vim-lang package
#     --> TimeZone: Asia/Shanghai
#     --> [*] Install the full set of locales


su - jhalfs

cd /mnt/lfs/LFSBuild

make -C /mnt/lfs/LFSBuild/jhalfs



exit

cd $LFS/LFSBuild

cp -r bin $LFS
cp -r dev $LFS
cp -r etc $LFS
cp -r lib $LFS
cp -r lib64 $LFS
cp -r media $LFS
cp -r mnt $LFS
cp -r opt $LFS
cp -r proc $LFS
cp -r root $LFS
cp -r run $LFS
cp -r sbin $LFS
cp -r srv $LFS
cp -r sys $LFS
cp -r usr $LFS
cp -r var $LFS

# At this time, you can delete the LFSBuild directory
mount -v --bind /dev $LFS/dev
mount -vt devpts devpts $LFS/dev/pts -o gid=5,mode=620
mount -vt proc proc $LFS/proc
mount -vt sysfs sysfs $LFS/sys
mount -vt tmpfs tmpfs $LFS/run

# Enter chroot environgment
chroot "$LFS" /usr/bin/env -i \
HOME=/root TERM="$TERM" PS1='\u:\w\$ ' \
PATH=/bin:/usr/bin:/sbin:/usr/sbin \
/bin/bash --login


cat > /etc/fstab << "EOF"
# Begin /etc/fstab
# file system mount-point type options dump fsck
# order
UUID=6dd75e3f-2f57-4c4c-a5ee-43de6e1dc900   /           ext4        errors=remount-ro   1       1
UUID=337c5bb5-5710-4ada-a5e2-3bf917849f3e   swap        swap        pri=1               0       0
UUID=4c143075-899d-4193-9a53-d1158d8bbfba   /boot       ext4        defaults            1       1
UUID=2b446db0-4c7d-4e12-aee4-68fca20ac0b3   /usr/local  ext4        defaults            1       1
UUID=9f93fb5e-32ae-4bb5-97d5-e4fa45d26400   /tmp        ext4        defaults            1       1
UUID=e5e060a6-c17a-4ab0-97aa-dee456860072   /src        ext4        defaults            1       1
UUID=48ba8f05-2e70-4541-9a85-4c4a4d513590   /home       ext4        defaults            1       1
proc                                        /proc       proc        nosuid,noexec,nodev 0       0
sysfs                                       /sys        sysfs       nosuid,noexec,nodev 0       0
devpts                                      /dev/pts    devpts      gid=5,mode=620      0       0
tmpfs                                       /run        tmpfs       defaults            0       0
devtmpfs                                    /dev        devtmpfs    mode=0755,nosuid    0       0
# End /etc/fstab
EOF

cd sources
tar -xvf linux-4.9.9.tar.xz
cd linux-4.9.9
make mrproper
make menuconfig

Device Drivers
    --> Generic Driver Options
        --> [ ] Support for uevent helper
        --> [*] Maintain a devtmpfs filesystem to mount at /dev [CONFIG_DEVTMPFS]
    --> SCSI device support
        --> [*] SCSI low-level drivers
            --> [*] BusLogic SCSI support
            --> [*] VMware PVSCSI driver support
    --> Fusion MPT device support
        [*] Fusion MPT ScsiHost drivers for SPI
        [*] Fusion MPT ScsiHost drivers for SAS

Processor type and features
    --> [*] EFI stub support


make -j12
make modules_install
cp -v arch/x86/boot/bzImage /boot/vmlinuz-4.9.9-lfs-8.0
cp -v System.map /boot/System.map-4.9.9
cp -v .config /boot/config-4.9.9
install -d /usr/share/doc/linux-4.9.9
cp -r Documentation/* /usr/share/doc/linux-4.9.9
cd .. && rm -rfv linux-4.9.9

# Configuring Linux Module Load Order
install -v -m755 -d /etc/modprobe.d
cat > /etc/modprobe.d/usb.conf << "EOF"
# Begin /etc/modprobe.d/usb.conf
install ohci_hcd /sbin/modprobe ehci_hcd ; /sbin/modprobe -i ohci_hcd ; true
install uhci_hcd /sbin/modprobe ehci_hcd ; /sbin/modprobe -i uhci_hcd ; true
# End /etc/modprobe.d/usb.conf
EOF

# Using GRUB to Set Up the Boot Process
grub-install --target=i386-pc /dev/sdc

cat > /boot/grub/grub.cfg << "EOF"
# Begin /boot/grub/grub.cfg
set default=0
set timeout=50
insmod ext2
set root=(hd0,msdos3)
menuentry "GNU/Linux, Linux 4.9.9-lfs-8.0" {
linux /vmlinuz-4.9.9-lfs-8.0 root=/dev/sdc1 ro
}
EOF

echo "maxu-pc" > /etc/hostname


logout
umount -v $LFS/dev/pts
umount -v $LFS/dev
umount -v $LFS/run
umount -v $LFS/proc
umount -v $LFS/sys
umount -v $LFS/boot
umount -v $LFS/usr/local
umount -v $LFS/tmp
umount -v $LFS/usr/src
umount -v $LFS/home
umount -v $LFS
