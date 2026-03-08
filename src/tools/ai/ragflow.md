# RAGFlow

## 说明

- 本页记录 `RAGFlow` 的本地 Docker 部署流程.
- 适合先快速跑起一个可访问的 RAG 服务实例, 再逐步接入模型和知识库.

## 使用 Docker 部署

```sh
git clone https://github.com/infiniflow/ragflow.git
cd ragflow/docker

docker compose -f docker-compose-CN.yml up -d

docker logs -f ragflow-server
```

部署后通过浏览器访问:

```text
http://127.0.0.1
```

## 最小使用流程

1. 先确认 Docker 与 `docker compose` 可正常使用.
2. 启动服务并观察关键容器日志.
3. 能打开 Web 页面后, 再配置模型与嵌入服务.
4. 最后再做文档导入, 切片和检索测试.

## 常见关注点

- 启动前先确认 Docker 和端口占用情况.
- 若在国内网络环境下部署, `docker-compose-CN.yml` 往往更方便.
- 真正用于知识库实验时, 还需要继续补模型接入, 文档导入和检索参数配置.
- 若页面可打开但问答不可用, 优先检查模型接口, 向量化服务和存储组件状态.

## 后续可补主题

- 模型供应商和 API 配置.
- 文档切片策略.
- 检索参数与召回调优.
- 用户权限和持久化目录说明.
