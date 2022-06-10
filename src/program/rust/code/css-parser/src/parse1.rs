use std::fmt::Write;

pub use nom::character::complete::{char as nom_char, u8 as nom_u8};
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until},
    character::complete::{alphanumeric0, alphanumeric1, anychar, multispace0, multispace1},
    combinator::map,
    error::{ContextError, Error as IError, ErrorKind, ParseError},
    multi::many0,
    number::complete::float,
    sequence::{pair, preceded, terminated, tuple},
    IResult,
};

pub trait Parser<T = Self> {
    fn parse(i: &str) -> IResult<&str, T>;
}

/// 解析 空符号 和 注释
pub fn parse_useless(i: &str) -> IResult<&str, Vec<&str>> {
    many0(alt((multispace1, parse_comment)))(i)
}

/// 过滤 ' ' 和 '\t'
#[inline]
pub fn skip_sp<'a, O, P, E: ParseError<&'a str>>(
    parser: P,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    P: nom::Parser<&'a str, O, E>,
{
    preceded(multispace0, parser)
}

/// 解析注释
pub fn parse_comment(i: &str) -> IResult<&str, &str> {
    map(
        tuple((tag("/*"), take_until("*/"), tag("*/"))),
        |(_, text, _)| text,
    )(i)
}

/// 给 输入的解析器 去除空格 和 注释
pub fn skip_useless<'a, O, P>(parser: P) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    P: nom::Parser<&'a str, O, IError<&'a str>>,
{
    map(tuple((parse_useless, parser, parse_useless)), |(_, s, _)| s)
}
