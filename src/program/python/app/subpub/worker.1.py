from queue import Queue, Empty
from threading import Event, Thread
import time
import socket
import select

clients = dict()

port = 20000

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


class WorkerExit(Exception):
    pass


class Action:
    def __init__(self, name, cmd, description):
        self.name = name
        self.cmd = cmd
        self.description = description

    def run(self):
        status = 0
        stdout = ''
        stderr = ''
        return (status, stdout, stderr)


class Test1Action(Action):

    def __init__(self):
        super().__init__('test1', 'ls -al', 'test1')

    def run(self):
        print('run test1.')
        return (0, 'ok', '')


class Test2Action(Action):

    def __init__(self):
        super().__init__('test2', 'ls -al', 'test2')

    def run(self):
        print('run test2.')
        return (0, 'ok', '')


class Task:
    def __init__(self, name, actions: list):
        self.name = name
        self.actions: list = actions
        self.status = -1
        self.exit = 0

    def stop(self):
        self.__exit = 1

    def start(self):
        d = Thread(target=self.__run)
        d.start()

    def __run(self):
        for action in self.actions:
            if self.exit:
                self.status = -2
                return
            action.run()


class TaskStatus:
    def __init__(self, is_ok=0):
        self.status = -1


class Worker:
    def __init__(self):
        self.task_queue: Queue = Queue()
        self.tasks: list = list()
        self.clients: dict = clients
        self.task_process = None

    def add_task(self, task: Task):
        self.tasks.append(task)

    def list_task(self):
        return [{'name': task.name, 'cmd': task.cmd, 'description': task.description} for task in self.tasks]

    def start(self):
        self.terminated = Event()
        t = Thread(target=self.__bootstrap)
        t.daemon = True
        t.start()

        s = Thread(target=self.__subscribe)
        s.daemon = True
        s.start()

    def close(self):
        self.task_queue.put(WorkerExit)

    def join(self):
        self.terminated.wait()

    def __bootstrap(self):
        try:
            self.run()
        except WorkerExit:
            pass
        finally:
            self.terminated.set()

    def __subscribe(self):
        server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        server.setblocking(0)
        server_address = (clients['server']['host'], clients['server']['port'])
        server.bind(server_address)
        server.listen(5)

        inputs = [server]
        outputs = []
        message_queues = {}

        print('    listening on {} {}'.format(server_address[0], server_address[1]))

        while inputs:

            readable, writable, exceptional = select.select(
                inputs, outputs, inputs)

            for s in readable:

                if s is server:
                    connection, client_address = s.accept()
                    connection.setblocking(0)
                    inputs.append(connection)
                    message_queues[connection] = Queue()
                    print('    connection from ', client_address)
                else:
                    data = s.recv(1024)
                    if data:
                        print('    received: ', data)
                        if s not in outputs:
                            outputs.append(s)
                    else:
                        print('    closing ', s.getpeername())
                        if s in outputs:
                            outputs.remove(s)
                        inputs.remove(s)
                        s.close()
                        del message_queues[s]

            for s in writable:
                try:
                    next_msg = message_queues[s].get_nowait()
                except Empty:
                    print('    error: {} msg queue empty'.format(s.getpeername()))
                else:
                    print('    sending {!r} to {}'.format(
                        next_msg, s.getpeername()))
                    s.send(next_msg)

            for s in exceptional:
                print('    exception on ', s.getpeername())
                inputs.remove(s)
                if s in outputs:
                    outputs.remove(s)
                del message_queues[s]

    def run(self):
        while True:
            task = self.task_queue.get()
            if task is WorkerExit:
                raise WorkerExit()
            ret = task.run()
            if ret[0] == 0:
                print('task "{}" successed'.format(task.name))
            else:
                print('task "{}" failed'.format(task.name))


print(get_ip_addr())

worker = Worker()

worker.add_task(Test1Action())
worker.add_task(Test2Action())

worker.start()

time.sleep(5)
print(worker.list_task())

worker.close()

worker.join()
