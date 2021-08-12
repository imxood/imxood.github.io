import time
import logging as log
import multiprocessing as mp


log.basicConfig(level=log.DEBUG)

# mp.set_start_method('spawn')
mp.set_start_method('spawn', True)


# 类似的全局变量是不能多进程共享的
a = {
    'b': 10
}


def echo_run_time(func):
    def wrapper(*args, **kw):
        local_time = time.time()
        ret = func(*args, **kw)
        log.info("Function [{}] run time is {:.3f}s".format(func.__name__, time.time() - local_time))
        return ret
    return wrapper


def test(a):
    print(a['b'])
    print("i'm a test")
    a['b'] = 12
    print(a['b'])
    time.sleep(10)
    print("i'm a test ok")


@echo_run_time
def main():

    manager = mp.Manager()
    opt = manager.dict()

    a['b'] = 11
    p = mp.Process(target=test, args=(opt,))
    p.start()

    time.sleep(3)

    print(time.time())

    p.terminate()
    print(time.time())

    p.join()
    print(time.time())

    print(a['b'])
    print('main thread ok')
    print('is alive', p.is_alive())


if __name__ == "__main__":
    main()
    pass
