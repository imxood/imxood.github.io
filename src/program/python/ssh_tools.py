
import os
import sys
from paramiko import SSHClient, AutoAddPolicy
import time
import logging
import threading
from contextlib import contextmanager
from subprocess import Popen, PIPE
import signal


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
        pass

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

    def read_proc_output(self, reader):
        while True:
            data = reader(self.read_chunk_size)
            if not data:
                break
            yield data.decode('utf-8', "replace")

    def write_our_output(self, stream, string):
        stream.write(string)
        stream.flush()

    def _handle_output(self, buffer_, hide, output, reader):
        for data in self.read_proc_output(reader):
            if not hide:
                self.write_our_output(stream=output, string=data)
            buffer_.append(data)

    def handle_stdout(self, buffer_, hide, output):
        self._handle_output(
            buffer_, hide, output, reader=self.read_proc_stdout
        )

    def handle_stderr(self, buffer_, hide, output):
        self._handle_output(
            buffer_, hide, output, reader=self.read_proc_stderr
        )

    def _run(self, hide, out_stream=sys.stdout, err_stream=sys.stderr):

        thread_args = dict()

        output = []
        error = []

        thread_args[self.handle_stdout] = {
            'buffer_': output,
            'hide': hide,
            'output': out_stream
        }

        thread_args[self.handle_stderr] = {
            'buffer_': error,
            'hide': hide,
            'output': err_stream
        }

        for target, kwargs in thread_args.items():
            t = ExceptionHandleThread(target=target, kwargs=kwargs)
            t.start()

        while True:
            if self.process_is_finished:
                break
            time.sleep(0.01)

        self.stop()

        return (self.returncode, ''.join(output), ''.join(error))


class Local(Runner):

    def __init__(self, *args, **kwargs):
        super(Local, self).__init__(*args, **kwargs)

    def run(self, command, hide, env=None):
        self.process = Popen(
            command,
            shell=True,
            env=env,
            stdout=PIPE,
            stderr=PIPE,
            stdin=PIPE,
        )
        super()._run(hide)

    def read_proc_stdout(self, num_bytes):
        return os.read(self.process.stdout.fileno(), num_bytes)

    def read_proc_stderr(self, num_bytes):
        return os.read(self.process.stderr.fileno(), num_bytes)

    def stop(self):
        self.process.kill()

    @property
    def process_is_finished(self):
        if self.process.poll() is None:
            return False
        return True

    @property
    def returncode(self):
        return self.process.returncode


class Remote(Runner):

    def __init__(self, *args, **kwargs):
        super(Remote, self).__init__(*args, **kwargs)

    def run(self, command, channel, hide):
        self.channel = channel
        self.channel.exec_command(command)
        super()._run(hide)

    def read_proc_stdout(self, num_bytes):
        return self.channel.recv(num_bytes)

    def read_proc_stderr(self, num_bytes):
        return self.channel.recv_stderr(num_bytes)

    def stop(self):
        self.channel.close()

    @property
    def process_is_finished(self):
        return self.channel.exit_status_ready()

    @property
    def returncode(self):
        return self.channel.recv_exit_status()


class RemoteShell:

    def __init__(self, hostname='127.0.0.1', port=22, username='root', password=None, log=None):

        self.connect_kwargs = dict(
            hostname=hostname,
            port=port,
            username=username,
            password=password
        )

        self.log = log if log is not None else logging.getLogger(__name__)
        self.command_cwds = list()
        
        self.local_runner = Local()
        self.remote_runner = Remote()

    def __enter__(self):

        self.client = SSHClient()
        self.client.set_missing_host_key_policy(AutoAddPolicy())

        self.transport = None
        self._sftp = None

        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.close()

    def _open(self):
        if self.is_connected:
            return
        self.client.connect(**self.connect_kwargs)
        self._transport = self.client.get_transport()

    @contextmanager
    def cd(self, path):
        self.command_cwds.append(path)
        try:
            yield
        finally:
            self.command_cwds.pop()

    @property
    def sftp(self):
        self._open()
        if self._sftp is None:
            self._sftp = self.client.open_sftp()
        return self._sftp

    @property
    def cwd(self):

        if not self.command_cwds:
            return ""

        # get the index for the subset of paths starting with the last / or ~
        for i, path in reversed(list(enumerate(self.command_cwds))):
            if path.startswith("~") or path.startswith("/"):
                break

        paths = [path.replace(" ", r"\ ") for path in self.command_cwds[i:]]
        return os.path.join(*paths)

    def put(self, local_file='', remote_file='', recursive=False):
        """upload files or directory to remote server
        """
        try:
            if os.path.isdir(local_file):
                if recursive == False:
                    return False

                name = os.path.basename(local_file)

                local_root = local_file

                if name != os.path.basename(remote_file):
                    remote_root = '{}/{}'.format(remote_file, name)
                else:
                    remote_root = remote_file

                if not self.exist(remote_root):
                    self.mkdir(remote_root)

                for root, dirs, files in os.walk(local_root):
                    for dir in dirs:
                        remote_dir = os.path.relpath('{}/{}'.format(root, dir), local_root)
                        remote_dir = '{}/{}'.format(remote_root, remote_dir)
                        self.mkdir(remote_dir)
                    for file in files:
                        local_file = '{}/{}'.format(root, file)
                        remote_file = os.path.relpath(local_file, local_root)
                        remote_file = '{}/{}'.format(remote_root, remote_file)
                        self.sftp.put(local_file, remote_file)
                return True

            self.sftp.put(local_file, remote_file)
            return True

        except IOError as e:
            self.log.debug('transfer failed! {}'.format(e))
            return False

    def exist(self, path):
        """judge whether the path existes
        """
        try:
            self.sftp.stat(path)
        except IOError as e:
            if 'No such file' in str(e):
                return False
            raise
        else:
            return True

    def mkdir(self, path, ignore_existed=True):
        """make a directory, if ignore_existed == True, it will happen error if the dir existes
        """
        try:
            self.sftp.mkdir(path)
        except IOError:
            if ignore_existed:
                pass
            else:
                raise

    def local(self, cmd, hide=False):
        '''run local command
        '''
        command = cmd

        self.local_runner.run(command, hide)

    def remote(self, cmd, hide=False):
        '''run remote command
        '''
        self._open()

        command = self._prefix_commands(cmd)

        channel = self.client.get_transport().open_session()

        self.remote_runner.run(command, channel, hide)

    @property
    def is_connected(self):
        return self.transport.active if self.transport else False

    def close(self):
        if self.is_connected:
            self.client.close()

    def _prefix_commands(self, command):
        return 'cd {} && {}'.format(self.cwd, command)


def test():

    with RemoteShell(port=2222) as s:
        s.local('ls -al')
        s.remote('mkdir -p test1/test2/test3')
        s.remote('ls -al')
        with s.cd('test1'):
            s.put('web', 'test1', recursive=True)
            s.remote('ls -al')
            with s.cd('test2'):
                s.remote('ls -al')
                with s.cd('test3'):
                    s.remote('ls -al', hide=True)
        s.remote('dmesg')
        # s.sftp.mkdir('test1_1')


if __name__ == "__main__":

    test()
