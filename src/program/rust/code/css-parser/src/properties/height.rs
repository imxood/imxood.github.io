use css_parser_macro::CssCodec;

use super::{Length, Percentage};
use crate::utils::nom::*;

#[derive(Debug, PartialEq, CssCodec)]
pub enum Height {
    Length(Length),
    Percentage(Percentage),
}
