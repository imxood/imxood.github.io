mod length;
pub use length::*;

mod width;
pub use width::*;

use crate::parse::CssCodec;
use crate::utils::nom::*;
use css_parser_macro::define_property_list;

define_property_list!([width, "<length>", "enum"]);
