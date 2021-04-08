use proc_macro::TokenStream;
// use quote::quote;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::{Fn, Paren},
    Attribute, Expr, Ident, Result, Token, Type, Visibility,
};

struct Arg {
    pub field: Ident,
    pub tk: Token![:],
    pub ty: Type,
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Arg {
            field: input.parse()?,
            tk: input.parse()?,
            ty: input.parse()?,
        })
    }
}

/// Host function
struct Host {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub r#fn: Fn,
    pub name: Ident,
    pub paren_token: Paren,
    pub fields: Punctuated<Arg, Token![,]>,
    pub arrow_t: Token![->],
    pub return_t: Type,
    pub content: Expr,
}

impl Parse for Host {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Host {
            attrs: input.call(Attribute::parse_outer)?,
            vis: input.parse()?,
            r#fn: input.parse()?,
            name: input.parse()?,
            paren_token: parenthesized!(content in input),
            fields: content.parse_terminated(Arg::parse)?,
            arrow_t: input.parse()?,
            return_t: input.parse()?,
            content: input.parse()?,
        })
    }
}

pub fn host(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ic = item.clone();
    let _input: Host = parse_macro_input!(ic);

    // let expanded = quote! {
    //     #input
    // };
    //
    // TokenStream::from(expanded)
    item
}
