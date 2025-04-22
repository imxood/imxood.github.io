##

cargo install cargo-generate cargo-binutils cargo-tree cargo-expand cargo-edit cargo-wix cargo-bundle

## 打包

打包工具

cargo install cargo-dist

在需要打包的 crate 目录下, 执行:

dist init

dist build (根据提示指定 tag?)

参考项目: https://github.com/axodotdev/axolotlsay

### cargo-dist 的扩展工具 oranda

用于给 cargo-dist 打包后的文件, 创建静态站点

cargo install oranda --locked --profile=dist

构建你的站点:

oranda build

## cargo-udeps

检查未使用的依赖

cargo install cargo-udeps

需要 nightly 环境, 检查并不是 100% 准确

用法:

    cargo udeps

## cargo-bloat

检查 crate 占用可执行文件空间的情况

cargo install cargo-bloat

用法:

    cargo bloat --release --crates

## upx 工具

用于压缩 exe 可执行文件, 40M 的程序, 竟然被压缩到了 11M, 强大!

下载地址: https://github.com/upx/upx/releases

用法:

    upx --best --lzma target/release/your_executable

## feluda 检查是否存在 限制性证书

cargo install feluda

在 rust 项目 目录中, 执行 `feluda`
