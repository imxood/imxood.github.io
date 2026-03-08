# docker-compose 用法

## 说明

- 本页记录 `docker-compose` 的常见启动方式和高频命令.
- 当前内容以单机本地开发和部署排查中的最小操作为主.

## 指定一个 `docker-compose.yml` 运行

```sh
docker-compose -f ./docker-compose.yml up
```

## 常用命令

```sh
docker-compose up
docker-compose up -d
docker-compose down
docker-compose logs -f
docker-compose ps
docker-compose exec <service> sh
```

## 常见使用场景

### 后台启动服务

```sh
docker-compose up -d
```

### 查看某个服务日志

```sh
docker-compose logs -f <service>
```

### 进入容器排查问题

```sh
docker-compose exec <service> sh
```

## 使用提示

- `up -d` 适合后台启动服务.
- `down` 会停止并移除当前 compose 创建的容器和网络.
- 如果项目同时维护多份配置, 可以继续叠加多个 `-f` 参数组合环境.
- 新环境第一次启动前, 建议先检查镜像版本, 端口和挂载目录.

## 相关页面

- [Docker 总览](./README.md)
- [Dockerfile 用法](./Dockerfile用法.md)
