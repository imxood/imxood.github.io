mod color;
pub use color::*;

mod length;
pub use length::*;

mod percentage;
pub use percentage::*;

mod image;
pub use image::*;

mod width;
pub use width::*;

mod height;
pub use height::*;

mod position;
pub use position::*;

mod background;
pub use background::*;

mod background_color;
pub use background_color::*;

mod background_image;
pub use background_image::*;

mod background_position;
pub use background_position::*;

use crate::utils::nom::*;
use css_parser_macro::define_property_list;

define_property_list!(
    [width, "<length> | <percentage>", "enum"],
    [height, "<length> | <percentage>", "enum"],
    [
        background,
        "[<background-color> | <background-image>] <background-position>?",
        "struct"
    ],
    [background_color, "<color>", "struct"],
    [background_image, "<image>", "struct"],
    [background_position, "<position>", "struct"],
);

pub trait ToComputedValue {
    type ComputedValue;

    fn to_computed_value(&self) -> Self::ComputedValue;
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
