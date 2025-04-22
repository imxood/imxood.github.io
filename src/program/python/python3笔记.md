# python3 笔记

## python 库

-   readchar, 读取键盘输入的通用库
-   fabric, paramiko
    execute shell commands remotely over SSH

## python 环境

### vscode 调试 python 多进程

    multiprocessing.set_start_method('spawn', True)

    在launch.json文件中添加:
        "subProcess": true

### install lastest pip

    curl https://bootstrap.pypa.io/get-pip.py | python3 - --user

### install python

    sudo add-apt-repository ppa:deadsnakes/ppa
    sudo apt update && sudo apt install -y python3.7

    cd /usr/lib/python3/dist-packages
    sudo ln -s apt_pkg.cpython-{35m,37m}-x86_64-linux-gnu.so

    sudo update-alternatives --install /usr/bin/python3 python3 /usr/bin/python3.5 1
    sudo update-alternatives --install /usr/bin/python3 python3 /usr/bin/python3.7 2

    sudo update-alternatives --config python3

### source install python3.6

    wget https://www.python.org/ftp/python/3.6.2/Python-3.6.2.tar.xz
    tar -xvf Python-3.6.2.tar.xz
    cd Python-3.6.2
    ./configure
    make
    make install

### 使用 pyenv 管理 python

    sudo apt-get install --no-install-recommends make build-essential libssl-dev zlib1g-dev libbz2-dev libreadline-dev libsqlite3-dev wget curl llvm libncurses5-dev xz-utils tk-dev libxml2-dev libxmlsec1-dev libffi-dev liblzma-dev

    curl https://pyenv.run | bash

    pyenv install -v -g 3.7.4

    pyenv virtualenv 3.7.4-debug py3.7

    pyenv activate py3.7

### 搜索包是否被安装

方法 1:

    try:
        from pip._internal.utils.misc import get_installed_distributions
    except ImportError:  # pip<10
        from pip import get_installed_distributions

方法 2:

    import pkg_resources

    dists = [d for d in pkg_resources.working_set]

## python 开发笔记

len(obj), 计算字符串, 列表, 元组, 字典等类型的长度

type(obj), 判断对象的类型

isinstance(obj, class_name), 判断是否是指定类型的变量

### 字符串常见用法

### 元组

### 字典

### 列表

### json

### shutil

    shutil.copytree(src, dst, symlinks=False), 复制目录
    shutil.copytree(src, dst, ignore=shutil.ignore_patterns('*~', '*.pyc')), 忽略指定类型的文件

### python 版本

    os.name, 当前操作系统的类型，当前只注册了3个值：posix , nt , java, (linux/windows/java虚拟机)

    sys.platform, "linux", "win32"

    platform.system(), 系统名称('Linux', 'Windows', or 'Java'): 'Linux', 如果无法确定就返回None

    platform.platform(), 'Linux-4.15.0-55-generic-x86_64-with-Ubuntu-16.04-xenial'

    platform.version(), Ubuntu中, 获取操作系统版本号: '#60~16.04.2-Ubuntu SMP Thu Jul 4 09:03:09 UTC 2019'

    platform.architecture(), Ubuntu中: ('64bit', 'ELF')

    platform.machine(), 'x86_64'

    platform.node(), '主机名', 计算机的网络名称

    platform.processor(), 'x86_64', 计算机处理器信息

    platform.uname(), 包含上面所有的信息汇总

## 报错

    Run: python3 get-pip.py --user

    Error: Could not fetch URL https://pypi.tuna.tsinghua.edu.cn/simple/pip/: There was a problem confirming the ssl certificate

    Solved:

        sudo apt-get install apt-transport-https ca-certificates software-properties-common

        或

        python3 get-pip.py --user --trusted-host pypi.tuna.tsinghua.edu.cn

## pip 升级问题

    Traceback (most recent call last):
    File "/usr/bin/pip3", line 9, in <module>
        from pip import main
    ImportError: cannot import name 'main'

    curl https://bootstrap.pypa.io/get-pip.py | python3 - --user

## pip setuptools 版本太低

    pkg_resources.VersionConflict: (setuptools 20.7.0 (/usr/lib/python3/dist-packages), Requirement.parse('setuptools>=40.0'))
    pip3 install -U --user setuptools

## 使用 matplotlib 时 报错

UserWarning: FigureCanvasAgg is non-interactive, and thus cannot be shown
plt.show()
