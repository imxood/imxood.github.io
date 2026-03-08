# windows-rs

## 说明

- `windows-rs` 用于在 Rust 中调用 Windows API, WinRT API 和相关系统类型.
- 本页聚焦一个常见的字符串传参与返回值转换场景: `String -> HSTRING -> PCWSTR`.
- 若你在排查 FFI 或平台绑定问题, 也可以结合 [Rust 问题汇总](./rust问题汇总.md) 一起看.

## 适用场景

- 在 Rust 桌面程序中调用 Windows Shell, 窗口, 文件系统等系统 API.
- 需要把 Rust 字符串安全地转换为 Windows 侧常见的 UTF-16 表示.
- 需要从返回的宽字符串结果再转回 Rust 侧可用的文本类型.

## 常见依赖

```toml
[dependencies]
windows = { version = "0.62", features = [
  "Win32_UI_Shell",
  "Win32_Foundation",
] }
```

- 实际 `features` 需要按目标 API 所在模块启用.
- 若编译时报模块未找到, 首先检查 `features` 是否覆盖到对应命名空间.

## 示例: 设置与读取 AppUserModelID

```rust
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use windows::core::{HSTRING, PCWSTR};
use windows::Win32::UI::Shell;

pub fn set_current_process_explicit_app_user_model_id() {
    let app_id = HSTRING::from("EhExt");

    unsafe {
        match Shell::SetCurrentProcessExplicitAppUserModelID(PCWSTR::from_raw(app_id.as_ptr())) {
            Ok(_) => {
                log::info!("SetCurrentProcessExplicitAppUserModelID ok");
            }
            Err(error) => {
                log::info!("SetCurrentProcessExplicitAppUserModelID error: {error}");
            }
        }
    }
}

pub fn get_current_process_explicit_app_user_model_id() {
    unsafe {
        match Shell::GetCurrentProcessExplicitAppUserModelID() {
            Ok(value) => {
                let value = OsString::from_wide(value.as_wide())
                    .to_string_lossy()
                    .into_owned();
                log::info!("GetCurrentProcessExplicitAppUserModelID: {value}");
            }
            Err(error) => {
                log::info!("GetCurrentProcessExplicitAppUserModelID error: {error}");
            }
        }
    }
}
```

## 理解要点

### `HSTRING`

- 适合和 WinRT / Windows 相关 API 交互.
- 内部是 UTF-16 表示, 常见于现代 Windows 绑定类型.

### `PCWSTR`

- 表示只读宽字符串指针.
- 常用于把 UTF-16 数据传给底层 Windows API.

### 返回值回到 Rust 字符串

- 对宽字符结果, 常见做法是借助 `OsString::from_wide` 转换.
- 转成 `String` 时, 可使用 `to_string_lossy()` 作为兼容路径.

## 生命周期注意点

- `PCWSTR::from_raw(app_id.as_ptr())` 依赖底层数据在调用期间仍然有效.
- 因此要像示例那样先把 `HSTRING` 绑定到局部变量, 不要直接把临时值内联成悬空指针来源.
- 如果 API 需要长期持有这段指针, 还要进一步确认其所有权与释放策略.

## 排障建议

- 编译不过时, 先查 `features` 和 `use` 路径是否正确.
- 运行时报错时, 结合返回的 `HRESULT` 或 `windows` crate 错误信息定位.
- 如果只是字符串互转有问题, 先把问题缩小到 `HSTRING`, `PWSTR`, `PCWSTR` 三类类型之间的区别.

## 延伸阅读

- 若问题更偏原生互操作, 可继续看 `libc`, `bindgen`, `Windows API` 等相关页面.
- 若目标是桌面 GUI 工程, 还要结合实际框架对窗口句柄和消息循环的封装方式一起看.
