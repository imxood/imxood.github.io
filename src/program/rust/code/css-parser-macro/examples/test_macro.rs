mod parse;
mod properties;
mod utils;

use crate::properties::CssCodec;
use properties::{parse_width, Width};

fn main() {
    let width = Width::new();
    width.to_css_string();
    parse_width("100").unwrap();
}
