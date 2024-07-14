# Windows 安装 OpenSSH Server

[下载 OpenSSH Server](https://github.com/PowerShell/Win32-OpenSSH/releases)

下载解压后, 用管理员权限打开Powershell, 执行: .\install-sshd.ps1

如果无法执行, "在此系统上禁止运行脚本", 执行:

	set-ExecutionPolicy ALLSIGNED

安装完成后, 配置文件在:

C:\ProgramData\ssh

修改 sshd_config:

	Port 22
	ListenAddress 0.0.0.0
	PubkeyAuthentication yes
	PasswordAuthentication no
	PermitEmptyPasswords no

启动防火墙, 对端口22的访问限制:

	TCP  22

上面的操作完成后, 打开服务管理器，启动 OpenSSH Authentication Agent 服务和 OpenSSH SSH Server 服务

现在, 就可以使用ssh, 用你的用户名和密码远程访问你的主机了
