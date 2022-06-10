use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn derive(input: syn::DeriveInput) -> proc_macro::TokenStream {
    let ident = &input.ident;

    let mut tokenstream = TokenStream::new();

    /*
        CssCodec::to_css
    */

    let tokens = match &input.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
            ..
        }) => {
            let mut tokens = TokenStream::new();
            let len = named.len();
            eprintln!("len: {:?}", &len);

            named.iter().enumerate().for_each(|(i, f)| {
                eprintln!("i: {:?}", &i);
                let ident = f.ident.as_ref();
                tokens.extend(quote! {
                    self.#ident.to_css(dest)?;
                });
                if i == len - 1 {
                    tokens.extend(quote! {
                        dest.write_char(' ')?;
                    });
                }
            });
            quote! {
                #tokens
                Ok(())
            }
        }
        syn::Data::Enum(syn::DataEnum { ref variants, .. }) => {
            let mut tokens = TokenStream::new();
            variants.iter().for_each(|f| {
                let ident = f.ident.clone();
                tokens.extend(quote! {
                    Self::#ident(v) => v.to_css(dest),
                });
            });
            quote! {
                match self {
                    #tokens
                }
            }
        }
        _ => panic!("CssCodec derive panic"),
    };

    let to_css_func = quote!(
        fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
            #tokens
        }
    );

    /*
        CssCodec::parse
    */

    let parse_func_name = format_ident!("parse_{}", ident.to_string().to_case(Case::Snake));
    let parse_func = quote!(
        fn parse(input: &str) -> IResult<&str, Self> {
            crate::properties::#parse_func_name(input)
        }
    );
    tokenstream.extend(quote! {
        impl crate::properties::CssCodec for #ident {
            #parse_func
            #to_css_func
        }
    });

    tokenstream.into()
}
