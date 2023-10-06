## 使用 svelte 创建组件库

npx degit sveltejs/component-template my-new-component
cd my-new-component
yarn

## 使用 svelte-kit

<!-- npm create svelte@latest my-app -->
yarn create svelte my-app

cd my-app
yarn
yarn dev -- --open

### 在 svelte-kit 项目中使用 pixijs

参考: https://svelte-pixi.com/docs/getting-started/creating-a-project

yarn add pixi.js svelte-pixi

## 事件

Dom元素加载完成后, 执行:
onMount

当响应式变量更新后触发, 但 onMount事件前后也会触发:
beforeUpdate
afterUpdate


只要 响应式变量 resized 有变化, 就执行代码块:
``` svelte
$: {
    console.log('resized', resized)
    console.log("112233333");
}
```
