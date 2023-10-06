# 查看 dll 或 lib 文件函数定义

## dll文件

打开 powershell for vs 2022, 执行:

dumpbin /exports dll名文件名 > output.txt

dumpbin /headers dll名文件名, 查看库的位数, 在开头: machine (x86)

## lib文件

dumpbin /LINKERMEMBER Test.lib > output.txt
