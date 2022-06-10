use core::fmt::{Display, Formatter};

use crate::properties::{CssCodec, Property};
use crate::utils::nom::*;

// #[path = "./def_properties.rs"]
// pub mod properties;

#[derive(Debug, PartialEq)]
pub struct CssEntities(Vec<CssEntity>);

#[derive(Debug, PartialEq)]
pub enum CssEntity {
    Block {
        selectors: Selectors,
        properties: Properties,
    },
}

#[derive(Debug, PartialEq)]
pub struct Selectors(pub(crate) Vec<Selector>);

#[derive(Debug, PartialEq)]
pub struct Properties(pub(crate) Vec<Property>);

#[derive(Debug, PartialEq)]
pub enum Selector {
    /// # 开头, 唯一Id匹配的元素
    Id(String),
    /// . 开头,
    Class(String),
    /// 纯字符串
    Tag(String),
    /// * 表示所有子元素
    All,
    /// 组合多种选择器
    /// 如: "#user .friend" 表示 所有id为user的元素下, 所有的class为friend的元素
    Combinator(Vec<Selector>),
}

impl Display for Selector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Selector::Id(id) => {
                write!(f, "#{}", id)
            }
            Selector::Class(class) => {
                write!(f, ".{}", class)
            }
            Selector::Tag(tag) => {
                write!(f, "{}", tag)
            }
            Selector::All => {
                write!(f, "*")
            }
            Selector::Combinator(c) => {
                if let Some((last, c)) = c.split_last() {
                    for s in c.iter() {
                        write!(f, "{} ", s)?;
                    }
                    write!(f, "{}", last)?;
                }
                Ok(())
            }
        }
    }
}

pub fn parse_css(i: &str) -> IResult<&str, CssEntities> {
    skip_useless(all_consuming(parse_entities))(i)
}

fn parse_entities(i: &str) -> IResult<&str, CssEntities> {
    map(many0(parse_entity), |e| CssEntities(e))(i)
}

fn parse_entity(i: &str) -> IResult<&str, CssEntity> {
    skip_useless(parse_block)(i)
}

/// 解析 Block
fn parse_block(i: &str) -> IResult<&str, CssEntity> {
    map(
        tuple((parse_selectors, parse_parameters)),
        |(selectors, properties)| CssEntity::Block {
            selectors,
            properties,
        },
    )(i)
}

/// 解析 选择器列表
fn parse_selectors(i: &str) -> IResult<&str, Selectors> {
    map(
        preceded(
            // 预取一个字符, 如果不是 { 就继续执行
            skip_useless(peek(none_of("{"))),
            separated_list1(tag(","), skip_useless(parse_selector)),
        ),
        |selectors| Selectors(selectors),
    )(i)
}

/// 解析 选择器
fn parse_selector(i: &str) -> IResult<&str, Selector> {
    let (i, output) = is_not(",{")(i)?;
    let mut selector = Vec::<Selector>::new();
    for i in output.split_ascii_whitespace().into_iter() {
        let (_, s) = alt((
            map(preceded(tag("#"), is_not(",{")), |i: &str| {
                Selector::Id(i.trim().into())
            }),
            map(preceded(tag("."), is_not(",{")), |i: &str| {
                Selector::Class(i.trim().into())
            }),
            map(is_not(",{"), |i: &str| Selector::Tag(i.trim().into())),
            map(tag("*"), |_| Selector::All),
        ))(i)?;
        selector.push(s);
    }
    let selector = if selector.len() == 1 {
        selector.pop().unwrap()
    } else {
        Selector::Combinator(selector)
    };
    Ok((i, selector))
}

/// 解析 选择器列表
fn parse_parameters(i: &str) -> IResult<&str, Properties> {
    skip_useless(delimited(
        skip_useless(tag("{")),
        map(
            many0(terminated(
                skip_useless(Property::parse),
                skip_useless(nom_char(';')),
            )),
            |properties| Properties(properties),
        ),
        skip_useless(tag("}")),
    ))(i)
}
