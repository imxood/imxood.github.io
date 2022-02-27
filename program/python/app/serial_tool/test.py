#!/usr/bin/env python3

import logging
import sys
import queue

import serial
from serial import Serial
from serial.threaded import ReaderThread, Packetizer

logging.basicConfig(level=logging.DEBUG)


log_queue = queue.Queue()

def test1():

    ser = Serial(port='/dev/ttyUSB0', baudrate=115200, stopbits=1, parity=serial.PARITY_ODD, bytesize=8, timeout=60)

    # ser.open()

    print("open success")

    while ser.isOpen():

        try:
            data = ser.read()
            data = data.decode('utf-8', 'replace')
            print(data, end='')
        except:
            print(data)

        pass

        # '*%IR iZYy.zM W \x16y.L\tj1S-֢4ӺdS4NR*%Ij\n\x0276&S'

        # dataStr = str(base64.decodestring(base64.encodestring()))

        # data = ser.readline() #???,? \n ??,???? \n ????,??
        # data = ser.readlines() #????????
        # data = ser.xreadlines() #????????


    print("not open")




def test2():
    serialHandler = SerialHandler()
    serialHandler.loop('/dev/ttyUSB0')


class LineReader(Packetizer):
    """
    Read and write (Unicode) lines from/to serial port.
    The encoding is applied.
    """

    TERMINATOR = b'\n'
    ENCODING = 'utf-8'
    UNICODE_HANDLING = 'replace'

    def connection_made(self, transport):
        super(LineReader, self).connection_made(transport)
        sys.stdout.write('port opened\n')

    def handle_packet(self, packet):
        try:
            self.handle_line(packet.decode(self.ENCODING, self.UNICODE_HANDLING))
        except UnicodeEncodeError as err:
            pass

    def handle_line(self, data):
        log_queue.put_nowait(data)


class SerialHandler:

    def __init__(self):
        pass

    def deal_line_func(self, log_line):
        try:
            print(log_line)
        except UnicodeEncodeError as err:
            print('ERROR: ', str(err))
        return True

    def loop(self, port, baudrate=115200):

        ser = serial.serial_for_url(port, baudrate=115200)

        log_data = list()

        timeout = 30

        # auto close serial
        with ReaderThread(ser, LineReader) as protocol:

            while True:
                try:
                    log_line = log_queue.get(timeout=timeout).strip()

                    if self.deal_line_func(log_line) == False:
                        break

                    log_data.append(log_line)

                except queue.Empty:
                    continue

        return log_data


if __name__ == "__main__":
    test1()
    pass
