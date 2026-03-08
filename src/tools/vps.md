# VPS 运维笔记

## 说明

- 本页记录新购 `VPS` 的最小初始化步骤和常用运维命令.
- 适合做个人服务部署, 远程开发环境和轻量实验机的快速检查清单.

## 初始检查清单

1. 确认系统版本, 时区和 SSH 连接方式.
2. 完成普通用户创建和权限配置.
3. 配置防火墙与必要端口.
4. 再安装业务运行时, 容器或面板工具.

## mdserver-web

- 项目: <https://github.com/midoks/mdserver-web#jsdelivr%E5%AE%89%E8%A3%85%E5%9C%B0%E5%9D%80>

安装:

```sh
curl --insecure -fsSL https://cdn.jsdelivr.net/gh/midoks/mdserver-web@latest/scripts/install.sh | sudo bash
```

使用提醒:

- 部署后记得开放面板和业务所需端口.
- 建议先确认面板仅绑定可信地址, 或配合额外认证方式使用.

## 新用户与权限

```sh
sudo useradd -m -s /bin/bash xxx
sudo usermod -a -G sudo,adm,dialout xxx
sudo passwd xxx
```

建议:

- 日常维护优先使用普通用户, 减少长期直接使用 `root`.
- 若涉及串口或设备透传, 再按需补充用户组.

## Rust 环境

在 `~/.bashrc` 中设置:

```sh
export RUSTUP_DIST_SERVER="https://rsproxy.cn"
export RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"
```

安装:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://rsproxy.cn/rustup-init.sh | sh
```

### Cargo 镜像

写入 `~/.cargo/config`:

```toml
[source.crates-io]
replace-with = 'rsproxy'

[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"

[registries.rsproxy]
index = "https://rsproxy.cn/crates.io-index"

[net]
git-fetch-with-cli = true
```

## 防火墙

```sh
sudo ufw status
sudo ufw allow 22/tcp
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw enable
```

使用建议:

- 先只开放真正需要的端口, 不要一开始全部放开.
- 每新增服务后, 重新核对监听端口和外部暴露范围.

## 运维建议

- 定期做系统更新和基础备份.
- 优先启用 SSH 密钥登录, 再考虑关闭密码登录.
- 对公开服务至少增加日志, 监控和异常告警的最小闭环.
- 如果一台机器上承载多个服务, 建议配合 `Docker` 或反向代理统一管理入口.
