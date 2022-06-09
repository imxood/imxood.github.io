use core::ops::{Add, Mul};
use std::cmp::Ordering;

use css_parser_macro::{Parser, ToCss};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till1};
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};
use nom::IResult;
use nom::{character::complete::digit1, sequence::pair};

use crate::color::Color;
use crate::{parse::nom_char, serialize::ToCss};

pub type Float = f32;

/// 每一个像素 app units 的数量
pub const AU_PER_PX: Float = 60.;
/// 每一寸 app units 的数量
pub const AU_PER_IN: Float = AU_PER_PX * 96.;
/// 每一厘米 app units 的数量
pub const AU_PER_CM: Float = AU_PER_IN / 2.54;
/// 每一毫米 app units 的数量
pub const AU_PER_MM: Float = AU_PER_IN / 25.4;

// struct StyleContext {
//     property: Property,
// }

pub trait ToComputedValue {
    ///
    type ComputedValue;

    /// Convert a specified value to a computed value, using itself and the data
    /// inside the `Context`.
    fn to_computed_value(&self) -> Self::ComputedValue;
}

impl Parser for Float {
    fn parse(input: &str) -> IResult<&str, Self> {
        float(input)
    }
}

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

impl ToCss for Length {
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

impl crate::parse::Parser for Length {
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
}

#[derive(Debug, PartialEq)]
pub struct Percentage(Float);

impl ToComputedValue for Percentage {
    type ComputedValue = Float;

    fn to_computed_value(&self) -> Self::ComputedValue {
        self.0
    }
}

impl crate::parse::Parser for Percentage {
    fn parse(i: &str) -> IResult<&str, Self> {
        let (i, num) = preceded(multispace0, digit1)(i)?;
        let num = str::parse::<Float>(num).unwrap();
        let _ = nom_char('%')(i)?;
        Ok((i, Self(num)))
    }
}

impl ToCss for Percentage {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        dest.write_fmt(format_args!("{}%", self.0))
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Position {
    pub x: Float,
    pub y: Float,
}

impl ToComputedValue for Position {
    type ComputedValue = Float;

    fn to_computed_value(&self) -> Self::ComputedValue {
        self.x
    }
}

impl crate::parse::Parser for Position {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(
            tuple((
                multispace0,
                nom_char('('),
                multispace0,
                digit1,
                multispace0,
                nom_char(','),
                multispace0,
                digit1,
                multispace0,
                nom_char(')'),
            )),
            |(_, _, _, x, _, _, _, y, _, _)| {
                let x = str::parse::<Float>(x).unwrap();
                let y = str::parse::<Float>(y).unwrap();
                Self { x, y }
            },
        )(i)
    }
}

impl ToCss for Position {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        dest.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Image {
    pub url: String,
}

impl ToComputedValue for Image {
    type ComputedValue = String;

    fn to_computed_value(&self) -> Self::ComputedValue {
        self.url.clone()
    }
}

impl crate::parse::Parser for Image {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(
            tuple((
                multispace0,
                tag("url("),
                multispace0,
                take_till1(|c: char| c == ')'),
                nom_char(')'),
            )),
            |(_, _, _, url, _): (_, _, _, &str, _)| {
                let url = url.to_string();
                Self { url }
            },
        )(i)
    }
}

impl ToCss for Image {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        dest.write_fmt(format_args!("url({})", &self.url))
    }
}

#[derive(Debug, PartialEq, Parser, ToCss)]
pub enum Width {
    Length(Length),
    Percentage(Percentage),
}

#[derive(Debug, PartialEq)]
pub enum Height {
    Length(Length),
    Percentage(Percentage),
}

#[derive(Debug, Default, PartialEq)]
pub struct Background {
    pub background_color: BackgroundColor,
    pub background_image: BackgroundImage,
    pub background_position: BackgroundPosition,
}

#[derive(Debug, Default, PartialEq)]
pub struct BackgroundColor {
    pub color: Color,
}

#[derive(Debug, Default, PartialEq)]
pub struct BackgroundImage {
    pub image: Image,
}

#[derive(Debug, Default, PartialEq, Parser, ToCss)]
pub struct BackgroundPosition {
    pub position: Position,
}
