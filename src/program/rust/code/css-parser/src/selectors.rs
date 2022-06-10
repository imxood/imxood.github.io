use core::fmt::{Display, Formatter};

use crate::parse::{skip_sp, CssCodec};
use crate::properties::Property;
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
    all_consuming(parse_entities)(i)
}

fn parse_entities(i: &str) -> IResult<&str, CssEntities> {
    map(skip_sp(many0(parse_entity)), |e| CssEntities(e))(i)
}

fn parse_entity(i: &str) -> IResult<&str, CssEntity> {
    parse_block(i)
}

/// 解析 Block
fn parse_block(i: &str) -> IResult<&str, CssEntity> {
    map(
        tuple((skip_sp(parse_selectors), parameter_block(parse_parameters))),
        |(selectors, properties)| CssEntity::Block {
            selectors,
            properties,
        },
    )(i)
}

/// 解析 选择器列表
fn parse_selectors(i: &str) -> IResult<&str, Selectors> {
    map(
        is_not_block_ending(separated_list1(tag(","), skip_sp(parse_selector))),
        |selectors| Selectors(selectors),
    )(i)
}

/// 解析 选择器
fn parse_selector(i: &str) -> IResult<&str, Selector> {
    let (i, output) = is_not(",{")(i)?;
    let mut selector = Vec::<Selector>::new();
    for i in output.split_ascii_whitespace().into_iter() {
        let (_, s) = alt((
            parse_selector_id,
            parse_selector_class,
            parse_selector_tag,
            parse_selector_all,
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

/// 解析 Selector::Id
fn parse_selector_id(i: &str) -> IResult<&str, Selector> {
    map(preceded(tag("#"), is_not(",{")), |i: &str| {
        Selector::Id(i.trim().into())
    })(i)
}

/// 解析 Selector::Class
fn parse_selector_class(i: &str) -> IResult<&str, Selector> {
    map(preceded(tag("."), is_not(",{")), |i: &str| {
        Selector::Class(i.trim().into())
    })(i)
}

/// 解析 Selector::Tag
fn parse_selector_tag(i: &str) -> IResult<&str, Selector> {
    map(is_not(",{"), |i: &str| Selector::Tag(i.trim().into()))(i)
}

/// 解析 Selector::All
fn parse_selector_all(i: &str) -> IResult<&str, Selector> {
    map(tag("*"), |_| Selector::All)(i)
}

/// 解析 选择器列表
fn parse_parameters(i: &str) -> IResult<&str, Properties> {
    map(
        many0(skip_sp(is_not_block_ending(parse_parameter))),
        |properties| Properties(properties),
    )(i)
}

fn parse_parameter(input: &str) -> IResult<&str, Property> {
    Property::parse(input)
}

/*
    utils
*/

fn parameter_block<'a, O, P>(parser: P) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    P: NomParser<&'a str, O, nom::error::Error<&'a str>>,
{
    delimited(tag("{"), parser, tag("}"))
}

/// 判断 下一个字符 不是 { 和 }
fn is_not_block_ending<'a, O, P>(parser: P) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    P: NomParser<&'a str, O, nom::error::Error<&'a str>>,
{
    // 如果下一个字符 不是{} 任意的一个, 则执行 parser, 使用peek, 不会对 preceded 的 input 产生影响
    preceded(peek(none_of("{}")), parser)
}
