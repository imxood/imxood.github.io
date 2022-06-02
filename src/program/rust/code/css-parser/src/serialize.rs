use crate::color::Color;

pub trait ToCss {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write;

    #[inline]
    fn to_css_string(&self) -> String {
        let mut s = String::new();
        self.to_css(&mut s).unwrap();
        s
    }
}

macro_rules! impl_tocss_for_int {
    ($T: ty) => {
        impl<'a> ToCss for $T {
            fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
            where
                W: core::fmt::Write,
            {
                let mut buf = itoa::Buffer::new();
                dest.write_str(buf.format(*self))
            }
        }
    };
}

impl_tocss_for_int!(i8);
impl_tocss_for_int!(u8);
impl_tocss_for_int!(i16);
impl_tocss_for_int!(u16);
impl_tocss_for_int!(i32);
impl_tocss_for_int!(u32);
impl_tocss_for_int!(i64);
impl_tocss_for_int!(u64);

impl ToCss for f32 {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        dest.write_str(&format!("{}", &self))
    }
}

impl ToCss for Color {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        dest.write_fmt(format_args!(
            "rgba({}, {}, {}, {})",
            self.red, self.green, self.blue, self.alpha,
        ))
    }
}
