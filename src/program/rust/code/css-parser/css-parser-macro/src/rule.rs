
use std::fmt::{Debug, Display, Write};

use convert_case::{Case, Casing};
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, multispace0};
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{pair, preceded, tuple};
use nom::IResult;
use nom::{branch::alt, bytes::complete::take_till1};

use crate::parse::{nom_char, skip_useless};

pub trait Parser<T = Self> {
    fn parse(i: &str) -> IResult<&str, T>;
}

// 优先级: 数量描述 (QuantitySymbol) > 组合 ([]) > 并置 (一个或多个空格) > 与 (&&) > 或 (||) > 互斥 (|)

/// 定义规则, 用于解析 属性描述, 生成 属性结构体 和 属性的解析器
#[derive(Debug, Clone, PartialEq)]
pub enum RuleObject {
    /// [], 表示 多个规则的组合
    /// bold [ thin && <length> ]
    Brackets(Box<RuleObject>, Option<QuantitySymbol>),

    /// 并置项, 被 空格分离的不同的项, 有先后顺序
    /// 如: A B  等效于 [A B]
    Item(Box<RuleObject>, Option<Box<RuleObject>>),

    /// &&, 各个选项必须出现, 但是顺序任意
    /// 如: A && B && C
    DoubleAmpersand(Box<RuleObject>, Option<Box<RuleObject>>),

    /// ||, 至少一个 最多不超过 双杠参数值 的数量, 顺序任意
    /// 如: A || B || C
    DoubleBar(Box<RuleObject>, Option<Box<RuleObject>>),

    /// |, 只能取 这些选项中的一个
    /// 如: A | B | C
    SingleBar(Box<RuleObject>, Option<Box<RuleObject>>),

    /// 表示 一个特定的属性类型 的值
    Variable(String, Option<QuantitySymbol>),

    /// 符号, 常量字符串
    Symbol(String, Option<QuantitySymbol>),
}

// <grid-template> | <grid-template-rows> / [ auto-flow && dense? ] <grid-auto-columns>?
impl Parser for RuleObject {
    fn parse(i: &str) -> IResult<&str, Self> {
        parse_single_bar(i)
    }
}

impl Display for RuleObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Brackets(arg0, arg1) => {
                f.write_str("[ ")?;
                Display::fmt(arg0, f)?;
                f.write_str(" ]")?;
                if let Some(quantity) = arg1 {
                    Display::fmt(quantity, f)?;
                }
                Ok(())
            }
            Self::Item(arg0, arg1) => {
                Display::fmt(arg0, f)?;
                if let Some(arg1) = arg1 {
                    f.write_char(' ')?;
                    Display::fmt(arg1, f)?;
                }
                Ok(())
            }
            Self::DoubleAmpersand(arg0, arg1) => {
                Display::fmt(arg0, f)?;
                if let Some(arg1) = arg1 {
                    f.write_str(" && ")?;
                    Display::fmt(arg1, f)?;
                }
                Ok(())
            }
            Self::DoubleBar(arg0, arg1) => {
                Display::fmt(arg0, f)?;
                if let Some(arg1) = arg1 {
                    f.write_str(" || ")?;
                    Display::fmt(arg1, f)?;
                }
                Ok(())
            }
            Self::SingleBar(arg0, arg1) => {
                Display::fmt(arg0, f)?;
                if let Some(arg1) = arg1 {
                    f.write_str(" | ")?;
                    Display::fmt(arg1, f)?;
                }
                Ok(())
            }
            Self::Variable(arg0, arg1) => {
                f.write_char('<')?;
                Display::fmt(arg0, f)?;
                f.write_char('>')?;
                if let Some(arg1) = arg1 {
                    Display::fmt(arg1, f)?;
                }
                Ok(())
            }
            Self::Symbol(arg0, arg1) => {
                Display::fmt(arg0, f)?;
                if let Some(arg1) = arg1 {
                    Display::fmt(arg1, f)?;
                }
                Ok(())
            }
        }
    }
}

