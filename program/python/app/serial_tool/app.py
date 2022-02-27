import sys
from enum import Enum

import serial
import serial.tools.list_ports
from serial.threaded import ReaderThread, Protocol

from PySide2.QtGui import QPalette
from PySide2.QtCore import Qt, QObject, Slot, Signal
from PySide2.QtWidgets import QApplication, qApp, QWidget
from PySide2.QtWidgets import QPushButton, QComboBox, QGridLayout, QDialog, QPlainTextEdit, QStatusBar
from PySide2.QtWidgets import QTabWidget, QDialogButtonBox, QVBoxLayout, QLineEdit, QFormLayout


import re


g_key_map = {
    Qt.Key_Left: 0x25,
    Qt.Key_Up: 0x26,
    Qt.Key_Right: 0x27,
    Qt.Key_Down: 0x28,
    Qt.Key_Backspace: 0x08,
    Qt.Key_Tab: 0x09,
    Qt.Key_Enter: 0x0D,
    Qt.Key_Shift: 0x10,
    Qt.Key_Control: 0x11,
    Qt.Key_Alt: 0x12,
    Qt.Key_Space: 0x20,
    Qt.Key_Insert: 0x2D,
    Qt.Key_Delete: 0x2E
}


_ANSI2HTML_STYLES = {}
ANSI2HTML_CODES_RE = re.compile('(?:\033\\[(\d+(?:;\d+)*)?([cnRhlABCDfsurgKJipm]))')
ANSI2HTML_PALETTE = {
    # See http://ethanschoonover.com/solarized
    'solarized': ['#073642', '#D30102', '#859900', '#B58900', '#268BD2', '#D33682', '#2AA198', '#EEE8D5', '#002B36', '#CB4B16', '#586E75', '#657B83', '#839496', '#6C71C4', '#93A1A1', '#FDF6E3'],
    # Above mapped onto the xterm 256 color palette
    'solarized-xterm': ['#262626', '#AF0000', '#5F8700', '#AF8700', '#0087FF', '#AF005F', '#00AFAF', '#E4E4E4', '#1C1C1C', '#D75F00', '#585858', '#626262', '#808080', '#5F5FAF', '#8A8A8A', '#FFFFD7'],
    # Gnome default:
    'tango': ['#000000', '#CC0000', '#4E9A06', '#C4A000', '#3465A4', '#75507B', '#06989A', '#D3D7CF', '#555753', '#EF2929', '#8AE234', '#FCE94F', '#729FCF', '#AD7FA8', '#34E2E2', '#EEEEEC'],
    # xterm:
    'xterm': ['#000000', '#CD0000', '#00CD00', '#CDCD00', '#0000EE', '#CD00CD', '#00CDCD', '#E5E5E5', '#7F7F7F', '#FF0000', '#00FF00', '#FFFF00', '#5C5CFF', '#FF00FF', '#00FFFF', '#FFFFFF'],
    'console': ['#000000', '#AA0000', '#00AA00', '#AA5500', '#0000AA', '#AA00AA', '#00AAAA', '#AAAAAA', '#555555', '#FF5555', '#55FF55', '#FFFF55', '#5555FF', '#FF55FF', '#55FFFF', '#FFFFFF'],
}


def _ansi2html_get_styles(palette):
    if palette not in _ANSI2HTML_STYLES:
        p = ANSI2HTML_PALETTE.get(palette, ANSI2HTML_PALETTE['console'])

        regular_style = {
            '1': '',  # bold
            '2': 'opacity:0.5',
            '4': 'text-decoration:underline',
            '5': 'font-weight:bold',
            '7': '',
            '8': 'display:none',
        }
        bold_style = regular_style.copy()
        for i in range(8):
            regular_style['3%s' % i] = 'color:%s' % p[i]
            regular_style['4%s' % i] = 'background-color:%s' % p[i]

            bold_style['3%s' % i] = 'color:%s' % p[i + 8]
            bold_style['4%s' % i] = 'background-color:%s' % p[i + 8]

        # The default xterm 256 colour p:
        indexed_style = {}
        for i in range(16):
            indexed_style['%s' % i] = p[i]

        for rr in range(6):
            for gg in range(6):
                for bb in range(6):
                    i = 16 + rr * 36 + gg * 6 + bb
                    r = (rr * 40 + 55) if rr else 0
                    g = (gg * 40 + 55) if gg else 0
                    b = (bb * 40 + 55) if bb else 0
                    indexed_style['%s' % i] = ''.join('%02X' % c if 0 <= c <= 255 else None for c in (r, g, b))

        for g in range(24):
            i = g + 232
            l = g * 10 + 8
            indexed_style['%s' % i] = ''.join('%02X' % c if 0 <= c <= 255 else None for c in (l, l, l))

        _ANSI2HTML_STYLES[palette] = (regular_style, bold_style, indexed_style)
    return _ANSI2HTML_STYLES[palette]


