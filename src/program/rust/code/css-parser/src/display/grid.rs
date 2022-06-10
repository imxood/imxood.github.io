use crate::serialize::CssCodec;

#[derive(Debug)]
pub struct GridTemplateColumns;

impl CssCodec for GridTemplateColumns {
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
pub struct GridTemplateRows;

impl CssCodec for GridTemplateRows {
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
pub struct AlignItems;

impl CssCodec for AlignItems {
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
pub struct AlignContent;

impl CssCodec for AlignContent {
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
pub struct JustifyContent;

impl CssCodec for JustifyContent {
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
