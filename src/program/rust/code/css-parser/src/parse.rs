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
