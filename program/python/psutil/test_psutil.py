import psutil as ps
from pprint import pprint


if __name__ == "__main__":

    try:
        """ need privilege """
        for proc in ps.process_iter():
            print(proc.as_dict())
        procs = {p.pid: p.info for p in ps.process_iter(
            ['pid', 'name', 'username', 'cmdline'])}
        pprint(procs)
    except Exception as e:
        print(e)

    for pid in ps.pids():
        try:
            proc = ps.Process(pid)
            print(proc.name())
            print(proc.username())
            print(proc.cmdline())
        except Exception as e:
            print(e)
