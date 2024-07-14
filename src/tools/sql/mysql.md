## 安装开源版本

    sudo apt install mariadb-server

    sudo mysql_secure_installation

        ps: 不修改root用户的密码, 使用系统的认证

    sudo mariadb

    创建新用户admin:
        create user 'admin'@'localhost' identified by 'admin';

    查看当前所有用户:
        select user from mysql.user;
    
    给新用户admin添加root权限:
        grant all on *.* to 'admin'@'localhost';
        flush privileges;

    删除用户:
        delete from user where user = "admin";
        flush privileges;

    数据库的创建与删除:
        create database test_db;
        drop database test_db;

    查看所有的数据库:
        show databases;
    
    选中数据库:
        use test_db;
    
    显示数据库中所有表:    
        show tables;

    带有默认编码与字符集:
        create database if not exists `db-mcms-open` default charset utf8 collate utf8_general_ci;

    查询默认端口号:
        show global variables like 'port';

