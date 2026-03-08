# ECharts

## 说明

- `ECharts` 适合快速构建常见业务图表, 上手成本低于底层绘图库.
- 官方文档: <https://echarts.apache.org/handbook/zh/get-started/>

## 适合场景

- 柱状图, 折线图, 饼图等标准业务图表.
- 需要较快完成配置式图表开发的后台系统.
- 对自定义程度要求不极端, 但需要生态和现成组件支持的场景.

## 最小示例

```js
const option = {
  xAxis: {
    type: 'category',
    data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri'],
  },
  yAxis: {
    type: 'value',
  },
  series: [
    {
      type: 'line',
      data: [120, 200, 150, 80, 70],
    },
  ],
}
```

## 使用建议

- 先把数据结构整理成 `series` / `xAxis` / `yAxis` 友好的形式.
- 做多图表页面时, 建议先统一主题, 颜色和 tooltip 交互风格.
- 如果定制程度越来越高, 再考虑 `D3` 或 `Canvas` / `WebGL` 方案.
- 若后续继续整理, 可补 `dataset`, 地图, 大屏适配和性能优化记录.
