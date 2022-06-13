use super::{color::Color, number::*, CssProp};
use crate::parser::*;
use derive_more::Constructor;

#[derive(Debug, Constructor, PartialEq)]
pub struct Width(LengthPercentage);

impl CssProp for Width {
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pairs) = CssParser::parse(CssRule::width, i).ok() {
            Self::parse(pairs.last().unwrap())
        } else {
            None
        }
    }

    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        for pair in pair.into_inner() {
            return LengthPercentage::parse(pair).map(|v| Self(v));
        }
        None
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        self.0.to_css(dest)
    }
}

#[derive(Debug, Constructor, PartialEq)]
pub struct Height(LengthPercentage);

impl CssProp for Height {
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pairs) = CssParser::parse(CssRule::height, i).ok() {
            Self::parse(pairs.last().unwrap())
        } else {
            None
        }
    }

    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        for pair in pair.into_inner() {
            return LengthPercentage::parse(pair).map(|v| Self(v));
        }
        None
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        self.0.to_css(dest)
    }
}

pub struct Padding(LengthPercentage);

impl CssProp for Padding {
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pairs) = CssParser::parse(CssRule::padding, i).ok() {
            Self::parse(pairs.last().unwrap())
        } else {
            None
        }
    }

    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        LengthPercentage::parse(pair).map(|v| Self(v))
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        self.0.to_css(dest)
    }
}

pub struct Margin(LengthPercentage);

impl CssProp for Margin {
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pairs) = CssParser::parse(CssRule::margin, i).ok() {
            Self::parse(pairs.last().unwrap())
        } else {
            None
        }
    }

    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        LengthPercentage::parse(pair).map(|v| Self(v))
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        self.0.to_css(dest)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BorderStyle {
    None,
    Dotted,
    Dashed,
    Solid,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
}

impl Default for BorderStyle {
    fn default() -> Self {
        Self::None
    }
}

impl CssProp for BorderStyle {
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pairs) = CssParser::parse(CssRule::border_style, i).ok() {
            Self::parse(pairs.last().unwrap())
        } else {
            None
        }
    }

    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        for pair in pair.into_inner() {
            let v = match pair.as_rule() {
                CssRule::none => Some(Self::None),
                CssRule::dotted => Some(Self::Dotted),
                CssRule::dashed => Some(Self::Dashed),
                CssRule::solid => Some(Self::Solid),
                CssRule::double => Some(Self::Double),
                CssRule::groove => Some(Self::Groove),
                CssRule::ridge => Some(Self::Ridge),
                CssRule::inset => Some(Self::Inset),
                CssRule::outset => Some(Self::Outset),
                _ => None,
            };
            return v;
        }
        None
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        match self {
            BorderStyle::None => dest.write_str("none"),
            BorderStyle::Dotted => dest.write_str("dotted"),
            BorderStyle::Dashed => dest.write_str("dashed"),
            BorderStyle::Solid => dest.write_str("solid"),
            BorderStyle::Double => dest.write_str("double"),
            BorderStyle::Groove => dest.write_str("groove"),
            BorderStyle::Ridge => dest.write_str("ridge"),
            BorderStyle::Inset => dest.write_str("inset"),
            BorderStyle::Outset => dest.write_str("outset"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LineWidth {
    Length(Length),
    Thin,
    Medium,
    Thick,
}

impl Default for LineWidth {
    fn default() -> Self {
        Self::Thin
    }
}

impl CssProp for LineWidth {
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pairs) = CssParser::parse(CssRule::line_width, i).ok() {
            Self::parse(pairs.last().unwrap())
        } else {
            None
        }
    }

    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        for pair in pair.into_inner() {
            let v = match pair.as_rule() {
                CssRule::length => Length::parse(pair).map(|v| Self::Length(v)),
                CssRule::thin => Some(Self::Thin),
                CssRule::medium => Some(Self::Medium),
                CssRule::thick => Some(Self::Thick),
                _ => None,
            };
            return v;
        }
        None
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        match self {
            LineWidth::Length(v) => v.to_css(dest),
            LineWidth::Thin => dest.write_str("thin"),
            LineWidth::Medium => dest.write_str("medium"),
            LineWidth::Thick => dest.write_str("thick"),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct BorderWidth {
    top: LineWidth,
    right: LineWidth,
    bottom: LineWidth,
    left: LineWidth,
}

impl CssProp for BorderWidth {
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pairs) = CssParser::parse(CssRule::border_width, i).ok() {
            Self::parse(pairs.last().unwrap())
        } else {
            None
        }
    }

    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        let mut line_widths = pair
            .into_inner()
            .filter_map(|pair| LineWidth::parse(pair))
            .collect::<Vec<_>>();
        println!("line_widths: {:?}", &line_widths);
        let len = line_widths.len();
        match len {
            1 => {
                let v0 = line_widths.pop().unwrap();
                Some(Self {
                    top: v0,
                    right: v0,
                    bottom: v0,
                    left: v0,
                })
            }
            2 => {
                let v1 = line_widths.pop().unwrap();
                let v0 = line_widths.pop().unwrap();
                Some(Self {
                    top: v0,
                    right: v1,
                    bottom: v0,
                    left: v1,
                })
            }
            3 => {
                let v2 = line_widths.pop().unwrap();
                let v1 = line_widths.pop().unwrap();
                let v0 = line_widths.pop().unwrap();
                Some(Self {
                    top: v0,
                    right: v1,
                    bottom: v2,
                    left: v1,
                })
            }
            4 => {
                let v3 = line_widths.pop().unwrap();
                let v2 = line_widths.pop().unwrap();
                let v1 = line_widths.pop().unwrap();
                let v0 = line_widths.pop().unwrap();
                Some(Self {
                    top: v0,
                    right: v1,
                    bottom: v2,
                    left: v3,
                })
            }
            _ => None,
        }
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        self.top.to_css(dest)?;
        dest.write_char(' ')?;
        self.right.to_css(dest)?;
        dest.write_char(' ')?;
        self.bottom.to_css(dest)?;
        dest.write_char(' ')?;
        self.left.to_css(dest)?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Border {
    pub border_style: BorderStyle,
    pub border_width: BorderWidth,
    pub color: Color,
}

impl CssProp for Border {
    fn parse_str(i: &str) -> Option<Self> {
        let ret = CssParser::parse(CssRule::border, i);
        match ret {
            Ok(pairs) => Self::parse(pairs.last().unwrap()),
            Err(e) => {
                println!("error: {:?}", &e);
                panic!("error!")
            }
        }
    }

    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        let mut border = Border::default();
        for pair in pair.into_inner() {
            println!("rule: {:?}, str: {}", pair.as_rule(), pair.as_str());
            // border_style | border_width | color
            match pair.as_rule() {
                CssRule::border_style => BorderStyle::parse(pair).map(|v| {
                    border.border_style = v;
                }),
                CssRule::border_width => BorderWidth::parse(pair).map(|v| {
                    border.border_width = v;
                }),
                CssRule::color => Color::parse(pair).map(|v| {
                    border.color = v;
                }),
                _ => None,
            };
        }
        Some(border)
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        self.border_width.to_css(dest)?;
        dest.write_char(' ')?;
        self.border_style.to_css(dest)?;
        dest.write_char(' ')?;
        self.color.to_css(dest)?;
        Ok(())
    }
}

