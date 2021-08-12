import os

import requests

from . import config
from dramatiq.results.backends import RedisBackend
from dramatiq.results import Results
from dramatiq.encoder import PickleEncoder
from dramatiq.brokers.redis import RedisBroker
from dramatiq import pipeline
from dramatiq import Middleware
import dramatiq
from threading import local

encoder = PickleEncoder()
backend = RedisBackend()

# encoder=encoder, host=config.HOST, port=config.PORT, db=config.DB, password=config.PASSWORD

broker = RedisBroker()

broker.add_middleware(Results(backend=backend))

dramatiq.set_broker(broker)

log = config.log


@dramatiq.actor(store_results=True)
def get_uri_contents(uri):
    return requests.get(uri).text


@dramatiq.actor(store_results=True)
def count_words(uri, text):
    count = len(text.split(" "))
    log.info(f"There are {count} words at {uri}.")
    return count


@dramatiq.actor
def hello_task():
    log.info("Hello, I'm Hello Task")
    return "ok"


@dramatiq.actor
def world_task(status):
    log.info('status: {}'.format(status))
    log.info("Hello, I'm Hello Task")
    return "ok"


@dramatiq.actor
def task_1():
    pipe = hello_task.message() | world_task.message()
    pipe.get_result(block=True)
    log.info("Hello, I'm Task_1")


@dramatiq.actor
def task_start_file_service():
    pass

@dramatiq.actor
def task_end_file_service():
    pass

@dramatiq.actor
def task_wait_file_service_complete():
    pass
