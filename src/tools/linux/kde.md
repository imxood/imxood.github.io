# KDE

## 说明

- 本页记录 `KDE` 桌面环境下的常用软件安装和系统配置操作.
- 当前内容以 `Ubuntu / KDE` 场景为主, 也适合作为桌面环境问题的速记入口.

## 常用软件

```sh
sudo snap install blender aria2c
```

适用场景:

- `blender`: 图形或建模相关工作.
- `aria2c`: 多连接下载和大文件拉取.

## Ubuntu 安装 AMD 显卡驱动

参考: <https://www.amd.com/zh-hans/support/kb/release-notes/rn-amdgpu-unified-linux-20-20>

```sh
./amdgpu-install --opencl=pal,legacy --no-dkms --headless
```

使用建议:

- 安装前先确认内核版本和驱动支持范围.
- 若只是远程计算或无桌面渲染需求, 可按需选择更精简的安装参数.

## 安装 OpenCL

- 参考: <https://rocmdocs.amd.com/en/latest/Installation_Guide/Installation-Guide.html>

排查方向:

- 确认驱动与 OpenCL 运行时是否版本匹配.
- 确认 GPU 是否已被系统和上层工具正确识别.

## 解除“输入密码以解锁密钥环”

```sh
sudo apt-get install seahorse
```

处理思路:

- 在应用程序中搜索 `seahorse`.
- 删除或重置对应的 keyring 项.
- 重新登录后观察提示是否消失.

## 整理建议

- 若后续继续补充, 可加入输入法, 缩放, 多屏和主题配置相关记录.
- 若问题已偏向发行版级驱动或包管理, 应回到 `Linux 总览` 或对应发行版页继续看.
