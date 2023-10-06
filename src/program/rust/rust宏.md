
## syn

用于把 rust 代码 转化成 rust 语法树

## quote

用于将 macro 中 的模板代码转换成 rust 的 TokenStream

## 把 ident 转换成 字符串

stringify!(#ident), 实际为 std::stringify! 宏.

## 如果是通过 bindgen 生成的代码, 存在 snake case warning

可以在 lib.rs 中禁用这些警告

#[allow(nonstandard_style)]
