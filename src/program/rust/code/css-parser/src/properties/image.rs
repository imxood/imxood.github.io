use super::ToComputedValue;
use crate::properties::CssCodec;
use crate::utils::nom::*;

#[derive(Debug, Default, PartialEq)]
pub struct Image {
    pub url: String,
}

impl CssCodec for Image {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(
            tuple((
                multispace0,
                tag("url("),
                multispace0,
                take_till1(|c: char| c == ')'),
                nom_char(')'),
            )),
            |(_, _, _, url, _): (_, _, _, &str, _)| {
                let url = url.to_string();
                Self { url }
            },
        )(i)
    }

    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        dest.write_fmt(format_args!("url({})", &self.url))
    }
}

impl ToComputedValue for Image {
    type ComputedValue = String;

    fn to_computed_value(&self) -> Self::ComputedValue {
        self.url.clone()
    }
}
