#!/usr/bin/env python3
import sys
import glob

if __name__ == "__main__":

    patern = sys.argv[1]
    print('patern: ',patern)

    files = sorted(glob.glob(patern, recursive=True))

    for file in files:
        print(file)

    print('found files count: ', len(files))
