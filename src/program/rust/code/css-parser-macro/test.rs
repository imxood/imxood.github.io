#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod parse {
    pub mod parse {}
    mod traits {}
}
mod properties {
    mod length {
        use crate::utils::nom::*;
        use super::CssCodec;
        pub type Float = f32;
        /// 每一个像素 app units 的数量
        pub const AU_PER_PX: Float = 60.;
        /// 每一寸 app units 的数量
        pub const AU_PER_IN: Float = AU_PER_PX * 96.;
        /// 每一厘米 app units 的数量
        pub const AU_PER_CM: Float = AU_PER_IN / 2.54;
        /// 每一毫米 app units 的数量
        pub const AU_PER_MM: Float = AU_PER_IN / 25.4;
        /// 精确长度
        pub enum Length {
            /// 像素
            Px(Float),
            /// 寸
            In(Float),
            /// 厘米
            Cm(Float),
            /// 毫米
            Mm(Float),
            /// 相对于父元素的倍数
            Em(Float),
            /// 相对于根元素的倍数
            Rem(Float),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Length {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Length::Px(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Px",
                            &__self_0,
                        )
                    }
                    Length::In(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "In",
                            &__self_0,
                        )
                    }
                    Length::Cm(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Cm",
                            &__self_0,
                        )
                    }
                    Length::Mm(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Mm",
                            &__self_0,
                        )
                    }
                    Length::Em(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Em",
                            &__self_0,
                        )
                    }
                    Length::Rem(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Rem",
                            &__self_0,
                        )
                    }
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for Length {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Length {
            #[inline]
            fn eq(&self, other: &Length) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (Length::Px(__self_0), Length::Px(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        (Length::In(__self_0), Length::In(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        (Length::Cm(__self_0), Length::Cm(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        (Length::Mm(__self_0), Length::Mm(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        (Length::Em(__self_0), Length::Em(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        (Length::Rem(__self_0), Length::Rem(__arg1_0)) => {
                            *__self_0 == *__arg1_0
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Length {
            #[inline]
            fn clone(&self) -> Length {
                let _: ::core::clone::AssertParamIsClone<Float>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Length {}
        impl CssCodec for Length {
            fn parse(i: &str) -> IResult<&str, Self> {
                let (i, num) = preceded(multispace0, digit1)(i)?;
                let num = str::parse::<Float>(num).unwrap();
                if i.is_empty() {
                    return Ok((i, Self::Px(num)));
                }
                let v = alt((
                    map(tag("px"), |_| Self::Px(num)),
                    map(tag("in"), |_| Self::In(num)),
                    map(tag("cm"), |_| Self::Cm(num)),
                    map(tag("mm"), |_| Self::Mm(num)),
                    map(tag("em"), |_| Self::Em(num)),
                    map(tag("rem"), |_| Self::Rem(num)),
                ))(i)?;
                Ok(v)
            }
            fn to_css<W>(&self, dest: &mut W) -> core::fmt::Result
            where
                W: core::fmt::Write,
            {
                match self {
                    Length::Px(v) => {
                        dest
                            .write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &["", "px"],
                                    &[::core::fmt::ArgumentV1::new_display(&v)],
                                ),
                            )
                    }
                    Length::In(v) => {
                        dest
                            .write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &["", "in"],
                                    &[::core::fmt::ArgumentV1::new_display(&v)],
                                ),
                            )
                    }
                    Length::Cm(v) => {
                        dest
                            .write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &["", "cm"],
                                    &[::core::fmt::ArgumentV1::new_display(&v)],
                                ),
                            )
                    }
                    Length::Mm(v) => {
                        dest
                            .write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &["", "mm"],
                                    &[::core::fmt::ArgumentV1::new_display(&v)],
                                ),
                            )
                    }
                    Length::Em(v) => {
                        dest
                            .write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &["", "em"],
                                    &[::core::fmt::ArgumentV1::new_display(&v)],
                                ),
                            )
                    }
                    Length::Rem(v) => {
                        dest
                            .write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &["", "rem"],
                                    &[::core::fmt::ArgumentV1::new_display(&v)],
                                ),
                            )
                    }
                }
            }
        }
    }
    pub use length::*;
    mod width {
        use crate::utils::nom::*;
        use css_parser_macro::CssCodec;
        use super::Length;
        pub enum Width {
            Length(Length),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Width {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Width::Length(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Length",
                            &__self_0,
                        )
                    }
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for Width {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Width {
            #[inline]
            fn eq(&self, other: &Width) -> bool {
                match (self, other) {
                    (Width::Length(__self_0), Width::Length(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                }
            }
        }
        impl crate::properties::CssCodec for Width {
            fn parse(input: &str) -> IResult<&str, Self> {
                crate::properties::parse_width(input)
            }
            fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
                match self {
                    Self::Length(v) => v.to_css(dest),
                }
            }
        }
        impl Width {
            pub fn new() -> Self {
                Self::Length(Length::Px(0.0))
            }
        }
    }
    pub use width::*;
    use crate::utils::nom::*;
    use css_parser_macro::define_property_list;
    pub enum Property {
        Width(Width),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Property {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Property::Width(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Width",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for Property {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Property {
        #[inline]
        fn eq(&self, other: &Property) -> bool {
            match (self, other) {
                (Property::Width(__self_0), Property::Width(__arg1_0)) => {
                    *__self_0 == *__arg1_0
                }
            }
        }
    }
    impl CssCodec for Property {
        fn parse(i: &str) -> nom::IResult<&str, Self> {
            let (i, i0) = nom::bytes::complete::is_not(";}")(i)?;
            let (_, (name, value)) = nom::sequence::separated_pair(
                skip_useless(
                    nom::bytes::complete::take_till1(|c: char| {
                        !c.is_alphanumeric() && c != '-'
                    }),
                ),
                skip_useless(nom_char(':')),
                skip_useless(nom::bytes::complete::take_till1(|c: char| c == ';')),
            )(i0)?;
            let (_, property) = match name {
                "width" => {
                    nom::combinator::map(Width::parse, |width| Self::Width(width))(value)
                }
                _ => {
                    ::core::panicking::panic_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[
                                "\u{89e3}\u{6790}\u{5c5e}\u{6027}\u{5931}\u{8d25}, property name: ",
                            ],
                            &[::core::fmt::ArgumentV1::new_display(&name)],
                        ),
                    )
                }
            }?;
            Ok((i, property))
        }
        fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
            match self {
                Self::Width(width) => {
                    dest.write_str("width: ")?;
                    width.to_css(dest)?;
                    dest.write_char(';')
                }
            }
        }
    }
    pub fn parse_width(i: &str) -> IResult<&str, Width> {
        map(Length::parse, |length| Width::Length(length))(i)
    }
    pub fn width(i: &str) -> Option<Width> {
        let (_, v) = opt(Width::parse)(i).unwrap_or_default();
        v
    }
    pub trait CssCodec<T = Self> {
        fn parse(i: &str) -> nom::IResult<&str, T>;
        fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result;
        #[inline]
        fn to_css_string(&self) -> String {
            let mut s = String::new();
            self.to_css(&mut s).unwrap();
            s
        }
    }
}
mod utils {
    pub mod nom {
        pub use nom::{
            branch::alt, bytes::complete::{is_not, tag, take_till1},
            character::complete::{digit1, multispace0, none_of},
            combinator::{all_consuming, map, opt, peek},
            multi::{many0, separated_list1},
            sequence::{delimited, pair, preceded, terminated, tuple},
            IResult, Parser as NomParser,
        };
        pub use nom::character::complete::{char as nom_char, u8 as nom_u8};
        use nom::{
            bytes::complete::take_until, character::complete::multispace1,
            error::{Error as IError, ParseError},
        };
        /// 解析 空符号 和 注释
        pub fn parse_useless(i: &str) -> IResult<&str, Vec<&str>> {
            many0(alt((multispace1, parse_comment)))(i)
        }
        /// 过滤 ' ' 和 '\t'
        #[inline]
        pub fn skip_sp<'a, O, P, E: ParseError<&'a str>>(
            parser: P,
        ) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
        where
            P: nom::Parser<&'a str, O, E>,
        {
            preceded(multispace0, parser)
        }
        /// 解析注释
        pub fn parse_comment(i: &str) -> IResult<&str, &str> {
            map(tuple((tag("/*"), take_until("*/"), tag("*/"))), |(_, text, _)| text)(i)
        }
        /// 给 输入的解析器 去除空格 和 注释
        pub fn skip_useless<'a, O, P>(
            parser: P,
        ) -> impl FnMut(&'a str) -> IResult<&'a str, O>
        where
            P: nom::Parser<&'a str, O, IError<&'a str>>,
        {
            map(tuple((parse_useless, parser, parse_useless)), |(_, s, _)| s)
        }
    }
}
use crate::properties::CssCodec;
use properties::{parse_width, Width};
fn main() {
    let width = Width::new();
    width.to_css_string();
    parse_width("100").unwrap();
}
