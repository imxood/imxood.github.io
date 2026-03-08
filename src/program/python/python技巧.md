# Python 技巧

## 说明

- 本页整理日常 Python 开发里高频但容易散落的小技巧.
- 当前先从“临时按模块运行源码”展开, 并补充几个常用场景.
- 适合做快速速查页, 不替代完整环境管理文档.

## 不安装包, 直接按模块运行

对于开源模块, 若当前只想临时运行源码而不安装, 可直接指定 `PYTHONPATH`:

```sh
PYTHONPATH="$PYTHONPATH:$(pwd)" python3 -m log_server "$@"
```

## 适用场景

- 临时调试源码仓库.
- 本地快速验证某个模块入口.
- 不希望污染当前虚拟环境时的短期实验.

## 其它高频技巧

### 查看模块导入路径

```python
import module_name
print(module_name.__file__)
```

- 适合排查到底加载的是哪个版本的模块.

### 快速展开字典参数

```python
params = {"name": "demo", "age": 18}
func(**params)
```

- 很适合做配置传参和函数适配.

### 用 `pathlib` 代替字符串拼路径

```python
from pathlib import Path
config = Path('config') / 'app.toml'
```

- 在跨平台脚本里更稳定, 可读性也更好.

### 命令行临时运行一段代码

```sh
python -c "print('hello')"
```

- 适合做快速验证和一次性脚本.

## 使用建议

- 适合临时调试, 本地实验和快速验证源码仓库.
- 若后续需要长期使用, 仍建议补齐正式安装或虚拟环境配置.
- 若项目依赖较多, 最终还是建议配合虚拟环境或 `Conda` / `pyenv` 使用.

## 相关文档

- [Python 总览](./README.md)
- [pyenv 使用说明](./pyenv使用说明.md)
- [创建进程](./process.md)
