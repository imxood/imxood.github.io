# python3 笔记

## 说明

- 本页整理 Python 3 开发中最常用的环境配置, 调试入口和基础速查.
- 历史版本安装命令会保留为参考, 但更推荐结合当前环境使用 `pyenv` 或系统包管理器做统一管理.
- 若问题更偏具体库和脚本技巧, 可继续看 [Python 技巧](./python技巧.md), [创建进程](./process.md), [socket 笔记](./socket.md).

## 常见库线索

- `readchar`: 读取键盘输入的轻量库.
- `fabric`, `paramiko`: 远程 SSH 执行与自动化运维常用库.
- 若要做环境隔离与包管理, 优先考虑 `venv` 或 `pyenv` 配合使用.

## 环境与版本管理

### 使用 `pyenv`

```sh
sudo apt-get install --no-install-recommends make build-essential libssl-dev zlib1g-dev libbz2-dev libreadline-dev libsqlite3-dev wget curl llvm libncurses5-dev xz-utils tk-dev libxml2-dev libxmlsec1-dev libffi-dev liblzma-dev
curl https://pyenv.run | bash
pyenv install -v 3.11.0
pyenv global 3.11.0
pyenv versions
python --version
```

- 适合在一台机器上维护多版本 Python.
- 与项目级隔离环境配合时更方便.

### 历史系统安装记录

```sh
sudo add-apt-repository ppa:deadsnakes/ppa
sudo apt update
sudo apt install -y python3.7
sudo update-alternatives --install /usr/bin/python3 python3 /usr/bin/python3.5 1
sudo update-alternatives --install /usr/bin/python3 python3 /usr/bin/python3.7 2
sudo update-alternatives --config python3
```

- 这类记录更适合旧系统或历史环境回溯.
- 现代环境里, 不建议频繁修改系统默认 `python3` 指向.

### 源码安装记录

```sh
wget https://www.python.org/ftp/python/3.6.2/Python-3.6.2.tar.xz
tar -xvf Python-3.6.2.tar.xz
cd Python-3.6.2
./configure
make
sudo make install
```

- 更适合做特殊版本验证或独立安装测试.
- 若只是日常开发, 维护成本通常高于 `pyenv`.

## 调试与开发

### VSCode 调试 Python 多进程

```python
multiprocessing.set_start_method('spawn', True)
```

`launch.json` 中补充:

```json
{
  "subProcess": true
}
```

- 对多进程程序调试很实用.
- Windows 和部分跨平台场景下, `spawn` 更容易获得一致行为.

### 查看对象与类型

```python
len(obj)
type(obj)
isinstance(obj, class_name)
```

- 这些是最基础的调试与排查入口.
- 遇到数据结构不确定时, 先确认类型往往最省时间.

## 包与环境排查

### 安装 `pip`

```sh
curl https://bootstrap.pypa.io/get-pip.py | python3 - --user
```

- 更适合用户目录安装场景.
- 若系统已经由包管理器提供 `pip`, 不一定需要重复覆盖.

### 搜索包是否已安装

方法 1:

```python
try:
    from pip._internal.utils.misc import get_installed_distributions
except ImportError:
    from pip import get_installed_distributions
```

方法 2:

```python
import pkg_resources

dists = [d for d in pkg_resources.working_set]
```

- 现代脚本里更推荐直接用 `importlib.metadata` 或 `pip list` 做外部检查.
- 历史代码里仍可能遇到上面两种写法.

## 平台信息速查

```python
import os
import sys
import platform

os.name
sys.platform
platform.system()
platform.platform()
platform.version()
platform.architecture()
platform.machine()
platform.node()
platform.processor()
platform.uname()
```

- 适合排查系统差异, 架构差异和平台相关 bug.
- 写跨平台脚本时, 最常用的是 `sys.platform` 与 `platform.system()`.

## 常见问题记录

### SSL 证书导致 `pip` 拉取失败

报错示例:

```text
Could not fetch URL https://... There was a problem confirming the ssl certificate
```

处理思路:

```sh
sudo apt-get install apt-transport-https ca-certificates software-properties-common
python3 get-pip.py --user --trusted-host pypi.tuna.tsinghua.edu.cn
```

- 先检查系统时间和 CA 证书是否正常.
- 若处于代理或镜像环境, 还要排查代理和镜像配置.

### `pip` 升级后入口异常

```text
ImportError: cannot import name 'main'
```

常见处理:

```sh
curl https://bootstrap.pypa.io/get-pip.py | python3 - --user
```

- 这类问题通常和旧版 `pip` 启动脚本不兼容有关.

### `setuptools` 版本过低

```text
pkg_resources.VersionConflict: (setuptools 20.7.0, Requirement.parse('setuptools>=40.0'))
```

处理方式:

```sh
pip3 install -U --user setuptools
```

### `matplotlib` 无法弹窗显示

```text
UserWarning: FigureCanvasAgg is non-interactive, and thus cannot be shown
```

- 先检查当前后端是否支持图形界面.
- 远程环境, WSL 或无桌面环境下, 往往需要改用保存图片或切换后端.

## 阅读路径

- 环境与版本管理优先看本页和 [pyenv 使用说明](./pyenv使用说明.md).
- 自动化调用外部命令优先看 [创建进程](./process.md).
- 网络底层调用优先看 [socket 笔记](./socket.md).
- 更偏代码技巧和标准库小用法时, 再看 [Python 技巧](./python技巧.md).
