import sys
import os
from datetime import datetime
import subprocess as sp

import readchar
import colorama

import serial
import serial.tools.list_ports
from serial.threaded import ReaderThread, Protocol


class SerialProtocol(Protocol):

    ENCODING = 'utf-8'
    UNICODE_HANDLING = 'replace'

    def __init__(self):
        self.transport = None
        self.start_time = datetime.now()

    def connection_made(self, transport):
        self.transport = transport

    def data_received(self, data):

        d = data.decode(self.ENCODING, self.UNICODE_HANDLING)
        print('"{} {}"'.format(len(d), d))

        index = d.find('\n')
        if index != -1:
            delta = datetime.now() - self.start_time
            date = '[{}]'.format(str(delta))
            d = d[:index+1] + date + d[index+1:]

        sys.stdout.write(d)
        sys.stdout.flush()

    def connection_lost(self, exc):
        if exc:
            print(exc)

        print('\r\nport closed')

        self.transport = None

    def write_packet(self, data, is_binary=0):
        if self.transport:
            if is_binary:
                self.transport.write(data)
            else:
                self.transport.write(data.encode(self.ENCODING, self.UNICODE_HANDLING))


if __name__ == "__main__":

    colorama.init()

    if 'PYTHONUNBUFFERED' not in os.environ:

        os.environ['PYTHONUNBUFFERED'] = '1'

        command = [sys.executable]
        command.extend(sys.argv)

        sp.call(command)

        sys.exit(0)

    ser = serial.serial_for_url('/dev/ttyUSB0', baudrate=115200, timeout=1)

    with ReaderThread(ser, SerialProtocol) as protocol:

        CTRL_S = chr(ord(readchar.key.CTRL_A) + ord('s') - ord('a'))

        while True:
            key = readchar.readkey()

            if key == readchar.key.CTRL_D:
                break

            if key == CTRL_S:
                sys.stdout.write('Save to files\n')
                sys.stdout.flush()
                protocol.write_packet('\r')

            protocol.write_packet(key)
