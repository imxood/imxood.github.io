use crate::parser::*;
use crate::properties::*;

use super::flex::*;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Display {
    inline: bool,
    layout: Layout,
}

impl CssProp for Display {
    fn rule() -> CssRule {
        CssRule::display
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let mut inline = false;
        for pair in pair.into_inner() {
            match pair.as_rule() {
                CssRule::inline => {
                    inline = true;
                }
                CssRule::flex_layout => {
                    return Self {
                        inline,
                        layout: Layout::FlexLayout(FlexLayout::parse(pair)),
                    };
                }
                _ => {}
            }
        }
        Self::default()
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        if self.inline {
            dest.write_str("inline ")?;
        }
        match &self.layout {
            Layout::FlexLayout(layout) => layout.to_css(dest)?,
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Layout {
    FlexLayout(FlexLayout),
}

impl Default for Layout {
    fn default() -> Self {
        Self::FlexLayout(FlexLayout::default())
    }
}

#[test]
fn test_display() {
    let display = Display::parse_str(
        "inline flex(flow: row wrap; align_items: center; justify_content: end;)",
    );
    assert_eq!(
        display,
        Ok(Display {
            inline: true,
            layout: Layout::FlexLayout(FlexLayout {
                flow: Flow {
                    direction: Direction::Row,
                    wrap_type: WrapType::Wrap
                },
                align_items: AlignItems(Align::Center),
                justify_content: JustifyContent(Align::End),
                ..Default::default()
            })
        })
    );
}
