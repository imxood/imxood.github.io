use crate::parser::*;
use crate::properties::*;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Flex {
    align_item: AlignItem,
    flex_grow: FlexGrow,
}

impl CssProp for Flex {
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pair) = CssParser::parse(CssRule::flex, i).ok() {
            Self::parse(pair.last().unwrap())
        } else {
            None
        }
    }

    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        let mut flex = Self::default();
        let mut changed = false;
        for pair in pair.into_inner() {
            match pair.as_rule() {
                CssRule::flex_grow => {
                    changed = true;
                    FlexGrow::parse(pair).map(|v| flex.flex_grow = v);
                }
                CssRule::align_item => {
                    changed = true;
                    AlignItem::parse(pair).map(|v| flex.align_item = v);
                }
                _ => {}
            };
        }
        if changed {
            Some(flex)
        } else {
            None
        }
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        self.flex_grow.to_css(dest)?;
        dest.write_char(' ')?;
        self.align_item.to_css(dest)?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct FlexGrow(pub PositiveNumber);

impl CssProp for FlexGrow {
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pair) = CssParser::parse(CssRule::flex_grow, i).ok() {
            Self::parse(pair.last().unwrap())
        } else {
            None
        }
    }

    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        for pair in pair.into_inner() {
            let v = match pair.as_rule() {
                CssRule::positive_number => PositiveNumber::parse(pair).map(|v| Self(v)),
                _ => None,
            };
            return v;
        }
        None
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        self.0.to_css(dest)
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct AlignItem(pub Align);

impl CssProp for AlignItem {
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pair) = CssParser::parse(CssRule::align_item, i).ok() {
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
pub struct FlexLayout {
    pub flow: Flow,
    pub justify_items: JustifyItems,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub align_content: AlignContent,
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
        let mut layout = Self::default();
        println!("rule: {:?} str: {}", pair.as_rule(), pair.as_str());
        for pair in pair.into_inner() {
            match pair.as_rule() {
                CssRule::flow => {
                    Flow::parse(pair).map(|v| {
                        layout.flow = v;
                    });
                }
                CssRule::justify_items => {
                    JustifyItems::parse(pair).map(|v| {
                        layout.justify_items = v;
                    });
                }
                CssRule::justify_content => {
                    JustifyContent::parse(pair).map(|v| {
                        layout.justify_content = v;
                    });
                }
                CssRule::align_items => {
                    AlignItems::parse(pair).map(|v| {
                        layout.align_items = v;
                    });
                }
                CssRule::align_content => {
                    AlignContent::parse(pair).map(|v| {
                        layout.align_content = v;
                    });
                }
                _ => {}
            }
        }
        Some(layout)
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        dest.write_fmt(format_args!(
            "flex(flow: {}; justify_items: {}; justify_content: {}; align_items: {}; align_content: {};)",
            self.flow.to_css_string(),
            self.justify_items.to_css_string(),
            self.justify_content.to_css_string(),
            self.align_items.to_css_string(),
            self.align_content.to_css_string(),
        ))
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Flow {
    pub direction: Direction,
    pub wrap_type: WrapType,
}

impl CssProp for Flow {
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pair) = CssParser::parse(CssRule::flow, i).ok() {
            Self::parse(pair.last().unwrap())
        } else {
            None
        }
    }

    fn parse(pair: Pair<CssRule>) -> Option<Self> {
        let mut flow = Self::default();
        for pair in pair.into_inner() {
            match pair.as_rule() {
                CssRule::direction => {
                    Direction::parse(pair).map(|v| {
                        flow.direction = v;
                    });
                }
                CssRule::wrap_type => {
                    WrapType::parse(pair).map(|v| {
                        flow.wrap_type = v;
                    });
                }
                _ => {}
            }
        }
        Some(flow)
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
pub struct JustifyItems(pub Align);

impl CssProp for JustifyItems {
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pair) = CssParser::parse(CssRule::justify_items, i).ok() {
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
pub struct JustifyContent(pub Align);

impl CssProp for JustifyContent {
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pair) = CssParser::parse(CssRule::justify_content, i).ok() {
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
pub struct AlignItems(pub Align);

impl CssProp for AlignItems {
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pair) = CssParser::parse(CssRule::align_items, i).ok() {
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
pub struct AlignContent(pub Align);

impl CssProp for AlignContent {
    fn parse_str(i: &str) -> Option<Self> {
        if let Some(pair) = CssParser::parse(CssRule::align_content, i).ok() {
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
fn test_flex() {
    let flex = Flex::parse_str("1.0");
    assert_eq!(
        flex,
        Some(Flex {
            flex_grow: FlexGrow(PositiveNumber(1.0)),
            ..Default::default()
        })
    );

    let flex = flex.unwrap();
    assert_eq!(flex.to_css_string().as_str(), "1 start");

    let flex = Flex::parse_str("-1.0");
    assert_eq!(flex, None);
}

#[test]
fn test_flex_layout() {
    let layout = FlexLayout::parse_str("flex()");
    assert_eq!(layout, Some(FlexLayout::default()));

    let layout = FlexLayout::parse_str("flex(flow: row wrap;)");
    assert_eq!(
        layout,
        Some(FlexLayout {
            flow: Flow {
                direction: Direction::Row,
                wrap_type: WrapType::Wrap
            },
            ..Default::default()
        })
    );

    let layout =
        FlexLayout::parse_str("flex(flow: row wrap; align_items: center; justify_content: end;)");
    assert_eq!(
        layout,
        Some(FlexLayout {
            flow: Flow {
                direction: Direction::Row,
                wrap_type: WrapType::Wrap
            },
            align_items: AlignItems(Align::Center),
            justify_content: JustifyContent(Align::End),
            ..Default::default()
        })
    );

    let layout = layout.unwrap();
    assert_eq!(layout.to_css_string().as_str(), "flex(flow: row wrap; justify_items: start; justify_content: end; align_items: center; align_content: start;)");
}
