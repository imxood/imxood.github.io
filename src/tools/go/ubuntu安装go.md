# Ubuntu 安装 Go

## 说明

- 本页整理 Ubuntu 环境下安装和配置 Go 的常见方式.
- 适合快速搭建开发环境, 校验 `GOROOT` / `GOPATH`, 以及处理旧项目的历史版本需求.
- 若只是做现代 Go 开发, 通常优先使用官方二进制包或系统仓库提供的较新版本.

## 常见安装方式

### 方式 1: 使用系统仓库或 PPA

```sh
sudo add-apt-repository ppa:gophers/archive
sudo apt-get update
sudo apt-get install golang-1.11-go
```

- 这是历史记录中的安装方式, 适合回看旧环境配置.
- 版本号需要按实际仓库内容调整, 不建议机械照搬旧版本号.

### 方式 2: 使用官方二进制包

- 适合希望明确控制安装版本时使用.
- 安装完成后, 通常只需把 `bin` 目录加入 `PATH`.

## 环境变量示例

```sh
export PATH=/usr/lib/go-1.12/bin:$PATH
export GOROOT=/usr/lib/go-1.12
export GOPATH=~/develop/go
```

- `GOROOT` 指向 Go 自身安装位置.
- `GOPATH` 用于历史项目工作目录和缓存布局.
- 新版 Go 已普遍基于 module 工作, 不必再把所有项目都放在 `GOPATH/src` 下.

## 安装后验证

```sh
go version
go env
which go
```

- 先确认命令路径是否符合预期.
- 再检查 `GOROOT`, `GOPATH`, `GOMODCACHE` 等环境是否正确.

## 常见建议

- 新项目优先使用 Go Modules, 不要再依赖旧式 `GOPATH` 工程结构.
- 如果系统里存在多套 Go, 要先确认当前 shell 实际命中了哪一个 `go`.
- 对 CI 或长期维护项目, 建议固定版本来源, 避免不同机器之间版本漂移.

## 常见问题

### `go version` 不是刚安装的版本

- 多半是 `PATH` 中还有旧版 Go.
- 先用 `which go` 或 `type -a go` 查看实际命中路径.

### 模块下载慢或失败

- 检查代理配置与网络环境.
- 必要时显式设置 `GOPROXY`.

### 旧项目仍依赖 `GOPATH`

- 可以保留 `GOPATH`, 但应明确区分“历史兼容”与“现代模块化项目”两类工作流.

## 相关文档

- [工具总览](../README.md)
- [Linux 总览](../linux/README.md)
