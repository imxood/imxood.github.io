## paramiko python的ssh客户端

    from paramiko import SSHClient, AutoAddPolicy

    connect_kwargs = dict(
        hostname=hostname,
        port=port,
        username=username,
        timeout=timeout,
    )
    if password:
        connect_kwargs['password'] = password

    client = SSHClient()
    client.set_missing_host_key_policy(AutoAddPolicy())

    client.connect(**connect_kwargs)
    transport = client.get_transport()

    # open channel
    channel = transport.open_session()

    # set exec_command's timeout
    channel.settimeout(timeout)

    # set stderr --> stdout
    channel.set_combine_stderr(True)

    # exec command
    channel.exec_command('ls')

    ... channel.recv ...


## 源码简单的分析

    在一个client中, 会有一个Transport作为一个传输线程
        
        Transport有一个packetizer对象
            packetizer拥有远程的socket, 并进行收发数据
        
        分析认证过程:
            client.connect:
                self._transport.auth_publickey(username, key)

    Transport是作为数据传输的载体

    更高级的对象是Channel, 多个Channel依赖单一的Transport, 每一个Channel有自己的 flow control,
        当一个Channel的数据太多没有被读取, 那么 ssh server 将不会给这个Channel发数据
        同样地, 当 Server 没有读完你发的数据, 那么你的发送操作可能会被阻塞, 除非你设置一个超时时间


    class Transport(threading.Thread, ClosingContextManager)

        # 由于ChannelMap是全局资源, 所以需要实现自己的ChannelMap, 并用lock住对全局资源的访问

        def __init__(...):
            # tracking open channels
            self._channels = ChannelMap()
            self.channel_events = {}  # (id -> Event)
            self.channels_seen = {}  # (id -> True)
            self._channel_counter = 0

        def _send_message(self, data):
            self.packetizer.send_message(data)

        def auth_publickey(self, username, key, event=None):
            if event is None:
                my_event = threading.Event()
            else:
                my_event = event
            self.auth_handler = AuthHandler(self)
            self.auth_handler.auth_publickey(username, key, my_event)
            if event is not None:
                # caller wants to wait for event themselves
                return []
            return self.auth_handler.wait_for_response(my_event)

        def open_channel(..:
            chanid = self._next_channel()
            m = Message()
            m.add_byte(cMSG_CHANNEL_OPEN)

    class AuthHandler(object):

        def auth_publickey(self, username, key, event):
            self.transport.lock.acquire()
            try:
                self.auth_event = event
                self.auth_method = "publickey"
                self.username = username
                self.private_key = key
                self._request_auth()
            finally:
                self.transport.lock.release()

        def _request_auth(self):
            m = Message()
            m.add_byte(cMSG_SERVICE_REQUEST)
            m.add_string("ssh-userauth")
            self.transport._send_message(m)
        
        def wait_for_response(self, event):
            max_ts = None
            if self.transport.auth_timeout is not None:
                max_ts = time.time() + self.transport.auth_timeout
            while True:
                event.wait(0.1)
                if not self.transport.is_active():
                    e = self.transport.get_exception()
                    if (e is None) or issubclass(e.__class__, EOFError):
                        e = AuthenticationException("Authentication failed.")
                    raise e
                if event.is_set():
                    break

    class Channel(ClosingContextManager):
        
        def exec_command(self, command):
            m = Message()
            m.add_byte(cMSG_CHANNEL_REQUEST)
            m.add_int(self.remote_chanid)
            m.add_string("exec")
            m.add_boolean(True)
            m.add_string(command)
            self._event_pending()
            self.transport._send_user_message(m)
            self._wait_for_event()
        
        def _wait_for_event(self):
            self.event.wait()
            assert self.event.is_set()
            if self.event_ready:
                return
            e = self.transport.get_exception()
            if e is None:
                e = SSHException("Channel closed.")
            raise e
    
    class ChannelMap(object):
        def __init__(self):
            # (id -> Channel)
            self._map = weakref.WeakValueDictionary()
            self._lock = threading.Lock()

        def put(self, chanid, chan):
            self._lock.acquire()
            try:
                self._map[chanid] = chan
            finally:
                self._lock.release()

        def get(self, chanid):
            self._lock.acquire()
            try:
                return self._map.get(chanid, None)
            finally:
                self._lock.release()

        def delete(self, chanid):
            self._lock.acquire()
            try:
                try:
                    del self._map[chanid]
                except KeyError:
                    pass
            finally:
                self._lock.release()

        def values(self):
            self._lock.acquire()
            try:
                return list(self._map.values())
            finally:
                self._lock.release()

        def __len__(self):
            self._lock.acquire()
            try:
                return len(self._map)
            finally:
                self._lock.release()
