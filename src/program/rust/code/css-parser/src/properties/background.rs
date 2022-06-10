use css_parser_macro::CssCodec;

use super::{BackgroundColor, BackgroundImage, BackgroundPosition};
use crate::utils::nom::*;

#[derive(Debug, Default, PartialEq, CssCodec)]
pub struct Background {
    pub background_color: BackgroundColor,
    pub background_image: BackgroundImage,
    pub background_position: BackgroundPosition,
}
