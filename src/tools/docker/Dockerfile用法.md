# Dockerfile 用法

## 说明

- 本页记录 `Dockerfile` 中 `ENTRYPOINT` 和 `CMD` 的职责边界.
- 这两者最容易混淆, 但理解后写镜像启动逻辑会清晰很多.

## `ENTRYPOINT`

- 常用于定义容器启动后的固定主命令.
- 适合那些“镜像的职责就是运行这个程序”的场景.

## `CMD`

- 常用于给启动命令提供默认参数.
- 运行 `docker run ...` 时, 可以用额外参数覆盖它.

## 典型理解

- `ENTRYPOINT` 决定“执行谁”.
- `CMD` 决定“默认带什么参数”.
- `docker run <image> ...` 往往更适合用于替换或补充 `CMD`.

## 组合示例

```dockerfile
ENTRYPOINT ["python", "app.py"]
CMD ["--host", "0.0.0.0", "--port", "8000"]
```

上面的镜像默认会执行:

```sh
python app.py --host 0.0.0.0 --port 8000
```

## 使用建议

- 固定应用入口时优先用 `ENTRYPOINT`.
- 需要给用户留默认参数覆写空间时再配合 `CMD`.
- 写生产镜像时, 还应继续关注基础镜像选择, 层数控制和缓存命中率.
- 若后续继续整理, 可补 `COPY`, `RUN`, `WORKDIR`, 多阶段构建和镜像瘦身记录.
