# Makepad

## 说明

- `Makepad` 是 Rust 生态中的 GUI / 创作型应用框架之一.
- 本页当前主要保留打包与发布方向的入口说明.

## 打包说明

- 官方文档: https://book.makepad.rs/zh/guide/appendix/packaging-guide

## 当前关注点

- 桌面端分发方式.
- 资源文件和运行时依赖的打包方式.
- 不同平台上的发布差异, 例如 Windows, macOS, Linux.

## 使用建议

- 如果只是验证界面与交互, 先跑开发模式即可, 不必一开始就做打包.
- 真正准备发布时, 优先核对官方 Packaging Guide, 再根据目标平台整理脚本.
- 若后续积累更多内容, 可继续补充安装流程, 示例项目结构和常见报错排查.
