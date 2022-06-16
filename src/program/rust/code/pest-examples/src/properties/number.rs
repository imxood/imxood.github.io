use std::{
    cmp::Ordering,
    ops::{Add, Mul},
};

use super::CssProp;
use crate::parser::*;

use derive_more::{Add, Constructor, Div, Mul, Sub};

pub type Float = f32;
pub type Int = i32;

impl CssProp for bool {
    fn rule() -> CssRule {
        CssRule::boolean
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let pair = pair.into_inner().next().unwrap();
        let v = match pair.as_rule() {
            CssRule::bool_true => true,
            CssRule::bool_false => false,
            _ => unreachable!(),
        };
        return v;
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        dest.write_fmt(format_args!("{}", self))
    }
}

impl CssProp for u8 {
    fn rule() -> CssRule {
        CssRule::uint8
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        pair.as_str().parse::<u8>().unwrap()
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        dest.write_fmt(format_args!("{}", self))
    }
}

impl CssProp for Int {
    fn rule() -> CssRule {
        CssRule::int
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        pair.as_str().parse::<Int>().unwrap()
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        dest.write_fmt(format_args!("{}", self))
    }
}

impl CssProp for Float {
    fn rule() -> CssRule {
        CssRule::float
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        pair.as_str().parse::<Float>().unwrap()
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        dest.write_fmt(format_args!("{}", self))
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy, Constructor)]
pub struct HexU8(pub u8);

impl CssProp for HexU8 {
    fn rule() -> CssRule {
        CssRule::hex_u8
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        u8::from_str_radix(pair.as_str(), 16)
            .map(|v| Self(v))
            .unwrap()
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        dest.write_fmt(format_args!("{}", self.0))
    }
}

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct PositiveNumber(pub f32);

impl CssProp for PositiveNumber {
    fn rule() -> CssRule {
        CssRule::positive_number
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        pair.as_str().parse::<f32>().map(|v| Self(v)).unwrap()
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        dest.write_fmt(format_args!("{}", self.0))
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LengthPercentage {
    Length(Length),
    Percentage(Percentage),
}

impl CssProp for LengthPercentage {
    fn rule() -> CssRule {
        CssRule::length_percentage
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            CssRule::length => Self::Length(Length::parse(pair)),
            CssRule::percentage => Self::Percentage(Percentage::parse(pair)),
            _ => unreachable!(),
        }
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        match self {
            LengthPercentage::Length(v) => v.to_css(dest),
            LengthPercentage::Percentage(v) => v.to_css(dest),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Constructor)]
pub struct Percentage(Float);

impl CssProp for Percentage {
    fn rule() -> CssRule {
        CssRule::percentage
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            CssRule::float => Self(pair.as_str().parse::<Float>().unwrap()),
            _ => unreachable!(),
        }
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        dest.write_fmt(format_args!("{}", self.0))
    }
}

// use super::ToComputedValue;

/// 每一个像素 app units 的数量
pub const AU_PER_PX: Float = 60.0;
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
    Px(Px),
    /// 寸
    In(In),
    /// 厘米
    Cm(Cm),
    /// 毫米
    Mm(Mm),
    /// 相对于父元素的倍数
    Em(Em),
    /// 相对于根元素的倍数
    Rem(Rem),
}

impl Default for Length {
    fn default() -> Self {
        Self::Px(Px::default())
    }
}

impl Length {
    pub fn to_px(&self) -> Float {
        let pixel = match *self {
            Length::Px(v) => v.to_px(),
            Length::In(v) => v.to_px(),
            Length::Cm(v) => v.to_px(),
            Length::Mm(v) => v.to_px(),
            Length::Em(v) => v.to_px(),
            Length::Rem(v) => v.to_px(),
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

impl CssProp for Length {
    fn rule() -> CssRule {
        CssRule::length
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            CssRule::px => Self::Px(Px::parse(pair)),
            CssRule::em => Self::Em(Em::parse(pair)),
            CssRule::rem => Self::Rem(Rem::parse(pair)),
            CssRule::cm => Self::Cm(Cm::parse(pair)),
            CssRule::mm => Self::Mm(Mm::parse(pair)),
            CssRule::inch => Self::In(In::parse(pair)),
            _ => unreachable!(),
        }
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        match self {
            Length::Px(px) => px.to_css(dest),
            Length::In(inch) => inch.to_css(dest),
            Length::Cm(cm) => cm.to_css(dest),
            Length::Mm(mm) => mm.to_css(dest),
            Length::Em(em) => em.to_css(dest),
            Length::Rem(rem) => rem.to_css(dest),
        }
    }
}

// impl ToComputedValue for Length {
//     type ComputedValue = Float;

//     fn to_computed_value(&self) -> Self::ComputedValue {
//         let pixel = match *self {
//             Length::Px(value) => value,
//             Length::In(value) => Self::in_to_px(value),
//             Length::Cm(value) => Self::cm_to_px(value),
//             Length::Mm(value) => Self::mm_to_px(value),
//             Length::Em(v) => todo!(),
//             Length::Rem(v) => todo!(),
//         };
//         pixel.min(f32::MAX).max(f32::MIN)
//     }
// }

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
            (Length::Px(x), y) => Length::Px(x + Px::new(y.to_px())),
            (Length::In(x), y) => Length::In(x + In::new(Self::px_to_in(y.to_px()))),
            (Length::Cm(x), y) => Length::Cm(x + Cm::new(Self::px_to_cm(y.to_px()))),
            (Length::Mm(x), y) => Length::Mm(x + Mm::new(Self::px_to_mm(y.to_px()))),
            (Length::Em(_), _) => self,
            (Length::Rem(_), _) => self,
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy, Constructor, Add, Sub, Mul, Div)]
pub struct Px(pub Float);

