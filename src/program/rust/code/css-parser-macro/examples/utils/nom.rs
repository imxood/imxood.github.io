pub use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_till1},
    character::complete::{digit1, multispace0, none_of},
    combinator::{all_consuming, map, opt, peek},
    multi::{many0, separated_list1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult, Parser as NomParser,
};

pub use nom::character::complete::{char as nom_char, u8 as nom_u8};
use nom::{
    bytes::complete::take_until,
    character::complete::multispace1,
    error::{Error as IError, ParseError},
};

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
