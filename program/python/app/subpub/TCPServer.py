import select
import socket
import struct

hosts = dict()

hosts['server'] = {'host': '127.0.0.1', 'port': 10000}


class EventHandler:

    def fileno(self):
        raise NotImplementedError('Must implement')

    def wants_to_receive(self):
        return False

    def handle_receive(self):
        pass

    def wants_to_send(self):
        return False

    def handle_send(self):
        pass


class TCPClient(EventHandler):

    def __init__(self, sock, handler_list):
        self.sock: socket.socket = sock
        self.outgoing: bytearray = bytearray()
        self.handler_list: list = handler_list

    def fileno(self):
        return self.sock.fileno()

    def wants_to_receive(self):
        return True

    def handle_receive(self):
        data = self.sock.recv(2)
        if data:
            nrecv = struct.unpack('H', data)[0]
            data = self.sock.recv(nrecv)
            print('data: ', data)


    def wants_to_send(self):
        return True if len(self.outgoing) else False

    def handle_send(self):
        nsent = self.sock.send(self.outgoing)
        self.outgoing = self.outgoing[nsent:]

    def close(self):
        self.handler_list.remove(self)


class TCPServer(EventHandler):

    def __init__(self, server_address, client_handler: TCPClient, handler_list):

        self.sock: socket.socket = socket.socket(
            socket.AF_INET, socket.SOCK_STREAM)

        self.sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, True)
        self.sock.bind(server_address)
        self.sock.listen(1)
        self.client_handler = client_handler
        self.handler_list: list = handler_list

    def fileno(self):
        return self.sock.fileno()

    def wants_to_receive(self):
        return True

    def handle_receive(self):
        client, addr = self.sock.accept()
        self.handler_list.append(
            self.client_handler(client, self.handler_list))
        print('\t new connection: ', addr)


def event_loop(handlers):

    while True:

        wants_recv = [handler for handler in handlers if handler.wants_to_receive()]

        wants_send = [handler for handler in handlers if handler.wants_to_send()]

        readable, writable, _ = select.select(wants_recv, wants_send, [])

        for h in readable:
            h.handle_receive()

        for h in writable:
            h.handle_send()


if __name__ == "__main__":

    handlers = []

    handlers.append(TCPServer(
        (hosts['server']['host'], hosts['server']['port']), TCPClient, handlers))

    event_loop(handlers)
