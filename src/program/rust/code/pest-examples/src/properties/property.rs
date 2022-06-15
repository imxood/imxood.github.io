use super::{box_model::*, display::*};
use crate::parser::*;
use crate::properties::*;
use derive_more::Constructor;

#[derive(Debug, PartialEq, Clone)]
pub enum Property {
    /* box model */
    Width(Width),
    Height(Height),
    Padding(Padding),
    Margin(Margin),
    Border(Border),
    /* layout */
    Display(Display),
    Flex(Flex),
}

#[derive(Debug, Default, PartialEq, Clone, Constructor)]
pub struct Properties(pub Vec<Property>);

impl CssProp for Properties {
    fn parse_str(i: &str) -> Option<Self> {
        match CssParser::parse(CssRule::properties, i) {
            Ok(pairs) => Self::parse(pairs.last().unwrap()),
            Err(e) => {
                panic!("error: {:?}", &e);
            }
        }
    }

    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        let mut properties = Self::default();
        for pair in pair.into_inner() {
            match pair.as_rule() {
                /* box model */
                CssRule::width => {
                    Width::parse(pair).map(|v| {
                        properties.0.push(Property::Width(v));
                    });
                }
                CssRule::height => {
                    Height::parse(pair).map(|v| {
                        properties.0.push(Property::Height(v));
                    });
                }
                CssRule::margin => {
                    Margin::parse(pair).map(|v| {
                        properties.0.push(Property::Margin(v));
                    });
                }
                CssRule::padding => {
                    Padding::parse(pair).map(|v| {
                        properties.0.push(Property::Padding(v));
                    });
                }
                CssRule::border => {
                    Border::parse(pair).map(|v| {
                        properties.0.push(Property::Border(v));
                    });
                }
                /* layout */
                CssRule::display => {
                    Display::parse(pair).map(|v| {
                        properties.0.push(Property::Display(v));
                    });
                }
                CssRule::flex => {
                    Flex::parse(pair).map(|v| {
                        properties.0.push(Property::Flex(v));
                    });
                }
                _ => {}
            }
        }
        Some(properties)
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        dest.write_str("{ ")?;
        let len = self.0.len();
        for (i, property) in self.0.iter().enumerate() {
            match property {
                Property::Width(width) => {
                    dest.write_str("width: ")?;
                    width.to_css(dest)?;
                }
                Property::Height(height) => {
                    dest.write_str("height: ")?;
                    height.to_css(dest)?;
                }
                Property::Padding(padding) => {
                    dest.write_str("padding: ")?;
                    padding.to_css(dest)?;
                }
                Property::Margin(margin) => {
                    dest.write_str("margin: ")?;
                    margin.to_css(dest)?;
                }
                Property::Border(border) => {
                    dest.write_str("border: ")?;
                    border.to_css(dest)?;
                }
                Property::Display(display) => {
                    dest.write_str("display: ")?;
                    display.to_css(dest)?;
                }
                Property::Flex(flex) => {
                    dest.write_str("flex: ")?;
                    flex.to_css(dest)?;
                }
            }

            dest.write_char(';')?;
            if i + 1 != len {
                dest.write_char(' ')?;
            }
        }
        dest.write_str(" }")?;
        Ok(())
    }
}

#[test]
fn test_properties() {
    let properties = Properties::parse_str("{width: 100px; height: 200em}");
    assert_eq!(
        properties,
        Some(Properties::new(vec![
            Property::Width(Width::new(LengthPercentage::Length(Length::Px(Px::new(
                100.0
            ))))),
            Property::Height(Height::new(LengthPercentage::Length(Length::Em(Em::new(
                200.0
            )))))
        ]))
    )
}
