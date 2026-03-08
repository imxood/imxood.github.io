# React 笔记

## 文档整理说明

- React 主笔记已统一收敛到当前路径.
- Node.js / npm / yarn 的基础环境说明统一整理在 [Node.js 笔记](../nodejs.md).

## 历史脚手架: Create React App

较老的 React 项目里经常会看到 `Create React App` 方案:

    sudo npm install -g create-react-app
    create-react-app my-app

也可以直接使用:

    npx create-react-app react-app

## 常见依赖

    npm install react-redux uuid --save

## 入门资料

学习自 [`超全面详细一条龙教程！从零搭建React项目全家桶`](https://zhuanlan.zhihu.com/p/104771562)

## 使用 xc-app 快速搭建 Vite + Rust WASM 项目

    yarn create xc-app
    cd PROJECT_NAME
    yarn
    yarn dev
