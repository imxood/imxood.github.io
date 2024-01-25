# 查看 dll 或 lib 文件函数定义

## dll文件

打开 powershell for vs 2022, 执行:

dumpbin /exports dll名文件名 > output.txt

dumpbin /headers dll名文件名, 查看库的位数, 在开头: machine (x86)

dumpbin /dependents exe程序, 查看 exe程序需要哪些dll依赖 (如果exe程序 缺失一些 dll时, 打开时 弹框提示 dll确实 或者 是直接闪退)

## lib文件

dumpbin /LINKERMEMBER Test.lib > output.txt

## 查看位数

dumpbin /HEADERS 库文件 | findstr machine
