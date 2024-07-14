## 安装 wine

    sudo apt install deepin-wine

## 安装 nodejs

    https://npm.taobao.org/mirrors/node/latest-v15.x/node-v15.3.0-linux-x64.tar.xz

    解压, 创建链接文件:
        sudo ln -s /develop/programs/node-v15.3.0-linux-x64/bin/node /usr/local/bin/
        sudo ln -s /develop/programs/node-v15.3.0-linux-x64/bin/npm /usr/local/bin/

    ## taobao 镜像

        npm config set registry https://registry.npm.taobao.org
        npm config get registry

## 安装 mysql

    sudo apt install libmecab2
    sudo apt install -f

    sudo dpkg -i mysql-community-client-plugins_8.0.22-1debian10_amd64.deb
    sudo dpkg -i mysql-community-client-core_8.0.22-1debian10_amd64.deb
    sudo dpkg -i mysql-community-client_8.0.22-1debian10_amd64.deb
    sudo dpkg -i mysql-client_8.0.22-1debian10_amd64.deb

    sudo dpkg -i mysql-common_8.0.22-1debian10_amd64.deb
    sudo dpkg -i mysql-community-server-core_8.0.22-1debian10_amd64.deb
    sudo dpkg -i mysql-community-server_8.0.22-1debian10_amd64.deb

    弹出框: 设置密码, 使用默认的加密方式

    登录 mysql:
        sudo mysql -uroot -p

## 安装 新版 clang

    vim /etc/apt/sources.list, 添加

        deb http://apt.llvm.org/buster/ llvm-toolchain-buster main 
        deb-src http://apt.llvm.org/buster/ llvm-toolchain-buster main 
        deb http://apt.llvm.org/buster/ llvm-toolchain-buster-10 main 
        deb-src http://apt.llvm.org/buster/ llvm-toolchain-buster-10 main 
        deb http://apt.llvm.org/buster/ llvm-toolchain-buster-11 main 
        deb-src http://apt.llvm.org/buster/ llvm-toolchain-buster-11 main

    wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key|sudo apt-key add -

    sudo apt-get update

    sudo apt-get install clang-11