def ansi2html(text, palette='solarized'):
    def _ansi2html(m):
        if m.group(2) != 'm':
            return ''
        state = None
        sub = ''
        cs = m.group(1)
        cs = cs.strip() if cs else ''
        for c in cs.split(';'):
            c = c.strip().lstrip('0') or '0'
            if c == '0':
                while stack:
                    sub += '</span>'
                    stack.pop()
            elif c in ('38', '48'):
                extra = [c]
                state = 'extra'
            elif state == 'extra':
                if c == '5':
                    state = 'idx'
                elif c == '2':
                    state = 'r'
            elif state:
                if state == 'idx':
                    extra.append(c)
                    state = None
                    # 256 colors
                    color = indexed_style.get(c)  # TODO: convert index to RGB!
                    if color is not None:
                        sub += '<span style="%s:%s">' % ('color' if extra[0] == '38' else 'background-color', color)
                        stack.append(extra)
                elif state in ('r', 'g', 'b'):
                    extra.append(c)
                    if state == 'r':
                        state = 'g'
                    elif state == 'g':
                        state = 'b'
                    else:
                        state = None
                        try:
                            color = '#' + ''.join('%02X' % c if 0 <= c <= 255 else None for x in extra for c in [int(x)])
                        except (ValueError, TypeError):
                            pass
                        else:
                            sub += '<span style="%s:%s">' % ('color' if extra[0] == '38' else 'background-color', color)
                            stack.append(extra)
            else:
                if '1' in stack:
                    style = bold_style.get(c)
                else:
                    style = regular_style.get(c)
                if style is not None:
                    sub += '<span style="%s">' % style
                    stack.append(c)  # Still needs to be added to the stack even if style is empty (so it can check '1' in stack above, for example)
        return sub
    stack = []
    regular_style, bold_style, indexed_style = _ansi2html_get_styles(palette)
    sub = ANSI2HTML_CODES_RE.sub(_ansi2html, text)
    while stack:
        sub += '</span>'
        stack.pop()
    return sub


class ConnectParam:
    def __init__(self, type):
        self.type = type


class SerialParam(ConnectParam):
    def __init__(self, port, baud=115200):
        ConnectParam.__init__(self, 'serial')
        self.port = port
        self.baud = baud


class ConnectType(Enum):
    Serial = 1
    Ssh = 2


class ComboBox(QComboBox):

    clicked = Signal()

    def __init__(self, parent=None):
        QComboBox.__init__(self, parent)

    def mousePressEvent(self, event):
        if event.button() == Qt.LeftButton:
            self.clicked.emit()
        super().mousePressEvent(event)


