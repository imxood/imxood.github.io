# windows 工具

## 终端工具

    CMDER, http://cmder.net

    Windows Terminal

## 杀死指定进程

    根据名称:
        taskkill -f -PID ProgessName

## PowerShell 命令

    .\python-3.9.2-amd64.exe /passive InstallAllUsers=1 PrependPath=1 Include_test=0 Include_tcltk=0 TargetDir=$python_dir
    Start-Sleep -Seconds 1
    $p = Get-Process -Name "python-3.9.2-amd64"
    Wait-Process -Id $p.id

    $p = Start-Process ".\putty-64bit-0.74-installer.msi" -ArgumentList '/passive INSTALLDIR=$putty_dir' -PassThru -Wait
    Print-Status "putty" $p.ExitCode $p.StandardError

## 包管理工具

    vcpkg choco

## 创建服务工具

https://github.com/tickbh/wmproxy.git
