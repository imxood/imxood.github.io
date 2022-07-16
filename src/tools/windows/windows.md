## 中控机报错: 应用程序无法正常启动 0xc000007b

[参考: How To Fix Error 0xc000007b in Windows 10](https://www.youtube.com/watch?v=kzYbDPxdMbo)

http://www.mediafire.com/file/97erhoox4d3zabq/aio210.zip/file

### 查看可执行程序依赖库

http://dependencywalker.com/

###

https://docs.microsoft.com/en-us/sysinternals/downloads/procmon

上面的办法都没什么用, 我遇到的问题, 64位dll与32位dll

## 环境变量

### cmd

设置临时环境变量

    set A=a/b/c
    set A=a/b/c;%A%

### PowerShell

查看 环境变量列表

    ls env:

    $env:PATH

设置临时环境变量

    $env:A="a/b/c"
    $env:A="a/b/c;$env:A"
