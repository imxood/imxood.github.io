# Dora 向导

## 说明

- 本页记录 `dora-rs` 的安装方式与最小数据流项目创建流程.
- 适合作为机器人数据流编排框架的快速上手入口.

## dora 向导

https://dora-rs.ai/docs/guides/

### 安装 dora

这里使用 git release 的方式

https://github.com/dora-rs/dora/releases

下载对应平台, 解压 并设置环境变量 即可

## RUST 语言

创建基本项目

    dora new test_dora --kind dataflow

会自动创建 3 个节点 talker_1 talker_2 listener_1

    cd test_dora

编译 3 个节点:

    dora build .\dataflow.yml

运行这 3 个节点:

    dora run .\dataflow.yml

## python 语言

https://dora-rs.ai/docs/guides/getting-started/conversation_py
