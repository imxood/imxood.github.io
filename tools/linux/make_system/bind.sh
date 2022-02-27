#!/bin/bash

mkdir /mnt/chrootdir

mount /dev/nvme0n1p4 /mnt/chrootdir

for dir in proc dev sys etc bin sbin var usr lib lib64 tmp; do
    mkdir /mnt/chrootdir/$dir && mount --bind /$dir /mnt/chrootdir/$dir
done

