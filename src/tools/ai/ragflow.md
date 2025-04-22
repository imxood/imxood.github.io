## 使用docker 部署 ragflow

```sh
git clone https://github.com/infiniflow/ragflow.git

cd ragflow/docker

# 构建 并 运行
docker compose -f docker-compose-CN.yml up -d

# 查看运行状态
docker logs -f ragflow-server

# 浏览器访问 http://127.0.0.1
```
