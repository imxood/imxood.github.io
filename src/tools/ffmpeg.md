# FFmpeg

## 说明

- `FFmpeg` 是常用的音视频编解码, 转码, 封装和媒体处理工具链.
- 本页当前以 Linux 环境下的源码编译和常见报错排查为主.

## 适用场景

- 需要自定义编译选项或启用特定编解码库.
- 系统仓库中的 `FFmpeg` 版本过旧, 需要自行升级.
- 排查编译依赖缺失, `pkg-config` 检测失败或位置无关代码相关问题.

## Deepin 编译准备

```sh
sudo apt install libasound2-dev libpulse-dev
sudo apt install libsdl2-dev
```

## Ubuntu 编译

参考: <https://trac.ffmpeg.org/wiki/CompilationGuide/Ubuntu>

```sh
sudo apt-get -y install autoconf automake build-essential cmake git-core libass-dev libfreetype6-dev libgnutls28-dev libsdl2-dev libtool libva-dev libvdpau-dev libvorbis-dev libxcb1-dev libxcb-shm0-dev libxcb-xfixes0-dev meson ninja-build pkg-config texinfo wget yasm zlib1g-dev
sudo apt-get install libx264-dev
sudo apt-get install libx265-dev libnuma-dev
sudo apt-get install libvpx-dev
sudo apt-get install libfdk-aac-dev
sudo apt-get install libmp3lame-dev
sudo apt-get install libopus-dev

git clone --branch release/4.4 https://github.com/ffmpeg/ffmpeg
cd ffmpeg
./configure --prefix="/develop/programs/ffmpeg_build" --enable-pic
make -j
make install
```

## 常见报错

### `gnutls not found using pkg-config`

```sh
sudo apt-get install libunistring-dev
```

### `.rodata can not be used when making a PIE object`

- 重新配置时增加 `--enable-pic`.
- 重新编译前建议先 `make clean`.

## 使用建议

- 优先明确目标是“直接用发行版包”, 还是“必须自己编译”, 不要无谓增加维护成本.
- 编译前先确定是否真的需要额外编解码库, 不然依赖会快速膨胀.
- 若只是做日常转码或探测, 后续可继续补 `ffmpeg`, `ffprobe` 常用命令速记.
