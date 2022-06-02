use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_till1, take_while, take_while1, take_while_m_n},
    character::complete::{alphanumeric1, multispace0, one_of},
    combinator::{map, opt, peek, value},
    error::{context, ParseError},
    multi::separated_list0,
    number::complete::double,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
pub enum JsonValue {
    Null,
    Str(String),
    Boolean(bool),
    Num(f64),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

fn main() {
    let json_data = include_str!("./simple.json");
    let (_i, json) = parse_json(json_data).unwrap();
    println!("{:#?}", json);
}

fn parse_json(i: &str) -> IResult<&str, JsonValue> {
    delimited(
        multispace0,
        alt((
            map(parse_object, JsonValue::Object),
            map(parse_array, JsonValue::Array),
        )),
        multispace0,
    )(i)
}

/// 一个 nom解析器 的原型是这样的:
/// `Input -> IResult<Input, Output, Error>`, with `IResult` defined as:
/// `type IResult<I, O, E = (I, ErrorKind)> = Result<(I, O), Err<E>>;`
///
/// 由于输入和输出 有相同的生命周期, 这产生的值 实际上是 输入数据 的子切片, 没有内存分配, 这是 nom 性能高的一个原因
fn parse_string(i: &str) -> IResult<&str, &str> {
    println!("parse string, raw str: {:?}", i);
    let parse_str = escaped(
        take_till1(|c: char| c == '\\' || c == '"' || c.is_ascii_control()),
        '\\',
        alt((
            tag("\""),
            tag("\\"),
            tag("/"),
            tag("b"),
            tag("f"),
            tag("n"),
            tag("n"),
            tag("r"),
            tag("t"),
            parse_hex,
        )),
    );
    // 此处 两个引号 表示: 如果 是空字符串 就直接返回了, 如果非空, 则继续解析
    alt((tag("\"\""), delimited(tag("\""), parse_str, tag("\""))))(i)
}

fn parse_hex(i: &str) -> IResult<&str, &str> {
    context(
        "hex string",
        preceded(
            peek(tag("u")),
            take_while_m_n(5, 5, |c: char| c.is_ascii_hexdigit() || c == 'u'),
        ),
    )(i)
}

fn parse_boolean(i: &str) -> IResult<&str, bool> {
    let parse_true = value(true, tag("true"));
    let parse_false = value(false, tag("false"));
    context("parse boolean", alt((parse_true, parse_false)))(i)
}

fn parse_null(i: &str) -> IResult<&str, JsonValue> {
    context("parse null", map(tag("null"), |_| JsonValue::Null))(i)
}

fn parse_array(i: &str) -> IResult<&str, Vec<JsonValue>> {
    delimited(
        tag("["),
        separated_list0(tag(","), delimited(multispace0, parse_value, multispace0)),
        tag("]"),
    )(i)
}

fn parse_object(i: &str) -> IResult<&str, HashMap<String, JsonValue>> {
    delimited(
        tag("{"),
        map(
            separated_list0(
                tag(","),
                separated_pair(
                    delimited(multispace0, parse_string, multispace0),
                    tag(":"),
                    delimited(multispace0, parse_value, multispace0),
                ),
            ),
            |tuple_vec: Vec<(&str, JsonValue)>| {
                tuple_vec
                    .into_iter()
                    .map(|(k, v)| (String::from(k), v))
                    .collect()
            },
        ),
        tag("}"),
    )(i)
}

fn parse_value(i: &str) -> IResult<&str, JsonValue> {
    delimited(
        multispace0,
        alt((
            map(parse_object, JsonValue::Object),
            map(parse_array, JsonValue::Array),
            map(double, |b| JsonValue::Num(b)),
            map(parse_string, |s| JsonValue::Str(s.to_string())),
            map(parse_boolean, |b| JsonValue::Boolean(b)),
            parse_null,
        )),
        multispace0,
    )(i)
}
