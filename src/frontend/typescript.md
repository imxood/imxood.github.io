# Typescript 的一些用法记录

```ts
// 指定 对象数组 的变量类型
let data: { country: string; population: number }[];
```

```ts
// 对于 [undefined, undefined] | [number, number] 这样的数据类型的优雅处理
d3.extent(iPopulation) as [number, number];
```

## 默认的严格模式 约束太强, 取消严格模式

在 tsconfig.json 中, 设置:
"strict": false,

## vite 中使用 csv

但又遇到错误: Cannot find module 'data.csv' or its corresponding type declarations.

yarn add -D @rollup/plugin-dsv

修改 vite.config.js:

import dsv from '@rollup/plugin-dsv'

defineConfig({
plugins: [
dsv(), // 添加这个插件
],
})

使用时 需要:

import CSV from '../data/data.csv'
let data = CSV as {date: string, close: number}[]

并在 src 目录下放置一个 csv-shim.d.ts 文件, 内容是:

declare module "\*.csv" {}

## 根据条件 忽略字段

```ts
{
    pressDrag: true,
    wheel: false, // handled by Wheel plugin below
    ...(IS_READONLY_MODE ? {} : { keyToPress: ['Space'] }),
}
```

## 实现 遍历器

```ts
class XXX {
    ...

    [Symbol.iterator]() {
        let step = 1
        let ticks = this._ticks
        const length = this._domain.length
        const that = this

        // 根据 ticks数 判断遍历的次数
        if (length > this._ticks) {
            // 计算步长
            step = Math.ceil(length / this._ticks)
        }

        // 计算需要访问的索引列表
        const indexes = []
        for (let i = 0; i < this._ticks; i++) {
            indexes.push(i * step)
        }
        if (this._ticks * step < length) {
            indexes.push(length - 1)
            ticks++
        }

        let index = 0

        return {
        // 先根据 index, 判断是否需要退出, 再 准备数据, 更新index
        next(this) {
            const value = {
            data: that._domain[indexes[index]],
            index: indexes[index],
            //   text: that._tick_format(that._domain[index]),
            } as TickData<DATA>

            return {
            value,
            done: index++ === indexes.length,
            }
        },
        }
  }
}
```

## 实现 重载

```ts
class XXX {
    ticks();
    ticks(ticks: number);
    ticks(ticks?: number) {
        if (ticks) {
            this._ticks = ticks;
            return this;
        } else {
            return this._ticks;
        }
    }
}
```
