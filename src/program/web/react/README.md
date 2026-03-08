# React 示例目录

## 说明

- 本目录保存历史 `React` 示例工程与实验代码, 主要承担样例归档职责.
- 正式知识文档请优先查看 [前端 React 笔记](../../../frontend/react/react.md).
- 若目标是新建项目, 不建议直接从这些旧样例出发.

## 当前示例

### 仍适合作为结构对照的样例

- [Umi React 示例项目](./ant-app/README.md)
- [React 单文件示例](./demo/README.md)
- [Vite React 示例项目](./react-demo/README.md)

### CRA 时代样例

- [React my-app 示例说明](./my-app/README.md)
- [CRA React 示例归档](./react-app/README.md)

## 当前治理结论

- `react-app/` 已列为第一批低优先级迁出候选.
- `my-app/` 暂保留为同代脚手架对照样本.
- 若后续继续收缩主知识树, 可把 `CRA` 相关目录收敛成单一说明入口.

## 阅读路径

1. 查 React 概念或 API 时, 先回主文档.
2. 需要看旧版 `CRA` 结构时, 进入 `my-app` 或 `react-app`.
3. 需要看 `Umi 2` 结构时, 进入 `ant-app`.
4. 需要看早期 `Vite + React` 工程时, 进入 `react-demo`.

## 使用建议

- 需要查知识点时, 优先返回主文档.
- 需要验证脚手架结构或运行方式时, 再进入对应示例目录.
- 需要迁移旧项目时, 优先抽取配置思想, 不要直接整仓复制历史模板.
