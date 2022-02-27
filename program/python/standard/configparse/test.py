#!/usr/bin/env python3

import os
import os.path as path
import configparser as cp
import paramiko
import getpass
import re
import time

script_path = path.abspath(path.dirname(__file__))
config_file = path.join(script_path, 'test.conf')

config = cp.ConfigParser()
config.read(config_file)

host = config['linux']['host']
port = config['linux']['port']

print("host: {}, port: {}".format(host, port))
