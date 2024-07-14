
# find your device
lsblk

# first make three partions: / /boot /home
# ...


# mount point: /mnt /mnt/boot /mnt/home
# ...

# pacman -S arch-install-scripts
sudo pacstrap /mnt base base-devel

# need to check the fstab's correctness
sudo genfstab -U /mnt | sudo tee /mnt/etc/fstab

sudo arch-chroot /mnt

# timedatectl set-timezone Asia/Shanghai
ls -sf /usr/share/zoneinfo/Asia/Shanghai /etc/localtime

# timedatectl set-ntp true
hwclock --systohc

cat - > /etc/pacman.conf << eof
[archlinuxcn]
SigLevel = Optional TrustAll
Server = https://mirrors.tuna.tsinghua.edu.cn/archlinuxcn/\$arch
eof

# ## China
# Server = http://mirrors.tuna.tsinghua.edu.cn/archlinux/$repo/os/$arch

nano /etc/pacman.d/mirrorlist

pacman -Sy archlinuxcn-keyring

pacman -S vim yay aria2c mirrorlist-rankmirrors-hook

wget -O /etc/pacman.d/mirrorlist https://www.archlinux.org/mirrorlist/all/
#或是
wget -O /etc/pacman.d/mirrorlist https://www.archlinux.org/mirrorlist/?country=CN

cp mirrorlist mirrorlist.backup

rankmirrors -n 0 mirrorlist.backup | sudo tee mirrorlist

pacman -Syy

XferCommand = /usr/bin/aria2c --allow-overwrite=true -c --file-allocation=none --log-level=info -m2 --max-connection-per-server=10 --max-file-not-found=5 --min-split-size=1M --no-conf --remote-time=true --summary-interval=10 -t5 -d / -o %o %u
#XferCommand = /usr/bin/curl -L -C - -f -o %o %u


pacman -S bash-completion
. /etc/bash.bashrc

# en_US.UTF-8 UTF-8
# zh_CN.UTF-8 UTF-8
vim /etc/locale.gen

locale-gen


# localectl set-locale LANG=en_US.UTF-8
/echo 'LANG=en_US.UTF-8' > /etc/locale.cfg

# hostnamectl set-hostname maxu-pc
echo 'YourPcName' > /etc/hostname

cat - > /etc/hosts << eof
127.0.0.1       localhost
::1             localhost
127.0.0.1       maxu-pc.localdomain maxu-pc
eof

pacman -S grub os-prober efibootmgr
grub-install --target=x86_64-efi --efi-directory=/boot/efi --boot-directory=/boot --bootloader-id=ArchLinux --debug

# grub-install --target=x86_64-efi --efi-directory=/mnt/boot --boot-directory=/mnt/boot --bootloader-id=ubuntu-kylin --debug

grub-mkconfig -o /boot/grub/grub.cfg
passwd

# add new user
useradd -m -g wheel -s /bin/bash UserName
passwd UserName

# search: wheel, add sudo authority to wheel group
visudo

# Chinese font
pacman -S ttf-dejavu ttf-liberation wqy-zenhei wqy-microhei

# install xorg-server
pacman -S xorg-server xorg-server-utils xorg-xinit

# video driver ? mhwd-nvidia
pacman -S xf86-video-intel nvidia

# input device
pacman -S xf86-input-libinput xf86-input-synaptics

# display manager
pacman -S lightdm lightdm-gtk-greeter
systemctl enable lightdm.service

# add greeter-session=lightdm-gtk-greeter
vim /etc/lightdm/lightdm.conf

# audio
pacman -S pulseaudio pulseaudio-alsa alsa-utils

# net
pacman -S net-tools networkmanager
systemctl enable NetworkManager.service

# wifi
pacman -S iw wpa_supplicant dialog wireless_tools

# uncompress
pacman -S p7zip file-roller unrar

# ntfs
pacman -S ntfs-3g dosfstools

# ap
pacman -S create_ap

# Color
vim /etc/pacman.conf

# desktop
pasman -S xfce4 xfce4-goodies

# icon theme
yay -S numix-circle-icon-theme-git

# gtk theme
yay -S gtk-theme-arc-git

# https://www.xfce-look.org

mkdir ~/.themes

# XFWM4 Themes
# Greybird-yosemite-v2.zip
# XFCE Settings --> Window Manager --> select "Greybird-yosemite"
mv Greybird-yosemite ~/.themes


# mouse wheel
# Open Window Manager Tweaks
#       --> Accessibility
#               --> untick "Use mouse wheel on title bar to roll up the window"
#       --> Workspace
#               --> untick "Use mouse wheel on the desktop to switch workspaces"


# brower
pacman -S google-chrome

# proxy
pacman -S v2ray
systemctl enable v2ray.service

# pinyin
pacman -S fcitx fcitx-configtool fcitx-googlepinyin

cat - > ~/.xprofile << eof
export LC_CTYPE=zh_CN.UTF-8
export XIM=fcitx  
export XIM_PROGRAM=fcitx
export GTK_IM_MODULE=fcitx
export QT_IM_MODULE=fcitx
export XMODIFIERS="@im=fcitx"
eof


# libreioffice
# sudo pacman -S libreoffice-still
sudo pacman -S libreoffice-fresh

# # kde desktop
# pacman -S plasma-desktop kdebase

# # theme
# pacman -S arc-kde

# # icon
# pacman -S papirus-icon-theme

# # image view tool
# pacman -S gwenview


# pacman -S xf86-video-vmware
# pacman -S open-vm-tools
# systemctl enable vmtoolsd.service

exit
