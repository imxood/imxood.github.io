# libp2p relay 示例运行记录

## 说明

- 本页记录 `libp2p-learn` 示例中 relay 场景的最小启动顺序.
- 适合本地或双机验证中继转发流程.
- 这是一份“运行记录页”, 更适合保留命令和实验顺序, 不强行扩写为完整教程.

## 实验角色

- `relay`: 提供中继能力的节点.
- `listener`: 通过 relay 暴露可被拨号的节点.
- `dialer`: 通过 relay 主动连接到 listener 的节点.

## 启动顺序

### 1. 启动 relay

```sh
cargo run --bin relay_v2 -- --port 4001 --secret-key-seed 0
```

### 2. 启动监听端

```sh
RUST_LOG=info cargo run --bin client -- --secret-key-seed 1 --mode listen --relay-address /ip4/$SERVER_IP/tcp/4001/p2p/12D3KooWDpJ7As7BWAwRMfu1VU2WCqNjvq387JEYKDBj4kx6nXTN
```

### 3. 启动拨号端

```sh
RUST_LOG=info cargo run --bin client -- --secret-key-seed 2 --mode dial --relay-address /ip4/$SERVER_IP/tcp/4001/p2p/12D3KooWDpJ7As7BWAwRMfu1VU2WCqNjvq387JEYKDBj4kx6nXTN --remote-peer-id 12D3KooWPjceQrSwdWXPyLLeABRXmuqt69Rg3sBYbU1Nft9HyQ6X
```

## 使用提示

- 先启动 relay, 再启动 listener, 最后启动 dialer.
- 运行前把 `$SERVER_IP` 替换为实际中继主机地址.
- 示例中的 `PeerId` 是一次记录, 如果重新生成密钥, 需要同步更新命令.

## 排查建议

- 如果无法建立连接, 先确认 relay 节点端口是否真正监听.
- 再确认 `PeerId` 与密钥种子是否对应.
- 若是跨机器测试, 还要检查公网地址, 防火墙和 NAT 环境.

## 归档说明

- 本页保留的是“命令顺序 + 实验参数”信息.
- 若后续需要系统化整理 libp2p 知识, 应在 `program/rust/` 下单独建立专题页.
