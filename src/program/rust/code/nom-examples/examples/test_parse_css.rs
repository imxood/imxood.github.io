use std::fmt::{Display, Formatter};

use nom::branch::alt;
use nom::bytes::complete::{is_not, take_until};
use nom::character::complete::{multispace1, none_of};
use nom::combinator::{all_consuming, map, peek};
use nom::multi::{many0, separated_list1};
use nom::sequence::{preceded, separated_pair, terminated, tuple};
use nom::{bytes::complete::tag, sequence::delimited};
use nom::{IResult, Parser};

fn main() {
    let css_data = include_str!("./simple.css");
    let (_, ret) = parse_css(css_data).unwrap();
    println!("{:#?}", &ret);
}

#[derive(Debug, PartialEq, Eq)]
pub struct CssEntities(Vec<CssEntity>);

#[derive(Debug, PartialEq, Eq)]
pub enum CssEntity {
    Block {
        selectors: Selectors,
        parameters: Parameters,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub struct Selectors(pub(crate) Vec<Selector>);

#[derive(Debug, PartialEq, Eq)]
pub struct Parameters(pub(crate) Vec<(String, String)>);

#[derive(Debug, PartialEq, Eq)]
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
    map(non_useless(many0(parse_entity)), |e| CssEntities(e))(i)
}

fn parse_entity(i: &str) -> IResult<&str, CssEntity> {
    parse_block(i)
}

/// 解析 Block
fn parse_block(i: &str) -> IResult<&str, CssEntity> {
    map(
        tuple((
            non_useless(parse_selectors),
            parameter_block(parse_parameters),
        )),
        |(selectors, parameters)| CssEntity::Block {
            selectors,
            parameters,
        },
    )(i)
}

/// 解析 选择器列表
fn parse_selectors(i: &str) -> IResult<&str, Selectors> {
    map(
        is_not_block_ending(separated_list1(tag(","), non_useless(parse_selector))),
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

// /// 解析 Selector::Combinator
// fn parse_selector_combinator(i: &str) -> IResult<&str, Selector> {
//     map(is_not(",{"), |i: &str| {
//         Selector::Combinator(i.trim().into())
//     })(i)
// }

/// 解析 选择器列表
fn parse_parameters(i: &str) -> IResult<&str, Parameters> {
    map(
        many0(non_useless(is_not_block_ending(parse_parameter))),
        |parameters| Parameters(parameters),
    )(i)
}

fn parse_parameter(i: &str) -> IResult<&str, (String, String)> {
    map(
        terminated(
            separated_pair(
                non_useless(is_not(":;}")),
                tag(":"),
                non_useless(is_not(";}")),
            ),
            alt((tag(";"), tag("}"))),
        ),
        |(k, v)| (k.into(), v.into()),
    )(i)
}

/*
    utils
*/

fn parameter_block<'a, O, P>(parser: P) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    P: Parser<&'a str, O, nom::error::Error<&'a str>>,
{
    delimited(tag("{"), parser, tag("}"))
}

/// 判断 下一个字符 不是 { 和 }
fn is_not_block_ending<'a, O, P>(parser: P) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    P: Parser<&'a str, O, nom::error::Error<&'a str>>,
{
    // 如果下一个字符 不是{} 任意的一个, 则执行 parser, 使用peek, 不会对 preceded 的 input 产生影响
    preceded(peek(none_of("{}")), parser)
}

/// 给 输入的解析器 去除空格 和 注释
fn non_useless<'a, O, P>(parser: P) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    P: Parser<&'a str, O, nom::error::Error<&'a str>>,
{
    map(tuple((parse_useless, parser, parse_useless)), |(_, s, _)| s)
}

/// 解析 空符号 和 注释
fn parse_useless(i: &str) -> IResult<&str, Vec<&str>> {
    many0(alt((multispace1, parse_comment)))(i)
}

/// 解析注释
fn parse_comment(i: &str) -> IResult<&str, &str> {
    map(
        tuple((tag("/*"), take_until("*/"), tag("*/"))),
        |(_, text, _)| text,
    )(i)
}

#[cfg(test)]
mod test {
    use super::{
        parse_block, parse_comment, parse_css, parse_entities, parse_selector,
        parse_selector_class, parse_selector_id, parse_selector_tag, parse_selectors, CssEntities,
        CssEntity, Parameters, Selector, Selectors,
    };

    #[test]
    fn test_selector() {
        assert_eq!(
            parse_selector_id("#hello"),
            Ok(("", Selector::Id("hello".into())))
        );

        assert_eq!(
            parse_selector_class(".hello"),
            Ok(("", Selector::Class("hello".into())))
        );

        assert_eq!(
            parse_selector_tag("div"),
            Ok(("", Selector::Tag("div".into())))
        );

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
                    parameters: Parameters(vec![("width".into(), "100%".into())])
                }
            ))
        );
        assert_eq!(
            parse_block("/* this is comment */ #user { width: 100%; }"),
            Ok((
                "",
                CssEntity::Block {
                    selectors: Selectors(vec![Selector::Id("user".into())]),
                    parameters: Parameters(vec![("width".into(), "100%".into())])
                }
            ))
        );
        assert_eq!(
            parse_block("/* this is comment */ div { width: 100%; }"),
            Ok((
                "",
                CssEntity::Block {
                    selectors: Selectors(vec![Selector::Tag("div".into())]),
                    parameters: Parameters(vec![("width".into(), "100%".into())])
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
                        parameters: Parameters(vec![("width".into(), "100%".into())])
                    },
                    CssEntity::Block {
                        selectors: Selectors(vec![Selector::Class("user0".into())]),
                        parameters: Parameters(vec![("width".into(), "100%".into())])
                    },
                    CssEntity::Block {
                        selectors: Selectors(vec![Selector::Id("user0".into())]),
                        parameters: Parameters(vec![("height".into(), "100%".into())])
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
                    parameters: Parameters(vec![("width".into(), "100%".into())])
                }
            ))
        );
        assert_eq!(
            parse_block("/* this is comment */ div { width: 100%; }"),
            Ok((
                "",
                CssEntity::Block {
                    selectors: Selectors(vec![Selector::Tag("div".into())]),
                    parameters: Parameters(vec![("width".into(), "100%".into())])
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
                        parameters: Parameters(vec![
                            ("display".into(), "block".into()),
                            ("height".into(), "500px".into())
                        ])
                    },
                    CssEntity::Block {
                        selectors: Selectors(vec![Selector::Class("first".into())]),
                        parameters: Parameters(vec![("height".into(), "50px".into())])
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
                        parameters: Parameters(vec![("height".into(), "50px".into())])
                    }
                ])
            ))
        );
    }
}
