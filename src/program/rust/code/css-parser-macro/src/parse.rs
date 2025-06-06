pub use nom::character::complete::{char as nom_char, u8 as nom_u8};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alphanumeric1, anychar, multispace0, multispace1},
    combinator::map,
    error::{ContextError, Error as IError, ErrorKind, ParseError},
    multi::many0,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug)]
pub struct DebugError {
    message: String,
}

impl ParseError<&str> for DebugError {
    // on one line, we show the error code and the input that caused it
    fn from_error_kind(input: &str, kind: ErrorKind) -> Self {
        let message = format!("{:?}:\t{:?}\n", kind, input);
        println!("{}", message);
        Self { message }
    }

    // if combining multiple errors, we show them one after the other
    fn append(input: &str, kind: ErrorKind, other: Self) -> Self {
        let message = format!("{}{:?}:\t{:?}\n", other.message, kind, input);
        println!("{}", message);
        Self { message }
    }

    fn from_char(input: &str, c: char) -> Self {
        let message = format!("'{}':\t{:?}\n", c, input);
        println!("{}", message);
        Self { message }
    }

    fn or(self, other: Self) -> Self {
        let message = format!("{}\tOR\n{}\n", self.message, other.message);
        println!("{}", message);
        Self { message }
    }
}

impl ContextError<&str> for DebugError {
    fn add_context(input: &str, ctx: &'static str, other: Self) -> Self {
        let message = format!("{}\"{}\":\t{:?}\n", other.message, ctx, input);
        println!("{}", message);
        Self { message }
    }
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

pub fn pascal(i: &str) -> IResult<&str, String> {
    let (i, s) = many0(alt((
        map(alphanumeric1, |s: &str| {
            let (s1, s2) = s.split_at(1);
            s1.to_uppercase() + s2
        }),
        map(anychar, |c| {
            if c != '-' && c != '_' && c != ' ' {
                c.to_string()
            } else {
                "".to_string()
            }
        }),
    )))(i)?;
    let s = s.iter().fold(String::new(), |acc, x| acc + x);
    Ok((i, s))
}
