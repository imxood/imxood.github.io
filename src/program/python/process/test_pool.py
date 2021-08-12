import os
import sys
import io
import tempfile
import time
import random
import argparse
import logging
import queue
from multiprocessing import Queue, Process, Manager, Pool, Lock, set_start_method
from threading import Thread
from enum import Enum
from os.path import abspath, dirname

import signal
from pynput import keyboard
from test_common import tlog, get_logger
from test_shell import Shell


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument('--enable', type=bool, default=False)
    parser.add_argument('--devices', type=str, required=True)
    parser.add_argument('--count', type=int, default=0)
    # 如果有'--names', 任意多个参数 --- [type]
    parser.add_argument('--names', type=str, nargs='*')
    # 如果有'--props', 那么至少一个参数 --- [type]
    parser.add_argument('--props', type=str, nargs='+')
    # 如果有'--attrs', 最多一个参数 --- type
    parser.add_argument('--attrs', type=str, nargs='?')
    return parser.parse_args()


def task(args):

    # 参数解包
    data, share_datas = args

    time_start = time.time()

    time.sleep(0.5)
    data['a'] = random.randint(0, 10000)
    share_datas.append(data)
    time.sleep(0.5)

    time_end = time.time()

    tlog.info('[%d]time_diff: %f' % (os.getpid(), time_end - time_start))


class TaskStatus(Enum):

    Success = 0
    Running = 1
    Stopping = 2
    Error = -1


class Task(object):

    def __init__(self, func, priority=100, args=None):
        """ priority: 0 is the max priority """

        self.func = func
        self.name = func.__name__
        self.priority = priority if priority > 0 else 0
        self.status = 0
        self.args = args

    def run(self):
        if (self.status != 0):
            return self.status
        if self.args:
            if type(self.args) is tuple:
                return self.func(*self.args)
            return self.func(self.args)
        else:
            return self.func()

    def stop(self):
        pass


class TaskProcess(Process):

    def __init__(self, task: Task, priority=10):
        self.task = task
        self.queue = Queue()
        Process.__init__(self)

    def start(self):
        super().start()

    def run(self):
        sys.exit(self.task.run())


class TaskManager(object):

    def __init__(self, log=logging.getLogger()):
        # access to unique resources
        self.lock = Lock()
        self.current_low_priority = 0

        self.stoped = False
        self.log = log

        self.uncoming_task_procs = list()
        self.running_task_procs = list()
        self.error_task_procs = list()

    def add_task(self, task):
        # if there are running_task_procs
        if self.running_task_procs:
            if self.current_low_priority:
                if task.priority <= self.current_low_priority:
                    self.log.error('error, the task to add must be more low than the running lowest task priority')
        self.uncoming_task_procs.append(TaskProcess(task))

    def list_group(self, array):
        index = []
        for i, _ in enumerate(array):
            if i < len(array) - 1 and array[i + 1].task.priority != array[i].task.priority:
                index.append(i + 1)

        def take(array, n):
            for i in range(n):
                yield next(array)

        if not hasattr(array, 'next'):
            array = iter(array)

        begin = 0
        for item in index:
            x = list(take(array, item - begin))
            begin = item
            yield x

        yield list(array)

    def monitor(self):
        # sort by task's priority
        self.running_task_procs = sorted(self.running_task_procs, key=lambda x: x.task.priority)

        procs_group = list(self.list_group(self.running_task_procs))

        for same_prio_procs in procs_group:

            for proc in same_prio_procs:
                proc.start()

            while True:
                for proc in same_prio_procs:
                    if not self.stoped:
                        # has error, stop all alived task
                        if not proc.is_alive() and proc.exitcode:
                            for p in same_prio_procs:
                                if p.is_alive():
                                    p.terminate()
                            self.log.error('task[%s] error, stopped all alived task' % proc.task.name)
                            sys.exit(0)
                    else:
                        for p in same_prio_procs:
                            if p.is_alive():
                                p.terminate()
                        sys.exit(1)
                # all is ok
                if all([not proc.is_alive() and proc.exitcode == 0 for proc in same_prio_procs]):
                    break
                time.sleep(0.02)

        self.running_task_procs.clear()

    def start(self):

        self.running_task_procs.extend(self.uncoming_task_procs)
        self.uncoming_task_procs.clear()

        self.monitor()

    def stop(self):
        self.stoped = True
        # self.queue.put({'cmd': 'exit'})


manager = None


