# Rust 代码示例归档

## 说明

- 本目录保存 Rust 相关的实验工程, 示例代码和技术验证项目.
- 这里以可运行样例归档为主, 不等同于稳定的知识文档.
- 若只是查概念或用法, 优先返回 `program/rust/` 下的主文档.

## 当前归档目录

### 解析与模板

- [css-parser 示例](./css-parser/README.md)
- [css-parser-macro 示例](./css-parser-macro/README.md)
- [Handlebars 示例](./hbs-examples/README.md)
- [nom 示例](./nom-examples/README.md)
- [pest 示例](./pest-examples/README.md)
- [Tera 示例](./tera-examples/README.md)

### 系统与工具

- [call-process 示例](./call-process/README.md)
- [notify-copy 示例](./notify-copy/README.md)
- [serial-demo 示例](./serial-demo/README.md)
- [Windows API 示例](./windows-api-demo/README.md)

### 图形与渲染

- [wgpu 示例](./wgpu-examples/README.md)
- [raqote 示例](./raqote-examples/README.md)
- [字体资源归档](./fonts/README.md)

### 网络与服务

- [libp2p relay 示例运行记录](./libp2p-learn/Readmd.md)
- [Poem 示例](./poem-examples/README.md)

### Web 与桌面混合实验

- [Svelte Web + Tauri 示例归档](./svelte-web/README.md)

## 第一批低优先级迁出候选

- [字体资源归档](./fonts/README.md): 当前更偏资源归档, 后续可迁出 `src/` 主知识树.
- [Svelte Web + Tauri 示例归档](./svelte-web/README.md): 当前更偏重型混合实验归档, 正式知识已回流主文档.
- [test_usb 实验目录说明](../test_usb/README.md): 已补最小入口页, 后续可继续评估是否并入更统一的实验归档区.

## 适合如何使用

- 需要回看某类技术验证的最小工程时, 可按目录名检索.
- 需要把旧实验沉淀为正式知识页时, 应把结论回收至 `program/rust/` 主文档.
- 需要运行某个样例时, 先检查各自的 `Cargo.toml`, `package.json` 和本地工具链版本.

## 归档边界

- 本目录保留的是样例工程上下文, 不是面向检索的知识总结.
- 某个样例一旦有稳定复用价值, 应升级为正式文档, 而不是无限扩写样例目录 README.
- 长期无维护价值的实验目录, 后续可继续评估是否迁出主知识树.
