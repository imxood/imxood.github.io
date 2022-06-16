use super::{CssProp, Float};
use crate::parser::*;
use crate::properties::*;
use derive_more::Constructor;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    Rgb(Rgb),
    Rgba(Rgba),
    Hex(Hex),
}

impl CssProp for Color {
    fn rule() -> CssRule {
        CssRule::color
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            CssRule::rgb => Self::Rgb(Rgb::parse(pair)),
            CssRule::rgba => Self::Rgba(Rgba::parse(pair)),
            CssRule::hex => Self::Hex(Hex::parse(pair)),
            _ => unreachable!(),
        }
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        match self {
            Color::Rgb(v) => v.to_css(dest),
            Color::Rgba(v) => v.to_css(dest),
            Color::Hex(v) => v.to_css(dest),
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::Rgb(Rgb::default())
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy, Constructor)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl CssProp for Rgb {
    fn rule() -> CssRule {
        CssRule::rgb
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let mut pairs = pair.into_inner();
        let (pair0, pair1, pair2) = (
            pairs.next().unwrap(),
            pairs.next().unwrap(),
            pairs.next().unwrap(),
        );
        Self {
            r: u8::parse(pair0),
            g: u8::parse(pair1),
            b: u8::parse(pair2),
        }
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        dest.write_fmt(format_args!("rgb({}, {}, {})", self.r, self.g, self.b))
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy, Constructor)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: Float,
}

impl CssProp for Rgba {
    fn rule() -> CssRule {
        CssRule::rgba
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let mut pairs = pair.into_inner();
        let (pair0, pair1, pair2, pair3) = (
            pairs.next().unwrap(),
            pairs.next().unwrap(),
            pairs.next().unwrap(),
            pairs.next().unwrap(),
        );

        Self {
            r: u8::parse(pair0),
            g: u8::parse(pair1),
            b: u8::parse(pair2),
            a: Float::parse(pair3),
        }
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        dest.write_fmt(format_args!(
            "rgba({}, {}, {}, {})",
            self.r, self.g, self.b, self.a
        ))
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy, Constructor)]
pub struct Hex {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl CssProp for Hex {
    fn rule() -> CssRule {
        CssRule::hex
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let mut pairs = pair.into_inner();
        let (pair0, pair1, pair2) = (
            pairs.next().unwrap(),
            pairs.next().unwrap(),
            pairs.next().unwrap(),
        );
        Self {
            r: HexU8::parse(pair0).0,
            g: HexU8::parse(pair1).0,
            b: HexU8::parse(pair2).0,
        }
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        dest.write_fmt(format_args!("#{}{}{})", self.r, self.g, self.b))
    }
}

#[test]
fn test_rgb() {
    let rgb = Rgb::parse_str("rgb(0, 255, 230)");
    assert_eq!(rgb, Ok(Rgb::new(0, 255, 230)));
}

#[test]
fn test_rgba() {
    let rgba = Rgba::parse_str("rgba(255, 0, 100, 0.1)");
    assert_eq!(rgba, Ok(Rgba::new(255, 0, 100, 0.1)));
}

#[test]
fn test_hex() {
    let hex = Hex::parse_str("#ff00aa");
    assert_eq!(hex, Ok(Hex::new(255, 0, 0xaa)));
}

#[test]
fn test_color() {
    let color = Color::parse_str("rgb(0, 255, 230)");
    assert_eq!(color, Ok(Color::Rgb(Rgb::new(0, 255, 230))));

    let color = Color::parse_str("rgba(100, 255, 230, 0.1)");
    assert_eq!(color, Ok(Color::Rgba(Rgba::new(100, 255, 230, 0.1))));

    let color = Color::parse_str("#ff00aa");
    assert_eq!(color, Ok(Color::Hex(Hex::new(255, 0, 0xaa))));
}
