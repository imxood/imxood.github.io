# Linux 常用命令

## 说明

- 本页整理 Linux 日常排查和下载场景里最常用的一批命令.
- 当前以 `curl`, `grep`, `find`, `tar`, `ss`, `ps`, `chmod` 为主, 适合快速速查.
- 若问题已经升级为系统层排障, 建议继续看 [Linux 综合笔记](./Linux笔记.md).

## 网络与下载

### `curl`

```sh
curl -v URL
curl -i URL
curl -L URL
curl -I URL
curl -L -o file.zip https://example.com/file.zip
```

常见含义:

- `-v`: 输出详细请求和响应过程.
- `-i`: 返回响应头和响应体.
- `-L`: 自动跟随重定向.
- `-I`: 只请求响应头.
- `-o`: 指定输出文件名.

常见场景:

```sh
curl -I https://example.com
curl -X POST https://example.com/api -H "Content-Type: application/json" -d '{"name":"demo"}'
```

## 文本与文件查找

### `grep`

```sh
grep -R "keyword" .
grep -n "keyword" file.txt
grep -RIn "keyword" src/
```

- 适合查日志, 配置和源码中的关键字.

### `find`

```sh
find . -name "*.log"
find . -type f -mtime -1
find . -type d -name build
```

- 适合按名称, 类型和修改时间筛选文件.

## 压缩与归档

### `tar`

```sh
tar -czf archive.tar.gz dir/
tar -xzf archive.tar.gz
tar -tf archive.tar.gz
```

- 打包, 解压和查看压缩包内容都很常见.

## 进程与端口

### `ps`

```sh
ps aux | grep python
```

- 用于查看进程是否在运行.

### `ss`

```sh
ss -tunlp
ss -lnt
```

- 适合查看端口监听和网络连接状态.
- 排查服务不通时通常比老的 `netstat` 更直接.

## 权限与执行

### `chmod`

```sh
chmod +x script.sh
chmod 644 file.txt
chmod -R 755 dir/
```

- 常用于脚本执行权限和基础文件权限调整.
- 生产环境里不要对大目录粗暴 `777`.

## 使用建议

- 排查接口问题时, 先用 `curl -v` 或 `curl -I` 看响应状态和重定向.
- 查日志和源码时, `grep` 与 `find` 常常一起使用.
- 涉及端口问题时, 优先看 `ss -tunlp`.
- 更系统的服务管理和防火墙命令, 可继续看 [Linux 综合笔记](./Linux笔记.md).

## 相关文档

- [Linux 工具](./linux工具.md)
- [Linux 常见问题](./issues.md)
- [Linux 综合笔记](./Linux笔记.md)