impl RuleObject {
    pub fn parser(&self, prop_name: &str, struct_type: &str) -> String {
        match self {
            RuleObject::Brackets(obj, quantity) => {
                if let Some(quantity) = quantity {
                    match quantity {
                        QuantitySymbol::Option => {
                            format!("opt({})", obj.parser(prop_name, struct_type))
                        }
                        QuantitySymbol::Star => {
                            format!("many0({})", obj.parser(prop_name, struct_type))
                        }
                        QuantitySymbol::Plus => {
                            format!("many1({})", obj.parser(prop_name, struct_type))
                        }
                        QuantitySymbol::Times(min, max) => {
                            format!(
                                "many_m_n({}, {}, {})",
                                min,
                                max,
                                obj.parser(prop_name, struct_type)
                            )
                        }
                    }
                } else {
                    obj.parser(prop_name, struct_type)
                }
            }
            RuleObject::Item(left, right) => {
                if let Some(right) = right {
                    format!(
                        "pair({}, {})",
                        left.parser(prop_name, struct_type),
                        right.parser(prop_name, struct_type)
                    )
                } else {
                    left.parser(prop_name, struct_type)
                }
            }
            RuleObject::DoubleAmpersand(left, right) => {
                if let Some(right) = right {
                    let one = left.parser(prop_name, struct_type);
                    let two = right.parser(prop_name, struct_type);
                    format!(
                        "alt(( alt(({}, {})), alt(({}, {})) ))",
                        &one, &two, &two, &one
                    )
                } else {
                    left.parser(prop_name, struct_type)
                }
            }
            RuleObject::DoubleBar(left, right) => {
                if let Some(right) = right {
                    let one = left.parser(prop_name, struct_type);
                    let two = right.parser(prop_name, struct_type);
                    // alt(( tuple(({}, {})), tuple(({}, {})) ))
                    format!(
                        "alt(( tuple(({}, {})), tuple(({}, {})) ))",
                        &one, &two, &two, &one,
                    )
                } else {
                    left.parser(prop_name, struct_type)
                }
            }
            RuleObject::SingleBar(left, right) => {
                if let Some(right) = right {
                    format!(
                        "alt(({}, {}))",
                        // map(Length::parse, |length| Self::Length(length)),
                        left.parser(prop_name, struct_type),
                        right.parser(prop_name, struct_type)
                    )
                } else {
                    left.parser(prop_name, struct_type)
                }
            }
            RuleObject::Variable(variable, quantity) => {
                let snake = variable.to_case(Case::Snake);
                let pascal = variable.to_case(Case::Pascal);
                let prop_name = prop_name.to_case(Case::Pascal);
                if let Some(quantity) = quantity {
                    if struct_type == "enum" {
                        match quantity {
                            QuantitySymbol::Option => {
                                format!(
                                    "map(opt(map({}::parse, |{}| {}::{}({}))), |_|{{}})",
                                    &pascal, &snake, &prop_name, &pascal, &snake
                                )
                            }
                            QuantitySymbol::Star => {
                                format!(
                                    "map(many0(map({}::parse, |{}| {}::{}({}))), |_|{{}})",
                                    &pascal, &snake, &prop_name, &pascal, &snake
                                )
                            }
                            QuantitySymbol::Plus => {
                                format!(
                                    "map(many1(map({}::parse, |{}| {}::{}({}))), |_|{{}})",
                                    &pascal, &snake, &prop_name, &pascal, &snake
                                )
                            }
                            QuantitySymbol::Times(min, max) => {
                                format!(
                                    "map(many_m_n({}, {}, map({}::parse, |{}| {}::{}({}))), |_|{{}})",
                                    min, max, &pascal, &snake, &prop_name, &pascal, &snake
                                )
                            }
                        }
                    } else if struct_type == "struct" {
                        match quantity {
                            QuantitySymbol::Option => {
                                format!(
                                    "map(opt(map({}::parse, |{}| {{ v.{} = {}; }})), |_|{{}})",
                                    &pascal, &snake, &snake, &snake
                                )
                            }
                            QuantitySymbol::Star => {
                                format!(
                                    "map(many0(map({}::parse, |{}| {{ v.{} = {}; }})), |_|{{}})",
                                    &pascal, &snake, &pascal, &snake
                                )
                            }
                            QuantitySymbol::Plus => {
                                format!(
                                    "map(many1(map({}::parse, |{}| {{ v.{} = {}; }})), |_|{{}})",
                                    &pascal, &snake, &pascal, &snake
                                )
                            }
                            QuantitySymbol::Times(min, max) => {
                                format!(
                                    "map(many_m_n({}, {}, map({}::parse, |{}| {{ v.{} = {}; }})), |_|{{}})",
                                    min, max, &pascal, &snake, &pascal, &snake
                                )
                            }
                        }
                    } else {
                        "".to_string()
                    }
                } else {
                    if struct_type == "enum" {
                        format!(
                            "map({}::parse, |{}| {}::{}({}))",
                            &pascal, &snake, &prop_name, &pascal, &snake,
                        )
                    } else if struct_type == "struct" {
                        format!(
                            "map({}::parse, |{}| {{ v.{} = {}; }})",
                            &pascal, &snake, &snake, &snake
                        )
                    } else {
                        "".to_string()
                    }
                }
            }
            RuleObject::Symbol(_, _) => todo!(), // RuleObject::Symbol(symbol, quantity) => {
                                                 //     if let Some(quantity) = quantity {
                                                 //         match quantity {
                                                 //             QuantitySymbol::Option => {
                                                 //                 format!("opt({}::parse_{})", prop_name, symbol.to_case(Case::Snake))
                                                 //             }
                                                 //             QuantitySymbol::Star => {
                                                 //                 format!("many0({}::parse_{})", prop_name, symbol.to_case(Case::Snake))
                                                 //             }
                                                 //             QuantitySymbol::Plus => {
                                                 //                 format!("many1({}::parse_{})", prop_name, symbol.to_case(Case::Snake))
                                                 //             }
                                                 //             QuantitySymbol::Times(min, max) => {
                                                 //                 format!(
                                                 //                     "many_m_n({}, {}, {}::parse_{})",
                                                 //                     min,
                                                 //                     max,
                                                 //                     prop_name,
                                                 //                     symbol.to_case(Case::Snake)
                                                 //                 )
                                                 //             }
                                                 //         }
                                                 //     } else {
                                                 //         format!("{}::parse_{}", prop_name, symbol.to_case(Case::Snake))
                                                 //     }
                                                 // }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum QuantitySymbol {
    /// ? 0次或1次
    Option,
    /// * 0次或多次
    Star,
    /// + 1次或多次
    Plus,
    /// {m, n} 至少m次 最多n次
    Times(usize, usize),
    // // []! 方括号后面的叹号表示该组是必须的, 并且至少产生一个值, 即使组内项目全部允许忽略, 也至少保留一个
    // // ExclamationPoint,
}

impl Parser for QuantitySymbol {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((
            map(skip_useless(nom_char('?')), |_| Self::Option),
            map(skip_useless(nom_char('*')), |_| Self::Star),
            map(skip_useless(nom_char('+')), |_| Self::Plus),
            // map(skip_useless(nom_char('!')), |_| Self::ExclamationPoint),
            map(
                tuple((
                    skip_useless(nom_char('{')),
                    skip_useless(digit1),
                    skip_useless(digit1),
                    skip_useless(nom_char('}')),
                )),
                |(_, min, max, _)| {
                    Self::Times(
                        str::parse::<usize>(min).unwrap(),
                        str::parse::<usize>(max).unwrap(),
                    )
                },
            ),
        ))(i)
    }
}

impl Display for QuantitySymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Option => f.write_char('?'),
            Self::Star => f.write_char('*'),
            Self::Plus => f.write_char('+'),
            Self::Times(min, max) => f.write_fmt(format_args!("{{{}, {}}}", min, max)),
            // // Self::ExclamationPoint => f.write_char('!'),
        }
    }
}

