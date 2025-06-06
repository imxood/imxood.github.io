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

## vue3 webpack

[Vue3+Electron整合方式](https://github.com/nofacer/vue3-electron)

    npm init -y

    npm install -D webpack webpack-cli
    npm install -D vue@next vue-loader@next @vue@compiler-sfc

    npm install -D style-loader sass-loader node-sass css-loader

    npm install -D url-loader

    过程主要时:
        在 webpack config 中配置怎么解析vue文件,scss文件,scss样式,require路径,
        解析 entry 中指定的文件, output 指定了 输出的文件, 这就是一个打包的过程


webpack.config.js配置:

```javascript
    const path = require('path')
    const { VueLoaderPlugin } = require('vue-loader')

    module.exports = {
        entry: './app.js',
        output: {
            filename: 'bundle.js',
            path: path.resolve(__dirname, 'dist')
        },
        module: {
            rules: [
                {
                    test: /\.vue$/,
                    loader: 'vue-loader'
                },
                // 普通的 `.scss` 文件和 `*.vue` 文件中的
                // `<style lang="scss">` 块都应用它
                {
                    test: /\.scss$/,
                    use: [
                        'style-loader',
                        'css-loader',
                        'sass-loader'
                    ]
                },
                // 图片加载, base64数据, 如: <img :src="require('@/img/vue_logo.png').default">
                {
                    test: /\.(png|jpg|gif)$/i,
                    use: [
                        {
                            loader: 'url-loader',
                            options: {
                                limit: 8192,
                            },
                        },
                    ],
                },
            ]
        },
        plugins: [
            new VueLoaderPlugin()
        ],
        resolve: {
            // 设置import或require时可以使用@作为路径, 如:
            alias: {
                '@': path.resolve('src')
            }
        }
    }
```

## vue3 webpack electron

[Vue3+Electron整合方式](https://github.com/nofacer/vue3-electron)

    npm install -D electron

    npm install -D electron-builder

    指定 目标环境为: electron-renderer
    配置 vue项目的打包输出 以及 electron 的打包输出
    在 package.json 中指定electron启动的入口文件 electron 的打包输出的js文件
    并添加:
        "scripts": {
            "start": "electron .",
            "build": "./node_modules/.bin/webpack",
            "dist": "electron-builder"
        },
        "postinstall": "electron-builder install-app-deps",
        "build": {
            "files": [
                "./dist/**/*",
                "./index.html"
            ],
            "directories": {
                "output": "package"
            }
        },
    "build" 用于 electron builder 需要打包哪些文件 以及 输出到哪里

webpack.config.js 配置:

```javascript
const path = require('path')
const { VueLoaderPlugin } = require('vue-loader')

module.exports = {
    mode: 'development',
    target: "electron-renderer",
    entry: {
        "bundle": ["./app.js"],
        "main": ["./main.js"]
    },
    output: {
        filename: '[name].js',
        path: path.resolve(__dirname, 'dist')
    },
    module: {
        rules: [
            {
                test: /\.vue$/,
                loader: 'vue-loader'
            },
            // 普通的 `.scss` 文件和 `*.vue` 文件中的
            // `<style lang="scss">` 块都应用它
            {
                test: /\.scss$/,
                use: [
                    'style-loader',
                    'css-loader',
                    'sass-loader'
                ]
            },
            // 图片加载, base64数据, 如: <img :src="require('@/img/vue_logo.png').default">
            {
                test: /\.(png|jpg|gif)$/i,
                use: [
                    {
                        loader: 'url-loader',
                        options: {
                            limit: 8192,
                        },
                    },
                ],
            },
        ]
    },
    plugins: [
        new VueLoaderPlugin()
    ],
    resolve: {
        // 设置import或require时可以使用@作为路径, 如:
        alias: {
            '@': path.resolve('src')
        }
    }
}
```

## vite-electron 创建 vite + electron 项目

    yarn create vite-electron electron-app

## 使用 electron-vue-next (vite vue3 electron)

    # 我使用默认配置
    npm init electron-vue-next


## electron 默认不能在渲染进程中使用node环境

[上下文隔离](https://www.electronjs.org/docs/tutorial/context-isolation)

## import 和 require 区别

在 typescript 使用 import  
在 js 中使用 require

## dplayer

[用于测试的视频](http://samples.mplayerhq.hu/)

## ffmpeg 使用

    # 生成 hls格式 视频
    ffmpeg -i ./demo.mp4 -hls_time 10 -hls_list_size 0 -f hls test/demo.m3u8

    ~$ tree test/
    test/
    ├── demo0.ts
    ├── demo1.ts
    ├── demo2.ts
    ├── demo3.ts
    ├── demo4.ts
    ├── demo5.ts
    ├── demo6.ts
    └── demo.m3u8

## ffmpeg

    # 安装 模拟的 video loop设备
    https://github.com/umlaeute/v4l2loopback --> make && sudo make install-all, 加载ko并运行

    # 强制 format ?
    # v4l2loopback-ctl set-caps /dev/video0 "UYVY:640x480"

    # 创建新loop设备
    sudo v4l2loopback-ctl add -n "loopy doopy" /dev/video7

    # 删除设备
    sudo v4l2loopback-ctl delete /dev/video7

    # 安装 v412-ctl 工具 (video4linux)
    sudo apt install v4l-utils

    # 当 设备 没有输入时 停留 3000ms 后, 输出 空帧 (但是继续输入, ffplay似乎不动了?)
    v4l2-ctl -d /dev/video0 -c timeout=3000

    # 超时图片, 取代空帧 ?
    v4l2loopback-ctl set-timeout-image -t 3000 /dev/video0 service-unavailable.png

    # 查看设备列表
    v4l2-ctl --list-devices

    # 查看当前摄像头支持的视频压缩格式
    v4l2-ctl -d /dev/video0 --list-formats

    # 改变分辨率, 同时, 会根据输出文件后缀改变 video format
    ffmpeg -i demo.mp4 -vf scale=1280x720 output.mkv

    # 输出视频数据到 /dev/video0 设备上
    # read video in the real framerate
    # input file
    # webcams expect raw video
    # the same format used by my real one
    ffmpeg -re -i pickle-rick.mkv -vcodec rawvideo -pix_fmt yuv420p -framerate 25 -f v4l2 /dev/video0

    # 播放 /dev/video0 上的视频数据 (没有声音)
    ffplay /dev/video0

    # 记录 /dev/video0 设备上的数据 10s, 输出到 output.mkv 文件
    ffmpeg -f v4l2 -framerate 25 -video_size 640x480 -i /dev/video0 -t 00:00:10 output.mkv

## 库

    pkg-config --cflags --libs gstreamer-1.0
