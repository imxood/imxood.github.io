# pyenv 使用说明

## 说明

- `pyenv` 用于在同一台机器上管理多个 Python 版本.
- 适合做项目隔离, 老项目兼容和测试不同解释器版本的场景.

## 自动安装 pyenv

```sh
curl https://pyenv.run | bash
```

根据终端提示, 将下面的命令添加到 `~/.bashrc`:

```sh
export PYENV_ROOT="$HOME/.pyenv"
command -v pyenv >/dev/null || export PATH="$PYENV_ROOT/bin:$PATH"
eval "$(pyenv init -)"
```

如果需要 `pyenv-virtualenv`, 继续追加:

```sh
eval "$(pyenv virtualenv-init -)"
```

## Ubuntu 手动安装

```sh
git clone https://github.com/pyenv/pyenv.git ~/.pyenv
```

## 常见使用流程

### 查看可安装版本

```sh
pyenv install --list
```

### 安装指定版本

```sh
pyenv install 3.11.9
```

### 设置全局或局部版本

```sh
pyenv global 3.11.9
pyenv local 3.10.14
```

## 使用建议

- 新项目优先用 `pyenv local` 在项目目录内固定版本.
- 多项目并存时, 不要只依赖系统自带 Python.
- 若命令未生效, 优先检查 shell 初始化脚本是否已重新加载.
- 若后续继续整理, 可补 `pyenv-virtualenv`, 构建依赖和 Windows 替代方案说明.
