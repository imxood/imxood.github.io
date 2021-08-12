import queue
import time
from threading import Thread, Event





class Actor:



class PaintActor(Actor):
    def run(self):
        while True:
            v = self.recv()
            print('Get msg: ', v)

# actor = PaintActor()
# actor.start()

# time.sleep(5)
# actor.send('Hello')

# time.sleep(5)
# actor.send('World')

# time.sleep(5)
# actor.close()
# actor.join()
