use core::ops::{Add, Mul};
use std::cmp::Ordering;

use crate::{parse::Parser, serialize::ToCss, properties::Property};

pub type Float = f32;

/// 每一个像素 app units 的数量
pub const AU_PER_PX: Float = 60.;
/// 每一寸 app units 的数量
pub const AU_PER_IN: Float = AU_PER_PX * 96.;
/// 每一厘米 app units 的数量
pub const AU_PER_CM: Float = AU_PER_IN / 2.54;
/// 每一毫米 app units 的数量
pub const AU_PER_MM: Float = AU_PER_IN / 25.4;

struct StyleContext {
    property: Property,
}

pub trait ToComputedValue {
    ///
    type ComputedValue;

    /// Convert a specified value to a computed value, using itself and the data
    /// inside the `Context`.
    fn to_computed_value(&self) -> Self::ComputedValue;
}

/// 精确长度
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AbsoluteLength {
    /// 像素
    Px(Float),
    /// 寸
    In(Float),
    /// 厘米
    Cm(Float),
    /// 毫米
    Mm(Float),
}

impl ToComputedValue for AbsoluteLength {
    type ComputedValue = Float;

    fn to_computed_value(&self) -> Self::ComputedValue {
        self.to_px()
    }
}

impl AbsoluteLength {
    /// 转换到像素值
    #[inline]
    pub fn to_px(&self) -> Float {
        let pixel = match *self {
            AbsoluteLength::Px(value) => value,
            AbsoluteLength::In(value) => value * (AU_PER_IN / AU_PER_PX),
            AbsoluteLength::Cm(value) => value * (AU_PER_CM / AU_PER_PX),
            AbsoluteLength::Mm(value) => value * (AU_PER_MM / AU_PER_PX),
        };
        pixel.min(f32::MAX).max(f32::MIN)
    }
}

impl PartialOrd for AbsoluteLength {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_px().partial_cmp(&other.to_px())
    }
}

impl Mul<Float> for AbsoluteLength {
    type Output = AbsoluteLength;

    #[inline]
    fn mul(self, scalar: Float) -> AbsoluteLength {
        match self {
            AbsoluteLength::Px(v) => AbsoluteLength::Px(v * scalar),
            AbsoluteLength::In(v) => AbsoluteLength::In(v * scalar),
            AbsoluteLength::Cm(v) => AbsoluteLength::Cm(v * scalar),
            AbsoluteLength::Mm(v) => AbsoluteLength::Mm(v * scalar),
        }
    }
}

impl Add<AbsoluteLength> for AbsoluteLength {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (AbsoluteLength::Px(x), AbsoluteLength::Px(y)) => AbsoluteLength::Px(x + y),
            (AbsoluteLength::In(x), AbsoluteLength::In(y)) => AbsoluteLength::In(x + y),
            (AbsoluteLength::Cm(x), AbsoluteLength::Cm(y)) => AbsoluteLength::Cm(x + y),
            (AbsoluteLength::Mm(x), AbsoluteLength::Mm(y)) => AbsoluteLength::Mm(x + y),
            _ => AbsoluteLength::Px(self.to_px() + rhs.to_px()),
        }
    }
}

/// 字体长度
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FontRelativeLength {
    /// A "em" value: https://drafts.csswg.org/css-values/#em
    /// 相对于父元素的倍数
    Em(Float),
    /// A "rem" value: https://drafts.csswg.org/css-values/#rem
    /// 相对于根元素的倍数
    Rem(Float),
}

impl ToComputedValue for FontRelativeLength {
    type ComputedValue = Float;

    fn to_computed_value(&self) -> Self::ComputedValue {
        match *self {
            FontRelativeLength::Em(v) => todo!(),
            FontRelativeLength::Rem(v) => todo!(),
        }
    }
}

pub enum Length {
    Absolute(AbsoluteLength),
    FontLength(FontRelativeLength),
}

#[derive(Debug, Clone, Copy)]
pub enum Number {
    /// 具体数值
    Specific(Float),
    /// 百分比例
    Percent(Float),
    /// 内部值
    Innner(NumberInnner),
}

impl Default for Number {
    fn default() -> Self {
        Number::Innner(NumberInnner::Auto)
    }
}

impl ToCss for Number {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        match *self {
            Number::Specific(v) => v.to_css(dest),
            Number::Percent(v) => {
                v.to_css(dest)?;
                dest.write_char('%')
            }
            Number::Innner(v) => v.to_css(dest),
        }
    }
}

impl Parser<Number> for Number {
    fn parse(i: &str) -> nom::IResult<&str, Number> {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NumberInnner {
    /// 默认值
    Default,
    /// 自动值
    Auto,
    /// 继承父元素
    Inherit,
    /// 数值类型为 0, 非数值为 default
    Unset,
}

impl ToCss for NumberInnner {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        match *self {
            NumberInnner::Default => dest.write_str("default"),
            NumberInnner::Auto => dest.write_str("auto"),
            NumberInnner::Inherit => dest.write_str("inherit"),
            NumberInnner::Unset => dest.write_str("unset"),
        }
    }
}
