from fabric import Connection


c = Connection('mx@xxxx', connect_kwargs={'password': 'xxxx'})

# c.put('py', '/home/mx/test')
# c.run('ls -al ~/test')

with c.cd('~'):
    c.run('pwd')

with c.cd('~/test'):
    c.run('pwd')
