use std::fmt::{Debug, Display, Write};
use std::ops::Range;

use convert_case::{Case, Casing};
use derive_more::IsVariant;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, multispace0};
use nom::combinator::{map, opt};
use nom::multi::{many0, many1, many_m_n};
use nom::sequence::{delimited, pair, preceded, tuple};
use nom::IResult;
use nom::{branch::alt, bytes::complete::take_till1};

use crate::parse::{nom_char, skip_sp, Parser};

use super::flex::FlexDirection;

// pub fn parse_flex_direction(i: &str) -> IResult<&str, FlexDirection> {

// }

// /* <'grid-template'> values */
// grid: none;
// grid: "a" 100px "b" 1fr;
// grid: [linename1] "a" 100px [linename2];
// grid: "a" 200px "b" min-content;
// grid: "a" minmax(100px, max-content) "b" 20%;
// grid: 100px / 200px;
// grid: minmax(400px, min-content) / repeat(auto-fill, 50px);

// /* <'grid-template-rows'> /
//    [ auto-flow && dense? ] <'grid-auto-columns'>? values */
// grid: 200px / auto-flow;
// grid: 30% / auto-flow dense;
// grid: repeat(3, [line1 line2 line3] 200px) / auto-flow 300px;
// grid: [line1] minmax(20em, max-content) / auto-flow dense 40%;

// /* [ auto-flow && dense? ] <'grid-auto-rows'>? /
//    <'grid-template-columns'> values */
// grid: auto-flow / 200px;
// grid: auto-flow dense / 30%;
// grid: auto-flow 300px / repeat(3, [line1 line2 line3] 200px);
// grid: auto-flow dense 40% / [line1] minmax(20em, max-content);

// /* Global values */
// grid: inherit;
// grid: initial;
// grid: unset;

// grid: [ auto-flow && dense? ] <grid-auto-rows>? / <grid-template-columns> | <grid-template> | <grid-template-rows> / [ auto-flow && dense? ] <grid-auto-columns>?

// 优先级: 数量描述 (QuantitySymbol) > 组合 ([]) > 并置 (一个或多个空格) > 与 (&&) > 或 (||) > 互斥 (|)

#[derive(Debug, PartialEq, IsVariant)]
pub enum RuleObject {
    // [], 表示 多个规则的组合
    // bold [ thin && <length> ]
    Brackets(Box<RuleObject>, Option<QuantitySymbol>),

    // 并置项, 被 空格分离的不同的项, 有先后顺序
    Item(Box<RuleObject>, Option<Box<RuleObject>>),

    // &&, 各个选项必须出现, 但是顺序任意
    // 如: A && B:
    //  A A1 && B B1， 等效于 [A A1] && [B B1]
    DoubleAmpersand(Box<RuleObject>, Option<Box<RuleObject>>),

    // ||, 至少一个 最多不超过 双杠参数值 的数量, 顺序任意
    // 如: A || B || C， 可以是 A, A B, B C A 等
    //  A A1 || B B1， 等效于 [A A1] || [B B1]
    DoubleBar(Box<RuleObject>, Option<Box<RuleObject>>),

    // |, 只能取 这些选项中的一个
    // 如: A | B | C， 只能是 A 或 B 或 C
    //  A A1 | B B1 等效于 [A A1] | [B B1]
    SingleBar(Box<RuleObject>, Option<Box<RuleObject>>),

    // 表示 一个特定的属性类型 的值
    Variable(String, Option<QuantitySymbol>),

    // 符号, 如: auto-flow
    Symbol(String, Option<QuantitySymbol>),
}

// let data = json!({
//     "background": "[background-color || background-image] background-position?",
//     "width": "<length> || <percentage-length>",
//     "height": "<length> || <percentage-length>",
// });

// "#ff00ff"
// "#ff00ff (100,100)"

// BackgroundBuilder::parse_background_color
// BackgroundBuilder::parse_background_image
// BackgroundBuilder::parse_background_position

