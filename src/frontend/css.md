## 禁用 鼠标事件

可以用于 阻止 绝对定位的 div 的 mouseover mouseout 等光标事件

pointer-events: none

svg 添加遮罩, 捕获所有光标事件

``` html
<g class="brush" fill="none" pointer-events="all">
    <rect class="overlay" pointer-events="all" cursor="crosshair" x="20" y="20" width="460" height="260"></rect>
    ...
</g>
```
