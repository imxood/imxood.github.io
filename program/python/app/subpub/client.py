import struct
from queue import Queue, Empty
from threading import Event, Thread
import time
import socket
import select

hosts = dict()

hosts['server'] = {'name': 'server', 'host': '127.0.0.1', 'port': 10000}
hosts['host1'] = {'name': 'host_1', 'host': '127.0.0.1', 'port': 10001}


def get_ip_addr():
    try:
        s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        s.connect(('8.8.8.8', 80))
        ip = s.getsockname()[0]
    finally:
        s.close()
    return ip


class Client:

    def __init__(self):
        self.queue = Queue()
        d = Thread(target=self.run)
        d.start()

    def __send_msg(self, sock: socket.socket, data):
        length = sock.send(struct.pack('H', len(data)))
        print("\tsend {} bytes".format(length))
        length = sock.send(data.encode())
        print("\tsend {} bytes".format(length))

    def close(self):
        self.exit = Event()

    def run(self):
        client_sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        server_address = (hosts['server']['host'], hosts['server']['port'])
        client_sock.connect(server_address)
        client_sock.setblocking(0)

        try:
            while True:
                msg = self.queue.get()
                self.__send_msg(client_sock, msg)
                # if len(msg) != length:
                #     print('\tsend msg failed, length is wrong')
                #     exit(-1)
                data = client_sock.recv(2)
                length = struct.unpack('H', data.decode())[0]
                print('\t received data, len: {}, data: {}'.format(2, length))
                data = client_sock.recv(length)
                data = data.decode()
                print('\t received data, len: {}, data: {}'.format(len(data), data))
        except Exception:
            pass

if __name__ == "__main__":
    client = Client()

    while True:
        client.queue.put('Hello,')
        client.queue.put('World')
        time.sleep(5)
