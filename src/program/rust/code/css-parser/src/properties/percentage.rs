use super::{ToComputedValue, Float};
use crate::properties::CssCodec;
use crate::utils::nom::*;

#[derive(Debug, PartialEq)]
pub struct Percentage(Float);

impl ToComputedValue for Percentage {
    type ComputedValue = Float;

    fn to_computed_value(&self) -> Self::ComputedValue {
        self.0
    }
}

impl CssCodec for Percentage {
    fn parse(i: &str) -> IResult<&str, Self> {
        let (i, num) = preceded(multispace0, digit1)(i)?;
        let num = str::parse::<Float>(num).unwrap();
        let _ = nom_char('%')(i)?;
        Ok((i, Self(num)))
    }

    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        dest.write_fmt(format_args!("{}%", self.0))
    }
}
