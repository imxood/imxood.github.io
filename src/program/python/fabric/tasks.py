import os
import stat
import sys
import tempfile

import coloredlogs
coloredlogs.install(level='INFO', fmt='%(asctime)s %(name)s %(levelname)s %(message)s', datefmt='%m-%d %H:%M:%S')

from invoke import task
from fabric import Connection
from pathlib import Path
import tarfile

import paramiko
from paramiko import SSHClient, AutoAddPolicy
from paramiko.channel import Channel

import logging

from logging import Logger

# log = logging.getLogger('paramiko.transport')
# log.setLevel(logging.INFO)


curr_dir = Path(__file__).parent


logger = logging.getLogger('ssh.tool')

class SshTool():

    def __init__(self, dev_info, logger: Logger=logger):

        # self.dev_info = dev_info

        self.connect_info = dict(
            host=dev_info['hostname'],
            user=dev_info['username'],
            connect_kwargs={'password': dev_info['password']}
        )

        self.logger = logger

    def __enter__(self):

        self.client = Connection(**self.connect_info)
        self.sftp = self.client.sftp()

        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.close()

    def close(self):
        self.client.close()

    def cd(self, path):
        return self.client.cd(path)

    def run(self, command, **kwargs):
        return self.client.run(command, **kwargs)

    def local(self, *args, **kwargs):
        return self.client(*args, **kwargs)

    def exists(self, path):
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

    def isdir(self, path):
        try:
            return stat.S_ISDIR(self.sftp.stat(path).st_mode)
        except IOError:
            return False

    def mkdir(self, path, ignore_existed=True):
        """make a directory, if ignore_existed == True, it will happen error if the dir existes
        """
        try:
            if self.exists(path):
                return False
            self.sftp.mkdir(path)
            return True
        except IOError as e:
            if ignore_existed:
                pass
            else:
                return False
        return True

    def rm(self, path):
        if self.exists(path) and not self.isdir(path):
            self.sftp.remove(path)

    def rm_dir(self, path):

        if self.exists(path):

            files = self.sftp.listdir(path=path)

            for f in files:
                filepath = '{}/{}'.format(path, f)
                if self.isdir(filepath):
                    self.rm_dir(filepath)
                else:
                    self.sftp.remove(filepath)
            self.sftp.rmdir(path)

    def put(self, *args, **kwargs):
        try:
            self.client.put(*args, **kwargs)
            return True
        except IOError as e:
            self.logger.error('transfer failed! err: {}'.format(e))
            return False

    def put_dir(self, source_dir, dest_dir):
        
        target_dir = Path(dest_dir, Path(source_dir).name)
        if self.exists(target_dir.__str__()):
            self.rm_dir(target_dir.__str__())

        local_tar_file = Path(tempfile.gettempdir(), os.path.basename(source_dir) + '.tar.gz')
        if local_tar_file.exists():
            os.remove(local_tar_file)

        self.__make_targz(source_dir, local_tar_file)

        remote_tar_file = Path(dest_dir, local_tar_file.name).__str__()

        self.put(local_tar_file, dest_dir)
        self.run('python3 -m tarfile -e {} {}'.format(remote_tar_file, dest_dir))
        self.rm(remote_tar_file)
        
        os.remove(local_tar_file)
        
        return True

    def __make_targz(self, source_dir, output_filename):
        with tarfile.open(output_filename, 'w:gz') as tar:
            tar.add(source_dir)


@task
def clean(c, docs=True, bytecode=False, extra=''):
    patterns = ['build']
    if docs:
        patterns.append('docs/_build')
    if bytecode:
        patterns.append('**/*.pyc')
    if extra:
        patterns.append(extra)
    for pattern in patterns:
        c.run("rm -rf {}".format(pattern))

@task
def build(c, docs=False):
    c.run("python setup.py build")
    if docs:
        c.run("sphinx-build docs docs/_build")

@task
def upload(c):

    dev_info = {
        "hostname": "10.239.137.60",
        "port": 22,
        "username": "pi",
        "password": "raspberry",
    }
    with SshTool(dev_info) as ssh:
        # output_file = curr_dir / 'test.tar.gz'
        # source_dir = curr_dir.parent / 'app'
        # make_targz(output_file, source_dir)
        ssh.put_dir('tests', '/tmp')
        pass
        # put_dir(c, 'tests', 'validation')
