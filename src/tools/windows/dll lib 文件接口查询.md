# 查看 dll 或 lib 文件函数定义

## 说明

- 本页记录 Windows 下查看动态库和静态库导出符号的常用方式.
- 对排查 ABI, 位数不匹配和缺失依赖很有帮助.

## 查看 DLL 导出函数

打开 `Developer PowerShell for VS 2022` 或带有 `dumpbin` 的开发者命令行:

```powershell
dumpbin /exports demo.dll > output.txt
```

查看库的位数:

```powershell
dumpbin /headers demo.dll
```

输出开头常能看到 `machine (x86)` 或 `machine (x64)`.

## 查看 EXE 依赖的 DLL

```powershell
dumpbin /dependents app.exe
```

- 当程序启动时提示缺少 DLL 或直接闪退时, 这条命令很实用.

## 查看 C# DLL

- 可使用 Visual Studio 自带的 `ildasm` 打开 .NET 程序集.
- 适合查看托管 DLL 中的类型, 方法和元数据结构.

## 查看 LIB 导出成员

```powershell
dumpbin /linkermember Test.lib > output.txt
```

## 快速查看位数

```powershell
dumpbin /headers Test.lib | findstr machine
```
