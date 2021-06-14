//! Host function parser
use crate::arg::Arg;
use proc_macro2::Span;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{Fn, Paren},
    Attribute, Expr, Ident, Result, Token, Type, Visibility,
};

/// Host function
pub struct HostFunction {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    _fn: Fn,
    pub name: Ident,
    pub paren_token: Paren,
    pub fields: Punctuated<Arg, Token![,]>,
    pub arrow_t: Token![->],
    pub return_t: Type,
    pub content: Expr,
}

#[allow(clippy::eval_order_dependence)]
impl Parse for HostFunction {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(HostFunction {
            attrs: input.call(Attribute::parse_outer)?,
            vis: input.parse()?,
            _fn: input.parse()?,
            name: input.parse()?,
            paren_token: parenthesized!(content in input),
            fields: content.parse_terminated(Arg::parse)?,
            arrow_t: input.parse()?,
            return_t: input.parse()?,
            content: input.parse()?,
        })
    }
}

impl HostFunction {
    /// Generate struct ident from function name
    pub fn struct_ident(&self) -> Ident {
        let name: String = self
            .name
            .to_string()
            .split('_')
            .into_iter()
            .map(|s| {
                s.chars()
                    .into_iter()
                    .enumerate()
                    .map(|(i, v)| {
                        if i == 0 {
                            v.to_uppercase().to_string()
                        } else {
                            v.to_string()
                        }
                    })
                    .collect::<String>()
            })
            .collect();
        Ident::new(&name, Span::call_site())
    }
}
