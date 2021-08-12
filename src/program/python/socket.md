# socket 笔记

    socket.getaddrinfo(host, port, family=0, type=0, proto=0, flags=0)
        返回一个元组:
            (family, type, proto, canonname, sockaddr)

        example:
            根据主机名 端口等信息, 获取主机的详细信息

            >>> socket.getaddrinfo("example.org", 80, proto=socket.IPPROTO_TCP)
            [(<AddressFamily.AF_INET6: 10>, <SocketType.SOCK_STREAM: 1>,
                6, '', ('2606:2800:220:1:248:1893:25c8:1946', 80, 0, 0)),
                (<AddressFamily.AF_INET: 2>, <SocketType.SOCK_STREAM: 1>,
                6, '', ('11.11.11.11', 80))]

            sock = socket.socket(addressfamily, sockettype)
            sock.connect(('11.11.11.11', 80))

    