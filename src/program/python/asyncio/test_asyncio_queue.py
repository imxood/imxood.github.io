import asyncio
import time
import logging
from pprint import pprint
from multiprocessing import Process
from socketserver import UDPServer, BaseRequestHandler
from socket import socket, AF_INET, SOCK_DGRAM


logging.basicConfig(level=logging.DEBUG,
                    format='[%(name)s %(levelname)s] %(message)s', datefmt='%H:%M:%S')


class UDPRequestHandler(BaseRequestHandler):

    def handle(self):
        print('Got connection from', self.client_address)
        msg, sock = self.request
        print('from %s -- recv msg: %s', self.client_address, msg)
        sock.sendto(b'I\'m server', self.client_address)


def UdpServer():
    server = UDPServer(('', 10001), UDPRequestHandler)
    server.serve_forever()


async def queue_task(queue):
    while True:
        try:
            msg = await queue.get()
            print('queue_task queu recv:', msg)
            queue.task_done()
            break
        except asyncio.CancelledError:
            print('queue task is canceled!')


async def application(queue):
    try:
        s = socket(AF_INET, SOCK_DGRAM)
        serv_addr = ('127.0.0.1', 10001)
        a = s.sendto(b'Who are you?', serv_addr)
        raise IOError
        await queue.put(s.recv(1024).decode())
        await queue.join()
    except IOError:
        logging.error('application throw IOError!')
    # while True:
    #     s.recv()


async def broker(queue):
    await application(queue)


async def broker1(queue):
    await broker(queue)


async def broker2(queue):
    await broker1(queue)


async def broker3(queue):
    await broker2(queue)


async def main():
    queue = asyncio.Queue()
    queue_future = asyncio.ensure_future(queue_task(queue))
    app_future = asyncio.ensure_future(broker(queue))
    await asyncio.gather(queue_future, app_future)
    logging.info('main ok')


if __name__ == '__main__':
    # proc = Process(target=UdpServer)
    # proc.start()
    # time.sleep(1)

    loop = asyncio.get_event_loop()
    loop.run_until_complete(main())
    loop.close()

    # asyncio.run(main())

    # proc.terminate()
