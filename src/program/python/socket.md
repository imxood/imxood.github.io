# socket 笔记

## 说明

- 本页整理 Python `socket` 标准库中最常见的地址解析与 TCP 连接用法.
- 适合做网络诊断工具, 简单客户端, 端口探测和协议实验.
- 若只是做高层 HTTP 请求, 通常优先使用更上层库, 不必直接从 `socket` 开始.

## 地址解析: `socket.getaddrinfo`

```python
socket.getaddrinfo(host, port, family=0, type=0, proto=0, flags=0)
```

返回值中的每一项通常是:

```python
(family, type, proto, canonname, sockaddr)
```

典型示例:

```python
import socket

infos = socket.getaddrinfo(
    "example.org",
    80,
    proto=socket.IPPROTO_TCP,
)

for family, socktype, proto, canonname, sockaddr in infos:
    print(family, socktype, proto, canonname, sockaddr)
```

## 这个函数适合做什么

- 根据主机名和端口解析出可连接的地址列表.
- 同时兼容 IPv4 和 IPv6.
- 在自己实现连接逻辑时, 可以先拿到所有候选地址再逐个尝试.

## 最小 TCP 客户端示例

```python
import socket

infos = socket.getaddrinfo(
    "example.org",
    80,
    type=socket.SOCK_STREAM,
    proto=socket.IPPROTO_TCP,
)

family, socktype, proto, _, sockaddr = infos[0]

with socket.socket(family, socktype, proto) as sock:
    sock.settimeout(5)
    sock.connect(sockaddr)
    sock.sendall(b"GET / HTTP/1.0
Host: example.org

")
    data = sock.recv(4096)
    print(data.decode("utf-8", errors="replace"))
```

## 更简单的连接方式

```python
import socket

with socket.create_connection(("example.org", 80), timeout=5) as sock:
    sock.sendall(b"GET / HTTP/1.0
Host: example.org

")
    print(sock.recv(4096))
```

- 对简单 TCP 客户端来说, `socket.create_connection` 往往更直接.
- 它会自动处理一部分地址解析和连接尝试逻辑.

## 常见概念

### `family`

- 常见值包括 `AF_INET` 和 `AF_INET6`.
- 分别对应 IPv4 和 IPv6.

### `type`

- `SOCK_STREAM` 通常表示 TCP.
- `SOCK_DGRAM` 通常表示 UDP.

### `sockaddr`

- 是真正拿来传给 `connect` 的地址信息.
- IPv4 和 IPv6 对应的元组结构略有差异.

## 使用建议

- 做网络探测时, 建议显式设置超时, 例如 `settimeout(5)`.
- 需要同时兼容 IPv4 / IPv6 时, 优先通过 `getaddrinfo` 获取候选地址.
- 如果只是实现简单客户端, 优先考虑 `socket.create_connection`.
- 若要写长期稳定的协议服务, 应再补充异常处理, 重试, 超时和资源清理策略.

## 常见问题

### `connect` 卡住很久

- 多半是没有设置超时.
- 也可能是 DNS 解析慢, 网络被代理或防火墙阻断.

### 地址解析结果很多

- 这是正常现象, 尤其是同时返回 IPv4 和 IPv6 时.
- 可以按需求优先选择某个 `family`.

### 为什么不用更高层库

- 原始 `socket` 更适合做协议学习, 底层排查和轻量实验.
- 业务开发里若协议已经成熟, 通常优先选择更高层封装.

## 相关文档

- [Python 总览](./README.md)
- [创建进程](./process.md)