impl Px {
    pub fn to_px(&self) -> Float {
        self.0
    }
}

impl CssProp for Px {
    fn rule() -> CssRule {
        CssRule::px
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let pair = pair.into_inner().next().unwrap();
        Self::new(pair.as_str().parse::<f32>().unwrap())
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        dest.write_fmt(format_args!("{}px", self.0))
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy, Constructor, Add, Sub, Mul, Div)]
pub struct Em(Float);

impl Em {
    pub fn to_px(&self) -> Float {
        // self.0
        0.0
    }
}

impl CssProp for Em {
    fn rule() -> CssRule {
        CssRule::em
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let pair = pair.into_inner().next().unwrap();
        Self::new(pair.as_str().parse::<f32>().unwrap())
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        dest.write_fmt(format_args!("{}em", self.0))
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy, Constructor, Add, Sub, Mul, Div)]
pub struct Rem(Float);

impl Rem {
    pub fn to_px(&self) -> Float {
        // self.0
        0.0
    }
}

impl CssProp for Rem {
    fn rule() -> CssRule {
        CssRule::rem
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let pair = pair.into_inner().next().unwrap();
        Self::new(pair.as_str().parse::<f32>().unwrap())
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        dest.write_fmt(format_args!("{}rem", self.0))
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy, Constructor, Add, Sub, Mul, Div)]
pub struct Mm(Float);

impl Mm {
    pub fn to_px(&self) -> Float {
        Length::mm_to_px(self.0)
    }
}

impl CssProp for Mm {
    fn rule() -> CssRule {
        CssRule::mm
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let pair = pair.into_inner().next().unwrap();
        Self::new(pair.as_str().parse::<f32>().unwrap())
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        dest.write_fmt(format_args!("{}mm", self.0))
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy, Constructor, Add, Sub, Mul, Div)]
pub struct Cm(Float);

impl Cm {
    pub fn to_px(&self) -> Float {
        Length::cm_to_px(self.0)
    }
}

impl CssProp for Cm {
    fn rule() -> CssRule {
        CssRule::cm
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let pair = pair.into_inner().next().unwrap();
        Self::new(pair.as_str().parse::<f32>().unwrap())
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        dest.write_fmt(format_args!("{}cm", self.0))
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy, Constructor, Add, Sub, Mul, Div)]
pub struct In(Float);

impl In {
    pub fn to_px(&self) -> Float {
        Length::in_to_px(self.0)
    }
}

impl CssProp for In {
    fn rule() -> CssRule {
        CssRule::inch
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let pair = pair.into_inner().next().unwrap();
        Self::new(pair.as_str().parse::<f32>().unwrap())
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        dest.write_fmt(format_args!("{}in", self.0))
    }
}

#[test]
fn test_uint8() {
    let v = u8::parse_str("230");
    assert_eq!(v, Ok(230));

    let v = u8::parse_str("255");
    assert_eq!(v, Ok(255));

    let v = u8::parse_str("100");
    assert_eq!(v, Ok(100));

    let v = u8::parse_str("10");
    assert_eq!(v, Ok(10));

    let v = u8::parse_str("5");
    assert_eq!(v, Ok(5));

    let v = u8::parse_str("0");
    assert_eq!(v, Ok(0));
}

#[test]
fn test_boolean() {
    let v = bool::parse_str("true");
    assert_eq!(v, Ok(true));

    let v = bool::parse_str("false");
    assert_eq!(v, Ok(false));

    let v = bool::parse_str("hello");
    assert_eq!(v.is_err(), true);
}

#[test]
fn test_int() {
    let v = Int::parse_str("128");
    assert_eq!(v, Ok(128));

    let v = Int::parse_str("0");
    assert_eq!(v, Ok(0));

    let v = Int::parse_str("100000000");
    assert_eq!(v, Ok(100000000));
}

#[test]
fn test_float() {
    let v = Float::parse_str("0.0");
    assert_eq!(v, Ok(0.0));

    let v = Float::parse_str("-1.0");
    assert_eq!(v, Ok(-1.0));

    let v = Float::parse_str("1.0");
    assert_eq!(v, Ok(1.0));
}

#[test]
fn test_hex_u8() {
    let v = HexU8::parse_str("fa");
    assert_eq!(v, Ok(HexU8::new(0xfa)));

    let v = HexU8::parse_str("faa");
    assert_eq!(v, Ok(HexU8::new(0xfa)));

    let v = HexU8::parse_str("00");
    assert_eq!(v, Ok(HexU8::new(0)));
}