class Console(QPlainTextEdit):

    key_data_signal = Signal(str)

    def __init__(self, parent=None):

        self.parent = parent

        QPlainTextEdit.__init__(self, parent)

        self.m_localEchoEnabled = False

        self.document().setMaximumBlockCount(100)

        p = QPalette()
        p.setColor(QPalette.Base, Qt.black)
        p.setColor(QPalette.Text, Qt.green)

        self.setPalette(p)

    def putData(self, data):
        # d = ansi2html(data)
        # self.appendHtml(data)
        if data == '\r':
            return
        self.insertPlainText(data)
        bar = self.verticalScrollBar()
        bar.setValue(bar.maximum())

    def setLocalEchoEnabled(self, bool_value):
        self.m_localEchoEnabled = bool_value

    def keyPressEvent(self, e):

        # if key == Qt.Key_Backspace or key == Qt.Key_Left or key == Qt.Key_Right or key == Qt.Key_Up or key == Qt.Key_Down:
        #     return

        if self.m_localEchoEnabled:
            QPlainTextEdit.keyPressEvent(e)

        # key = e.text()

        if self.parent.alive:
            if e.text():
                print('key: ', e.text)
                self.key_data_signal.emit(e.text())
            elif e.key() in g_key_map:
                print('key: 0x%x' % g_key_map[e.key()])
                self.key_data_signal.emit(chr(g_key_map[e.key()]))

    def mousePressEvent(self, e):
        self.setFocus()

    # def mouseDoubleClickEvent(self, e):
    #     pass

    # def contextMenuEvent(self, e):
    #     pass


class SerialProtocol(Protocol, QObject):

    ENCODING = 'utf-8'
    UNICODE_HANDLING = 'replace'

    connected_signal = Signal()
    connect_losed_signal = Signal()
    ingoing_data_signal = Signal(str)

    def __init__(self):

        QObject.__init__(self)
        self.transport = None

    def bind_signals(self, parent):
        self.connected_signal.connect(parent.connected)
        self.connect_losed_signal.connect(parent.connect_losed)
        self.ingoing_data_signal.connect(parent.ingoing_data)
        self.connected_signal.emit()

    def connection_made(self, transport):
        """Store transport"""
        self.transport = transport

    def data_received(self, data):
        """Buffer received data, find TERMINATOR, call handle_packet"""
        d = data.decode(self.ENCODING, self.UNICODE_HANDLING)
        sys.stdout.write('line received: {!r}\n'.format(d))
        self.ingoing_data_signal.emit(d)

    def connection_lost(self, exc):
        if exc:
            print(exc)

        sys.stdout.write('port closed\n')

        self.transport = None
        self.connect_losed_signal.emit()

    def write_packet(self, data, is_binary):
        if self.transport:
            if is_binary:
                self.transport.write(data)
            else:
                self.transport.write(data.encode(self.ENCODING, self.UNICODE_HANDLING))


class SerialWindow(QWidget):

    def __init__(self, parent=None, port=None, baud=115200):

        QWidget.__init__(self, parent)

        self.alive = False

        self.console = Console(self)
        self.console.key_data_signal.connect(self.write_data)

        self.status_bar = QStatusBar(self)

        layout = QGridLayout(self)
        layout.addWidget(self.console)
        layout.addWidget(self.status_bar)

        ser = serial.serial_for_url(port, baudrate=baud, timeout=1)

        t = ReaderThread(ser, SerialProtocol)
        t.start()

        _, self.protocol = t.connect()
        self.protocol.bind_signals(self)

    def showStatusMessage(self, message):
        self.status_bar.showMessage(message)

    @Slot()
    def connected(self):
        self.showStatusMessage('opened')
        self.alive = True

    @Slot()
    def connect_losed(self):
        self.showStatusMessage('closed')
        self.alive = False

    @Slot(str)
    def ingoing_data(self, data):
        self.console.putData(data)

    @Slot(str, int)
    def write_data(self, data, is_binary=0):
        self.protocol.write_packet(data, is_binary)

    @Slot()
    def handle_error(self, error):
        self.showStatusMessage(error)
        pass


class NewConnectDialog(QDialog):

    new_connect_window_signal = Signal(ConnectParam)

    def __init__(self, parent):

        QDialog.__init__(self, parent)

        self.parent = parent

        self.setWindowTitle('新建窗口')

        tabwidget = QTabWidget()
        tabwidget.addTab(SerialConnectForm(self), '串口')
        tabwidget.addTab(SshConnectForm(self), u'SSH')

        layout = QVBoxLayout()
        layout.addWidget(tabwidget)

        self.setLayout(layout)

        self.new_connect_window_signal.connect(parent.new_connect_window)

    @Slot(ConnectParam)
    def new_connect(self, param):
        self.new_connect_window_signal.emit(param)
        self.close_window()

    @Slot()
    def close_window(self):
        self.close()


