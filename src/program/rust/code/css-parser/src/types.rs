use nom::branch::alt;
use nom::bytes::complete::{tag, take_till1};
use nom::character::complete::{digit1, multispace0};
use nom::combinator::{map, opt};
use nom::number::complete::float;
use nom::sequence::{pair, preceded, tuple};
use nom::IResult;

use css_parser_macro::{define_property_list, CssCodec};

use crate::color::Color;
use crate::parse::nom_char;
use crate::serialize::CssCodec;

pub type Float = f32;

/// 每一个像素 app units 的数量
pub const AU_PER_PX: Float = 60.;
/// 每一寸 app units 的数量
pub const AU_PER_IN: Float = AU_PER_PX * 96.;
/// 每一厘米 app units 的数量
pub const AU_PER_CM: Float = AU_PER_IN / 2.54;
/// 每一毫米 app units 的数量
pub const AU_PER_MM: Float = AU_PER_IN / 25.4;

define_property_list!(
    [width, "<length> | <percentage>", "enum"],
    [height, "<length> | <percentage>", "enum"],
    [
        background,
        "[<background-color> | <background-image>] <background-position>?",
        "struct"
    ],
    [background_color, "<color>", "struct"],
    [background_image, "<image>", "struct"],
    [background_position, "<position>", "struct"],
);

// struct StyleContext {
//     property: Property,
// }

impl CssCodec for Float {
    fn parse(input: &str) -> IResult<&str, Self> {
        float(input)
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        dest.write_fmt(format_args!("{}", self))
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

#[derive(Debug, PartialEq, CssCodec)]
pub enum Width {
    Length(Length),
    Percentage(Percentage),
}
