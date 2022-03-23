## deepin 编译

```sh
# 声音
sudo apt install libasound2-dev libpulse-dev
# sdl2
sudo apt install libsdl2-dev
```



## ubuntu 编译

[参考](https://trac.ffmpeg.org/wiki/CompilationGuide/Ubuntu)

```sh
sudo apt-get -y install \
    autoconf \
    automake \
    build-essential \
    cmake \
    git-core \
    libass-dev \
    libfreetype6-dev \
    libgnutls28-dev \
    libsdl2-dev \
    libtool \
    libva-dev \
    libvdpau-dev \
    libvorbis-dev \
    libxcb1-dev \
    libxcb-shm0-dev \
    libxcb-xfixes0-dev \
    meson \
    ninja-build \
    pkg-config \
    texinfo \
    wget \
    yasm \
    zlib1g-dev

sudo apt-get install libx264-dev
sudo apt-get install libx265-dev libnuma-dev
sudo apt-get install libvpx-dev
sudo apt-get install libfdk-aac-dev
sudo apt-get install libmp3lame-dev
sudo apt-get install libopus-dev

git clone --branch release/4.4 https://github.com/ffmpeg/ffmpeg

cd ffmpeg

# ./configure --help, 查看所有的配置
./configure --prefix="/develop/programs/ffmpeg_build" --enable-pic

# 默认证书是: LGPL version 2.1 or later
# 会自动编译静态库.
# --ld="clang", 使用clang的链接器

make -j

make install

报错:
    gnutls not found using pkg-config

解决:
    sudo apt-get install libunistring-dev


报错:
    `.rodata' can not be used when making a PIE object; recompile with -fPIC
解决:
    --enable-pic

需要 make clean, 删除以前的build, 并配置 编译
```