import os
import os.path as path
import configparser as cp

config_dir = path.abspath(path.dirname(__file__))
config_file = path.join(config_dir, 'test.conf')

config = cp.ConfigParser()
config.read(config_file)
