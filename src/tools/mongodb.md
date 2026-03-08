# MongoDB

## 说明

- 本页记录 MongoDB 在 Windows 和 Docker 场景下的常见运维操作.
- 当前重点包括密码重置, 容器启动和本地副本集初始化.

## Windows 重置密码

```powershell
# 如果服务已启动, 先关闭服务
net stop MongoDB

# 以无密码方式运行 MongoDB 服务
C:\Program Files\MongoDB\Server\8.0\bin\mongod.exe --dbpath C:\Program Files\MongoDB\Server\8.0\data --port 27017 --noauth

cd C:\Program Files\MongoDB\Server\8.0\bin
.\mongod.exe --config mongod.cfg --noauth

# 启动客户端
C:\Program Files\MongoDB\Server\8.0\bin\mongosh.exe
```

在客户端中创建管理员用户:

```javascript
use admin
db.createUser({
  user: "admin",
  pwd: "你现在想设的新密码666",
  roles: [{ role: "root", db: "admin" }]
})

show dbs
```

## Docker 运行 MongoDB

```sh
docker run -d --name mongodb \
  -v D:/docker/mongodb/data:/data/db \
  -v D:/docker/mongodb/configdb:/data/configdb \
  -p 27017:27017 \
  -e MONGODB_INITDB_ROOT_USERNAME=root \
  -e MONGODB_INITDB_ROOT_PASSWORD=mx123456 \
  --restart=always \
  mongodb/mongodb-community-server:7.0.15-ubuntu2204
```

## Windows 创建副本集

安装 `mongodb` 与 `mongosh` 后, 修改:

```text
C:\Program Files\MongoDB\Server\8.0\bin\mongod.cfg
```

配置两处关键项:

```cfg
bindIp: 0.0.0.0

replication:
  replSetName: rs0
```

重启 MongoDB 服务后, 打开 `mongosh.exe` 并检查:

```javascript
rs.status()
```

如果提示 `no replset config has been received`, 执行初始化:

```javascript
rs.initiate({_id:"rs0",members:[{_id:0,host:"localhost:27017"}]})
```

也可顺手创建管理员用户:

```javascript
use admin
db.createUser({ user: 'root', pwd: '111111', roles: [{ role: "root", db: "admin" }] });
```

## 使用建议

- Windows 下做密码重置时, 先确认当前服务确实已经停掉, 避免端口冲突.
- 初始化副本集前, 先把 `bindIp` 和 `replSetName` 配对设置好.
- Docker 场景中, 数据卷路径和认证信息最好提取到更稳定的部署脚本中.
