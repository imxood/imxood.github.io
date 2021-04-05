# mdbook 构建工具

[官方文档](https://rust-lang.github.io/mdBook/index.html)

## 安装

### 从预编译文件中安装

[预编译文件](https://github.com/rust-lang/mdBook/releases)

### 安装发布到 Crates.io 的版本

cargo install mdbook

## mdbook 用法

    mdbook help
    mdbook init
    mdbook build
    mdbook watch --open
    mdbook serve --open -n 0.0.0.0

## 可选项

output.html.print 是否开起打印功能, 打印功能是 它打开生成的一个包含所有内容的print.html页面, 然后打印这个文件...如果保存为pdf的话没有目录

no-section-label 这个可以去掉侧边栏的数字前缀

output.html.fold 可以控制sidebar是否可以折叠及默认行为
