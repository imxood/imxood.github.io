# Vite React 示例项目

## 说明

- 本目录是一个基于 `Vite + React + TypeScript` 的历史示例工程.
- 目录中还保留了一个 `wasm-test/` 子目录, 用于 Rust / WASM 联调实验.
- 这页更适合作为“结构说明”, 不承担 React 或 Vite 正式知识页的职责.

## 适用场景

- 回看早期 `Vite + React` 工程结构.
- 对照现代模板理解工具链演进.
- 验证 Rust / WebAssembly 与前端工程的联调思路.

## 当前特征

- React 17.
- `Vite 2` 时代的脚手架结构.
- 包含 [wasm-test 实验说明](./wasm-test/README.md) 子工程, 适合回看前端与 Rust 混合实验方式.
- 更适合作为旧工程组织方式的参考, 不建议直接作为新项目模板.

## 常用命令

```sh
npm install
npm run dev
npm run build
```

## 目录边界

- 前端知识点回到 `frontend/` 主文档.
- Rust / WASM 侧知识点回到 `program/rust/` 相关专题页.
- `wasm-test/` 由 [wasm-test 实验说明](./wasm-test/README.md) 承接入口.
- 本目录仅保留工程结构和实验上下文.

## 使用建议

- 若只是查 React 或 Vite 知识点, 优先回到主文档.
- 若需要运行该项目, 先确认 Node 版本与旧依赖是否兼容.
- 若是为了迁移历史工程, 建议只提取结构和集成思路, 不直接复制旧依赖组合.
