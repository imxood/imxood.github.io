# 查看 dll 或 lib 文件函数定义

## dll文件

打开 powershell for vs 2022, 执行:

dumpbin /exports dll名文件名 > output.txt

dumpbin /headers dll名文件名, 查看库的位数, 在开头: machine (x86)

dumpbin /dependents exe程序, 查看 exe程序需要哪些dll依赖 (如果exe程序 缺失一些 dll时, 打开时 弹框提示 dll确实 或者 是直接闪退)

## c# 动态库?

使用 visual studio 2022 中自带的 ildasm工具, 运行它 会打开一个窗口, 在其中打开 c# dll 文件, 可以看到 dll中导出的接口.

## lib文件

dumpbin /LINKERMEMBER Test.lib > output.txt

## 查看位数

dumpbin /HEADERS 库文件 | findstr machine

