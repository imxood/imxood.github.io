# RustDesk

## 说明

- `RustDesk` 是 Rust 生态中常见的远程桌面项目之一.
- 本页当前以源码构建为主, 适合做本地编译和二次开发排查.

## 编译参考

- 官方说明: <https://github.com/rustdesk/rustdesk#raw-steps-to-build>

## Windows 构建

```sh
vcpkg install libvpx:x64-windows-static libyuv:x64-windows-static opus:x64-windows-static aom:x64-windows-static
cargo build --release
cargo run --release
```

## 常见关注点

- 构建前先确认 `vcpkg` 环境与依赖架构一致.
- 若 Rust 侧编译失败, 再回头检查系统库是否已正确集成到构建环境中.
- 若只是使用远程桌面功能, 通常不必自行构建, 可优先使用官方发布版本.
- 若要做二次开发, 建议先把最小构建流程跑通, 再进入 UI 或网络层修改.
