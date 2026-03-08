# CSS 交互技巧

## 说明

- 本页记录前端交互中容易忽略但非常实用的 `CSS` 技巧.
- 当前重点围绕 `pointer-events` 和图层事件穿透场景.

## 禁用鼠标事件

可用于阻止绝对定位元素拦截 `mouseover`, `mouseout`, `click` 等事件:

```css
pointer-events: none;
```

## 典型场景

- 悬浮提示层只负责展示, 不应拦截底层按钮事件.
- 覆盖在图表上的装饰层需要“可见但不可点”.
- 需要让透明遮罩视觉存在, 但把交互继续交给下层节点.

## SVG 遮罩捕获事件

在 `SVG` 交互中, 也可通过显式开启 `pointer-events` 来捕获光标事件:

```html
<g class="brush" fill="none" pointer-events="all">
    <rect class="overlay" pointer-events="all" cursor="crosshair" x="20" y="20" width="460" height="260"></rect>
</g>
```

## 使用建议

- 先区分“元素不响应事件”和“元素透明但仍需接收事件”这两类需求.
- 对绝对定位浮层, `pointer-events` 往往比复杂的事件转发更直接.
- 在图表, 画布和 `SVG` 场景里, 要特别留意事件究竟由容器, 遮罩还是具体图元接收.
- 如果交互异常, 先用浏览器开发者工具检查当前命中元素和计算后的 `pointer-events` 值.