// alt((BackgroundBuilder::parse_background_color, BackgroundBuilder::parse_background_image))

// BackgroundBuilder::parse_background_position

// text.to_case(Case::Pascal).into()

// 解析格式: [ A && B] <C>

// tuple((tuple((tag("auto-flow"), tag("dense"))), ))

impl RuleObject {
    pub fn parser(&self, builder: &str) -> String {
        match self {
            RuleObject::Brackets(obj, quantity) => {
                if let Some(quantity) = quantity {
                    match quantity {
                        QuantitySymbol::QuestionMark => format!("opt({})", obj.parser(builder)),
                        QuantitySymbol::Asterisk => format!("many0({})", obj.parser(builder)),
                        QuantitySymbol::Plus => format!("many1({})", obj.parser(builder)),
                        QuantitySymbol::CurlyBraces(min, max) => {
                            format!("many_m_n({}, {}, {})", min, max, obj.parser(builder))
                        }
                    }
                } else {
                    obj.parser(builder)
                }
            }
            RuleObject::Item(left, right) => {
                if let Some(right) = right {
                    format!("alt(({}, {}))", left.parser(builder), right.parser(builder))
                } else {
                    left.parser(builder)
                }
            }
            RuleObject::DoubleAmpersand(left, right) => {
                if let Some(right) = right {
                    format!("alt(({}, {}))", left.parser(builder), right.parser(builder))
                } else {
                    left.parser(builder)
                }
            }
            RuleObject::DoubleBar(left, right) => {
                if let Some(right) = right {
                    format!("alt(({}, {}))", left.parser(builder), right.parser(builder))
                } else {
                    left.parser(builder)
                }
            }
            RuleObject::SingleBar(left, right) => {
                if let Some(right) = right {
                    format!("alt(({}, {}))", left.parser(builder), right.parser(builder))
                } else {
                    left.parser(builder)
                }
            }
            RuleObject::Variable(variable, quantity) => {
                if let Some(quantity) = quantity {
                    match quantity {
                        QuantitySymbol::QuestionMark => {
                            format!("opt({}::parse_{})", builder, variable.to_case(Case::Snake))
                        }
                        QuantitySymbol::Asterisk => format!(
                            "many0({}::parse_{})",
                            builder,
                            variable.to_case(Case::Snake)
                        ),
                        QuantitySymbol::Plus => format!(
                            "many1({}::parse_{})",
                            builder,
                            variable.to_case(Case::Snake)
                        ),
                        QuantitySymbol::CurlyBraces(min, max) => {
                            format!(
                                "many_m_n({}, {}, {}::parse_{})",
                                min,
                                max,
                                builder,
                                variable.to_case(Case::Snake)
                            )
                        }
                    }
                } else {
                    format!("{}::parse_{}", builder, variable.to_case(Case::Snake))
                }
            }
            RuleObject::Symbol(symbol, quantity) => {
                if let Some(quantity) = quantity {
                    match quantity {
                        QuantitySymbol::QuestionMark => {
                            format!("opt({}::parse_{})", builder, symbol.to_case(Case::Snake))
                        }
                        QuantitySymbol::Asterisk => {
                            format!("many0({}::parse_{})", builder, symbol.to_case(Case::Snake))
                        }
                        QuantitySymbol::Plus => {
                            format!("many1({}::parse_{})", builder, symbol.to_case(Case::Snake))
                        }
                        QuantitySymbol::CurlyBraces(min, max) => {
                            format!(
                                "many_m_n({}, {}, {}::parse_{})",
                                min,
                                max,
                                builder,
                                symbol.to_case(Case::Snake)
                            )
                        }
                    }
                } else {
                    format!("{}::parse_{}", builder, symbol.to_case(Case::Snake))
                }
            }
        }
    }
}

