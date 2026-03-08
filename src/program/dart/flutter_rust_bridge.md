# flutter_rust_bridge

## 说明

- `flutter_rust_bridge` 用于在 `Flutter` 与 `Rust` 之间生成跨语言绑定代码.
- 适合在保留 `Flutter` UI 的同时, 把性能敏感或可复用逻辑放到 `Rust` 侧实现.

## 参考资料

- 官方集成文档: <https://cjycode.com/flutter_rust_bridge/integrate/new_crate.html>

## 最小接入思路

1. 在 `Flutter` 工程中创建一个独立的 `Rust` 库.
2. 在 `Rust` 中定义对外 API.
3. 通过 `flutter_rust_bridge_codegen` 生成 `Dart` 侧桥接代码.
4. 在 `Flutter` 页面中调用生成后的接口完成联调.

## Rust 部分

在 `Flutter` 项目中创建一个 Rust 库项目:

```sh
cargo new native --lib
```

在 `Cargo.toml` 中设置:

```toml
[lib]
crate-type = ["staticlib", "cdylib", "rlib"]
```

添加依赖:

```sh
cargo add flutter_rust_bridge
```

示例 API:

```rust
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

## Flutter 部分

```sh
flutter pub add ffi flutter_rust_bridge freezed_annotation
flutter pub add --dev ffigen build_runner freezed
```

## 生成 Dart 代码

```sh
flutter_rust_bridge_codegen --rust-input native/src/api.rs --dart-output lib/bridge_generated.dart
```

## 联调关注点

- `Rust` 导出库的产物路径要与 `Flutter` 构建流程对应.
- 修改 `Rust` API 后, 记得重新生成桥接代码.
- 复杂参数类型要优先确认是否已被当前版本的生成器良好支持.
- 排错时先做最小 `greet` 示例, 再逐步引入异步, 流或复杂结构体.

## 建议阅读顺序

1. 先看本页跑通最小跨语言调用.
2. 再结合 [Flutter + Rust 环境](./flutter_rust.md) 检查本地工具链.
3. 真正进入业务后, 再补线程模型, 错误传递和打包发布相关笔记.
