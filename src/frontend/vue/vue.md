# vue 笔记

## 使用 Vite 工具

[vite 文档](https://cn.vitejs.dev/guide/)

### 创建项目

    yarn create @vitejs/app

## 历史脚手架: vue-cli

在较老的项目里, 也可能会看到基于 `vue-cli` 的初始化方式:

    yarn global add vue-cli
    vue init webpack vue-app

如果是新项目, 优先使用 `Vite` 方案.

## vuejs

1. 如果一个数据依赖于其他数据, 那么把这个数据设计为 computed.
2. 如果你需要在某个数据变化时做一些事情, 使用 watch 来观察这个数据变化.
