use core::fmt::{Display, Formatter};

use nom::branch::alt;
use nom::bytes::complete::take_till;
use nom::character::complete::none_of;
use nom::combinator::{all_consuming, map, peek};
use nom::multi::{many0, separated_list1};
use nom::sequence::terminated;
use nom::{
    bytes::complete::{is_not, tag},
    sequence::{delimited, preceded, tuple},
};
use nom::{IResult, Parser as NomParser};

use crate::parse::{nom_char, skip_useless, Parser};
use crate::properties::Property;

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

#[cfg(test)]
mod test {
    use crate::{
        parse::parse_comment,
        properties::{height, width, Property},
    };

    use super::{
        parse_block, parse_css, parse_entities, parse_selector, parse_selectors, CssEntities,
        CssEntity, Properties, Selector, Selectors,
    };

    #[test]
    fn test_selector() {
        assert_eq!(parse_selector("div"), Ok(("", Selector::Tag("div".into()))));

        assert_eq!(
            parse_selectors("div"),
            Ok(("", Selectors(vec![Selector::Tag("div".into())])))
        );
    }

    #[test]
    fn test_comment() {
        assert_eq!(
            parse_comment("/* this is comment */"),
            Ok(("", " this is comment "))
        );
    }

    #[test]
    fn test_block() {
        assert_eq!(
            parse_block("/* this is comment */ .user { width: 100%; }"),
            Ok((
                "",
                CssEntity::Block {
                    selectors: Selectors(vec![Selector::Class("user".into())]),
                    properties: Properties(vec![Property::Width(width("100%").unwrap())])
                }
            ))
        );
        assert_eq!(
            parse_block("/* this is comment */ #user { width: 100%; }"),
            Ok((
                "",
                CssEntity::Block {
                    selectors: Selectors(vec![Selector::Id("user".into())]),
                    properties: Properties(vec![Property::Width(width("100%").unwrap())])
                }
            ))
        );
        assert_eq!(
            parse_block("/* this is comment */ div { width: 100%; }"),
            Ok((
                "",
                CssEntity::Block {
                    selectors: Selectors(vec![Selector::Tag("div".into())]),
                    properties: Properties(vec![Property::Width(width("100%").unwrap())])
                }
            ))
        );
    }

    #[test]
    fn test_entities() {
        assert_eq!(
            parse_entities(
                r#"
                    /* this is comment */
                    div {
                        width: 100%;
                    }
                    
                    /* this is comment */
                    .user0 {
                        width: 100%;
                    }

                    /* this is comment */
                    #user0 {
                        height: 100%;
                    }
                "#
            ),
            Ok((
                "",
                CssEntities(vec![
                    CssEntity::Block {
                        selectors: Selectors(vec![Selector::Tag("div".into())]),
                        properties: Properties(vec![Property::Width(width("100%").unwrap())])
                    },
                    CssEntity::Block {
                        selectors: Selectors(vec![Selector::Class("user0".into())]),
                        properties: Properties(vec![Property::Width(width("100%").unwrap())])
                    },
                    CssEntity::Block {
                        selectors: Selectors(vec![Selector::Id("user0".into())]),
                        properties: Properties(vec![Property::Height(height("100%").unwrap())])
                    }
                ])
            ))
        );
        assert_eq!(
            parse_block("/* this is comment */ #user { width: 100%; }"),
            Ok((
                "",
                CssEntity::Block {
                    selectors: Selectors(vec![Selector::Id("user".into())]),
                    properties: Properties(vec![Property::Width(width("100%").unwrap())])
                }
            ))
        );
        assert_eq!(
            parse_block("/* this is comment */ div { width: 100%; }"),
            Ok((
                "",
                CssEntity::Block {
                    selectors: Selectors(vec![Selector::Tag("div".into())]),
                    properties: Properties(vec![Property::Width(width("100%").unwrap())])
                }
            ))
        );
    }

    #[test]
    fn test_css_file() {
        let css_data = include_str!("./simple.css");
        assert_eq!(
            parse_css(css_data),
            Ok((
                "",
                CssEntities(vec![
                    CssEntity::Block {
                        selectors: Selectors(vec![Selector::Id("content".into())]),
                        properties: Properties(vec![
                            // Property::Display(Display(1.0)),
                            Property::Height(height("500.0").unwrap()),
                        ])
                    },
                    CssEntity::Block {
                        selectors: Selectors(vec![Selector::Class("first".into())]),
                        properties: Properties(vec![Property::Height(height("50.0").unwrap()),])
                    },
                    CssEntity::Block {
                        selectors: Selectors(vec![
                            Selector::Combinator(vec![
                                Selector::Id("content".into()),
                                Selector::Class("first".into())
                            ],),
                            Selector::Combinator(vec![
                                Selector::Id("content".into()),
                                Selector::Class("second".into())
                            ])
                        ]),
                        properties: Properties(vec![Property::Height(height("50.0").unwrap())])
                    }
                ])
            ))
        );
    }
}

#[test]
fn test() {
    let css_data = include_str!("./simple.css");
    let (_, ret) = parse_css(css_data).unwrap();
    println!("{:#?}", &ret);
}
