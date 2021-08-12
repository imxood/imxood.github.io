import os
import sys
import io
import time
import logging
import threading
import stat
import signal
import json
import pickle
from subprocess import Popen, PIPE, STDOUT
from pprint import pprint
from contextlib import contextmanager
import queue

import paramiko
from paramiko import SSHClient, AutoAddPolicy
from paramiko.channel import Channel
from pathlib import Path

import tempfile
import tarfile
from test_common import tlog

log = tlog


class ExceptionHandleThread(threading.Thread):

    def __init__(self, **kwargs):
        super(ExceptionHandleThread, self).__init__(**kwargs)
        self.daemon = True
        self.kwargs = kwargs

    def run(self):
        try:
            super(ExceptionHandleThread, self).run()
        except BaseException:
            self.exc_info = sys.exc_info()
            msg = "Encountered exception {!r} in thread for {!r}"
            name = "_run"
            if "target" in self.kwargs:
                name = self.kwargs["target"].__name__
            print(msg.format(self.exc_info[1], name))


class Runner:

    read_chunk_size = 1000

    def __init__(self):
        self.command_cwds = list()

    def read_proc_stdout(self, num_bytes):
        raise NotImplementedError

    def read_proc_stderr(self, num_bytes):
        raise NotImplementedError

    def stop(self):
        raise NotImplementedError

    @property
    def process_is_finished(self):
        raise NotImplementedError

    @property
    def returncode(self):
        raise NotImplementedError

    @property
    def cwd(self):
        raise NotImplementedError

    def handle_output(self, buffer_, hide, reader, log, msgQ: queue.Queue = None):

        while True:
            data = reader(self.read_chunk_size)
            if not data:
                break
            data = data.decode('utf-8', "replace")
            if not hide:
                log.info(data)
            if msgQ:
                msgQ.put(data)
            buffer_.append(data)

    def handle_stdout(self, buffer_, hide, log, msgQ: queue.Queue = None):
        self.handle_output(buffer_, hide, reader=self.read_proc_stdout, log=log, msgQ=msgQ)

    def handle_stderr(self, buffer_, hide, log, msgQ: queue.Queue = None):
        self.handle_output(buffer_, hide, reader=self.read_proc_stderr, log=log, msgQ=msgQ)

    def _run(self, hide, log: logging.Logger, msgQ: queue.Queue = None):

        thread_args = dict()

        output = []
        error = []

        thread_args[self.handle_stdout] = {
            'buffer_': output,
            'hide': hide,
            'log': log,
            'msgQ': msgQ
        }

        thread_args[self.handle_stderr] = {
            'buffer_': error,
            'hide': hide,
            'log': log,
            'msgQ': msgQ
        }

        for target, kwargs in thread_args.items():
            t = ExceptionHandleThread(target=target, kwargs=kwargs)
            t.start()

        while True:
            ret = self.process_is_finished
            if ret:
                break
            time.sleep(0.01)

        return (self.returncode, ''.join(output), ''.join(error))

    @contextmanager
    def cd(self, path):
        self.command_cwds.append(path)
        try:
            yield
        finally:
            self.command_cwds.pop()

    def _prefix_commands(self, command):
        cwd = self.cwd
        if cwd:
            return 'cd {} && {}'.format(self.cwd, command)
        return command


class Local(Runner):

    def __init__(self, *args, **kwargs):
        self.process = None
        super(Local, self).__init__(*args, **kwargs)

    def run(self, cmd, hide, env=None, log=logging.getLogger(), msgQ: queue.Queue = None):

        command = self._prefix_commands(cmd)

        self.process = Popen(
            command,
            shell=True,
            env=env,
            stdout=PIPE,
            stderr=STDOUT
        )
        return self._run(hide, log=log, msgQ=msgQ)

    def read_proc_stdout(self, num_bytes):
        return os.read(self.process.stdout.fileno(), num_bytes)

    def read_proc_stderr(self, num_bytes):
        return os.read(self.process.stderr.fileno(), num_bytes)

    def stop(self):
        if self.process:
            self.process.kill()

    @property
    def process_is_finished(self):
        if self.process.poll() is None:
            return False
        return True

    @property
    def returncode(self):
        return self.process.returncode

    @property
    def cwd(self):

        if not self.command_cwds:
            self.command_cwds.append(os.getcwd())

        # get the index for the subset of paths starting with the last / or ~
        for i, path in reversed(list(enumerate(self.command_cwds))):
            if path.startswith("~") or path.startswith("/"):
                break
        paths = [path.replace(" ", r"\ ") for path in self.command_cwds[i:]]
        return os.path.join(*paths)


