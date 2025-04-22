## 过滤

过滤指定端口

    tcp.port == 8080
    tcp.port == 8080
    tcp.srcport == 8080
    tcp.srcport == 8080
    tcp.dstport == 8080
    tcp.port == 8080 or udp.port == 8080
    tcp.port >= 1000 and tcp.port <= 2000
    tcp.port in {80 443}

http and tcp.port == 8080