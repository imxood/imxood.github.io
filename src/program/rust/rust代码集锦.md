# 一些好的 可能会使用的 代码

## 错误处理

```rust
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StatusCode(NonZeroU8);

impl StatusCode {
    pub fn canonical_reason(&self) -> Option<&'static str> {
        canonical_reason(self.0.get())
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
            pub const $konst: StatusCode = StatusCode(unsafe { NonZeroU8::new_unchecked($num) });
        )+

        }

        fn canonical_reason(num: u8) -> Option<&'static str> {
            match num {
                $(
                $num => Some($phrase),
                )+
                _ => None
            }
        }
    }
}

status_codes! {
    (0, SUCCESS, "Success");
    (1, FAIL, "Fail");
    (2, PROCESSING, "Processing");
    (3, READY, "Ready");
}

impl Default for StatusCode {
    #[inline]
    fn default() -> StatusCode {
        StatusCode::READY
    }
}

impl std::fmt::Debug for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl std::fmt::Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            u8::from(*self),
            self.canonical_reason().unwrap_or("<unknown status code>")
        )
    }
}

impl From<StatusCode> for u8 {
    #[inline]
    fn from(status: StatusCode) -> u8 {
        status.0.get()
    }
}
```