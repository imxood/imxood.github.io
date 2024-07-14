#!/bin/bash

# gparted

# BLANK 1MB
# swap 4GB
# root 20GB
# usr 10GB
# opt 10GB
# tmp 5GB
# src 30GB
# home 10GB

mkdir -pv $LFS
mount -v -t ext4 /dev/sda3 $LFS

# mkswap /dev/sda3
mkdir -v $LFS/usr
mount -v -t ext4 /dev/sda4 $LFS/usr

mkdir -v $LFS/sources
chmod -v a+wt $LFS/sources

tar -xvf /home/mx/Downloads/software/linux/lfs-packages-8.4.tar -C $LFS/sources

# http://www.linuxfromscratch.org/lfs/downloads/stable/wget-list
# wget --input-file=wget-list --continue --directory-prefix=$LFS/sources
# aria2c -d $LFS/sources http://www.linuxfromscratch.org/lfs/downloads/stable/wget-list
# aria2c -d $LFS/sources -c -x 5 --input-file=http://www.linuxfromscratch.org/lfs/downloads/stable/wget-list

aria2c -d $LFS/sources http://www.linuxfromscratch.org/lfs/downloads/stable/md5sums

pushd $LFS/sources
md5sum -c md5sums
popd

mkdir -v $LFS/tools
ln -sv $LFS/tools /

if [ -z "$(cat /etc/passwd | grep lfs)" ]; then
  groupadd lfs
  useradd -s /bin/bash -g lfs -m -k /dev/null lfs
  passwd lfs
fi

chown -v lfs $LFS/tools
chown -v lfs $LFS/sources

su - lfs
cat > ~/.bash_profile << "EOF"
exec env -i HOME=$HOME TERM=$TERM PS1='\u:\w\$ ' /bin/bash
EOF

cat > ~/.bashrc << "EOF"
set +h
umask 022
LFS=/mnt/lfs
LC_ALL=POSIX
LFS_TGT=$(uname -m)-lfs-linux-gnu
PATH=/tools/bin:/bin:/usr/bin
export LFS LC_ALL LFS_TGT PATH
EOF





./build.1.sh








# 6.2. Preparing Virtual Kernel File Systems

mkdir -pv $LFS/{dev,proc,sys,run}

# 6.2.1. Creating Initial Device Nodes

mknod -m 600 $LFS/dev/console c 5 1
mknod -m 666 $LFS/dev/null c 1 3

# 6.2.2. Mounting and Populating /dev

mount -v --bind /dev $LFS/dev

# 6.2.3. Mounting Virtual Kernel File Systems

mount -vt devpts devpts $LFS/dev/pts -o gid=5,mode=620
mount -vt proc proc $LFS/proc
mount -vt sysfs sysfs $LFS/sys
mount -vt tmpfs tmpfs $LFS/run


if [ -h $LFS/dev/shm ]; then
    mkdir -pv $LFS/$(readlink $LFS/dev/shm)
fi


# 6.4. Entering the Chroot Environment

chroot "$LFS" /tools/bin/env -i \
    HOME=/root                  \
    TERM="$TERM"                \
    PS1='(lfs chroot) \u:\w\$ ' \
    PATH=/bin:/usr/bin:/sbin:/usr/sbin:/tools/bin \
    /tools/bin/bash --login +h

# 6.5. Creating Directories

mkdir -pv /{bin,boot,etc/{opt,sysconfig},home,lib/firmware,mnt,opt}
mkdir -pv /{media/{floppy,cdrom},sbin,srv,var}
install -dv -m 0750 /root
install -dv -m 1777 /tmp /var/tmp
mkdir -pv /usr/{,local/}{bin,include,lib,sbin,src}
mkdir -pv /usr/{,local/}share/{color,dict,doc,info,locale,man}
mkdir -v  /usr/{,local/}share/{misc,terminfo,zoneinfo}
mkdir -v  /usr/libexec
mkdir -pv /usr/{,local/}share/man/man{1..8}

case $(uname -m) in
    x86_64) mkdir -v /lib64 ;;
esac

mkdir -v /var/{log,mail,spool}
ln -sv /run /var/run
ln -sv /run/lock /var/lock
mkdir -pv /var/{opt,cache,lib/{color,misc,locate},local}

ln -sv /tools/bin/{bash,cat,chmod,dd,echo,ln,mkdir,pwd,rm,stty,touch} /bin
ln -sv /tools/bin/{env,install,perl,printf}         /usr/bin
ln -sv /tools/lib/libgcc_s.so{,.1}                  /usr/lib
ln -sv /tools/lib/libstdc++.{a,so{,.6}}             /usr/lib

install -vdm755 /usr/lib/pkgconfig

ln -sv bash /bin/sh

ln -sv /proc/self/mounts /etc/mtab

cat > /etc/passwd << "EOF"
root:x:0:0:root:/root:/bin/bash
bin:x:1:1:bin:/dev/null:/bin/false
daemon:x:6:6:Daemon User:/dev/null:/bin/false
messagebus:x:18:18:D-Bus Message Daemon User:/var/run/dbus:/bin/false
nobody:x:99:99:Unprivileged User:/dev/null:/bin/false
EOF

cat > /etc/group << "EOF"
root:x:0:
bin:x:1:daemon
sys:x:2:
kmem:x:3:
tape:x:4:
tty:x:5:
daemon:x:6:
floppy:x:7:
disk:x:8:
lp:x:9:
dialout:x:10:
audio:x:11:
video:x:12:
utmp:x:13:
usb:x:14:
cdrom:x:15:
adm:x:16:
messagebus:x:18:
input:x:24:
mail:x:34:
kvm:x:61:
wheel:x:97:
nogroup:x:99:
users:x:999:
EOF


# relogin
exec /tools/bin/bash --login +h

# The /var/log/wtmp file records all logins and logouts.
# The /var/log/lastlog file records when each user last logged in.
# The /var/log/faillog file records failed login attempts.
# The /var/log/btmp file records the bad login attempts.
touch /var/log/{btmp,lastlog,faillog,wtmp}
chgrp -v utmp /var/log/lastlog
chmod -v 664  /var/log/lastlog
chmod -v 600  /var/log/btmp


./build.2.sh