def task1(queue: Queue):
    tlog.info("enter task1")
    time.sleep(4)
    msg = 'please exit'
    tlog.info('put msg: %s' % msg)
    queue.put(msg)
    tlog.info("leave task1")
    return 0


def task2(queue: Queue):
    tlog.info("enter task2")
    tlog.info('wait for queue msg ...')
    msg = queue.get()
    tlog.info('get msg: %s' % msg)
    tlog.info("leave task2")
    return 0


def task3():
    tlog.info("enter task3")
    time.sleep(10)
    tlog.info("leave task3")
    return 1


def task4():
    tlog.info("enter task4")
    time.sleep(5)
    tlog.info("leave task4")
    return 0


def task5():
    for i in range(5):
        tlog.info("leave task4")
        time.sleep(0.1)
    return 0


def serial_line_handler(msgQ: queue.Queue, timeout: float):
    lines = []
    timeout_old = timeout
    STOP_TIME = 3
    while True:
        try:
            line = msgQ.get(timeout=timeout)
            timeout = timeout_old
            lines.append(line)
            if timeout == STOP_TIME:
                break
            if line.startswith('FATAL_ERROR'):
                timeout = STOP_TIME
                continue
            if line.startswith('SUCCESS'):
                timeout = STOP_TIME
        except queue.Empty:
            break


def serial_task(env_name, project='', casename=''):

    dev_inf = get_dev_inf(env_name, 'serial')

    with Shell(dev_inf, vlog) as s:

        vlog.info('serial ...')

        with s.cd_remote(REMOTE_WORKSPACE):

            slog = get_logger('serial', serial_line_handler)
            command = 'plink -serial /dev/ttyUSB0 -sercfg 115200,8,n,1,N'

            ret = s.remote(command, log=slog)
            if ret[0]:
                vlog.error('run [{}] failed'.format(command))
                return ret[0]
            return 0


def log_task():
    pass


def test1():
    datas = [{'a': d} for d in range(5)]

    option = parse_args()
    print(option)

    time_start = time.time()

    with Manager() as mng:

        share_datas = mng.list()

        with Pool(2) as p:
            tasks_args = [(data, share_datas) for data in datas]
            p.map(task, tasks_args)

        # for data in datas:
        #     tlog.info(data['a'])
        time_end = time.time()

        tlog.info('[%d]time_start: %f' % (os.getpid(), time_start))
        tlog.info('[%d]time_end: %f' % (os.getpid(), time_end))
        tlog.info('[%d]time_diff: %f' % (os.getpid(), time_end - time_start))

        tlog.info('len: %d' % len(share_datas))


def test2():
    log = logging.getLogger('')
    tlog.info('main pid: %d' % os.getpid())
    queue = Queue()
    manager = TaskManager()
    # task_1 = Task(task1, 80, queue)
    # task_2 = Task(task2, 80, queue)
    # manager.add_task(task_1)
    # manager.add_task(task_2)
    # manager.add_task(Task(task3, 30))
    manager.add_task(Task(task4, 30))
    manager.add_task(Task(serial_task, 30))
    manager.start()

# test 1

## keyboard.KeyCode(char='c')

# COMBINATION = {keyboard.Key.ctrl, keyboard.KeyCode.from_char('c')}
# current = set()


# def on_press(key):
#     tlog.info(key)
#     if key in COMBINATION:
#         current.add(key)
#         if all(k in current for k in COMBINATION):
#             tlog.info('All modifiers active!')
#     if key == keyboard.Key.esc:
#         listener.stop()


# def on_release(key):
#     try:
#         current.remove(key)
#     except KeyError:
#         pass


# test 2
def on_activate_ctrl_c():
    tlog.info('<ctrl>+c pressed')


def on_activate_ctrl_shift_a():
    tlog.info('<ctrl>+<shift>+a pressed')


def signal_handler(sig, frame):
    pid = os.getpid()
    if manager:
        manager.stop()
        tlog.info('pid: %s Canceled all task!' % pid)
    sys.exit(0)


signal.signal(signal.SIGINT, signal_handler)

if __name__ == "__main__":
    tlog.info("i'm fine")
    set_start_method('spawn')

    # test1()
    # listener = keyboard.Listener(on_press=on_press, on_release=on_release)
    # listener.start()  # start to listen on a separate thread
    # listener.join()  # remove if main thread is polling self.keys
    # listener = keyboard.GlobalHotKeys({
    #     '<ctrl>+c': on_activate_ctrl_c,
    #     '<ctrl>+<shift>+a': on_activate_ctrl_shift_a,
    # })
    # listener.start()
    test2()
