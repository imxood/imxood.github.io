#!/usr/bin/env python3

import os
import os.path as path
import configparser as cp
import paramiko
import getpass
import re
import time

script_path = path.abspath(path.dirname(__file__))
config_file = path.join(script_path, 'test.conf')


class RemoteShell():

    def __init__(self, user, password, host='127.0.0.1', port='22', is_print=True):
        try:
            self.host = host
            self.port = port
            self.user = user
            self.password = password
            self.is_print = is_print

            self.ssh_fd = paramiko.SSHClient()
            self.ssh_fd.set_missing_host_key_policy(paramiko.AutoAddPolicy())
            self.ssh_fd.connect(host, port, user, password)

            # self.print('connected to "{}@{} -p {}"\n'.format(self.user, self.password, self.port))

        except Exception as e:
            self.print('connect failed: {},  "{}@{} -p {}"\n'.format(e, self.user, self.password, self.port))
            exit(-1)

    def exec(self, cmd):

        stdout = ''
        stderr = ''
        buff_size = 10240000
        exit_status = 0

        channel = self.ssh_fd.get_transport().open_session()
        channel.get_pty()

        channel.exec_command(cmd)

        while not channel.exit_status_ready():

            time.sleep(0.001)

            if not channel.recv_ready() and not channel.recv_stderr_ready():
                continue

            out = channel.recv(buff_size).decode()
            stdout += out
            self.print(out)

            if out and re.search(r'\[sudo\] password for', out):
                length = channel.send(self.password+'\n')
                if length != len(self.password) + 1:
                    out = 'send passwd failed, ret len is wrong\n'
                    self.print(out)
                    exit_status = -1
                    stderr += out
                    return (exit_status, stdout, stderr)

            while channel.recv_ready():
                out = channel.recv(buff_size).decode()
                stdout += out
                self.print(out)

            while channel.recv_stderr_ready():
                out = channel.recv_stderr(buff_size).decode()
                stderr += out
                self.print(out)
                exit_status = -2

        return (exit_status, stdout, stderr)

    def print(self, out):
        if self.is_print:
            print(out, end='')

    def close(self):
        self.ssh_fd.close()

    def __del__(self):
        self.ssh_fd.close()
        # self.print('connect closed: "{}@{} -p {}"\n'.format(self.user, self.host, self.port))

user = input('Enter user:')
password = getpass.getpass()

shell = RemoteShell(user, password)

shell.exec('ls')

shell.exec('sudo -k dmesg')
