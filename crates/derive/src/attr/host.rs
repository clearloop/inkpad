use crate::fun::HostFunction;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::parse_macro_input;

/// Parse host function
pub fn parse(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ts = TokenStream::from(item);
    let input: HostFunction = parse_macro_input!(ts);

    // construct struct
    let struct_ident = input.struct_ident();

    // construct host trait
    let module = attr.to_string();
    let name_str = input.name.to_string();

    // construct function
    let content = input.content;
    let mut attr_t = TokenStream2::new();
    for attr in input.attrs {
        attr.to_tokens(&mut attr_t);
    }

    // declare args
    let arg_len = &input.fields.len();
    let mut args = TokenStream2::new();
    for i in 0..*arg_len {
        (&mut args).extend(input.fields[i].declare(i))
    }

    // quote output
    let tks = quote! {
        #attr_t
        pub struct #struct_ident;

        impl Host for #struct_ident {
            fn module() -> &'static str {
                #module
            }

            fn name() -> &'static str {
                #name_str
            }

            fn wrap(sandbox: &mut Sandbox, args: &[Value]) -> Result<ReturnValue> {
                if args.len() != #arg_len {
                    return Err(Error::WrongArugmentLength);
                }

                #args

                #content
            }
        }
    };

    tks.into()
}
