use crate::utils::nom::*;
use css_parser_macro::CssCodec;

use super::{Length, Percentage};

#[derive(Debug, PartialEq, CssCodec)]
pub enum Width {
    Length(Length),
    Percentage(Percentage),
}

impl Width {
    pub fn new() -> Self {
        Self::Length(Length::Px(0.0))
    }
}
