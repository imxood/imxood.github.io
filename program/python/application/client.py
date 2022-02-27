#!/usr/bin/env python3

from socket import *

s = socket(AF_INET, SOCK_DGRAM)

s.sendto(b'',('localhost',14000))
s.recvfrom(128)

s.sendto(b'Hello',('localhost',15000))
s.recvfrom(128)
