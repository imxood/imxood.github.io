use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_while_m_n},
    character::complete::{alphanumeric1, digit1, one_of, none_of},
    combinator::{map, map_res},
    sequence::{preceded, tuple},
    AsChar, IResult, error::ErrorKind,
};

#[derive(Debug, Default)]
struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    fn new(red: u8, green: u8, blue: u8) -> Color {
        Self { red, green, blue }
    }
    pub fn parse<'a>(&mut self, i: &'a str) -> IResult<&'a str, ()> {
        map(parse_rgb, |color| {
            *self = color;
        })(i)
    }
}

fn from_hex(i: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(i, 16)
}

fn is_hex_num(c: char) -> bool {
    c.is_hex_digit()
}

/// 把输入字符串, 取出2个符号后, 使用 is_hex_num 判断这两个符号 是否是十六进制数, 是的话, 传递给 from_hex, 转换成u8类型的数据
fn hex_single(i: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_num), from_hex)(i)
}

fn parse_rgb(i: &str) -> IResult<&str, Color> {
    // 识别 "#"
    let (i, _) = tag("#")(i)?;

    // 使用 tuple, 上一个 hex_single 的结果的 input 会作为下一个 hex_single 的 input
    // 每一个 hex_single 会获取2个十六进制字符, 并转换为u8数据
    // 把 3个u8数据
    let (input, (red, green, blue)) = tuple((hex_single, hex_single, hex_single))(i)?;
    Ok((input, Color::new(red, green, blue)))
}

fn parse_str(i: &str) -> IResult<&str, &str> {
    println!("parse_str i: {:?}", i);
    preceded(tag("#"), alphanumeric1)(i)
}

fn main() {
    println!("{:?}", parse_rgb("#FF0000"));
    let mut color = Color::default();
    println!("{:?}", color.parse("#ffssff"));
    println!("color: {:?}", &color);

    alt((
        map(parse_rgb, |color| {
            println!("color: {:?}", &color);
        }),
        map(parse_str, |s| {
            println!("s: {:?}", s);
        }),
    ))("#ffssff")
    .unwrap();
}

#[test]
fn test_none_of() {
    println!("{:?}", none_of::<_, _, (&str, ErrorKind)>("abc")("cyas"));
}