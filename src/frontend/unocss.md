## unocss

参考 vite 中配置 unocss: https://github.com/unocss/unocss/tree/main/packages/vite

快速搜索关键缩写: https://uno.antfu.me

```js
// vite.config.ts
import UnoCSS from 'unocss/vite'
import {
    presetIcons,
    presetUno,
} from 'unocss'

export default {
  plugins: [
    UnoCSS({
            presets: [
                presetUno(),
                presetIcons(),
            ]
        }),
  ],
}

// main.ts
import 'uno.css'
```