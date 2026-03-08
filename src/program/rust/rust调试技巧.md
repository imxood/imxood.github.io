# Rust 调试技巧

## 说明

- 本页整理 `Rust` 项目在本地调试时的高频做法.
- 当前重点是 `VSCode + CodeLLDB`, 回溯信息和多 crate 工程排错.

## VSCode + CodeLLDB

常见做法是使用 `CodeLLDB` 插件来调试 Rust 程序.

`launch.json` 示例:

```json
{
    "type": "lldb",
    "request": "launch",
    "name": "Cargo launch",
    "cargo": {
        "args": [
            "-Z",
            "unstable-options",
            "-C",
            "${workspaceFolder}/ProjectName",
            "build"
        ]
    },
    "args": ["ProjectArg1", "ProjectArg2"]
}
```

## 配置理解

- `type = lldb`: 表示使用 `CodeLLDB` 调试器.
- `cargo.args`: 控制编译参数和项目路径.
- `args`: 用于传递程序运行参数.
- 多 crate 项目里, 要特别确认目标 crate 和二进制名称是否正确.

## 常见调试入口

### 运行时崩溃

可优先打开回溯信息:

```sh
RUST_BACKTRACE=1 cargo run
```

若需要更完整信息:

```sh
RUST_BACKTRACE=full cargo run
```

### 单元测试排错

```sh
cargo test -- --nocapture
```

适用场景:

- 需要看到测试里的输出日志.
- 需要先缩小失败范围, 再进入断点调试.

## 使用建议

- 先用日志和回溯缩小范围, 再进入图形调试器, 效率通常更高.
- 如果项目依赖 `nightly` 或特殊构建参数, 建议把调试配置固定在 `.vscode/launch.json` 中.
- 遇到跨线程或异步问题时, 优先构造最小复现样例.
- 若后续继续整理, 可补 `gdb/lldb` 命令, `tokio` 调试和 `wasm` 侧排错笔记.
