# Wireshark

## 说明

- `Wireshark` 适合做抓包分析, 协议排查和网络链路定位.
- 常见误区是把“捕获过滤器”和“显示过滤器”混用, 导致抓不到包或过滤结果异常.

## 过滤指定端口

```text
tcp.port == 8080
tcp.srcport == 8080
tcp.dstport == 8080
tcp.port == 8080 or udp.port == 8080
tcp.port >= 1000 and tcp.port <= 2000
tcp.port in {80 443}
http and tcp.port == 8080
```

## 常见过滤条件

### 地址过滤

```text
ip.addr == 192.168.1.10
ip.src == 192.168.1.10
ip.dst == 192.168.1.10
```

### 协议过滤

```text
dns
http
tls
arp
icmp
```

### 内容检索

```text
frame contains "error"
http.request.method == "POST"
dns.qry.name contains "example"
```

## 抓包排查流程

1. 先确认抓包网卡和抓包时机是否正确.
2. 抓包阶段尽量少用过严的捕获过滤器, 先把流量保留下来.
3. 进入分析阶段后, 再用显示过滤器逐步收敛到目标连接.
4. 先看 DNS, TCP 三次握手, TLS 握手, 再看应用层请求和响应.
5. 若怀疑重传, 超时或 MTU 问题, 继续关注 `tcp.analysis.*` 相关字段.

## 使用建议

- 先区分“捕获过滤器”和“显示过滤器”, 避免抓包阶段就把流量漏掉.
- 排查单个服务时, 优先从端口过滤开始, 再叠加 `http`, `dns`, `tls` 等协议条件.
- 分析 HTTPS 问题时, 若无法解密应用层内容, 至少也能先观察连接建立, 证书交换和重传行为.
- 对比正常与异常抓包, 往往比单独看一份抓包更容易定位问题.
