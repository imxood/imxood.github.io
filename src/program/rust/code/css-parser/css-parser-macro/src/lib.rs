use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod parse;
mod parser;
mod properties;
mod rule;
mod to_css;
mod utils;

#[proc_macro]
pub fn define_property_list(input: TokenStream) -> TokenStream {
    properties::derive(input).into()
}

#[proc_macro_derive(ToCss)]
pub fn derive_to_css(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    to_css::derive(input).into()
}

#[proc_macro_derive(Parser)]
pub fn derive_parser(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    parser::derive(input).into()
}
