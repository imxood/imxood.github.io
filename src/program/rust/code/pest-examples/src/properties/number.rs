use std::{
    cmp::Ordering,
    ops::{Add, Mul},
};

use super::CssProp;
use crate::parser::*;

use derive_more::{Add, Constructor, Div, Mul, Sub};

pub type Float = f32;
pub type Int = i32;

impl CssProp for u8 {
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pairs) = CssParser::parse(CssRule::uint8, i).ok() {
            Self::parse(pairs.last().unwrap())
        } else {
            None
        }
    }
    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        pair.as_str().parse::<u8>().map(|v| v).ok()
    }
    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        dest.write_fmt(format_args!("{}", self))
    }
}

impl CssProp for Int {
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pairs) = CssParser::parse(CssRule::int, i).ok() {
            Self::parse(pairs.last().unwrap())
        } else {
            None
        }
    }
    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        match pair.as_rule() {
            CssRule::int => pair.as_str().parse::<Int>().map(|v| v).ok(),
            _ => None,
        }
    }
    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        dest.write_fmt(format_args!("{}", self))
    }
}

impl CssProp for Float {
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pairs) = CssParser::parse(CssRule::float, i).ok() {
            Self::parse(pairs.last().unwrap())
        } else {
            None
        }
    }
    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        match pair.as_rule() {
            CssRule::float => pair.as_str().parse::<Float>().map(|v| v).ok(),
            _ => None,
        }
    }
    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        dest.write_fmt(format_args!("{}", self))
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy, Constructor)]
pub struct HexU8(pub u8);

impl CssProp for HexU8 {
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pairs) = CssParser::parse(CssRule::hex_u8, i).ok() {
            Self::parse(pairs.last().unwrap())
        } else {
            None
        }
    }
    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        u8::from_str_radix(pair.as_str(), 16).map(|v| Self(v)).ok()
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
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pairs) = CssParser::parse(CssRule::length_percentage, i).ok() {
            Self::parse(pairs.last().unwrap())
        } else {
            None
        }
    }

    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        println!("rule: {:?}, str: {}", pair.as_rule(), pair.as_str());
        for pair in pair.into_inner() {
            let v = match pair.as_rule() {
                CssRule::length => Length::parse(pair).map(|v| Self::Length(v)),
                CssRule::percentage => Percentage::parse(pair).map(|v| Self::Percentage(v)),
                _ => None,
            };
            return v;
        }
        None
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
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pairs) = CssParser::parse(CssRule::percentage, i).ok() {
            Self::parse(pairs.last().unwrap())
        } else {
            None
        }
    }
    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        match pair.as_rule() {
            CssRule::percentage => {
                for inner_pair in pair.into_inner() {
                    match inner_pair.as_rule() {
                        CssRule::float => {
                            return inner_pair.as_str().parse::<Float>().map(|v| Self(v)).ok()
                        }
                        _ => break,
                    }
                }
                None
            }
            _ => None,
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
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pairs) = CssParser::parse(CssRule::length, i).ok() {
            Self::parse(pairs.last().unwrap())
        } else {
            None
        }
    }
    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        // println!("rule: {:?} text: {}", pair.as_rule(), pair.as_str());
        match pair.as_rule() {
            CssRule::length => {
                for inner_pair in pair.into_inner() {
                    match inner_pair.as_rule() {
                        CssRule::px => {
                            return Px::parse(inner_pair).map(|px| Self::Px(px));
                        }
                        CssRule::em => {
                            return Em::parse(inner_pair).map(|em| Self::Em(em));
                        }
                        CssRule::rem => {
                            return Rem::parse(inner_pair).map(|rem| Self::Rem(rem));
                        }
                        CssRule::cm => {
                            return Cm::parse(inner_pair).map(|cm| Self::Cm(cm));
                        }
                        CssRule::mm => {
                            return Mm::parse(inner_pair).map(|mm| Self::Mm(mm));
                        }
                        CssRule::inch => {
                            return In::parse(inner_pair).map(|inch| Self::In(inch));
                        }
                        _ => {}
                    };
                }
                None
            }
            _ => None,
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
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pairs) = CssParser::parse(CssRule::px, i).ok() {
            Self::parse(pairs.last().unwrap())
        } else {
            None
        }
    }
    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        for pair in pair.into_inner() {
            return pair.as_str().parse::<f32>().map(|v| Self::new(v)).ok();
        }
        None
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
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pairs) = CssParser::parse(CssRule::em, i).ok() {
            Self::parse(pairs.last().unwrap())
        } else {
            None
        }
    }
    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        // println!("rule: {:?} text: {}", pair.as_rule(), pair.as_str());
        for pair in pair.into_inner() {
            return pair.as_str().parse::<f32>().map(|v| Self::new(v)).ok();
        }
        None
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
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pairs) = CssParser::parse(CssRule::rem, i).ok() {
            Self::parse(pairs.last().unwrap())
        } else {
            None
        }
    }
    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        for pair in pair.into_inner() {
            return pair.as_str().parse::<f32>().map(|v| Self::new(v)).ok();
        }
        None
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
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pairs) = CssParser::parse(CssRule::mm, i).ok() {
            Self::parse(pairs.last().unwrap())
        } else {
            None
        }
    }
    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        for pair in pair.into_inner() {
            return pair.as_str().parse::<f32>().map(|v| Self::new(v)).ok();
        }
        None
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
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pairs) = CssParser::parse(CssRule::cm, i).ok() {
            Self::parse(pairs.last().unwrap())
        } else {
            None
        }
    }
    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        for pair in pair.into_inner() {
            return pair.as_str().parse::<f32>().map(|v| Self::new(v)).ok();
        }
        None
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
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pairs) = CssParser::parse(CssRule::inch, i).ok() {
            Self::parse(pairs.last().unwrap())
        } else {
            None
        }
    }
    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        for pair in pair.into_inner() {
            return pair.as_str().parse::<f32>().map(|v| Self::new(v)).ok();
        }
        None
    }
    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        dest.write_fmt(format_args!("{}in", self.0))
    }
}

#[test]
fn test_uint8() {
    let v = u8::parse_str("230");
    assert_eq!(v, Some(230));

    let v = u8::parse_str("255");
    assert_eq!(v, Some(255));

    let v = u8::parse_str("100");
    assert_eq!(v, Some(100));

    let v = u8::parse_str("10");
    assert_eq!(v, Some(10));

    let v = u8::parse_str("5");
    assert_eq!(v, Some(5));

    let v = u8::parse_str("0");
    assert_eq!(v, Some(0));
}

#[test]
fn test_int() {
    let v = Int::parse_str("128");
    assert_eq!(v, Some(128));

    let v = Int::parse_str("0");
    assert_eq!(v, Some(0));

    let v = Int::parse_str("100000000");
    assert_eq!(v, Some(100000000));
}

#[test]
fn test_float() {
    let v = Float::parse_str("0.0");
    assert_eq!(v, Some(0.0));

    let v = Float::parse_str("-1.0");
    assert_eq!(v, Some(-1.0));

    let v = Float::parse_str("1.0");
    assert_eq!(v, Some(1.0));
}

#[test]
fn test_hex_u8() {
    let v = HexU8::parse_str("fa");
    assert_eq!(v, Some(HexU8::new(0xfa)));

    let v = HexU8::parse_str("faa");
    assert_eq!(v, Some(HexU8::new(0xfa)));

    let v = HexU8::parse_str("00");
    assert_eq!(v, Some(HexU8::new(0)));
}
