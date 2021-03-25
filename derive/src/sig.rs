use proc_macro::TokenStream;
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input, Expr,
};

/// ReturnValue of wasmi host function
struct Ret(pub Expr);

impl Parse for Ret {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Ret(input.parse::<Expr>()?))
    }
}

/// parser for #[sig(...)]
pub fn parse(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as Ret);
    match args.0 {
        Expr::Verbatim(ts) => {
            println!("{:?}", ts);
        }
        _ => {
            println!("Not Verbatim");
        }
    }
    println!("abccbbc");
    item
}
