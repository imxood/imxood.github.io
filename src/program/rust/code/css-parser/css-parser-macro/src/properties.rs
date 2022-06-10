use convert_case::{Case, Casing};
use proc_macro::TokenTree;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::LitStr;

use crate::rule::{Parser, RuleObject};

pub fn define(input: proc_macro::TokenStream) -> TokenStream {
    let token_streams = input
        .into_iter()
        .filter(|item| match item {
            proc_macro::TokenTree::Group(_) => true,
            _ => false,
        })
        .map(|item| match item {
            proc_macro::TokenTree::Group(group) => group.stream(),
            _ => panic!("don't reach"),
        })
        .collect::<Vec<_>>();

    let mut token_stream = TokenStream::new();

    let prop_infos = token_streams
        .into_iter()
        .map(|input| {
            let prop_desc = PropDesc::new(input);
            prop_desc.struct_infos()
        })
        .collect::<Vec<_>>();

    /*
        enum Property
    */

    let mut tokens = TokenStream::new();
    prop_infos.iter().for_each(|prop_info| {
        let name_pascal = format_ident!("{}", prop_info.name.to_case(Case::Pascal));
        tokens.extend(quote! {
            #name_pascal(#name_pascal),
        });
    });

    token_stream.extend(quote! {
        #[derive(Debug, PartialEq)]
        pub enum Property {
            #tokens
        }
    });

    /*
        CssCodec::parse
    */

    let mut parse_func = TokenStream::new();
    let mut tokens = TokenStream::new();
    prop_infos.iter().for_each(|prop_info| {
        let name_pascal = format_ident!("{}", prop_info.name.to_case(Case::Pascal));
        let name_snake = format_ident!("{}", prop_info.name.to_case(Case::Snake));
        let name_kebab = LitStr::new(&prop_info.name.to_case(Case::Kebab), Span::call_site());

        tokens.extend(quote! {
            #name_kebab => nom::combinator::map(
                #name_pascal::parse,
                |#name_snake| Self::#name_pascal(#name_snake)
            )(value),
        });
    });

    parse_func.extend(quote! {
        fn parse(i: &str) -> nom::IResult<&str, Self> {
            let (i, i0) = nom::bytes::complete::is_not(";}")(i)?;
            let (_, (name, value)) = nom::sequence::separated_pair(
                crate::parse::parse::skip_useless(nom::bytes::complete::take_till1(|c: char| !c.is_alphanumeric() && c != '-')),
                crate::parse::parse::skip_useless(crate::parse::parse::nom_char(':')),
                crate::parse::parse::skip_useless(nom::bytes::complete::take_till1(|c: char| c == ';')),
            )(i0)?;
            let (_, property) = match name {
                #tokens
                _ => panic!("解析属性失败, property name: {}", name),
            }?;
            Ok((i, property))
        }
    });

    /*
        CssCodec::to_css
    */

    let mut to_css_func = TokenStream::new();
    let mut tokens = TokenStream::new();
    prop_infos.iter().for_each(|prop_info| {
        let name_pascal = format_ident!("{}", prop_info.name.to_case(Case::Pascal));
        let name_snake = format_ident!("{}", prop_info.name.to_case(Case::Snake));
        let name_str_kebab = LitStr::new(
            &format!("{}: ", prop_info.name.to_case(Case::Kebab)),
            Span::call_site(),
        );

        tokens.extend(quote! {
            Self::#name_pascal(#name_snake) => {
                dest.write_str(#name_str_kebab)?;
                #name_snake.to_css(dest)?;
                dest.write_char(';')
            },
        });
    });

    to_css_func.extend(quote! {
        fn to_css<W: core::fmt::Write>(&self, dest: &mut W) -> core::fmt::Result
        {
            match self {
                #tokens
            }
        }
    });

    token_stream.extend(quote! {
        impl crate::parse::CssCodec for Property {
            #parse_func
            #to_css_func
        }
    });

    /*
        生成 parse_*function*
    */

    let mut tokens = TokenStream::new();
    prop_infos.iter().for_each(|prop_info| {
        let name_pascal = format_ident!("{}", prop_info.name.to_case(Case::Pascal));
        let parse_func_snake = format_ident!("parse_{}", prop_info.name.to_case(Case::Snake));
        let parser: TokenStream = prop_info.parser.parse().unwrap();

        let token = match prop_info.prop_type.as_str() {
            "struct" => {
                quote! {
                    let mut v = #name_pascal::default();
                    let (i, _) = #parser(i)?;
                    Ok((i, v))
                }
            }
            "enum" => {
                quote! {
                    #parser(i)
                }
            }
            _ => {
                panic!("parse_*function* args wrong");
            }
        };

        tokens.extend(quote! {
            pub fn #parse_func_snake(i: &str) -> IResult<&str, #name_pascal> {
                #token
            }
        });
    });
    token_stream.extend(tokens);

    /*
        生成 基本类型的 function, 快速创建对象
    */

    let mut tokens = TokenStream::new();
    prop_infos.iter().for_each(|prop_info| {
        let name_pascal = format_ident!("{}", prop_info.name.to_case(Case::Pascal));
        let name_snake = format_ident!("{}", prop_info.name.to_case(Case::Snake));

        tokens.extend(quote! {
            pub fn #name_snake(i: &str) -> Option<#name_pascal> {
                let (_, v) = opt(#name_pascal::parse)(i).unwrap_or_default();
                v
            }
        });
    });
    token_stream.extend(tokens);

    token_stream
}

