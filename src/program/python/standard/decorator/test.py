import functools
import time


def log1(func):
    @functools.wraps(func)
    def wrapper(*args, **kwargs):
        print("before call {}()".format(func.__name__))
        func(*args, **kwargs)
        print("after call {}()".format(func.__name__))
    return wrapper

# now = log(now)
@log1
def yesterday():
    print("current timestramp: {}".format(time.time()))


yesterday()


def log2(text):
    def d(func):
        @functools.wraps(func)
        def wrapper(*args, **kwargs):
            print("text: {}".format(text))
            print("before call {}()".format(func.__name__))
            func(*args, **kwargs)
            print("after call {}()".format(func.__name__))
        return wrapper
    return d

# now = log2('hello')(now)(text)
@log2('hello')
def now(text):
    print("text: {}".format(text))
    print("current timestramp: {}".format(time.time()))


now('world')
