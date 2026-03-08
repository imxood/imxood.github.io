# Deepin

## 说明

- 本页记录 `Deepin` 场景下的一些历史安装与环境配置操作.
- 当前重点包括 `Wine`, `Node.js`, `MySQL` 和新版 `clang` 工具链的安装线索.
- 这类记录偏系统初始化与兼容配置, 真正执行前应结合当前发行版版本再次确认.

## 安装 Wine

```sh
sudo apt install deepin-wine
```

适用场景:

- 需要在 `Deepin` 环境下运行部分 Windows 程序.
- 想直接复用系统已有的 `Deepin Wine` 兼容层.

## 安装 Node.js

历史下载记录:

```text
https://npm.taobao.org/mirrors/node/latest-v15.x/node-v15.3.0-linux-x64.tar.xz
```

解压后创建链接:

```sh
sudo ln -s /develop/programs/node-v15.3.0-linux-x64/bin/node /usr/local/bin/
sudo ln -s /develop/programs/node-v15.3.0-linux-x64/bin/npm /usr/local/bin/
```

设置镜像源:

```sh
npm config set registry https://registry.npm.taobao.org
npm config get registry
```

使用建议:

- 这类手工软链方式更适合历史环境记录.
- 如果是新机器, 优先考虑更稳定的版本管理方案或系统包管理方式.

## 安装 MySQL

```sh
sudo apt install libmecab2
sudo apt install -f

sudo dpkg -i mysql-community-client-plugins_8.0.22-1debian10_amd64.deb
sudo dpkg -i mysql-community-client-core_8.0.22-1debian10_amd64.deb
sudo dpkg -i mysql-community-client_8.0.22-1debian10_amd64.deb
sudo dpkg -i mysql-client_8.0.22-1debian10_amd64.deb

sudo dpkg -i mysql-common_8.0.22-1debian10_amd64.deb
sudo dpkg -i mysql-community-server-core_8.0.22-1debian10_amd64.deb
sudo dpkg -i mysql-community-server_8.0.22-1debian10_amd64.deb
```

补充说明:

- 安装过程中若弹出设置密码窗口, 这里的历史记录是“使用默认的加密方式”.
- 登录 MySQL:

```sh
sudo mysql -uroot -p
```

## 安装新版 clang

编辑 `/etc/apt/sources.list`, 添加:

```conf
deb http://apt.llvm.org/buster/ llvm-toolchain-buster main
deb-src http://apt.llvm.org/buster/ llvm-toolchain-buster main
deb http://apt.llvm.org/buster/ llvm-toolchain-buster-10 main
deb-src http://apt.llvm.org/buster/ llvm-toolchain-buster-10 main
deb http://apt.llvm.org/buster/ llvm-toolchain-buster-11 main
deb-src http://apt.llvm.org/buster/ llvm-toolchain-buster-11 main
```

导入 key 并安装:

```sh
wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | sudo apt-key add -
sudo apt-get update
sudo apt-get install clang-11
```

## 使用建议

- `Deepin` 相关记录受系统版本影响较大, 真正执行前要先确认软件源是否仍可用.
- 老版本手工安装包和软链配置, 更适合当作迁移参考, 不建议直接无脑复用.
- 若后续继续整理, 可补系统源, 输入法, Docker, 远程桌面和 Deepin Wine 的实际排错记录.
