use super::color::Color;
use super::serialize::ToCss;
use crate::display::{flex, grid};
use crate::parse::{nom_char, nom_u8, skip_sp, Parser};
use crate::types::Float;

use nom::branch::{alt, permutation};
use nom::bytes::complete::{is_not, tag, take_till};
use nom::combinator::map;
use nom::sequence::{separated_pair, terminated};
use nom::IResult;

pub const PROPERTY_COUNT: usize = 3;

pub struct PropertiesBuilder {
    pub background: Option<Background>,
    pub height: Option<Height>,
    pub width: Option<Width>,
}

impl PropertiesBuilder {
    pub fn background(mut self, v: Background) -> Self {
        self.background = Some(v);
        self
    }
    pub fn height(mut self, v: Height) -> Self {
        self.height = Some(v);
        self
    }
    pub fn width(mut self, v: Width) -> Self {
        self.width = Some(v);
        self
    }
}

pub struct ComputedProperties {
    pub background: Background,
    pub height: Height,
    pub width: Width,
}

#[derive(Debug)]
pub enum Property {
    Background(Background),
    Height(Height),
    Width(Width),
}

impl Property {
    pub fn name(&self) -> &str {
        match *self {
            Self::Background(_) => "background",
            Self::Height(_) => "height",
            Self::Width(_) => "width",
        }
    }
}

impl ToCss for Property {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        match self {
            Self::Background(v) => v.to_css(dest),
            Self::Height(v) => v.to_css(dest),
            Self::Width(v) => v.to_css(dest),
        }
    }
}

impl Parser for Property {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            terminated(
                separated_pair(skip_sp(is_not(":;}")), tag(":"), skip_sp(is_not(";}"))),
                alt((tag(";"), tag("}"))),
            ),
            |(k, v)| match k {
                "background" => {
                    let (_, background) = Background::parse(v).unwrap();
                    Self::Background(background)
                }
                "height" => {
                    let (_, height) = Height::parse(v).unwrap();
                    Self::Height(height)
                }
                "width" => {
                    let (_, width) = Width::parse(v).unwrap();
                    Self::Width(width)
                }

                _ => panic!("Except!!!"),
            },
        )(input)
    }
}

#[derive(Debug)]
pub struct Background {
    pub background_color: Color,
    pub background_position_x: Float,
    pub background_position_y: Float,
}

impl Parser for Background {
    fn parse(i: &str) -> IResult<&str, Self> {
        let (_, background_color) = map(
            permutation((
                tag("background-color:"),
                take_till(|c| c == ';'),
                Color::parse,
            )),
            |(_, _, background_color)| background_color,
        )(i)
        .unwrap();
        let (_, background_position_x) = map(
            permutation((
                tag("background-position-x:"),
                take_till(|c| c == ';'),
                Float::parse,
            )),
            |(_, _, background_position_x)| background_position_x,
        )(i)
        .unwrap();
        let (_, background_position_y) = map(
            permutation((
                tag("background-position-y:"),
                take_till(|c| c == ';'),
                Float::parse,
            )),
            |(_, _, background_position_y)| background_position_y,
        )(i)
        .unwrap();

        Ok((
            "",
            Self {
                background_color,
                background_position_x,
                background_position_y,
            },
        ))
    }
}

impl ToCss for Background {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        self.background_color.to_css(dest)?;
        dest.write_char(' ')?;
        self.background_position_x.to_css(dest)?;
        dest.write_char(' ')?;
        self.background_position_y.to_css(dest)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Height(pub Float);

impl Parser for Height {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(Float::parse, |height| Self(height))(i)
    }
}

impl ToCss for Height {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        self.0.to_css(dest)
    }
}

#[derive(Debug)]
pub struct Width(pub Float);

impl Parser for Width {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(Float::parse, |width| Self(width))(i)
    }
}

impl ToCss for Width {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        self.0.to_css(dest)
    }
}
