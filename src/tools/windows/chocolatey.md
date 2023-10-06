## 使用管理员权限

Set-ExecutionPolicy Bypass -Scope Process -Force; iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))

choco -v

### 避免频繁的确认信息

choco feature enable -n allowGlobalConfirmation

## 安装到指定路径

设置环境变量

安装文件路径 (安装过程中的下载文件/中间文件):

ChocolateyInstall: D:\ChocolateyInstall

设置 安装包(bin路径之类) 安装的路径 (这个似乎没啥用?) (默认好像在 C:\Program Files):

ChocolateyToolsLocation: D:\choco-pkgs

## 安装软件

### 基本环境

choco install cmake git llvm python notepadplusplus

### stm32 环境环境

choco install make openocd gcc-arm-embedded

## 缩小任务栏图标

执行 https://github.com/valinet/ExplorerPatcher Release中的小工具 (具体使用没细究), 再设置:

注册表: 计算机\HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced

TaskbarSmallIcons 为 1 就可以了

## 恢复Win10右键菜单

使用 cmd, 普通权限:

设置:
reg add HKCU\Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\InprocServer32 /f /ve

移除:
reg.exe delete "HKCU\Software\Classes\CLSID\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\InprocServer32" /va /f

执行后需要重启 资源管理器

## alt+tab 快捷键

禁止切换浏览器标签: 设置 -> 多任务处理 -> 对齐或按Alt+Tab时显示应用中的标签页

不显示选项卡
