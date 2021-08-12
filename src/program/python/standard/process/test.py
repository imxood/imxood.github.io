import os
import time
import subprocess as sp
import multiprocessing

# command = "ls -al".split()
dir = os.path.dirname(os.path.abspath(__file__))
command = "bash %s/test.sh" % dir
print(command.split())

proc = sp.Popen(command.split(), stdout=sp.PIPE, stderr=sp.STDOUT)
proc.wait()
for line in proc.stdout:
   #the real code does filtering here
   print ("test:", line.rstrip())
# print(proc.returncode)
#


def test_process():
    print('hello')
    time.sleep(5)
    print('the world')

p = multiprocessing.Process(target=test_process)
p.start()

p.join()
