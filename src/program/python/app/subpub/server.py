from queue import Queue, Empty
from threading import Event, Thread
import time
import socket
import json
import struct

clients = dict()

clients['server'] = {'host': '127.0.0.1', 'port': 10000}
clients['host1'] = {'host': '127.0.0.1', 'port': 10001}
clients['host2'] = {'host': '127.0.0.1', 'port': 10002}


def get_ip_addr():
    try:
        s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        s.connect(('8.8.8.8', 80))
        ip = s.getsockname()[0]
    finally:
        s.close()
    return ip

class Client:

    def __init__(self, sock : socket.socket):
        self.sock : socket.socket = sock

    def send(self, command):
        command = json.dumps(command).encoe()
        if self.sock.send(struct.pack('H', len(command))) == 2:
            if self.sock.send(command) == len(command):
                print('\tsend successed')
                return
        print('\tsend failed')

    def recv(self):
        data = self.sock.recv(2)
        if data:
            length = struct.unpack('H', data)[0]
            data = self.sock.recv(length)
            if len(data) == length:
                print('\trecv successed')
                return data
        print('\trecv failed')


class Server:

    def __init__(self, server_address):
        server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        server.setblocking(0)
        server.bind(server_address)
        server.listen(5)

        self.clients = dict()

        print('\tserver listening on ', server_address)

        while True:
            client_sock, client_address = server.accept()
            client_sock.setblocking(0)

            client = Client(client_sock)

            self.clients[client_address] = client

            client



        inputs = [server]
        outputs = []
        message_queues = {}

        while inputs:

            readable, writable, exceptional = select.select(inputs, outputs, inputs)

            for s in readable:

                if s is server:

                    inputs.append(connection)
                    message_queues[connection] = Queue()
                    print('\tconnection from ', client_address)
                else:
                    data = s.recv(1024)
                    if data:
                        print('\treceived: ', data)
                        if s not in outputs:
                            outputs.append(s)
                    else:
                        print('\tclosing ', s.getpeername())
                        if s in outputs:
                            outputs.remove(s)
                        inputs.remove(s)
                        s.close()
                        del message_queues[s]

            for s in writable:
                message_queue = message_queues[s]
                next_msg = ''
                try:
                    if message_queue:
                        next_msg = message_queue.get_nowait()
                except Empty:
                    print('\t{} queue empty'.format(s.getpeername()))
                    outputs.remove(s)
                else:
                    print('\tsending {!r} to {}'.format(next_msg, s.getpeername()))
                    s.send(next_msg)

            for s in exceptional:
                print('\texception on ', s.getpeername())
                inputs.remove(s)
                if s in outputs:
                    outputs.remove(s)
                del message_queues[s]

if __name__ == "__main__":
    server_address = (clients['server']['host'], clients['server']['port'])
    Server(server_address)
