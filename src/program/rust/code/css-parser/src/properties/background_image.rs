use css_parser_macro::CssCodec;
use crate::utils::nom::*;

use super::Image;

#[derive(Debug, Default, PartialEq, CssCodec)]
pub struct BackgroundImage {
    pub image: Image,
}