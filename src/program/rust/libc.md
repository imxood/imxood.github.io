# libc 与 C 运行时互操作

## 说明

- 本页记录 Rust 与 C 运行时交互时的一个实际片段: 自定义 `printf`.
- 适合处理“C 库输出和宿主程序日志时序不同步”的场景.

## 自定义 `printf`

遇到过这样一种情况: Rust 调用 C 函数时, 最终编译出的进程由 C# 宿主启动, `printf` 的输出相对 Rust 日志存在滞后, 因此用 Rust 导出的 `r_printf` 替代 C 侧的 `printf`.

C 侧替换方式:

```c
int r_printf(const char *format, ...);

#define printf r_printf
#define DBGPRINTF r_printf
```

Rust 侧实现:

```rust
use std::io::Write;
use printf_compat::{format, output};

#[no_mangle]
pub unsafe extern "C" fn r_printf(ss: *const i8, mut args: ...) -> i32 {
    let mut s = String::new();
    let bytes_written = format(ss, args.as_va_list(), output::fmt_write(&mut s));
    print!("{}", s);
    std::io::stdout().flush().unwrap();
    bytes_written
}
```

## 使用场景

- C 库和 Rust 日志需要统一输出顺序.
- 宿主程序对 stdout 的刷新时机较敏感.
- 需要在 Rust 侧接管 C 风格可变参数打印入口.

## 注意

- 这种做法更偏工程层兼容技巧, 要注意 ABI 和可变参数实现方式.
- 若跨平台使用, 需要进一步验证不同目标平台上的可用性.
