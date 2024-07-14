## windows7 中支持的 target

rustup install 1.77.2

指定项目的默认目标 `rustup override set 1.77.2-x86_64-pc-windows-msvc`

或者, 创建文件 `rust-toolchain.toml`, 写入:

```toml
[toolchain]
channel = "1.77.2-x86_64-pc-windows-msvc"
```
