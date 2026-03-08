# VirtualBox 使用指南

## 说明

- 本页记录 Linux 环境下安装和启用 `VirtualBox` 的最小步骤.
- 适合快速搭建本地虚拟机实验环境或回忆内核模块相关配置.

## 安装

```sh
yay -S virtualbox linux56-virtualbox-host-modules virtualbox-ext-oracle
sudo gpasswd -a $USER vboxusers
sudo vboxreload
reboot now
```

## 步骤理解

- 安装 `VirtualBox` 本体.
- 安装对应内核版本的 host modules.
- 把当前用户加入 `vboxusers` 组.
- 重新加载模块并重启系统, 确保驱动生效.

## 使用建议

- 安装时要特别注意宿主机当前内核版本与 `host-modules` 包是否匹配.
- 若启动虚拟机时报权限或驱动错误, 优先检查用户组, 内核模块和 Secure Boot 相关限制.
- 发行版升级内核后, 若 `VirtualBox` 失效, 往往要先回头检查模块是否需要重装.
