import dramatiq
import requests
import sys
import time
from dramatiq.middleware import add_middleware
from dramatiq.brokers.redis import RedisBroker

from base.config import config


broker = RedisBroker(host = config['redis']['host'], password = config['redis']['password'])

dramatiq.set_broker(broker)

@dramatiq.actor
def count_words(url):
    response = requests.get(url)
    count = len(response.text.split(' '))
    print("There are {} words at {}.".format(count, url))
    time.sleep(5)
    print('ok')