impl Display for RuleObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Brackets(arg0, arg1) => {
                f.write_str("[ ")?;
                Display::fmt(arg0, f)?;
                if let Some(arg1) = arg1 {
                    f.write_str(&arg1.to_string())?;
                }
                f.write_str(" ]")
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum QuantitySymbol {
    /// ? 0次或1次
    QuestionMark,
    /// * 0次或多次
    Asterisk,
    /// + 1次或多次
    Plus,
    /// {m, n} 至少m次 最多n次
    CurlyBraces(usize, usize),
    // []! 方括号后面的叹号表示该组是必须的, 并且至少产生一个值, 即使组内项目全部允许忽略, 也至少保留一个
    // ExclamationPoint,
}

impl Display for QuantitySymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::QuestionMark => f.write_char('?'),
            Self::Asterisk => f.write_char('*'),
            Self::Plus => f.write_char('+'),
            // Self::ExclamationPoint => f.write_char('!'),
            Self::CurlyBraces(min, max) => f.write_fmt(format_args!("{{{}, {}}}", min, max)),
        }
    }
}

impl Parser for QuantitySymbol {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((
            map(preceded(multispace0, nom_char('?')), |_| Self::QuestionMark),
            map(preceded(multispace0, nom_char('*')), |_| Self::Asterisk),
            map(preceded(multispace0, nom_char('+')), |_| Self::Plus),
            // map(preceded(multispace0, nom_char('!')), |_| {
            //     Self::ExclamationPoint
            // }),
            map(
                tuple((
                    multispace0,
                    nom_char('{'),
                    multispace0,
                    digit1,
                    multispace0,
                    digit1,
                    multispace0,
                    nom_char('}'),
                )),
                |(_, _, _, min, _, max, _, _)| {
                    Self::CurlyBraces(
                        str::parse::<usize>(min).unwrap(),
                        str::parse::<usize>(max).unwrap(),
                    )
                },
            ),
        ))(i)
    }
}

// <grid-template> | <grid-template-rows> / [ auto-flow && dense? ] <grid-auto-columns>?
impl Parser for RuleObject {
    fn parse(i: &str) -> IResult<&str, Self> {
        parse_single_bar(i)
    }
}

