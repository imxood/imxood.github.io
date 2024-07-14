## 使用 flutter_rust_bridge 生成 dart 代码

参考: https://cjycode.com/flutter_rust_bridge/integrate/new_crate.html

### Rust 部分

进入 flutter 项目目录, 创建一个 rust 项目, 并 手动设置生成库类型为 "cdylib"

```
cargo new native --lib
```

```
[lib]
crate-type = ["staticlib", "cdylib", "rlib"]
```

rlib 是方便直接在 rust 中 执行 test, staticlib 可以在 MacOS 和 IOS 中直接链接静态库, 其它平台 使用动态库

给 rust 项目添加依赖

```
cargo add flutter_rust_bridge
```

创建 api.rs 文件, 并在 lib.rs 中 引用 api 模块.

在 api.rs 中, 定义一个 greet 函数, 作为演示.

```
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

### Flutter 部分

```
flutter pub add ffi flutter_rust_bridge freezed_annotation
flutter pub add --dev ffigen build_runner freezed
```

2023.1129 当前支持 ffigen 在 10.0 以下版本, 默认为 10.0 版本, 需要改成 9.0

## 生成 dart 代码

flutter_rust_bridge_codegen --rust-input native/src/api.rs --dart-output lib/bridge_generated.dart

## flutter 依赖

flutter pub add --dev freezed build_runner
flutter pub add freezed_annotation
