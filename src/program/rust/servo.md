## servo

### 在 Deepin 系统上 编译 servo

    git clone https://github.com/servo/servo
    cd servo

    根据 servo 的 git 上的编译方法 (参考), 遇到错误: gstream 版本1.16, 但是我系统中 只有 1.14, 无法编译 media, 已经一番折腾搞定了编译, 并成功运行起来了

    复制 mach 文件为 mach.py, 去掉开头几行, 在 vscode 中边调试运行 mach.py, 从代码中学习!!~~~

    1. 修改文件 python/servo/bootstrap.py, 在388行 "distrib.lower() not in" 的下面, 添加一行:
        'deepin',

    2. 安装依赖
        ./mach bootstrap
    
    3. 编译, 由于至少需要 gstream 版本1.16 问题, 无法编译 media库, 所以使用 dummy media
        ./mach build --dev --media-stack dummy --with-layout-2020 --verbose

        实际运行的命令是:
            rustup run --install nightly-2022-04-11 cargo build --manifest-path ./ports/winit/Cargo.toml --features "media-dummy native-bluetooth egl layout-2020" --timings -v

            或

            cd ./ports/winit && cargo build --features "media-dummy layout-2020" --timings -v

    4. 运行
        ./mach run tests/html/about-mozilla.html

    调试 servo 的配置:

        {
            "type": "lldb",
            "request": "launch",
            "name": "debug servo",
            "program": "${workspaceFolder}/target/debug/servo",
            "args": [
                "tests/html/about-mozilla.html"
            ],
            "cwd": "${workspaceFolder}",
            "sourceLanguages": [
                "rust"
            ],
        },
    
    调试时, 渲染界面出现很慢, 几分钟后才出现? 直接运行立即就有了? 为啥???

    生成的 properties.rs 这个文件很重要, 定义了 Dom 的所有 属性列表 和 属性解析 等,

    NON_CUSTOM_PROPERTY_ID_COUNT 属性数量

    在文件 components/style/properties/build.py 中
        通过 components/style/properties 中 shorthands 和 longhands 目录下 的 .mako.rs 文件
        生成 properties.rs
