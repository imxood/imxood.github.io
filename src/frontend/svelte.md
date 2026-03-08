# Svelte

## 说明

- 本页整理 `Svelte` 和 `SvelteKit` 的历史初始化记录与基础响应式要点.
- 适合作为快速回忆“如何起项目”和“响应式语法怎么写”的入口页.

## 初始化项目

### 组件库模板

```sh
npx degit sveltejs/component-template my-new-component
cd my-new-component
yarn
```

### SvelteKit 项目

```sh
yarn create svelte my-app
cd my-app
yarn
yarn dev -- --open
```

说明:

- 以上命令属于当前知识库中的历史记录.
- 具体脚手架命令可能随版本变化, 真正执行前应再核对官方文档.

## 集成 PixiJS

参考: <https://svelte-pixi.com/docs/getting-started/creating-a-project>

```sh
yarn add pixi.js svelte-pixi
```

适用场景:

- 在 `Svelte` 中快速接入 2D 图形或交互画布能力.
- 用响应式状态驱动渲染参数变更.

## 生命周期与响应式

- `onMount`: DOM 已挂载后执行.
- `beforeUpdate` / `afterUpdate`: 组件更新前后触发.
- `$:` 语句块可自动追踪依赖并重新计算.

```svelte
$: {
    console.log('resized', resized)
}
```

## 使用建议

- 页面较复杂时, 把状态, 视图和副作用拆到独立模块.
- 第三方图形库接入时, 优先把实例化逻辑放到 `onMount`.
- 若后续继续整理, 可补 `store`, `layout`, `路由` 和 `SSR` 相关专题.
