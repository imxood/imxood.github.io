use crate::parser::*;
use crate::properties::*;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Flex {
    align_item: AlignItem,
    flex_grow: FlexGrow,
}

impl CssProp for Flex {
    fn rule() -> CssRule {
        CssRule::flex
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let mut flex = Self::default();
        for pair in pair.into_inner() {
            match pair.as_rule() {
                CssRule::flex_grow => {
                    flex.flex_grow = FlexGrow::parse(pair);
                }
                CssRule::align_item => {
                    flex.align_item = AlignItem::parse(pair);
                }
                _ => {}
            };
        }
        flex
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
    fn rule() -> CssRule {
        CssRule::flex_grow
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let pair = pair.into_inner().next().unwrap();
        Self(PositiveNumber::parse(pair))
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        self.0.to_css(dest)
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct AlignItem(pub Align);

impl CssProp for AlignItem {
    fn rule() -> CssRule {
        CssRule::align_item
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let pair = pair.into_inner().next().unwrap();
        Self(Align::parse(pair))
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
    fn rule() -> CssRule {
        CssRule::flex_layout
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let mut layout = Self::default();
        for pair in pair.into_inner() {
            match pair.as_rule() {
                CssRule::flow => {
                    layout.flow = Flow::parse(pair);
                }
                CssRule::justify_items => {
                    layout.justify_items = JustifyItems::parse(pair);
                }
                CssRule::justify_content => {
                    layout.justify_content = JustifyContent::parse(pair);
                }
                CssRule::align_items => {
                    layout.align_items = AlignItems::parse(pair);
                }
                CssRule::align_content => {
                    layout.align_content = AlignContent::parse(pair);
                }
                _ => {
                    unreachable!()
                }
            }
        }
        layout
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
    fn rule() -> CssRule {
        CssRule::flow
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let mut flow = Self::default();
        for pair in pair.into_inner() {
            match pair.as_rule() {
                CssRule::direction => {
                    flow.direction = Direction::parse(pair);
                }
                CssRule::wrap_type => {
                    flow.wrap_type = WrapType::parse(pair);
                }
                _ => {}
            }
        }
        flow
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
    fn rule() -> CssRule {
        CssRule::direction
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            CssRule::row => Self::Row,
            CssRule::column => Self::Column,
            CssRule::row_reverse => Self::RowReverse,
            CssRule::column_reverse => Self::ColumnReverse,
            _ => unreachable!(),
        }
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
    fn rule() -> CssRule {
        CssRule::wrap_type
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            CssRule::wrap => Self::Wrap,
            CssRule::nowrap => Self::NoWrap,
            CssRule::wrap_reverse => Self::WrapReverse,
            _ => unreachable!(),
        }
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
    fn rule() -> CssRule {
        CssRule::justify_items
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let pair = pair.into_inner().next().unwrap();
        Self(Align::parse(pair))
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        self.0.to_css(dest)
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct JustifyContent(pub Align);

impl CssProp for JustifyContent {
    fn rule() -> CssRule {
        CssRule::justify_content
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let pair = pair.into_inner().next().unwrap();
        Self(Align::parse(pair))
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        self.0.to_css(dest)
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct AlignItems(pub Align);

impl CssProp for AlignItems {
    fn rule() -> CssRule {
        CssRule::align_items
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let pair = pair.into_inner().next().unwrap();
        Self(Align::parse(pair))
    }

    fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
        self.0.to_css(dest)
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct AlignContent(pub Align);

impl CssProp for AlignContent {
    fn rule() -> CssRule {
        CssRule::align_content
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let pair = pair.into_inner().next().unwrap();
        Self(Align::parse(pair))
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
    fn rule() -> CssRule {
        CssRule::align
    }

    fn parse(pair: Pair<CssRule>) -> Self {
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            CssRule::start => Align::Start,
            CssRule::center => Align::Center,
            CssRule::end => Align::End,
            _ => unreachable!(),
        }
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
        Ok(Flex {
            flex_grow: FlexGrow(PositiveNumber(1.0)),
            ..Default::default()
        })
    );

    let flex = flex.unwrap();
    assert_eq!(flex.to_css_string().as_str(), "1 start");

    let flex = Flex::parse_str("-1.0");
    println!("flex: {:?}", &flex);
    assert_eq!(flex.is_err(), true);
}

#[test]
fn test_flex_layout() {
    let layout = FlexLayout::parse_str("flex()");
    assert_eq!(layout, Ok(FlexLayout::default()));

    let layout = FlexLayout::parse_str("flex(flow: row wrap;)");
    assert_eq!(
        layout,
        Ok(FlexLayout {
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
        Ok(FlexLayout {
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
