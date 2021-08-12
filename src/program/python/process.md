
## 创建进程

1. 阻塞

    # 此处的两个PIPE是必须的, 因为下面用管道通信
    with sp.Popen(cmd, stdout=sp.PIPE, stderr=sp.PIPE, shell=True) as proc:

        stdout, stderr = proc.communicate()
        proc.wait()

        lines = str(stdout, encoding='utf-8').split('\n')

2. 即时

    with sp.Popen(cmd, stdout=sp.PIPE, stderr=sp.STDOUT, shell=True) as proc:

        for line in iter(proc.stdout.readline, b''):
            line = str(line, encoding='utf-8').rstrip()
