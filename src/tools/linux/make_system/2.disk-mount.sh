#!/bin/bash

set -e -x

export LFS=/mnt/lfs

echo "LFS: $LFS"

# 挂载"/"文件系统
mkdir -pv $LFS
mkfs -v -t ext4 /dev/sdc1
mount -v -t ext4 /dev/sdc1 $LFS

# 内存交换分区
mkswap /dev/sdc2
swapon -v /dev/sdc2

# 挂载"boot"文件系统
mkdir -pv $LFS/boot
mkfs -v -t ext4 /dev/sdc3
mount -v -t ext4 /dev/sdc3 $LFS/boot

# 挂载"/usr/local"文件系统
mkdir -pv $LFS/usr/local
mkfs -v -t ext4 /dev/sdc5
mount -v -t ext4 /dev/sdc5 $LFS/usr/local

# 挂载"/tmp"文件系统
mkdir -pv $LFS/tmp
mkfs -v -t ext4 /dev/sdc6
mount -v -t ext4 /dev/sdc6 $LFS/tmp

# 挂载"/usr/src"文件系统
mkdir -pv $LFS/usr/src
mkfs -v -t ext4 /dev/sdc7
mount -v -t ext4 /dev/sdc7 $LFS/usr/src

# 挂载"/home"文件系统
mkdir -pv $LFS/home
mkfs -v -t ext4 /dev/sdc8
mount -v -t ext4 /dev/sdc8 $LFS/home




mount -v -t ext4 /dev/sdc1 $LFS
mount -v -t ext4 /dev/sdc3 $LFS/boot
mount -v -t ext4 /dev/sdc5 $LFS/usr/local
mount -v -t ext4 /dev/sdc6 $LFS/tmp
mount -v -t ext4 /dev/sdc7 $LFS/usr/src
mount -v -t ext4 /dev/sdc8 $LFS/home