// 解析 方括号
// [ A && B || C ... ]
fn parse_brackets(i: &str) -> IResult<&str, RuleObject> {
    map(
        tuple((
            skip_useless(nom_char('[')),
            skip_useless(RuleObject::parse),
            skip_useless(nom_char(']')),
            skip_useless(opt(QuantitySymbol::parse)),
        )),
        |(_, obj, _, quantity)| RuleObject::Brackets(Box::new(obj), quantity),
    )(i)
}

// 解析 空格, 并置项
// A && B && C || ...
fn parse_continuous_item(i: &str) -> IResult<&str, RuleObject> {
    let (i, obj1) = alt((parse_brackets, parse_variable, parse_string))(i)?;

    let (i, mut obj2) = preceded(
        multispace0,
        many0(preceded(
            multispace0,
            // 解析优先级更高的选项, 若失败, 则自动降级 解析, 直到
            parse_continuous_item,
        )),
    )(i)?;

    if obj2.is_empty() {
        return Ok((i, obj1));
    }

    Ok((
        i,
        RuleObject::Item(Box::new(obj1), obj2.pop().map(|v| Box::new(v))),
    ))
}

// 解析 双与号
// A && B && C || ...
fn parse_double_ampersand(i: &str) -> IResult<&str, RuleObject> {
    let (i, obj1) = alt((parse_continuous_item, parse_variable, parse_string))(i)?;

    let (i, mut obj2) = preceded(
        multispace0,
        many0(preceded(
            tag("&&"),
            // 解析优先级更高的选项, 若失败, 则自动降级 解析, 直到
            parse_double_ampersand,
        )),
    )(i)?;

    if obj2.is_empty() {
        return Ok((i, obj1));
    }

    Ok((
        i,
        RuleObject::DoubleAmpersand(Box::new(obj1), obj2.pop().map(|v| Box::new(v))),
    ))
}