class Remote(Runner):

    def __init__(self, dev_info):
        super(Remote, self).__init__()

        self.dev_info = dev_info

        self.connect_kwargs = dict(
            hostname=dev_info['hostname'],
            port=dev_info['port'],
            username=dev_info['username'],
            timeout=1,
        )

        self.client = SSHClient()
        self.client.set_missing_host_key_policy(AutoAddPolicy())

        self.transport = None
        self.sftp = None
        self.iswindows = False

    def open(self):
        if self.is_connected:
            return
        self.client.connect(**self.connect_kwargs)
        self.transport = self.client.get_transport()

        print('Remote opened')

        if not self.isdir('/tmp'):
            self.iswindows = True

    def close(self):
        if self.is_connected:
            self.client.close()
            self.transport = None
            print('Remote closed')

    def run(self, cmd, hide=False, shell=None, env={}, log=logging.getLogger(), msgQ: queue.Queue = None):
        self.open()
        command = self._prefix_commands(cmd)
        self.channel = self.transport.open_session()

        self.update_environment(env, shell)

        self.channel.exec_command(command)
        ret = self._run(hide, log=log, msgQ=msgQ)
        self.channel.close()
        return ret

    def update_environment(self, env, shell):
        if not shell:
            if self.iswindows:
                shell = r'c:\windows\system64\windowspowershell\v1.0\powershell.exe'
            else:
                shell = '/bin/bash'
        if self.iswindows:
            env['COMSPEC'] = shell
        else:
            env['SHELL'] = shell

        self.channel.update_environment(env)

    def read_proc_stdout(self, num_bytes):
        return self.channel.recv(num_bytes)

    def read_proc_stderr(self, num_bytes):
        return self.channel.recv_stderr(num_bytes)

    @property
    def process_is_finished(self):
        ret = self.channel.exit_status_ready()
        return ret

    @property
    def returncode(self):
        return self.channel.recv_exit_status()

    @property
    def cwd(self):

        if not self.command_cwds:
            self.command_cwds.append(self._sftp.getcwd())

        # get the index for the subset of paths starting with the last / or ~
        for i, path in reversed(list(enumerate(self.command_cwds))):
            if not isinstance(path, str):
                return ""
            if path.startswith("~") or path.startswith("/"):
                break
        paths = [path.replace(" ", r"\ ") for path in self.command_cwds[i:]]
        return os.path.join(*paths)

    def exists(self, path):
        """judge whether the remote path existes
        """
        try:
            self._sftp.stat(path)
        except IOError as e:
            if 'No such file' in str(e):
                return False
            raise
        else:
            return True

    def isdir(self, path):
        """ judge if the remote path existes """
        try:
            return stat.S_ISDIR(self._sftp.stat(path).st_mode)
        except IOError:
            return False

    def mkdir(self, path):
        """ it need is '/' path if the path is multilevel"""
        """ the path look like c:/a/b/c below windows """
        if self.isdir(path):
            raise Exception('the directory specified already exists: ' + path)
        try:
            dirs = path.split('/')
            dir = dirs.pop(0)
            while True:
                if not self.isdir(dir):
                    self._sftp.mkdir(dir)
                if len(dirs) == 0 or len(dirs) == 1 and dirs[0].strip() == '':
                    return
                dir = dir + '/' + dirs.pop(0)
        except Exception as e:
            raise e

    def rm(self, path):
        """ delete remote file, not a dir """
        if self.exists(path) and not self.isdir(path):
            self._sftp.remove(path)

    def rm_dir(self, path):
        """ delete remote dir, not a file """

        if self.exists(path):

            files = self._sftp.listdir(path=path)

            for f in files:
                filepath = '{}/{}'.format(path, f)
                if self.isdir(filepath):
                    self.rm_dir(filepath)
                else:
                    self._sftp.remove(filepath)
            self._sftp.rmdir(path)

    def put(self, local_file, remote):
        """ upload local file to remote, is local file, not a local dir!! """

        ori_local = local_file
        if not os.path.isabs(local_file):
            local_file = os.path.join(os.getcwd(), local_file)

        if not os.path.exists(local_file):
            log.error('local file "{}" doesn\'t exist.'.format(ori_local))
            return False

        try:
            if self.isdir(remote):
                remote = os.path.join(remote, os.path.basename(local_file))
            self._sftp.put(local_file, remote)
            log.info('[local file] {} --> [remote file] {}'.format(local_file, remote))
            return True
        except IOError as e:
            log.error('transfer failed! err: {}'.format(e))
            return False

    def put_dir(self, local_dir, dest_dir):
        """ upload local dir to remote, is local dir, not a local file!! """

        target_dir = Path(dest_dir, Path(local_dir).name)

        if self.exists(target_dir.__str__()):
            self.rm_dir(target_dir.__str__())

        local_tar_file = Path(tempfile.gettempdir(),
                              os.path.basename(local_dir) + '.tar.gz')
        if local_tar_file.exists():
            os.remove(local_tar_file)

        self.__make_targz(local_dir, local_tar_file)

        remote_tar_file = Path(dest_dir, local_tar_file.name).__str__()

        self.put(local_tar_file, remote_tar_file)
        self.run('python3 -m tarfile -e {} {}'.format(remote_tar_file, dest_dir))
        self.rm(remote_tar_file)

        os.remove(local_tar_file)

        log.info('[local dir] {} --> [remote file] {}'.format(local_dir, dest_dir))

        return True

    def __make_targz(self, local_dir, output_filename):
        with tarfile.open(output_filename, 'w:gz') as tar:
            tar.add(local_dir, arcname=os.path.basename(local_dir))

    @property
    def is_connected(self):
        return self.transport.active if self.transport else False

    @property
    def _sftp(self):
        self.open()
        if self.sftp is None:
            self.sftp = self.client.open_sftp()
        return self.sftp


