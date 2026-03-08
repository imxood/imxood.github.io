# Docker 总览

## 说明

- 本目录收录 Docker 基础笔记, Dockerfile / docker-compose 速查, 以及若干历史部署配置.
- 如果是日常镜像, 容器, 卷, 网络问题, 优先查看 [Docker 基础与技巧](./docker笔记.md).
- 如果目标是回看某个旧部署目录, 再进入 `gitlab/`, `service/`, `esp32/`, `idf/` 等子目录.

## 常用入口

- [Docker 基础与技巧](./docker笔记.md)
- [Dockerfile 用法](./Dockerfile用法.md)
- [docker-compose 用法](./docker-compose用法.md)
- [部署自动化环境](./部署自动化环境.md)

## 阅读路径建议

1. 刚接触 Docker 时, 先看 `Docker 基础与技巧`.
2. 需要写镜像时, 再看 `Dockerfile 用法`.
3. 需要编排多容器服务时, 再看 `docker-compose 用法`.
4. 如果只是回看历史部署组合, 再进入 `gitlab/` 和 `service/` 这些归档目录.

## 配置归档

- [GitLab 与 Taiga 部署记录](./gitlab/Readme.md)
- [服务容器记录](./service/Readme.md)
- `esp32/`: ESP32 镜像与 compose 配置.
- `idf/`: ESP-IDF 开发环境容器配置.
- `zbox/`: 历史留存目录, 当前未形成正式文档.

## 使用建议

- 当前目录里有一批“可运行配置”和一批“知识笔记”, 使用时注意区分.
- 对于历史 compose 文件, 建议先检查镜像版本, 端口占用和宿主机挂载路径, 不要直接照搬到新环境.
- 若文档内容继续增多, 后续可把“通用知识”和“历史部署样例”进一步拆层管理.
