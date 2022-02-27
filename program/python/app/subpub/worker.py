from queue import Queue, Empty
from threading import Event, Thread
import time
import redis

clients = dict()

port = 20000

clients['server'] = {'host': '127.0.0.1', 'port': 10000}
clients['host1'] = {'host': '127.0.0.1', 'port': 10001}
clients['host2'] = {'host': '127.0.0.1', 'port': 10002}

redis_conn = redis.Redis(host='127.0.0.1', password='123456')


sub = redis_conn.pubsub()
sub.subscribe('/task')

