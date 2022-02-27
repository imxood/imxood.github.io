import os
import subprocess
import time
import inspect
import textwrap

# parent_rfd, child_wfd = os.pipe()
# child_rfd, parent_wfd = os.pipe()

# def hello():
#     print('hello, the world')

# source = inspect.getsource(hello)
# source = textwrap.dedent(source)
# source = source.replace('    ', '\t')


# proc = subprocess.Popen(
#     args=['python', '-c', 'print("%s")' % source],
#     # args=['python'],
#     # stdin=child_rfd,
#     # stdout=child_wfd,
#     shell=True
# )

# # time.sleep(0.5)
# # os.write(parent_wfd, b'import os;\ndata = os.read(0, 1024);\nprint(data)')
# # os.write(parent_wfd, b'hello')

# stdout, stderr = proc.communicate()
# proc.wait()

# print(stdout)
# print(stderr)


import os
import sys

R, W = os.pipe()
r, w = os.pipe()

if os.fork():
    os.dup2(0, 100)
    os.dup2(R, 0)

    os.dup2(r, 101)

    os.close(R)
    os.close(r)
    os.close(W)
    os.close(w)
    os.execl(sys.executable, 'print(1)')

os.write(1, 'MITO000\n'.encode())

C = os.read(0, 1024)

fp = os.fdopen(W, 'wb', 0)
fp.write(C)
fp.close()

fp = os.fdopen(w, 'wb', 0)
fp.write(C)
fp.close()

os.write(1, b'MITO001\n')
# os.close(2)

os.write(1, b'sleep 2s ...')
time.sleep(2)
os.write(1, b'sleep 2s end')

os.write(W, b'101')

time.sleep(100000)
