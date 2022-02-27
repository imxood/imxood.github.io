import serial
from serial.tools.miniterm import Miniterm, unichr
import colorama

colorama.init(True)

serial_instance = serial.serial_for_url('/dev/ttyUSB0', 115200, do_not_open=True)

miniterm = Miniterm(serial_instance, filters=['colorize'])
miniterm.exit_character = unichr(29)
miniterm.menu_character = unichr(20)

miniterm.start()

try:
    miniterm.join(True)
except KeyboardInterrupt:
    pass
