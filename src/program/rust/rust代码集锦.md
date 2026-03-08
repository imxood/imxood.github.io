# Rust 代码集锦

## 说明

- 本页用于收集可复用的 Rust 代码片段.
- 当前先保留一个“轻量状态码类型 + 宏生成常量”的示例, 用来展示新类型封装与批量常量定义的写法.
- 如果后续片段继续增多, 建议再按“错误处理”, “并发”, “宏技巧”, “序列化”拆分小节.

## 示例: 状态码新类型封装

```rust
use std::fmt;
use std::num::NonZeroU8;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StatusCode(NonZeroU8);

impl StatusCode {
    pub const fn as_u8(self) -> u8 {
        self.0.get()
    }

    pub fn canonical_reason(self) -> Option<&'static str> {
        canonical_reason(self.as_u8())
    }
}

macro_rules! status_codes {
    (
        $(
            $(#[$docs:meta])*
            ($num:expr, $konst:ident, $phrase:expr);
        )+
    ) => {
        impl StatusCode {
            $(
                $(#[$docs])*
                pub const $konst: StatusCode = StatusCode(
                    unsafe { NonZeroU8::new_unchecked($num) }
                );
            )+
        }

        fn canonical_reason(num: u8) -> Option<&'static str> {
            match num {
                $(
                    $num => Some($phrase),
                )+
                _ => None,
            }
        }
    };
}

status_codes! {
    (1, SUCCESS, "Success");
    (2, FAIL, "Fail");
    (3, PROCESSING, "Processing");
    (4, READY, "Ready");
}

impl Default for StatusCode {
    fn default() -> StatusCode {
        StatusCode::READY
    }
}

impl From<StatusCode> for u8 {
    fn from(status: StatusCode) -> u8 {
        status.as_u8()
    }
}

impl fmt::Debug for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StatusCode({})", self.as_u8())
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            self.as_u8(),
            self.canonical_reason().unwrap_or("<unknown status code>")
        )
    }
}
```

## 这个例子值得参考的点

### 1. 用新类型包装原始值

- 业务语义比裸 `u8` 更清晰.
- 对外暴露的接口也更容易限制非法输入范围.

### 2. 用宏集中声明常量

- 避免多处重复写常量和说明文本.
- 让 `canonical_reason` 和常量定义保持同源.

### 3. 统一常见 trait

- `Default`, `Display`, `Debug`, `From` 都是很常见的外围接口.
- 片段一旦准备复用, 这类基础 trait 通常值得一起补齐.

## 关键注意点

- `NonZeroU8` 不能保存 `0`, 因此常量值绝不能写成 `0`.
- 如果你的业务语义里 `0` 是合法状态码, 应改用普通 `u8` 或换一种编码方式.
- `unsafe { NonZeroU8::new_unchecked(...) }` 必须保证传入值恒不为 `0`, 否则属于未定义行为.

## 适用场景

- 小型协议状态码封装.
- 内部错误码或流程状态枚举的轻量实现.
- 需要兼顾数值表示和可读文本说明的场景.

## 后续扩展方向

- 增加 `TryFrom<u8>` 支持外部输入校验.
- 增加 `serde` 支持, 便于序列化和接口传输.
- 当状态集合稳定且语义强时, 也可以直接评估是否改为 `enum`.
