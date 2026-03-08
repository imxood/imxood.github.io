# Node.js 笔记

## 国内源

npm config set registry http://registry.npmmirror.com

## Linux 环境安装

    sudo apt-get remove nodejs --purge

    # https://github.com/nodesource/distributions
    curl -sL https://deb.nodesource.com/setup_14.x | sudo -E bash -
    sudo apt-get install -y nodejs

    node -v
    npm -v

    sudo npm install -g yarn
    yarn config set registry 'https://registry.npmmirror.com'

## npm

    配置文件 $HOME/.npmrc
    查看所有配置:
        npm config ls -l

### 查看包版本

    npm ls electron     本地包版本号
    npm ls electron -g  全局包版本号

## yarn

    yarn config list

### 常见用法

    yarn global add typescript

    # 前端构建工具
    sudo yarn global add grunt-cli

    ISSUE: You need to have Ruby and Sass installed and in your PATH for this task to work.
    SOLVED: yarn add --save node-sass grunt-sass

## 工具

    yarn global add npm-check
    npm-check -u

## electron

npm config set ELECTRON_MIRROR http://npm.taobao.org/mirrors/electron/
npm install --save-dev electron
