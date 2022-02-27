import dramatiq
import requests
import sys
from dramatiq.brokers.redis import RedisBroker

from base.config import config

broker = RedisBroker(host = config['redis']['host'], password = config['redis']['password'])

dramatiq.set_broker(broker)

@dramatiq.actor
def count_words(url):
    pass

if __name__ == '__main__':
    queue = count_words.send(sys.argv[1])
    print(queue)
