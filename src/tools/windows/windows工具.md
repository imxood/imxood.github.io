# Windows 工具

## 说明

- 本页汇总 Windows 下常用终端工具, 进程管理命令和若干自动化安装片段.
- 内容以日常开发环境整理为主, 适合作为“工具与命令入口页”.

## 终端工具

- `Cmder`: <https://cmder.net>
- `Windows Terminal`

## 进程管理

按名称或 PID 结束进程:

```bat
taskkill /f /pid <PID>
taskkill /f /im <ProcessName.exe>
```

## PowerShell 安装片段

### 静默安装 Python 示例

```powershell
.\python-3.9.2-amd64.exe /passive InstallAllUsers=1 PrependPath=1 Include_test=0 Include_tcltk=0 TargetDir=$python_dir
Start-Sleep -Seconds 1
$p = Get-Process -Name "python-3.9.2-amd64"
Wait-Process -Id $p.id
```

### 静默安装 MSI 示例

```powershell
$p = Start-Process ".\putty-64bit-0.74-installer.msi" -ArgumentList '/passive INSTALLDIR=$putty_dir' -PassThru -Wait
Print-Status "putty" $p.ExitCode $p.StandardError
```

## 包管理工具

- [Chocolatey](./chocolatey.md)
- [vcpkg](./vcpkg.md)

## 服务与远程

- [NSSM](./nssm.md)
- [Windows 安装 OpenSSH Server](./安装openssh%20server.md)

## 其他工具线索

- 内存泄露分析: `Dr. Memory`
- 历史项目记录: <https://github.com/tickbh/wmproxy.git>

## 使用建议

- 本页适合保留“工具入口 + 最小命令 + 跳转关系”, 不适合堆积过长系统设置细节.
- 若某个工具已经形成独立知识页, 本页只保留导航和最小提示.
- 与系统维护相关的更长记录, 应回到 [Windows 笔记](./windows.md).
