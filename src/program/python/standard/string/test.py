import os
import re

# \x1b[91m

data = '\x80'
print(data)
print(len(data))

data = '你好'
print(data)
print(type(data))
print(repr(data))
print(len(data))
print(data.encode().decode())

# data = data.decode(encoding="utf-8", errors='ignore')

data = 'Default \x1b[91mLight red, Default \x1b[93mLight yellow, Default \x1b[92mLight green\x1b[0m'
print(data)

ansi_escape = re.compile(r'\x1B\[[0-?]*[ -/]*[@-~]')

data = b'\x1b[0m<inf> encoder_run: counter: 2205\x1b[0m\r\n'
data = data.decode().rstrip()

data = ansi_escape.sub('', data)

print(data)


# b'*%IR \xa8\xeb\xd1i\xeb\x81ZY\xa1y.zM \xacW\x8b\x91 \xaf\x16\xa1y\xae\xc9\xc5.L\x93\tj1S\xb2-\xd6\xa24\xd3\xbadS\xca4NR*%Ij'
# b'\x96\x027\xaa6&S\x92\r\xa1\xc1pu\xb1o\xac\xb1:\xc2\xc90\x920\x820'
# b'\xc1pu\xb1o\xac\xb1.Z\x96\x020'
# b"\xc1pu\xb1o\xac\xb1.+\xb5e'"
# b'AP\x05j'
# b'U\xa5tZ\xeb\xb9.\xa9'
# b'r]\x0b\xa2\xa1rY\x91_Y\x8b\xc9y\x15j'
# b'X\xc1_\x16\xeb\x89aK\x82x&\x822\x820b X\xc1_\x16\xeb\x89a\xcbJ\x91:\x82,'
# b"\xc1pu\xb1o\xac\xb1.+\xb5e'"
# b'AP\x05j'
# b'X\xc1_\x16\xeb\x89aK\x82x&\x822\x820b X\xc1_\x16\xeb\x89a\xcbJ\x91:\x8a,'
# b"\xc1pu\xb1o\xac\xb1.+\xb5e'"
# b'AP\x15j'
# b'r]\x0b\xa2\xa1rY\x91_Y\x8b\xc9y%j'
# b'X\xc1_\x16\xeb\x89aK\x82x&\x822\x820b X\xc1_\x16\xeb\x89a\xcbJ\x91:\x8a,'
# b"\xc1pu\xb1o\xac\xb1.+\xb5e'"
# b'AP\x15j'
# b'X\xc1_\x16\xeb\x89aK\x82x&\x822\x820b X\xc1_\x16\xeb\x89a\xcbJ\x91:\x92,'
# b"\xc1pu\xb1o\xac\xb1.+\xb5e'"
# b'AP%j'
# b'AP%j\n

pass
