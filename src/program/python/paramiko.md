# Paramiko

## 说明

- `Paramiko` 是 Python 中常用的 SSH 自动化库.
- 适合远程命令执行, 文件传输和运维脚本编写场景.

## SSH 客户端基本用法

```python
from paramiko import SSHClient, AutoAddPolicy

connect_kwargs = dict(
    hostname=hostname,
    port=port,
    username=username,
    timeout=timeout,
)
if password:
    connect_kwargs['password'] = password

client = SSHClient()
client.set_missing_host_key_policy(AutoAddPolicy())

client.connect(**connect_kwargs)
transport = client.get_transport()

channel = transport.open_session()
channel.settimeout(timeout)
channel.set_combine_stderr(True)
channel.exec_command('ls')
```

## 常见能力

- 执行远程命令.
- 通过 `SFTP` 上传和下载文件.
- 复用一个连接上的多个 `Channel`.
- 结合超时, 日志和重试做自动化运维脚本.

## 源码简单分析

- 一个 `client` 中会维护一个 `Transport` 作为传输线程.
- `Transport` 内部持有 `packetizer`, 负责基于远程 socket 收发数据.
- 更高级的通信抽象是 `Channel`, 多个 `Channel` 共享同一个 `Transport`.

### 认证过程线索

- `client.connect()`
- `self._transport.auth_publickey(username, key)`
- `AuthHandler._request_auth()`

## 使用建议

- 生产脚本中尽量不要长期使用 `AutoAddPolicy`, 应显式管理主机指纹.
- 当 `Channel` 的数据没有及时读取时, 服务端可能暂停继续发送.
- 当服务端没有及时消费你发送的数据时, 写入也可能阻塞, 因此超时和流控都很重要.
- 若后续继续整理, 可补 `SFTP`, 跳板机, 长连接复用和异常重试示例.
