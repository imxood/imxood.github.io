# Linux 运维笔记

## 说明

- 本页整理常见的 Linux 系统维护与排障命令, 适合作为日常速查入口.
- 内容以 Ubuntu / Debian 生态为主, 也覆盖部分通用命令行工具用法.
- 若你只是查某个发行版的个性化配置, 可优先回到本目录其它专题页.

## 适合如何使用

- 需要快速回忆包管理, 网络, 防火墙, 服务管理命令时, 先看本页.
- 需要发行版特定配置时, 再看 `Deepin`, `ArchLinux`, `manjaro笔记` 等页面.
- 需要 Wine, shell 或系统构建相关内容时, 继续进入 `wine/`, `shell/`, `make_system/`.

## 常用命令速查

### 软件包与系统信息

```sh
sudo apt update
sudo apt upgrade
sudo apt install <package>
sudo apt remove <package>
sudo apt autoremove
uname -a
lsb_release -a
```

- 包管理优先使用系统仓库, 避免长期混用来源不明的第三方源.
- 遇到依赖问题时, 可以先做 `sudo apt update` 再重新安装.

### 服务与启动项

```sh
systemctl status <service>
sudo systemctl start <service>
sudo systemctl restart <service>
sudo systemctl enable <service>
sudo systemctl disable <service>
journalctl -u <service> -f
```

- 服务异常时, 一般先看 `systemctl status` 和 `journalctl` 日志.
- 涉及图形登录器切换时, 还要同时确认 `display-manager.service` 指向哪个实现.

### 磁盘与挂载

```sh
df -h
lsblk
sudo fdisk -l
mount | grep <path>
sudo mount /dev/<device> <mountpoint>
sudo umount <mountpoint>
```

- 排查磁盘空间时优先看 `df -h`.
- 排查设备识别与分区情况时优先看 `lsblk`.

### 网络与端口

```sh
ip addr
ip route
ss -tunlp
ping <host>
curl -I <url>
```

- 端口占用优先用 `ss -tunlp` 查看.
- 网络连通性问题通常按 `网卡 -> 路由 -> DNS -> 目标服务` 顺序检查.

## 防火墙速记

### UFW 常用操作

```sh
sudo ufw enable
sudo ufw disable
sudo ufw status verbose
sudo ufw allow 22/tcp
sudo ufw allow 80/tcp comment 'accept Apache'
sudo ufw allow 443/tcp comment 'accept HTTPS connections'
sudo ufw allow 1194/udp comment 'OpenVPN server'
sudo ufw allow 3000:4000/tcp
sudo ufw allow from 104.22.10.214
sudo ufw allow from 104.22.11.213 to any port 25 proto tcp
sudo ufw deny from 203.5.1.43
sudo ufw status numbered
sudo ufw delete 6
sudo ufw reload
```

- 调整规则前先看 `sudo ufw status numbered`, 便于回滚和删除.
- 开放端口时尽量明确协议, 端口和来源地址, 减少误暴露风险.

### 诊断辅助

```sh
sudo more /var/log/ufw.log
sudo tail -f /var/log/ufw.log
sudo ufw show listening
sudo ufw show added
sudo ufw reset
```

- 规则改完不生效时, 可以先检查日志和监听情况.
- `reset` 会清空现有规则, 使用前应确认是否需要备份当前配置.

## 常见系统操作

### 安装 squid

```sh
sudo apt install squid
systemctl status squid
```

- 适合快速搭本地或局域网代理实验环境.
- 生产环境还需要补充访问控制, 日志和转发策略配置.

### 切换 display manager

```sh
systemctl status display-manager.service
sudo dpkg-reconfigure gdm3
sudo dpkg-reconfigure sddm
```

- 先确认当前实际运行的是 `gdm3`, `sddm` 还是其它显示管理器.
- 切换桌面环境后无法进图形界面时, 这个步骤很常用.

### Ubuntu 20 安装 Deepin 桌面

```sh
sudo add-apt-repository ppa:ubuntudde-dev/stable
sudo apt install ubuntudde-dde
systemctl status display-manager.service
sudo dpkg-reconfigure sddm
```

- 若登录界面异常, 通常要检查显示管理器是否与桌面环境组合正确.
- 更完整的 Deepin 相关环境配置可继续看 [Deepin](./deepin.md).

## 动态依赖排查

### `ldd`

```sh
ldd /usr/local/bin/st-info
```

- 用于查看 ELF 可执行文件依赖了哪些动态库.
- 若输出里出现 `not found`, 说明对应动态库未安装, 未在搜索路径中, 或版本不匹配.
- 很多 GUI 程序, 驱动工具和自己编译的二进制都适合先用它做依赖自检.

## 常见故障记录

### `GLIBC_2.28 not found`

- 这类报错通常意味着二进制构建环境高于当前系统运行环境.
- 常见思路是: 降低构建环境版本, 使用目标系统兼容的容器重新构建, 或升级目标系统.
- 不建议直接手工替换系统 `glibc`, 风险很高.

参考: <https://blog.csdn.net/Youning_Yim/article/details/129343107>

## 目录边界

- 本页保留通用 Linux 运维与系统管理速记.
- 发行版专属配置请放在各自页面.
- 某个工具已经有独立专题时, 本页只保留导航和最小速记, 避免与专题页重复维护.
