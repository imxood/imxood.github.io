# PostgreSQL 使用

笔记来自: https://cloud.tencent.com/developer/article/1632506

## 安装

	ubuntu 安装命令:
		sudo apt install postgresql postgresql-contrib

	PostgreSQL contrib 软件包，它可以提供 PostgreSQL 数据库系统的一些额外特性

	一旦安装完成， PostgreSQL 服务将会自动启动

	验证:
		sudo -u postgres psql -c "SELECT version();"
	输出:
		                            version
		-------------------------------------------------------------------------------------------------------------------------------
		PostgreSQL 12.4 (Ubuntu 12.4-0ubuntu0.20.04.1) on x86_64-pc-linux-gnu, compiled by gcc (Ubuntu 9.3.0-10ubuntu2) 9.3.0, 64-bit
		(1 row)

## PostgreSQL 基础

	PostgreSQL 数据库访问权限是通过角色来处理的。一个角色代表一个数据库用户或者一个数据库用户组。

	PostgreSQL 支持多种身份认证方式。最常用的方法如下：

		Trust - 只要满足pg_hba.conf定义的条件，一个角色就可以不使用密码就能连接服务器
		Password - 通过密码，一个角色可以连接服务器。密码可以被存储为 scram-sha-256, md5, 和 password(明文)。
		Ident - 仅仅支持 TCP/IP 连接。它通常通过一个可选的用户名映射表，获取客户端操作系统用户名。
		Peer - 和 Ident 一样，仅仅支持本地连接。

		PostgreSQL 客户端身份验证通常被定义在pg_hba.conf文件中。默认情况下，对于本地连接，PostgreSQL 被设置成身份认证防范 peer。
		ubuntu: /etc/postgresql/12/main/pg_hba.conf

		为了以postgres用户身份登录 PostgreSQL 服务器，首先切换用户，然后使用psql工具访问 PostgreSQL:
			sudo su - postgres
			psql

		从这里开始，你可以与 PostgreSQL 实例交互。退出 PostgreSQL Shell，输入:
			\q

		你也可以不切换用户，而使用sudo命令访问 PostgreSQL：
			sudo -u postgres psql

		通常，postgres用户仅仅在本地被使用。

## PostgreSQL 基本操作

仅仅超级用户和拥有CREATEROLE权限的角色可以创建新角色。

在下面的例子中，我们创建一个名称为john的角色，一个名称为johndb的数据库，并且授予数据库上的权限：

01. 创建一个新的 PostgreSQL 角色：

	sudo su - postgres -c "createuser john"

02. 创建一个新的 PostgreSQL 数据库:

	sudo su - postgres -c "createdb johndb"

想要授权用户操作数据库，连接到 PostgreSQL shell:

	sudo -u postgres psql

并且运行下面的 query:

	grant all privileges on database johndb to john;

## 启用远程访问 PostgreSQL 服务器

	默认情况下，PostgreSQL 服务器仅仅监听本地网络接口：127.0.0.1

	为了允许远程访问你的 PostgreSQL 服务器，打开配置文件postgresql.conf并且在CONNECTIONS AND AUTHENTICATION一节添加 listen_addresses = '*'
	ubuntu: /etc/postgresql/12/main/postgresql.conf

		sudo nano /etc/postgresql/12/main/postgresql.conf
		#------------------------------------------------------------------------------
		# CONNECTIONS AND AUTHENTICATION
		#------------------------------------------------------------------------------

		# - Connection Settings -

		listen_addresses = '*'     # what IP address(es) to listen on;

	保存文件并且重启 PostgreSQL 服务：

		sudo service postgresql restart

	使用ss工具验证修改：

		ss -nlt | grep 5432

	输出显示 PostgreSQL 服务器正在监听所有的网络接口(0.0.0.0):

		LISTEN  0        244              0.0.0.0:5432           0.0.0.0:*
		LISTEN  0        244                 [::]:5432              [::]:*

	下一步就是配置服务器接受远程连接，编辑pg_hba.conf文件。

	下面是一些例子，显示不同的用户场景：

		# TYPE  DATABASE        USER            ADDRESS                 METHOD

		# The user jane can access all databases from all locations using md5 password
		host    all             jane            0.0.0.0/0                md5

		# The user jane can access only the janedb from all locations using md5 password
		host    janedb          jane            0.0.0.0/0                md5

		# The user jane can access all databases from a trusted location (192.168.1.134) without a password
		host    all             jane            192.168.1.134            trust

	最后一步就是在你的防火墙上打开端口5432端口。

	假设你正在使用UFW来管理你的防火墙，并且你想允许从192.168.1.0/24子网过来的访问，你应该运行下面的命令：

		sudo ufw allow proto tcp from 192.168.1.0/24 to any port 5432

	确保你的防火墙被配置好，并仅仅接受来自受信任 IP 范围的连接。

## 常用操作

	查询所有用户:
		select * from pg_user;

	指定用户登录,并打开数据库, 查询所有表:
		psql --user x2gouser_maxu --password -h localhost --dbname x2go_sessions
	查询所有表:
		\dt
