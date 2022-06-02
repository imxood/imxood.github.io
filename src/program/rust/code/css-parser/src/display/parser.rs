use nom::sequence::preceded;
use nom::IResult;
use nom::{branch::alt, bytes::complete::take_till1};

use crate::parse::{nom_char, skip_sp, Parser};

use super::flex::FlexDirection;

// pub fn parse_flex_direction(i: &str) -> IResult<&str, FlexDirection> {

// }

// grid: <'grid-template'> | <'grid-template-rows'> / [ auto-flow && dense? ] <'grid-auto-columns'>? | [ auto-flow && dense? ] <'grid-auto-rows'>? / <'grid-template-columns'>
// grid-template: none | [ <'grid-template-rows'> / <'grid-template-columns'> ] | [ <line-names>? <string> <track-size>? <line-names>? ]+ [ / <explicit-track-list> ]?
// grid-template-rows:

// /* [ auto-flow && dense? ] <'grid-auto-rows'>? /
//    <'grid-template-columns'> values */
//    grid: auto-flow / 200px;
//    grid: auto-flow dense / 30%;
//    grid: auto-flow 300px / repeat(3, [line1 line2 line3] 200px);
//    grid: auto-flow dense 40% / [line1] minmax(20em, max-content);

// 优先级: 数量符号 > 组合 [] > 并置 "多个空格" > 与 "&&" > 或 "||" > 互斥 "|"

// 解析过程是: 1. 解析优先级 遍历每一个优先级, 用每一个优先级去 目标中 查询 是否存在匹配, 若匹配到了, 则解析, 对应优先级下的数据,

pub enum RuleObject {
    // 方括号, 表示 多个规则的集合
    // bold [ thin && <length> ]
    Brackets,
    /// 并置, 一个或多个空格, 必须按照特定的顺序放置
    /// 如: bold <length> , thin 可以是: bold 100px , thin, 顺序不能颠倒
    Juxtaposition,
    // 各个部分必须出现, 但是顺序任意
    // 如: bold && <length>:
    DoubleAmpersand,
    // 双杠, 至少一个 最多不超过 双杠参数值 的数量, 顺序任意
    // 如: A || B || C， 可以是 A, A B, B C A 等
    DoubleBar(Vec<RuleObject>),
    // 单杠, 只能取 这些选项中的一个
    // 如: A | B | C， 只能是 A 或 B 或 C
    SingleBar(Vec<RuleObject>),
    // 可变的值
    Variable(Variable),
    // auto-flow
    Symbol(String),
    // [ auto-flow && dense? ]
    Combinator(Vec<RuleObject>),
}
// <'grid-template'> | <'grid-template-rows'> / [ auto-flow && dense? ] <'grid-auto-columns'>?
impl Parser for RuleObject {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt(())
    }
}

// 解析方括号
fn parse_brackets(i: &str) -> IResult<&str, RuleObject> {
    let i = i.trim_start();
    let (i, input) = preceded(nom_char('['), skip_sp(take_till1(|c| c == ']')))(i)?;
    RuleObject::parse(i)
}

// 解析并置, 一个或多个括号
fn parse_juxtaposition(i: &str) -> IResult<&str, RuleObject> {
    let i = i.trim_start();
    let (i, input) = preceded(nom_char('['), skip_sp(take_till1(|c| c == ']')))(i)?;
    RuleObject::parse(i)
}

pub enum Variable {
    // 有尖括号 不带单引号 的类型
    // 如: <track-size>
    Type,
    // 有尖括号 且带 单引号, 表示 一个特定的属性类型 的值
    // 如: <'grid-template'>
    PropType,
}

pub enum QuantitySymbol {
    // ? 0次或1次
    QuestionMark(Option<RuleObject>),
    // * 0次或多次
    Asterisk(Vec<RuleObject>),
    // + 1次或多次
    Plus(Vec<RuleObject>),
    // {m, n} 至少m次 最多n次
    CurlyBraces(Vec<RuleObject>),
    // #  一次或多次, 与 + 功能一致, 但是多次出现必须以逗号分隔
    // 如: bold smaller#, 例子: bold smaller, smaller, smaller
    HashMark(Vec<RuleObject>),
    // []! 方括号后面的叹号表示该组是必须的, 并且至少产生一个值, 即使组内项目全部允许忽略, 也至少保留一个
    ExclamationPoint(Vec<RuleObject>),
}
