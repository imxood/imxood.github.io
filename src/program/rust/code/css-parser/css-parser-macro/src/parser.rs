use convert_case::{Case, Casing};
use nom::IResult;
use proc_macro::TokenStream;
use quote::{quote, format_ident};

pub fn derive(input: syn::DeriveInput) -> TokenStream {
    let ident = input.ident;
    let parse_fun = format_ident!("parse_{}", ident.to_string().to_case(Case::Snake));
    quote!(
        impl crate::parse::Parser for #ident {
            fn parse(input: &str) -> IResult<&str, Self> {
                css_parser::properties::#parse_fun(input)
            }
        }
    )
    .into()
}
