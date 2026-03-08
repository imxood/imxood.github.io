# Svelte Web + Tauri 示例归档

## 说明

- 本目录是一个 `SvelteKit` + `Tauri` 的混合实验项目.
- 目录中同时包含前端工程和 `src-tauri/` 桌面端壳层.
- 当前更适合作为技术组合实验档案, 而不是正式知识入口.

## 当前结构

- `src/`: Svelte 前端代码.
- `src-tauri/`: Tauri 桌面端壳层.
- `static/`: 静态资源.

## 当前治理结论

- 当前目录已列为第一批低优先级迁出候选.
- `Svelte` 相关知识应回到 [frontend/svelte.md](../../../../frontend/svelte.md).
- `Tauri` 相关知识应回到 [program/rust/tauri.md](../../tauri.md).
- 本轮继续保留样例工程上下文, 但不再提升其导航权重.

## 初始化记录

```sh
npm create svelte@latest svelte-web
yarn add -D tailwindcss postcss autoprefixer daisyui
```

## 常用命令

```sh
yarn
yarn dev
yarn build
```

## 适合如何使用

- 回看 `SvelteKit + Tauri` 的混合工程结构.
- 对照前端工程与桌面壳层之间的目录边界.
- 回忆 `tailwindcss` 和 `daisyui` 的接入顺序.

## 后续动作

- 若后续只保留结构参考价值, 可整体迁出主知识树.
- 若其中有稳定结论, 应回收至正式知识页, 不再继续在样例目录 README 中扩写.
