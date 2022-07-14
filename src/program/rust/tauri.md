2022.07.14

    cargo install tauri-cli --version "^1.0.0"

    cargo tauri --help

    mkdir tauri_started && cd tauri_started

    cargo tauri init

    根据提示:

    ✔ What is your app name? · tauri_started
    ✔ What should the window title be? · hello world
    ✔ Where are your web assets (HTML/CSS/JS) located, relative to the "<current dir>/src-tauri/tauri.conf.json" file that will be created? · ../web
    ✔ What is the url of your dev server? · ../web

    cargo tauri dev

    遇到一个错误 libpang.. 啥的 undefined 的错误.

    临时解决方法: cp /usr/lib/x86_64-linux-gnu/libpangoft2-1.0.so.0 src-tauri/target/debug/deps/

## 安装 tauri 环境

    依赖:

        sudo apt update && sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev appmenu-gtk3-module libgtk-3-dev squashfs-tools

    npm:

        curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.35.2/install.sh | bash
        . ~/.bashrc
        nvm install node --latest-npm
        nvm use node
        npm config set registry https://registry.npm.taobao.org
        npm config get registry

        npm install -g yarn
        yarn config set registry https://registry.npm.taobao.org


    cargo install tauri-bundler --force

    yarn global add @vue/cli

    vue create tauri-app

    cd tauri-app

    yarn add element-ui tauri

    yarn tauri init

    yarn serve
    yarn build

    yarn tauri dev
    yarn tauri build --debug

    yarn tauri info

## nvs

    export NVS_HOME="$HOME/.nvs"
    git clone https://github.com/jasongin/nvs "$NVS_HOME"
    . "$NVS_HOME/nvs.sh" install

    nvs remote node https://npm.taobao.org/mirrors/node/
    nvs remote

    nvs ls
    nvs add lts

    echo 'nvs use latest' >> ~/.bashrc