// 解析 双杠
// A || B || C || ...
fn parse_double_bar(i: &str) -> IResult<&str, RuleObject> {
    let (i, obj1) = alt((parse_double_ampersand, parse_variable, parse_string))(i)?;

    let (i, mut obj2) = preceded(
        multispace0,
        many0(preceded(
            tag("||"),
            // 解析优先级更高的选项, 若失败, 则自动降级 解析, 直到
            parse_double_bar,
        )),
    )(i)?;

    if obj2.is_empty() {
        return Ok((i, obj1));
    }

    Ok((
        i,
        RuleObject::DoubleBar(Box::new(obj1), obj2.pop().map(|v| Box::new(v))),
    ))
}

// 解析 单杠
fn parse_single_bar(i: &str) -> IResult<&str, RuleObject> {
    let (i, obj1) = alt((parse_double_bar, parse_variable, parse_string))(i)?;

    let (i, mut obj2) = preceded(
        multispace0,
        many0(preceded(
            tag("|"),
            // 解析优先级更高的选项, 若失败, 则自动降级 解析, 直到
            parse_single_bar,
        )),
    )(i)?;

    if obj2.is_empty() {
        return Ok((i, obj1));
    }

    Ok((
        i,
        RuleObject::SingleBar(Box::new(obj1), obj2.pop().map(|v| Box::new(v))),
    ))
}

// 解析 可变的值
fn parse_variable(i: &str) -> IResult<&str, RuleObject> {
    // 如: <background_color>
    preceded(
        multispace0,
        map(
            tuple((
                nom_char('<'),
                take_till1(|c: char| !c.is_alphanumeric() && c != '-'),
                nom_char('>'),
                opt(QuantitySymbol::parse),
            )),
            |(_, i, _, quantity)| RuleObject::Variable(i.to_string(), quantity),
        ),
    )(i)
}

// 解析 符号
// 字母 数字 和 下划线 组成的字符串
fn parse_string(i: &str) -> IResult<&str, RuleObject> {
    // 如: Bold
    preceded(
        multispace0,
        map(
            pair(
                take_till1(|c: char| !c.is_alphanumeric() && c != '-'),
                opt(QuantitySymbol::parse),
            ),
            |(i, quantity)| RuleObject::Symbol(i.to_string(), quantity),
        ),
    )(i)
}
