## 根据 c 语言的头文件 生成 rust 接口

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
    // 生成 rust原生的 enum类型
    .default_enum_style(EnumVariation::Rust {
        non_exhaustive: true,
    })
    .generate_comments(true)
    // 输入的 头文件
    .header("src/error.h")
    .prepend_enum_name(false)
    // Tell cargo to invalidate the built crate whenever any of the
    // included header files changed.
    .parse_callbacks(Box::new(CargoCallbacks))
    // 生成 Layout测试
    .layout_tests(false)
    // Finish the builder and generate the bindings.
    .generate()
    .expect("Unable to generate bindings");

// 生成的内容写入到 src/bindings.rs
bindings
    .write_to_file("src/bindings.rs")
    .expect("Couldn't write bindings!");
```