class Shell():

    def __init__(self, dev_info, log=logging.getLogger()):

        self.local_runner = Local()
        self.remote_runner = Remote(dev_info)

        log.debug('Shell __init__')

    def __enter__(self):
        log.debug('Shell __enter__')
        self.remote_runner.open()
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.remote_runner.close()

    def local(self, cmd, hide=False, error_count=1):
        """ run local command """
        ret = None
        while error_count:
            error_count -= 1
            ret = self.local_runner.run(cmd, hide)
            if ret[0] == 0:
                break
        return ret

    def remote(self, cmd, env={}, hide=False, error_count=1, log=logging.getLogger(), msgQ: queue.Queue = None):
        """ run remote command """
        ret = None
        while error_count:
            error_count -= 1
            ret = self.remote_runner.run(cmd, hide, env=env, log=log, msgQ=msgQ)
            if ret[0] == 0:
                break
            log.warning('run [{}] failed, out: {}, err: {}'.format(cmd, ret[1], ret[2]))
            if error_count > 0:
                time.sleep(0.1)
                log.warning('retry...')
        return ret

    @property
    def cd_local(self):
        return self.local_runner.cd

    @property
    def cwd_local(self):
        return self.local_runner.cwd

    @property
    def cd_remote(self):
        return self.remote_runner.cd

    @property
    def cwd_remote(self):
        return self.remote_runner.cwd

    @property
    def is_connected(self):
        return self.remote_runner.is_connected

    def reconnect(self, timeout_s=60):
        while timeout_s > 0:
            """ open will timeout after 1s if no connection """
            try:
                self.remote_runner.open()
            except Exception as e:
                timeout_s -= 1
                if timeout_s % 10 == 0:
                    log.info('reconnect ... %ds' % timeout_s)
            if self.is_connected:
                break
        if self.is_connected:
            log.info('reconnect ok!')
            return True
        else:
            log.error('reconnect failed!')
            return False

    def exists(self, path):
        """judge whether the remote path existes
        """
        return self.remote_runner.exists(path)

    def isdir(self, path):
        return self.remote_runner.isdir(path)

    def mkdir(self, path):
        self.remote_runner.mkdir(path)

    def rm(self, path):
        self.remote_runner.rm(path)

    def rm_dir(self, path):
        self.remote_runner.rm_dir(path)

    def put(self, local_file, remote):
        return self.remote_runner.put(local_file, remote)

    def put_dir(self, local_dir, dest_dir):
        return self.remote_runner.put_dir(local_dir, dest_dir)


def test():

    dev_info = {
        'hostname': '127.0.0.1',
        'username': 'maxu',
        'port': 22
    }

    with Shell(dev_info) as s:
        s.remote('echo %COMSPEC%', env={'A': 'a'})
        s.isdir('/tmp')
        s.mkdir('test1/test2/test3')
        with s.cd_remote('test1'):
            s.put_dir('tools', 'test1')
            s.remote('ls')
            with s.cd_remote('test2'):
                s.remote('ls')
                with s.cd_remote('test3'):
                    s.remote('ls -al', hide=True)


if __name__ == "__main__":

    try:
        test()
    except KeyboardInterrupt as err:
        print(err)
