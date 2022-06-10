use crate::properties::CssCodec;

use super::{Float, ToComputedValue};
use crate::utils::nom::*;

#[derive(Debug, Default, PartialEq)]
pub struct Position {
    pub x: Float,
    pub y: Float,
}

impl CssCodec for Position {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(
            tuple((
                multispace0,
                nom_char('('),
                multispace0,
                digit1,
                multispace0,
                nom_char(','),
                multispace0,
                digit1,
                multispace0,
                nom_char(')'),
            )),
            |(_, _, _, x, _, _, _, y, _, _)| {
                let x = str::parse::<Float>(x).unwrap();
                let y = str::parse::<Float>(y).unwrap();
                Self { x, y }
            },
        )(i)
    }

    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        dest.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}

impl ToComputedValue for Position {
    type ComputedValue = Float;

    fn to_computed_value(&self) -> Self::ComputedValue {
        self.x
    }
}
