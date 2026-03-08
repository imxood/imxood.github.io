# 创建进程

## 说明

- 本页整理 Python 中创建子进程的常见方式, 以 `subprocess` 标准库为主.
- 适合处理命令执行, 日志转发, 外部工具调用和构建脚本集成.
- 若任务只是简单执行一次命令, 优先考虑 `subprocess.run`; 若要持续读取输出, 再使用 `Popen`.

## 常见选择

### `subprocess.run`

- 适合一次性执行命令并等待结束.
- 代码更短, 参数也更直观.

```python
import subprocess

result = subprocess.run(
    ["python", "--version"],
    capture_output=True,
    text=True,
    check=False,
)

print(result.returncode)
print(result.stdout)
print(result.stderr)
```

### `subprocess.Popen`

- 适合需要边执行边读取输出, 或者自行控制生命周期的场景.
- 构建工具, 长时间任务和守护型进程包装中最常见.

## 阻塞式读取完整输出

```python
import subprocess as sp

with sp.Popen(
    cmd,
    stdout=sp.PIPE,
    stderr=sp.PIPE,
    shell=True,
) as proc:
    stdout, stderr = proc.communicate()
    lines = stdout.decode("utf-8", errors="replace").splitlines()
```

适合场景:

- 命令执行时间较短.
- 需要一次性拿到完整标准输出和标准错误.
- 后续逻辑依赖进程结束后的完整结果.

## 实时读取日志输出

```python
import subprocess as sp

with sp.Popen(
    cmd,
    stdout=sp.PIPE,
    stderr=sp.STDOUT,
    shell=True,
) as proc:
    for line in iter(proc.stdout.readline, b""):
        text = line.decode("utf-8", errors="replace").rstrip()
        print(text)
```

适合场景:

- 需要边运行边展示日志.
- 构建命令, 下载命令和长时间任务.
- 需要把外部程序输出实时转发到 UI 或终端.

## 常用参数

### `cwd`

- 指定子进程工作目录.
- 很适合执行项目内脚本或构建命令.

### `env`

- 指定或覆盖环境变量.
- 适合临时注入代理, token 或构建参数.

### `text=True`

- 让输出直接按文本处理, 避免手工 `decode`.
- 但实时读取二进制流时, 仍可能需要手工控制字节流处理.

### `timeout`

- 避免命令无限卡住.
- 用于自动化脚本时很常见.

## 使用建议

- 若命令参数来自外部输入, 尽量避免直接 `shell=True` 拼接字符串.
- 优先使用列表参数形式, 例如 `['git', 'status']`, 这样更安全.
- 实时展示日志时, 通常把 `stderr` 合并到 `stdout` 会更简单.
- 如果进程可能产出大量输出, 不要同时长期不消费 `stdout` / `stderr`, 否则可能阻塞.

## 常见问题

### 1. 为什么命令在终端能跑, 在脚本里失败

- 先检查 `cwd` 是否正确.
- 再检查环境变量是否完整, 例如 `PATH`.
- 最后确认是否依赖 shell 特性, 比如通配符, 管道或重定向.

### 2. 为什么实时日志没有输出

- 有些程序默认对非交互环境启用缓冲.
- 可尝试程序自身的“无缓冲”参数, 或改为逐行刷新输出.

### 3. 为什么 `shell=True` 有风险

- 因为字符串拼接命令容易引入命令注入问题.
- 自动化场景里尤其要避免把用户输入直接拼进 shell 命令.

## 相关文档

- [Python 总览](./README.md)
- [socket 笔记](./socket.md)
- [Python 技巧](./python技巧.md)
