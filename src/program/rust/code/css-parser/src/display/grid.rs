use crate::{parse::Parser, serialize::ToCss};

#[derive(Debug)]
pub struct GridTemplateColumns;

impl Parser for GridTemplateColumns {
    fn parse(i: &str) -> nom::IResult<&str, Self> {
        todo!()
    }
}
impl ToCss for GridTemplateColumns {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        todo!()
    }
}

#[derive(Debug)]
pub struct GridTemplateRows;

impl Parser for GridTemplateRows {
    fn parse(i: &str) -> nom::IResult<&str, Self> {
        todo!()
    }
}
impl ToCss for GridTemplateRows {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        todo!()
    }
}

#[derive(Debug)]
pub struct AlignItems;

impl Parser for AlignItems {
    fn parse(i: &str) -> nom::IResult<&str, Self> {
        todo!()
    }
}
impl ToCss for AlignItems {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        todo!()
    }
}

#[derive(Debug)]
pub struct AlignContent;

impl Parser for AlignContent {
    fn parse(i: &str) -> nom::IResult<&str, Self> {
        todo!()
    }
}
impl ToCss for AlignContent {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        todo!()
    }
}

#[derive(Debug)]
pub struct JustifyContent;

impl Parser for JustifyContent {
    fn parse(i: &str) -> nom::IResult<&str, Self> {
        todo!()
    }
}
impl ToCss for JustifyContent {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        todo!()
    }
}
