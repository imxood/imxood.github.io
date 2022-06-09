use proc_macro2::TokenStream;
use quote::quote;

pub fn derive(input: syn::DeriveInput) -> proc_macro::TokenStream {
    let ident = input.ident.clone();

    let mut tokenstream = TokenStream::new();
    let mut tokens = TokenStream::new();

    match &input.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
            ..
        }) => {
            let len = named.len();
            eprintln!("len: {:?}", &len);

            named.iter().enumerate().for_each(|(i, f)| {
                eprintln!("i: {:?}", &i);
                let ident = f.ident.as_ref();
                if i == len - 1 {
                    tokens.extend(quote! {
                        self.#ident.to_css(dest)?;
                        Ok(())
                    });
                } else {
                    tokens.extend(quote! {
                        self.#ident.to_css(dest)?;
                        dest.write_char(' ')?;
                    });
                }
            });
            tokenstream.extend(quote! {
                match self {
                    #tokens
                }
            });
            eprintln!("tokenstream: {:?}", tokenstream.to_string());
        }
        syn::Data::Enum(syn::DataEnum { ref variants, .. }) => {
            variants.iter().for_each(|f| {
                let ident = f.ident.clone();
                tokens.extend(quote! {
                    Self::#ident(v) => v.to_css(dest),
                });
            });
            tokenstream.extend(tokens);
        }
        _ => panic!("ToCss derive panic"),
    }

    quote!(
        impl crate::serialize::ToCss for #ident {
            fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result {
                #tokenstream
            }
        }
    )
    .into()
}
