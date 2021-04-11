# mdbook 构建工具

[官方文档](https://rust-lang.github.io/mdBook/index.html)

## 安装

- 方法1. [预编译文件](https://github.com/rust-lang/mdBook/releases)
- 方法2. 编译:

   ``` cargo install mdbook ```

## mdbook 用法

    mdbook --help
    mdbook <SUBCOMMAND> --help

    mdbook init 初始化项目
    mdbook init --theme, 初始化项目, 并复制默认的theme到你的项目目录
    mdbook build, 编译生成静态文件
    mdbook watch --open, 编译成静态文件并打开浏览器, 如果文件改变就重新build
    mdbook serve --open, 编译成静态文件并创建server, 打开浏览器, 如果文件改变就重新build

## book.toml 配置

output.html.print 是否开起打印功能, 打印功能是 它打开生成的一个包含所有内容的print.html页面, 然后打印这个文件...如果保存为pdf的话没有目录

no-section-label 这个可以去掉侧边栏的数字前缀

output.html.fold 可以控制sidebar是否可以折叠及默认行为

## preprocessor

[数学公式 mdbook-katex](https://github.com/lzanini/mdbook-katex)

[流程图 mdbook-mermaid](https://github.com/badboy/mdbook-mermaid)

cargo install mdbook-katex

cargo install mdbook-mermaid
mdbook-mermaid install path/to/your/book

## 源码分析

1. 使用 `` env_logger `` 输出log, 通过 RUST_LOG 环境变量 控制 log 的输出, 如: `` export RUST_LOG=mdbook ``, 启用可执行程序 mdbook 的 all logging

2. 使用 `` clap `` 解析命令行参数

3. 加载配置

   使用 `` toml `` 加载 book.toml文件, 并序列化成 BookConfig对象. BookConfig对象需要实现 serde 的 Deserialize

   使用 `` std::env `` 读取环境变量, 根据环境变量中 "MDBOOK_" 前缀的变量更新配置

   如: MDBOOK_BOOK__TITLE="Hello" 即设置 book.title 的值为"Hello"

4. 解析 SUMMARY.md 文件, 得到 book 对象, 包含了每一个章节的content.

   使用 `` pulldown-cmark `` 解析 markdown文件

   核心思想: pull parser, 像游标一样移动, 提供 `` self.stream.next() `` 这样的方法向前移动游标, 返回值为解析到的事件, 对特定事件处理并loop.

   比如: `` Event::Start(Tag::Heading(1)) ``, 如果检查到这个事件就执行:
   ```
       let tags = collect_events!(self.stream, end Tag::Heading(1));
   ```

   这样就能拿到解析到的数据. 通过对不同的事件处理 并 loop, 完成解析工作.

   `` parse_numbered ``, 解析中间部分的列表. 即: Numbered Chapter

   `` parse_nested_numbered ``, 用于解析一个子列表

   `` parse_nested_item ``, 用于解析子列表中的某一项

   `` next_event() `` 中, 通过 `` parser.back.take() `` 若为None则向前移动游标, 否则就返回 `` back.take() ``的结果.


   ```
    例子1:

   - [首页](./README.md)

   具体的event过程:
      Parsing prefix items
      Next event: Some(Start(List(None)))
      Next event: Some(Start(Item))
      Next event: Some(Start(Link(Inline, Borrowed("./README.md"), Borrowed(""))))
      Next event: Some(Text(Borrowed("首页")))
      Next event: Some(End(Link(Inline, Borrowed("./README.md"), Borrowed(""))))
      Next event: Some(End(Item))
      Next event: Some(End(List(None)))
      Next event: Some(Start(Heading(1)))
      Back: Start(Heading(1))

   例子2:

   - [硬件]()
      - [晶体管](./hardware/晶体管.md)
      - [MOS管](./hardware/MOS管.md)

   具体的event过程:
      Parsing numbered chapters at level 0
      Next event: Some(Start(List(None)))
      Next event: Some(Start(Item))
      Next event: Some(Start(Paragraph))
      Next event: Some(Start(Link(Inline, Borrowed(""), Borrowed(""))))
      Next event: Some(Text(Borrowed("硬件")))
      Next event: Some(End(Link(Inline, Borrowed(""), Borrowed(""))))
      Found chapter: 1. 硬件 ([draft])
      Next event: Some(End(Paragraph))
      Next event: Some(Start(List(None)))
      Parsing numbered chapters at level 1.
      Next event: Some(Start(Item))
      Next event: Some(Start(Link(Inline, Borrowed("./hardware/晶体管.md"), Borrowed(""))))
      Next event: Some(Text(Borrowed("晶体管")))
      Next event: Some(End(Link(Inline, Borrowed("./hardware/晶体管.md"), Borrowed(""))))
      Found chapter: 1.1. 晶体管 (./hardware/晶体管.md)
      Next event: Some(End(Item))
      Next event: Some(Start(Item))
      Next event: Some(Start(Link(Inline, Borrowed("./hardware/MOS管.md"), Borrowed(""))))
      Next event: Some(Text(Borrowed("MOS管")))
      Next event: Some(End(Link(Inline, Borrowed("./hardware/MOS管.md"), Borrowed(""))))
      Found chapter: 1.2. MOS管 (./hardware/MOS管.md)
      Next event: Some(End(Item))
      Next event: Some(End(List(None)))
   ```

5. preprocessor

   preprocessor 以子进程的方式被 mdbook主程序 调用, mdbook主程序 把 book 对象 以stdio 的方式传递给 preprocessor进程, preprocessor进程对每一个章节的内容进行预处理

6. renderer工作过程

   默认使用的是 "html"渲染器, 用于把 preprocessor 的结果解析成html页面.

7. 使用 `` notify `` 捕获特定路径下文件的changed事件

8. 使用 `` warp `` 用于创建http server 和 websocket server, 提供 web服务, 并支持 reload
