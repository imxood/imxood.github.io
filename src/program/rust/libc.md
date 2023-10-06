## 自定义 printf

遇到了这种情况, 在rust中调用c函数时, 编译出的进程在c#中, printf函数的输出会滞后在rust后面, 所以自定义了 r_printf rust函数 替代了 c中的printf函数.

```c
int r_printf(const char *format, ...);

#define printf r_printf

#define DBGPRINTF r_printf
```

```rs
use std::io::Write;
use printf_compat::{format, output};

#[no_mangle]
pub unsafe extern "C" fn r_printf(ss: *const i8, mut args: ...) -> i32 {
    // let format = unsafe { CStr::from_ptr(format) };
    // let mut result = String::new();
    // let _ = write!(result, "{}", format.to_str().unwrap());
    // result.len() as i32

    let mut s = String::new();
    let bytes_written = format(ss, args.as_va_list(), output::fmt_write(&mut s));
    print!("{}", s);
    std::io::stdout().flush().unwrap();
    bytes_written
}
```
