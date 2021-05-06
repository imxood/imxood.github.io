# electron+neon

## neon

[getting-started](https://neon-bindings.com/docs/getting-started/)

### Install the Neon CLI

    Unix 依赖:
        Python 2.7 (Python 3 is not supported)
        make
        A proper C/C++ compiler toolchain, like GCC
    安装:
        npm install --global neon-cli

    验证:
        neon version

### 实践neon

    # 创建项目
    neon new thread-count

    # 编译
    cd thread-count
    neon build --release

    # 验证
    node
    > require('.')
    hello node
    {}
    >

    # 清理
    neon clean

## 打包

~~方法1:~~

~~npm install electron-packager -g~~  
~~electron-packager .~~

方法2:

[electron-builder](https://www.electron.build/)

## 使用 vue

    npm install -g @vue/cli


## rust+neon+electron

    新建:
        neon new rust-api
        cd rust-api
        npm link

    添加一个 electron basic
    git clone https://github.com/electron/electron-quick-start
    cd electron-quick-start

    npm install rust-api
    npm install electron-build-env neon-cli --save-dev

    添加:
    "scripts": {
        "run": "npm run build && npm run start",
        "start": "electron .",
        "build": "electron-build-env neon build rust-api",
        "release": "electron-build-env neon build rust-api --release"
    }

        ps: 注意这里的"build"参数: rust-api

    npm run

## electron vue

[electron-vue](https://github.com/SimulatedGREG/electron-vue)

    添加vue-cli:
        npm install -g vue-cli

    使用vue-cli创建新项目:
        vue init simulatedgreg/electron-vue electron-app

    Error: Unable to install `vue-devtools`
    解决:
        yarn add vue-devtools --dev
        编辑 src/main/index.dev.js:

        import { app, BrowserWindow } from "electron";

        // Install `vue-devtools`
        app.on("ready", async () => {
            // let installExtension = require('electron-devtools-installer')
            // installExtension.default(installExtension.VUEJS_DEVTOOLS)
            //   .then(() => {})
            //   .catch(err => {
            //     console.log('Unable to install `vue-devtools`: \n', err)
            //   })
            await new BrowserWindow.addDevToolsExtension(
                "node_modules/vue-devtools/vender"
            );
        });

    运行:
        npm run dev

## element-ui

[element-ui](https://element.eleme.cn/#/zh-CN/component/quickstart)

## mockjs + axios 很香

[vue项目中使用mockjs+axios模拟后台数据返回](https://www.cnblogs.com/steamed-twisted-roll/p/10823871.html)

## 使用 electron-vue-next (vite vue3)

    # 我使用的是默认配置
    npm init electron-vue-next
