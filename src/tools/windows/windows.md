# Windows 笔记

## 说明

- 本页整理 Windows 系统清理, 常见排障, 环境变量, 端口与系统设置的高频记录.
- 当前内容更偏系统维护与问题排查, 适合作为 Windows 运维速查页.

## 系统清理

### 常见缓存目录

- `AppData\Local\Yarn\Cache`: Yarn 缓存目录, 可用 `yarn cache clean` 清理.
- `AppData\Local\Temp`: 临时文件目录, 可按需清理.
- `AppData\Roaming\Tencent`: 某些软件缓存目录, 清理前先确认无业务依赖.

## 程序依赖与运行错误

### `0xc000007b`

- 这类错误常和运行库位数不匹配, 缺失 DLL 或运行时环境异常有关.
- 典型排查方向:
  - 目标程序是 `32` 位还是 `64` 位.
  - 依赖 DLL 位数是否一致.
  - VC 运行库是否完整.

### 查看程序依赖

- `Dependency Walker`: <https://dependencywalker.com/>
- `Procmon`: <https://docs.microsoft.com/en-us/sysinternals/downloads/procmon>

## 环境变量

### `cmd`

```bat
set A=a/b/c
set A=a/b/c;%A%
```

### `PowerShell`

```powershell
ls env:
$env:PATH
$env:A="a/b/c"
$env:A="a/b/c;$env:A"
```

- 临时环境变量只对当前会话有效.
- 长期环境变量建议通过系统设置统一配置.

## 端口与进程排查

### 查看进程占用端口

```bat
netstat -ano | findstr 6220
```

- `6220` 为端口号.
- 查到 PID 后, 再结合任务管理器或 `taskkill` 继续处理.

### 进程异常退出排查

- 打开 `eventvwr.msc`.
- 路径: `Windows 日志 -> 应用程序`.
- 可按时间和事件来源筛选 `Application Error`, `Windows Error Reporting`.

## 串口与设备

### 清除 COM 端口占用记录

- 打开 `regedit`.
- 路径: `HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\COM Name Arbiter\Devices`.
- 删除不再需要的历史 COM 端口映射前, 建议先备份注册表.

### 删除 VirtualBox 虚拟网卡

- 先启用目标虚拟网卡, 让其出现在列表中.
- 再在 VirtualBox 的虚拟网卡管理界面中删除.

## 安全与系统设置

### 关闭防火墙后禁用通知

- 参考: <https://www.disktool.cn/content-center/win11-turn-off-firewall-2111.html>

### 关闭实时保护

- 参考: <https://blog.csdn.net/COCO56/article/details/128613164>
- 若提示“此设置由管理员进行管理”, 还要检查相关组策略与注册表项.

## 界面与桌面问题

### 桌面图标间距异常

- 注册表路径: `HKEY_CURRENT_USER\Control Panel\Desktop\WindowMetrics`.
- 可调整 `IconSpacing` 与 `IconVerticalSpacing`.
- 修改前建议先导出备份.

## 网络与浏览器

### Edge 禁止 HTTP 自动重定向到 HTTPS

- 打开 `edge://net-internals/#hsts`.
- 在 `Delete domain security policies` 中删除对应域名策略.

## 版本与授权问题

### Windows 10 LTSC 授权异常记录

- 可通过调整 `SoftwareProtectionPlatform` 下的 `SkipRearm` 等配置后结合 `slmgr -REARM` 处理.
- 这类操作涉及系统授权机制, 修改前建议备份并确认版本适用性.

## 使用建议

- 本页优先记录高频系统维护与排障线索, 不宜继续无节制堆积零散命令.
- 若某个主题已经形成稳定专题, 应拆分到独立页面.
- 与包管理和服务部署相关的内容, 可继续看 `Chocolatey`, `NSSM`, `OpenSSH Server` 等专题页.
