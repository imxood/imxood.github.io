## mdserver

linux vps web面板

https://github.com/midoks/mdserver-web#jsdelivr%E5%AE%89%E8%A3%85%E5%9C%B0%E5%9D%80

安装:

curl --insecure -fsSL https://cdn.jsdelivr.net/gh/midoks/mdserver-web@latest/scripts/install.sh | sudo bash

防火墙 需要开启 外部访问的某些端口

## 添加新用户 添加组 修改密码

sudo useradd -m -s /bin/bash xxx

sudo usermod -a -G sudo,adm,dialout xxx

sudo passwd xxx

## rust 环境

vi ~/.bashrc

export RUSTUP_DIST_SERVER="https://rsproxy.cn"
export RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"

curl --proto '=https' --tlsv1.2 -sSf https://rsproxy.cn/rustup-init.sh | sh

### cargo mirror

vi ~/.cargo/config

[source.crates-io]
replace-with = 'rsproxy'

[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"

[registries.rsproxy]
index = "https://rsproxy.cn/crates.io-index"

[net]
git-fetch-with-cli = true

### 防火墙

sudo ufw status

sudo ufw allow 22/tcp
sudo ufw delete allow 22
