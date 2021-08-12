import logging
import tempfile
import io

# logging.basicConfig(level=logging.INFO,
#                     format='%(message)s',
#                     handlers=[console],
#                     datefmt='%H:%M:%S')


class SerialStreamHandler(logging.StreamHandler):

    # max_repeat_line_num = 50

    def __init__(self, stream=None):
        self.buffer = io.StringIO()
        self.lastline = ''
        super().__init__(stream)

    def __exit__(self, exc_type, exc_val, exc_tb):
        pos = self.buffer.tell()
        if pos != 0:
            text = self.buffer.getvalue()
            msg = self.format(text)
            self.stream.write(msg)
            self.flush()
        print('SerialStreamHandler.__exit__()')

    def get_lines(self, text: str):
        pos = self.buffer.tell()
        if pos != 0:
            self.buffer.write(text)
            text = self.buffer.getvalue()
            self.buffer.truncate()
            self.buffer.seek(0)
        lines = text.split(self.terminator)
        if lines[-1] != '':
            self.buffer.write(lines[-1])
            return lines[:-1]
        else:
            self.buffer.truncate()
            self.buffer.seek(0)
            return lines

    def emit(self, record: logging.LogRecord):
        try:
            lines = self.get_lines(record.getMessage())
            for line in lines:
                if line:
                    record.msg = line
                    msg = self.format(record)
                    self.stream.write(msg + self.terminator)
                    self.flush()
            if len(lines) > 1 and lines[-1] == '':
                self.lastline = lines[-2]

        except RecursionError:
            raise
        except Exception:
            self.handleError(record)


# for test log

console = logging.StreamHandler()
console.setLevel(logging.DEBUG)
console.setFormatter(logging.Formatter('%(asctime)s [%(name)s]   %(message)s'))

tlog = logging.Logger('test')
tlog.addHandler(console)


# file handler
# log_path = tempfile.gettempdir() + '/test_serial.log'
# fileHandler = logging.FileHandler(log_path, 'w')
# fileHandler.setLevel(logging.DEBUG)
# fileHandler.setFormatter(logging.Formatter('%(message)s'))

def get_logger(logger_name, logfile=tempfile.gettempdir() + '/test_serial.log'):
    serialHandler = SerialStreamHandler()
    serialHandler.setLevel(logging.DEBUG)
    serialHandler.setFormatter(logging.Formatter('%(asctime)s [%(name)s]   %(message)s'))

    slog = logging.Logger(logger_name)
    slog.addHandler(serialHandler)

    return slog