#[test]
fn test_width() {
    let v = Width::parse_str("90%");
    assert_eq!(
        v,
        Some(Width::new(LengthPercentage::Percentage(Percentage::new(
            90.0
        ))))
    );

    let v = Width::parse_str("100px");
    assert_eq!(
        v,
        Some(Width::new(LengthPercentage::Length(Length::Px(Px::new(
            100.0
        )))))
    );

    let v = Width::parse_str("100em");
    assert_eq!(
        v,
        Some(Width::new(LengthPercentage::Length(Length::Em(Em::new(
            100.0
        )))))
    );
}

#[test]
fn test_height() {
    let v = Height::parse_str("90%");
    assert_eq!(
        v,
        Some(Height::new(LengthPercentage::Percentage(Percentage::new(
            90.0
        ))))
    );

    let v = Height::parse_str("100px");
    assert_eq!(
        v,
        Some(Height::new(LengthPercentage::Length(Length::Px(Px::new(
            100.0
        )))))
    );

    let v = Height::parse_str("100em");
    assert_eq!(
        v,
        Some(Height::new(LengthPercentage::Length(Length::Em(Em::new(
            100.0
        )))))
    );
}

#[test]
fn test_border_width() {
    let border_width = BorderWidth::parse_str("100px thin 300em").unwrap();
    let border_width_css = border_width.to_css_string();
    println!("border_width_css: {:?}", &border_width_css);
    assert_eq!(&border_width_css, "100px thin 300em thin");
}

#[test]
fn test_border_style() {
    let border_style = BorderStyle::parse_str("solid").unwrap();
    let border_style_css = border_style.to_css_string();
    println!("border_style_css: {:?}", &border_style_css);
    assert_eq!(&border_style_css, "solid");
}

#[test]
fn test_border() {
    let border = Border::parse_str("solid");
    assert_eq!(
        border,
        Some(Border {
            border_style: BorderStyle::Solid,
            ..Default::default()
        })
    );

    let border = Border::parse_str("100rem 200px thin 300em solid");
    assert_eq!(
        border,
        Some(Border {
            border_style: BorderStyle::Solid,
            border_width: BorderWidth {
                top: LineWidth::Length(Length::Rem(Rem::new(100.0))),
                right: LineWidth::Length(Length::Px(Px::new(200.0))),
                bottom: LineWidth::Thin,
                left: LineWidth::Length(Length::Em(Em::new(300.0))),
            },
            ..Default::default()
        })
    );
}
