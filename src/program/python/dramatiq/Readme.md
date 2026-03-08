# Dramatiq

## 说明

- `Dramatiq` 是一个偏轻量的 Python 后台任务处理框架.
- 适合做异步任务分发, 延迟执行和简单队列消费场景.

## 适用场景

- Web 应用中的异步邮件, 消息通知和批处理任务.
- 需要基于 `Redis` 或其他 broker 做轻量任务队列.
- 希望比重量级任务系统更快上手的后台作业处理方案.

## 项目

- [dramatiq 项目](https://github.com/Bogdanp/dramatiq)

## 安装 Redis

```sh
sudo apt install redis
```

## 安装 Dramatiq

```sh
pip install --user dramatiq[redis,watch]
```

## Redis 配置为外网可访问

编辑 `/etc/redis/redis.conf`:

- 注释掉: `bind 127.0.0.1`

重启服务:

```sh
sudo systemctl restart redis.service
```

## 使用建议

- 如果用于生产环境, 还需要继续补鉴权, 防火墙和网络隔离配置.
- 开发阶段可以先在本机跑最小 worker 验证消息收发, 再考虑部署拓扑.
- 若任务复杂度继续增长, 建议同时规划重试策略, 监控和任务幂等性.
