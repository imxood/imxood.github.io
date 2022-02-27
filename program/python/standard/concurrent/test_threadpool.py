import os
import time
from multiprocessing import Pool, TimeoutError


def sleep_time(number):
    print("os.pid: {}, sleeping {}s".format(os.getpid(), number))
    time.sleep(number)
    print("os.pid: {}, exited".format(os.getpid()))

results = list()

def test_pool_map():

    print("wating work process...")

    print(results)

    with Pool() as pool:
        print("work start")
        results = pool.map(sleep_time, range(5))
        print("work end, results: ", results)

    print("work process finished")


def callback(result):
    print("callback: ", result)
    results.append(result)


def test_pool_map_async():

    print("wating work process...")

    with Pool() as pool:
        result = pool.map_async(sleep_time, range(5), callback=callback)
        print(result)
        pool.close()
        pool.join()

    print("work process finished")


if __name__ == "__main__":
    test_pool_map()
    # test_pool_map_async()
