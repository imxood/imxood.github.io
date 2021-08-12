#!/usr/bin/env python3

import os

from urllib.request import urlopen
from zipfile import ZipFile
from io import BytesIO

script_dir = os.path.dirname(os.path.abspath(__file__))

url = 'https://sourceforge.net/code-snapshots/svn/a/as/astyle/code/astyle-code-r672-trunk.zip'

resp = urlopen(url)

with ZipFile(BytesIO(resp.read())) as zip_file:
    for file in zip_file.namelist():
        zip_file.extract(file, script_dir)


# cd ..

# touch output

# cd output

# cmake ..

# make

# make install

# rm ..
