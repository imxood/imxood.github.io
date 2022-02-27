#!/bin/bash

pip3 uninstall -y log_server

rm -rf build/ dist/ log_server.egg-info/

python3 setup.py sdist

pip3 install dist/log_server-0.1.tar.gz
