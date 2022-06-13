use crate::parser::*;
use crate::properties::*;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct FlexLayout {
    pub main_flow: MainFlow,
    pub main_items: MainItems,
    pub main_content: MainContent,
    pub cross_items: CrossItems,
    pub cross_content: CrossContent,
}

impl CssProp for FlexLayout {
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pair) = CssParser::parse(CssRule::flex_layout, i).ok() {
            Self::parse(pair.last().unwrap())
        } else {
            None
        }
    }

    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        let pairs = pair.into_inner();
        let mut layout = Self::default();
        for pair in pairs {
            match pair.as_rule() {
                CssRule::main_flow => {
                    MainFlow::parse(pair).map(|v| {
                        layout.main_flow = v;
                    });
                }
                CssRule::main_items => {
                    MainItems::parse(pair).map(|v| {
                        layout.main_items = v;
                    });
                }
                CssRule::main_content => {
                    MainContent::parse(pair).map(|v| {
                        layout.main_content = v;
                    });
                }
                CssRule::cross_items => {
                    CrossItems::parse(pair).map(|v| {
                        layout.cross_items = v;
                    });
                }
                CssRule::cross_content => {
                    CrossContent::parse(pair).map(|v| {
                        layout.cross_content = v;
                    });
                }
                _ => {}
            }
        }
        Some(layout)
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        dest.write_fmt(format_args!(
            "flex(main_flow: {}; main_items: {}; main_content: {}; cross_items: {}; cross_content: {};)",
            self.main_flow.to_css_string(),
            self.main_items.to_css_string(),
            self.main_content.to_css_string(),
            self.cross_items.to_css_string(),
            self.cross_content.to_css_string(),
        ))
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct MainFlow {
    pub direction: Direction,
    pub wrap_type: WrapType,
}

impl CssProp for MainFlow {
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pair) = CssParser::parse(CssRule::main_flow, i).ok() {
            Self::parse(pair.last().unwrap())
        } else {
            None
        }
    }

    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        let mut main_flow = Self::default();
        for pair in pair.into_inner() {
            match pair.as_rule() {
                CssRule::direction => {
                    Direction::parse(pair).map(|v| {
                        main_flow.direction = v;
                    });
                }
                CssRule::wrap_type => {
                    WrapType::parse(pair).map(|v| {
                        main_flow.wrap_type = v;
                    });
                }
                _ => {}
            }
        }
        Some(main_flow)
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        self.direction.to_css(dest)?;
        dest.write_char(' ')?;
        self.wrap_type.to_css(dest)?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Row
    }
}

impl CssProp for Direction {
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pair) = CssParser::parse(CssRule::direction, i).ok() {
            Self::parse(pair.last().unwrap())
        } else {
            None
        }
    }

    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        for pair in pair.into_inner() {
            let v = match pair.as_rule() {
                CssRule::row => Some(Self::Row),
                CssRule::row_reverse => Some(Self::RowReverse),
                CssRule::column => Some(Self::Column),
                CssRule::column_reverse => Some(Self::ColumnReverse),
                _ => None,
            };
            return v;
        }
        None
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        match self {
            Direction::Row => dest.write_str("row"),
            Direction::RowReverse => dest.write_str("row_reverse"),
            Direction::Column => dest.write_str("column"),
            Direction::ColumnReverse => dest.write_str("column_reverse"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum WrapType {
    Wrap,
    NoWrap,
    WrapReverse,
}

impl Default for WrapType {
    fn default() -> Self {
        Self::NoWrap
    }
}

impl CssProp for WrapType {
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pair) = CssParser::parse(CssRule::wrap_type, i).ok() {
            Self::parse(pair.last().unwrap())
        } else {
            None
        }
    }

    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        for pair in pair.into_inner() {
            let v = match pair.as_rule() {
                CssRule::wrap => Some(Self::Wrap),
                CssRule::nowrap => Some(Self::NoWrap),
                CssRule::wrap_reverse => Some(Self::WrapReverse),
                _ => None,
            };
            return v;
        }
        None
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        match self {
            WrapType::Wrap => dest.write_str("wrap"),
            WrapType::NoWrap => dest.write_str("nowrap"),
            WrapType::WrapReverse => dest.write_str("wrap_reverse"),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct MainItems(pub Align);

impl CssProp for MainItems {
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pair) = CssParser::parse(CssRule::main_items, i).ok() {
            Self::parse(pair.last().unwrap())
        } else {
            None
        }
    }

    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        for pair in pair.into_inner() {
            return Align::parse(pair).map(|v| Self(v));
        }
        None
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        self.0.to_css(dest)
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct MainContent(pub Align);

impl CssProp for MainContent {
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pair) = CssParser::parse(CssRule::main_content, i).ok() {
            Self::parse(pair.last().unwrap())
        } else {
            None
        }
    }

    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        for pair in pair.into_inner() {
            return Align::parse(pair).map(|v| Self(v));
        }
        None
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        self.0.to_css(dest)
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct CrossItems(pub Align);

impl CssProp for CrossItems {
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pair) = CssParser::parse(CssRule::cross_items, i).ok() {
            Self::parse(pair.last().unwrap())
        } else {
            None
        }
    }

    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        for pair in pair.into_inner() {
            return Align::parse(pair).map(|v| Self(v));
        }
        None
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        self.0.to_css(dest)
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct CrossContent(pub Align);

impl CssProp for CrossContent {
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pair) = CssParser::parse(CssRule::cross_content, i).ok() {
            Self::parse(pair.last().unwrap())
        } else {
            None
        }
    }

    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        for pair in pair.into_inner() {
            return Align::parse(pair).map(|v| Self(v));
        }
        None
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        self.0.to_css(dest)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Align {
    Start,
    Center,
    End,
}

impl Default for Align {
    fn default() -> Self {
        Self::Start
    }
}

impl CssProp for Align {
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pair) = CssParser::parse(CssRule::align, i).ok() {
            Self::parse(pair.last().unwrap())
        } else {
            None
        }
    }

    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        for pair in pair.into_inner() {
            let v = match pair.as_rule() {
                CssRule::start => Some(Align::Start),
                CssRule::center => Some(Align::Center),
                CssRule::end => Some(Align::End),
                _ => None,
            };
            return v;
        }
        None
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        match self {
            Align::Start => dest.write_str("start"),
            Align::Center => dest.write_str("center"),
            Align::End => dest.write_str("end"),
        }
    }
}

#[test]
fn test_flex_layout() {
    let layout = FlexLayout::parse_str("flex()");
    assert_eq!(layout, Some(FlexLayout::default()));

    let layout = FlexLayout::parse_str("flex(main_flow: row wrap;)");
    assert_eq!(
        layout,
        Some(FlexLayout {
            main_flow: MainFlow {
                direction: Direction::Row,
                wrap_type: WrapType::Wrap
            },
            ..Default::default()
        })
    );

    let layout =
        FlexLayout::parse_str("flex(main_flow: row wrap; cross_items: center; main_content: end;)");
    assert_eq!(
        layout,
        Some(FlexLayout {
            main_flow: MainFlow {
                direction: Direction::Row,
                wrap_type: WrapType::Wrap
            },
            cross_items: CrossItems(Align::Center),
            main_content: MainContent(Align::End),
            ..Default::default()
        })
    );
}
