use css_parser_macro::CssCodec;
use crate::utils::nom::*;

use super::Color;

#[derive(Debug, Default, PartialEq, CssCodec)]
pub struct BackgroundColor {
    pub color: Color,
}
