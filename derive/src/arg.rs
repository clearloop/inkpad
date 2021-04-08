//! Function argument
use proc_macro2::{Punct, Spacing, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    Ident, Result, Token, Type,
};

/// Function argument
pub struct Arg {
    pub field: Ident,
    pub tk: Token![:],
    pub ty: Type,
}

impl ToTokens for Arg {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.append(self.field.clone());
        tokens.append(Punct::new(':', Spacing::Joint));
        self.ty.to_tokens(tokens);
    }
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

impl Arg {
    /// Declare args
    pub fn declare(&self, nth: usize) -> TokenStream2 {
        let ty = &self.ty;
        let field = &self.field;

        quote! {
            let #field: #ty = args[#nth].into();
        }
    }
}
