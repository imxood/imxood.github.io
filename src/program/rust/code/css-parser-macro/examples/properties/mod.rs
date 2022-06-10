mod length;
pub use length::*;

mod width;
pub use width::*;

use crate::utils::nom::*;
use css_parser_macro::define_property_list;

define_property_list!([width, "<length>", "enum"]);

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
