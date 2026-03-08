# UnoCSS

## 说明

- `UnoCSS` 是原子化 CSS 引擎, 适合快速搭建样式系统.
- 适合与 `Vite` 配合, 用较低心智成本构建可组合的样式规则.

## 参考资料

- Vite 集成: <https://github.com/unocss/unocss/tree/main/packages/vite>
- 规则搜索: <https://uno.antfu.me>

## Vite 配置示例

```ts
// vite.config.ts
import UnoCSS from 'unocss/vite'
import { presetIcons, presetUno } from 'unocss'

export default {
  plugins: [
    UnoCSS({
      presets: [presetUno(), presetIcons()],
    }),
  ],
}

// main.ts
import 'uno.css'
```

## 常见能力

- `presetUno`: 提供基础原子类规则.
- `presetIcons`: 让图标也能按类名方式使用.
- `shortcuts`: 适合为高频样式组合取别名.
- `rules`: 适合补项目自定义原子规则.

## 使用建议

- 若项目本身已采用原子化样式思路, `UnoCSS` 的接入会很自然.
- 与图标预设组合时, 能减少手写样式和资源文件管理成本.
- 团队协作时, 建议统一约定 `shortcuts` 和自定义规则的命名风格.
- 若后续继续整理, 可补“图标配置”, “主题变量” 和 “与组件库协作”示例.
