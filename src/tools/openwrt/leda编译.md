# OpenWrt / LEDE 编译

## 说明

- 本页记录 `LEDE/OpenWrt` 源码构建的最小流程.
- 适合作为本地尝试自定义固件时的环境准备速记.
- 如果目标已经转向具体机型适配, 还需要继续补“目标配置”, “插件选择”与“刷写方式”记录.

## 适用场景

- 第一次在本地准备 `OpenWrt/LEDE` 编译环境.
- 想先跑通源码下载, `feeds` 更新和一次完整构建.
- 排查下载阶段或编译阶段的基础失败问题.

## manjaro 安装编译环境

```sh
yay -S openwrt-devel
```

## 下载代码

```sh
git clone https://github.com/coolsnowwolf/lede
cd lede
```

## 更新 feeds

```sh
./scripts/feeds update -a
./scripts/feeds install -a
```

## 配置与编译

```sh
make menuconfig
make -j11 download V=s
make -j11 V=s
```

## 常见关注点

- 第一次构建前先确认磁盘空间和网络环境足够稳定.
- `download` 阶段失败时, 优先先把依赖拉全再重新编译.
- 真正面向具体路由器时, 要先确认目标架构, 分区布局和刷机方式.
- 多线程编译数不要机械照抄, 应结合本机 CPU, 内存和 IO 情况调整.

## 相关文档

- [OpenWrt 总览](./README.md)
- [OpenWrt 试玩笔记](./openwrt笔记.md)
