# Manjaro 笔记

## 说明

- 本页记录 `Manjaro` 初始化安装, 国内源, 输入法, 开发环境和常用软件配置.
- 大多数命令来自历史版本实测, 使用前请结合当前系统版本确认.

## 系统镜像与启动盘

下载镜像:

- <https://mirrors.tuna.tsinghua.edu.cn/osdn/storage/g/m/ma/manjaro/xfce/19.0.2/manjaro-xfce-19.0.2-200311-linux54.iso>

写入 U 盘:

```sh
dd if=manjaro-xfce-19.0.2-200311-linux54.iso of=/dev/sdb bs=16M
```

## 基础初始化

```sh
sudo pacman-mirrors -c China
sudo pacman -Syyu
sudo pacman -S vim
```

添加 `archlinuxcn` 源:

```conf
[archlinuxcn]
SigLevel = Optional TrustedOnly
Server = https://mirrors.ustc.edu.cn/archlinuxcn/$arch
```

```sh
sudo pacman -Syy && sudo pacman -S archlinuxcn-keyring
```

## 中文与输入法

安装中文字体:

```sh
sudo pacman -S wqy-bitmapfont wqy-microhei wqy-microhei-lite wqy-zenhei
```

安装输入法:

```sh
sudo pacman -S kcm-fcitx fcitx-googlepinyin
sudo pacman -S fcitx-lilydjwg-git fcitx-configtool fcitx-sogoupinyin
```

`~/.xprofile` 示例:

```sh
export LC_ALL=zh_CN.UTF-8
export GTK_IM_MODULE=fcitx
export QT_IM_MODULE=fcitx
export XMODIFIERS="@im=fcitx"
```

如果搜狗输入法异常, 可先卸载并清理配置:

```sh
sudo pacman -Rs fcitx-lilydjwg-git fcitx-configtool fcitx-sogoupinyin
cd ~/.config
rm -rf SogouPY SogouPY.users sogou-qimpanel fcitx
```

## Shell 与开发环境

安装 `zsh` 与 `oh-my-zsh`:

```sh
sudo pacman -S zsh
sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"
sudo usermod -s /bin/zsh USERNAME
```

可选主题记录: `mortalscumbag`, `afowler`, `gentoo`, `gallois`.

## 常用软件

```sh
sudo pacman -S visual-studio-code-bin
sudo pacman -S screenfetch
sudo pacman -S clang gdb
sudo pacman -S deepin-screenshot
```

## 使用建议

- 新系统初始化时, 先完成镜像源, 系统更新和基础编辑器安装.
- 输入法异常通常优先检查 `fcitx` 配置和 `~/.xprofile` 环境变量.
- AUR 或第三方源包较多时, 更新前应先确认依赖关系和兼容风险.
- 若后续继续整理, 可补驱动安装, Docker, 虚拟化和开发机备份策略.
