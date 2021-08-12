import os
import sys
import time
import platform
import pkg_resources
import logging
import subprocess as sp
import multiprocessing as mp

mp.set_start_method('spawn', True)

logging.basicConfig(level=logging.INFO, format='%(asctime)s,%(msecs)03d %(message)s', datefmt='%H:%M:%S')
# paramiko.common.logging.basicConfig(level=paramiko.common.DEBUG)

def get_log(name, level=logging.DEBUG):

    logger = logging.getLogger(name)
    logger.setLevel(level)

    try:
        import coloredlogs
        if logger.handlers and isinstance(logger.handlers[0], coloredlogs.StandardErrorHandler):
            return logger

        coloredlogs.install(logger=logger,
                            fmt='%(asctime)s,%(msecs)03d %(message)s', datefmt='%H:%M:%S')
    except Exception:
        pass

    return logger


def run_cmd(cmd, env=os.environ, logger=logging):
    '''run shell command'''

    logger.info('run cmd: %s' % cmd)
    try:
        with sp.Popen(cmd, shell=True, stdout=sp.PIPE, stderr=sp.STDOUT, bufsize=1, env=env) as p:
            for line in p.stdout:
                logger.info(line.decode().strip())
            return p.poll()
    except KeyboardInterrupt as ki:
        logger.error('KeyboardInterrupt')
        sys.exit(-1)
    except SystemExit as se:
        logger.error('SystemExit')
        sys.exit(-2)


def pip_requires_install(require_packages=[], logger=logging):
    """ install pip dependency """

    if not require_packages:
        return

    install_packages = []

    package_set = pkg_resources.working_set

    for package in require_packages:
        if package not in package_set.by_key:
            install_packages.append(package)

    if not install_packages:
        return

    logger.info('required pip package: {}'.format(require_packages))

    cmd = '{} -m pip install --user {} -i https://pypi.tuna.tsinghua.edu.cn/simple --proxy=http://child-prc.intel.com:913'

    package_failed = []

    for p in install_packages:
        if run_cmd(cmd.format(sys.executable, p)):
            package_failed.append(p)

    if len(package_failed) > 0:
        logger.error('\tpackage [{}] install failed'.format(package_failed))
        logger.error('you can install these packages manually.')
        sys.exit(-1)

    logger.info('all packages is installed successfully')


class MulProcess(mp.Process):

    def __init__(self, *args, **kwargs):

        super().__init__(*args, **kwargs)

        self.prio = 0
        self.depends = list()

        # fatal: 0, warning:1, info: 2, debug: 3
        self.error_level = 0

    def add_depend(self, processes):
        if isinstance(processes, list):
            self.depends.extend(processes)
        elif isinstance(processes, MulProcess):
            self.depends.append(processes)

    def stop(self):

        for p in self.depends:
            if p.is_alive():
                p.terminate()

        if self.is_alive():
            self.terminate()


class Scheduler:

    def __init__(self):
        self.processes = list()
        self.logger = get_log(__name__)

    def push(self, process):
        if isinstance(process, list):
            self.processes.extend(process)
        elif isinstance(process, MulProcess):
            self.processes.append(process)

    def stop(self, processe=None):

        if processe is None:

            for p in self.processes:
                p.terminate()
                self.logger.error("terminated process [{}]".format(p.name))

        else:

            for p in processe.depends:
                if p.is_alive():
                    p.terminate()
                if p in self.processes:
                    self.processes.remove(p)

            if processe in self.processes:
                self.processes.remove(processe)

    def run(self):

        for p in self.processes:
            if not p.is_alive():
                p.start()

        while True:

            need_stop = True

            for p in self.processes[:]:

                if p.is_alive():
                    need_stop = False
                    continue

                if p.exitcode == 0:
                    self.processes.remove(p)

                elif p.exitcode:

                    self.stop(p)

                    if p.error_level == 0:
                        self.logger.error('process [{}] failed'.format(p.name))
                        self.stop()
                        sys.exit(-1)
                    if p.error_level == 1:
                        self.logger.warning('process [{}] failed'.format(p.name))

            if need_stop or len(self.processes) == 0:
                break

            time.sleep(0.02)

        return 0
