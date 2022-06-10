use css_parser_macro::CssCodec;
use crate::utils::nom::*;

use super::Position;

#[derive(Debug, Default, PartialEq, CssCodec)]
pub struct BackgroundPosition {
    pub position: Position,
}
