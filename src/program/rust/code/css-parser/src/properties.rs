use nom::branch::alt;
use nom::bytes::complete::{is_not, take_till1};
use nom::combinator::{map, opt};
use nom::sequence::{pair, separated_pair};
use nom::IResult;

use crate::{
    color::Color,
    parse::{nom_char, skip_useless, Parser},
    serialize::ToCss,
    types::{Image, Length, Percentage, Position},
};

#[derive(Debug, PartialEq)]
pub enum Property {
    Width(Width),
    Height(Height),
    Background(Background),
    BackgroundColor(BackgroundColor),
    BackgroundImage(BackgroundImage),
    BackgroundPosition(BackgroundPosition),
    GridTemplateColumns(GridTemplateColumns),
}

impl Parser for Property {
    fn parse(i: &str) -> IResult<&str, Self> {
        let (i, i0) = is_not(";}")(i)?;
        let (_, (name, value)) = separated_pair(
            skip_useless(take_till1(|c: char| !c.is_alphanumeric() && c != '-')),
            skip_useless(nom_char(':')),
            skip_useless(take_till1(|c: char| c == ';')),
        )(i0)?;
        let (_, property) = match name {
            "width" => map(Width::parse, |width| Self::Width(width))(value),
            "height" => map(Height::parse, |height| Self::Height(height))(value),
            "background" => {
                map(Background::parse, |background| Self::Background(background))(value)
            }
            "background-color" => map(BackgroundColor::parse, |background_color| {
                Self::BackgroundColor(background_color)
            })(value),
            "background-image" => map(BackgroundImage::parse, |background_image| {
                Self::BackgroundImage(background_image)
            })(value),
            "background-position" => map(BackgroundPosition::parse, |background_position| {
                Self::BackgroundPosition(background_position)
            })(value),
            "grid-template-columns" => map(GridTemplateColumns::parse, |grid_template_columns| {
                Self::GridTemplateColumns(grid_template_columns)
            })(value),

            _ => panic!("解析属性失败, property name: {}", name),
        }?;
        Ok((i, property))
    }
}

impl ToCss for Property {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        match self {
            Self::Width(width) => {
                dest.write_str("width: ")?;
                width.to_css(dest)?;
                dest.write_char(';')
            }
            Self::Height(height) => {
                dest.write_str("height: ")?;
                height.to_css(dest)?;
                dest.write_char(';')
            }
            Self::Background(background) => {
                dest.write_str("background: ")?;
                background.to_css(dest)?;
                dest.write_char(';')
            }
            Self::BackgroundColor(background_color) => {
                dest.write_str("background-color: ")?;
                background_color.to_css(dest)?;
                dest.write_char(';')
            }
            Self::BackgroundImage(background_image) => {
                dest.write_str("background-image: ")?;
                background_image.to_css(dest)?;
                dest.write_char(';')
            }
            Self::BackgroundPosition(background_position) => {
                dest.write_str("background-position: ")?;
                background_position.to_css(dest)?;
                dest.write_char(';')
            }
            Self::GridTemplateColumns(grid_template_columns) => {
                dest.write_str("grid-template-columns: ")?;
                grid_template_columns.to_css(dest)?;
                dest.write_char(';')
            }
        }
    }
}

pub fn width(i: &str) -> Option<Width> {
    let (_, v) = opt(Width::parse)(i).unwrap_or_default();
    v
}

pub fn height(i: &str) -> Option<Height> {
    let (_, v) = opt(Height::parse)(i).unwrap_or_default();
    v
}

pub fn background(i: &str) -> Option<Background> {
    let (_, v) = opt(Background::parse)(i).unwrap_or_default();
    v
}

pub fn background_color(i: &str) -> Option<BackgroundColor> {
    let (_, v) = opt(BackgroundColor::parse)(i).unwrap_or_default();
    v
}

pub fn background_image(i: &str) -> Option<BackgroundImage> {
    let (_, v) = opt(BackgroundImage::parse)(i).unwrap_or_default();
    v
}

