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

electron neno
rust
