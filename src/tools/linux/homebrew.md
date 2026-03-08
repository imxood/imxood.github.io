# Homebrew

## 说明

- 本页整理 Linux 环境下安装和使用 `Homebrew` 的最小步骤.
- 适合在系统包管理器之外补充较新的开发工具或交叉编译工具链.
- 若系统仓库已经满足需求, 通常优先使用发行版原生包管理器.

## 适用场景

- 系统仓库版本过旧, 但又不想手工源码编译.
- 需要快速补装某些开发工具, 并希望在多台机器间保持一致安装方式.
- 想给用户态工具做隔离, 减少对系统环境的直接污染.

## 安装

```sh
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

安装后常见初始化动作:

```sh
brew doctor
brew --version
brew config
```

## 路径与环境变量

- Linux 下常见安装位置包括 `~/.linuxbrew` 或 Homebrew 当前默认目录.
- 安装完成后需要把 `brew` 所在目录加入 `PATH`.
- 若 shell 启动后找不到 `brew`, 先检查 profile 文件是否已写入环境变量.

## 常见用法

### 搜索包

```sh
brew search <name>
```

### 安装包

```sh
brew install <formula>
```

### 查看环境建议

```sh
brew doctor
```

### 历史记录中的交叉编译工具链示例

```sh
brew install --cask gcc-arm-embedded
```

- 这类工具安装后, 建议立刻检查版本和可执行路径.

## 使用建议

- 优先确认发行版自带包是否已经满足需求, 避免多套包管理器混用过深.
- 对编译链工具, 安装后最好立刻检查版本和可执行路径.
- 涉及系统级依赖时, `brew` 与发行版包并不一定完全兼容, 混搭要谨慎.
- 如果团队机器较多, 应约定清楚到底用系统包, `brew`, 还是容器化工具链.

## 常见问题

### `brew` 安装成功但命令找不到

- 多半是 `PATH` 未生效.
- 先检查 shell profile 是否加载了 Homebrew 环境脚本.

### 包装好但运行失败

- 先看是否缺系统级依赖.
- 再看该工具是否更适合由发行版原生包提供.

### 同一工具出现多个版本

- 很可能是系统仓库和 `brew` 同时安装了同名程序.
- 先用 `which` 或 `type -a` 确认当前实际命中路径.

## 相关页面

- [Linux 总览](./README.md)
- [Linux 工具](./linux工具.md)
