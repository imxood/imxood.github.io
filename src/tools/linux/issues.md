# Linux 常见问题

## 说明

- 本页收敛 Linux 环境下常见的证书, 包管理和初始化排错线索.
- 当前内容先以 `apt` 和 HTTPS 证书问题为主, 后续可继续补网络, 图形环境和权限问题.
- 若问题更偏系统命令和基础排查, 也可结合 [Linux 综合笔记](./Linux笔记.md) 一起看.

## 常见问题 1: HTTPS 证书导致包拉取失败

报错示例:

```text
Error: Could not fetch URL https://...
There was a problem confirming the ssl certificate
```

处理方式:

```sh
sudo apt-get install apt-transport-https ca-certificates software-properties-common
```

## 排查顺序

### 1. 先看系统时间

- 时间错误会直接导致证书校验异常.
- 新装系统, 双系统切换和离线设备上最常见.

### 2. 再看 CA 证书包

- 检查 `ca-certificates` 是否安装完整.
- 必要时重新安装或更新证书缓存.

### 3. 再看代理或镜像源

- 公司代理, 自签名证书和镜像站异常都可能导致 HTTPS 拉取失败.
- 如果只是某一个源失败, 也要排查该源本身的证书与可用性.

### 4. 最后看网络链路

- DNS 配置错误, 网络劫持和透明代理也会表现为证书问题.
- 可先用 `curl -v` 单独验证目标 URL.

## 其它高频排查方向

### `apt update` 很慢或卡住

- 先确认镜像源是否可用.
- 再确认 DNS 和代理配置.
- 必要时切回官方源做对照测试.

### 安装包时依赖冲突

- 先做 `sudo apt update`.
- 再看是否混用了第三方源, PPA 或手工安装包.
- 尽量避免长期混用发行版包和来源不明的二进制包.

### 命令找不到

- 先看程序是否已安装.
- 再检查 `PATH` 和 shell profile 是否正确加载.
- 若刚安装完软件仍找不到命令, 可以先重开 shell 再验证.

## 使用建议

- 涉及网络代理, 包源和 SSL 问题时, 建议优先记录完整报错, 不要只保留结论.
- 新系统初始化时, 可以把“时间同步, 证书, 包源, DNS”作为第一轮检查项.
- 若问题已经演变成发行版特有故障, 建议回到对应专题页继续记录.

## 相关文档

- [Linux 总览](./README.md)
- [Linux 综合笔记](./Linux笔记.md)
- [Linux 常用命令](./linux常用命令.md)
