#![recursion_limit = "256"]
extern crate proc_macro;

#[macro_use]
extern crate quote;
extern crate syn;

use perl_xs_macro_support as support;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn perlxs(attr: TokenStream, input: TokenStream) -> TokenStream {
    match support::function::expand(attr.into(), input.into()) {
        Ok(tokens) => {
            if cfg!(feature = "debug_print_generated_code") {
                println!("{}", tokens);
            }
            tokens.into()
        }
        Err(error) => panic!("Macro expansion error {:?}", error),
    }
}

#[proc_macro]
pub fn package(input: TokenStream) -> TokenStream {
    match support::package::expand(input.into()) {
        Ok(tokens) => {
            if cfg!(feature = "debug_print_generated_code") {
                println!("{}", tokens);
            }
            tokens.into()
        }
        Err(error) => panic!("Macro expansion error {:?}", error),
    }
}

#[proc_macro_derive(DeriveTryFromContext, attributes(perlxs))]
pub fn from_kv(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match support::derive::expand(input.into()) {
        Ok(tokens) => {
            if cfg!(feature = "debug_print_generated_code") {
                println!("{}", tokens);
            }
            tokens.into()
        }
        Err(error) => panic!("Macro expansion error {:?}", error),
    }
}
