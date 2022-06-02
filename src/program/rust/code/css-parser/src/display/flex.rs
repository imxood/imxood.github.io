use crate::{parse::Parser, serialize::ToCss};

#[derive(Debug)]
pub struct FlexDirection;

impl Parser for FlexDirection {
    fn parse(i: &str) -> nom::IResult<&str, Self> {
        todo!()
    }
}
impl ToCss for FlexDirection {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        todo!()
    }
}

#[derive(Debug)]
pub struct FlexWrap;

impl Parser for FlexWrap {
    fn parse(i: &str) -> nom::IResult<&str, Self> {
        todo!()
    }
}
impl ToCss for FlexWrap {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        todo!()
    }
}

#[derive(Debug)]
pub struct FlexFlow;

impl Parser for FlexFlow {
    fn parse(i: &str) -> nom::IResult<&str, Self> {
        todo!()
    }
}
impl ToCss for FlexFlow {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        todo!()
    }
}