pub fn background_position(i: &str) -> Option<BackgroundPosition> {
    let (_, v) = opt(BackgroundPosition::parse)(i).unwrap_or_default();
    v
}

pub fn grid_template_columns(i: &str) -> Option<GridTemplateColumns> {
    let (_, v) = opt(GridTemplateColumns::parse)(i).unwrap_or_default();
    v
}

#[derive(Debug, PartialEq)]
pub enum Width {
    Length(Length),
    Percentage(Percentage),
}

impl Parser for Width {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((
            map(Length::parse, |length| Self::Length(length)),
            map(Percentage::parse, |percentage| Self::Percentage(percentage)),
        ))(i)
    }
}

impl ToCss for Width {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        match self {
            Self::Length(v) => v.to_css(dest),
            Self::Percentage(v) => v.to_css(dest),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Height {
    Length(Length),
    Percentage(Percentage),
}

impl Parser for Height {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((
            map(Length::parse, |length| Self::Length(length)),
            map(Percentage::parse, |percentage| Self::Percentage(percentage)),
        ))(i)
    }
}

impl ToCss for Height {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        match self {
            Self::Length(v) => v.to_css(dest),
            Self::Percentage(v) => v.to_css(dest),
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Background {
    pub background_color: BackgroundColor,
    pub background_image: BackgroundImage,
    pub background_position: BackgroundPosition,
}

impl Parser for Background {
    fn parse(i: &str) -> IResult<&str, Self> {
        let mut v = Self::default();
        let (i, _) = pair(
            alt((
                map(BackgroundColor::parse, |background_color| {
                    v.background_color = background_color;
                }),
                map(BackgroundImage::parse, |background_image| {
                    v.background_image = background_image;
                }),
            )),
            map(
                opt(map(BackgroundPosition::parse, |background_position| {
                    v.background_position = background_position;
                })),
                |_| {},
            ),
        )(i)?;
        Ok((i, v))
    }
}

impl ToCss for Background {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        self.background_color.to_css(dest)?;
        dest.write_char(' ')?;

        self.background_image.to_css(dest)?;
        dest.write_char(' ')?;

        self.background_position.to_css(dest)?;

        Ok(())
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct BackgroundColor {
    pub color: Color,
}

impl Parser for BackgroundColor {
    fn parse(i: &str) -> IResult<&str, Self> {
        let mut v = Self::default();
        let (i, _) = map(Color::parse, |color| {
            v.color = color;
        })(i)?;
        Ok((i, v))
    }
}

impl ToCss for BackgroundColor {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        self.color.to_css(dest)?;

        Ok(())
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct BackgroundImage {
    pub image: Image,
}

impl Parser for BackgroundImage {
    fn parse(i: &str) -> IResult<&str, Self> {
        let mut v = Self::default();
        let (i, _) = map(Image::parse, |image| {
            v.image = image;
        })(i)?;
        Ok((i, v))
    }
}

impl ToCss for BackgroundImage {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        self.image.to_css(dest)?;

        Ok(())
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct BackgroundPosition {
    pub position: Position,
}

impl Parser for BackgroundPosition {
    fn parse(i: &str) -> IResult<&str, Self> {
        let mut v = Self::default();
        let (i, _) = map(Position::parse, |position| {
            v.position = position;
        })(i)?;
        Ok((i, v))
    }
}

impl ToCss for BackgroundPosition {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        self.position.to_css(dest)?;

        Ok(())
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct GridTemplateColumns {
    pub position: Position,
}

impl Parser for GridTemplateColumns {
    fn parse(i: &str) -> IResult<&str, Self> {
        let mut v = Self::default();
        let (i, _) = map(Position::parse, |position| {
            v.position = position;
        })(i)?;
        Ok((i, v))
    }
}

impl ToCss for GridTemplateColumns {
    fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
    where
        W: core::fmt::Write,
    {
        self.position.to_css(dest)?;

        Ok(())
    }
}
