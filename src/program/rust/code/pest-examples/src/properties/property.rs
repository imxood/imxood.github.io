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
    fn rule() -> CssRule {
        CssRule::properties
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let mut properties = Self::default();
        for pair in pair.into_inner() {
            match pair.as_rule() {
                /* box model */
                CssRule::width => {
                    properties.0.push(Property::Width(Width::parse(pair)));
                }
                CssRule::height => {
                    properties.0.push(Property::Height(Height::parse(pair)));
                }
                CssRule::margin => {
                    properties.0.push(Property::Margin(Margin::parse(pair)));
                }
                CssRule::padding => {
                    properties.0.push(Property::Padding(Padding::parse(pair)));
                }
                CssRule::border => {
                    properties.0.push(Property::Border(Border::parse(pair)));
                }
                /* layout */
                CssRule::display => {
                    properties.0.push(Property::Display(Display::parse(pair)));
                }
                CssRule::flex => {
                    properties.0.push(Property::Flex(Flex::parse(pair)));
                }
                _ => unreachable!(),
            }
        }
        properties
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
        Ok(Properties::new(vec![
            Property::Width(Width::new(LengthPercentage::Length(Length::Px(Px::new(
                100.0
            ))))),
            Property::Height(Height::new(LengthPercentage::Length(Length::Em(Em::new(
                200.0
            )))))
        ]))
    )
}
