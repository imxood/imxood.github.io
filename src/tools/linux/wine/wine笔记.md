# wine笔记

wiki: https://wiki.archlinux.org/index.php/Wine_(%E7%AE%80%E4%BD%93%E4%B8%AD%E6%96%87)

## LinuxMint19 安装wine

    sudo dpkg --add-architecture i386
    wget -nc https://dl.winehq.org/wine-builds/winehq.key
    sudo apt-key add winehq.key
    sudo add-apt-repository 'deb https://dl.winehq.org/wine-builds/ubuntu/ focal main'

    sudo apt update
    sudo apt install --install-recommends winehq-devel

    # 记得执行这一行, 在命令行中执行, 可以使用代理, 有利于后面的下载
    winecfg

    # 系统配了代理, 安装速度还是很快的
    mono install --> install
    gecko installer -->  install
    gecko installer -->  install



## wine 设置中文

```
sudo apt install fonts-wqy-microhei

cp /usr/share/fonts/truetype/wqy/wqy-microhei.ttc ~/.wine/drive_c/windows/Fonts



wqy-microhei.reg:


REGEDIT4

[HKEY_LOCAL_MACHINE\Software\Microsoft\Windows NT\CurrentVersion\FontLink\SystemLink]
"Lucida Sans Unicode"="wqy-microhei.ttc"
"Microsoft Sans Serif"="wqy-microhei.ttc"
"Microsoft YaHei"="wqy-microhei.ttc"
"微软雅黑"="wqy-microhei.ttc"
"MS Sans Serif"="wqy-microhei.ttc"
"Tahoma"="wqy-microhei.ttc"
"Tahoma Bold"="wqy-microhei.ttc"
"SimSun"="wqy-microhei.ttc"
"Arial"="wqy-microhei.ttc"
"Arial Black"="wqy-microhei.ttc"
"宋体"="wqy-microhei.ttc"
"新細明體"="wqy-microhei.ttc"

```

wine regedit wqy-microhei.reg

LC_ALL=zh_CN.UTF-8 wine regedit wqy-microhei.reg
