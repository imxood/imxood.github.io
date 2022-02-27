import asyncio
import time
from pprint import pprint
from multiprocessing import Process
from socketserver import UDPServer, BaseRequestHandler


class UDPRequestHandler(BaseRequestHandler):

    def handle(self):
        print('Got connection from', self.client_address)
        msg, sock = self.request
        print('from %s -- recv msg: %s', self.client_address, msg)
        sock.sendto(b'I\'m server', self.client_address)


def UdpServer():
    server = UDPServer(('', 10001), UDPRequestHandler)
    server.serve_forever()


if __name__ == '__main__':
    proc = Process(target=UdpServer)
    proc.start()
    proc.join()
