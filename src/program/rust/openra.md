# OpenRA 构建记录

## 说明

- 本页记录在 Deepin 系统上编译 OpenRA 的依赖准备与构建流程.
- 虽然项目本体偏 C# / .NET, 但当前归档在 Rust 目录中作为游戏方向的历史构建记录保留.

参考: https://github.com/OpenRA/OpenRA/blob/bleed/INSTALL.md

## 配置 C# 环境

参考: https://docs.microsoft.com/zh-cn/dotnet/core/install/linux-scripted-manual#scripted-install

    cd ~/Downloads
    wget https://dot.net/v1/dotnet-install.sh

    chmod +x ./dotnet-install.sh
    ./dotnet-install.sh -c 5.0

    默认安装到了 ~/.dotnet 路径下, 这个路径需要配置到环境变量下

    echo "export PATH=~/.dotnet:$PATH" >> ~/.bashrc
    . ~/.bashrc

## 安装 linux 依赖包

    sudo apt install libfreetype6 libopenal1 liblua5.1-0 libsdl2-2.0-0

## 获取代码 并编译

    git clone https://github.com/OpenRA/OpenRA.git

    cd OpenRA
    make -j10

    ./launch-game.sh