// 解析 方括号
// [ A && B || C ... ]
fn parse_brackets(i: &str) -> IResult<&str, RuleObject> {
    preceded(
        multispace0,
        map(
            tuple((
                nom_char('['),
                RuleObject::parse,
                nom_char(']'),
                opt(QuantitySymbol::parse),
            )),
            |(_, obj, _, quantity)| RuleObject::Brackets(Box::new(obj), quantity),
        ),
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
    // e.g., <background_color>
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
    // e.g., Bold
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

mod test {
    use convert_case::{Case, Casing};
    use nom::branch::alt;
    use nom::character::complete::{alpha1, multispace0};
    use nom::combinator::map;
    use nom::sequence::preceded;
    use nom::{bytes::streaming::tag, character::complete::digit1, sequence::tuple, IResult};
    use serde_json::json;

    use crate::{
        display::parser::{parse_double_bar, RuleObject},
        parse::Parser,
    };

    use super::parse_double_ampersand;

    #[test]
    fn test_parse() {
        println!(
            "{:?}",
            parse_double_ampersand("auto-flow && dense").unwrap()
        );
        // A | B C && D
        let raw = "<grid-template> | <grid-template-rows> [auto-flow && dense?] <grid-auto-columns>? | [ auto-flow && dense? ] <grid-auto-rows>? <grid-template-columns>";

        struct RuleBuilder {
            prop_builder: String,
            rule_obj: RuleObject,
        }

        impl RuleBuilder {
            pub fn new(prop_name: &str, prop_rule: &str) -> RuleBuilder {
                // text.to_case(Case::Pascal)
                let prop_builder = prop_name.to_case(Case::Pascal) + "Builder";
                let (_, rule_obj) = RuleObject::parse(prop_rule).unwrap();
                Self {
                    rule_obj,
                    prop_builder,
                }
            }

            pub fn build(self) -> String {
                self.rule_obj.parser(&self.prop_builder)
            }
        }

        let (_, obj) = RuleObject::parse(raw).unwrap();
        println!("output: {:#?}", &obj);
        println!("output: {}", &obj);
        println!("raw:    {}", raw);
        println!("parser: {}", obj.parser("GridBuilder"));
        // alt((grid-template::parser(), alt((alt((grid-template-rows::parser(), alt((alt((auto-flow::parser(), opt(dense::parser()))), opt(grid-auto-columns::parser()))))), alt((alt((auto-flow::parser(), opt(dense::parser()))), alt((opt(grid-auto-rows::parser()), grid-template-columns::parser()))))))))

        let parser = RuleBuilder::new("grid", "[ A && B ] <C>").build();
        println!("output parser: {}", &parser);

        // let builder = GridBuilder::default();

        // let f = alt((alt((A::parse, B::parse)), C::parse));

        #[derive(Debug)]
        struct GridBuilder {
            pub inner: Grid,
            pub rule: RuleObject,
        }

        impl GridBuilder {
            pub fn new() -> Self {
                let (_, rule) = RuleObject::parse("[ A && B ] <C>").unwrap();
                Self {
                    inner: Grid::default(),
                    rule,
                }
            }

            pub fn update_a(&mut self, a: A) {
                self.inner.a = a;
            }
            pub fn update_b(&mut self, b: B) {
                self.inner.b = b;
            }
            pub fn update_c(&mut self, c: C) {
                self.inner.c = c;
            }

            pub fn build(self) -> Grid {
                self.inner
            }
        }

        #[derive(Debug, Default)]
        struct Grid {
            pub a: A,
            pub b: B,
            pub c: C,
        }

        #[derive(Debug, Default)]
        struct A(u8);

        impl Parser for A {
            fn parse(i: &str) -> IResult<&str, Self> {
                map(preceded(multispace0, digit1), |s: &str| {
                    Self(str::parse::<u8>(s).unwrap())
                })(i)
            }
        }

        #[derive(Debug, Default)]
        struct B(String);

        impl Parser for B {
            fn parse(i: &str) -> IResult<&str, Self> {
                map(preceded(multispace0, alpha1), |s: &str| Self(s.to_string()))(i)
            }
        }

        #[derive(Debug, Default)]
        struct C(u8);

        impl Parser for C {
            fn parse(i: &str) -> IResult<&str, Self> {
                map(preceded(multispace0, digit1), |s: &str| {
                    Self(str::parse::<u8>(s).unwrap())
                })(i)
            }
        }

        assert_eq!(
            parse_double_bar("[ auto-flow && dense ] <grid-auto-rows>"),
            Ok((
                "",
                RuleObject::Item(
                    Box::new(RuleObject::Brackets(
                        Box::new(RuleObject::DoubleAmpersand(
                            Box::new(RuleObject::Symbol("auto-flow".into(), None)),
                            Some(Box::new(RuleObject::Symbol("dense".into(), None)))
                        )),
                        None
                    )),
                    Some(Box::new(RuleObject::Variable(
                        "grid-auto-rows".into(),
                        None
                    )))
                )
            ))
        );
    }

    #[test]
    fn test_parse_double_bar() {
        assert_eq!(
            parse_double_bar("auto-flow || bold && dense || big"),
            Ok((
                "",
                RuleObject::DoubleBar(
                    Box::new(RuleObject::Symbol("auto-flow".into(), None)),
                    Some(Box::new(RuleObject::DoubleBar(
                        Box::new(RuleObject::DoubleAmpersand(
                            Box::new(RuleObject::Symbol("bold".into(), None)),
                            Some(Box::new(RuleObject::Symbol("dense".into(), None))),
                        )),
                        Some(Box::new(RuleObject::Symbol("big".into(), None))),
                    )))
                )
            ))
        );
    }
}
