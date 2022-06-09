mod parse;

use css_parser_macro::{define_property_list, CssCodec};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::multispace0;
use nom::combinator::{map, opt};
use nom::sequence::preceded;
use nom::IResult;

define_property_list!([width, "<length>", "enum"]);

fn main() {
    let width = Width::new();
    parse_width("").unwrap();
}

#[derive(Debug, PartialEq, CssCodec)]
pub enum Width {
    Length(Length),
}

impl Width {
    pub fn new() -> Self {
        Self::Length(Length::Px(0.0))
    }
}

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

pub trait CssCodec<T = Self> {
    fn parse(i: &str) -> IResult<&str, T>;

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result;

    #[inline]
    fn to_css_string(&self) -> String {
        let mut s = String::new();
        self.to_css(&mut s).unwrap();
        s
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
