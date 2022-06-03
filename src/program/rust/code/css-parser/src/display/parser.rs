use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::{delimited, preceded, tuple};
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

// 优先级: 数量符号 > 组合 [] > 与 "&&" > 或 "||" > 互斥 "|"

#[derive(Debug)]
pub enum RuleObject {
    // 方括号, 表示 多个规则的组合
    // bold [ thin && <length> ]
    Brackets(Box<RuleObject>, Option<QuantitySymbol>),

    // 双引号, 各个选项必须出现, 但是顺序任意
    // 如: bold && <length>:
    DoubleAmpersand(Vec<Box<RuleObject>>),

    // 双杠, 至少一个 最多不超过 双杠参数值 的数量, 顺序任意
    // 如: A || B || C， 可以是 A, A B, B C A 等
    DoubleBar(Vec<Box<RuleObject>>),

    // 单杠, 只能取 这些选项中的一个
    // 如: A | B | C， 只能是 A 或 B 或 C
    SingleBar(Vec<Box<RuleObject>>),

    // 表示 一个特定的属性类型 的值
    Variable(String, Option<QuantitySymbol>),

    // 符号, 如: auto-flow
    Symbol(String, Option<QuantitySymbol>),
}

#[derive(Debug)]
pub enum QuantitySymbol {
    // ? 0次或1次
    QuestionMark,
    // * 0次或多次
    Asterisk,
    // + 1次或多次
    Plus,
    // {m, n} 至少m次 最多n次
    CurlyBraces(u8, u8),
    // #  一次或多次, 与 + 功能一致, 但是多次出现必须以逗号分隔
    // 如: bold smaller#, 例子: bold smaller, smaller, smaller
    HashMark,
    // []! 方括号后面的叹号表示该组是必须的, 并且至少产生一个值, 即使组内项目全部允许忽略, 也至少保留一个
    ExclamationPoint,
}

// impl Parser for QuantitySymbol {
//     fn parse(i: &str) -> IResult<&str, Self> {
//         preceded(multispace0, second)(i)
//     }
// }

// <grid-template> | <grid-template-rows> / [ auto-flow && dense? ] <grid-auto-columns>?
impl Parser for RuleObject {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((
            parse_brackets,
            parse_double_ampersand,
            parse_double_bar,
            parse_single_bar,
            parse_variable,
            parse_string,
        ))(i)
    }
}

// pub fn parse_rule_object(i: &str) -> IResult<&str, RuleObject> {
//     let (i, obj) =
// }

// 解析 方括号
fn parse_brackets(i: &str) -> IResult<&str, RuleObject> {
    delimited(
        multispace0,
        map(
            delimited(nom_char('['), RuleObject::parse, nom_char(']')),
            |obj| RuleObject::Brackets(Box::new(obj), None),
        ),
        multispace0,
    )(i)
}

pub fn parse_operation(i: &str) -> IResult<&str, RuleObject> {
    alt((parse_brackets, parse_variable, parse_string))(i)
    // parse_double_ampersand,
    // parse_double_bar,
    // parse_single_bar,
    // parse_variable,
    // parse_string,
}

// 解析 双引号
fn parse_double_ampersand(i: &str) -> IResult<&str, RuleObject> {
    let (i, obj1) = alt((parse_brackets, parse_variable, parse_string))(i)?;

    let (i, obj2) = delimited(
        multispace0,
        many0(preceded(
            tag("&&"),
            // 解析优先级更高的选项, 若失败, 则自动降级 解析, 直到
            parse_double_ampersand,
        )),
        multispace0,
    )(i)?;

    let mut objs = vec![Box::new(obj1)];
    for obj in obj2 {
        objs.push(Box::new(obj));
    }

    Ok((i, RuleObject::DoubleAmpersand(objs)))
}

// 解析 双杠
// A || B || C || D ... E ... F
fn parse_double_bar(i: &str) -> IResult<&str, RuleObject> {
    let i = i.trim_start();
    let (i, input) = preceded(nom_char('['), skip_sp(take_till1(|c| c == ']')))(i)?;
    RuleObject::parse(i)
}

// 解析 单杠
fn parse_single_bar(i: &str) -> IResult<&str, RuleObject> {
    let i = i.trim_start();
    let (i, input) = preceded(nom_char('['), skip_sp(take_till1(|c| c == ']')))(i)?;
    RuleObject::parse(i)
}

// 解析 可变的值
fn parse_variable(i: &str) -> IResult<&str, RuleObject> {
    // e.g., <background_color>
    delimited(
        multispace0,
        map(
            delimited(
                nom_char('<'),
                take_till1(|c: char| !c.is_alphanumeric() || c != '_'),
                nom_char('>'),
            ),
            |i: &str| RuleObject::Variable(i.to_string(), None),
        ),
        multispace0,
    )(i)
}

// 解析 符号
// 字母 数字 和 下划线 组成的字符串
fn parse_string(i: &str) -> IResult<&str, RuleObject> {
    // e.g., Bold
    delimited(
        multispace0,
        map(
            take_till1(|c: char| !c.is_alphanumeric() && c != '_'),
            |i: &str| RuleObject::Variable(i.to_string(), None),
        ),
        multispace0,
    )(i)
}

mod test {
    use super::parse_double_ampersand;

    #[test]
    fn test_parse_double_ampersand() {
        println!("{:#?}", parse_double_ampersand("A && B && C && D").unwrap());
    }
}
