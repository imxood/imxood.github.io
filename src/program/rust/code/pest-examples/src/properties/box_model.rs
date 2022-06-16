use super::{color::Color, number::*, CssProp};
use crate::parser::*;
use derive_more::Constructor;

#[derive(Debug, Constructor, PartialEq, Clone, Copy)]
pub struct Width(LengthPercentage);

impl CssProp for Width {
    fn rule() -> CssRule {
        CssRule::width
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let pair = pair.into_inner().next().unwrap();
        Self(LengthPercentage::parse(pair))
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        self.0.to_css(dest)
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Constructor)]
pub struct Height(LengthPercentage);

impl CssProp for Height {
    fn rule() -> CssRule {
        CssRule::height
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let pair = pair.into_inner().next().unwrap();
        Self(LengthPercentage::parse(pair))
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        self.0.to_css(dest)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Padding(LengthPercentage);

impl CssProp for Padding {
    fn rule() -> CssRule {
        CssRule::padding
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let pair = pair.into_inner().next().unwrap();
        Self(LengthPercentage::parse(pair))
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        self.0.to_css(dest)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Margin(LengthPercentage);

impl CssProp for Margin {
    fn rule() -> CssRule {
        CssRule::margin
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let pair = pair.into_inner().next().unwrap();
        Self(LengthPercentage::parse(pair))
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
    fn rule() -> CssRule {
        CssRule::border_style
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            CssRule::none => Self::None,
            CssRule::dotted => Self::Dotted,
            CssRule::dashed => Self::Dashed,
            CssRule::solid => Self::Solid,
            CssRule::double => Self::Double,
            CssRule::groove => Self::Groove,
            CssRule::ridge => Self::Ridge,
            CssRule::inset => Self::Inset,
            CssRule::outset => Self::Outset,
            _ => unreachable!(),
        }
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
    fn rule() -> CssRule {
        CssRule::line_width
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            CssRule::length => Self::Length(Length::parse(pair)),
            CssRule::thin => Self::Thin,
            CssRule::medium => Self::Medium,
            CssRule::thick => Self::Thick,
            _ => unreachable!(),
        }
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
    fn rule() -> CssRule {
        CssRule::border_width
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let mut line_widths = pair
            .into_inner()
            .map(|pair| LineWidth::parse(pair))
            .collect::<Vec<_>>();
        let len = line_widths.len();
        match len {
            1 => {
                let v0 = line_widths.pop().unwrap();
                Self {
                    top: v0,
                    right: v0,
                    bottom: v0,
                    left: v0,
                }
            }
            2 => {
                let v1 = line_widths.pop().unwrap();
                let v0 = line_widths.pop().unwrap();
                Self {
                    top: v0,
                    right: v1,
                    bottom: v0,
                    left: v1,
                }
            }
            3 => {
                let v2 = line_widths.pop().unwrap();
                let v1 = line_widths.pop().unwrap();
                let v0 = line_widths.pop().unwrap();
                Self {
                    top: v0,
                    right: v1,
                    bottom: v2,
                    left: v1,
                }
            }
            4 => {
                let v3 = line_widths.pop().unwrap();
                let v2 = line_widths.pop().unwrap();
                let v1 = line_widths.pop().unwrap();
                let v0 = line_widths.pop().unwrap();
                Self {
                    top: v0,
                    right: v1,
                    bottom: v2,
                    left: v3,
                }
            }
            _ => unreachable!(),
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
    fn rule() -> CssRule {
        CssRule::border
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let mut border = Border::default();
        for pair in pair.into_inner() {
            match pair.as_rule() {
                CssRule::border_style => {
                    border.border_style = BorderStyle::parse(pair);
                }
                CssRule::border_width => {
                    border.border_width = BorderWidth::parse(pair);
                }
                CssRule::color => {
                    border.color = Color::parse(pair);
                }
                _ => unreachable!(),
            };
        }
        border
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
        Ok(Width::new(LengthPercentage::Percentage(Percentage::new(
            90.0
        ))))
    );

    let v = Width::parse_str("100px");
    assert_eq!(
        v,
        Ok(Width::new(LengthPercentage::Length(Length::Px(Px::new(
            100.0
        )))))
    );

    let v = Width::parse_str("100em");
    assert_eq!(
        v,
        Ok(Width::new(LengthPercentage::Length(Length::Em(Em::new(
            100.0
        )))))
    );
}

#[test]
fn test_height() {
    let v = Height::parse_str("90%");
    assert_eq!(
        v,
        Ok(Height::new(LengthPercentage::Percentage(Percentage::new(
            90.0
        ))))
    );

    let v = Height::parse_str("100px");
    assert_eq!(
        v,
        Ok(Height::new(LengthPercentage::Length(Length::Px(Px::new(
            100.0
        )))))
    );

    let v = Height::parse_str("100em");
    assert_eq!(
        v,
        Ok(Height::new(LengthPercentage::Length(Length::Em(Em::new(
            100.0
        )))))
    );
}

#[test]
fn test_border_width() {
    let border_width = BorderWidth::parse_str("100px thin 300em");
    assert_eq!(
        border_width,
        Ok(BorderWidth {
            top: LineWidth::Length(Length::Px(Px::new(100.0))),
            right: LineWidth::Thin,
            bottom: LineWidth::Length(Length::Em(Em::new(300.0))),
            left: LineWidth::Thin,
        })
    );
}

#[test]
fn test_border_style() {
    let border_style = BorderStyle::parse_str("solid");
    assert_eq!(border_style, Ok(BorderStyle::Solid));
}

#[test]
fn test_border() {
    let border = Border::parse_str("solid");
    assert_eq!(
        border,
        Ok(Border {
            border_style: BorderStyle::Solid,
            ..Default::default()
        })
    );

    let border = Border::parse_str("100rem 200px thin 300em solid");
    assert_eq!(
        border,
        Ok(Border {
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
