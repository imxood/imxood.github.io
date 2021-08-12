from collections import defaultdict
from contextlib import contextmanager

class Task:
    def __init__(self, name):
        self.name = name

    def send(self, msg):
        print('{} send: {}'.format(self.name, msg))


class DisplayMessage:
    def __init__(self):
        self.count = 0

    def send(self, msg):
        print('msg[{}]: {!r}'.format(self.count, msg))
        self.count += 1


class Exchange:
    def __init__(self):
        self._subscribers = set()
        pass

    def attach(self, task):
        self._subscribers.add(task)

    def detach(self, task):
        self._subscribers.remove(task)

    def send(self, task):
        for subscriber in self._subscribers:
            subscriber.send(task)

    @contextmanager
    def subscribe(self, *tasks):
        for task in tasks:
            self.attach(task)
        try:
            yield
        finally:
            for task in tasks:
                self.detach(task)


_exchanges = defaultdict(Exchange)

_exchanges['test'] = Exchange()


def get_exchange(name):
    return _exchanges[name]



task_a = Task('task_a')
task_b = Task('task_b')
d = DisplayMessage()

exc = get_exchange('test')

# exc.attach(task_a)
# exc.attach(task_b)
# exc.attach(d)

with exc.subscribe(task_a, task_b, d):
    exc.send('msg1')
    exc.send('msg2')

if __name__ == "__main__":
    pass
