# Windows 安装 OpenSSH Server

## 说明

- 本页记录在 Windows 上手动安装 `OpenSSH Server` 的基本流程.
- 适合较早版本系统, 手动部署 `Win32-OpenSSH`, 或需要显式控制服务配置的场景.

## 下载与安装

- 下载地址: <https://github.com/PowerShell/Win32-OpenSSH/releases>

下载并解压后, 使用管理员权限打开 PowerShell, 执行:

```powershell
.\install-sshd.ps1
```

如果提示“在此系统上禁止运行脚本”, 可临时调整执行策略:

```powershell
Set-ExecutionPolicy AllSigned
```

## 配置文件位置

```text
C:\ProgramData\ssh
```

常见 `sshd_config` 配置项:

```conf
Port 22
ListenAddress 0.0.0.0
PubkeyAuthentication yes
PasswordAuthentication no
PermitEmptyPasswords no
```

## 服务与防火墙

- 放行 `TCP 22`.
- 在服务管理器中确认并启动:
  - `OpenSSH Authentication Agent`
  - `OpenSSH SSH Server`

## 常见检查项

- 确认 `sshd` 服务是否已安装并处于运行状态.
- 确认防火墙已放行目标端口.
- 确认登录用户公钥放置位置与权限正确.
- 若是局域网外访问, 还要检查路由器映射与公网入口.

## 使用建议

- 新版本 Windows 已经逐步内置 OpenSSH 能力, 优先检查系统功能里是否可直接启用.
- 若使用公钥登录, 应优先禁用纯密码登录以降低风险.
- 安装后建议立即用局域网客户端做一次实际连接验证.

## 相关文档

- [Windows 总览](./README.md)
- [NSSM](./nssm.md)
