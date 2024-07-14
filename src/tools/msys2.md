## mysy2 中使用 系统环境变量

添加环境变量:

MSYS2_PATH_TYPE=inherit

重启 MSYS2后, 即可

## pacman 基本命令

pacman -Syu 更新源, 升级所有软件包

pacman -Ss glib2 搜索glib库
pacman -S msys/glib2-devel 安装该库


搭建 64位开发环境:

    pacman -S --needed base-devel mingw-w64-x86_64-toolchain

搭建 32位开发环境:

    pacman -S base-devel mingw-w64-i686-toolchain

    <!-- pacman -S mingw-w64-i686-cmake 安装32位cmake -->


## 国内源

在 msys64\etc\pacman.d 目录下：
分别是：
mirrorlist.mingw32
mirrorlist.mingw64
mirrorlist.msys

### mirrorlist.mingw32

```
##
## 32-bit Mingw-w64 repository mirrorlist
##

## 清华大学镜像源
Server = https://mirrors.tuna.tsinghua.edu.cn/msys2/mingw/i686/
## 中国科学技术大学镜像源
Server = http://mirrors.ustc.edu.cn/msys2/mingw/i686/
## 北京理工大学镜像源
Server = http://mirror.bit.edu.cn/msys2/mingw/i686/
## 上海交通大学镜像源
Server = https://mirrors.sjtug.sjtu.edu.cn/msys2/mingw/i686/

```

### mirrorlist.mingw64

```
##
## 64-bit Mingw-w64 repository mirrorlist
##

## 清华大学镜像源
Server = https://mirrors.tuna.tsinghua.edu.cn/msys2/mingw/x86_64/
## 中国科学技术大学镜像源
Server = http://mirrors.ustc.edu.cn/msys2/mingw/x86_64/
## 北京理工大学镜像源
Server = http://mirror.bit.edu.cn/msys2/mingw/x86_64/
## 上海交通大学镜像源
Server = https://mirrors.sjtug.sjtu.edu.cn/msys2/mingw/x86_64/

```

### mirrorlist.msys

```
##
## MSYS2 repository mirrorlist
##

## 清华大学镜像源
Server = https://mirrors.tuna.tsinghua.edu.cn/msys2/msys/$arch/
## 中国科学技术大学镜像源
Server = http://mirrors.ustc.edu.cn/msys2/msys/$arch/
## 北京理工大学镜像源
Server = http://mirror.bit.edu.cn/msys2/msys/$arch/
## 上海交通大学镜像源
Server = https://mirrors.sjtug.sjtu.edu.cn/msys2/msys/$arch/

```
