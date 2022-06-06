use nom::branch::alt;
use nom::bytes::streaming::tag;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::number::complete::float;
use nom::sequence::tuple;
use nom::IResult;

use crate::parse::nom_char;
use crate::parse::nom_u8;
use crate::parse::Parser;

// normalized

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Default for Color {
    fn default() -> Self {
        Self::BLACK
    }
}

macro_rules! rgb {
    ($red: expr, $green: expr, $blue: expr) => {
        Color {
            red: $red,
            green: $green,
            blue: $blue,
            alpha: 255,
        }
    };
}

impl Color {
    pub const RED: Self = rgb!(255, 0, 0);

    pub const GLEEN: Self = rgb!(0, 255, 0);

    pub const BLUE: Self = rgb!(0, 0, 255);

    pub const YELLOW: Self = rgb!(255, 255, 0);

    pub const WHITE: Self = rgb!(255, 255, 255);

    pub const BLACK: Self = rgb!(0, 0, 0);
}

impl Color {
    #[inline(always)]
    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: 255,
        }
    }

    #[inline(always)]
    pub fn rgba(red: u8, green: u8, blue: u8, alpha: f32) -> Color {
        Self {
            red,
            green,
            blue,
            alpha: (alpha * 255.0) as u8,
        }
    }
}

impl Color {
    /// Returns the red channel in a floating point number form, from 0 to 1.
    #[inline]
    pub fn red_f32(&self) -> f32 {
        self.red as f32 / 255.0
    }

    /// Returns the green channel in a floating point number form, from 0 to 1.
    #[inline]
    pub fn green_f32(&self) -> f32 {
        self.green as f32 / 255.0
    }

    /// Returns the blue channel in a floating point number form, from 0 to 1.
    #[inline]
    pub fn blue_f32(&self) -> f32 {
        self.blue as f32 / 255.0
    }

    /// Returns the alpha channel in a floating point number form, from 0 to 1.
    #[inline]
    pub fn alpha_f32(&self) -> f32 {
        self.alpha as f32 / 255.0
    }
}

impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        [
            self.red_f32(),
            self.blue_f32(),
            self.green_f32(),
            self.alpha_f32(),
        ]
    }
}

// /// 从字符串中获取Rgba
// /// 1. 以'#'开头, 后面是 3个或4个 8位的十六进制数字, 如: "#12345678"
// /// 2. 4个u8的数组或元组, 如: "[255, 0, 0, 255]"
// ///
// impl From<&str> for Color {
//     fn from(s: &str) -> Self {
//         // 以 "#" 开头, 3个 十六进制数字
//         if s.len() == 7 && s.starts_with('#') {
//             let r = u8::from_str_radix(&s[1..3], 16).unwrap_or(0);
//             let g = u8::from_str_radix(&s[3..5], 16).unwrap_or(0);
//             let b = u8::from_str_radix(&s[5..7], 16).unwrap_or(0);
//             return Self::rgb(r, g, b);
//         }

//         // 以 "#" 开头, 4个 十六进制数字
//         if s.len() == 9 && s.starts_with('#') {
//             let r = u8::from_str_radix(&s[1..3], 16).unwrap_or(0);
//             let g = u8::from_str_radix(&s[3..5], 16).unwrap_or(0);
//             let b = u8::from_str_radix(&s[5..7], 16).unwrap_or(0);
//             let a = u8::from_str_radix(&s[7..9], 16).unwrap_or(255);
//             return Self::rgba(r, g, b, a);
//         }

//         // 4个u8的数组或元组
//         let s = s.chars().filter(|c| !c.is_whitespace()).collect::<String>();
//         if s.starts_with('(') && s.starts_with(')') || s.starts_with('[') && s.starts_with(']') {
//             let s = &s[1..s.len() - 1];
//             let s = s.split(',').collect::<Vec<_>>();
//             if s.len() != 4 {
//                 return Self::default();
//             }
//             let r = u8::from_str_radix(s[0], 16).unwrap_or(255);
//             let g = u8::from_str_radix(s[1], 16).unwrap_or(255);
//             let b = u8::from_str_radix(s[2], 16).unwrap_or(255);
//             let a = u8::from_str_radix(s[3], 16).unwrap_or(255);
//             return Self::rgba(r, g, b, a);
//         }
//         Self::default()
//     }
// }

impl Parser for Color {
    /// rgb(255, 255 , 255)
    /// rgba(255, 255, 255, 1.0)
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((
            map(
                tuple((
                    tag("rgb("),
                    multispace0,
                    nom_u8,
                    multispace0,
                    nom_char(','),
                    multispace0,
                    nom_u8,
                    multispace0,
                    nom_char(','),
                    multispace0,
                    nom_u8,
                    multispace0,
                    nom_char(')'),
                )),
                |(_, _, r, _, _, _, g, _, _, _, b, _, _)| Color::rgb(r, g, b),
            ),
            map(
                tuple((
                    tag("rgba("),
                    multispace0,
                    nom_u8,
                    multispace0,
                    nom_char(','),
                    multispace0,
                    nom_u8,
                    multispace0,
                    nom_char(','),
                    multispace0,
                    nom_u8,
                    multispace0,
                    nom_char(','),
                    multispace0,
                    float,
                    multispace0,
                    nom_char(')'),
                )),
                |(_, _, r, _, _, _, g, _, _, _, b, _, _, _, a, _, _)| Color::rgba(r, g, b, a),
            ),
        ))(i)
    }
}

/// 从u32数据中获取Rgba
/// 如: 1.00000ff
impl From<u32> for Color {
    fn from(n: u32) -> Self {
        Self {
            red: (n >> 24 & 255) as u8,
            green: (n >> 16 & 255) as u8,
            blue: (n >> 8 & 255) as u8,
            alpha: (n & 255) as u8,
        }
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for Color {}

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for Color {}
