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
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pairs) = CssParser::parse(CssRule::color, i).ok() {
            Self::parse(pairs.last().unwrap())
        } else {
            None
        }
    }

    fn parse(pair: Pair<Rule>) -> Option<Self> {
        for pair in pair.into_inner() {
            let v = match pair.as_rule() {
                CssRule::rgb => Rgb::parse(pair).map(|v| Self::Rgb(v)),
                CssRule::rgba => Rgba::parse(pair).map(|v| Self::Rgba(v)),
                CssRule::hex => Hex::parse(pair).map(|v| Self::Hex(v)),
                _ => None,
            };
            return v;
        }
        None
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
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pairs) = CssParser::parse(CssRule::rgb, i).ok() {
            Self::parse(pairs.last().unwrap())
        } else {
            None
        }
    }

    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        let colors = pair
            .into_inner()
            .filter_map(|v| u8::parse(v))
            .collect::<Vec<_>>();
        if colors.len() != 3 {
            return None;
        }
        Some(Self {
            r: colors[0],
            g: colors[1],
            b: colors[2],
        })
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
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pairs) = CssParser::parse(CssRule::rgba, i).ok() {
            Self::parse(pairs.last().unwrap())
        } else {
            None
        }
    }

    fn parse(pair: pest::iterators::Pair<crate::parser::CssRule>) -> Option<Self> {
        let mut pairs = pair.into_inner().into_iter();
        let pair0 = pairs.next();
        let pair1 = pairs.next();
        let pair2 = pairs.next();
        let pair3 = pairs.next();
        if pair0.is_none() || pair1.is_none() || pair2.is_none() || pair3.is_none() {
            return None;
        }

        let r = u8::parse(pair0.unwrap());
        let g = u8::parse(pair1.unwrap());
        let b = u8::parse(pair2.unwrap());
        let a = Float::parse(pair3.unwrap());

        if r.is_none() || g.is_none() || b.is_none() || a.is_none() {
            return None;
        }

        Some(Self {
            r: r.unwrap(),
            g: g.unwrap(),
            b: b.unwrap(),
            a: a.unwrap(),
        })
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
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pairs) = CssParser::parse(CssRule::hex, i).ok() {
            Self::parse(pairs.last().unwrap())
        } else {
            None
        }
    }

    fn parse(pair: pest::iterators::Pair<crate::parser::CssRule>) -> Option<Self> {
        let colors = pair
            .into_inner()
            .filter_map(|v| HexU8::parse(v))
            .collect::<Vec<_>>();
        if colors.len() != 3 {
            return None;
        }
        Some(Self {
            r: colors[0].0,
            g: colors[1].0,
            b: colors[2].0,
        })
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        dest.write_fmt(format_args!("#{}{}{})", self.r, self.g, self.b))
    }
}

#[test]
fn test_rgb() {
    let rgb = Rgb::parse_str("rgb(0, 255, 230)");
    assert_eq!(rgb, Some(Rgb::new(0, 255, 230)));
}

#[test]
fn test_rgba() {
    let rgba = Rgba::parse_str("rgba(255, 0, 100, 0.1)");
    assert_eq!(rgba, Some(Rgba::new(255, 0, 100, 0.1)));
}

#[test]
fn test_hex() {
    let hex = Hex::parse_str("#ff00aa");
    assert_eq!(hex, Some(Hex::new(255, 0, 0xaa)));
}

#[test]
fn test_color() {
    let color = Color::parse_str("rgb(0, 255, 230)");
    assert_eq!(color, Some(Color::Rgb(Rgb::new(0, 255, 230))));

    let color = Color::parse_str("rgba(100, 255, 230, 0.1)");
    assert_eq!(color, Some(Color::Rgba(Rgba::new(100, 255, 230, 0.1))));

    let color = Color::parse_str("#ff00aa");
    assert_eq!(color, Some(Color::Hex(Hex::new(255, 0, 0xaa))));
}
