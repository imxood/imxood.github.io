# Rust 工具

## 文档整理说明

- 本文档用于记录 Rust 生态中的常用工具, 包括 Cargo 扩展, 打包工具, 二进制分析工具, 以及日常命令行环境配置.
- 旧版 Cargo 扩展工具记录已收敛到本文, 后续统一以当前页面为准.

## 常用 Cargo 扩展

```sh
cargo install cargo-generate cargo-binutils cargo-tree cargo-expand cargo-edit cargo-wix cargo-bundle
```

### cargo-edit

用于升级, 增删依赖:

```sh
cargo install cargo-edit
cargo upgrade
```

### cargo-machete

检查未使用的 crate:

```sh
cargo install cargo-machete
cargo machete
```

### cargo-udeps

检查未使用的依赖:

```sh
cargo install cargo-udeps
cargo udeps
```

需要 `nightly` 环境, 结果并不是 100% 准确.

### cargo-bloat

显示不同 crate 占用输出文件大小的情况:

```sh
cargo install cargo-bloat
cargo bloat --release --crates
```

### cargo-autoinherit

自动将 workspace 中项目的依赖转到 workspace 共享配置:

```sh
cargo install cargo-autoinherit
cargo autoinherit
```

也可以根据日志检查是否依赖了多个不同版本的 crate.

## 打包

### cargo-dist

打包工具:

```sh
cargo install cargo-dist
```

在需要打包的 crate 目录下执行:

```sh
dist init
dist build
```

参考项目: https://github.com/axodotdev/axolotlsay

### oranda

用于给 `cargo-dist` 打包后的文件创建静态站点:

```sh
cargo install oranda --locked --profile=dist
oranda build
```

## 二进制与证书工具

### upx

用于压缩可执行文件:

```sh
upx --best --lzma target/release/your_executable
```

下载地址: https://github.com/upx/upx/releases

### feluda

检查是否存在限制性证书:

```sh
cargo install feluda
feluda
```

## Nushell

```sh
# 设置默认编辑器
$env.config.buffer_editor = "code"

# 打开配置
config nu

# 在配置中添加别名
alias cc = claude --dangerously-skip-permission

# 在配置中添加自动完成
source "E:/git/rust/nu_scripts/custom-completions/git/git-completions.nu"
source "E:/git/rust/nu_scripts/custom-completions/cargo/cargo-completions.nu"
source "E:/git/rust/nu_scripts/custom-completions/npm/npm-completions.nu"
```