class BaseForm(QDialog):

    def __init__(self, parent):

        QDialog.__init__(self, parent)

        self.main_layout = QVBoxLayout(self)
        self.setModal(True)

    def add_confirm_cancel_box(self):

        cancel_btn = QPushButton('取消')
        ok_btn = QPushButton('确定')

        self.ok_cancel_box = QDialogButtonBox()

        self.ok_cancel_box.addButton(ok_btn, QDialogButtonBox.AcceptRole)
        self.ok_cancel_box.addButton(cancel_btn, QDialogButtonBox.RejectRole)

        self.main_layout.addWidget(self.ok_cancel_box)


class SerialConnectForm(BaseForm):

    ok_signal = Signal(SerialParam)

    def __init__(self, parent):

        BaseForm.__init__(self, parent)

        self.setWindowTitle('串口设置')

        self.serial_name_comb = ComboBox()
        self.serial_name_comb.addItem('Please choose device')
        self.serial_name_comb.setEditable(True)
        # self.serial_name_comb.clicked.connect(self.list_ports)

        self.serial_port_edit = QLineEdit()
        self.serial_baud_edit = QLineEdit()

        format_layout = QFormLayout()
        format_layout.addRow('设备名:', self.serial_name_comb)
        format_layout.addRow('端口:', self.serial_port_edit)
        format_layout.addRow('波特率:', self.serial_baud_edit)

        self.main_layout.addLayout(format_layout)

        self.add_confirm_cancel_box()

        self.ok_cancel_box.accepted.connect(self.ok)
        self.ok_cancel_box.rejected.connect(parent.close_window)
        self.ok_signal.connect(parent.new_connect)

        self.list_ports()

        print("Now there are {} Items".format(self.serial_name_comb.count()))

    @Slot()
    def ok(self):
        print('accepted')
        device = self.serial_name_comb.currentText()
        serialParam = SerialParam(device)
        self.ok_signal.emit(serialParam)

    def __deinit__(self):
        print('SerialConnectForm __deinit__')

    def list_ports(self):
        port_list = serial.tools.list_ports.comports()
        if len(port_list):
            self.serial_name_comb.clear()
            for index, port in enumerate(port_list):
                print(port.device)
                self.serial_name_comb.addItem(port.device)


class SshConnectForm(BaseForm):

    def __init__(self, parent=None):

        BaseForm.__init__(self, parent)

        self.setWindowTitle('SSH设置')
        self.setFixedSize(600, 450)

        self.host_edit = QLineEdit(self)
        self.port_edit = QLineEdit(self)
        self.user_edit = QLineEdit(self)

        format_layout = QFormLayout()
        format_layout.addRow('HOST:', self.host_edit)
        format_layout.addRow('PORT:', self.port_edit)
        format_layout.addRow('USER:', self.user_edit)

        self.main_layout.addLayout(format_layout)

        self.add_confirm_cancel_box()


class MainWindow(QWidget):

    def __init__(self, parent=None):

        QWidget.__init__(self, parent)

        self.setWindowTitle('串口助手')

        self.setFixedSize(800, 600)

        self.tabwidget = QTabWidget(self)

        new_connect_btn = QPushButton(text='新建连接', parent=self.tabwidget)
        new_connect_btn.clicked.connect(self.new_connect_dialog)

        self.tabwidget.addTab(new_connect_btn, '欢迎')

        self.layout = QVBoxLayout()
        self.layout.addWidget(self.tabwidget)

        self.setLayout(self.layout)

        desktop = qApp.desktop()

        self.move((desktop.width() - self.width()) / 2, (desktop.height() - self.height()) / 2)

    @Slot()
    def new_connect_dialog(self):

        dialog = NewConnectDialog(self)
        dialog.setModal(True)
        dialog.show()

    @Slot(ConnectParam)
    def new_connect_window(self, param: ConnectParam):
        if param.type == 'serial':
            serialWindow = SerialWindow(self, param.port, param.baud)
            self.tabwidget.addTab(serialWindow, '串口')
            self.tabwidget.setCurrentWidget(serialWindow)


app = QApplication()

mainWindow = MainWindow()
mainWindow.show()

sys.exit(app.exec_())
