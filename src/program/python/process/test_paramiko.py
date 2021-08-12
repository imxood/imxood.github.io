import os
import sys
import asyncio
import stat
import tempfile
import logging
import tarfile
import io
from pathlib import Path
from contextlib import contextmanager

from paramiko import SSHClient, AutoAddPolicy

logging.basicConfig(level=logging.INFO, format='%(asctime)s [%(name)s]    %(message)s')


class Remote():

    def __init__(self, username, password=None, hostname='127.0.0.1', port=22, timeout=1, log=logging.getLogger()):

        self.command_cwds = list()
        self.log = log

        self.connect_kwargs = dict(
            hostname=hostname,
            port=port,
            username=username,
            timeout=timeout,
        )
        if not password:
            self.connect_kwargs['password'] = password

        self.client = SSHClient()
        self.client.set_missing_host_key_policy(AutoAddPolicy())

        self.transport = None
        self.is_opened_sftp = False
        self.iswindows = False

    def __enter__(self):
        self.open()
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.close()

    def open(self):
        if self.is_connected:
            return
        self.client.connect(**self.connect_kwargs)
        self.transport = self.client.get_transport()

        if not self.isdir('/tmp'):
            self.iswindows = True

    def close(self):
        if self.is_connected:
            self.client.close()
            self.transport = None

    @property
    def is_connected(self):
        return self.transport.active if self.transport else False

    @property
    def sftp(self):
        self.open()
        if not hasattr(self, '_sftp') or getattr(self, '_sftp') is None:
            self._sftp = self.client.open_sftp()
        return self._sftp

    @property
    def cwd(self):

        if not self.command_cwds:
            self.command_cwds.append(self.sftp.getcwd())

        # get the index for the subset of paths starting with the last / or ~
        for i, path in reversed(list(enumerate(self.command_cwds))):
            if not isinstance(path, str) and not isinstance(path, Path):
                return ""
            path = str(path)
            if path.startswith("~") or path.startswith("/"):
                break
        paths = [str(path).replace(" ", r"\ ") for path in self.command_cwds[i:]]
        return os.path.join(*paths)

    @contextmanager
    def cd(self, path):
        self.command_cwds.append(path)
        try:
            yield
        finally:
            self.command_cwds.pop()

    def _get_lines(self, stringio, text: str, terminator='\n'):
        pos = stringio.tell()
        if pos != 0:
            stringio.write(text)
            text = stringio.getvalue()
            stringio.seek(0)
            stringio.truncate()
        lines = text.split(terminator)
        if lines[-1] != '':
            stringio.write(lines[-1])
            return lines[:-1]
        else:
            stringio.seek(0)
            stringio.truncate()
            return lines

    async def recv(self, deal_func, num_bytes, hide, bufferQ, terminator='\n'):
        buffer = io.StringIO()
        while True:
            # if the data is empty, the transmission ends
            data = deal_func(num_bytes)
            if not data:
                if bufferQ:
                    bufferQ.task_done()
                break
            data = data.decode('utf-8', "replace")
            lines = self._get_lines(buffer, data, terminator)
            for line in lines:
                if line:
                    if bufferQ:
                        await bufferQ.put(line)
                    if not hide:
                        self.log.info(line)
            # if len(lines) > 1 and lines[-1] == '':
            #     self.lastline = lines[-2]
        return data

    async def run(self, cmd, hide=False, timeout=30, bufferQ: asyncio.Queue = None, num_bytes=1024):
        self.open()
        self.log.info('run cmd: %s', cmd)
        command = self._prefix_commands(cmd)
        self.log.debug('real cmd: %s', command)
        # open channel
        channel = self.transport.open_session()
        # set exec_command's timeout
        channel.settimeout(timeout)
        # exec command
        channel.exec_command(command)
        # wait the task's output
        output_future = asyncio.ensure_future(self.recv(channel.recv, num_bytes, hide, bufferQ))
        output_err_future = asyncio.ensure_future(self.recv(channel.recv_stderr, num_bytes, hide, bufferQ))
        output, output_err = await asyncio.gather(output_future, output_err_future)
        # get tha command return code
        retcode = channel.recv_exit_status()
        # close channel
        channel.close()
        return (retcode, output, output_err)

    def isdir(self, path):
        """ judge if the remote path existes """
        try:
            return stat.S_ISDIR(self.sftp.stat(str(path)).st_mode)
        except IOError:
            return False

    def mkdir(self, path):
        """ it need is '/' path if the path is multilevel"""
        """ the path look like c:/a/b/c below windows """
        # if self.isdir(path):
        #     raise Exception('the directory specified already exists: ' + path)
        try:
            dirs = path.__str__().split('/')
            dir = dirs.pop(0)
            while True:
                if not self.isdir(dir):
                    self.sftp.mkdir(dir)
                if len(dirs) == 0 or len(dirs) == 1 and dirs[0].strip() == '':
                    return
                dir = dir + '/' + dirs.pop(0)
        except Exception as e:
            raise e

    def rm(self, path):
        """ delete remote file, not a dir """
        if self.exists(path) and not self.isdir(path):
            self.sftp.remove(path)

    def rmdir(self, path):
        """ delete remote dir, not a file """

        if self.exists(path):

            files = self.sftp.listdir(str(path))

            for f in files:
                filepath = '{}/{}'.format(path, f)
                if self.isdir(filepath):
                    self.rmdir(filepath)
                else:
                    self.sftp.remove(filepath)
            self.sftp.rmdir(str(path))

    def put(self, local_file, remote_file, hide=False):
        """ upload local file to remote, is local file, not a local dir!! """
        local_file = Path(local_file)
        ori_local = local_file
        remote_file = Path(remote_file)
        if not local_file.is_absolute():
            local_file = Path(os.getcwd(), local_file)

        if not local_file.exists():
            self.log.error('local file "{}" doesn\'t exist.'.format(ori_local))
            return False

        try:
            if not self.isdir(remote_file.parent):
                self.mkdir(remote_file.parent)
            self.sftp.put(str(local_file), str(remote_file))
            if not hide:
                self.log.info('[local file] {} --> [remote file] {}'.format(local_file, remote_file))
            return True
        except IOError as e:
            self.log.error('transfer failed! err: {}'.format(e))
            return False

    async def put_dir(self, local_dir, dest_dir, hide=False):
        """ upload local dir to remote, is local dir, not a local file!! """

        local_dir = Path(local_dir)
        dest_dir = Path(dest_dir)

        target_dir = dest_dir / local_dir.name

        if self.isdir(target_dir):
            self.rmdir(target_dir)

        local_tar_file = Path(tempfile.gettempdir(), local_dir.name + '.tar.gz')
        if local_tar_file.exists():
            local_tar_file.unlink()

        self._make_targz(local_dir, local_tar_file)

        remote_tar_file = Path(dest_dir, local_tar_file.name).__str__()

        self.put(local_tar_file, remote_tar_file, hide=True)
        await self.run('python3 -m tarfile -e {} {}'.format(remote_tar_file, dest_dir))
        self.rm(remote_tar_file)

        local_tar_file.unlink()
        if not hide:
            self.log.info('[local dir] {} --> [remote dir] {}'.format(local_dir, dest_dir))

        return True

    def exists(self, path):
        """judge whether the remote path existes
        """
        try:
            self.sftp.stat(str(path))
        except IOError as e:
            if 'No such file' in str(e):
                return False
            raise
        else:
            return True

    def _make_targz(self, local_dir, output_filename):
        with tarfile.open(output_filename, 'w:gz') as tar:
            tar.add(local_dir, arcname=os.path.basename(local_dir))

    def _prefix_commands(self, command):
        cwd = self.cwd
        if cwd:
            return 'cd {} && {}'.format(self.cwd, command)
        return command


console = logging.StreamHandler()
console.setLevel(logging.DEBUG)
console.setFormatter(logging.Formatter('%(asctime)s [%(name)s]    %(message)s'))

fileHandler = logging.FileHandler(tempfile.gettempdir() + '/test.log', 'w')
fileHandler.setLevel(logging.DEBUG)
fileHandler.setFormatter(logging.Formatter('%(message)s'))

slog = logging.Logger('serial')
slog.addHandler(console)
slog.addHandler(fileHandler)

tlog = logging.getLogger()


async def serial_task():

    bufferQ = asyncio.Queue()
    log = logging.getLogger()

    with Remote('maxu', log=log) as r:

        log.info('serial ...')

        command = 'plink -serial /dev/ttyUSB0 -sercfg 115200,8,n,1,N'

        task = asyncio.ensure_future(r.run(command, bufferQ=bufferQ))

        while True:
            try:
                line = await asyncio.wait_for(bufferQ.get(), timeout=10)
                log.info(line)
            except asyncio.QueueEmpty:
                await task.cancel()
                break
        return 0


async def main():
    with Remote('maxu', log=logging.getLogger()) as r:
        ret = await r.run('ls -al')
        print(ret)
    await serial_task()


if __name__ == "__main__":
    asyncio.run(main())