#[derive(Debug, Clone)]
struct PropDesc {
    name: String,
    desc: String,
    prop_type: String,
}

impl PropDesc {
    pub fn new(input: proc_macro::TokenStream) -> PropDesc {
        let items = input
            .into_iter()
            .filter(|item| match item {
                proc_macro::TokenTree::Ident(_) | proc_macro::TokenTree::Literal(_) => true,
                _ => false,
            })
            .collect::<Vec<_>>();
        let (name, desc, prop_type) = match (items.get(0), items.get(1), items.get(2)) {
            (
                Some(TokenTree::Ident(name)),
                Some(TokenTree::Literal(desc)),
                Some(TokenTree::Literal(prop_type)),
            ) => {
                let desc = desc.to_string();
                let desc = desc[1..desc.len() - 1].to_string();
                let prop_type = prop_type.to_string();
                let prop_type = prop_type[1..prop_type.len() - 1].to_string();
                (name.to_string(), desc, prop_type)
            }
            _ => panic!("define_property_list macro args wrong."),
        };
        // if items.len() > 3 {
        //     panic!("define_property_list macro args wrong.")
        // }
        Self {
            name,
            desc,
            prop_type,
        }
    }

    pub fn struct_infos(&self) -> PropInfo {
        let (_, rule_obj) = RuleObject::parse(&self.desc).unwrap();
        let (name, prop_type) = (self.name.clone(), self.prop_type.clone());
        let parser = rule_obj.parser(&name, &prop_type);
        let mut info = PropInfo::new(name, prop_type, parser);
        rule_obj.build_struct(&mut info);
        info
    }
}

#[derive(Debug)]
struct PropInfo {
    name: String,
    prop_type: String,
    parser: String,
    parse_list: Vec<String>,
    symbols: Vec<String>,
}

impl PropInfo {
    pub fn new(name: String, prop_type: String, parser: String) -> Self {
        let name = name.to_string();
        Self {
            name,
            symbols: vec![],
            parser,
            prop_type,
            parse_list: vec![],
        }
    }

    pub fn add_variable(&mut self, variable: &str) {
        self.parse_list.push(variable.to_string());
    }

    pub fn add_symbol(&mut self, symbol: &str) {
        self.symbols.push(symbol.to_string());
    }
}

trait StructBuilder {
    fn build_struct(&self, f: &mut PropInfo);
}

impl StructBuilder for RuleObject {
    fn build_struct(&self, f: &mut PropInfo) {
        match self {
            Self::Brackets(arg0, _arg1) => arg0.build_struct(f),
            Self::Item(arg0, arg1) => {
                arg0.build_struct(f);
                if let Some(arg1) = arg1 {
                    arg1.build_struct(f);
                }
            }
            Self::DoubleAmpersand(arg0, arg1) => {
                arg0.build_struct(f);
                if let Some(arg1) = arg1 {
                    arg1.build_struct(f);
                }
            }
            Self::DoubleBar(arg0, arg1) => {
                arg0.build_struct(f);
                if let Some(arg1) = arg1 {
                    arg1.build_struct(f);
                }
            }
            Self::SingleBar(arg0, arg1) => {
                arg0.build_struct(f);
                if let Some(arg1) = arg1 {
                    arg1.build_struct(f);
                }
            }
            Self::Variable(arg0, _arg1) => f.add_variable(arg0),
            Self::Symbol(arg0, _arg1) => f.add_symbol(&arg0),
        }
    }
}
