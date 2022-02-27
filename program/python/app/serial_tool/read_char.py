import sys
import tty
import termios

stdin = sys.stdin.fileno()
tattr = termios.tcgetattr(stdin)

try:
    tty.setraw(stdin, termios.TCSANOW)

    while True:
        char = sys.stdin.read(1)
        if ord(char) == 0x04:
            break
        sys.stdout.write(str(ord(char)))
        sys.stdout.flush()
finally:
    termios.tcsetattr(stdin, termios.TCSANOW, tattr)
