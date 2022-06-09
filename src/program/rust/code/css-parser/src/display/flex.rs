use crate::serialize::CssCodec;

#[derive(Debug)]
pub struct FlexDirection;

impl CssCodec for FlexDirection {
    fn parse(i: &str) -> nom::IResult<&str, Self> {
        todo!()
    }

    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        todo!()
    }
}

#[derive(Debug)]
pub struct FlexWrap;

impl CssCodec for FlexWrap {
    fn parse(i: &str) -> nom::IResult<&str, Self> {
        todo!()
    }

    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        todo!()
    }
}

#[derive(Debug)]
pub struct FlexFlow;

impl CssCodec for FlexFlow {
    fn parse(i: &str) -> nom::IResult<&str, Self> {
        todo!()
    }

    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        todo!()
    }
}
