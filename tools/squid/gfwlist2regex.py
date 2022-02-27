from urllib import request
import re
from base64 import b64decode
from pathlib import Path

SCRIPT_DIR = Path(__file__).parent


LIST_URL = 'https://raw.githubusercontent.com/gfwlist/gfwlist/master/gfwlist.txt'
DECODE_FILE = SCRIPT_DIR / 'decode.txt'
BLACK_FILE = SCRIPT_DIR / 'gfw.url_regex.lst'
WHITE_FILE = SCRIPT_DIR / 'cn.url_regex.lst'


def convert_line(line):
    line = line.rstrip()
    # regex already
    if line[0] == '/' and line[-1] == '/':
        # remove https?:\/\/[^\/]+
        rline = line[1:-1]
        rline = rline.replace(r'^https?:\/\/[^\/]+', r'^[^\/]+')
        return rline

    if line.startswith('||'):
        rline = line[2:]
        rline = rline.replace(r'http://', '')
        rline = rline.replace(r'https://', '')
        rline = re.escape(rline)
        rline = rline.replace(r'\*', '(.*)')
        # return '^https?:\/\/[^\/]+' + rline
        return '^[^\/]*' + rline
    elif line.startswith('|'):
        rline = line[1:]
        rline = rline.replace(r'http://', '')
        rline = rline.replace(r'https://', '')
        rline = re.escape(rline)
        rline = rline.replace(r'\*', '.*')
        return '^' + rline
    elif line[-1] == '|':
        rline = line[:-1]
        rline = rline.replace(r'http://', '')
        rline = rline.replace(r'https://', '')
        rline = re.escape(rline)
        rline = rline.replace(r'\*', '.*')
        return rline + '$'
    else:
        rline = line
        rline = rline.replace(r'http://', '')
        rline = rline.replace(r'https://', '')
        rline = re.escape(rline)
        rline = rline.replace(r'\*', '.*')
        return rline


def convert(gfwlist):
    black = open(BLACK_FILE, 'w')
    white = open(WHITE_FILE, 'w')

    for l in gfwlist.split('\n'):
        #l = l[:-1]
        if not l or l[0] == '!' or l[0] == '[':
            continue

        if l.startswith('@@'):
            white.write(convert_line(l[2:]) + '\n')
        else:
            black.write(convert_line(l) + '\n')


def main():
    req = request.urlopen(LIST_URL)
    with req as f:
        src = f.read()
        src = b64decode(src).decode()
        open(DECODE_FILE, 'w').write(src)
        # decode.close()
        # src = open(DECODE_FILE, 'r').read()
        convert(src)


if __name__ == '__main__':
    main()
