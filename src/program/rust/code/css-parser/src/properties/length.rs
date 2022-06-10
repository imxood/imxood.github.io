use std::cmp::Ordering;
use std::ops::{Mul, Add};

use crate::properties::CssCodec;
use crate::utils::nom::*;

use super::ToComputedValue;

pub type Float = f32;

/// 每一个像素 app units 的数量
pub const AU_PER_PX: Float = 60.;
/// 每一寸 app units 的数量
pub const AU_PER_IN: Float = AU_PER_PX * 96.;
/// 每一厘米 app units 的数量
pub const AU_PER_CM: Float = AU_PER_IN / 2.54;
/// 每一毫米 app units 的数量
pub const AU_PER_MM: Float = AU_PER_IN / 25.4;

/// 精确长度
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Length {
    /// 像素
    Px(Float),
    /// 寸
    In(Float),
    /// 厘米
    Cm(Float),
    /// 毫米
    Mm(Float),
    /// 相对于父元素的倍数
    Em(Float),
    /// 相对于根元素的倍数
    Rem(Float),
}

impl Length {
    pub fn to_px(&self) -> Float {
        let pixel = match *self {
            Length::Px(value) => value,
            Length::In(value) => value * (AU_PER_IN / AU_PER_PX),
            Length::Cm(value) => value * (AU_PER_CM / AU_PER_PX),
            Length::Mm(value) => value * (AU_PER_MM / AU_PER_PX),
            Length::Em(v) => 0.0,
            Length::Rem(v) => 0.0,
        };
        pixel.min(f32::MAX).max(f32::MIN)
    }

    #[inline]
    pub fn in_to_px(value: Float) -> Float {
        value * (AU_PER_IN / AU_PER_PX)
    }

    #[inline]
    pub fn cm_to_px(value: Float) -> Float {
        value * (AU_PER_CM / AU_PER_PX)
    }

    #[inline]
    pub fn mm_to_px(value: Float) -> Float {
        value * (AU_PER_MM / AU_PER_PX)
    }

    #[inline]
    pub fn px_to_in(px: Float) -> Float {
        px / (AU_PER_IN / AU_PER_PX)
    }

    #[inline]
    pub fn px_to_cm(px: Float) -> Float {
        px / (AU_PER_CM / AU_PER_PX)
    }

    #[inline]
    pub fn px_to_mm(px: Float) -> Float {
        px / (AU_PER_MM / AU_PER_PX)
    }
}

impl CssCodec for Length {
    fn parse(i: &str) -> IResult<&str, Self> {
        let (i, num) = preceded(multispace0, digit1)(i)?;
        let num = str::parse::<Float>(num).unwrap();
        if i.is_empty() {
            return Ok((i, Self::Px(num)));
        }
        let v = alt((
            map(tag("px"), |_| Self::Px(num)),
            map(tag("in"), |_| Self::In(num)),
            map(tag("cm"), |_| Self::Cm(num)),
            map(tag("mm"), |_| Self::Mm(num)),
            map(tag("em"), |_| Self::Em(num)),
            map(tag("rem"), |_| Self::Rem(num)),
        ))(i)?;
        Ok(v)
    }

    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        match self {
            Length::Px(v) => dest.write_fmt(format_args!("{}px", v)),
            Length::In(v) => dest.write_fmt(format_args!("{}in", v)),
            Length::Cm(v) => dest.write_fmt(format_args!("{}cm", v)),
            Length::Mm(v) => dest.write_fmt(format_args!("{}mm", v)),
            Length::Em(v) => dest.write_fmt(format_args!("{}em", v)),
            Length::Rem(v) => dest.write_fmt(format_args!("{}rem", v)),
        }
    }
}

impl ToComputedValue for Length {
    type ComputedValue = Float;

    fn to_computed_value(&self) -> Self::ComputedValue {
        let pixel = match *self {
            Length::Px(value) => value,
            Length::In(value) => Self::in_to_px(value),
            Length::Cm(value) => Self::cm_to_px(value),
            Length::Mm(value) => Self::mm_to_px(value),
            Length::Em(v) => todo!(),
            Length::Rem(v) => todo!(),
        };
        pixel.min(f32::MAX).max(f32::MIN)
    }
}

impl PartialOrd for Length {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_px().partial_cmp(&other.to_px())
    }
}

impl Mul<Float> for Length {
    type Output = Length;

    #[inline]
    fn mul(self, scalar: Float) -> Length {
        match self {
            Length::Px(v) => Length::Px(v * scalar),
            Length::In(v) => Length::In(v * scalar),
            Length::Cm(v) => Length::Cm(v * scalar),
            Length::Mm(v) => Length::Mm(v * scalar),
            Length::Em(v) => Length::Em(v * scalar),
            Length::Rem(v) => Length::Rem(v * scalar),
        }
    }
}

impl Add<Length> for Length {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Length::Px(x), Length::Px(y)) => Length::Px(x + y),
            (Length::In(x), Length::In(y)) => Length::In(x + y),
            (Length::Cm(x), Length::Cm(y)) => Length::Cm(x + y),
            (Length::Mm(x), Length::Mm(y)) => Length::Mm(x + y),
            (Length::Em(x), Length::Em(y)) => Length::Em(x + y),
            (Length::Rem(x), Length::Rem(y)) => Length::Rem(x + y),
            (Length::Px(x), y) => Length::Px(x + y.to_px()),
            (Length::In(x), y) => Length::In(Self::px_to_in(Self::in_to_px(x) + y.to_px())),
            (Length::Cm(x), y) => Length::Cm(Self::px_to_cm(Self::cm_to_px(x) + y.to_px())),
            (Length::Mm(x), y) => Length::Mm(Self::px_to_mm(Self::mm_to_px(x) + y.to_px())),
            (Length::Em(_), _) => self,
            (Length::Rem(_), _) => self,
        }
    }
}
