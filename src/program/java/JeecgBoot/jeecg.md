
## 安装 nodejs

    https://npm.taobao.org/mirrors/node/latest-v15.x/node-v15.3.0-linux-x64.tar.xz

    解压, 创建链接文件:
        sudo ln -s /develop/programs/node-v15.3.0-linux-x64/bin/node /usr/local/bin/
        sudo ln -s /develop/programs/node-v15.3.0-linux-x64/bin/npm /usr/local/bin/

## taobao 镜像

    npm config set registry https://registry.npm.taobao.org
    npm config get registry

# 安装yarn

    sudo npm install -g yarn
    yarn config set registry https://registry.npm.taobao.org

# 前端项目初始化

    # 下载依赖
    yarn install

    # 启动
    yarn run serve

    # 编译项目
    yarn run build

    # Lints and fixes files
    yarn run lint

## 后端项目配置 -- JeecgBoot 单体升级微服务启动

### 微服务通过 Nacos 实现服务注册发现和统一配置中心

    docker  pull nacos/nacos-server
    docker run -d -p 8848:8848 --env MODE=standalone  --name nacos  nacos/nacos-server

    访问Nacos:
        localhost:8848/nacos
        账号: nacos/nacos


    创建 jeecg.yaml, 文件存放路径：jeecg-boot-starter/jeecg-boot-starter-cloud/nacos/jeecg.yaml
    创建 jeecg-dev.yaml, 文件存放路径：jeecg-boot-starter/jeecg-boot-starter-cloud/nacos/jeecg-dev.yaml

    jeecg-dev.yaml 存储经常要修改的配置，一般需要个性化修改的如下：

        1、数据库的连接池修改
        2、redis 配置
        3、rabbitmq 配置
        4、xxljob 配置
        5、路由配置route 加载方式
        6、是否开启xxljob (默认不启用)

### 启动jeecg-cloud-system-start服务

    将 jeecg-boot-module-system/pom.xml 中的 jeecg-boot-module-demo 依赖注释掉 (没有demo依赖可忽略此步骤)

    修改pom文件中spring-boot-maven-plugin 打包插件configuration.skip=true

## log cannot be resolved

    手动安装 lombok

        https://projectlombok.org/download
        
        下载 https://projectlombok.org/downloads/lombok.jar

        用法说明:
            https://projectlombok.org/setup/eclipse
            https://blog.51cto.com/4925054/2127840

    通过 eclipde 的 url 安装 lombok

        https://projectlombok.org/p2

## 后端项目启动

    项目结构说明：
        ├─jeecg-boot-parent（父POM： 项目依赖、modules组织）
        │  ├─jeecg-boot-base-common（共通Common模块： 底层工具类、注解、接口）
        │  ├─jeecg-boot-module-system （系统管理模块： 系统管理、权限等功能） -- 默认作为启动项目
        │  ├─jeecg-boot-module-{?} （自己扩展新模块项目，启动的时候，在system里面引用即可） 

### 初始化数据库，要求mysql5.7+

    -- 创建mysql库
    create database `jeecg-boot` default character set utf8mb4 collate utf8mb4_general_ci;

    -- 手工执行Sql脚步
    source db/jeecgboot-mysql-5.7.sql

### 修改项目配置文件 (数据库配置、redis配置)

    配置文件： jeecg-boot-module-system/src/main/resources/application-dev.yml
    项目名称、端口号配置 (可以不改):
    默认配置——  端口号是8080，项目名称是jeecg-boot

    server:
        port: 8080
        servlet:
        context-path: /jeecg-boot

    数据库配置:

        spring:
            datasource:
                dynamic: 
                datasource: 
                    #主数据源
                    master: 
                        url: jdbc:mysql://127.0.0.1:3306/jeecg-boot?characterEncoding=UTF-8&useUnicode=true&useSSL=false
                        username: root
                        password: root
                        driver-class-name: com.mysql.jdbc.Driver

    Redis配置(配置redis的host和port):

        #redis 配置
        redis:
            database: 0
            host: 127.0.0.1
            lettuce:
            pool:
                max-active: 8   #最大连接数据库连接数,设 0 为没有限制
                max-idle: 8     #最大等待连接中的数量,设 0 为没有限制
                max-wait: -1ms  #最大建立连接等待时间。如果超过此时间将接到异常。设为-1表示无限制。
                min-idle: 0     #最小等待连接中的数量,设 0 为没有限制
            shutdown-timeout: 100ms
            password: ''
            port: 6379

### 启动项目

    /jeecg-boot-module-system/src/main/java/org/jeecg/JeecgSystemApplication.java
    Run As "Java Application"
