# OpenResty

## 说明

- 本页记录 `OpenResty` / `Nginx` 场景下一个常见配置: 上传文件大小限制.
- 适合排查大文件上传时报 `413 Request Entity Too Large` 一类问题.

## `client_max_body_size`

可在 `server` 或更高层级中设置上传文件的最大尺寸:

```nginx
server {
    listen 80;
    listen 443 ssl;
    listen 443 quic;
    server_name auto.iotim.com;

    client_max_body_size 200m;

    ...
}
```

## 使用建议

- 该值应根据业务实际大小设置, 例如 `50m`, `100m`, `200m`.
- 若通过反向代理转发到后端服务, 还要同时确认后端服务本身的请求体大小限制.
- 修改后记得执行配置检查并重载服务.
- 如果上传链路很长, 还要一起检查超时配置和临时目录空间.

## 常见命令

```sh
nginx -t
nginx -s reload
```

## 适合排查的场景

- 文件上传直接返回 `413`.
- 前端上传中断, 但后端日志没有进入业务处理.
- 反向代理层和应用层对最大请求体限制不一致.

## 后续可补主题

- 反向代理基础配置.
- `location` 匹配和转发规则.
- `Lua` 扩展与动态路由.
- HTTPS 与证书管理笔记.
