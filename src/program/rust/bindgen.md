# bindgen

## 说明

- `bindgen` 用于根据 C 或 C++ 头文件自动生成 Rust FFI 绑定代码.
- 适合需要和已有原生库交互, 又不想手写大量 `extern` 声明的场景.
- 更原始的安装记录已并入当前文档体系, 后续统一以本页为准.

## 根据 C 头文件生成 Rust 接口

```rust
#[derive(Debug)]
pub struct CargoCallbacks;

impl ParseCallbacks for CargoCallbacks {
    fn process_comment(&self, comment: &str) -> Option<String> {
        println!("comment: {}", comment);
        None
    }
}

let bindings = Builder::default()
    .default_enum_style(EnumVariation::Rust {
        non_exhaustive: true,
    })
    .generate_comments(true)
    .header("src/error.h")
    .prepend_enum_name(false)
    .parse_callbacks(Box::new(CargoCallbacks))
    .layout_tests(false)
    .generate()
    .expect("Unable to generate bindings");

bindings
    .write_to_file("src/bindings.rs")
    .expect("Couldn't write bindings!");
```

## 常见关注点

- `header(...)`: 指定输入头文件.
- `default_enum_style(...)`: 控制枚举映射成 Rust 风格还是常量风格.
- `layout_tests(false)`: 可避免部分平台差异导致的布局测试问题.
- `parse_callbacks(...)`: 可在生成时接管注释, 重建依赖追踪等行为.

## 使用建议

- 先把 C 侧头文件和依赖链梳理干净, 再生成绑定会更稳.
- 生成结果通常建议进入专门文件, 例如 `bindings.rs`, 不要手写和自动生成混在一起.
- 生成失败时, 常见排查方向是 `libclang`, 头文件搜索路径和平台宏定义.
