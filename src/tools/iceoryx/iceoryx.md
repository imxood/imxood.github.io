# iceoryx

## 说明

- `iceoryx` 是一个偏底层的零拷贝进程间通信中间件.
- 本页记录源码拉取和最小编译命令, 适合回忆本地试编流程.

## 获取源码

```sh
git clone https://github.com/eclipse-iceoryx/iceoryx.git
cd iceoryx
```

## CMake 构建

```sh
cmake -Bbuild -Hiceoryx_meta -DBUILD_TEST=ON -DINTROSPECTION=OFF -DBINDING_C=ON -DEXAMPLES=ON
cmake --build build -j 20
```

## 参数理解

- `BUILD_TEST=ON`: 构建测试代码, 方便验证环境是否完整.
- `INTROSPECTION=OFF`: 关闭 introspection 组件, 减少当前构建内容.
- `BINDING_C=ON`: 启用 C 语言绑定.
- `EXAMPLES=ON`: 同时构建示例程序, 便于后续上手.

## 使用建议

- 初次接触时先把示例和测试跑通, 再接入自己的工程.
- 若编译失败, 优先检查 `CMake`, 编译器版本和系统依赖是否齐全.
- 若后续继续整理, 可补运行模型, `RouDi`, 发布订阅示例和调试方法.
