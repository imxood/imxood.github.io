#!/usr/bin/env python3

import os
import os.path as path
import configparser as cp

script_path = path.abspath(path.dirname(__file__))

config_file = path.join(script_path, 'test.conf')

config = cp.ConfigParser()

config.read(config_file)

print(config.sections())

print(config['DEFAULT']['ServerAliveInterval'])

print(config['bitbucket.org']['User'])

with open(path.join(script_path, 'test.1.conf'), 'w') as f:
    config.write(f)
