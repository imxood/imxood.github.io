## ubuntu 编译

[参考](https://trac.ffmpeg.org/wiki/CompilationGuide/Ubuntu)

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

    git clone https://github.com/FFmpeg/FFmpeg.git

    cd FFmpeg

    PKG_CONFIG_PATH="/develop/programs/ffmpeg_build/lib/pkgconfig" ./configure \
        --prefix="/develop/programs/ffmpeg_build" \
        --pkg-config-flags="--static" \
        --extra-cflags="-I/develop/programs/ffmpeg_build/include" \
        --extra-ldflags="-L/develop/programs/ffmpeg_build/lib" \
        --extra-libs="-lpthread -lm" \
        --ld="g++" \
        --bindir="/develop/programs/ffmpeg_build/bin" \
        --enable-gpl \
        --enable-gnutls \
        --enable-libass \
        --enable-libfdk-aac \
        --enable-libfreetype \
        --enable-libmp3lame \
        --enable-libopus \
        --enable-libvorbis \
        --enable-libvpx \
        --enable-libx264 \
        --enable-libx265 \
        --enable-nonfree

    报错:
        gnutls not found using pkg-config

    解决:
        sudo apt-get install libunistring-dev


    报错:
        `.rodata' can not be used when making a PIE object; recompile with -fPIC
    解决:
        --enable-pic

        需要 make clean, 并删除以前的build, 避免影响

