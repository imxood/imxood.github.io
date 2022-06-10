use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod css_codec;
mod parse;
mod properties;
mod rule;

#[proc_macro]
pub fn define_property_list(input: TokenStream) -> TokenStream {
    properties::define(input).into()
}

#[proc_macro_derive(CssCodec)]
pub fn derive_css_codec(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    css_codec::derive(input).into()
}
